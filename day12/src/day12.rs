use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn load_data() -> Vec<(char, i32)> {
    let br = BufReader::new(File::open("./input/input.txt").unwrap());

    let mut lines: Vec<(char, i32)> = Vec::new();
    for line in br.lines() {
        let line = line.unwrap();
        lines.push((line.chars().next().unwrap(), i32::from_str(&line[1..]).unwrap()));
    }

    lines
}

pub fn part1(cmds: &Vec<(char, i32)>) -> i32 {
    let mut ship = Ship::new();
    
    for (action, value) in cmds {
        ship.exec_action(action, *value);
    }

    ship.x.abs() + ship.y.abs()
}

pub fn part2() {

}

#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West
}

static facing_change: [Direction; 4] = [Direction::North, Direction::East, Direction::South, Direction::West];

struct Ship {
    facing: Direction,
    x: i32,
    y: i32
}

impl Ship {
    fn new() -> Self {
        Ship {
            facing: Direction::East,
            x: 0,
            y: 0
        }
    }

    fn exec_action(&mut self, action: &char, value: i32) {
        match action {
            'N' => self.y += value,
            'S' => self.y -= value,
            'E' => self.x += value,
            'W' => self.x -= value,
            'L' => {
                let cur_facing_index = facing_change.iter().position(|f| *f == self.facing).unwrap() as i32;
                let ticks = value / 90;
                let new_facing = (cur_facing_index - ticks).rem_euclid(4) as usize;
                //self.facing = facing_change[new_facing].clone();
                self.facing = match new_facing {
                    0 => Direction::North,
                    1 => Direction::East,
                    2 => Direction::South,
                    3 => Direction::West,
                    _ => panic!("Not possible")
                }
            },
            'R' => {
                let cur_facing_index = facing_change.iter().position(|f| *f == self.facing).unwrap() as i32;
                let ticks = value / 90;
                let new_facing = (cur_facing_index + ticks).rem_euclid(4) as usize;
                //self.facing = facing_change[new_facing].clone();
                self.facing = match new_facing {
                    0 => Direction::North,
                    1 => Direction::East,
                    2 => Direction::South,
                    3 => Direction::West,
                    _ => panic!("Not possible")
                }
            },
            'F' => {
                match self.facing {
                    Direction::North => self.y += value,
                    Direction::South => self.y -= value,
                    Direction::East => self.x += value,
                    Direction::West => self.x -= value,
                }
            }
            _ => panic!("Invalid action")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let cmds = vec![('F', 10), ('N', 3), ('F', 7), ('R', 90), ('F', 11)];
        assert_eq!(part1(&cmds), 25);
    }
}
