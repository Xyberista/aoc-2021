use criterion::{black_box, criterion_group, Criterion};

use aoc_2021::days_optimized;

pub fn day_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("day_1 optimized");
    let report = days_optimized::day_1::get_report();
    group.bench_function("part 1", |b| {
        b.iter(|| days_optimized::day_1::part_1(black_box(&report)))
    });
    group.bench_function("part 2", |b| {
        b.iter(|| days_optimized::day_1::part_2(black_box(&report)))
    });
    group.finish();
}

pub fn day_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("day_2 optimized");
    let instructions = days_optimized::day_2::get_instructions();
    group.bench_function("part 1", |b| {
        b.iter(|| days_optimized::day_2::part_1(black_box(&instructions)))
    });
    group.bench_function("part 2", |b| {
        b.iter(|| days_optimized::day_2::part_2(black_box(&instructions)))
    });
    group.finish();
}

pub fn day_15(c: &mut Criterion) {
    let mut group = c.benchmark_group("day_15 optimized");
    let board = days_optimized::day_15::get_board();
    group.bench_function("part 1", |b| {
        b.iter(|| days_optimized::day_15::part_1(black_box(&board)))
    });
    group.bench_function("part 2", |b| {
        b.iter(|| days_optimized::day_15::part_2(black_box(&board)))
    });
    group.finish();
}

criterion_group!(alpha, day_1, day_2);
// criterion_group!(beta, day_6);
criterion_group!(
    gamma, // day_11,
    // day_12,
    day_15
);
// criterion_group!(delta, day_16);
// criterion_group!(epsilon, day_21);
// criterion_group!(zeta, day_26);
