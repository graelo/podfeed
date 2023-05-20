//! Convert from info files to rss.

use std::path::{Path, PathBuf};

use async_std::stream::StreamExt;
use hard_xml::XmlWrite;
use photon_rs::{
    native::{open_image, save_image},
    transform::{padding_bottom, padding_left, padding_right, padding_top, resize},
    Rgba,
};

use crate::{info, rss, Result};

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

/// Parse channel & episodes, and return the rendered xml.
pub async fn process<P: AsRef<Path>>(base_dir: P, dirpath: P, base_url: P) -> Result<String> {
    let episode_infofiles = info::episode::available_episodes(dirpath.as_ref()).await?;
    println!(
        "- {} ({} episodes)",
        dirpath.as_ref().to_string_lossy(),
        episode_infofiles.len()
    );

    let mut episodes_with_indexes: Vec<(rss::episode::Episode, u32)> = vec![];
    for episode_infofile in episode_infofiles {
        let (episode_info, episode_enclosure, episode_image_filepath) =
            episode_infofile.parse().await?;

        let (rss_episode, playlist_index) = convert_episode(
            base_dir.as_ref(),
            base_url.as_ref(),
            &episode_info,
            &episode_enclosure,
            &episode_image_filepath,
        )?;
        episodes_with_indexes.push((rss_episode, playlist_index));
    }

    // Sort episodes by playlist index.
    let episodes = {
        episodes_with_indexes.sort_by(|a, b| a.1.cmp(&b.1));
        episodes_with_indexes
            .into_iter()
            .map(|(episode, _)| episode)
            .collect::<Vec<_>>()
    };

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
        content_namespace: "http://purl.org/rss/1.0/modules/content/".into(),
        channel: rss_channel,
    };
    let rendered_rss = feed.to_string()?;
    let xml_prolog = r#"<?xml version="1.0" encoding="UTF-8"?>"#;

    let rendered_rss = format!("{xml_prolog}\n{rendered_rss}");
    Ok(rendered_rss)
}

/// Convert channel Info into a RSS Channel.
pub fn convert_channel<P: AsRef<Path>>(
    base_dir: P,
    base_url: P,
    source: &info::channel::Info,
    image_filepath: P,
    episodes: Vec<rss::episode::Episode>,
) -> Result<rss::channel::Channel> {
    resize_image_to_fill(image_filepath.as_ref(), 1400)?;

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
        last_build_date: format!(
            "{}",
            source.last_build_date().format("%a, %d %b %Y %H:%M:%S %z")
        ),
        pub_date: format!("{}", source.pub_date().format("%a, %d %b %Y %H:%M:%S %z")),
        category: source.category().to_string(),
        generator: source.generator().to_string(),
        explicit_content: source.explicit_content().to_string(),
        channel_type: source.channel_type().to_string(),
        episodes,
    };

    Ok(channel)
}

/// Convert episode Info to a RSS Episode.
pub fn convert_episode<P: AsRef<Path>>(
    base_dir: P,
    base_url: P,
    source: &info::episode::Info,
    enclosure: &info::episode::Enclosure,
    image_filepath: P,
) -> Result<(rss::episode::Episode, u32)> {
    resize_image_to_fill(image_filepath.as_ref(), 1400)?;

    let target = rss::episode::Episode {
        guid: source.guid.clone(),
        pub_date: format!("{}", &source.pub_date().format("%a, %d %b %Y %H:%M:%S %z")),
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

    let playlist_index = source.playlist_index;

    Ok((target, playlist_index))
}

fn resize_image_to_fill<P: AsRef<Path>>(image_filepath: P, target_size: u32) -> Result<()> {
    let img = open_image(&image_filepath.as_ref().to_string_lossy())?;
    let (a, b) = (img.get_width(), img.get_height());
    let t = target_size;
    if !(a == t && b == t) {
        let (na, nb, p_left, p_right, p_top, p_bot) = if a > b {
            let pb = t - t * b / a;
            let p_top = pb / 2;
            let p_bot = pb - p_top;
            (t, t * b / a, 0, 0, p_top, p_bot)
        } else {
            let pa = t - t * a / b;
            let p_left = pa / 2;
            let p_right = pa - p_left;
            (t * a / b, t, p_left, p_right, 0, 0)
        };

        let new_img = resize(
            &img,
            na,
            nb,
            photon_rs::transform::SamplingFilter::CatmullRom,
        );
        let black = Rgba::new(0, 0, 0, 0);
        let new_img = padding_left(&new_img, p_left, black);
        let black = Rgba::new(0, 0, 0, 0);
        let new_img = padding_right(&new_img, p_right, black);
        let black = Rgba::new(0, 0, 0, 0);
        let new_img = padding_top(&new_img, p_top, black);
        let black = Rgba::new(0, 0, 0, 0);
        let new_img = padding_bottom(&new_img, p_bot, black);

        // let (fa, fb) = (new_img.get_width(), new_img.get_height());
        // let path = image_filepath.as_ref().to_string_lossy();
        // println!("dims: {a}x{b} -> {na}, {nb}, {p_left}, {p_right}, {p_top}, {p_bot} -> {fa}x{fb} - {path}");

        save_image(new_img, &image_filepath.as_ref().to_string_lossy())?;
    }

    Ok(())
}
