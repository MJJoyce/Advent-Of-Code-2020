use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};


fn main() {
    let mut input_data = load_data().unwrap();
    input_data.sort_unstable();

    println!("\nPart 1: Max seat id: {}", input_data[input_data.len() - 1]);

    for pair in input_data.windows(2) {
        if pair[1] - pair[0] == 2 {
            println!("\nPart 2: Seat id: {}", pair[1] - 1);
        }
    }
}

fn load_data() -> Result<Vec<u16>> {
    let br = BufReader::new(File::open("./input/input.txt")?);
    let mut lines: Vec<u16> = Vec::new();

    for line in br.lines() {
        let mut val = 0;
        for (i, c) in line?.chars().rev().enumerate() {
            if c == 'R' || c == 'B' {
                val += 2u16.pow(i as u32);
            }
        }
        lines.push(val);
    }

    Ok(lines)
}
