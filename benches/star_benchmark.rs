use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 1", |b| b.iter(|| 0));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
