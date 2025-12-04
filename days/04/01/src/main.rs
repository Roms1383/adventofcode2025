use std::{error::Error, str::FromStr};

use aoc_2025_04_01::Grid;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("days/04/input.txt").expect("Failed to read input file");
    let grid = Grid::from_str(&input).expect("Failed to turn input into grid");
    println!("total rolls accessibles: {}", grid.count_accessibles());
    Ok(())
}
