use std::error::Error;

use crate::Part;

const DIAL_SIZE: i32 = 100;

#[derive(PartialEq, Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Each line of input must start with a valid rotation"),
        }
    }
}

pub fn answer(input: &str, part: Part) -> Result<i32, Box<dyn Error>> {
    let mut curr = 50;
    let mut prev = curr;
    let mut password = 0;
    for rotation in input.split_whitespace() {
        let direction = std::convert::Into::<Direction>::into(
            rotation
                .chars()
                .next()
                .expect("Each line of input must start with a character"),
        );
        let distance = rotation[1..].parse::<i32>()?;
        match direction {
            Direction::Left => {
                curr -= distance;
            }
            Direction::Right => {
                curr += distance;
            }
        }
        if part == Part::Two {
            password += if curr > 0 {
                curr / DIAL_SIZE
            } else if curr < 0 {
                if prev == 0 && curr.abs() < DIAL_SIZE {
                    0
                } else {
                    (curr.abs() as f64 / DIAL_SIZE as f64).ceil() as i32
                }
            } else if curr == 0 {
                1
            } else {
                todo!()
            };
        }
        curr = curr.rem_euclid(DIAL_SIZE);
        if part == Part::One && curr == 0 {
            password += 1;
        }
        #[cfg(debug_assertions)]
        dbg!(prev, direction, distance, curr, password, '-');
        prev = curr;
    }
    Ok(password)
}

#[cfg(test)]
mod tests {
    use crate::{Part, day1::answer};

    const EXAMPLE_INPUT: &str = include_str!("../input/1.0");
    const CHALLENGE_INPUT: &str = include_str!("../input/1.1");

    #[test]
    fn part1_example() {
        assert_eq!(answer(EXAMPLE_INPUT, Part::One).unwrap(), 3);
    }

    #[test]
    fn part1_challenge() {
        assert_eq!(answer(CHALLENGE_INPUT, Part::One).unwrap(), 1052);
    }

    #[test]
    fn part2_example() {
        assert_eq!(answer(EXAMPLE_INPUT, Part::Two).unwrap(), 6);
    }

    #[test]
    fn part2_distance_over_dial_size() {
        assert_eq!(
            answer(
                r#"
R1000
L50
R50
R50
R50
L268
R18
R200
L300
L99"#,
                Part::Two
            )
            .unwrap(),
            21
        );
    }
}
