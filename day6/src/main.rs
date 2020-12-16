use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let br = BufReader::new(File::open("./input/input.txt").unwrap());

    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    let mut group_answer_sets: Vec<HashSet<char>> = Vec::with_capacity(8);

    let mut lines_iter = br.lines().peekable();

    while let Some(l) = lines_iter.next() {
        let line = l.unwrap();
        let clean_line = line.trim().to_lowercase();

        if clean_line == "" || lines_iter.peek().is_none() {
            if lines_iter.peek().is_none()  {
                let ans: HashSet<char> = clean_line.chars().collect();
                group_answer_sets.push(ans);
            }

            // Not really a fan of cloning this twice. Tried to figure out a
            // nice way to "take" the value for intersection but couldn't get it
            // to cooperate. So ... leaving it this way.
            let mut union = group_answer_sets[0].clone();
            let mut intersection = group_answer_sets[0].clone();

            for answers in group_answer_sets[1..].iter() {
                union = union.union(answers).cloned().collect();
                intersection = intersection.intersection(answers).cloned().collect();
            }

            sum_part1 += union.len();
            sum_part2 += intersection.len();

            group_answer_sets.clear();
            continue;
        }

        let ans: HashSet<char> = clean_line.chars().collect();
        group_answer_sets.push(ans);

    }

    println!("\nPart 1: Yes sum: {}", sum_part1);
    println!("\nPart 1: Yes sum intersection: {}", sum_part2);
}
