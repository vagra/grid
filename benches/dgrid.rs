use criterion::*;

mod bench_mods;

use bench_mods::dgrid::*;


fn dgrid_benchmark(c: &mut Criterion) {
    c.bench_function("bench dgrid 100", |b| b.iter(|| bench_dgrid(black_box(100))));
}

criterion_group!(
    benches,
    dgrid_benchmark
);
criterion_main!(benches);