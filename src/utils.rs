pub fn load_puzzle_as_chars(src: &str) -> Result<Vec<Vec<char>>, &'static str> {
    let path = std::path::Path::new("./puzzle_inputs/").join(src);
    let puzzle = std::fs::read_to_string(path).or(Err("Failed to read puzzle file"))?;
    let lines = puzzle.lines();
    let line_chars: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
    Ok(line_chars)
}
