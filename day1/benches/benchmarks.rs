use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day1::*;

static DAY_1_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

fn criterion_benchmark(c: &mut Criterion) {

    let input= DAY_1_INPUT.to_string();
    let input_ref = &input;
    let mut group = c.benchmark_group("day1_part1");

    group.bench_function("naive",
        |b| b.iter(|| day1_part1_naive(black_box(input_ref))));

    group.bench_function("no_string_copy",
        |b| b.iter(|| day1_part1_no_string_copy(black_box(input_ref))));

    group.bench_function("final",
        |b| b.iter(|| day1_part1(black_box(input_ref))));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);