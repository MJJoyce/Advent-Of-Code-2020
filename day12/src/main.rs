use day12::day12::{load_data, part1, part2};

fn main() {
    let input_data = load_data();

    let answer = part1(&input_data);
    println!("\nPart 1: Manhattan Distance from start: {}", answer);

    let answer = part2(&input_data);
    println!("\nPart 2: Manhattan Distance from start: {}", answer);
}
