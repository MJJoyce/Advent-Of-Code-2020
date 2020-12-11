use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let input_data = load_data().unwrap();

    part1(&input_data);
    part2(&input_data);
}

fn load_data() -> Result<Vec<(usize, usize, char, String)>, Error> {
    let br = BufReader::new(File::open("./input/input.txt")?);

    let mut v: Vec<(usize, usize, char, String)> = Vec::new();
    for line in br.lines() {
        let unwrapped_line = line.unwrap();
        let parts: Vec<&str> = unwrapped_line.split(':').collect();
        let spec_parts: Vec<&str> = parts[0].split(' ').collect();
        let range_parts: Vec<&str> = spec_parts[0].split('-').collect();
        v.push((
            range_parts[0].parse::<usize>().unwrap(),
            range_parts[1].parse::<usize>().unwrap(),
            spec_parts[1].chars().next().unwrap(),
            parts[1].trim().to_string()
        ));
    }

    Ok(v)
}

fn part1(input: &[(usize, usize, char, String)]) {
    let mut valid_count = 0;
    for (r_start, r_end, pw_char, pw) in input {
        let char_range = std::ops::RangeInclusive::new(r_start, r_end);
        let char_count: usize = pw.matches(&pw_char.to_string()).count();

        if char_range.contains(&&char_count) {
            valid_count += 1;
        }
    }

    println!("\nPart1\n--------------");
    println!("Valid PWs: {}", valid_count);
}

fn part2(input: &[(usize, usize, char, String)]) {
    let mut valid_count = 0;
    for (r_start, r_end, pw_char, pw) in input {
        let start = r_start - 1;
        let end = r_end - 1;
        let pw_vec: Vec<char> = pw.chars().collect();

        if pw_vec[start] != pw_vec[end] &&
            (pw_vec[start] == *pw_char || pw_vec[end] == *pw_char) {
            valid_count += 1;
        }
    }

    println!("\nPart2\n--------------");
    println!("Valid PWs: {}", valid_count);
}
