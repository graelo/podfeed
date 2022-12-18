//! Convert from info files to rss.

use async_std::stream::StreamExt;
use std::path::{Path, PathBuf};

use hard_xml::XmlWrite;

use crate::{info, rss, Result};

/// Convert episode Info to a RSS Episode.
pub fn convert_episode<P: AsRef<Path>>(
    base_dir: P,
    base_url: P,
    source: &info::episode::Info,
    enclosure: &info::episode::Enclosure,
    image_filepath: P,
) -> Result<rss::episode::Episode> {
    let target = rss::episode::Episode {
        guid: source.guid.clone(),
        pub_date: format!("{}", &source.pub_date().format("%a, %d %b %Y %H:%M:%S")),
        title: source.title.clone(),
        link: source.link.clone(),
        description: source.description.clone(),
        enclosure: rss::episode::Enclosure {
            file_url: replace_base(
                base_dir.as_ref(),
                base_url.as_ref(),
                enclosure.video_filepath.as_ref(),
            ),
            file_length: enclosure.video_filelength.to_string(),
            file_type: enclosure.video_filetype.clone(),
        },
        author: source.author.clone(),
        image: rss::episode::Image {
            file_url: replace_base(
                base_dir.as_ref(),
                base_url.as_ref(),
                image_filepath.as_ref(),
            ),
        },
        duration: source.duration_seconds.to_string(),
        explicit_content: "false".into(),
    };

    Ok(target)
}

/// Replace the parent base directory with the serving base url.
fn replace_base<P: AsRef<Path>>(base_dir: P, base_url: P, filepath: P) -> String {
    filepath
        .as_ref()
        .strip_prefix(base_dir)
        .map(|s| base_url.as_ref().join(s))
        .unwrap()
        .to_string_lossy()
        .to_string()
}

/// Convert channel Info into a RSS Channel.
pub fn convert_channel<P: AsRef<Path>>(
    base_dir: P,
    base_url: P,
    source: &info::channel::Info,
    image_filepath: P,
    episodes: Vec<rss::episode::Episode>,
) -> Result<rss::channel::Channel> {
    let channel = rss::channel::Channel {
        title: source.title.clone(),
        description: source.description.clone(),
        link: source.link.clone(),
        image: rss::channel::Image {
            image_url: replace_base(
                base_dir.as_ref(),
                base_url.as_ref(),
                image_filepath.as_ref(),
            ),
        },
        author: source.author.clone(),
        language: source.language().to_string(),
        last_build_date: source.last_build_date(),
        pub_date: source.pub_date().to_string(),
        category: source.category().to_string(),
        generator: source.generator().to_string(),
        explicit_content: source.explicit_content().to_string(),
        channel_type: source.channel_type().to_string(),
        episodes,
    };

    Ok(channel)
}

/// Parse channel & episodes, and return the rendered xml.
pub async fn process<P: AsRef<Path>>(base_dir: P, dirpath: P, base_url: P) -> Result<String> {
    let episode_infofiles = info::episode::available_episodes(dirpath.as_ref()).await?;

    let mut episodes: Vec<rss::episode::Episode> = vec![];
    for episode_infofile in episode_infofiles {
        let (episode_info, episode_enclosure, episode_image_filepath) =
            episode_infofile.parse().await?;

        let rss_episode = convert_episode(
            base_dir.as_ref(),
            base_url.as_ref(),
            &episode_info,
            &episode_enclosure,
            &episode_image_filepath,
        )?;
        episodes.push(rss_episode);
    }

    let channel_infofile = info::channel::available_channel(&dirpath).await?;
    let (channel_info, channel_image_filepath) = channel_infofile.parse().await?;
    let rss_channel = convert_channel(
        base_dir.as_ref(),
        base_url.as_ref(),
        &channel_info,
        &channel_image_filepath,
        episodes,
    )?;

    let feed = rss::Rss {
        version: "2.0".into(),
        namespace: "http://www.itunes.com/dtds/podcast-1.0.dtd".into(),
        channel: rss_channel,
    };
    let rendered_rss = feed.to_string()?;
    let xml_prolog = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

    let rendered_rss = format!("{xml_prolog}\n{rendered_rss}");
    Ok(rendered_rss)
}

/// List all playlist directories.
pub async fn available_directories<P: AsRef<Path>>(data_dirpath: P) -> Result<Vec<PathBuf>> {
    let mut directories: Vec<PathBuf> = vec![];

    let mut entries = async_std::fs::read_dir(data_dirpath.as_ref()).await?;
    while let Some(entry) = entries.next().await {
        let entry = entry?;
        let path = entry.path();
        if path.file_name().unwrap() == "Cache" {
            continue;
        }
        if path.is_dir().await {
            directories.push(path.into());
        }
    }

    Ok(directories)
}
