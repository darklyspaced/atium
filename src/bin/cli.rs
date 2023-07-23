use atium::atium::{run_file, run_repl, Cli};
use clap::Parser;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();
    if let Some(file) = cli.script {
        run_file(&file)?;
    } else {
        run_repl()?;
    }

    Ok(())
}
