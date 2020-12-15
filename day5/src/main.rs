use std::cmp::max;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};


fn main() {
    let input_data = load_data().unwrap();

    part1(&input_data);
    part2(&input_data);
}

fn load_data() -> Result<Vec<String>> {
    let br = BufReader::new(File::open("./input/input.txt")?);
    br.lines().collect()
}

fn part1(input: &[String]) {
    let mut max_seat_id = 0;
    
    for line in input.into_iter() {
        let row = calculate_row(&line[..7]);
        let col = calculate_col(&line[7..]);
        let cur_seat_id = row * 8 + col;

        max_seat_id = max(max_seat_id, cur_seat_id);
    }

    println!("\nPart 1:\nMax Seat ID {}", max_seat_id);
}

fn part2(input: &[String]) {
    println!("\nPart 2:");

    let mut ids: Vec<u32> = Vec::with_capacity(1100);
    for line in input.into_iter() {
        let row = calculate_row(&line[..7]);
        let col = calculate_col(&line[7..]);
        let cur_seat_id = row * 8 + col;
        ids.push(cur_seat_id);
    }

    ids.sort_unstable();

    let mut prev = ids[0] - 1;
    for id in ids {
        if id - prev != 1 {
            println!("Seat id gap found: Prev: {} Cur: {}", prev, id);
            println!("Seat id: {}", id - 1);
            break
        }

        prev = id;
    }
}

fn calculate_row(row_id: &str) -> u32 {
    let b_string = row_id.replace("B", "1");
    let b_string = b_string.replace("F", "0");

    u32::from_str_radix(&b_string, 2).unwrap()
}

fn calculate_col(col_id: &str) -> u32 {
    let b_string = col_id.replace("R", "1");
    let b_string = b_string.replace("L", "0");

    u32::from_str_radix(&b_string, 2).unwrap()
}
