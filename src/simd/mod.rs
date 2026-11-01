mod avx2;
mod scalar;

use crate::ascii::NonNullAscii;

/// Check if `src` is valid [`TinyAscii`] content.
///
/// # Safety
///
/// `src.len()` must be less than or equal 128 (`MAX_LEN`).
///
/// Failing to uphold those invariants will cause a write out of bounds.
#[inline]
pub unsafe fn is_valid_tinyascii(src: &[u8]) -> bool {
    cfg_select! {
        target_feature = "avx2" => {
            // SAFETY:
            // 1. AVX2 version has same safety requirements as `self::is_valid_tinyascii`
            // 2. `is_valid_tinyascii` requires target_feature avx2 enabled
            unsafe { avx2::is_valid_tinyascii(src) }
        }
        _ => {
            // SAFETY: scalar version has same safety requirements as `self::is_valid_tinyascii`
            unsafe { scalar::is_valid_tinyascii(src) }
        }
    }
}

/// Fill destination (`dst`) with valid bytes from source (`src`) using
/// a replacement character for invalid bytes in source.
///
/// # Safety
///
/// - `src.len()` must be less than or equal 128 (`MAX_LEN`).
/// - `dst` length must be greater or equal to `src` length (`dst.len() >= src.len()`).
///
/// Failing to uphold those invariants will cause a write out of bounds.
#[inline]
pub unsafe fn fill_ascii(dst: &mut [u8], src: &[u8], replacement: NonNullAscii) {
    cfg_select! {
        target_feature = "avx2" => {
            // SAFETY:
            // 1. AVX2 version has same safety requirements as `self::fill_ascii`
            // 2. `fill_ascii` requires target_feature avx2 enabled
            unsafe { avx2::fill_ascii(dst, src, replacement) }
        }
        _ => {
            // SAFETY: scalar version has same safety requirements as `self::fill_ascii`
            unsafe { scalar::fill_ascii(dst, src, replacement) }
        }
    }
}

mod shared {
    /// Whether a single byte is valid `TinyAscii` content.
    #[inline(always)]
    pub(super) const fn is_valid_byte(b: u8) -> bool {
        b.wrapping_sub(1) < 127
    }
}
