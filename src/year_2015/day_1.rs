use crate::{utils::load_puzzle_as_chars, DayResult};

pub fn solution() -> Result<DayResult, &'static str> {
    let puzzle = load_puzzle_as_chars("2015/day1")?;
    Ok(DayResult {
        star1: 7.to_string(),
        star2: 8.to_string(),
    })
}
