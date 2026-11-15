#![allow(unused)]

#[cfg(target_arch = "x86")]
use core::arch::x86::*;

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
use core::mem::MaybeUninit;

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

/// AVX2 version of [`crate::simd::is_valid_tinyascii`].
///
/// # Safety
///
/// Same safety requirements as [`crate::simd::is_valid_tinyascii`].
#[inline]
#[target_feature(enable = "avx2")]
pub(super) unsafe fn is_valid_tinyascii(src: &[u8]) -> bool {
    #![allow(clippy::undocumented_unsafe_blocks)]

    // SAFETY: `src` length is less or equal than `N` as ensured
    // by the caller.
    unsafe { core::hint::assert_unchecked(MAX_LEN >= src.len()) };

    let len = src.len();
    let p = src.as_ptr();

    if len > 64 {
        let a0 = unsafe { _mm256_loadu_si256(p.cast()) };
        let a1 = unsafe { _mm256_loadu_si256(p.add(32).cast()) };
        let b1 = unsafe { _mm256_loadu_si256(p.add(len - 64).cast()) };
        let b0 = unsafe { _mm256_loadu_si256(p.add(len - 32).cast()) };
        let m0 = _mm256_or_si256(invalid_mask_256(a0), invalid_mask_256(a1));
        let m1 = _mm256_or_si256(invalid_mask_256(b1), invalid_mask_256(b0));
        let m = _mm256_or_si256(m0, m1);
        return _mm256_testz_si256(m, m) != 0;
    }

    if len >= 32 {
        let a = unsafe { _mm256_loadu_si256(p.cast()) };
        let b = unsafe { _mm256_loadu_si256(p.add(len - 32).cast()) };
        let m = _mm256_or_si256(invalid_mask_256(a), invalid_mask_256(b));
        return _mm256_testz_si256(m, m) != 0;
    }

    if len >= 16 {
        let a = unsafe { _mm_loadu_si128(p.cast()) };
        let b = unsafe { _mm_loadu_si128(p.add(len - 16).cast()) };
        let m = _mm_or_si128(invalid_mask_128(a), invalid_mask_128(b));
        return _mm_testz_si128(m, m) != 0;
    }

    if len >= 8 {
        let a = unsafe { p.cast::<u64>().read_unaligned() };
        let b = unsafe { p.add(len - 8).cast::<u64>().read_unaligned() };
        return !(has_invalid_u64(a) | has_invalid_u64(b));
    }

    if len >= 4 {
        let a = unsafe { p.cast::<u32>().read_unaligned() };
        let b = unsafe { p.add(len - 4).cast::<u32>().read_unaligned() };
        return !has_invalid_u64(u64::from(a) | (u64::from(b) << 32));
    }

    if len >= 2 {
        let a = unsafe { p.cast::<u16>().read_unaligned() };
        let b = unsafe { p.add(len - 2).cast::<u16>().read_unaligned() };
        return !has_invalid_u32(u32::from(a) | (u32::from(b) << 16));
    }

    if len == 1 {
        return NonNullAscii::is_valid_u8(unsafe { p.read() });
    }

    true
}

/// AVX2 version of [`crate::simd::copy_valid_tinyascii`].
///
/// # Safety
///
/// Same safety requirements as [`crate::simd::copy_valid_tinyascii`].
#[inline]
#[target_feature(enable = "avx2")]
pub(super) unsafe fn copy_valid_tinyascii(src: &[u8], dst: &mut [MaybeUninit<u8>]) -> bool {
    #![allow(clippy::undocumented_unsafe_blocks)]

    // SAFETY: `src` length is less or equal than `N` as ensured
    // by the caller.
    unsafe { core::hint::assert_unchecked(MAX_LEN >= src.len()) };

    // SAFETY: `dst` length is greater or equal than `src` length
    // as ensured by the caller.
    unsafe { core::hint::assert_unchecked(dst.len() >= src.len()) };

    let len = src.len();
    let s = src.as_ptr();
    let d = dst.as_mut_ptr();

    if len > 64 {
        let a0 = unsafe { _mm256_loadu_si256(s.cast()) };
        let a1 = unsafe { _mm256_loadu_si256(s.add(32).cast()) };
        let m0 = _mm256_or_si256(invalid_mask_256(a0), invalid_mask_256(a1));
        unsafe { _mm256_storeu_si256(d.cast(), a0) };
        unsafe { _mm256_storeu_si256(d.add(32).cast(), a1) };

        let b1 = unsafe { _mm256_loadu_si256(s.add(len - 64).cast()) };
        let b0 = unsafe { _mm256_loadu_si256(s.add(len - 32).cast()) };
        let m1 = _mm256_or_si256(invalid_mask_256(b1), invalid_mask_256(b0));
        unsafe { _mm256_storeu_si256(d.add(len - 64).cast(), b1) };
        unsafe { _mm256_storeu_si256(d.add(len - 32).cast(), b0) };

        let m = _mm256_or_si256(m0, m1);
        return _mm256_testz_si256(m, m) != 0;
    }

    if len >= 32 {
        let a = unsafe { _mm256_loadu_si256(s.cast()) };
        let b = unsafe { _mm256_loadu_si256(s.add(len - 32).cast()) };
        unsafe { _mm256_storeu_si256(d.cast(), a) };
        unsafe { _mm256_storeu_si256(d.add(len - 32).cast(), b) };

        let m = _mm256_or_si256(invalid_mask_256(a), invalid_mask_256(b));
        return _mm256_testz_si256(m, m) != 0;
    }

    if len >= 16 {
        let a = unsafe { _mm_loadu_si128(s.cast()) };
        let b = unsafe { _mm_loadu_si128(s.add(len - 16).cast()) };
        unsafe { _mm_storeu_si128(d.cast(), a) };
        unsafe { _mm_storeu_si128(d.add(len - 16).cast(), b) };

        let m = _mm_or_si128(invalid_mask_128(a), invalid_mask_128(b));
        return _mm_testz_si128(m, m) != 0;
    }

    if len >= 8 {
        let a = unsafe { s.cast::<u64>().read_unaligned() };
        let b = unsafe { s.add(len - 8).cast::<u64>().read_unaligned() };
        unsafe { d.cast::<u64>().write_unaligned(a) };
        unsafe { d.add(len - 8).cast::<u64>().write_unaligned(b) };

        return !(has_invalid_u64(a) | has_invalid_u64(b));
    }

    if len >= 4 {
        let a = unsafe { s.cast::<u32>().read_unaligned() };
        let b = unsafe { s.add(len - 4).cast::<u32>().read_unaligned() };
        unsafe { d.cast::<u32>().write_unaligned(a) };
        unsafe { d.add(len - 4).cast::<u32>().write_unaligned(b) };

        let x = u64::from(a) | (u64::from(b) << 32);
        return !has_invalid_u64(x);
    }

    if len >= 2 {
        let a = unsafe { s.cast::<u16>().read_unaligned() };
        let b = unsafe { s.add(len - 2).cast::<u16>().read_unaligned() };
        unsafe { d.cast::<u16>().write_unaligned(a) };
        unsafe { d.add(len - 2).cast::<u16>().write_unaligned(b) };

        let x = u32::from(a) | (u32::from(b) << 16);
        return !has_invalid_u32(x);
    }

    if len == 1 {
        let b = unsafe { s.read() };
        unsafe { d.cast::<u8>().write(b) };
        return NonNullAscii::is_valid_u8(b);
    }

    true
}

/// AVX2 version of [`crate::simd::copy_tinyascii`].
///
/// # Safety
///
/// Same safety requirements as [`crate::simd::copy_tinyascii`].
#[inline]
#[target_feature(enable = "avx2")]
pub(super) unsafe fn copy_tinyascii(src: &[u8], dst: &mut [MaybeUninit<u8>]) {
    #![allow(clippy::undocumented_unsafe_blocks)]

    // SAFETY: `src` length is less or equal than `N` as ensured
    // by the caller.
    unsafe { core::hint::assert_unchecked(MAX_LEN >= src.len()) };

    // SAFETY: `dst` length is greater or equal than `src` length
    // as ensured by the caller.
    unsafe { core::hint::assert_unchecked(dst.len() >= src.len()) };

    let len = src.len();
    let s = src.as_ptr();
    let d = dst.as_mut_ptr();

    if len > 64 {
        let a0 = unsafe { _mm256_loadu_si256(s.cast()) };
        let a1 = unsafe { _mm256_loadu_si256(s.add(32).cast()) };
        let b1 = unsafe { _mm256_loadu_si256(s.add(len - 64).cast()) };
        let b0 = unsafe { _mm256_loadu_si256(s.add(len - 32).cast()) };
        unsafe { _mm256_storeu_si256(d.cast(), a0) };
        unsafe { _mm256_storeu_si256(d.add(32).cast(), a1) };
        unsafe { _mm256_storeu_si256(d.add(len - 64).cast(), b1) };
        unsafe { _mm256_storeu_si256(d.add(len - 32).cast(), b0) };

        return;
    }

    if len >= 32 {
        let a = unsafe { _mm256_loadu_si256(s.cast()) };
        let b = unsafe { _mm256_loadu_si256(s.add(len - 32).cast()) };
        unsafe { _mm256_storeu_si256(d.cast(), a) };
        unsafe { _mm256_storeu_si256(d.add(len - 32).cast(), b) };

        return;
    }

    if len >= 16 {
        let a = unsafe { _mm_loadu_si128(s.cast()) };
        let b = unsafe { _mm_loadu_si128(s.add(len - 16).cast()) };
        unsafe { _mm_storeu_si128(d.cast(), a) };
        unsafe { _mm_storeu_si128(d.add(len - 16).cast(), b) };

        return;
    }

    if len >= 8 {
        let a = unsafe { s.cast::<u64>().read_unaligned() };
        let b = unsafe { s.add(len - 8).cast::<u64>().read_unaligned() };
        unsafe { d.cast::<u64>().write_unaligned(a) };
        unsafe { d.add(len - 8).cast::<u64>().write_unaligned(b) };

        return;
    }

    if len >= 4 {
        let a = unsafe { s.cast::<u32>().read_unaligned() };
        let b = unsafe { s.add(len - 4).cast::<u32>().read_unaligned() };
        unsafe { d.cast::<u32>().write_unaligned(a) };
        unsafe { d.add(len - 4).cast::<u32>().write_unaligned(b) };

        return;
    }

    if len >= 2 {
        let a = unsafe { s.cast::<u16>().read_unaligned() };
        let b = unsafe { s.add(len - 2).cast::<u16>().read_unaligned() };
        unsafe { d.cast::<u16>().write_unaligned(a) };
        unsafe { d.add(len - 2).cast::<u16>().write_unaligned(b) };

        return;
    }

    if len == 1 {
        let b = unsafe { s.read() };
        unsafe { d.cast::<u8>().write(b) };
    }
}

/// Whether any byte of `x` is invalid (NUL or > 127).
#[inline(always)]
const fn has_invalid_u64(x: u64) -> bool {
    const LOW: u64 = 0x0101_0101_0101_0101;
    const HIGH: u64 = 0x8080_8080_8080_8080;
    // Classic "has zero byte" trick, plus any set top bit.
    ((x.wrapping_sub(LOW) & !x) | x) & HIGH != 0
}

/// 32-bit version of [`has_invalid_u64`].
#[inline(always)]
const fn has_invalid_u32(x: u32) -> bool {
    const LOW: u32 = 0x0101_0101;
    const HIGH: u32 = 0x8080_8080;
    ((x.wrapping_sub(LOW) & !x) | x) & HIGH != 0
}

/// AVX2 version of [`crate::simd::fill_ascii`].
///
/// # Safety
///
/// Same safety requirements as [`crate::simd::fill_ascii`].
#[inline]
#[target_feature(enable = "avx2")]
pub(super) unsafe fn fill_ascii(
    src: &[u8],
    dst: &mut [MaybeUninit<u8>],
    replacement: NonNullAscii,
) {
    #![allow(clippy::undocumented_unsafe_blocks)]

    // SAFETY: `src` length is less or equal than `N` as ensured
    // by the caller.
    unsafe { core::hint::assert_unchecked(MAX_LEN >= src.len()) };

    // SAFETY: `dst` length is greater or equal than `src` length
    // as ensured by the caller.
    unsafe { core::hint::assert_unchecked(dst.len() >= src.len()) };

    let len = src.len();
    let s = src.as_ptr();
    let d = dst.as_mut_ptr();
    let repl = replacement.to_u8();

    if len > 64 {
        let repl = _mm256_set1_epi8(repl as i8);

        let a0 = unsafe { _mm256_loadu_si256(s.cast()) };
        let a1 = unsafe { _mm256_loadu_si256(s.add(32).cast()) };
        let b1 = unsafe { _mm256_loadu_si256(s.add(len - 64).cast()) };
        let b0 = unsafe { _mm256_loadu_si256(s.add(len - 32).cast()) };
        let fa0 = _mm256_blendv_epi8(a0, repl, invalid_mask_256(a0));
        let fa1 = _mm256_blendv_epi8(a1, repl, invalid_mask_256(a1));
        let fb1 = _mm256_blendv_epi8(b1, repl, invalid_mask_256(b1));
        let fb0 = _mm256_blendv_epi8(b0, repl, invalid_mask_256(b0));
        unsafe { _mm256_storeu_si256(d.cast(), fa0) };
        unsafe { _mm256_storeu_si256(d.add(32).cast(), fa1) };
        unsafe { _mm256_storeu_si256(d.add(len - 64).cast(), fb1) };
        unsafe { _mm256_storeu_si256(d.add(len - 32).cast(), fb0) };

        return;
    }

    if len >= 32 {
        let repl = _mm256_set1_epi8(repl as i8);

        let a = unsafe { _mm256_loadu_si256(s.cast()) };
        let b = unsafe { _mm256_loadu_si256(s.add(len - 32).cast()) };
        let fa = _mm256_blendv_epi8(a, repl, invalid_mask_256(a));
        let fb = _mm256_blendv_epi8(b, repl, invalid_mask_256(b));
        unsafe { _mm256_storeu_si256(d.cast(), fa) };
        unsafe { _mm256_storeu_si256(d.add(len - 32).cast(), fb) };

        return;
    }

    if len >= 16 {
        let repl = _mm_set1_epi8(repl as i8);

        let a = unsafe { _mm_loadu_si128(s.cast()) };
        let b = unsafe { _mm_loadu_si128(s.add(len - 16).cast()) };
        let fa = _mm_blendv_epi8(a, repl, invalid_mask_128(a));
        let fb = _mm_blendv_epi8(b, repl, invalid_mask_128(b));
        unsafe { _mm_storeu_si128(d.cast(), fa) };
        unsafe { _mm_storeu_si128(d.add(len - 16).cast(), fb) };

        return;
    }

    if len >= 8 {
        let repl = _mm_set1_epi8(repl as i8);

        let a = unsafe { _mm_loadl_epi64(s.cast()) };
        let b = unsafe { _mm_loadl_epi64(s.add(len - 8).cast()) };
        let fa = _mm_blendv_epi8(a, repl, invalid_mask_128(a));
        let fb = _mm_blendv_epi8(b, repl, invalid_mask_128(b));
        unsafe { _mm_storel_epi64(d.cast(), fa) };
        unsafe { _mm_storel_epi64(d.add(len - 8).cast(), fb) };

        return;
    }

    if len >= 4 {
        let a = unsafe { s.cast::<u32>().read_unaligned() };
        let b = unsafe { s.add(len - 4).cast::<u32>().read_unaligned() };
        let f = fill_u64(u64::from(a) | (u64::from(b) << 32), repl);
        unsafe { d.cast::<u32>().write_unaligned(f as u32) };
        unsafe {
            d.add(len - 4)
                .cast::<u32>()
                .write_unaligned((f >> 32) as u32);
        }
        return;
    }

    if len >= 2 {
        let a = unsafe { s.cast::<u16>().read_unaligned() };
        let b = unsafe { s.add(len - 2).cast::<u16>().read_unaligned() };
        let f = fill_u32(u32::from(a) | (u32::from(b) << 16), repl);
        unsafe { d.cast::<u16>().write_unaligned(f as u16) };
        unsafe {
            d.add(len - 2)
                .cast::<u16>()
                .write_unaligned((f >> 16) as u16);
        }
        return;
    }

    if len == 1 {
        let b = unsafe { s.read() };
        let valid = if NonNullAscii::is_valid_u8(b) {
            b
        } else {
            repl
        };
        unsafe {
            d.cast::<u8>().write(valid);
        }
    }
}

/// Replaces every invalid byte (NUL or > 127) of `x` with `repl`.
#[inline(always)]
const fn fill_u64(x: u64, repl: u8) -> u64 {
    const LOW: u64 = 0x0101_0101_0101_0101;
    const LOW7: u64 = 0x7f7f_7f7f_7f7f_7f7f;
    const HIGH: u64 = 0x8080_8080_8080_8080;
    // High bit of each byte is set iff its low 7 bits are non-zero
    // (the add cannot carry across bytes), and cleared if its top bit is set.
    let valid = ((x & LOW7).wrapping_add(LOW7)) & !x & HIGH;
    // 0x01 per valid byte, times 0xff gives a full-byte mask without carries.
    let mask = (valid >> 7) * 0xff;
    (x & mask) | ((repl as u64).wrapping_mul(LOW) & !mask)
}

/// 32-bit version of [`fill_u64`].
#[inline(always)]
const fn fill_u32(x: u32, repl: u8) -> u32 {
    const LOW: u32 = 0x0101_0101;
    const LOW7: u32 = 0x7f7f_7f7f;
    const HIGH: u32 = 0x8080_8080;
    let valid = ((x & LOW7).wrapping_add(LOW7)) & !x & HIGH;
    let mask = (valid >> 7) * 0xff;
    (x & mask) | ((repl as u32).wrapping_mul(LOW) & !mask)
}

/// AVX2 version of [`crate::simd::fill_value`].
///
/// # Safety
///
/// Same safety requirements as [`crate::simd::fill_value`].
#[inline]
#[target_feature(enable = "avx2")]
pub(super) unsafe fn fill_value(dst: &mut [MaybeUninit<u8>], value: NonNullAscii) {
    #![allow(clippy::undocumented_unsafe_blocks)]

    // SAFETY: `dst` length is less or equal than `MAX_LEN` as ensured
    // by the caller.
    unsafe { core::hint::assert_unchecked(MAX_LEN >= dst.len()) };

    let len = dst.len();
    let d = dst.as_mut_ptr();
    let val = value.to_u8();

    if len > 64 {
        let v = _mm256_set1_epi8(val as i8);
        unsafe { _mm256_storeu_si256(d.cast(), v) };
        unsafe { _mm256_storeu_si256(d.add(32).cast(), v) };
        unsafe { _mm256_storeu_si256(d.add(len - 64).cast(), v) };
        unsafe { _mm256_storeu_si256(d.add(len - 32).cast(), v) };

        return;
    }

    if len >= 32 {
        let v = _mm256_set1_epi8(val as i8);
        unsafe { _mm256_storeu_si256(d.cast(), v) };
        unsafe { _mm256_storeu_si256(d.add(len - 32).cast(), v) };

        return;
    }

    if len >= 16 {
        let v = _mm_set1_epi8(val as i8);
        unsafe { _mm_storeu_si128(d.cast(), v) };
        unsafe { _mm_storeu_si128(d.add(len - 16).cast(), v) };

        return;
    }

    if len >= 8 {
        let v = u64::from(val) * 0x0101_0101_0101_0101;
        unsafe { d.cast::<u64>().write_unaligned(v) };
        unsafe { d.add(len - 8).cast::<u64>().write_unaligned(v) };

        return;
    }

    if len >= 4 {
        let v = u32::from(val) * 0x0101_0101;
        unsafe { d.cast::<u32>().write_unaligned(v) };
        unsafe { d.add(len - 4).cast::<u32>().write_unaligned(v) };

        return;
    }

    if len >= 2 {
        let v = u16::from(val) * 0x0101;
        unsafe { d.cast::<u16>().write_unaligned(v) };
        unsafe { d.add(len - 2).cast::<u16>().write_unaligned(v) };

        return;
    }

    if len == 1 {
        unsafe { d.cast::<u8>().write(val) };
    }
}
