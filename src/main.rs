use std::collections::HashMap;
use util_macros::get_solutions;
mod utils;

fn main() -> Result<(), &'static str> {
    get_solutions!();

    println!("{}", solutions.len());

    let (year, day) = parse_args()?;
    println!("You chose year {year}, puzzle {day}");
    let event = solutions
        .get(&year)
        .ok_or("The solutions for that event have not been completed")?;

    let solution = event
        .get_day(day)
        .ok_or("The solution to that day hasn't been completed")?;

    let puzzle_input: PuzzleInput = utils::load_puzzle_as_chars(&format!("{year}/day{day}"))
        .or(Err("Failed to find the puzzle file"))?;

    // Time it
    let start = std::time::Instant::now();

    let result = solution(puzzle_input);

    let end_time = start.elapsed();

    println!("The solution to Event: {year}, Day: {day} is");
    let DayResult { star1, star2 } = result;
    println!("Star 1: {star1}");
    println!("Star 2: {star2}");
    println!("The solution took {end_time:.3?}");
    Ok(())
}

fn parse_args() -> Result<(u32, u32), &'static str> {
    // Ignore 0th arg as it is location.
    let year: String = std::env::args().nth(1).ok_or("No year passed")?;
    let day: String = std::env::args().nth(2).ok_or("No day passed")?;
    Ok((
        year.parse().or(Err("Failed to parse year"))?,
        day.parse().or(Err("Failed to parse day"))?,
    ))
}

type Day = &'static dyn Fn(PuzzleInput) -> DayResult;
type PuzzleInput = Vec<Vec<char>>;

#[derive(Default)]
struct DayResult {
    star1: String,
    star2: String,
}

impl DayResult {
    fn new() -> Self {
        Self::default()
    }

    fn set_star1(&mut self, solution: String) {
        self.star1 = solution;
    }

    fn set_star2(&mut self, solution: String) {
        self.star2 = solution;
    }
}

struct Event {
    days: HashMap<u32, Day>,
}

impl Event {
    fn get_day(&self, day: u32) -> Option<&Day> {
        self.days.get(&day)
    }
}
