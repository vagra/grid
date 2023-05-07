use criterion::*;

mod bench_mods;

use bench_mods::{*, dgrid::*};



const COUNTS: usize = 10000;

fn dgrid_benchmark(c: &mut Criterion) {
    let rng = &mut init_seed();
    let grid = &mut create_grid(COUNTS, rng);
    let actors = &mut create_actors(&grid, rng);

    c.bench_function("bench dgrid 10000", |b| b.iter(|| {
        move_actors(actors, grid, rng);
        turn_actors(actors, grid, rng);
    }));
}

criterion_group!(
    benches,
    dgrid_benchmark
);
criterion_main!(benches);