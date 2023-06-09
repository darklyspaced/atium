use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, Read},
};

use clap::Parser;
use color_eyre::{eyre::Context, Result};

use super::scanner::Scanner;

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

fn run(program: String) {
    println!("bye");
    let mut scanner = Scanner::new(program);
    scanner.scan_tokens();
    println!("{:#?}", scanner.tokens);
}

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line: {}] Error{}: {}", line, location, message);
}
