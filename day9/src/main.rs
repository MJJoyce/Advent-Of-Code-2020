use day9::day9::{load_data, part1, part2, part2_dynamic_window};

fn main() {
    let input_data = load_data();

    let (index, invalid_num) = part1(&input_data);
    println!("\nPart 1: Invalid num: {} at index {}", invalid_num, index);

    println!("\nPart 2:");
    let weakness = part2(&input_data, invalid_num);
    println!("\tBrute Force: Weakness: {}", weakness);

    let weakness = part2_dynamic_window(&input_data, invalid_num);
    println!("\tDynamic Window: Weakness: {}", weakness);
}
