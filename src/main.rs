use aoc25::*;
use std::error::Error;

const DAY: u8 = 4;

fn main() -> Result<(), Box<dyn Error>> {
    match DAY {
        1 => day1::solve()?,
        2 => day2::solve()?,
        3 => day3::solve()?,
        4 => day4::solve()?,
        _ => todo!(),
    }
    Ok(())
}
