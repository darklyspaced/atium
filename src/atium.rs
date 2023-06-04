use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, Read},
};

use clap::Parser;
use color_eyre::{eyre::Context, Result};

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
    loop {
        print!("> ");
        for line in stdin().lock().lines() {
            run(line?);
        }
    }
}

fn run(program: String) {
    println!("{program}");
    todo!("implement the run function")
}

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line: {}] Error{}: {}", line, location, message);
}
