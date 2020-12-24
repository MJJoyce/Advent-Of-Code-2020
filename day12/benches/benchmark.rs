use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day12::day12::{load_data, part1, part2};

fn part1_benchmark(c: &mut Criterion) {
    let input_data = load_data();
    c.bench_function("day 12 part 1", |b| b.iter(|| part1(black_box(&mut input_data.clone()))));
}

fn part2_benchmark(c: &mut Criterion) {
    let input_data = load_data();

    let mut group = c.benchmark_group("day 12 part 2");
    group.bench_function("day 12 part 2", |b| b.iter(|| part2(black_box(&mut input_data.clone()))));
    group.finish();
}

criterion_group!(benches, part1_benchmark, part2_benchmark);
criterion_main!(benches);
