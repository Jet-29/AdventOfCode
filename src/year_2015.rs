use crate::{Day, Event};

mod day_1;

pub fn get_event() -> Event {
    let solutions = [&day_1::solution as Day];

    let mut days = [None; 25];
    days[..solutions.len()].copy_from_slice(&solutions.map(Some)[..]);
    Event { days }
}
