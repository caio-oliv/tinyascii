//! `TinyAscii` constructor benchmarks.

#![allow(clippy::undocumented_unsafe_blocks)]

mod common;

use core::assert_matches;
use core::hint::black_box;

use common::{REPLACEMENT, each_cap, sample_ascii_string, sample_non_ascii_string};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::Rng;
use tinyascii::{TinyAscii, error::WriteAsciiError};

#[derive(Debug)]
pub struct CtorScenario {
    pub name: &'static str,
    pub text: String,
}

impl CtorScenario {
    pub fn fill_ascii<R: Rng>(rng: &mut R, len: usize) -> Self {
        let text = sample_ascii_string(rng, len);

        assert_matches!(TinyAscii::<128>::try_new(&text), Ok(_));

        Self {
            name: "fill_ascii",
            text,
        }
    }

    pub fn fill_non_ascii<R: Rng>(rng: &mut R, len: usize) -> Self {
        let text = sample_non_ascii_string(rng, len);

        assert_matches!(
            TinyAscii::<128>::try_new(&text),
            Err(WriteAsciiError::InvalidAscii)
        );

        Self {
            name: "fill_non_ascii",
            text,
        }
    }
}

/// Benchmarks a function that builds a value from the payload.
pub fn run_scenarios<const N: usize, R>(
    c: &mut Criterion,
    method: &str,
    scenarios: &[CtorScenario],
    bench: impl Fn(&CtorScenario) -> R,
) {
    let mut g = c.benchmark_group(method);
    for sc in scenarios {
        g.bench_function(BenchmarkId::new(sc.name, N), |b| {
            b.iter(|| black_box(bench(black_box(sc))));
        });
    }
    g.finish();
}

fn try_new<const N: usize>(c: &mut Criterion) {
    let mut rng = common::rng();
    let scenarios = [CtorScenario::fill_ascii(&mut rng, N)];
    run_scenarios::<N, _>(c, "try_new", &scenarios, |sc| {
        TinyAscii::<N>::try_new(&sc.text)
    });
}

fn try_from_bytes<const N: usize>(c: &mut Criterion) {
    let mut rng = common::rng();
    let scenarios = [CtorScenario::fill_ascii(&mut rng, N)];
    run_scenarios::<N, _>(c, "try_from_bytes", &scenarios, |sc| {
        TinyAscii::<N>::try_from_bytes(sc.text.as_bytes())
    });
}

fn from_ascii_lossy<const N: usize>(c: &mut Criterion) {
    let mut rng = common::rng();
    let scenarios = [
        CtorScenario::fill_ascii(&mut rng, N),
        CtorScenario::fill_non_ascii(&mut rng, N),
    ];
    run_scenarios::<N, _>(c, "from_ascii_lossy", &scenarios, |sc| {
        TinyAscii::<N>::from_ascii_lossy(sc.text.as_bytes(), black_box(REPLACEMENT))
    });
}

fn all(c: &mut Criterion) {
    each_cap!(try_new, c, <8, 32, 64, 128>);
    each_cap!(try_from_bytes, c, <8, 32, 64, 128>);
    each_cap!(from_ascii_lossy, c, <8, 32, 64, 128>);
}

criterion_group!(benches, all);
criterion_main!(benches);
