use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day-01-trebuchet/input/input.txt")?;
    day_01::solve(&input);

    Ok(())
}
