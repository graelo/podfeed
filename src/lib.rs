#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod config;
pub mod convert;
pub mod error;
pub mod info;
pub mod rss;

/// Result type for this crate.
pub type Result<T> = std::result::Result<T, error::Error>;
