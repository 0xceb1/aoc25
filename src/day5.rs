use std::cmp::max;
use std::ops::RangeInclusive;
use std::{error::Error, fs::File, io, io::prelude::*};

pub fn solve() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}

fn part1() -> Result<(), Box<dyn Error>> {
    let file = File::open("./inputs/input5.txt")?;
    let reader = io::BufReader::new(file);
    let mut rg: Vec<RangeInclusive<u64>> = Vec::new();
    let mut is_range = true;
    let mut cnt = 0;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            is_range = false;
            continue;
        }

        if is_range {
            let line_rg: Vec<u64> = line
                .split('-')
                .map(|s| s.parse())
                .collect::<Result<Vec<_>, _>>()?;

            rg.push(line_rg[0]..=line_rg[1]);
            continue;
        }

        let id: u64 = line.parse()?;
        if rg.iter().any(|int| int.contains(&id)) {
            cnt += 1;
        }
    }
    println!("{}", cnt);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let file = File::open("./inputs/input5.txt")?;
    let reader = io::BufReader::new(file);
    let mut rg: Vec<RangeInclusive<u64>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let line_rg: Vec<u64> = line
            .split('-')
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, _>>()?;
        rg.push(line_rg[0]..=line_rg[1]);
    }

    // merge intervals
    let mut merged: Vec<RangeInclusive<u64>> = Vec::new();

    rg.sort_by_key(|r| *r.start());

    let mut curr_rg = rg[0].clone();

    for next_rg in rg.into_iter().skip(1) {
        if next_rg.start() <= curr_rg.end() {
            curr_rg = *curr_rg.start()..=*max(curr_rg.end(), next_rg.end());
            continue;
        } else {
            merged.push(curr_rg);
            curr_rg = next_rg;
        }
    }
    merged.push(curr_rg);

    let cnt: u64 = merged
        .into_iter()
        .map(|r| (*r.end()) - (*r.start()) + 1)
        .sum();
    println!("{}", cnt);
    Ok(())
}
