use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::ops::RangeInclusive;
use std::str::FromStr;

use regex::Regex;

// Shameless stolen from SO ...
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn main() {
    let input_data = load_data().unwrap();

    part1(&input_data);
    part2(&input_data);
}

fn load_data() -> Result<Vec<HashMap<String, String>>> {
    let br = BufReader::new(File::open("./input/input.txt")?);

    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let mut passport: HashMap<String, String> = HashMap::new();
    for line in br.lines() {
        let l = line.unwrap();
        let clean_line = l.trim();

        if clean_line == "" {
            passports.push(passport);
            passport = HashMap::new();
            continue;
        }

        for part in clean_line.split(" ") {
            match part.find(':') {
                None => panic!("Invalid passport component {:?}", part),
                Some(index) => {
                    match (&part[..index], &part[index + 1..]) {
                        (key, val) => passport.insert(key.to_string(), val.to_string()),
                        _ => panic!("Unable to split passport component {:?}", part)
                    };
                }
            };
        }
    }

    // Last passport isn't terminated by a newline ...
    passports.push(passport);

    Ok(passports)
}

fn is_passport_well_formed(passport: &HashMap<String, String>) -> bool {
    let required_fields: Vec<String> = vec_of_strings!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut valid = true;

    for f in required_fields.into_iter() {
        match passport.get(&f) {
            None => {valid = false; break;},
            Some(_) => continue
        };
    }

    let key_check: HashSet<String> = passport.keys().cloned().collect(); 
    if key_check.len() != passport.keys().len() {
        println!("Duplicate keys detected: is_valid:{} | {:?}", valid, passport);
    }

    valid
}

fn part1(input: &Vec<HashMap<String, String>>) {
    let mut valid_count = 0;

    for passport in input {
        if is_passport_well_formed(&passport) {
            valid_count += 1;
        } 
    }
    println!("\nPart 1: Valid Count: {}", valid_count);
}

fn part2(input: &Vec<HashMap<String, String>>) {
    let mut valid_count = 0;
    let eye_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    let pid_rgx = Regex::new(r"^\d{9}$").unwrap();
    let hcl_rgx = Regex::new(r"^#[\da-f]{6}$").unwrap();
    let hgt_cm_range = RangeInclusive::new(150, 193);
    let hgt_in_range = RangeInclusive::new(59, 76);

    for passport in input {
        if !is_passport_well_formed(&passport) {
            continue;
        }

        let mut valid = true;
        for (key, val) in passport {
            match key.as_str() {
                "cid" => continue,
                "byr" => {
                    // Looks like the *yr values aren't anything but 4 digits so
                    // we can skip the check ...
                    match u16::from_str(val) {
                        Ok(v) => if !(1920 <= v && v <= 2002) {valid = false; break;},
                        Err(_) => {valid = false; break;}
                    };
                },
                "iyr" => {
                    match u16::from_str(val) {
                        Ok(v) => if !(2010 <= v && v <= 2020) {valid = false; break;},
                        Err(_) => {valid = false; break;}
                    };
                },
                "eyr" => {
                    match u16::from_str(val) {
                        Ok(v) => if !(2020 <= v && v <= 2030) {valid = false; break;},
                        Err(_) => {valid = false; break;}
                    };
                },
                "hgt" => {
                    let (h, unit) = val.split_at(val.len() - 2);
                    match u16::from_str(h) {
                        Ok(height) => {
                            if unit != "in" && unit != "cm" ||
                               unit == "cm" && !hgt_cm_range.contains(&height) ||
                               unit == "in" && !hgt_in_range.contains(&height) {
                                   valid = false; break;
                            }
                        },
                        Err(_) => {valid = false; break;}
                    }
                },
                "hcl" => {
                    if !hcl_rgx.is_match(val) {valid = false; break;}
                },
                "ecl" => {
                    match eye_colors.iter().position(|&v| v == val) {
                        Some(_) => continue,
                        None => {valid = false; break;}
                    }
                },
                "pid" => {
                    if !pid_rgx.is_match(val) {valid = false; break;}
                },
                _ => panic!("This isn't possible")
            };
        }

        if valid {
            valid_count += 1;
        } 
    }

    println!("\nPart 2: Valid Count: {}", valid_count);
}
