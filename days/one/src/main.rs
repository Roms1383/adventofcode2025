use std::{error::Error, str::FromStr};

use aoc_2025_01::{Dial, Rotation};

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("days/one/input.txt").expect("Failed to read input file");
    let mut rotation: Rotation;
    let mut dial = Dial::default();
    let mut password = 0usize;
    for line in input.lines() {
        rotation = Rotation::from_str(line)?;
        dial = dial.rotate(rotation);
        if dial.is_zero() {
            password += 1;
        }
    }
    println!("password is: {password}");
    Ok(())
}
