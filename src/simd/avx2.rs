#![allow(unused)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::ascii::NonNullAscii;
use crate::tinyascii::MAX_LEN;

/// Mask with all bits set in lanes holding an invalid byte (NUL or > 127).
///
/// `1 > v` as signed bytes holds exactly for `v == 0` and `v >= 0x80`.
#[inline]
#[target_feature(enable = "avx2")]
fn invalid_mask_256(v: __m256i) -> __m256i {
    _mm256_cmpgt_epi8(_mm256_set1_epi8(1), v)
}

/// 128-bit version of [`invalid_mask_256`].
#[inline]
#[target_feature(enable = "avx2")]
fn invalid_mask_128(v: __m128i) -> __m128i {
    _mm_cmpgt_epi8(_mm_set1_epi8(1), v)
}

/// Whether a single byte is valid `TinyAscii` content.
#[inline(always)]
const fn is_valid_byte(b: u8) -> bool {
    b.wrapping_sub(1) < 127
}

/// AVX2 version of [`crate::simd::is_valid_tinyascii`].
///
/// Inputs are processed with a few (possibly overlapping) vector loads
/// chosen by length class, so no byte-by-byte tail loop is needed.
///
/// # Safety
///
/// Same sefety requirements as [`crate::simd::is_valid_tinyascii`].
#[inline]
#[target_feature(enable = "avx2")]
pub(super) unsafe fn is_valid_tinyascii(src: &[u8]) -> bool {
    // SAFETY: `src` length is less or equal than `N` as ensured
    // by the caller.
    unsafe { core::hint::assert_unchecked(MAX_LEN >= src.len()) };

    let len = src.len();
    let p = src.as_ptr();

    if len >= 32 {
        let mut acc = _mm256_setzero_si256();
        let mut i = 0;

        while i + 32 < len {
            // SAFETY: `i + 32 < len`, so 32 bytes are readable at `i`.
            let v = unsafe { _mm256_loadu_si256(p.add(i).cast()) };
            acc = _mm256_or_si256(acc, invalid_mask_256(v));
            i += 32;
        }

        // Last (possibly overlapping) chunk.
        // SAFETY: `len >= 32`, so 32 bytes are readable at `len - 32`.
        let v = unsafe { _mm256_loadu_si256(p.add(len - 32).cast()) };
        acc = _mm256_or_si256(acc, invalid_mask_256(v));

        return _mm256_testz_si256(acc, acc) != 0;
    }

    if len >= 16 {
        // SAFETY: `len >= 16`, so both 16-byte windows are in bounds.
        let (a, b) = unsafe {
            (
                _mm_loadu_si128(p.cast()),
                _mm_loadu_si128(p.add(len - 16).cast()),
            )
        };
        let m = _mm_or_si128(invalid_mask_128(a), invalid_mask_128(b));
        return _mm_testz_si128(m, m) != 0;
    }

    if len >= 8 {
        // SAFETY: `len >= 8`, so both 8-byte windows are in bounds.
        let (a, b) = unsafe {
            (
                _mm_loadl_epi64(p.cast()),
                _mm_loadl_epi64(p.add(len - 8).cast()),
            )
        };
        let m = invalid_mask_128(_mm_unpacklo_epi64(a, b));
        return _mm_testz_si128(m, m) != 0;
    }

    for idx in 0..len {
        // SAFETY: `src` is within len bounds (`len = src.len()`).
        let b = *unsafe { src.get_unchecked(idx) };
        if !is_valid_byte(b) {
            return false;
        }
    }

    true
}

/// AVX2 version of [`crate::simd::fill_ascii`].
///
/// Inputs are processed with a few (possibly overlapping) vector accesses
/// chosen by length class, so no byte-by-byte tail loop is needed. Overlapping
/// stores are safe because each output byte only depends on the same source byte.
///
/// # Safety
///
/// Same sefety requirements as [`crate::simd::fill_ascii`].
#[inline]
#[target_feature(enable = "avx2")]
pub(super) unsafe fn fill_ascii(dst: &mut [u8], src: &[u8], replacement: NonNullAscii) {
    // SAFETY: `src` length is less or equal than `N` as ensured
    // by the caller.
    unsafe { core::hint::assert_unchecked(MAX_LEN >= src.len()) };

    // SAFETY: `dst` length is greater or equal than `src` length
    // as ensured by the caller.
    unsafe { core::hint::assert_unchecked(dst.len() >= src.len()) };

    let len = src.len();
    let p = src.as_ptr();
    let d = dst.as_mut_ptr();
    let repl = replacement.to_u8();

    if len >= 32 {
        let repl = _mm256_set1_epi8(repl as i8);
        let mut i = 0;

        while i + 32 < len {
            // SAFETY: `i + 32 < len <= dst.len()`, so 32 bytes are readable
            // from `src` and writable in `dst` at `i`.
            unsafe {
                let v = _mm256_loadu_si256(p.add(i).cast());
                let f = _mm256_blendv_epi8(v, repl, invalid_mask_256(v));
                _mm256_storeu_si256(d.add(i).cast(), f);
            }
            i += 32;
        }

        // Last (possibly overlapping) chunk.
        // SAFETY: `len >= 32`, so 32 bytes are in bounds at `len - 32`.
        unsafe {
            let v = _mm256_loadu_si256(p.add(len - 32).cast());
            let f = _mm256_blendv_epi8(v, repl, invalid_mask_256(v));
            _mm256_storeu_si256(d.add(len - 32).cast(), f);
        }
        return;
    }

    if len >= 16 {
        let repl = _mm_set1_epi8(repl as i8);

        // SAFETY: `len >= 16`, so both 16-byte windows are in bounds of
        // `src` and `dst`. Both loads precede the stores.
        unsafe {
            let a = _mm_loadu_si128(p.cast());
            let b = _mm_loadu_si128(p.add(len - 16).cast());
            let fa = _mm_blendv_epi8(a, repl, invalid_mask_128(a));
            let fb = _mm_blendv_epi8(b, repl, invalid_mask_128(b));
            _mm_storeu_si128(d.cast(), fa);
            _mm_storeu_si128(d.add(len - 16).cast(), fb);
        }
        return;
    }

    if len >= 8 {
        let repl = _mm_set1_epi8(repl as i8);

        // SAFETY: `len >= 8`, so both 8-byte windows are in bounds of
        // `src` and `dst`.
        unsafe {
            let a = _mm_loadl_epi64(p.cast());
            let b = _mm_loadl_epi64(p.add(len - 8).cast());
            let fa = _mm_blendv_epi8(a, repl, invalid_mask_128(a));
            let fb = _mm_blendv_epi8(b, repl, invalid_mask_128(b));
            _mm_storel_epi64(d.cast(), fa);
            _mm_storel_epi64(d.add(len - 8).cast(), fb);
        }
        return;
    }

    for idx in 0..len {
        // SAFETY: `src` is within len bounds (`len = src.len()`).
        let b = *unsafe { src.get_unchecked(idx) };
        let val = if is_valid_byte(b) { b } else { repl };
        // SAFETY: `dst` is within `src.len()` bounds.
        *unsafe { dst.get_unchecked_mut(idx) } = val;
    }
}
