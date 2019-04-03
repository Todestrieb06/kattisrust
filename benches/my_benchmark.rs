#[macro_use]
extern crate criterion;

use criterion::Criterion;

fn benchmarked(s: String) {}

fn benchmark(c: &mut Criterion) {
    c.bench_function("bench", |b| b.iter(|| benchmarked("0 1 2 2 2 7".to_string())));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);