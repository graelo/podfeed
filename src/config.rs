//! Configuration.

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint};
use clap_complete::Shell;

/// Save or restore Tmux sessions.
#[derive(Debug, Parser)]
#[clap(author, about, version)]
#[clap(propagate_version = true)]
pub struct Config {
    /// Selection of commands.
    #[command(subcommand)]
    pub command: Command,
}

/// Indicate whether to save (resp. restore) the Tmux sessions to (resp. from) a backup.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Save the Tmux sessions to a new backup file.
    ///
    /// Sessions, windows, and panes geometry + content are saved in an archive format inside the
    /// backup folder. In that folder, the backup name is expected to be similar to
    /// `backup-20220531T123456.tar.zst`.
    ///
    /// If you run this command via a Tmux keybinding, use the `--to-tmux` flag in order to send a
    /// one-line report to the Tmux status bar. If you run this command from the terminal, ignore
    /// this flag in order to print the one-line report in the terminal.
    Generate {
        /// Data dirpath.
        ///
        /// Path to folders created by the downloader.
        #[arg(short = 'd', long = "data-dir", value_hint = ValueHint::DirPath, env = "DATADIR")]
        data_dir: PathBuf,

        /// Base URL.
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
