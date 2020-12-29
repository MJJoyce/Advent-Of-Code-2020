use day13::day13::{load_data, part1, part2};

fn main() {
    let input_data = load_data();

    let answer = part1(input_data.0, input_data.1);
    println!("\nPart 1: {}", answer);
}
