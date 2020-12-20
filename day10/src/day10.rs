use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;


pub fn load_data() -> Vec<u64> {
    let br = BufReader::new(File::open("./input/input.txt").unwrap());
    let mut adapters = vec![0];
    for line in br.lines() {
        let line = line.unwrap();
        adapters.push(line.trim().parse::<u64>().unwrap());
    }

    adapters.sort_unstable();
    adapters.push(adapters[adapters.len() - 1] + 3);
    adapters
}

pub fn part1(input: &Vec<u64>) -> u64 {
    let mut d_1 = 0;
    let mut d_3 = 0;

    for window in input.windows(2) {
        match window[1] - window[0] {
            1 => d_1 += 1,
            3 => d_3 += 1,
            _ => ()
        }
    }

    d_1 * d_3
}

pub fn part2() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        input.sort();
        assert_eq!(part1(&input), 35);
    }
}
