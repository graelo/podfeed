//! Configuration.

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint};
use clap_complete::Shell;

/// Parse podfeed command-line arguments.
#[derive(Debug, Parser)]
#[clap(author, about, version)]
#[clap(propagate_version = true)]
pub struct Config {
    /// Selection of commands.
    #[command(subcommand)]
    pub command: Command,
}

/// Select a podfeed command.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Generate RSS feeds from yt-dlp data directories.
    Generate {
        /// Root directory containing channel directories created by yt-dlp.
        #[arg(short = 'd', long = "data-dir", value_hint = ValueHint::DirPath, env = "DATADIR")]
        data_dir: PathBuf,

        /// Public URL corresponding to the data directory.
        #[arg(long = "base-url", env = "BASEURL")]
        base_url: String,
    },

    /// Print a shell completion script to stdout.
    GenerateCompletion {
        /// Shell for which you want completion.
        #[arg(value_enum, value_parser = clap::value_parser!(Shell))]
        shell: Shell,
    },
}
