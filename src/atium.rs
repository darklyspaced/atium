use clap::Parser;
use color_eyre::{eyre::Context, Result};

use super::interpreter::interpret;
use super::parser;
use super::scanner::Scanner;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, Read},
};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Atium {
    pub script: Option<String>,
}

pub fn run_file(file: &str) -> Result<()> {
    let mut buf = String::default();
    let f_handle = File::open(file).wrap_err(format!("reading \"{file}\""))?;
    let mut f_handle = BufReader::new(f_handle);
    f_handle.read_to_string(&mut buf)?;

    run(buf);
    Ok(())
}

pub fn run_repl() -> Result<()> {
    let mut input = stdin().lock();
    let mut buf = String::new();
    while input.read_line(&mut buf)? != 0 {
        run(buf.clone())
    }
    Ok(())
}

fn run(src: String) {
    let mut scanner = Scanner::new(src);
    if let Err(e) = scanner.scan_tokens() {
        println!("{}", e);
    }
    let mut parser = parser::Parser::new(scanner.tokens.clone());
    let errs = interpret(parser.parse().unwrap());
    if !errs.is_empty() {
        for err in errs {
            println!("{err}");
        }
    }
}
