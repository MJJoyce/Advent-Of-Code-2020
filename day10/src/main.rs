use day10::day10::{load_data, part1, part2};

fn main() {
    let input_data = load_data();

    let answer = part1(&input_data);
    println!("\nPart 1: Count 1 differences * Count 3 differences: {}", answer);

    let answer = part2(&input_data);
    println!("\nPart 2: Distinct adapter arrangements: {}", answer);
}
