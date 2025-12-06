use crate::read_rectangle;
use std::{error::Error, fs::File, io, io::prelude::*};

pub fn solve() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}

fn part1() -> Result<(), Box<dyn Error>> {
    let file = File::open("./inputs/input6.txt")?;
    let reader = io::BufReader::new(file);
    let mut worksheet: Vec<Vec<u64>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    for line in reader.lines() {
        let line: Vec<_> = line?.split_whitespace().map(str::to_string).collect();
        if worksheet.is_empty() {
            worksheet = vec![Vec::new(); line.len()];
        }
        if line[0].parse::<u64>().is_ok() {
            for (i, num) in line.iter().enumerate() {
                worksheet[i].push(num.parse::<u64>().unwrap());
            }
        } else {
            operators = line
                .into_iter()
                .map(|c| c.parse::<char>().unwrap())
                .collect();
        }
    }
    let res: u64 = worksheet
        .into_iter()
        .enumerate()
        .map(|(i, arr)| match operators[i] {
            '*' => arr.iter().product::<u64>(),
            '+' => arr.iter().sum::<u64>(),
            _ => panic!(),
        })
        .sum();
    println!("{}", res);
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let file = File::open("./inputs/input6.txt")?;
    let reader = io::BufReader::new(file);
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut operators: Vec<char> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i < 4 {
            grid.push(line.chars().collect::<Vec<char>>());
        } else {
            operators = line
                .split_whitespace()
                .map(|c| c.parse::<char>().unwrap())
                .collect();
        }
    }

    let length = grid[0].len();
    let nrow = grid.len();
    let is_all_empty: Vec<bool> = (0..length)
        .map(|col| (0..nrow).all(|row| grid[row][col].is_whitespace()))
        .collect();

    let mut mats: Vec<Vec<Vec<_>>> = Vec::new();
    let mut l: usize = 0;

    for (i, b) in is_all_empty.iter().enumerate() {
        if *b {
            mats.push(read_rectangle("./inputs/input6.txt", 0, 3, l, i - 1));
            l = i + 1;
        } else {
            continue;
        }
    }
    mats.push(read_rectangle("./inputs/input6.txt", 0, 3, l, length - 1));

    fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        assert!(!v.is_empty());
        (0..v[0].len())
            .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
            .collect()
    }

    let tmats: Vec<Vec<String>> = mats
        .into_iter()
        .map(|mat| {
            transpose(mat)
                .into_iter()
                .map(|row| row.into_iter().collect::<String>())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    dbg!(&tmats[0]);

    let worksheet: Vec<Vec<u64>> = tmats
        .into_iter()
        .map(|mat| {
            mat.into_iter()
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect();

    let res: u64 = worksheet
        .into_iter()
        .enumerate()
        .map(|(i, arr)| match operators[i] {
            '*' => arr.iter().product::<u64>(),
            '+' => arr.iter().sum::<u64>(),
            _ => panic!(),
        })
        .sum();

    println!("{}", res);

    Ok(())
}
