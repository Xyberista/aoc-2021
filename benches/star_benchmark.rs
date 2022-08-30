use criterion::{criterion_group, criterion_main, Criterion};

mod optimized;

use aoc_2021::{days, days_optimized, utils::Output};
use optimized::{complete, parts};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 1", |b| b.iter(|| 0));
}

pub fn day_15_combined(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 15");
    group.sample_size(10);
    group.bench_function("standard", |b| b.iter(|| days::day_15(Output::Return)));
    group.bench_function("optimized", |b| {
        b.iter(|| days_optimized::day_15(Output::Return))
    });
    group.finish();
}

// criterion_group!(benches, criterion_benchmark);
criterion_group!(benches, day_15_combined);
criterion_main!(benches, complete, parts);
