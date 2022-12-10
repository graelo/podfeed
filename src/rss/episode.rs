//! RSS Episode.

use hard_xml::XmlWrite;

/// Represents a podcasts episode.
#[derive(Debug, PartialEq, Eq, XmlWrite)]
#[xml(tag = "item")]
pub struct Episode {
    /// GUID of the episode.
    #[xml(flatten_text = "guid")]
    pub guid: String,

    /// Publication date of the episode.
    #[xml(flatten_text = "pubDate")]
    pub pub_date: String,

    /// Title of the episode.
    #[xml(flatten_text = "title", cdata)]
    pub title: String,

    /// Link to the webpage of the episode.
    #[xml(flatten_text = "link")]
    pub link: String,

    /// Description of the episode.
    #[xml(flatten_text = "description", cdata)]
    pub description: String,

    /// Player metadata for the episode.
    #[xml(child = "enclosure")]
    pub enclosure: Enclosure,

    /// Author of the episode (usually = channel name).
    #[xml(flatten_text = "itunes:author")]
    pub author: String,

    /// Image of the episode.
    #[xml(child = "itunes:image")]
    pub image: Image,

    /// Duration of the episode.
    #[xml(flatten_text = "itunes:duration")]
    pub duration: String,

    /// Classification of the episode.
    #[xml(flatten_text = "itunes:explicit")]
    pub explicit_content: String,
}

/// Player metadata for an episode.
#[derive(Debug, PartialEq, Eq, XmlWrite)]
#[xml(tag = "enclosure")]
pub struct Enclosure {
    /// URL of the episode's video file.
    #[xml(attr = "url")]
    pub file_url: String,

    /// Video file size.
    #[xml(attr = "length")]
    pub file_length: String,

    /// Video type.
    #[xml(attr = "type")]
    pub file_type: String,
}

/// Image for an episode.
#[derive(Debug, PartialEq, Eq, XmlWrite)]
#[xml(tag = "itunes:image")]
pub struct Image {
    /// URL of the episode's image file.
    #[xml(attr = "href")]
    pub file_url: String,
}
