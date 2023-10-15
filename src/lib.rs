use interpreter::Interpreter;
use rustyline::DefaultEditor;
use scanner::Scanner;
use std::path::PathBuf;
use thiserror::Error;

pub mod ast;
pub mod cli;
pub mod environment;
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
    run(contents, None).map(|_| ())
}
pub fn run_prompt() -> Result<(), LoxError> {
    //println!("{:?}", stmt);
    let mut interpreter = interpreter::Interpreter::new();
    let mut rl = DefaultEditor::new()?;
    loop {
        let readline = rl.readline(">> ");
        let line = readline?;
        rl.add_history_entry(line.as_str())?;
        let mut scanner = Scanner::new(line.as_str());
        let tokens = scanner.scan_tokens()?;
        let mut parser = parser::Parser::new(tokens);
        let stmt = parser.parse()?;
        println!("{:?}", stmt);
        match interpreter.interpret(stmt) {
            Ok(_) => (),
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
fn run(source: String, interpreter: Option<Interpreter>) -> Result<Interpreter, LoxError> {
    let mut scanner = Scanner::new(source.as_str());
    let tokens = scanner.scan_tokens()?;
    let mut parser = parser::Parser::new(tokens);
    let stmt = parser.parse()?;
    //println!("{:?}", stmt);
    let mut interpreter = match interpreter {
        Some(i) => i,
        None => interpreter::Interpreter::new(),
    };
    match interpreter.interpret(stmt) {
        Ok(_) => Ok(interpreter),
        Err(e) => {
            eprintln!("{}", e);
            Ok(interpreter)
        }
    }
}
