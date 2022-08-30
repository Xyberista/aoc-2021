use criterion::{black_box, criterion_group, Criterion};

use aoc_2021::{days_optimized, utils::Output};

pub fn day_15_parts(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 15 parts");
    let board = days_optimized::day_15::get_board();
    group.bench_function("day 15 part 1", |b| {
        b.iter(|| days_optimized::day_15::part_1(black_box(&board)))
    });
    group.bench_function("day 15 part 2", |b| {
        b.iter(|| days_optimized::day_15::part_2(black_box(&board)))
    });
    group.finish();
}

pub fn day_15_complete(c: &mut Criterion) {
    c.bench_function("day 15 complete", |b| {
        b.iter(|| days_optimized::day_15(Output::Return))
    });
}

criterion_group!(parts, day_15_parts);
criterion_group!(complete, day_15_complete);
