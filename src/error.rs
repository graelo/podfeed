//! This crate's error type.

use std::{io, path::PathBuf};

/// Describes all errors from this crate.
///
/// - errors during backup operations.
/// - errors reported by tmux
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Unsupported archive version.
    #[error("unsupported archive version: `{0}`")]
    ArchiveVersion(String),

    /// Backup file contains no metadata.
    #[error("missing metadata: `{0}`")]
    MissingMetadata(String),

    /// Source directory contains no channel file.
    #[error("missing channel info file: `{0}`")]
    MissingChannelInfoFile(PathBuf),

    /// Source directory contains multiple channel files.
    #[error("multiple channel info files: `{0}`")]
    MultipleChannelInfoFiles(PathBuf),

    /// Configuration error.
    #[error("unexpected configuration: `{0}`")]
    ConfigError(String),

    /// Image conversion error.
    #[error("image conversion error: `{source}`")]
    Image {
        #[from]
        /// Source error.
        source: image::ImageError,
    },

    /// Serde JSON error.
    #[error("serde json error: `{source}`")]
    Json {
        #[from]
        /// Source error,
        source: serde_json::Error,
    },

    /// XML error.
    #[error("xml error: `{source}`")]
    Xml {
        #[from]
        /// Source error,
        source: hard_xml::XmlError,
    },

    /// Some IO error.
    #[error("failed with io: `{source}`")]
    Io {
        #[from]
        /// Source error.
        source: io::Error,
    },
}
