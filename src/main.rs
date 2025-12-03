mod cli;
use clap::Parser;
use cli::{Cli, run};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    run(cli)?;
    Ok(())
}
