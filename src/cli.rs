use advent_of_code::{Part, day1, day2, day3, day4, define_days};
use clap::{Parser, ValueEnum};
use paste::paste;
use std::{
    error::Error,
    fmt::{Display, Write},
};

define_days! {
    One => 1
    Two => 2
    Three => 3
    Four => 4
}

#[derive(Parser)]
pub struct Cli {
    #[clap(short, long)]
    pub day: Day,
    #[clap(short, long)]
    pub part: Part,
    #[clap(short, long)]
    pub input: Input,
}

#[derive(Clone, ValueEnum)]
pub enum Input {
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
