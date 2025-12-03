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
            password += match curr.cmp(&0) {
                std::cmp::Ordering::Less => curr.abs() / DIAL_SIZE + if prev > 0 { 1 } else { 0 },
                std::cmp::Ordering::Equal => 1,
                std::cmp::Ordering::Greater => curr / DIAL_SIZE,
            }
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
    use crate::{Part, day1::answer, example_challenge_2_parts_tests};

    example_challenge_2_parts_tests!(1, 3, 1052, 6, 6295);

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
L150
L49"#,
                Part::Two
            )
            .unwrap(),
            22
        );
    }
}
