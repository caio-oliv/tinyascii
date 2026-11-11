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

/// AVX2 version of [`crate::simd::is_valid_tinyascii`].
///
/// # Safety
///
/// Same sefety requirements as [`crate::simd::is_valid_tinyascii`].
#[inline]
#[target_feature(enable = "avx2")]
pub(super) unsafe fn is_valid_tinyascii(src: &[u8]) -> bool {
    #![allow(clippy::undocumented_unsafe_blocks)]

    // SAFETY: `src` length is less or equal than `N` as ensured
    // by the caller.
    unsafe { core::hint::assert_unchecked(MAX_LEN >= src.len()) };

    let len = src.len();
    let ptr = src.as_ptr();

    // `len <= MAX_LEN == 128`, so no loop is needed: overlapping vectors
    // cover every length.
    if len > 64 {
        // `64 < len <= 128`: `[0, 64)` and `[len - 64, len)` cover `[0, len)`.
        let a0 = unsafe { _mm256_loadu_si256(ptr.cast()) };
        let a1 = unsafe { _mm256_loadu_si256(ptr.add(32).cast()) };
        let b1 = unsafe { _mm256_loadu_si256(ptr.add(len - 64).cast()) };
        let b0 = unsafe { _mm256_loadu_si256(ptr.add(len - 32).cast()) };
        let m0 = _mm256_or_si256(invalid_mask_256(a0), invalid_mask_256(a1));
        let m1 = _mm256_or_si256(invalid_mask_256(b1), invalid_mask_256(b0));
        let m = _mm256_or_si256(m0, m1);
        return _mm256_testz_si256(m, m) != 0;
    }

    if len >= 32 {
        // `32 <= len <= 64`: `[0, 32)` and `[len - 32, len)` cover `[0, len)`.
        let a = unsafe { _mm256_loadu_si256(ptr.cast()) };
        let b = unsafe { _mm256_loadu_si256(ptr.add(len - 32).cast()) };
        let m = _mm256_or_si256(invalid_mask_256(a), invalid_mask_256(b));
        return _mm256_testz_si256(m, m) != 0;
    }

    if len >= 16 {
        let a = unsafe { _mm_loadu_si128(ptr.cast()) };
        let b = unsafe { _mm_loadu_si128(ptr.add(len - 16).cast()) };
        let m = _mm_or_si128(invalid_mask_128(a), invalid_mask_128(b));
        return _mm_testz_si128(m, m) != 0;
    }

    if len >= 8 {
        let a = unsafe { _mm_loadl_epi64(ptr.cast()) };
        let b = unsafe { _mm_loadl_epi64(ptr.add(len - 8).cast()) };
        let m = invalid_mask_128(_mm_unpacklo_epi64(a, b));
        return _mm_testz_si128(m, m) != 0;
    }

    // Fewer than 8 bytes: overlapping integer windows, no loop.
    if len >= 4 {
        // `4 <= len < 8`: `[0, 4)` and `[len - 4, len)` cover `[0, len)`.
        let a = unsafe { ptr.cast::<u32>().read_unaligned() };
        let b = unsafe { ptr.add(len - 4).cast::<u32>().read_unaligned() };
        return !has_invalid_u64(u64::from(a) | (u64::from(b) << 32));
    }

    if len >= 2 {
        // `2 <= len < 4`: `[0, 2)` and `[len - 2, len)` cover `[0, len)`.
        let a = unsafe { ptr.cast::<u16>().read_unaligned() };
        let b = unsafe { ptr.add(len - 2).cast::<u16>().read_unaligned() };
        return !has_invalid_u32(u32::from(a) | (u32::from(b) << 16));
    }

    if len == 1 {
        return NonNullAscii::is_valid_u8(unsafe { *ptr });
    }

    true
}

/// AVX2 version of [`crate::simd::copy_valid_tinyascii`].
///
/// All of `src` is copied to `dst` regardless of validity. Inputs are
/// processed with a few (possibly overlapping) vector accesses chosen by
/// length class; overlapping stores write identical bytes.
///
/// # Safety
///
/// Same sefety requirements as [`crate::simd::copy_valid_tinyascii`].
#[inline]
#[target_feature(enable = "avx2")]
pub(super) unsafe fn copy_valid_tinyascii(src: &[u8], dst: &mut [u8]) -> bool {
    // SAFETY: `src` length is less or equal than `N` as ensured
    // by the caller.
    unsafe { core::hint::assert_unchecked(MAX_LEN >= src.len()) };

    // SAFETY: `dst` length is greater or equal than `src` length
    // as ensured by the caller.
    unsafe { core::hint::assert_unchecked(dst.len() >= src.len()) };

    let len = src.len();
    let s_ptr = src.as_ptr();
    let d_ptr = dst.as_mut_ptr();

    // `len <= MAX_LEN == 128`, so no loop is needed: straight-line code of
    // overlapping vectors covers every length. A loop that stores the loaded
    // vector is turned by LLVM into a `memcpy` call (loop idiom recognition).
    if len > 64 {
        // SAFETY: `64 < len <= 128`, so `[0, 64)` and `[len - 64, len)` are
        // in bounds and together cover `[0, len)`.
        let m = unsafe {
            let a0 = _mm256_loadu_si256(s_ptr.cast());
            let a1 = _mm256_loadu_si256(s_ptr.add(32).cast());
            let b1 = _mm256_loadu_si256(s_ptr.add(len - 64).cast());
            let b0 = _mm256_loadu_si256(s_ptr.add(len - 32).cast());
            _mm256_storeu_si256(d_ptr.cast(), a0);
            _mm256_storeu_si256(d_ptr.add(32).cast(), a1);
            _mm256_storeu_si256(d_ptr.add(len - 64).cast(), b1);
            _mm256_storeu_si256(d_ptr.add(len - 32).cast(), b0);
            let m0 = _mm256_or_si256(invalid_mask_256(a0), invalid_mask_256(a1));
            let m1 = _mm256_or_si256(invalid_mask_256(b1), invalid_mask_256(b0));
            _mm256_or_si256(m0, m1)
        };
        return _mm256_testz_si256(m, m) != 0;
    }

    if len >= 32 {
        // SAFETY: `32 <= len <= 64`, so both 32-byte windows are in bounds
        // and together cover `[0, len)`. Both loads precede the stores.
        let m = unsafe {
            let a = _mm256_loadu_si256(s_ptr.cast());
            let b = _mm256_loadu_si256(s_ptr.add(len - 32).cast());
            _mm256_storeu_si256(d_ptr.cast(), a);
            _mm256_storeu_si256(d_ptr.add(len - 32).cast(), b);
            _mm256_or_si256(invalid_mask_256(a), invalid_mask_256(b))
        };
        return _mm256_testz_si256(m, m) != 0;
    }

    if len >= 16 {
        // SAFETY: `len >= 16`, so both 16-byte windows are in bounds of
        // `src` and `dst`. Both loads precede the stores.
        let m = unsafe {
            let a = _mm_loadu_si128(s_ptr.cast());
            let b = _mm_loadu_si128(s_ptr.add(len - 16).cast());
            _mm_storeu_si128(d_ptr.cast(), a);
            _mm_storeu_si128(d_ptr.add(len - 16).cast(), b);
            _mm_or_si128(invalid_mask_128(a), invalid_mask_128(b))
        };
        return _mm_testz_si128(m, m) != 0;
    }

    if len >= 8 {
        // SAFETY: `len >= 8`, so both 8-byte windows are in bounds of
        // `src` and `dst`. Both loads precede the stores.
        let m = unsafe {
            let a = _mm_loadl_epi64(s_ptr.cast());
            let b = _mm_loadl_epi64(s_ptr.add(len - 8).cast());
            _mm_storel_epi64(d_ptr.cast(), a);
            _mm_storel_epi64(d_ptr.add(len - 8).cast(), b);
            invalid_mask_128(_mm_unpacklo_epi64(a, b))
        };
        return _mm_testz_si128(m, m) != 0;
    }

    // Fewer than 8 bytes. A scalar copy loop would be turned into a `memcpy`
    // call by LLVM, so use overlapping integer accesses instead.
    if len >= 4 {
        // SAFETY: `4 <= len < 8`, so both 4-byte windows are in bounds of
        // `src` and `dst` and together cover `[0, len)`.
        let x = unsafe {
            let a = s_ptr.cast::<u32>().read_unaligned();
            let b = s_ptr.add(len - 4).cast::<u32>().read_unaligned();
            d_ptr.cast::<u32>().write_unaligned(a);
            d_ptr.add(len - 4).cast::<u32>().write_unaligned(b);
            u64::from(a) | (u64::from(b) << 32)
        };
        return !has_invalid_u64(x);
    }

    if len >= 2 {
        // SAFETY: `2 <= len < 4`, so both 2-byte windows are in bounds of
        // `src` and `dst` and together cover `[0, len)`.
        let x = unsafe {
            let a = s_ptr.cast::<u16>().read_unaligned();
            let b = s_ptr.add(len - 2).cast::<u16>().read_unaligned();
            d_ptr.cast::<u16>().write_unaligned(a);
            d_ptr.add(len - 2).cast::<u16>().write_unaligned(b);
            u32::from(a) | (u32::from(b) << 16)
        };
        return !has_invalid_u32(x);
    }

    if len == 1 {
        // SAFETY: `len == 1`, so index 0 is in bounds of `src` and `dst`.
        let b = unsafe {
            let b = *s_ptr;
            *d_ptr = b;
            b
        };
        return NonNullAscii::is_valid_u8(b);
    }

    true
}

/// Whether any byte of `x` is invalid (NUL or > 127).
#[inline(always)]
fn has_invalid_u64(x: u64) -> bool {
    const LOW: u64 = 0x0101_0101_0101_0101;
    const HIGH: u64 = 0x8080_8080_8080_8080;
    // Classic "has zero byte" trick, plus any set top bit.
    ((x.wrapping_sub(LOW) & !x) | x) & HIGH != 0
}

/// 32-bit version of [`has_invalid_u64`].
#[inline(always)]
fn has_invalid_u32(x: u32) -> bool {
    const LOW: u32 = 0x0101_0101;
    const HIGH: u32 = 0x8080_8080;
    ((x.wrapping_sub(LOW) & !x) | x) & HIGH != 0
}

/// AVX2 version of [`crate::simd::fill_ascii`].
///
/// Inputs are processed with a few (possibly overlapping) vector accesses
/// chosen by length class. Overlapping stores are safe because each output
/// byte only depends on the same source byte.
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
    let s_ptr = src.as_ptr();
    let d_ptr = dst.as_mut_ptr();
    let repl = replacement.to_u8();

    if len > 64 {
        let repl = _mm256_set1_epi8(repl as i8);

        // SAFETY: `64 < len <= 128`, so `[0, 64)` and `[len - 64, len)` are
        // in bounds of `src` and `dst` and together cover `[0, len)`.
        unsafe {
            let a0 = _mm256_loadu_si256(s_ptr.cast());
            let a1 = _mm256_loadu_si256(s_ptr.add(32).cast());
            let b1 = _mm256_loadu_si256(s_ptr.add(len - 64).cast());
            let b0 = _mm256_loadu_si256(s_ptr.add(len - 32).cast());
            let fa0 = _mm256_blendv_epi8(a0, repl, invalid_mask_256(a0));
            let fa1 = _mm256_blendv_epi8(a1, repl, invalid_mask_256(a1));
            let fb1 = _mm256_blendv_epi8(b1, repl, invalid_mask_256(b1));
            let fb0 = _mm256_blendv_epi8(b0, repl, invalid_mask_256(b0));
            _mm256_storeu_si256(d_ptr.cast(), fa0);
            _mm256_storeu_si256(d_ptr.add(32).cast(), fa1);
            _mm256_storeu_si256(d_ptr.add(len - 64).cast(), fb1);
            _mm256_storeu_si256(d_ptr.add(len - 32).cast(), fb0);
        }
        return;
    }

    if len >= 32 {
        let repl = _mm256_set1_epi8(repl as i8);

        // SAFETY: `32 <= len <= 64`, so both 32-byte windows are in bounds
        // and together cover `[0, len)`. Both loads precede the stores.
        unsafe {
            let a = _mm256_loadu_si256(s_ptr.cast());
            let b = _mm256_loadu_si256(s_ptr.add(len - 32).cast());
            let fa = _mm256_blendv_epi8(a, repl, invalid_mask_256(a));
            let fb = _mm256_blendv_epi8(b, repl, invalid_mask_256(b));
            _mm256_storeu_si256(d_ptr.cast(), fa);
            _mm256_storeu_si256(d_ptr.add(len - 32).cast(), fb);
        }
        return;
    }

    if len >= 16 {
        let repl = _mm_set1_epi8(repl as i8);

        // SAFETY: `len >= 16`, so both 16-byte windows are in bounds of
        // `src` and `dst`. Both loads precede the stores.
        unsafe {
            let a = _mm_loadu_si128(s_ptr.cast());
            let b = _mm_loadu_si128(s_ptr.add(len - 16).cast());
            let fa = _mm_blendv_epi8(a, repl, invalid_mask_128(a));
            let fb = _mm_blendv_epi8(b, repl, invalid_mask_128(b));
            _mm_storeu_si128(d_ptr.cast(), fa);
            _mm_storeu_si128(d_ptr.add(len - 16).cast(), fb);
        }
        return;
    }

    if len >= 8 {
        let repl = _mm_set1_epi8(repl as i8);

        // SAFETY: `len >= 8`, so both 8-byte windows are in bounds of
        // `src` and `dst`.
        unsafe {
            let a = _mm_loadl_epi64(s_ptr.cast());
            let b = _mm_loadl_epi64(s_ptr.add(len - 8).cast());
            let fa = _mm_blendv_epi8(a, repl, invalid_mask_128(a));
            let fb = _mm_blendv_epi8(b, repl, invalid_mask_128(b));
            _mm_storel_epi64(d_ptr.cast(), fa);
            _mm_storel_epi64(d_ptr.add(len - 8).cast(), fb);
        }
        return;
    }

    // Fewer than 8 bytes: overlapping integer windows, no loop.
    if len >= 4 {
        // SAFETY: `4 <= len < 8`, so both 4-byte windows are in bounds of
        // `src` and `dst` and together cover `[0, len)`.
        unsafe {
            let a = s_ptr.cast::<u32>().read_unaligned();
            let b = s_ptr.add(len - 4).cast::<u32>().read_unaligned();
            let f = fill_u64(u64::from(a) | (u64::from(b) << 32), repl);
            // Truncation keeps the low / high 32 bits respectively.
            d_ptr.cast::<u32>().write_unaligned(f as u32);
            d_ptr
                .add(len - 4)
                .cast::<u32>()
                .write_unaligned((f >> 32) as u32);
        }
        return;
    }

    if len >= 2 {
        // SAFETY: `2 <= len < 4`, so both 2-byte windows are in bounds of
        // `src` and `dst` and together cover `[0, len)`.
        unsafe {
            let a = s_ptr.cast::<u16>().read_unaligned();
            let b = s_ptr.add(len - 2).cast::<u16>().read_unaligned();
            let f = fill_u32(u32::from(a) | (u32::from(b) << 16), repl);
            d_ptr.cast::<u16>().write_unaligned(f as u16);
            d_ptr
                .add(len - 2)
                .cast::<u16>()
                .write_unaligned((f >> 16) as u16);
        }
        return;
    }

    if len == 1 {
        // SAFETY: `len == 1`, so index 0 is in bounds of `src` and `dst`.
        unsafe {
            let b = *s_ptr;
            *d_ptr = if NonNullAscii::is_valid_u8(b) {
                b
            } else {
                repl
            };
        }
    }
}

/// Replaces every invalid byte (NUL or > 127) of `x` with `repl`.
#[inline(always)]
fn fill_u64(x: u64, repl: u8) -> u64 {
    const LOW: u64 = 0x0101_0101_0101_0101;
    const LOW7: u64 = 0x7f7f_7f7f_7f7f_7f7f;
    const HIGH: u64 = 0x8080_8080_8080_8080;
    // High bit of each byte is set iff its low 7 bits are non-zero
    // (the add cannot carry across bytes), and cleared if its top bit is set.
    let valid = ((x & LOW7).wrapping_add(LOW7)) & !x & HIGH;
    // 0x01 per valid byte, times 0xff gives a full-byte mask without carries.
    let mask = (valid >> 7) * 0xff;
    (x & mask) | (u64::from(repl).wrapping_mul(LOW) & !mask)
}

/// 32-bit version of [`fill_u64`].
#[inline(always)]
fn fill_u32(x: u32, repl: u8) -> u32 {
    const LOW: u32 = 0x0101_0101;
    const LOW7: u32 = 0x7f7f_7f7f;
    const HIGH: u32 = 0x8080_8080;
    let valid = ((x & LOW7).wrapping_add(LOW7)) & !x & HIGH;
    let mask = (valid >> 7) * 0xff;
    (x & mask) | (u32::from(repl).wrapping_mul(LOW) & !mask)
}
