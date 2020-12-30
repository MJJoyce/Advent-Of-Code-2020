use std::convert::TryInto;
use std::fs;
use std::str::FromStr;

use itertools::Itertools;

pub fn load_data() -> (usize, Vec<(usize, usize)>){
    let file_contents = fs::read_to_string("./input/input.txt").unwrap();
    let file_contents = file_contents.trim();
    let (target, ids) = file_contents.split('\n').collect_tuple().unwrap();

    let target = usize::from_str(target.trim()).unwrap();
    let mut bus_ids: Vec<(usize, usize)> = Vec::new();
    for (i, id) in ids.split(',').enumerate() {
        if !id.starts_with('x') {
            bus_ids.push((usize::from_str(id).unwrap(), i));
        }
    }
    (target, bus_ids)
}

pub fn part1(target: usize, bus_ids: Vec<usize>) -> usize {
    let mut rems: Vec<(usize, usize)> = bus_ids.iter().map(|id| (id - (target % id), *id)).collect();
    rems.sort_unstable_by_key(|k| k.0);
    rems[0].0 * rems[0].1
}

pub fn part2(input: Vec<(usize, usize)>) -> usize {
    // Sort the input vector from largest to smallest and adjust
    // the time offset values as necessary so our first index has
    // a zero offset.
    let mut inputs = input.clone();
    inputs.sort_unstable_by_key(|k| k.0);
    inputs.reverse();
    let ids: Vec<(isize, isize)> = inputs.iter().map(|(v, off)| {
        let offset = *off as isize - inputs[0].1 as isize;
        (*v as isize, offset)
    }).collect();

    // Take the two biggest bus ids and find a time that satisfies
    // their departure constraints. This will be the value that we
    // start iterating from to locate our answer.
    let mut t = ids[0].0;
    loop {
        if (t as isize + ids[1].1) % ids[1].0 == 0 {
            break;
        }

        t += ids[0].0;
    }

    // t is now the value that we should start iterating from. We
    // increment by the product of of the first 2 values since the
    // other values won't satisfy these two constraints.
    let delta = ids[0].0 * ids[1].0;
    loop {
        let mut valid = true;
        for (id, offset) in ids[2..].into_iter() {
            if (t as isize + offset) % id != 0 {
                valid = false;
                break;
            }
        }

        if valid {
            break;
        }

        t += delta;
    }

    // Get the original offset for our (likely) changed first index
    // value and adjust the calculated t by that amount.
    for (v, off) in input.iter() {
        if *v == ids[0].0 as usize {
            return t as usize - off;
        }
    }

    // Can't get here
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = vec![(17, 0), (13, 2), (19, 3)];
        assert_eq!(part2(input), 3417);

        let input = vec![(7, 0), (13, 1), (59, 4), (31, 6), (19, 7)];
        assert_eq!(part2(input), 1068781);
    }
}
