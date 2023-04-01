//! Handles the `.info.json` channel file created by yt-dlp.

use std::path::{Path, PathBuf};

use async_std::{fs, stream::StreamExt};
use chrono::{offset::Utc, DateTime, Local, NaiveDate};
use regex::Regex;
use serde::Deserialize;

use crate::{error::Error, Result};

/// Represents the info.json file of a channel.
#[derive(Debug, Clone)]
pub struct InfoFile {
    /// Youtube ID of the channel (playlist_id).
    pub youtube_id: String,
    /// Filepath of the playlist .
    pub filepath: PathBuf,
}

impl InfoFile {
    /// Parse the associated `Info` and return it along with the image url.
    pub async fn parse(&self) -> Result<(Info, PathBuf)> {
        let content = async_std::fs::read_to_string(&self.filepath).await?;
        let ch_info: Info = serde_json::from_str(&content)?;

        let image_filepath = self
            .filepath
            // remove ".json"
            .with_extension("")
            // replace ".info" with ".mp4"
            .with_extension("jpg");

        Ok((ch_info, image_filepath))
    }
}

/// Return the channel file in `dirpath`. If no file is found, or if multiple files are found,
/// return an error.
pub async fn available_channel<P: AsRef<Path>>(dirpath: P) -> Result<InfoFile> {
    let mut files: Vec<InfoFile> = vec![];

    let pattern = r#"NA--([a-zA-Z0-9-_]{18,34})--.*\.info\.json"#;
    // let pattern = r#"[^-]+--([a-zA-Z0-9-_]{34})--.*\.info\.json"#;
    let matcher = Regex::new(pattern).unwrap();

    let mut entries = fs::read_dir(dirpath.as_ref()).await?;
    while let Some(entry) = entries.next().await {
        let entry = entry?;
        let path = entry.path();
        if let Some(captures) = matcher.captures(&path.to_string_lossy()) {
            let youtube_id = &captures[1];
            let channel = InfoFile {
                youtube_id: youtube_id.into(),
                filepath: path.into(),
            };
            files.push(channel);
        }
    }

    match files.len() {
        0 => Err(Error::MissingChannelInfoFile(dirpath.as_ref().into())),
        1 => Ok(files.first().unwrap().clone()),
        _ => Err(Error::MultipleChannelInfoFiles(dirpath.as_ref().into())),
    }
}

/// The content of an info.json file for a channel.
#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    // /// Channel ID.
    // #[serde(rename = "id")]
    // pub guid: String,
    /// Channel last publication date.
    #[serde(rename = "modified_date")]
    pub upload_date: String,

    /// Channel title.
    pub title: String,

    /// Episode description.
    pub description: String,

    /// Channel webpage link (youtube page).
    #[serde(rename = "webpage_url")]
    pub link: String,

    /// Author (usually the channel name).
    #[serde(rename = "channel")]
    pub author: String,
}

impl Info {
    pub(crate) fn pub_date(&self) -> DateTime<Utc> {
        let naived_date = NaiveDate::parse_from_str(&self.upload_date, "%Y%m%d")
            .unwrap()
            .and_hms_opt(9, 10, 11)
            .unwrap();
        DateTime::<Utc>::from_utc(naived_date, Utc)
    }

    pub(crate) fn language(&self) -> &'static str {
        "en"
    }

    pub(crate) fn category(&self) -> &'static str {
        "Technology & Science"
    }

    pub(crate) fn last_build_date(&self) -> DateTime<Local> {
        Local::now()
    }

    pub(crate) fn generator(&self) -> &'static str {
        "ytdlp"
    }

    pub(crate) fn explicit_content(&self) -> &'static str {
        "false"
    }

    pub(crate) fn channel_type(&self) -> &'static str {
        "Serial"
    }
}
