use std::collections::HashSet;

use crate::{DayResult, PuzzleInput};

/// TODO MAKE CLEANER CAUSE IT UGLY.
/// use like a struct for each present giver and make them track their own houses.
/// also find better way than hashset.

pub fn solution(mut puzzle: PuzzleInput) -> DayResult {
    let mut solo_visited_houses = HashSet::new();
    let mut duo_visited_houses = HashSet::new();
    let mut solo_x = 0;
    let mut solo_y = 0;
    let mut santa_x = 0;
    let mut santa_y = 0;
    let mut robo_x = 0;
    let mut robo_y = 0;
    let mut turn = true;

    solo_visited_houses.insert((santa_x, santa_y));
    duo_visited_houses.insert((santa_x, santa_y));

    for direction in puzzle.remove(0) {
        match direction {
            '^' => {
                solo_y += 1;
                move_duo(&mut santa_y, &mut robo_y, &mut turn, 1);
            }
            'v' => {
                solo_y -= 1;
                move_duo(&mut santa_y, &mut robo_y, &mut turn, -1);
            }
            '>' => {
                solo_x += 1;
                move_duo(&mut santa_x, &mut robo_x, &mut turn, 1);
            }
            '<' => {
                solo_x -= 1;
                move_duo(&mut santa_x, &mut robo_x, &mut turn, -1);
            }
            _ => unreachable!(),
        }

        solo_visited_houses.insert((solo_x, solo_y));
        duo_visited_houses.insert(if turn {
            (santa_x, santa_y)
        } else {
            (robo_x, robo_y)
        });
    }

    let mut result = DayResult::new();
    result.set_star1(solo_visited_houses.len().to_string());
    result.set_star2(duo_visited_houses.len().to_string());
    result
}

fn move_duo(santa: &mut i32, robo: &mut i32, turn: &mut bool, value: i32) {
    if *turn {
        *santa += value;
    } else {
        *robo += value;
    }

    *turn = !*turn;
}
