use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() {
    let input_data = load_data().unwrap();

    part1(&input_data);
    part2(&input_data);
}

fn load_data() -> Result<Vec<Vec<char>>, Error> {
    let br = BufReader::new(File::open("./input/input.txt")?);

    let mut v: Vec<Vec<char>> = Vec::new();
    for line in br.lines() {
        v.push(line?.chars().collect());
    }

    Ok(v)
}

fn part1(input: &Vec<Vec<char>>) {
    let mut cc: usize = 3;
    let mut cr: usize = 1;
    let c_delta = 3;
    let r_delta = 1;
    let max_c = input[0].len();
    let max_r = input.len();

    let mut tree_encs = 0;

    while cr < max_r {
        match input[cr][cc] {
            '#' => tree_encs += 1,
            '.' => (),
             _  => panic!("Found invalid character in map")
        };

        cr += r_delta;
        cc = (cc + c_delta).rem_euclid(max_c);

    }

    println!("\nPart 1:\nTrees Encountered: {}", tree_encs);
}

fn part2(input: &Vec<Vec<char>>) {
    let mut tree_prod: u64 = 1;
    let max_c = input[0].len();
    let max_r = input.len();

    let slopes: [(usize, usize); 5] = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];

    println!("\nPart 2:");

    for (c_delta, r_delta) in slopes.iter() {
        let mut cc: usize = *c_delta;
        let mut cr: usize = *r_delta;
        let mut tree_encs = 0;

        while cr < max_r {
            match input[cr][cc] {
                '#' => tree_encs += 1,
                '.' => (),
                 _  => panic!("Found invalid character in map")
            };

            cr += *r_delta;
            cc = (cc + *c_delta).rem_euclid(max_c);

        }
        println!("({}, {}) Trees Encountered: {}", c_delta, r_delta, tree_encs);

        tree_prod *= tree_encs;
        println!("\tCurrent product: {}", tree_prod);
    }

}
