use day11::day11::{load_data, part1, part2};

fn main() {
    let mut input_data = load_data();

    let answer = part1(&mut input_data.clone());
    println!("\nPart 1: Number of occupied spaces: {}", answer);

    let answer = part2(&mut input_data);
    println!("\nPart 2: Number of occupied spaces: {}", answer);
}
