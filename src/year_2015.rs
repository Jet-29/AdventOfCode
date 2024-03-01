use crate::{Day, Event};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

pub fn get_event() -> Event {
    let solutions = [
        &day_1::solution as Day,
        &day_2::solution as Day,
        &day_3::solution as Day,
        &day_4::solution as Day,
        &day_5::solution as Day,
        &day_6::solution as Day,
    ];

    let mut days = [None; 25];
    days[..solutions.len()].copy_from_slice(&solutions.map(Some)[..]);
    Event { days }
}
