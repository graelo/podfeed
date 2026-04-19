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

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_episode() -> Episode {
        Episode {
            guid: "abc123".into(),
            pub_date: "Sun, 01 Jan 2023 09:10:11 +0000".into(),
            title: "Test Episode".into(),
            link: "https://youtube.com/watch?v=abc123".into(),
            description: "A test".into(),
            enclosure: Enclosure {
                file_url: "https://cdn.example.com/ep.mp4".into(),
                file_length: "99999".into(),
                file_type: "mp4".into(),
            },
            author: "Author".into(),
            image: Image {
                file_url: "https://cdn.example.com/thumb.png".into(),
            },
            duration: "600".into(),
            explicit_content: "false".into(),
        }
    }

    #[test]
    fn episode_serializes_to_xml() {
        let xml = sample_episode().to_string().unwrap();
        assert!(xml.contains("<item>"));
        assert!(xml.contains("<guid>abc123</guid>"));
        assert!(xml.contains("<![CDATA[Test Episode]]>"));
        assert!(xml.contains(r#"url="https://cdn.example.com/ep.mp4""#));
        assert!(xml.contains(r#"length="99999""#));
        assert!(xml.contains(r#"type="mp4""#));
        assert!(xml.contains("<itunes:duration>600</itunes:duration>"));
        assert!(xml.contains("<itunes:explicit>false</itunes:explicit>"));
    }

    #[test]
    fn enclosure_attributes_in_correct_order() {
        let xml = sample_episode().to_string().unwrap();
        // Enclosure should be a self-closing tag with url, length, type attributes
        assert!(xml.contains("<enclosure"));
        assert!(xml.contains("/>") || xml.contains("</enclosure>"));
    }
}
