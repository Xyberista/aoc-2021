use aoc_2021::{days, days_optimized, utils::Output};
use criterion::{criterion_group, Criterion};

pub fn day_1(c: &mut Criterion) {
    let mut group = c.benchmark_group("day_1 full_comparison");
    group.bench_function("standard", |b| b.iter(|| days::day_1()));
    group.bench_function("optimized", |b| {
        b.iter(|| days_optimized::day_1(Output::Print))
    });
    group.finish()
}

pub fn day_2(c: &mut Criterion) {
    let mut group = c.benchmark_group("day_2 full_comparison");
    group.bench_function("standard", |b| b.iter(|| days::day_2()));
    group.bench_function("optimized", |b| {
        b.iter(|| days_optimized::day_2(Output::Print))
    });
    group.finish()
}

criterion_group!(comparisons, day_1, day_2);
