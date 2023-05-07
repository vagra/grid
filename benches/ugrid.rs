use criterion::*;

mod bench_mods;

use bench_mods::ugrid::*;


fn ugrid_benchmark(c: &mut Criterion) {
    c.bench_function("bench ugrid 100", |b| b.iter(|| bench_ugrid(black_box(100))));
}

criterion_group!(
    benches,
    ugrid_benchmark
);
criterion_main!(benches);