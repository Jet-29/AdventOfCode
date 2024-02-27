use std::collections::HashMap;

mod utils;

mod year_2015;
mod year_2016;
mod year_2017;
mod year_2018;
mod year_2019;
mod year_2020;
mod year_2021;
mod year_2022;
mod year_2023;

fn main() -> Result<(), &'static str> {
    // Grab all solutions
    let mut solutions: HashMap<u32, Event> = HashMap::new();
    solutions.insert(2015, year_2015::get_event());

    let (year, day) = parse_args()?;
    println!("You chose year {year}, puzzle {day}");
    let event = solutions
        .get(&year)
        .ok_or("The solutions for that event have not been completed")?;

    let solution = event
        .get_day(day as usize)?
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
    days: [Option<Day>; 25],
}

impl Event {
    fn get_day(&self, day: usize) -> Result<Option<Day>, &'static str> {
        if day == 0 || day > 25 {
            Err("The day entered must be 1-25")
        } else {
            Ok(self.days[day - 1])
        }
    }
}
