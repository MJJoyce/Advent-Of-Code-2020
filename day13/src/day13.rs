use std::fs;
use std::str::FromStr;

use itertools::Itertools;

pub fn load_data() -> (u64, Vec<u64>){
    let file_contents = fs::read_to_string("./input/input.txt").unwrap();
    let file_contents = file_contents.trim();
    let (target, ids) = file_contents.split('\n').collect_tuple().unwrap();

    let target = u64::from_str(target.trim()).unwrap();
    let ids = ids.split(',').filter_map(|i|
        if !i.starts_with('x') {
            Some(u64::from_str(i).unwrap())
        } else {
            None
        }).collect();

    (target, ids)
}

pub fn part1(target: u64, bus_ids: Vec<u64>) -> u64 {
    let mut rems: Vec<(u64, u64)> = bus_ids.iter().map(|id| (id - (target % id), *id)).collect();
    rems.sort_unstable_by_key(|k| k.0);
    rems[0].0 * rems[0].1
}

pub fn part2() {
}
