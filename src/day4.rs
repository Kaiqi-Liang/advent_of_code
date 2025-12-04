use crate::Part;
use std::error::Error;

const NEIGHBOURS: [(i32, i32); 8] = [
    (1, 1),
    (0, 1),
    (-1, 1),
    (1, -1),
    (0, -1),
    (-1, -1),
    (1, 0),
    (-1, 0),
];

pub fn answer(input: &str, part: Part) -> Result<u128, Box<dyn Error>> {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let height = grid.len();
    let width = grid
        .first()
        .expect("Each line of input must not be empty")
        .len();
    let mut removed = true;
    let mut rolls = 0;
    while removed {
        removed = false;
        for x in 0..height {
            for y in 0..width {
                if grid[x][y] == '.' {
                    continue;
                }
                let neighbour_rolls = NEIGHBOURS
                    .iter()
                    .filter(|(delta_x, delta_y)| {
                        let x = x as i32;
                        let y = y as i32;
                        let height = height as i32;
                        let width = width as i32;
                        let new_x = delta_x + x;
                        let new_y = delta_y + y;
                        new_x >= 0
                            && new_x < height
                            && new_y >= 0
                            && new_y < width
                            && grid[new_x as usize][new_y as usize] == '@'
                    })
                    .count();
                #[cfg(debug_assertions)]
                if part == Part::One {
                    dbg!(x, y, neighbour_rolls, '-');
                }
                if neighbour_rolls < 4 {
                    rolls += 1;
                    removed = true;
                    if part == Part::Two {
                        grid[x][y] = '.';
                    }
                }
            }
        }
        if part == Part::One {
            break;
        }
    }
    Ok(rolls)
}

#[cfg(test)]
mod tests {
    use crate::{Part, day4::answer, example_challenge_2_parts_tests};

    example_challenge_2_parts_tests!(4, 13, 1549, 43, 8887);
}
