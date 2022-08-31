use aoc_2021::{days, days_optimized};
use criterion::{black_box, criterion_group, Criterion};

pub fn day_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("day_1 comparison");

    let input = days::day_1::input();
    group.bench_function("part 1 standard", |b| {
        b.iter(|| days::day_1::part_one(&input))
    });
    group.bench_function("part 2 standard", |b| {
        b.iter(|| days::day_1::part_one(&input))
    });

    let report = days_optimized::day_1::get_report();
    group.bench_function("part 1 optimized", |b| {
        b.iter(|| days_optimized::day_1::part_1(black_box(&report)))
    });
    group.bench_function("part 2 optimized", |b| {
        b.iter(|| days_optimized::day_1::part_2(black_box(&report)))
    });
    group.finish()
}

criterion_group!(comparisons, day_1);
