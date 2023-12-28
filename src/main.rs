use std::collections::HashMap;

const START_YEAR: u32 = 2015;

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
        .ok_or("The solutions for event {year} have not been completed")?;

    Ok(())
}

fn parse_args() -> Result<(u32, u32), &'static str> {
    // Ignore 0th arg as it is location.
    let year: String = std::env::args().nth(1).ok_or("No year passed")?;
    let day: String = std::env::args().nth(2).ok_or("No day passed")?;
    println!("Year passed, {year}");
    println!("Day passed, {day}");
    Ok((
        year.parse().or(Err("Failed to parse year"))?,
        day.parse().or(Err("Failed to parse day"))?,
    ))
}

type Day = &'static dyn FnOnce() -> DayResult;

struct DayResult {
    star1: String,
    star2: String,
}

struct Event {
    days: [Option<Day>; 25],
}
