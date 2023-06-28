use atium::atium::{run_file, run_repl, Atium};
use clap::Parser;
use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Atium::parse();
    if let Some(file) = cli.script {
        run_file(&file)?;
    } else {
        run_repl()?;
    }

    Ok(())
}
