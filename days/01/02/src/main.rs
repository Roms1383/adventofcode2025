use std::{error::Error, str::FromStr};

use aoc_2025_01_02::{Dial, Rotation};

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("days/01/input.txt").expect("Failed to read input file");
    let mut rotation: Rotation;
    let mut dial = Dial::default();
    for line in input.lines() {
        rotation = Rotation::from_str(line)?;
        dial.rotate(rotation);
    }
    println!("password is: {}", dial.password());
    Ok(())
}
