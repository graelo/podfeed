#![warn(missing_docs)]

//! Convert `*.info.json` files into a single RSS feed.

pub mod config;
pub mod convert;
pub mod error;
pub mod info;
pub mod rss;

/// Result type for this crate.
pub type Result<T> = std::result::Result<T, error::Error>;
