use day8::day8;

fn main() {
    let cmds = day8::load_data();

    let cpu_acc = day8::part1(&cmds);
    println!("\nPart 1: Acc prior to infinite loop: {}", cpu_acc);

    let (swap_index, cpu_acc) = day8::part2(&cmds);
    println!(
        "\nPart 2: Program terminated with cmd {} swapped. Acc: {}",
        swap_index, cpu_acc
    );
}

