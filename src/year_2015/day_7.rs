use std::collections::HashMap;

use crate::{DayResult, PuzzleInput};

pub fn solution(puzzle: PuzzleInput) -> DayResult {
    let puzzle_strings = puzzle
        .into_iter()
        .map(|gate| gate.iter().collect::<String>())
        .collect::<Vec<String>>();

    let puzzle_split_strings = puzzle_strings
        .iter()
        .map(|gate| gate.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut instructions: HashMap<&str, WireInput> = HashMap::new();

    for puzzle_instruction in puzzle_split_strings {
        if puzzle_instruction.len() == 3 {
            if let Ok(value) = puzzle_instruction[0].trim().parse::<u16>() {
                instructions.insert(puzzle_instruction[2], WireInput::Value(value));
            } else {
                instructions.insert(
                    puzzle_instruction[2],
                    WireInput::NoOp(puzzle_instruction[0]),
                );
            }
        } else if puzzle_instruction.len() == 4 {
            instructions.insert(puzzle_instruction[3], WireInput::Not(puzzle_instruction[1]));
        } else {
            match *puzzle_instruction.get(1).unwrap() {
                "RSHIFT" => instructions.insert(
                    puzzle_instruction[4],
                    WireInput::RShift(puzzle_instruction[0], puzzle_instruction[2]),
                ),
                "LSHIFT" => instructions.insert(
                    puzzle_instruction[4],
                    WireInput::LShift(puzzle_instruction[0], puzzle_instruction[2]),
                ),
                "OR" => instructions.insert(
                    puzzle_instruction[4],
                    WireInput::Or(puzzle_instruction[0], puzzle_instruction[2]),
                ),
                "AND" => instructions.insert(
                    puzzle_instruction[4],
                    WireInput::And(puzzle_instruction[0], puzzle_instruction[2]),
                ),

                _ => unreachable!(),
            };
        }
    }

    let a_value = recursive_calculate(&mut instructions, "a");

    let mut result = DayResult::new();
    result.set_star2(a_value.to_string());
    result
}

enum WireInput<'a> {
    RShift(&'a str, &'a str),
    LShift(&'a str, &'a str),
    Or(&'a str, &'a str),
    And(&'a str, &'a str),
    Not(&'a str),
    Value(u16),
    NoOp(&'a str),
}

fn recursive_calculate<'a>(
    instructions: &mut HashMap<&'a str, WireInput<'a>>,
    wire: &'a str,
) -> u16 {
    if let Ok(value) = wire.trim().parse() {
        return value;
    }
    let current_wire = instructions.remove(wire).unwrap();
    let result = match current_wire {
        WireInput::RShift(wire1, wire2) => {
            recursive_calculate(instructions, wire1) >> recursive_calculate(instructions, wire2)
        }
        WireInput::LShift(wire1, wire2) => {
            recursive_calculate(instructions, wire1) << recursive_calculate(instructions, wire2)
        }
        WireInput::Or(wire1, wire2) => {
            recursive_calculate(instructions, wire1) | recursive_calculate(instructions, wire2)
        }
        WireInput::And(wire1, wire2) => {
            recursive_calculate(instructions, wire1) & recursive_calculate(instructions, wire2)
        }
        WireInput::Not(wire) => !recursive_calculate(instructions, wire),
        WireInput::Value(value) => value,
        WireInput::NoOp(wire) => recursive_calculate(instructions, wire),
    };
    instructions.insert(wire, WireInput::Value(result));
    result
}
