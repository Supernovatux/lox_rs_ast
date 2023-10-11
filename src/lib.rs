use rustyline::DefaultEditor;
use scanner::Scanner;
use std::path::PathBuf;

pub mod cli;
pub mod scanner;
pub mod tokens;
pub fn run_file(file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Read the file and run
    let contents = std::fs::read_to_string(file)?;
    run(contents)
}
pub fn run_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline(">> ");
        let line = readline?;
        rl.add_history_entry(line.as_str())?;
        println!("Line: {}", line);
        run(line)?;
    }
}
fn run(source: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut scanner = Scanner::new(source.as_str());
    let tokens = scanner.scan_tokens()?;
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}
