use crate::{DayResult, PuzzleInput};

const BAD_PAIRS: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub fn solution(puzzle: PuzzleInput) -> DayResult {
    let mut old_nice_strings: u32 = 0;
    let mut new_nice_strings: u32 = 0;

    for possible_string in puzzle {
        let collected_possible_string = possible_string.iter().collect::<String>();
        let mut vowels: u32 = 0;
        let mut has_double_letter = false;
        let mut has_bad_pair = false;
        let mut has_duplicate_pair = false;
        let mut has_repeat_letter = false;

        let mut previous_letter: Option<&char> = None;
        let mut prev_prev_letter: Option<&char> = None;

        for letter in &possible_string {
            match letter {
                'a' | 'e' | 'i' | 'o' | 'u' => vowels += 1,
                _ => {}
            }

            if let Some(previous_letter) = previous_letter {
                has_double_letter |= previous_letter == letter;

                let sub_str = format!("{previous_letter}{letter}");
                if collected_possible_string.matches(&sub_str).count() >= 2 {
                    has_duplicate_pair = true;
                }
            }

            if let Some(prev_prev_letter) = prev_prev_letter {
                has_repeat_letter |= prev_prev_letter == letter;
            }
            prev_prev_letter = previous_letter;
            previous_letter = Some(letter);
        }

        for bad_pair in BAD_PAIRS {
            if collected_possible_string.contains(bad_pair) {
                has_bad_pair = true;
                break;
            }
        }
        if vowels >= 3 && has_double_letter && !has_bad_pair {
            old_nice_strings += 1;
        }
        if has_duplicate_pair && has_repeat_letter {
            new_nice_strings += 1;
        }
    }

    let mut result = DayResult::new();
    result.set_star1(old_nice_strings.to_string());
    result.set_star2(new_nice_strings.to_string());
    result
}
