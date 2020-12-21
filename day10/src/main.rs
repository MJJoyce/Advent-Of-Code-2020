use day10::day10::{load_data, part1, part2, part2_min_mem};

fn main() {
    let input_data = load_data();

    let answer = part1(&input_data);
    println!("\nPart 1: Count 1 differences * Count 3 differences: {}", answer);

    println!("\nPart 2:");
    let answer = part2(&input_data);
    println!("\tDistinct adapter arrangements: {}", answer);

    let answer = part2_min_mem(&input_data);
    println!("\tDistinct adapter arrangements (min mem): {}", answer);
}
