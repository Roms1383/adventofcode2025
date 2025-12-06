use std::{error::Error, str::FromStr};

use aoc_2025_06_01::Operations;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("days/06/input.txt").expect("Failed to read input file");
    let operations = Operations::from_str(&input)?;
    println!("cephalopod math: {}", operations.calculate());
    Ok(())
}
