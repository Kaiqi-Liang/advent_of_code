use advent_of_code::{Day, Part};
use clap::{Parser, ValueEnum};
use std::{error::Error, fs::read_to_string, path::Path};

#[derive(Parser)]
pub struct Cli {
    /// Day to run
    #[clap(short, long)]
    day: Day,

    /// Part to run
    #[clap(short, long)]
    part: Part,

	/// Input to use
    #[clap(short, long)]
    input: Input,
}

#[derive(Clone, ValueEnum, strum::Display)]
#[strum(serialize_all = "lowercase")]
enum Input {
    #[value(alias = "eg")]
    Example,
    #[value(alias = "ch")]
    Challenge,
}

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    cli.day.run(
        &read_to_string(
            Path::new("input").join(cli.day.to_string() + "." + &cli.input.to_string()),
        )?,
        cli.part,
    )
}
