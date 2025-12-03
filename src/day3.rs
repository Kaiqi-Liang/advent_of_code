use crate::Part;
use std::error::Error;

pub fn answer(input: &str, part: Part) -> Result<u128, Box<dyn Error>> {
    let mut total = 0;
    for bank in input.lines() {
        let ratings = bank
            .chars()
            .map(|rating| {
                rating
                    .to_digit(10)
                    .expect("Each rating must be a value from 1 to 9")
            })
            .collect::<Vec<_>>();
        let mut largest = String::new();
        let batteries = match part {
            Part::One => 2,
            Part::Two => 12,
        };
        let mut highest_rating_index = 0;
        for i in 0..batteries {
            let highest_rating = ratings[highest_rating_index..ratings.len() - (batteries - i - 1)]
                .iter()
                .max()
                .expect("Each line of ratings must not be empty");
            for (j, rating) in ratings.iter().enumerate() {
                if j >= highest_rating_index && rating == highest_rating {
                    highest_rating_index = j + 1;
                    break;
                }
            }
            largest += &highest_rating.to_string();
            dbg!(highest_rating, highest_rating_index);
        }
        #[cfg(debug_assertions)]
        dbg!(total, &largest, '-');
        total += largest.parse::<u128>()?;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use crate::{Part, day3::answer, example_challenge_2_parts_tests};

    example_challenge_2_parts_tests!(3, 357, 17100, 3121910778619, 170418192256861);
}
