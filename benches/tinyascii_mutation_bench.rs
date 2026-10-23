//! `TinyAscii` push and insert benchmarks.

#![allow(clippy::undocumented_unsafe_blocks)]

mod common;

use core::assert_matches;
use core::hint::black_box;

use common::{
    REPLACEMENT, each_cap, sample_ascii_string, sample_non_ascii_string, sample_tiny_ascii,
};
use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use rand::{Rng, RngExt};
use tinyascii::{NonNullAscii, TinyAscii, error::WriteAsciiError};

#[derive(Debug)]
pub struct MutationScenario<const N: usize> {
    pub name: &'static str,
    pub base: TinyAscii<N>,
    pub text: String,
    pub ascii: Vec<NonNullAscii>,
    pub idx: usize,
}

impl<const N: usize> MutationScenario<N> {
    pub fn fill_ascii<R: Rng>(rng: &mut R, base_len: usize) -> Self {
        let push_len = N - base_len;
        let idx = rng.random_range(0..base_len);

        let text = sample_ascii_string(rng, push_len);
        assert_matches!(TinyAscii::<128>::try_new(&text), Ok(_));

        let ascii = text
            .bytes()
            .map(|b| unsafe { NonNullAscii::from_u8_unchecked(b) })
            .collect();

        Self {
            name: "fill_ascii",
            base: sample_tiny_ascii(rng, base_len),
            text,
            ascii,
            idx,
        }
    }

    pub fn fill_non_ascii<R: Rng>(rng: &mut R, base_len: usize) -> Self {
        let push_len = N - base_len;
        let idx = rng.random_range(0..base_len);

        let text = sample_non_ascii_string(rng, push_len);
        assert_matches!(
            TinyAscii::<128>::try_new(&text),
            Err(WriteAsciiError::InvalidAscii)
        );

        let ascii = text
            .bytes()
            .map(|b| NonNullAscii::from_u8(b).unwrap_or(REPLACEMENT))
            .collect();

        Self {
            name: "fill_non_ascii",
            base: sample_tiny_ascii(rng, base_len),
            text,
            ascii,
            idx,
        }
    }
}

/// Benchmarks a method that mutates a copy of the base string.
pub fn run_scenarios<const N: usize, R>(
    c: &mut Criterion,
    method: &str,
    scenarios: &[MutationScenario<N>],
    op: impl Fn(&mut TinyAscii<N>, &MutationScenario<N>) -> R,
) {
    let mut g = c.benchmark_group(method);
    for sc in scenarios {
        g.bench_function(BenchmarkId::new(sc.name, N), |b| {
            b.iter_batched_ref(
                || sc.base.clone(),
                |s| black_box(op(s, black_box(sc))),
                BatchSize::SmallInput,
            );
        });
    }
    g.finish();
}

fn try_push<const N: usize>(c: &mut Criterion) {
    let mut rng = common::rng();
    let scenarios = [MutationScenario::fill_ascii(&mut rng, N / 2)];
    run_scenarios::<N, _>(c, "try_push", &scenarios, |s, sc| {
        for ascii in &sc.ascii {
            let _ = black_box(s.try_push(*ascii));
        }
    });
}

fn try_push_u8<const N: usize>(c: &mut Criterion) {
    let mut rng = common::rng();
    let scenarios = [MutationScenario::fill_ascii(&mut rng, N / 2)];
    run_scenarios::<N, _>(c, "try_push_u8", &scenarios, |s, sc| {
        for byte in sc.text.bytes() {
            let _ = black_box(s.try_push_u8(byte));
        }
    });
}

fn try_push_slice<const N: usize>(c: &mut Criterion) {
    let mut rng = common::rng();
    let scenarios = [MutationScenario::fill_ascii(&mut rng, N / 2)];
    run_scenarios::<N, _>(c, "try_push_slice", &scenarios, |s, sc| {
        s.try_push_slice(&sc.ascii)
    });
}

fn try_push_bytes<const N: usize>(c: &mut Criterion) {
    let mut rng = common::rng();
    let scenarios = [
        MutationScenario::fill_ascii(&mut rng, N / 2),
        MutationScenario::fill_non_ascii(&mut rng, N / 2),
    ];
    run_scenarios::<N, _>(c, "try_push_bytes", &scenarios, |s, sc| {
        s.try_push_bytes(sc.text.as_bytes())
    });
}

fn try_push_str<const N: usize>(c: &mut Criterion) {
    let mut rng = common::rng();
    let scenarios = [
        MutationScenario::fill_ascii(&mut rng, N / 2),
        MutationScenario::fill_non_ascii(&mut rng, N / 2),
    ];
    run_scenarios::<N, _>(c, "try_push_str", &scenarios, |s, sc| {
        s.try_push_str(&sc.text)
    });
}

fn try_push_lossy<const N: usize>(c: &mut Criterion) {
    let mut rng = common::rng();
    let scenarios = [
        MutationScenario::fill_ascii(&mut rng, N / 2),
        MutationScenario::fill_non_ascii(&mut rng, N / 2),
    ];
    run_scenarios::<N, _>(c, "try_push_lossy", &scenarios, |s, sc| {
        s.try_push_lossy(sc.text.as_bytes(), black_box(REPLACEMENT))
    });
}

fn try_insert<const N: usize>(c: &mut Criterion) {
    let mut rng = common::rng();
    let scenarios = [MutationScenario::fill_ascii(&mut rng, N / 2)];
    run_scenarios::<N, _>(c, "try_insert", &scenarios, |s, sc| {
        for ascii in &sc.ascii {
            let _ = black_box(s.try_insert(sc.idx, *ascii));
        }
    });
}

fn try_insert_slice<const N: usize>(c: &mut Criterion) {
    let mut rng = common::rng();
    let scenarios = [MutationScenario::fill_ascii(&mut rng, N / 2)];
    run_scenarios::<N, _>(c, "try_insert_slice", &scenarios, |s, sc| {
        s.try_insert_slice(sc.idx, &sc.ascii)
    });
}

fn all(c: &mut Criterion) {
    each_cap!(try_push, c, <128>);
    each_cap!(try_push_u8, c, <128>);
    each_cap!(try_insert, c, <128>);
    each_cap!(try_push_slice, c, <32, 128>);
    each_cap!(try_push_bytes, c, <32, 128>);
    each_cap!(try_push_str, c, <32, 128>);
    each_cap!(try_push_lossy, c, <32, 128>);
    each_cap!(try_insert_slice, c, <32, 128>);
}

criterion_group!(benches, all);
criterion_main!(benches);
