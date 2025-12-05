use crate::Part;
use std::error::Error;

enum State {
    Range,
    ID,
}

pub fn answer(input: &str, part: Part) -> Result<u128, Box<dyn Error>> {
    let mut available_fresh_ingredients = 0;
    let mut ranges = Vec::new();
    let mut state = State::Range;
    for line in input.lines() {
        if line.is_empty() {
            state = State::ID;
            if part == Part::Two {
                break;
            }
        } else {
            match state {
                State::Range => {
                    let range = line
                        .split('-')
                        .map(|num| num.parse::<u128>())
                        .collect::<Result<Vec<_>, _>>()?;
                    let (start, end) = (range[0], range[1]);
                    ranges.push((start, end))
                }
                State::ID => {
                    let id = line.parse::<u128>()?;
                    for &(start, end) in ranges.iter() {
                        if id >= start && id <= end {
                            available_fresh_ingredients += 1;
                            break;
                        }
                    }
                }
            }
        }
    }
    if part == Part::Two {
        ranges.sort_by_key(|&(start, _)| start);
        let mut i = 0;
        let mut last_range_overlap = false;
        while i < ranges.len() - 1 {
            let (start, mut end) = (ranges[i].0, ranges[i].1);
            for j in i + 1..ranges.len() {
                i = j;
                if end >= ranges[j].0 {
                    end = end.max(ranges[j].1);
                    if i == ranges.len() - 1 {
                        last_range_overlap = true;
                    }
                } else {
                    #[cfg(debug_assertions)]
                    dbg!(j, end, start, '-');
                    break;
                }
            }
            available_fresh_ingredients += end - start + 1;
        }
        if !last_range_overlap {
            available_fresh_ingredients += ranges[i].1 - ranges[i].0 + 1;
        }
    }
    Ok(available_fresh_ingredients)
}

#[cfg(test)]
mod tests {
    use crate::{Part, day5::answer, example_challenge_2_parts_tests};

    example_challenge_2_parts_tests!(5, 3, 885, 14, 348115621205535);

    #[test]
    fn part2_last_range_no_overlap() {
        assert_eq!(answer("1-2\n2-10\n9-11\n14-15", Part::Two).unwrap(), 13);
    }
}
