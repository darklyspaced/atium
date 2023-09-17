use clap::Parser;
use color_eyre::{eyre::Context, Report, Result};

use crate::atium::Atium;
use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, Read},
};

/// The outward facing CLI that handles command line input
///
/// This CLI passes all input to [`Atium`] which handles the internal logic
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    pub script: Option<String>,
    #[arg(long)]
    pub ast: bool,
}

/// Reads source code from file
pub fn run_file(file: &str) -> Result<()> {
    let mut buf = String::default();
    let f_handle = File::open(file).wrap_err(format!("reading \"{file}\""))?;
    let mut f_handle = BufReader::new(f_handle);
    f_handle.read_to_string(&mut buf)?;

    if let Err(errs) = run(&buf, Some(file)) {
        report(&errs);
    }
    Ok(())
}

fn report(errors: &[Report]) {
    println!("there was an error!");
    for err in errors {
        eprintln!("{err}");
    }
}

/// Reads source code line by line, as user enters it
pub fn run_repl() -> Result<()> {
    let mut input = stdin().lock();
    let mut buf = String::new();
    while input.read_line(&mut buf)? != 0 {
        if let Err(errs) = run(&buf, None) {
            report(&errs);
        }
        buf.clear();
    }
    Ok(())
}

fn run(src: &str, file: Option<&str>) -> Result<(), Vec<Report>> {
    let atium = Atium::new(src, file);
    atium.lex()?.parse()?.interpret()?;
    Ok(())
}
