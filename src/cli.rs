use clap::Parser;

use std::path::PathBuf;

/// A ranger-like flake.lock viewer
/// {n}https://github.com/nix-community/nix-melt
#[derive(Parser)]
#[command(version)]
pub(crate) struct Opts {
    /// Path to the flake.lock or the directory containing flake.lock
    #[arg(default_value = "flake.lock")]
    pub path: PathBuf,

    /// Format to display timestamps
    ///
    /// See https://time-rs.github.io/book/api/format-description.html for the syntax
    #[arg(
        short,
        long,
        default_value = "[year]-[month]-[day] [hour]:[minute] [offset_hour sign:mandatory]:[offset_minute]"
    )]
    pub time_format: String,
}
