use clap::Parser;
use log::*;
use lox_rs_ast::{
    cli::{Cli, Commands},
    run_file, run_prompt, LoxError,
};
use simplelog::*;
use std::fs::File;
fn main() -> Result<(), LoxError> {
    let args = Cli::parse();
    let config = ConfigBuilder::new()
        .set_level_color(Level::Error, Some(Color::Rgb(191, 0, 0)))
        .set_level_color(Level::Warn, Some(Color::Rgb(255, 127, 0)))
        .set_level_color(Level::Info, Some(Color::Rgb(192, 192, 0)))
        .set_level_color(Level::Debug, Some(Color::Rgb(63, 127, 0)))
        .set_level_color(Level::Trace, Some(Color::Rgb(127, 127, 255)))
        .build();

    CombinedLogger::init(vec![
        TermLogger::new(
            args.verbose.log_level_filter(),
            config,
            TerminalMode::Stdout,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create(format!("/tmp/{:?}.txt", chrono::offset::Local::now())).unwrap(),
        ),
    ])
    .unwrap();
    if let Some(file) = args.command {
        match file {
            Commands::File { file } => run_file(file),
        }
    } else {
        run_prompt()
    }
}
