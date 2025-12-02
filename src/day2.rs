use std::{collections::HashSet, error::Error};

use crate::Part;

pub fn answer(input: &str, part: Part) -> Result<u128, Box<dyn Error>> {
    let mut sum = 0;
    let mut invalid_ids = HashSet::new();
    for range in input.split(',') {
        let range = range
            .split('-')
            .map(|num| num.parse::<u128>())
            .collect::<Result<Vec<_>, _>>()?;
        let (start, end) = (range[0], range[1]);
        for num in start..=end {
            let string = num.to_string();
            match part {
                Part::One => {
                    let mut left = string;
                    if left.len() % 2 == 0 {
                        let right = left.split_off(left.len() / 2);
                        if left == right {
                            sum += num;
                        }
                    }
                }
                Part::Two => {
                    for repeated_size in 1..=string.len() / 2 {
                        if string.len() % repeated_size == 0 {
                            let mut chunks = Vec::new();
                            for chunk in 0..string.len() / repeated_size {
                                chunks.push(
                                    &string[chunk * repeated_size..(chunk + 1) * repeated_size],
                                );
                            }
                            if chunks.iter().collect::<HashSet<_>>().len() == 1 {
                                if !invalid_ids.contains(&num) {
                                    sum += num;
                                }
                                invalid_ids.insert(num);
                                #[cfg(debug_assertions)]
                                dbg!(num, sum, '-');
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use crate::{Part, day2::answer};

    const EXAMPLE_INPUT: &str = include_str!("../input/2.0");
    const CHALLENGE_INPUT: &str = include_str!("../input/2.1");

    #[test]
    fn part1_example() {
        assert_eq!(answer(EXAMPLE_INPUT, Part::One).unwrap(), 1227775554);
    }

    #[test]
    fn part1_challenge() {
        assert_eq!(answer(CHALLENGE_INPUT, Part::One).unwrap(), 40055209690);
    }

    #[test]
    fn part2_example() {
        assert_eq!(answer(EXAMPLE_INPUT, Part::Two).unwrap(), 4174379265);
    }

    #[test]
    fn part2_challenge() {
        assert_eq!(answer(CHALLENGE_INPUT, Part::Two).unwrap(), 40055209690);
    }

    #[test]
    fn part2_odd_number_of_digits() {
        assert_eq!(answer("100000000-300000000", Part::Two).unwrap(), 21);
    }
}
