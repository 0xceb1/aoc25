use std::{error::Error, fs::File, io, io::prelude::*};

pub fn solve() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}
pub fn part1() -> Result<(), Box<dyn Error>> {
    let file = File::open("./inputs/input1.txt")?;
    let reader = io::BufReader::new(file);
    let mut dial = 50;
    let mut cnt = 0;
    for line in reader.lines() {
        let line = line?;
        let step: i32 = line[1..].parse()?;
        let direction = line.chars().next().ok_or("XXX")?;
        match direction {
            'L' => dial = dial - step,
            'R' => dial = dial + step,
            _ => Err("XXX")?,
        }
        dial = dial.rem_euclid(100);
        if dial == 0 {
            cnt += 1;
        }
    }
    println!("{cnt}");
    Ok(())
}
pub fn part2() -> Result<(), Box<dyn Error>> {
    let file = File::open("./inputs/input1.txt")?;
    let reader = io::BufReader::new(file);
    let mut dial = 50;
    let mut cnt = 0;
    for line in reader.lines() {
        let line = line?;
        let step: i32 = line[1..].parse()?;
        let direction = line.chars().next().ok_or("XXX")?;
        match direction {
            'L' => {
                if dial == 0 {
                    cnt -= 1;
                }
                dial = dial - step;
                cnt -= dial.div_euclid(100);
                if dial.rem_euclid(100) == 0 {
                    cnt += 1;
                }
            }
            'R' => {
                dial = dial + step;
                cnt += dial.div_euclid(100);
            }
            _ => Err("XXX")?,
        }

        dial = dial.rem_euclid(100);
    }
    println!("{cnt}");
    Ok(())
}
