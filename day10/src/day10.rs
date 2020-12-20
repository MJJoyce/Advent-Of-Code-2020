use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;


pub fn load_data() -> Vec<u64> {
    let br = BufReader::new(File::open("./input/input.txt").unwrap());
    br.lines().map(|l| l.unwrap().trim().parse::<u64>().unwrap()).collect()
}

pub fn part1(input: &Vec<u64>) -> u64 {
    let mut d_1 = 0;
    let mut d_3 = 0;

    // Don't forget joltage difference between outlet (0) and first adapter!
    if input[0] == 1 {
        d_1 += 1;
    } else if input[0] == 3 {
        d_3 += 1;
    }
    

    for window in input.windows(2) {
        println!("Next pair: {:?}. D1: {} | D3: {}", window, d_1, d_3);

        match window[1] - window[0] {
            1 => {d_1 += 1; println!("Delta 1");},
            3 => {d_3 += 1; println!("Delta 3");},
            _ => ()
        }
    }

    // Don't forget the device's built in joltage adapter which is always
    // 3 higher than the highest adapter!
    d_1 * (d_3 + 1)
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
