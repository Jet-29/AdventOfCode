use crate::{DayResult, PuzzleInput};

pub fn solution(mut puzzle: PuzzleInput) -> DayResult {
    let mut floor: i32 = 0;
    let mut first_basement_idx = 0;
    for (idx, instruction) in puzzle.remove(0).iter().enumerate() {
        match instruction {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => unreachable!(),
        }
        if floor < 0 && first_basement_idx == 0 {
            // Plus one as the first instruction is instruction 1.
            first_basement_idx = idx + 1;
        }
    }

    let mut result = DayResult::new();
    result.set_star1(floor.to_string());
    result.set_star2(first_basement_idx.to_string());
    result
}
