use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::str::FromStr;

use itertools::Itertools;
use regex::Regex;

fn main() {
    let input_data = load_data();

    part1(&input_data);
    part2(&input_data);
}

fn load_data() -> HashMap<String, Vec<(u32, String)>> {
    let br = BufReader::new(File::open("./input/input.txt").unwrap());
    let mut ret = HashMap::new();

    let bags_regex = Regex::new(r" bag[s]??\.??").unwrap();

    for line in br.lines() {
        let l = line.unwrap();
        // Dropping all instances of bag[s] here to simplify further processing
        let cleaned_line = bags_regex.replace_all(l.trim(), "");

        let (bag, contains) = cleaned_line.split("contain").collect_tuple().unwrap();
        let contents: Vec<(u32, String)> = match contains.contains("no other") {
            false => {
                contains.split(',')
                    .map(|s| {
                        let s = s.trim();
                        (
                            // `s` here is of the form:
                            //    "<count> <bag name>"
                            //
                            // Grab the count character and convert it to a base 10 digit
                            s.chars().nth(0).unwrap().to_digit(10).unwrap(),
                            // Grab the rest of the string
                            s[2..].trim().to_string()
                        )
                    })
                    .collect()
            },
            true => vec![]
        };

        ret.insert(bag.trim().to_string(), contents);
    }

    ret
}

fn part1(input_data: &HashMap<String, Vec<(u32, String)>>) {
    let mut can_contain_gold_bag_count = 0;
    let mut gold_bag_counts: HashMap<&String, u32> = HashMap::with_capacity(input_data.len());

    for key in input_data.keys() {
        if ! gold_bag_counts.contains_key(key) {
            let count = calculate_gold_bag_count(key, &input_data, &mut gold_bag_counts);
            gold_bag_counts.insert(key, count);
        }
    }

    for (bag, count) in gold_bag_counts {
        if count > 0 {
            can_contain_gold_bag_count += 1;
        }
    }

    println!("\nPart 1: Can contain gold bag count: {}", can_contain_gold_bag_count);

}

fn calculate_gold_bag_count<'a>(bag: &String, bag_contents: &'a HashMap<String, Vec<(u32, String)>>, gold_bag_counts: &mut HashMap<&'a String, u32>) -> u32 {
    let (_, contents) = bag_contents.get_key_value(bag).unwrap();

    match contents.len() {
        0 => 0,
        _ => {
            let bag_count: Vec<u32> = contents.iter()
                .map(|c|
                    match c.1.contains("shiny gold") {
                        true => c.0,
                        false => {
                            c.0 * match gold_bag_counts.get_key_value(&c.1) {
                                Some((_, v)) => *v,
                                None => {
                                    let count = calculate_gold_bag_count(&c.1, bag_contents, gold_bag_counts);
                                    gold_bag_counts.insert(&c.1, count);
                                    count
                                }
                            }
                        }
                    }
                ).collect();
            bag_count.iter().sum()
        }
    }
}

fn part2(input_data: &HashMap<String, Vec<(u32, String)>>) {
    let nested_bags = calculate_nested_bags(&"shiny gold".to_string(), &input_data);
    println!("Part 2: Shiny Gold bag contents count: {}", nested_bags - 1);
}

fn calculate_nested_bags(bag: &String, bag_contents: &HashMap<String, Vec<(u32, String)>>) -> u32 {
    let (_, contents) = bag_contents.get_key_value(bag).unwrap();

    let count = match contents.len() {
        0 => 0,
        _ => contents.iter().map(|c| c.0 * calculate_nested_bags(&c.1, bag_contents)).sum()
    };

    count + 1
}
