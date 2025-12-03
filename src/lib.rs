use clap::ValueEnum;
use paste::paste;
use std::{error::Error, fmt::Display};

macro_rules! define_days {
    ($($name:ident => $num:literal,)*) => {
        $(
            paste! { pub mod [<day $num>]; }
        )*

        #[derive(Clone, ValueEnum)]
        pub enum Day {
            $(
                #[value(name = stringify!($num))]
                $name,
            )*
        }

        impl Display for Day {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Day::$name => f.write_str(stringify!($num)),
                    )*
                }
            }
        }

        impl Day {
            pub fn run(&self, input: &str, part: Part) -> Result<(), Box<dyn Error>> {
                match self {
                    $(
                        Day::$name => println!("{}", paste! { [<day $num>]::answer(input, part)? }),
                    )*
                }
                Ok(())
            }
        }
    };
}

define_days! {
    One => 1,
    Two => 2,
    Three => 3,
    Four => 4,
}

#[derive(Clone, ValueEnum, PartialEq)]
pub enum Part {
    #[value(name = "1")]
    One,
    #[value(name = "2")]
    Two,
}

#[cfg(test)]
#[macro_export]
macro_rules! example_challenge_2_parts_tests {
    ($day:literal, $part1_example_answer:literal, $part1_challenge_answer:literal, $part2_example_answer:literal, $part2_challenge_answer:literal) => {
        const EXAMPLE_INPUT: &str = include_str!(concat!("../input/", $day, ".example"));
        const CHALLENGE_INPUT: &str = include_str!(concat!("../input/", $day, ".challenge"));

        #[test]
        fn part1_example() {
            assert_eq!(
                answer(EXAMPLE_INPUT, Part::One).unwrap(),
                $part1_example_answer
            );
        }

        #[test]
        fn part1_challenge() {
            assert_eq!(
                answer(CHALLENGE_INPUT, Part::One).unwrap(),
                $part1_challenge_answer
            );
        }

        #[test]
        fn part2_example() {
            assert_eq!(
                answer(EXAMPLE_INPUT, Part::Two).unwrap(),
                $part2_example_answer
            );
        }

        #[test]
        fn part2_challenge() {
            assert_eq!(
                answer(CHALLENGE_INPUT, Part::Two).unwrap(),
                $part2_challenge_answer
            );
        }
    };
}
