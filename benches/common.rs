//! Shared input generation for the `TinyAscii` and `String` benchmarks.
//!
//! Both bench binaries build their samples from the same scenarios and seeds, so the
//! benchmark ids (`<method>/<scenario>/<N>`) line up and can be compared directly.

#![allow(unused)]
#![allow(clippy::undocumented_unsafe_blocks)]

use rand::{Rng, RngExt, SeedableRng, rngs::SmallRng};
use tinyascii::{NonNullAscii, TinyAscii};

const SEED: u64 = 115;

pub fn rng() -> SmallRng {
    SmallRng::seed_from_u64(SEED)
}

pub const REPLACEMENT: NonNullAscii = NonNullAscii::QuestionMark;

pub fn sample_tiny_ascii<R: Rng, const N: usize>(rng: &mut R, len: usize) -> TinyAscii<N> {
    let mut buf = [0; N];
    buf.fill_with(|| rng.random_range(1u8..128));
    unsafe { TinyAscii::from_parts(buf, len.min(N)) }
}

pub fn sample_ascii_vec<R: Rng>(rng: &mut R, len: usize) -> Vec<NonNullAscii> {
    let mut buf = Vec::with_capacity(len);
    for _ in 0..len {
        let byte = rng.random_range(1u8..128);
        buf.push(unsafe { NonNullAscii::from_u8_unchecked(byte) });
    }
    buf
}

pub fn sample_ascii_string<R: Rng>(rng: &mut R, len: usize) -> String {
    let mut bytes = vec![0; len];
    bytes.fill_with(|| rng.random_range(1..128));
    unsafe { String::from_utf8_unchecked(bytes) }
}

pub fn sample_non_ascii_string<R: Rng>(rng: &mut R, len: usize) -> String {
    let mut text = String::with_capacity(len);
    while text.len() < len - 4 {
        text.push(rng.random::<char>());
    }
    text.push('😀');
    text
}

/// Runs a generic bench function for every `N` capacity.
#[allow(unused_macros)]
macro_rules! each_cap {
    ($f:ident, $c:expr, <$($cap:literal),* $(,)?>) => {{
        $($f::<$cap>($c);)*
    }};
}
#[allow(unused_imports)]
pub(crate) use each_cap;
