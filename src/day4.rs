use std::{collections::HashSet, error::Error, fs::File, io, io::prelude::*};

pub fn solve() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}

fn part1() -> Result<(), Box<dyn Error>> {
    let file = File::open("./inputs/input4.txt")?;
    let reader = io::BufReader::new(file);
    let mut pos_rolls: HashSet<(usize, usize)> = HashSet::with_capacity(128);
    for (nrow, line) in reader.lines().enumerate() {
        let line = line?;
        for (ncol, c) in line.chars().enumerate() {
            if c == '@' {
                pos_rolls.insert((nrow + 1, ncol + 1));
            }
        }
    }

    fn is_accessed(pt: (usize, usize), pos_rolls: &HashSet<(usize, usize)>) -> bool {
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let cnt = directions
            .iter()
            .filter_map(|(dx, dy)| {
                let new_x = (pt.0 as isize + dx) as usize;
                let new_y = (pt.1 as isize + dy) as usize;
                pos_rolls.get(&(new_x, new_y))
            })
            .count();
        cnt < 4
    }
    let res = pos_rolls
        .iter()
        .filter(|&&pt| is_accessed(pt, &pos_rolls))
        .count();
    println!("{}", res);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let file = File::open("./inputs/input4.txt")?;
    let reader = io::BufReader::new(file);
    let mut pos_rolls: HashSet<(usize, usize)> = HashSet::with_capacity(128);
    for (nrow, line) in reader.lines().enumerate() {
        let line = line?;
        for (ncol, c) in line.chars().enumerate() {
            if c == '@' {
                pos_rolls.insert((nrow + 1, ncol + 1));
            }
        }
    }
    let init_num = pos_rolls.len();

    fn is_accessed(pt: (usize, usize), pos_rolls: &HashSet<(usize, usize)>) -> bool {
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let cnt = directions
            .iter()
            .filter_map(|(dx, dy)| {
                let new_x = (pt.0 as isize + dx) as usize;
                let new_y = (pt.1 as isize + dy) as usize;
                pos_rolls.get(&(new_x, new_y))
            })
            .count();
        cnt < 4
    }
    let mut stop = false;
    while !stop {
        let original = pos_rolls.clone();
        pos_rolls.retain(|&pt| !is_accessed(pt, &original));
        stop = pos_rolls.len() == original.len();
    }
    println!("{}", init_num - pos_rolls.len());

    Ok(())
}
