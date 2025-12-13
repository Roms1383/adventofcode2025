use std::{error::Error, str::FromStr};

use aoc_2025_09_01::Tiles;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("days/09/input.txt").expect("Failed to read input file");
    let tiles = Tiles::from_str(&input)?;
    println!("largest area: {}", tiles.largest_area());
    Ok(())
}
