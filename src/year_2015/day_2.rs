use crate::{DayResult, PuzzleInput};

pub fn solution(puzzle: PuzzleInput) -> DayResult {
    let mut total_paper = 0u64;
    let mut total_ribbon = 0u64;
    for dims in puzzle {
        let present = Present::from_chars(&dims);
        total_paper += present.calculate_paper();
        total_ribbon += present.calculate_ribbon();
    }

    let mut result = DayResult::new();
    result.set_star1(total_paper.to_string());
    result.set_star2(total_ribbon.to_string());
    result
}

struct Present {
    length: u64,
    width: u64,
    height: u64,
}

impl Present {
    fn from_chars(dims_input: &Vec<char>) -> Self {
        let mut dims = [0u64; 3];
        let mut idx = 0;
        for value in dims_input {
            match value {
                '0'..='9' => {
                    let value_int: u64 = u64::from(value.to_digit(10).unwrap());
                    dims[idx] = dims[idx] * 10 + value_int;
                }
                'x' => idx += 1,
                _ => unreachable!(),
            }
        }

        Self {
            length: dims[0],
            width: dims[1],
            height: dims[2],
        }
    }

    fn calculate_paper(&self) -> u64 {
        let surface_area = [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];
        let smallest_side = surface_area.iter().min().unwrap();

        2 * (surface_area[0] + surface_area[1] + surface_area[2]) + smallest_side
    }

    fn calculate_ribbon(&self) -> u64 {
        let mut sorted_sides = [self.length, self.width, self.height];
        sorted_sides.sort_unstable();
        2 * (sorted_sides[0] + sorted_sides[1]) + self.length * self.width * self.height
    }
}
