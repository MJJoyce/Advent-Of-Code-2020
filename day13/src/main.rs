use day13::day13::{load_data, part1, part2, part2_v2};

fn main() {
    let input_data = load_data();

    let part1_data = input_data.1.iter().map(|i| i.0).collect();
    let answer = part1(input_data.0, part1_data);
    println!("\nPart 1: {}", answer);

    // Initial part2 answer is painfully slow. Somewhere on the order
    // of 7 minutes to execute.
    //let answer = part2(input_data.1);

    let answer = part2_v2(input_data.1);
    println!("\nPart 2: {}", answer);
}
