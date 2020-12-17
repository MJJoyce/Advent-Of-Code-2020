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

// Iterate over each bag type and calculate how many shiny gold bags that type
// can contain. Save a count of the bag types that can contain a shiny gold bag.
//
// There's a good bit of overkill here since we care only that a bag can contain
// a shiny gold bag and not the number that it can contain. I guessed (incorrectly)
// that the count would be valuable in part 2. Oh well ...
fn part1(input_data: &HashMap<String, Vec<(u32, String)>>) {
    let mut can_contain_gold_bag_count = 0;
    let mut gold_bag_counts: HashMap<&String, u32> = HashMap::with_capacity(input_data.len());

    for key in input_data.keys() {
        if ! gold_bag_counts.contains_key(key) {
            let count = calculate_gold_bag_count(key, &input_data, &mut gold_bag_counts);
            gold_bag_counts.insert(key, count);
        }

        if *gold_bag_counts.get(key).unwrap() > 0 {
            can_contain_gold_bag_count += 1;
        }
    }

    println!("\nPart 1: Can contain gold bag count: {}", can_contain_gold_bag_count);

}

fn calculate_gold_bag_count<'a>(
        bag: &String,
        bag_contents: &'a HashMap<String, Vec<(u32, String)>>,
        gold_bag_counts: &mut HashMap<&'a String, u32>
    ) -> u32 {

    let contents = bag_contents.get(bag).unwrap();

    match contents.len() {
        // Current bag has no contents, so it contains no gold bags
        0 => 0,
        
        // Current bag contains gold bags relevant to what its component bags are.
        // For each set of bags of a type,
        //      If the type is shiny gold bag, use its count
        //      Otherwise, its value is the number of bags * how many shiny gold
        //          bags that bag type can contain
        _ => contents.iter().map(|c|
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
        ).sum()
    }
}

fn part2(input_data: &HashMap<String, Vec<(u32, String)>>) {
    let nested_bags = calculate_total_bags(&"shiny gold".to_string(), &input_data) - 1;
    println!("Part 2: Shiny Gold bag contents count: {}", nested_bags);
}

fn calculate_total_bags(bag: &String, bag_contents: &HashMap<String, Vec<(u32, String)>>) -> u32 {
    let (_, contents) = bag_contents.get_key_value(bag).unwrap();

    let count = match contents.len() {
        0 => 0,
        _ => contents.iter().map(|c| c.0 * calculate_total_bags(&c.1, bag_contents)).sum()
    };

    count + 1
}
