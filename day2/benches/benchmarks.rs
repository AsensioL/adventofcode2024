use criterion::{black_box, criterion_group, criterion_main, Criterion};
use skipper::*;


fn criterion_benchmark(c: &mut Criterion) {

    let v2 = [7.0, 8.0, 9.0, 10.0, 11.0, 12.0];

    let input_ref = &v2;

    let mut group = c.benchmark_group("day2_part2");

    group.bench_function("skip_nth0",
        |b| b.iter(|| skip_nth0(black_box(input_ref), 3)));

    group.bench_function("skip_nth1",
        |b| b.iter(|| skip_nth1(black_box(input_ref), 3).collect::<Vec<&f32>>()));

    group.bench_function("skip_nth2",
        |b| b.iter(|| skip_nth2(black_box(input_ref).iter(), 3)));

    group.bench_function("skip_nth3",
        |b| b.iter(|| skip_nth3(black_box(input_ref).iter(), 3)));

    group.bench_function("skip_nth4",
        |b| b.iter(|| skip_nth4(black_box(input_ref).iter(), 3).collect::<Vec<&f32>>()));

    group.bench_function("class",
        |b| b.iter(|| Skipper::new(black_box(input_ref).iter(), 3).collect::<Vec<&f32>>()));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);