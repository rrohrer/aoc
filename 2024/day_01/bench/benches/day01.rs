use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("part 2", |b| b.iter(|| unsafe { part2_fast() }));
}
extern "C" {
    pub fn part2_fast() -> i32;
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
