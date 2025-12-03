use advent_of_code::{Day, Part};
use clap::{Parser, ValueEnum};
use std::{
    error::Error,
    fmt::{Display, Write},
    fs::read_to_string,
    path::Path,
};

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    day: Day,
    #[clap(short, long)]
    part: Part,
    #[clap(short, long)]
    input: Input,
}

#[derive(Clone, ValueEnum)]
enum Input {
    #[value(alias = "eg")]
    Example,
    #[value(alias = "ch")]
    Challenge,
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::Example => f.write_char('0'),
            Input::Challenge => f.write_char('1'),
        }
    }
}

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    cli.day.run(
        &read_to_string(
            Path::new("input").join(cli.day.to_string() + "." + &cli.input.to_string()),
        )?,
        cli.part,
    )
}
