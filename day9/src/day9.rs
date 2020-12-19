use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use itertools::Itertools;

pub fn load_data() -> Vec<u64> {
    let br = BufReader::new(File::open("./input/input.txt").unwrap());
    br.lines().map(|line| u64::from_str(&line.unwrap().trim()).unwrap()).collect()
}

pub fn part1(input: &[u64]) -> (usize, u64) {
    for (i, preamble) in input.windows(25).enumerate() {
        let cur_index = 25 + i;
        let target = &input[cur_index];

        let valid_nums: Vec<u64> =
            preamble.iter().combinations(2)
            .map(|pair| pair[0] + pair[1])
            .collect();

        if !valid_nums.contains(target) {
            return (cur_index, *target);
        }
    }

    panic!("Won't ever get here");
}

// Naive Implementation for finding the series of numbers
// that adds up to our target. For all valid window sizes,
// look at all windows of our input data, sum it, and check
// if it's the target.
pub fn part2(input: &[u64], target: u64) -> u64 {
    for win_size in 2..input.len() {
        for window in input.windows(win_size) {
            let sum: u64 = window.iter().sum();

            if sum == target {
                let min = window.iter().min().unwrap();
                let max = window.iter().max().unwrap();
                return min + max;
            }
        }
    }

    panic!("Won't ever get here");
}

// A "2 pointer" dynamic window solution to part 2
//
// Increase / decrease the window of values that we're checking
// based on a running-sum's relationship to our target.
//
// If our sum is smaller than the target, we add another value
//      to the current range (incrementing the right index)
//
// If our sum is larger than the target, we remove a value
//      from the current range (incrementing the left index)
pub fn part2_dynamic_window(input: &[u64], target: u64) -> u64 {
    let mut l = 0;
    let mut r = 1;
    let mut sum = input[l] + input[r];

    while sum != target && r < input.len() {
        if sum < target {
            r += 1;
            sum += input[r];
        } else {
            sum -= input[l];
            l += 1;
        }
    }

    if sum == target {
        let window = &input[l..=r];
        let min = window.iter().min().unwrap();
        let max = window.iter().max().unwrap();
        min + max
    } else {
        panic!("We won't ever get here");
    }
}
