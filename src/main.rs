use aoc25::*;
use std::error::Error;

const DAY: u8 = 3;

fn main() -> Result<(), Box<dyn Error>> {
    match DAY {
        1 => day1::solve()?,
        2 => day2::solve()?,
        3 => day3::solve()?,
        _ => todo!(),
    }
    Ok(())
}
