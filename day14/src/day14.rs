#![feature(map_into_keys_values)]
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;
use regex::Regex;

pub fn load_data() {
}

// Part 1
//
// Part 1 specifies that the current bitmask is applied to a memory locations
// value immediately prior to it being written. A 0 or 1 in the mask means that
// corresponding bit should be overwritten. An X leaves that bit alone.
//
// Two version of a mask are created whenever a "mask line" is encountered. An
// "and" mask (where X's are set to 1) and an "or" mask (where X's are set to 0).
// A value is first bitwise and-ed against the "and" mask and then bitwise or-ed
// against the "or" mask. This sets 0's in the value from the mask (and keeps the X
// values) and then sets 1's in the value from the mask (and keeps the X values).
//
// The answer is the sum of set memory values.
pub fn part1() -> u64 {
    let mut and_mask = 0;
    let mut or_mask = 0;

    let br = BufReader::new(File::open("./input/input.txt").unwrap());
    let mut mem: HashMap<u64, u64> = HashMap::new();

    for line in br.lines() {
        let line = line.unwrap();
        let line = line.trim();

        if line.starts_with("mask") {
            let new_masks = parse_bitmask(line);
            and_mask = new_masks.0;
            or_mask = new_masks.1;
        } else {
            let (mem_loc, val) = parse_value(line);
            mem.insert(mem_loc, (val & and_mask) | or_mask);
        }
    }

    mem.into_iter().fold(0, |acc, (_, v)| acc + v)
}

// Part 2
//
// Part 2 specifies that the current bitmask is actually a "memory address decoder"
// where by the X values change to floating bits that result in N memory addresses
// (where N == # of 'X's in the mask.). A 0 in the mask means the bit is left
// unchanged and a 1 in the mask means the bit is set to 1.
//
// The "and" mask is no longer needed for Part 2. Instead, the memory location is
// bitwise or-ed against the "or" mask. Then, all permutations of the floating bit
// values are used to create the memory locations where the value should be written.
//
// The answer is the sum of set memory values.
pub fn part2() -> u64 {
    let mut or_mask = 0;
    let mut x_indices: Vec<usize> = Vec::new();

    let br = BufReader::new(File::open("./input/input.txt").unwrap());
    let mut mem: HashMap<u64, u64> = HashMap::new();

    for line in br.lines() {
        let line = line.unwrap();
        let line = line.trim();

        if line.starts_with("mask") {
            let new_masks = parse_bitmask(line);
            or_mask = new_masks.1;
            x_indices = extract_floating_indices(new_masks.2);
        } else {
            let (mem_loc, val) = parse_value(line);
            let mem_loc = mem_loc | or_mask;
            for loc in gen_mem_locations(&x_indices, mem_loc) {
                mem.insert(loc, val);
            }
        }
    }

    mem.into_iter().fold(0, |acc, (_, v)| acc + v)
}

// Extract masks from an input mask line
//
// Given a "mask line" of the form:
//  mask = 0010X01001X010000110100000X000010X11
//
// Extract out and return 3 masks:
//  - "and_mask": The mask with all X's converted to 1
//  - "or_mask": The mask with all X's converted to 0
//  - The raw mask
fn parse_bitmask(line: &str) -> (u64, u64, String) {
    let line: (&str, &str) = line.split('=').collect_tuple().unwrap();
    let mask = line.1.trim();

    let and_mask = mask.to_string().replace("X", "1");
    let or_mask = mask.to_string().replace("X", "0");

    (
        u64::from_str_radix(&and_mask, 2).unwrap(),
        u64::from_str_radix(&or_mask, 2).unwrap(),
        mask.to_string()
    )
}

// Extract memory and value from an input mem line
//
// Given a "mem line" of the form:
//  mem[41717] = 288
//
// Extract out the memory location (41717) and the value (288)
fn parse_value(line: &str) -> (u64, u64) {
    let line: (&str, &str) = line.split('=').collect_tuple().unwrap();

    let mem_re = Regex::new(r"mem\[(\d*)\]").unwrap();
    let mem = mem_re.captures(line.0.trim()).unwrap().get(1).unwrap().as_str();

    let val = line.1.trim();

    (
        u64::from_str_radix(mem, 10).unwrap(),
        u64::from_str_radix(val, 10).unwrap()
    )
}

// Extract the bits from a mask marked as "floating"
//
// Given a mask string of the form:
//  00000000000000000000000000000000X0XX"
// 
// Extract the bit indices of all "floating bits" (i.e., those marked X).
// Indices are "MSB" order. E.g., the indices for the above mask would
// be returned as [3, 1, 0]
fn extract_floating_indices(mask: String) -> Vec<usize> {
    let mut x_indices: Vec<usize> = Vec::new();

    for (i, c) in mask.char_indices() {
        if c == 'X' {
            x_indices.push(mask.len() - 1 - i);
        }
    }

    x_indices
}

// Generate all possible memory locations given indices of "floating" mask bits
//
// Return all possible memory locations by creating all permutations of toggled
// bits defined by the "floating" mask bit indices.
//
// The floating indices mark the bit locations that "float" in our memory location.
// This generations all possible bit-combinations for those indices and returns the
// corresponding modified memory locations.
//
// For example, consider the following mask and mem_lock values:
//  mask = "000000000000000000000000000000X1001X"
//  mem_loc = 58
//
//  The floating_indices for the above mask would be [5, 0]
//
//  There are 4 bit permutations for the 2 floating bits:
//  0 0
//  0 1
//  1 0
//  1 1
//
//  We generate these 4 permutations and set bits in mem_loc as appropriate for each one
//  mem_loc == 58 where bit 5 and 0 are set to 0 is 26
//      000000000000000000000000000000011010
//
//  mem_lock == 58 where bit 5 is set to 0 and bit 0 is set to 1 is 27
//      000000000000000000000000000000011011
//
//  And so on for each permutation
fn gen_mem_locations(floating_indices: &[usize], mem_loc: u64) -> Vec<u64> {
    let mut locs = Vec::new();

    // We need to generate every bit-toggled combination for the floating indices.
    // Loop over all the digits from 0 to 2^(# of floating indices) so we get all
    // combinations of bit values. Each bit in the permutation corresponds to whether
    // the same off-set float_index bit should be set to 0 or 1.
    //
    // For example, given a floating_indices of [5, 0] and a bit permutation of "0 1",
    // set mem_locs bit 5 to 0 and bit 0 to 1 and save that value.
    for bit_permutation in 0..2isize.pow(floating_indices.len() as u32) {
        let mut mem_loc_perm = mem_loc;
        let mut bit_i = floating_indices.len() as isize - 1;
        let mut bit: isize = 1 << bit_i;

        // Set each float bit based on the values described in the current bit_permutation
        while bit > 0 {
            match bit_permutation & bit {
                0 => {
                    mem_loc_perm &= !(1 << floating_indices[bit_i as usize]);
                },
                _ => {
                    mem_loc_perm |= 1 << floating_indices[bit_i as usize];
                }
            }

            bit >>= 1;
            bit_i -= 1;
        }

        locs.push(mem_loc_perm);
    }

    locs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1_masks() {
        let (and_mask, or_mask, _) = parse_bitmask("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        assert_eq!((11 & and_mask) | (or_mask), 73);
        assert_eq!((101 & and_mask) | (or_mask), 101);
        assert_eq!((0 & and_mask) | (or_mask), 64);
    }

    #[test]
    fn test_x_indices_extract() {
        let mask = "000000000000000000000000000000X1001X".to_string();
        assert_eq!(vec![5, 0], extract_floating_indices(mask));

        let mask = "00000000000000000000000000000000X0XX".to_string();
        assert_eq!(vec![3, 1, 0], extract_floating_indices(mask));
    }

    #[test]
    fn test_mem_loc_bit_toggles() {
        let mut mem_locs = gen_mem_locations(&vec![5, 0], 42 | 18);
        mem_locs.sort_unstable();
        assert_eq!(vec![26, 27, 58, 59], mem_locs);

        let mut mem_locs = gen_mem_locations(&vec![3, 1, 0], 26);
        mem_locs.sort_unstable();
        assert_eq!(vec![16, 17, 18, 19, 24, 25, 26, 27], mem_locs);
    }
}
