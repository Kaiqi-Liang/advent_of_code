use clap::ValueEnum;

pub mod day1;

#[derive(Clone, ValueEnum, PartialEq)]
pub enum Part {
    One,
    Two,
}
