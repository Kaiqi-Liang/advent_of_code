use clap::ValueEnum;

pub mod day1;
pub mod day2;

#[derive(Clone, ValueEnum, PartialEq)]
pub enum Part {
    One,
    Two,
}
