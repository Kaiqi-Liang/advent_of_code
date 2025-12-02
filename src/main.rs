use advent_of_code::{Part, day1, day2};
use std::{
    error::Error,
    fmt::{Display, Write},
    fs::read_to_string,
    path::Path,
};

use clap::{Parser, ValueEnum};

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    day: Day,
    #[clap(short, long)]
    part: Part,
    #[clap(short, long)]
    input: Input,
}

#[derive(Clone, ValueEnum)]
enum Day {
    One,
    Two,
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Day::One => f.write_char('1'),
            Day::Two => f.write_char('2'),
        }
    }
}

#[derive(Clone, ValueEnum)]
enum Input {
    Example,
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

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let input = read_to_string(
        Path::new("input").join(cli.day.to_string() + "." + &cli.input.to_string()),
    )?;
    match cli.day {
        Day::One => println!("{}", day1::answer(&input, cli.part)?),
        Day::Two => println!("{}", day2::answer(&input, cli.part)?),
    };
    Ok(())
}
