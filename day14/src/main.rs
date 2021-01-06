use day14::day14::{load_data, part1, part2};

fn main() {
    let answer = part1();
    assert_eq!(answer, 11884151942312);
    println!("\nPart 1: {}", answer);

    let answer = part2();
    assert_eq!(answer, 2625449018811);
    println!("\nPart 2: {}", answer);
}
