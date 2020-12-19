//extern crate day8;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day8::day8::{load_data, part1, part2};
//use day8::{load_data, part1, part2};

//let cmds = load_data();

//part1(&cmds);
//part2(&cmds);

fn part1_benchmark(c: &mut Criterion) {
    let cmds = load_data();
    c.bench_function("day 8 part 1", |b| b.iter(|| part1(black_box(&cmds))));
}

fn part2_benchmark(c: &mut Criterion) {
    let cmds = load_data();
    c.bench_function("day 8 part 2", |b| b.iter(|| part2(black_box(&cmds))));
}

criterion_group!(benches, part1_benchmark, part2_benchmark);
//criterion_group!(benches, part1_benchmark);
//criterion_group!(benches, part2_benchmark);
criterion_main!(benches);
