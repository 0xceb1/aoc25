use std::{error::Error, fs};

pub fn solve() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;
    Ok(())
}

fn part1() -> Result<(), Box<dyn Error>> {
    let msg = fs::read_to_string("./inputs/input2.txt")?;
    let msg = msg.trim();
    // println!("{msg}");
    // for r in msg.split(',') {
    //     for num in r.split('-') {
    //         print!("{num}  ");
    //         let num = num.parse::<i128>();
    //         println!("{:?}", num);
    //     }
    // }
    fn sum_invalid(range: &str) -> u64 {
        let range: Vec<u64> = range.split('-').map(|s| s.parse().unwrap()).collect();
        assert!(range.len() == 2);
        let lb = range[0];
        let ub = range[1];
        (lb..=ub)
            .filter(|&x| {
                let x = x.to_string();
                let len = x.len();
                x[..(len / 2)] == x[(len / 2)..]
            })
            .sum()
    }

    let res: u64 = msg.split(',').map(|range| sum_invalid(range)).sum();
    println!("{res}");
    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let msg = fs::read_to_string("./inputs/input2.txt")?;
    let msg = msg.trim();

    fn is_invalid(num: u64) -> bool {
        let s = num.to_string();
        let len = s.len();

        for pattern_len in 1..=(len / 2) {
            if len % pattern_len == 0 {
                let pattern = &s[0..pattern_len];
                if s.as_bytes()
                    .chunks(pattern_len)
                    .all(|chunk| chunk == pattern.as_bytes())
                {
                    return true;
                }
            }
        }

        false
    }

    fn sum_invalid(range: &str) -> u64 {
        let range: Vec<u64> = range.split('-').map(|s| s.parse().unwrap()).collect();
        assert!(range.len() == 2);
        let lb = range[0];
        let ub = range[1];
        (lb..=ub).filter(|&x| is_invalid(x)).sum()
    }

    let res: u64 = msg.split(',').map(|range| sum_invalid(range)).sum();
    println!("{res}");

    Ok(())
}
