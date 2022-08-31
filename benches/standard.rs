use aoc_2021::days;
use criterion::{black_box, criterion_group, Criterion};

pub fn day_3(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 3 standard");
    let input = days::day_3::input();
    group.bench_function("part 1", |b| {
        b.iter(|| days::day_3::part_one(black_box(&input)))
    });
    group.bench_function("part 2", |b| {
        b.iter(|| days::day_3::part_two(black_box(&input)))
    });
}

pub fn day_8(c: &mut Criterion) {
    let mut group = c.benchmark_group("day 8 standard");
    let input = days::day_8::input();
    group.bench_function("part 1", |b| {
        b.iter(|| days::day_8::part_one(black_box(&input)))
    });
    group.bench_function("part 2", |b| {
        b.iter(|| days::day_8::part_two(black_box(&input)))
    });
    group.finish();
}

criterion_group!(standard, day_3, day_8);
