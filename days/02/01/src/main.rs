use std::{error::Error, str::FromStr};

use aoc_2025_02_01::IDRanges;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("days/02/input.txt").expect("Failed to read input file");
    let ranges = IDRanges::from_str(&input).expect("Failed to turn input into ranges");
    println!("invalid(s): {}", ranges.sum_invalids());
    Ok(())
}
