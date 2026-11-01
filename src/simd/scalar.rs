#![allow(unused)]

use crate::ascii::NonNullAscii;
use crate::simd::shared;
use crate::tinyascii::MAX_LEN;

/// Scalar version of [`crate::simd::is_valid_tinyascii`].
///
/// # Safety
///
/// Same sefety requirements as [`crate::simd::is_valid_tinyascii`].
#[inline(always)]
pub(super) unsafe fn is_valid_tinyascii(src: &[u8]) -> bool {
    // SAFETY: `src` length is less or equal than `N` as ensured
    // by the caller.s
    unsafe { core::hint::assert_unchecked(MAX_LEN >= src.len()) };

    let mut acc = 1u8;
    for &b in src {
        acc &= shared::is_valid_byte(b) as u8;
    }

    acc != 0
}

/// Scalar version of [`crate::simd::fill_ascii`].
///
/// # Safety
///
/// Same sefety requirements as [`crate::simd::fill_ascii`].
#[inline(always)]
pub(super) unsafe fn fill_ascii(dst: &mut [u8], src: &[u8], replacement: NonNullAscii) {
    // SAFETY: `src` length is less or equal than `N` as ensured
    // by the caller.
    unsafe { core::hint::assert_unchecked(MAX_LEN >= src.len()) };

    // SAFETY: `dst` length is greater or equal than `src` length
    // as ensured by the caller.
    unsafe { core::hint::assert_unchecked(dst.len() >= src.len()) };

    for (idx, byte) in src.iter().copied().enumerate() {
        let val = if shared::is_valid_byte(byte) {
            byte
        } else {
            replacement.to_u8()
        };

        // SAFETY: `idx` is withing bounds of `dst` slice
        *unsafe { dst.get_unchecked_mut(idx) } = val;
    }
}
