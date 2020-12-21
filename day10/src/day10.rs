use std::collections::{VecDeque, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};


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

// Part 1 requires us to count the number of occurrences of 1 / 3 joltage deltas
// between nodes in our plugin chain. The answer is the product of these 2 counts.
pub fn part1(input: &[u64]) -> u64 {
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


// Part 2 asks us to determine the number of valid adapter configurations that can
// be generated.
//
// A given node's number of valid paths to the goal node is the sum of its children's
// valid paths. A node has at most 3 children. The 3 adapters that follow our current
// node in sorted order are potential children. A node is a valid child if the joltage
// difference between the two nodes is <= 3.
//
// In reverse node order, calculate the number of valid paths (sum of child node vals)
// and memoize that value. The answer to part 2 is the number of valid paths from the
// start node "0";
pub fn part2(input: &[u64]) -> u64 {
    let mut valid_paths: HashMap<u64, u64> = HashMap::with_capacity(input.len());

    // The last node (aka, the goal) has 1 path to the goal / itself
    valid_paths.insert(input[input.len() - 1], 1);

    // From last node to first, calculate the number of valid paths
    // the current node has to the goal and cache that result.
    for cur_index in (0 .. input.len() - 1).rev() {
        let cur_node = input[cur_index];
        let mut paths = 0;

        // Look ahead at most 3 nodes from our current node and determine how
        // many valid paths we have to the goal node. A path from our current
        // node to the look ahead node is valid if the delta joltage between them
        // is <= 3.
        let mut look_ahead = cur_index + 1;
        while look_ahead - cur_index < 4 && look_ahead < input.len() {
            let la_node = input[look_ahead];

            if la_node - cur_node <= 3 {
                paths += valid_paths.get(&(la_node as u64)).unwrap();
                look_ahead += 1;
            } else {
                break;
            }
        }

        valid_paths.insert(cur_node, paths);
    }

    valid_paths[&0]
}


// This is effectively the same part 2 implementation with memoization on only
// the last 3 nodes that we've processed. Anything more than that is a waste
// of memory.
pub fn part2_min_mem(input: &[u64]) -> u64 {
    let mut valid_paths: VecDeque<u64> = VecDeque::with_capacity(3);

    // The last node (aka, the goal) has 1 path to the goal / itself
    valid_paths.push_front(1);

    // From last node to first, calculate the number of valid paths
    // the current node has to the goal and cache that result.
    for cur_index in (0 .. input.len() - 1).rev() {
        let cur_node = input[cur_index];
        let mut paths = 0;

        // Look ahead at most 3 nodes from our current node and determine how
        // many valid paths we have to the goal node. A path from our current
        // node to the look ahead node is valid if the delta joltage between them
        // is <= 3.
        let mut look_ahead = 1;
        while look_ahead < 4 && cur_index + look_ahead < input.len() {
            let la_node = input[cur_index + look_ahead];

            if la_node - cur_node <= 3 {
                paths += *valid_paths.get(look_ahead as usize - 1).unwrap();
                look_ahead += 1;
            } else {
                break;
            }
        }

        if valid_paths.len() == 3 {
            valid_paths.pop_back();
        }
        valid_paths.push_front(paths);
    }

    *valid_paths.get(0).unwrap()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4, 0, 22];
        input.sort();
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn test_p2() {
        let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4, 0, 22];
        input.sort();
        assert_eq!(part2(&input), 8);
        assert_eq!(part2_min_mem(&input), 8);

        let mut input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38,
            39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3, 0, 52];
        input.sort();
        assert_eq!(part2(&input), 19208);
        assert_eq!(part2_min_mem(&input), 19208);
    }
}
