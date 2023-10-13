use rustyline::DefaultEditor;
use scanner::Scanner;
use std::path::PathBuf;
use thiserror::Error;

pub mod ast;
pub mod cli;
pub mod interpreter;
pub mod parser;
pub mod scanner;
pub mod tokens;
#[derive(Error, Debug)]
pub enum LoxError {
    #[error("{0}")]
    ParseError(#[from] parser::ParseError),
    #[error("{0}")]
    InterpreterError(#[from] interpreter::InterpreterError),
    #[error("{0}")]
    ScanError(#[from] scanner::ScanError),
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    #[error("{0}")]
    ReadlineError(#[from] rustyline::error::ReadlineError),
}

pub fn run_file(file: PathBuf) -> Result<(), LoxError> {
    // Read the file and run
    let contents = std::fs::read_to_string(file)?;
    run(contents)
}
pub fn run_prompt() -> Result<(), LoxError> {
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline(">> ");
        let line = readline?;
        rl.add_history_entry(line.as_str())?;
        match run(line) {
            Ok(_) => (),
            Err(e) => {
                log::error!("{}", e);
            }
        }
    }
}
fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source.as_str());
    let tokens = scanner.scan_tokens()?;
    let mut parser = parser::Parser::new(tokens);
    let stmt = parser.parse()?;
    //println!("{:?}", stmt);
    let mut interpreter = interpreter::Interpreter::new();
    match interpreter.interpret(stmt) {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("{}", e);
            Ok(())
        }
    }
}
