use std::fs::File;
use std::io::{BufRead, BufReader};

static DIRECTIONS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

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
                    continue;
                }

                let occupied = occupied_neighbors(y, x, true, spaces);
                if *space == '#' && occupied >= 4 || *space == 'L' && occupied == 0 {
                    update_spaces.push((x, y));
                }
            }
        }

        if update_spaces.len() == 0 {
            break;
        }

        for (x, y) in update_spaces.iter() {
            spaces[*y][*x] = if spaces[*y][*x] == 'L' { '#' } else { 'L' };
        }
        update_spaces.clear();
    }

    let mut occupied_count = 0;
    for row in spaces.iter() {
        for space in row {
            if *space == '#' {
                occupied_count += 1;
            }
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
                    continue;
                }

                let occupied = occupied_neighbors(y, x, false, spaces);
                if *space == '#' && occupied >= 5 || *space == 'L' && occupied == 0 {
                    update_spaces.push((x, y));
                }
            }
        }

        if update_spaces.len() == 0 {
            break;
        }

        for (x, y) in update_spaces.iter() {
            spaces[*y][*x] = if spaces[*y][*x] == 'L' { '#' } else { 'L' };
        }
        update_spaces.clear();
    }

    let mut occupied_count = 0;
    for row in spaces.iter() {
        for space in row {
            if *space == '#' {
                occupied_count += 1;
            }
        }
    }

    occupied_count
}

/// Calculate the number of occupied neighbors for a given location
///
/// # Arguments:
///
/// * `row` - The row of the location to check from
/// * `col` - The row of the location to check from
/// * `spaces` - The 2D space in which to search
/// * `immediate_neighbor` - Flag indicating if we should only check the nearest
///         neighbor in the indicated direction.
///
/// Returns:
///     The number of occupied neighbor spaces
fn occupied_neighbors(
    row: usize,
    col: usize,
    immediate_neighbors: bool,
    spaces: &Vec<Vec<char>>,
) -> u64 {

    let mut occupied_neighbors = 0;

    for c in DIRECTIONS.iter() {
        if let Some((nr, nc)) = locate_nearest_seat(row, col, c.0, c.1, spaces, immediate_neighbors)
        {
            if spaces[nr][nc] == '#' {
                occupied_neighbors += 1;
            }
        }
    }

    occupied_neighbors
}

/// Locate the nearest seat in a given direction from a given location
///
/// # Arguments:
///
/// * `row` - The row of the location to check from
/// * `col` - The row of the location to check from
/// * `row_dif` - The row direction to iterate in (one of -1, 0, 1)
/// * `col_dif` - The col direction to iterate in (one of -1, 0, 1)
/// * `spaces` - The 2D space in which to search
/// * `immediate_neighbor` - Flag indicating if we should only check the nearest
///         neighbor in the indicated direction.
fn locate_nearest_seat(
    row: usize,
    col: usize,
    row_dif: i64,
    col_dif: i64,
    spaces: &Vec<Vec<char>>,
    immediate_neighbor: bool,
) -> Option<(usize, usize)> {

    let mut row = row as i64 + row_dif;
    let mut col = col as i64 + col_dif;

    while row >= 0 && row <= (spaces.len() - 1) as i64 &&
          col >= 0 && col <= (spaces[0].len() - 1) as i64
    {
        if spaces[row as usize][col as usize] != '.' {
            return Some((row as usize, col as usize));
        }

        if immediate_neighbor {
            return None
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

        assert_eq!(part2(&mut spaces), 26);
    }
}
