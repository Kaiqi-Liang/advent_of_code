mod cli;
use clap::Parser;
use cli::Cli;
use std::{error::Error, fs::read_to_string, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let input = read_to_string(
        Path::new("input").join(cli.day.to_string() + "." + &cli.input.to_string()),
    )?;
    cli.day.run(&input, cli.part)?;
    Ok(())
}
