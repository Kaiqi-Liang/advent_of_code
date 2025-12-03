use std::error::Error;

use crate::Part;

pub fn answer(input: &str, part: Part) -> Result<u8, Box<dyn Error>> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::{Part, day3::answer, example_challenge_2_parts_tests};

    example_challenge_2_parts_tests!(3, 357, 0, 0, 0);
}
