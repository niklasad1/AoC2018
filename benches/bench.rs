#[macro_use]
extern crate criterion;
extern crate aoc_2018;

use aoc_2018::*;
use criterion::Criterion;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day1", |b| b.iter(|| day1::run()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
