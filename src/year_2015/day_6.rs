use crate::{DayResult, PuzzleInput};

pub fn solution(puzzle: PuzzleInput) -> DayResult {
    let mut bool_state = vec![vec![false; 1000]; 1000];
    let mut variable_state = vec![vec![0u64; 1000]; 1000];
    for instruction in puzzle {
        let instruction = instruction.iter().collect::<String>();
        // let instruction = String::from("turn on 0,0 t 999,999");
        let mut instructions = instruction.split_whitespace().collect::<Vec<&str>>();

        if instructions[0] == "turn" {
            instructions.remove(0);
        }

        let bool_operation = match *instructions.first().unwrap() {
            "on" => |_| true,
            "off" => |_| false,
            "toggle" => |current_state: bool| !current_state,
            _ => unreachable!(),
        };
        let variable_operation = match *instructions.first().unwrap() {
            "on" => |current_value: u64| current_value + 1,
            "off" => |current_value| std::cmp::max(current_value, 1) - 1, // Prevent going to negatives.
            "toggle" => |current_state| current_state + 2,
            _ => unreachable!(),
        };

        let start_coord = parse_coords(instructions[1]);
        let end_coord = parse_coords(instructions[3]);

        // For bool_state lights
        update_lights(&start_coord, &end_coord, &bool_operation, &mut bool_state);

        // For variable_state lights
        update_lights(
            &start_coord,
            &end_coord,
            &variable_operation,
            &mut variable_state,
        );
    }

    let total_bool_lights = total_lights_active(bool_state);
    let total_variable_lights = total_lights_active(variable_state);

    let mut result = DayResult::new();
    result.set_star1(total_bool_lights.to_string());
    result.set_star2(total_variable_lights.to_string());
    result
}

fn parse_coords(input: &str) -> (usize, usize) {
    let coords = input
        .split(',')
        .map(|value| str::parse(value).unwrap())
        .collect::<Vec<usize>>();
    (coords[0], coords[1])
}

fn update_lights<T: Copy>(
    start_coords: &(usize, usize),
    end_coords: &(usize, usize),
    operation: &dyn Fn(T) -> T,
    state: &mut Vec<Vec<T>>,
) {
    for cols in &mut state.as_mut_slice()[start_coords.0..=end_coords.0] {
        for light_state in &mut cols.as_mut_slice()[start_coords.1..=end_coords.1] {
            *light_state = operation(*light_state);
        }
    }
}

fn total_lights_active<T: Copy>(state: Vec<Vec<T>>) -> u128
where
    u128: From<T>,
{
    state
        .into_iter()
        .flat_map(|x| x.into_iter().map(|value| u128::from(value)))
        .sum()
}
