//! `TinyAscii` range benchmarks.
mod common;

use core::hint::black_box;
use core::range::Range;

use common::{each_cap, sample_ascii_vec, sample_tiny_ascii};
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::{Rng, RngExt};
use tinyascii::{NonNullAscii, TinyAscii};

#[derive(Debug)]
pub struct RangeScenario<const N: usize> {
    pub name: &'static str,
    pub base: TinyAscii<N>,
    pub ascii: Vec<NonNullAscii>,
    pub range: Range<usize>,
}

impl<const N: usize> RangeScenario<N> {
    pub fn fill_ascii<R: Rng>(rng: &mut R, base_len: usize) -> Self {
        let range_len = base_len / 2;
        let start = rng.random_range(0..=range_len);
        let end = rng.random_range(start..=base_len);

        let range = Range { start, end };
        let replace_len = rng.random_range(N - base_len..=N - base_len + range_len);

        Self {
            name: "fill_ascii",
            base: sample_tiny_ascii(rng, base_len),
            ascii: sample_ascii_vec(rng, replace_len),
            range,
        }
    }
}

/// Benchmarks a method that mutates a copy of the base string.
pub fn run_scenarios<const N: usize, R>(
    c: &mut Criterion,
    method: &str,
    bench: impl Fn(&mut TinyAscii<N>, &RangeScenario<N>) -> R,
) {
    let mut rng = common::rng();
    let sc = RangeScenario::fill_ascii(&mut rng, N / 2);

    let mut g = c.benchmark_group(method);
    g.bench_function(BenchmarkId::new(sc.name, N), |b| {
        b.iter_batched_ref(
            || sc.base.clone(),
            |s| black_box(bench(s, black_box(&sc))),
            BatchSize::SmallInput,
        );
    });
    g.finish();
}

fn extend_from_within<const N: usize>(c: &mut Criterion) {
    run_scenarios::<N, _>(c, "extend_from_within", |s, sc| {
        s.extend_from_within(sc.range)
    });
}

fn replace_range<const N: usize>(c: &mut Criterion) {
    run_scenarios::<N, _>(c, "replace_range", |s, sc| {
        s.replace_range(sc.range, &sc.ascii)
    });
}

fn remove_range<const N: usize>(c: &mut Criterion) {
    run_scenarios::<N, _>(c, "remove_range", |s, sc| s.remove_range(sc.range));
}

fn all(c: &mut Criterion) {
    each_cap!(extend_from_within, c, <32, 64, 128>);
    each_cap!(replace_range, c, <32, 64, 128>);
    each_cap!(remove_range, c, <32, 64, 128>);
}

criterion_group!(benches, all);
criterion_main!(benches);
