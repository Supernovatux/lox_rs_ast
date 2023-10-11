use std::path::PathBuf;

// Struct for clap cli for lox
//
use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    // Verbosity flag
    #[command(flatten)]
    pub verbose: Verbosity,
}
#[derive(Subcommand)]
pub enum Commands {
    File {
        /// file to to interpret. A positional argument
        file: PathBuf,
    },
}
