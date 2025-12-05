use std::{error::Error, fs::File, io, io::prelude::*};
pub fn solve() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}

fn part1() -> Result<(), Box<dyn Error>> {
    let file = File::open("./inputs/input3.txt")?;
    let reader = io::BufReader::new(file);
    let mut res = 0;
    for line in reader.lines() {
        let line = line?;
        let bs: Vec<u32> = line
            .chars()
            .map(|c| c.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .ok_or("cannot parse digits")?;
        let mut l: u32 = 1;
        let mut r: u32 = 1;
        let n = bs.len();
        for (i, &b) in bs.iter().enumerate() {
            if b > l && i != (n - 1) {
                l = b;
                r = 1;
            } else if b > l && i == (n - 1) {
                r = b;
            } else if b > r {
                r = b;
            } else {
                continue;
            }
        }
        // println!("line {}, out {}", line, 10 * l + r);
        res += 10 * l + r;
    }
    println!("{}", res);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let file = File::open("./inputs/input3.txt")?;
    let reader = io::BufReader::new(file);
    let mut res: u128 = 0;

    for line in reader.lines() {
        let mut max_num: Vec<u32> = Vec::new();
        let line = line?;
        let bs: Vec<u32> = line
            .chars()
            .map(|c| c.to_digit(10))
            .collect::<Option<Vec<_>>>()
            .ok_or("cannot parse digits")?;
        // println!("{:?}", &bs);
        let mut st: usize = 0;
        let n = bs.len();
        for nend in (1..=12).rev() {
            // dbg!(&bs[st..=(n - nend)]);
            let maxd = bs[st..=(n - nend)].iter().max().ok_or("No max")?;
            // println!("{}", maxd);
            st = bs[st..]
                .iter()
                .position(|d| d == maxd)
                .ok_or("No max pos")?
                + st
                + 1;
            // println!("st={}", st);
            max_num.push(maxd.clone());
        }
        res += max_num
            .into_iter()
            .map(|d| d as u128)
            .reduce(|acc, digit| acc * 10 + digit)
            .ok_or("cannot reduce")?;
    }
    println!("{}", res);
    Ok(())
}
