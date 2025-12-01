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
                (curr.abs() as f64 / DIAL_SIZE as f64) as i32
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
        dbg!(prev, direction, distance, curr, password, '-');
        prev = curr;
    }
    Ok(password)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::{Part, day1::answer};

    const EXAMPLE_INPUT: &str = r#"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"#;

    #[test]
    fn part1_example() {
        assert_eq!(answer(EXAMPLE_INPUT, Part::One).unwrap(), 3);
    }

    #[test]
    fn part1_challenge() {
        let challenge_input: &str = &read_to_string("input/1.1").unwrap();
        assert_eq!(answer(challenge_input, Part::One).unwrap(), 1052);
    }

    #[test]
    fn part2_example() {
        assert_eq!(answer(EXAMPLE_INPUT, Part::Two).unwrap(), 6);
    }

    #[test]
    fn part2_distance_over_dial_size() {
        const INPUT: &str = r#"
R1000
L50
R50
R50
R50
L268
R18
R200
L300"#;
        assert_eq!(answer(INPUT, Part::Two).unwrap(), 21);
    }
}
