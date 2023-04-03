mod framework;
pub mod tools;

use clap::{Parser, command, ArgGroup};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author = "SliceOfArdath", version, about = "Find code, fast.", long_about = None)]
pub struct Args {
    /// The regular expression used for searching.
    #[arg(required=true,value_name="PATTERN")]
    pattern: String,
    /// The file or directory to search.
    #[arg(value_name="PATH")]
    file: Option<PathBuf>,
}