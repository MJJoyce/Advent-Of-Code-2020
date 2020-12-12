use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::ops::RangeInclusive;
use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;

// Shameless stolen from SO ...
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

lazy_static! {
    static ref REQUIRED_FIELDS: Vec<String> = vec_of_strings!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    static ref EYE_COLORS: Vec<String> = vec_of_strings!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    static ref PID_RGX: Regex = Regex::new(r"^\d{9}$").unwrap();
    static ref HCL_RGX: Regex = Regex::new(r"^#[\da-f]{6}$").unwrap();
}

const HGT_CM_RANGE: RangeInclusive<u16> = RangeInclusive::new(150, 193);
const HGT_IN_RANGE: RangeInclusive<u16> = RangeInclusive::new(59, 76);


fn main() {
    let input_data = load_data().unwrap();

    part1(&input_data);
    part2(&input_data);
}

fn load_data() -> Result<Vec<Passport>> {
    let br = BufReader::new(File::open("./input/input.txt")?);

    let mut passports: Vec<Passport> = Vec::new();
    let mut passport = Passport::new();

    for line in br.lines() {
        let l = line.unwrap();
        let clean_line = l.trim().to_string();

        if clean_line == "" {
            passports.push(passport);
            passport = Passport::new();
            continue;
        }

        //passport.parse_fields(&clean_line);
        passport.parse_fields(&clean_line);
    }

    // Last passport isn't terminated by a newline ...
    passports.push(passport);

    Ok(passports)
}


fn part1(input: &Vec<Passport>) {
    let mut valid_count = 0;

    for passport in input {
        if passport.has_required_fields() {
            valid_count += 1;
        }
    }

    println!("\nPart 1: Valid Count: {}", valid_count);
}

fn part2(input: &Vec<Passport>) {
    let mut valid_count = 0;

    for passport in input {
        if passport.has_required_fields() && passport.has_valid_field_values() {
            valid_count += 1;
        }
    }

    println!("\nPart 2: Valid Count: {}", valid_count);
}

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn new() -> Self {
        Passport {fields: HashMap::new()}
    }

    fn parse_fields(&mut self, fields_text: &String) {
        //let clean_line = fields_text.trim();
        for part in fields_text.split(" ") {
            match part.find(':') {
                None => panic!("Invalid passport component {:?}", part),
                Some(index) => {
                    match (&part[..index], &part[index + 1..]) {
                        (key, val) => self.fields.insert(key.to_string(), val.to_string()),
                        _ => panic!("Unable to split passport component {:?}", part)
                    };
                }
            };
        }
    }

    fn has_required_fields(&self) -> bool {
        let mut valid = true;

        for f in REQUIRED_FIELDS.iter() {
            match self.fields.get(f) {
                None => {valid = false; break;},
                Some(_) => continue
            };
        }

        valid
    }

    fn has_valid_field_values(&self) -> bool {
        let mut valid = true;

        for (key, val) in self.fields.iter() {
            match key.as_str() {
                "cid" => continue,
                "byr" => {
                    // Looks like the *yr values aren't anything but 4 digits so
                    // we can skip the check ...
                    match u16::from_str(&val) {
                        Ok(v) => if !(1920 <= v && v <= 2002) {valid = false; break;},
                        Err(_) => {valid = false; break;}
                    };
                },
                "iyr" => {
                    match u16::from_str(&val) {
                        Ok(v) => if !(2010 <= v && v <= 2020) {valid = false; break;},
                        Err(_) => {valid = false; break;}
                    };
                },
                "eyr" => {
                    match u16::from_str(&val) {
                        Ok(v) => if !(2020 <= v && v <= 2030) {valid = false; break;},
                        Err(_) => {valid = false; break;}
                    };
                },
                "hgt" => {
                    let (h, unit) = val.split_at(val.len() - 2);
                    match u16::from_str(h) {
                        Ok(height) => {
                            if unit != "in" && unit != "cm" ||
                               unit == "cm" && !HGT_CM_RANGE.contains(&height) ||
                               unit == "in" && !HGT_IN_RANGE.contains(&height) {
                                   valid = false; break;
                            }
                        },
                        Err(_) => {valid = false; break;}
                    }
                },
                "hcl" => {
                    if !HCL_RGX.is_match(&val) {valid = false; break;}
                },
                "ecl" => {
                    match EYE_COLORS.iter().position(|v| v == val) {
                        Some(_) => continue,
                        None => {valid = false; break;}
                    }
                },
                "pid" => {
                    if !PID_RGX.is_match(&val) {valid = false; break;}
                },
                _ => panic!("This isn't possible")
            };
        }

        valid
    }
}
