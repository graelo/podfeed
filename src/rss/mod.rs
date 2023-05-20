//! Struct and functions for RSS feed.

use hard_xml::XmlWrite;

pub mod channel;
pub mod episode;

use channel::Channel;

/// Represents a RSS feed for a podcast.
#[derive(Debug, PartialEq, Eq, XmlWrite)]
#[xml(tag = "rss")]
pub struct Rss {
    /// Identifier.
    #[xml(attr = "version")]
    pub version: String,

    /// Namespace.
    #[xml(attr = "xmlns:itunes")]
    pub namespace: String,

    /// Additional namespace.
    #[xml(attr = "xmlns:content")]
    pub content_namespace: String,

    /// Channel definition.
    #[xml(child = "channel")]
    pub channel: Channel,
    // pub episodes: Vec<Episode>,
}
