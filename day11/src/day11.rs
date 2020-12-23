use std::collections::HashMap; use std::io::{BufRead, BufReader};
use std::fs::File;

use itertools::Itertools;

pub fn load_data() -> Vec<Vec<char>> {
    let br = BufReader::new(File::open("./input/input.txt").unwrap());

    let mut spaces: Vec<Vec<char>> = Vec::new();
    for line in br.lines() {
        let line = line.unwrap();
        spaces.push(line.trim().chars().collect());
    }

    spaces
}

pub fn part1(spaces: &mut Vec<Vec<char>>) -> u64 {
    let mut update_spaces: Vec<(usize, usize)> = Vec::new();

    loop {
        for (y, row) in spaces.iter().enumerate() {
            for (x, space) in row.iter().enumerate() {
                if spaces[y][x] == '.' {
                    continue
                }

                let mut occupied_neighbors = 0;

                if y > 0 {
                    if spaces[y - 1][x] == '#' {occupied_neighbors += 1;}

                    if x > 0 {
                        if spaces[y - 1][x - 1] == '#' {occupied_neighbors += 1;}
                    }

                    if x < row.len() - 1 {
                        if spaces[y - 1][x + 1] == '#' {occupied_neighbors += 1;}
                    }
                }

                if x > 0 {
                    if spaces[y][x - 1] == '#' {occupied_neighbors += 1;}
                }

                if x < row.len() - 1 {
                    if spaces[y][x + 1] == '#' {occupied_neighbors += 1;}
                }

                if y < spaces.len() - 1 {
                    if spaces[y + 1][x] == '#' {occupied_neighbors += 1;}

                    if x > 0 {
                        if spaces[y + 1][x - 1] == '#' {occupied_neighbors += 1;}
                    }

                    if x < row.len() - 1 {
                        if spaces[y + 1][x + 1] == '#' {occupied_neighbors += 1;}
                    }
                }

                if *space == '#' && occupied_neighbors >= 4 || *space == 'L' && occupied_neighbors == 0 {
                    update_spaces.push((x, y));
                }
            }
        }

        if update_spaces.len() == 0 {
            break
        }

        for (x, y) in &update_spaces {
            spaces[*y][*x] = if spaces[*y][*x] == 'L' {'#'} else {'L'};
        }

        update_spaces.clear();
    }

    let mut occupied_count = 0;
    for row in spaces.iter() {
        for space in row {
            if *space == '#' {occupied_count += 1;}
        }
    }

    occupied_count
}

pub fn part2(spaces: &mut Vec<Vec<char>>) -> u64 {
    let mut update_spaces: Vec<(usize, usize)> = Vec::new();

    loop {
        for (y, row) in spaces.iter().enumerate() {
            for (x, space) in row.iter().enumerate() {
                if spaces[y][x] == '.' {
                    continue
                }

                let mut occupied_neighbors = 0;
                for c in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)].iter() {
                    if let Some((nr, nc)) = locate_nearest_seat(y, x, c.0, c.1, spaces) {
                        if spaces[nr][nc] == '#' {
                            occupied_neighbors += 1;
                        }
                    }

                }

                if *space == '#' && occupied_neighbors >= 5 || *space == 'L' && occupied_neighbors == 0 {
                    update_spaces.push((x, y));
                }
            }
        }

        if update_spaces.len() == 0 {
            break
        }

        for (x, y) in &update_spaces {
            spaces[*y][*x] = if spaces[*y][*x] == 'L' {'#'} else {'L'};
        }

        update_spaces.clear();
    }

    let mut occupied_count = 0;
    for row in spaces.iter() {
        for space in row {
            if *space == '#' {occupied_count += 1;}
        }
    }

    occupied_count
}

fn locate_nearest_seat(row: usize, col: usize, row_dif: i64, col_dif: i64, spaces: &[Vec<char>]) -> Option<(usize, usize)> {
    let mut row = row as i64 + row_dif;
    let mut col = col as i64 + col_dif;

    while row >= 0 && row <= (spaces.len() - 1) as i64 && col >= 0 && col <= (spaces[0].len() - 1) as i64 {
        if spaces[row as usize][col as usize] != '.' {
            return Some((row as usize, col as usize));
        }

        row += row_dif;
        col += col_dif;
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let mut spaces: Vec<Vec<char>> = Vec::new();
        spaces.push("L.LL.LL.LL".to_string().chars().collect());
        spaces.push("LLLLLLL.LL".to_string().chars().collect());
        spaces.push("L.L.L..L..".to_string().chars().collect());
        spaces.push("LLLL.LL.LL".to_string().chars().collect());
        spaces.push("L.LL.LL.LL".to_string().chars().collect());
        spaces.push("L.LLLLL.LL".to_string().chars().collect());
        spaces.push("..L.L.....".to_string().chars().collect());
        spaces.push("LLLLLLLLLL".to_string().chars().collect());
        spaces.push("L.LLLLLL.L".to_string().chars().collect());
        spaces.push("L.LLLLL.LL".to_string().chars().collect());

        println!("{}", part2(&mut spaces));
        assert!(false);
    }
}
