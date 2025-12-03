use clap::ValueEnum;
mod day;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

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
        const EXAMPLE_INPUT: &str = include_str!(concat!("../input/", $day, ".0"));
        const CHALLENGE_INPUT: &str = include_str!(concat!("../input/", $day, ".1"));

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
