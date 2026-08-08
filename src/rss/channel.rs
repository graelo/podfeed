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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_serializes_metadata_and_cdata() {
        let channel = Channel {
            title: "A & B".into(),
            description: "Description with <markup>".into(),
            link: "https://example.com/feed".into(),
            image: Image {
                image_url: "https://example.com/image.jpg".into(),
            },
            author: "Author".into(),
            language: "en".into(),
            last_build_date: "Mon, 01 Jan 2024 00:00:00 +0000".into(),
            pub_date: "Mon, 01 Jan 2024 00:00:00 +0000".into(),
            category: "Technology & Science".into(),
            generator: "ytdlp".into(),
            explicit_content: "false".into(),
            channel_type: "Serial".into(),
            episodes: Vec::new(),
        };

        let xml = channel.to_string().unwrap();

        assert!(xml.contains("<channel>"));
        assert!(xml.contains("<title>A &amp; B</title>"));
        assert!(xml.contains("<![CDATA[Description with <markup>]]>"));
        assert!(xml.contains(r#"href="https://example.com/image.jpg""#));
        assert!(xml.contains("<itunes:type>Serial</itunes:type>"));
    }
}
