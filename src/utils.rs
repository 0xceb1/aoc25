use std::fs::File;
use std::io::{self, BufRead};

/// Read a rectangle region from a txt file, assume ascii encoding
pub fn read_rectangle(
    path: &str,
    top_row: usize,
    bottom_row: usize,
    left_col: usize,
    right_col: usize,
) -> Vec<Vec<char>> {
    let file = File::open(path).expect("INCORRECT PATH");
    let reader = io::BufReader::new(file);
    let mut res: Vec<Vec<char>> = Vec::new();

    for (y, line) in reader.lines().enumerate() {
        if (top_row..=bottom_row).contains(&y) {
            let line: Vec<char> = line
                .unwrap()
                .chars()
                .skip(left_col)
                .take(right_col - left_col + 1)
                .collect();
            res.push(line);
        } else {
            continue;
        }
    }
    res
}
