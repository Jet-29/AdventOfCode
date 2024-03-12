use crate::{DayResult, PuzzleInput};

pub fn solution(puzzle: PuzzleInput) -> DayResult {
    let mut characters_replaced = 0;
    let mut characters_added = 0;

    for code in puzzle {
        characters_replaced += 2;
        characters_added += 4;

        let mut idx = 1;
        while idx < code.len() - 1 {
            if code[idx] != '\\' {
                idx += 1;
                continue;
            }

            match *code.get(idx + 1).unwrap() {
                '\\' | '"' => {
                    characters_replaced += 1;
                    characters_added += 2;
                    idx += 2;
                }
                'x' => {
                    characters_replaced += 3;
                    characters_added += 1;
                    idx += 4;
                }
                _ => unreachable!(),
            }
        }
    }

    let mut result = DayResult::new();
    result.set_star1(characters_replaced.to_string());
    result.set_star2(characters_added.to_string());
    result
}
