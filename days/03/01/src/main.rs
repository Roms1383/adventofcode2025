use std::{error::Error, str::FromStr};

use aoc_2025_03_01::Banks;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("days/03/input.txt").expect("Failed to read input file");
    let ranges = Banks::from_str(&input).expect("Failed to turn input into banks");
    println!("total joltage: {}", ranges.total_joltage());
    Ok(())
}
