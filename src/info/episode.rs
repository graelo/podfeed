//! Handles the `.info.json` episode files created by yt-dlp.

use std::path::{Path, PathBuf};

use async_std::{fs, stream::StreamExt};
use chrono::{DateTime, NaiveDate, offset::Utc};
use regex::Regex;
use serde::Deserialize;

use crate::Result;

/// Represents the info.json file of an episode.
#[derive(Debug, Clone)]
pub struct InfoFile {
    /// Publication date of the episode.
    pub pub_date: NaiveDate,
    /// Youtube ID of the episode.
    pub youtube_id: String,
    /// Filepath of the episode .
    pub filepath: PathBuf,
}

impl InfoFile {
    /// Parse the associated `EpisodeInfo` and return it along with the enclosure.
    pub async fn parse(&self) -> Result<(Info, Enclosure, PathBuf)> {
        let content = async_std::fs::read_to_string(&self.filepath).await?;
        let ep_info: Info = serde_json::from_str(&content)?;

        let video_filepath: async_std::path::PathBuf = self
            .filepath
            // remove ".json"
            .with_extension("")
            // replace ".info" with ".mp4"
            .with_extension("mp4")
            .into();

        let video_filelength = video_filepath.metadata().await?.len();

        let image_filepath = self
            .filepath
            // remove ".json"
            .with_extension("")
            // replace ".info" with ".mp4"
            .with_extension("png");

        let enclosure = Enclosure {
            video_filepath: video_filepath.into(),
            video_filelength,
            video_filetype: "mp4".into(),
        };

        Ok((ep_info, enclosure, image_filepath))
    }
}

/// Return all episode files in `dirpath`.
pub async fn available_episodes<P: AsRef<Path>>(dirpath: P) -> Result<Vec<InfoFile>> {
    let mut episodes: Vec<InfoFile> = vec![];

    let pattern = r#"(\d{8})--(.{11})--.*\.info\.json"#;
    let matcher = Regex::new(pattern).unwrap();

    let mut entries = fs::read_dir(dirpath.as_ref()).await?;
    while let Some(entry) = entries.next().await {
        let entry = entry?;
        let path = entry.path();
        if let Some(captures) = matcher.captures(&path.to_string_lossy()) {
            let date_str = &captures[1];
            let pub_date = NaiveDate::parse_from_str(date_str, "%Y%m%d").unwrap();
            let youtube_id = &captures[2];
            let episode = InfoFile {
                pub_date,
                youtube_id: youtube_id.into(),
                filepath: path.into(),
            };
            episodes.push(episode);
        }
    }

    episodes.sort_unstable_by_key(|b| b.pub_date);

    Ok(episodes)
}

/// The content of an info.json file for an episode.
#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    /// Episode ID.
    #[serde(rename = "id")]
    pub guid: String,

    /// Publication date.
    #[serde(rename = "upload_date")]
    pub upload_date: String,

    /// Playlist Index.
    pub playlist_index: u32,

    /// Episode title.
    pub title: String,

    /// Episode webpage link (youtube page).
    #[serde(rename = "webpage_url")]
    pub link: String,

    /// Episode description.
    pub description: String,

    /// Author (usually the channel name).
    #[serde(rename = "channel")]
    pub author: String,

    /// Duration of the episode.
    #[serde(rename = "duration")]
    pub duration_seconds: u32,
}

impl Info {
    pub(crate) fn pub_date(&self) -> DateTime<Utc> {
        let naived_date = NaiveDate::parse_from_str(&self.upload_date, "%Y%m%d")
            .unwrap()
            .and_hms_opt(9, 10, 11)
            .unwrap();
        DateTime::<Utc>::from_naive_utc_and_offset(naived_date, Utc)
    }
}

/// Represents the video file for an episode.
#[derive(Debug, Clone)]
pub struct Enclosure {
    /// Path to the video file.
    pub video_filepath: PathBuf,
    /// File length of the video file.
    pub video_filelength: u64,
    /// File type of the video.
    pub video_filetype: String,
}
