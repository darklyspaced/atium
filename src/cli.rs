use clap::Parser;
use color_eyre::{eyre::Context, Report, Result};

use crate::atium::Atium;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, Read},
};

/// The outward facing CLI for client that reports errors, handles input and more
///
/// This CLI passes all input to [`Atium`] which handles the internal logic and in turn gets errors
/// from it and pretty prints them out.
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    pub script: Option<String>,
}

/// Reads source code from file
///
/// # Errors
/// 1. Errors when it fails to read the provided file
pub fn run_file(file: &str) -> Result<()> {
    let mut buf = String::default();
    let f_handle = File::open(file).wrap_err(format!("reading \"{file}\""))?;
    let mut f_handle = BufReader::new(f_handle);
    f_handle.read_to_string(&mut buf)?;

    if let Err(errs) = run(&buf) {
        report(&errs);
    }
    Ok(())
}

fn report(errors: &[Report]) {
    // TODO: buffer the error output
    for err in errors {
        println!("{err}");
    }
}

/// Reads source code line by line, as user enters it
///
/// # Errors
/// 1. Errors when it fails to read a line
pub fn run_repl() -> Result<()> {
    let mut input = stdin().lock();
    let mut buf = String::new();
    while input.read_line(&mut buf)? != 0 {
        if let Err(errs) = run(&buf) {
            report(&errs);
        }
        buf.clear();
    }
    Ok(())
}

fn run(src: &str) -> Result<(), Vec<Report>> {
    let atium = Atium::new(src);
    atium.lex()?.parse()?.interpret()?;
    Ok(())
}
