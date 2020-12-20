use day10::day10::{load_data, part1, part2};

fn main() {
    let mut input_data = load_data();

    input_data.sort_unstable();

    let answer = part1(&input_data);
    println!("\nPart 1: Count 1 differences * Count 3 differences: {}", answer);
}
