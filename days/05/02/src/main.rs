use std::{error::Error, str::FromStr};

use aoc_2025_05_02::Goods;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("days/05/input.txt").expect("Failed to read input file");
    let goods = Goods::from_str(&input).expect("Failed to turn input into goods");
    println!("fresh ingredients ranges: {}", goods.count_fresh_ranges());
    Ok(())
}
