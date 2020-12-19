use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day9::day9::{load_data, part1, part2, part2_dynamic_window};

fn part1_benchmark(c: &mut Criterion) {
    let input_data = load_data();
    c.bench_function("day 9 part 1", |b| b.iter(|| part1(black_box(&input_data))));
}

fn part2_benchmark(c: &mut Criterion) {
    let input_data = load_data();
    let target = 1212510616;

    let mut group = c.benchmark_group("day 9 part 2");
    group.bench_function("day 9 part 2 brute force", |b| b.iter(|| part2(black_box(&input_data), target)));
    group.bench_function("day 9 part 2 dynamic window", |b| b.iter(|| part2_dynamic_window(black_box(&input_data), target)));
    group.finish();
}

criterion_group!(benches, part1_benchmark, part2_benchmark);
criterion_main!(benches);
