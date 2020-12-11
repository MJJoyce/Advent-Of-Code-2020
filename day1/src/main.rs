use std::fs::File;
use itertools::Itertools;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn main() {
    let input_data = load_data().unwrap();

    println!("--------------\nNaive Solution");
    part1(&input_data);
    part2(&input_data);

    println!("--------------\nItertools Solution");
    itertools_test(&input_data);
}

fn load_data() -> Result<Vec<u64>, Error> {
    let br = BufReader::new(File::open("./input/input.txt")?);

    let mut v: Vec<u64> = Vec::new();
    for line in br.lines() {
        v.push(
            line?.trim().parse::<u64>()
                // We run into issues when dealing with the error type raised
                // by parse. Converting this to an Error resolves that mismatch.
                .map_err(|e| Error::new(ErrorKind::InvalidData, e))?
        );
    }

    Ok(v)
}

fn part1(input: &[u64]) {
    'all: for (p1, v1) in input.iter().enumerate() {
        for v2 in input[p1..].iter() {
            if v1 + v2 == 2020 {
                println!("Part 1 Result");
                println!("{} + {} == 2020", v1, v2);
                println!("{} * {} == {}", v1, v2, v1 * v2);
                println!("");
                break 'all;
            }
        }
    }
}

fn part2(input: &[u64]) {
    'all: for (p1, v1) in input.iter().enumerate() {
        for (p2, v2) in input[p1..].iter().enumerate() {
            for v3 in input[p2..].iter() {
                if v1 + v2 + v3 == 2020 {
                    println!("Part 2 Result");
                    println!("{} + {} + {} == 2020", v1, v2, v3);
                    println!("{} * {} * {} == {}", v1, v2, v3, v1 * v2 * v3);
                    println!("");
                    break 'all;
                }
            }
        }
    }
}

fn itertools_test(input: &[u64]) {
    for range in 2..=3 {
        for comb in input.iter().combinations(range) {
            // Use of sum() and product() here doesn't cooperate since we're
            // getting &u64 elements from combinations. The docs seem to suggest
            // that we would just get &u64 but I may be overlooking / misunderstanding
            // something ...
            //
            // So we'll do it the "old" way with fold
            if comb.iter().fold(0, |sum, x| sum + *x) == 2020 {
                println!("\nPart {} Solution", if range == 2 {1} else {2});
                println!("Sum Match: {:?}", comb);
                println!("Product: {}", comb.iter().fold(1, |prod, x| prod * *x));
            }
        }
    }
}
