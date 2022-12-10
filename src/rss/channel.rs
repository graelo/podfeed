//! RSS Channel.

use hard_xml::XmlWrite;

use super::episode::Episode;

/// Represents the `channel` element.
#[derive(Debug, PartialEq, Eq, XmlWrite)]
#[xml(tag = "channel")]
pub struct Channel {
    /// Title of the channel.
    #[xml(flatten_text = "title")]
    pub title: String,

    /// Description of the channel.
    #[xml(flatten_text = "description", cdata)]
    pub description: String,

    /// Link of the channel (usually the playlist URL).
    #[xml(flatten_text = "link")]
    pub link: String,

    /// Image of the channel.
    #[xml(child = "image")]
    pub image: Image,

    /// Author of the channel.
    #[xml(flatten_text = "itunes:author")]
    pub author: String,

    /// Language of the channel.
    #[xml(flatten_text = "language")]
    pub language: String,

    /// Last build date of the channel.
    #[xml(flatten_text = "lastBuildDate")]
    pub last_build_date: String,

    /// Last publication date of the channel.
    #[xml(flatten_text = "pubDate")]
    pub pub_date: String,

    /// Category of the channel.
    #[xml(flatten_text = "category")]
    pub category: String,

    /// Generator of the channel.
    #[xml(flatten_text = "generator")]
    pub generator: String,

    /// Classification of the channel.
    #[xml(flatten_text = "itunes:explicit")]
    pub explicit_content: String,

    /// Type of the channel: Serial or Episodic.
    #[xml(flatten_text = "itunes:type")]
    pub channel_type: String,

    /// Episodes in the channel.
    #[xml(child = "item")]
    pub episodes: Vec<Episode>,
}

/// Image for the channel.
#[derive(Debug, PartialEq, Eq, XmlWrite)]
#[xml(tag = "itunes:image")]
pub struct Image {
    /// URL of the channel's image file.
    #[xml(attr = "href")]
    pub image_url: String,
}
