use criterion::{criterion_group, criterion_main, Criterion};

mod optimized;
mod standard;
mod comparisons;

use aoc_2021::days;
use optimized::alpha;
use standard::standard;
use comparisons::comparisons;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 1", |b| b.iter(|| days::day_1()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(
    // benches,
    standard,
    alpha,
    comparisons,
);
