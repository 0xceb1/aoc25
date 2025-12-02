use aoc25::*;
use std::error::Error;

const DAY: u8 = 2;

fn main() -> Result<(), Box<dyn Error>> {
    match DAY {
        1 => day1::solve()?,
        2 => day2::solve()?,
        _ => todo!(),
    }
    Ok(())
}
