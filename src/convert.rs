//! Convert from info files to rss.

use std::path::{Path, PathBuf};

use futures::stream::StreamExt;
use hard_xml::XmlWrite;
use image::{DynamicImage, GenericImageView, imageops};

use crate::{Result, info, rss};

const TARGET_SIZE: u32 = 1400;

/// List all playlist directories.
pub async fn available_directories<P: AsRef<Path>>(data_dirpath: P) -> Result<Vec<PathBuf>> {
    let mut directories: Vec<PathBuf> = vec![];

    let mut entries = smol::fs::read_dir(data_dirpath.as_ref()).await?;
    while let Some(entry) = entries.next().await {
        let entry = entry?;
        let path = entry.path();
        if path.file_name().unwrap() == "Cache" {
            continue;
        }
        if smol::fs::metadata(&path).await?.is_dir() {
            directories.push(path);
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
        episodes_with_indexes.sort_by_key(|a| a.1);
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
    // Resize channel image to fill 1400x1400 and add the "1400x1400" suffix.
    let resized_image_filepath = get_resized_image_filepath(image_filepath.as_ref(), TARGET_SIZE);

    // Resize the channel image if the resized image does not already exist.
    if !std::fs::exists(&resized_image_filepath)? {
        resize_image_to_fill(
            image_filepath.as_ref(),
            resized_image_filepath.as_ref(),
            TARGET_SIZE,
        )?;
    }

    let channel = rss::channel::Channel {
        title: source.title.clone(),
        description: source.description.clone(),
        link: source.link.clone(),
        image: rss::channel::Image {
            image_url: replace_base(
                base_dir.as_ref(),
                base_url.as_ref(),
                resized_image_filepath.as_ref(),
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
    // Resize episode image to fill 1400x1400 and add the "1400x1400" suffix.
    let resized_image_filepath = get_resized_image_filepath(image_filepath.as_ref(), TARGET_SIZE);

    // Resize the episode image if the resized image does not already exist.
    if !std::fs::exists(&resized_image_filepath)? {
        resize_image_to_fill(
            image_filepath.as_ref(),
            resized_image_filepath.as_ref(),
            TARGET_SIZE,
        )?;
    }

    let target = rss::episode::Episode {
        guid: source.guid.clone(),
        pub_date: source
            .pub_date()
            .format("%a, %d %b %Y %H:%M:%S %z")
            .to_string(),
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
                resized_image_filepath.as_ref(),
            ),
        },
        duration: source.duration_seconds.to_string(),
        explicit_content: "false".into(),
    };

    let playlist_index = source.playlist_index;

    Ok((target, playlist_index))
}

/// Return the filepath to the resized image (same extension as the original image).
fn get_resized_image_filepath<P: AsRef<Path>>(image_filepath: P, target_size: u32) -> PathBuf {
    let mut path = image_filepath.as_ref().to_path_buf();
    let filename = path.file_stem().unwrap().to_string_lossy();
    let extension = image_filepath.as_ref().extension().unwrap();
    let filename = format!("{filename}-{target_size}x{target_size}");
    path.set_file_name(filename);
    path.set_extension(extension);
    path
}

/// Pad an image with the given color on all sides.
fn pad_image(
    img: &DynamicImage,
    pad_left: u32,
    pad_right: u32,
    pad_top: u32,
    pad_bottom: u32,
    pad_color: image::Rgb<u8>,
) -> image::RgbImage {
    let (w, h) = img.dimensions();
    let new_w = w + pad_left + pad_right;
    let new_h = h + pad_top + pad_bottom;
    let mut new_img = image::RgbImage::from_pixel(new_w, new_h, pad_color);
    image::imageops::replace(
        &mut new_img,
        &img.to_rgb8(),
        pad_left as i64,
        pad_top as i64,
    );
    new_img
}

/// Resize an image to fill a square of `target_size` pixels.
///
/// The image is first scaled (aspect ratio preserved) to fit within a
/// `target_size × target_size` box, then padded symmetrically with black bars
/// so the output is always an exact `target_size × target_size` square.
///
/// This guarantees the square dimensions required by podcast clients such as
/// Apple Podcasts, which reject episode artwork that is not exactly square
/// (e.g. 1399×1400) and falls back to the show cover instead.
///
/// This function saves the resized image to `path-1400x1400.png` if the `target_size` is 1400.
///
#[must_use = "Use the return value of this function"]
fn resize_image_to_fill<P: AsRef<Path>>(
    image_filepath: P,
    resized_image_filepath: P,
    target_size: u32,
) -> Result<()> {
    let img = image::open(image_filepath)?;
    let t = target_size;

    // Resize preserving the aspect ratio so the image fits inside a t×t box.
    // The aspect-ratio-preserving `resize` may round the smaller axis down by
    // one pixel, so the result is not guaranteed to be t wide/t tall.
    let resized = img.resize(t, t, imageops::FilterType::CatmullRom);
    let (w, h) = resized.dimensions();

    // Pad both axes symmetrically to reach an exact t×t square. Deriving the
    // padding from the actual resized dimensions (rather than from a
    // precomputed target box) makes the square guarantee independent of how
    // `resize` rounds each axis.
    let black = image::Rgb::from([0, 0, 0]);
    let p_left = (t - w) / 2;
    let p_right = (t - w) - p_left;
    let p_top = (t - h) / 2;
    let p_bot = (t - h) - p_top;

    let new_img = pad_image(&resized, p_left, p_right, p_top, p_bot, black);

    new_img.save(resized_image_filepath)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn available_directories_skips_cache_and_files() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::create_dir(directory.path().join("channel")).unwrap();
        std::fs::create_dir(directory.path().join("Cache")).unwrap();
        std::fs::write(directory.path().join("README"), "not a channel").unwrap();

        let mut directories = smol::block_on(available_directories(directory.path())).unwrap();
        directories.sort();

        assert_eq!(directories, vec![directory.path().join("channel")]);
    }

    #[test]
    fn replace_base_substitutes_directory_with_url() {
        let result = replace_base(
            Path::new("/data/podcasts"),
            Path::new("https://cdn.example.com/podcasts"),
            Path::new("/data/podcasts/channel1/episode.mp4"),
        );
        assert_eq!(
            result,
            "https://cdn.example.com/podcasts/channel1/episode.mp4"
        );
    }

    #[test]
    fn replace_base_handles_trailing_slash() {
        let result = replace_base(
            Path::new("/data"),
            Path::new("https://cdn.example.com"),
            Path::new("/data/dir/file.jpg"),
        );
        assert_eq!(result, "https://cdn.example.com/dir/file.jpg");
    }

    #[test]
    fn get_resized_image_filepath_appends_dimensions() {
        let path = PathBuf::from("/data/channel/image.jpg");
        let resized = get_resized_image_filepath(&path, 1400);
        assert_eq!(resized, PathBuf::from("/data/channel/image-1400x1400.jpg"));
    }

    #[test]
    fn get_resized_image_filepath_preserves_extension() {
        let png = PathBuf::from("/tmp/thumb.png");
        let resized = get_resized_image_filepath(&png, 800);
        assert_eq!(resized, PathBuf::from("/tmp/thumb-800x800.png"));
    }

    #[test]
    fn pad_image_produces_correct_dimensions() {
        let img = DynamicImage::new_rgb8(100, 60);
        let padded = pad_image(&img, 10, 10, 20, 20, image::Rgb([0, 0, 0]));
        assert_eq!(padded.width(), 120);
        assert_eq!(padded.height(), 100);
    }

    #[test]
    fn convert_episode_builds_rss_episode() {
        // Create a minimal 2x2 image on disk for the resize step.
        let tmp = tempfile::TempDir::new().unwrap();
        let img_path = tmp.path().join("thumb.png");
        image::RgbImage::from_pixel(2, 2, image::Rgb([0, 0, 0]))
            .save(&img_path)
            .unwrap();

        let source = info::episode::Info {
            guid: "abc123".into(),
            upload_date: "20230101".into(),
            playlist_index: 5,
            title: "Ep Title".into(),
            link: "https://youtube.com/watch?v=abc123".into(),
            description: "desc".into(),
            author: "Author".into(),
            duration_seconds: 600,
        };

        let enclosure = info::episode::Enclosure {
            video_filepath: tmp.path().join("video.mp4"),
            video_filelength: 123456,
            video_filetype: "mp4".into(),
        };

        let base_dir: &Path = tmp.path();
        let base_url: &Path = Path::new("https://cdn.example.com");
        let img: &Path = img_path.as_path();
        let (ep, idx) = convert_episode(base_dir, base_url, &source, &enclosure, img).unwrap();

        assert_eq!(idx, 5);
        assert_eq!(ep.guid, "abc123");
        assert_eq!(ep.author, "Author");
        assert_eq!(ep.duration, "600");
        assert_eq!(ep.enclosure.file_type, "mp4");
        assert_eq!(ep.enclosure.file_length, "123456");
        assert!(ep.enclosure.file_url.starts_with("https://cdn.example.com"));
        assert!(ep.image.file_url.contains("thumb-1400x1400.png"));
    }

    #[test]
    fn process_builds_feed_and_orders_episodes_by_playlist_index() {
        let tmp = tempfile::TempDir::new().unwrap();
        let channel_dir = tmp.path().join("channel");
        std::fs::create_dir(&channel_dir).unwrap();

        let channel_stem = "NA--PLtest-123456--Example_Playlist--NA";
        std::fs::write(
            channel_dir.join(format!("{channel_stem}.info.json")),
            r#"{
                "modified_date": "20230101",
                "title": "Example Channel",
                "description": "Channel description",
                "webpage_url": "https://youtube.com/playlist?list=PLtest-123456",
                "channel": "Author"
            }"#,
        )
        .unwrap();
        image::RgbImage::from_pixel(2, 2, image::Rgb([0, 0, 0]))
            .save(channel_dir.join(format!("{channel_stem}.jpg")))
            .unwrap();

        for (date, id, title, index) in [
            ("20230102", "aaaaaaaaaaa", "Later", 1),
            ("20230101", "bbbbbbbbbbb", "Earlier", 0),
        ] {
            let stem = format!("{date}--{id}--{title}");
            std::fs::write(
                channel_dir.join(format!("{stem}.info.json")),
                format!(
                    r#"{{
                        "id": "{id}",
                        "upload_date": "{date}",
                        "playlist_index": {index},
                        "title": "{title}",
                        "webpage_url": "https://youtube.com/watch?v={id}",
                        "description": "Episode description",
                        "channel": "Author",
                        "duration": 60
                    }}"#
                ),
            )
            .unwrap();
            std::fs::write(channel_dir.join(format!("{stem}.mp4")), [0_u8; 3]).unwrap();
            image::RgbImage::from_pixel(2, 2, image::Rgb([0, 0, 0]))
                .save(channel_dir.join(format!("{stem}.png")))
                .unwrap();
        }

        let feed = smol::block_on(process(
            tmp.path(),
            &channel_dir,
            Path::new("https://cdn.example.com"),
        ))
        .unwrap();

        assert!(feed.starts_with(r#"<?xml version="1.0" encoding="UTF-8"?>"#));
        assert!(feed.contains("<title>Example Channel</title>"));
        assert!(feed.contains("https://cdn.example.com/channel/"));
        let earlier = feed.find("<guid>bbbbbbbbbbb</guid>").unwrap();
        let later = feed.find("<guid>aaaaaaaaaaa</guid>").unwrap();
        assert!(earlier < later);
    }

    #[test]
    fn convert_channel_builds_rss_channel_and_resizes_image() {
        let tmp = tempfile::TempDir::new().unwrap();
        let image_filepath = tmp.path().join("channel.png");
        image::RgbImage::from_pixel(100, 50, image::Rgb([128, 128, 128]))
            .save(&image_filepath)
            .unwrap();

        let source = info::channel::Info {
            upload_date: "20230101".into(),
            title: "Channel title".into(),
            description: "Channel description".into(),
            link: "https://youtube.com/playlist?list=PLtest-12345".into(),
            author: "Author".into(),
        };
        let channel = convert_channel(
            tmp.path(),
            Path::new("https://cdn.example.com"),
            &source,
            &image_filepath,
            Vec::new(),
        )
        .unwrap();

        assert_eq!(channel.title, "Channel title");
        assert_eq!(channel.description, "Channel description");
        assert_eq!(
            channel.link,
            "https://youtube.com/playlist?list=PLtest-12345"
        );
        assert_eq!(channel.author, "Author");
        assert_eq!(channel.language, "en");
        assert_eq!(channel.category, "Technology & Science");
        assert_eq!(channel.generator, "ytdlp");
        assert_eq!(channel.explicit_content, "false");
        assert_eq!(channel.channel_type, "Serial");
        assert_eq!(channel.pub_date, "Sun, 01 Jan 2023 09:10:11 +0000");
        assert!(!channel.last_build_date.is_empty());
        assert_eq!(
            channel.image.image_url,
            "https://cdn.example.com/channel-1400x1400.png"
        );

        let resized = image::open(tmp.path().join("channel-1400x1400.png")).unwrap();
        assert_eq!(resized.dimensions(), (1400, 1400));
    }

    #[test]
    fn resize_image_to_fill_creates_square_output_for_landscape_image() {
        let tmp = tempfile::TempDir::new().unwrap();
        let src = tmp.path().join("wide.png");
        image::RgbImage::from_pixel(200, 100, image::Rgb([128, 128, 128]))
            .save(&src)
            .unwrap();

        let dst = tmp.path().join("wide-100x100.png");
        resize_image_to_fill(&src, &dst, 100).unwrap();

        let resized = image::open(&dst).unwrap();
        assert_eq!(resized.dimensions(), (100, 100));
    }

    #[test]
    fn resize_image_to_fill_creates_square_output_for_portrait_image() {
        let tmp = tempfile::TempDir::new().unwrap();
        let src = tmp.path().join("portrait.png");
        image::RgbImage::from_pixel(100, 200, image::Rgb([128, 128, 128]))
            .save(&src)
            .unwrap();

        let dst = tmp.path().join("portrait-100x100.png");
        resize_image_to_fill(&src, &dst, 100).unwrap();

        let resized = image::open(&dst).unwrap();
        assert_eq!(resized.dimensions(), (100, 100));
    }
}
