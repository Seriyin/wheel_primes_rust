use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prime_sieves::pritchard::*;
use prime_sieves::segmented::*;
use rand::distributions::Uniform;
use rand::prelude::{Distribution, SeedableRng};
use test_utils::FIXED_SEED;

pub fn criterion_benchmark(c: &mut Criterion) {
    let distribution = Uniform::from(0..20000000);
    let rng = rand::rngs::StdRng::from_seed(FIXED_SEED);
    let vec: Vec<usize> = distribution.sample_iter(rng).take(20000).collect();
    c.bench_function("sieve_pritchard", |b| {
        b.iter(|| {
            apply_sieve(sieve_pritchard, vec.iter().cycle());
        })
    });
    c.bench_function("sieve_segmented", |b| {
        b.iter(|| {
            apply_sieve(sieve_segmented, vec.iter().cycle());
        })
    });
}

fn apply_sieve<'a, F, T>(sieve: F, mut vec: impl Iterator<Item = &'a usize>) -> T
where F: Fn(usize) -> T {
    sieve(black_box(unsafe { *vec.next().unwrap_unchecked()}))
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
