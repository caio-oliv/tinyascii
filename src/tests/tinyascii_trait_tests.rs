#![allow(clippy::unwrap_used)]
#![allow(clippy::undocumented_unsafe_blocks)]

use crate::{
    NonNullAscii, TinyAscii32, TinyAscii64,
    tinyascii::{TinyAscii4, TinyAscii8, TinyAscii16, TinyAscii128},
};

// ---------------------------------------------------------------------------
// Deref — TinyAscii → &str
// ---------------------------------------------------------------------------

#[test]
fn deref_to_str() {
    let s: TinyAscii16 = TinyAscii16::try_new("hello").unwrap();

    // Deref gives us a `&str`, so string methods work
    assert_eq!(s.len(), 5);
    assert!(s.starts_with("hel"));
    assert!(s.ends_with("lo"));
    assert!(s.contains("ell"));
}

#[test]
fn deref_empty() {
    let s: TinyAscii4 = TinyAscii4::empty();

    assert_eq!(s.len(), 0);
    assert!(s.is_empty());
    assert_eq!(s.to_uppercase(), "");
}

// ---------------------------------------------------------------------------
// AsRef<str>
// ---------------------------------------------------------------------------

#[test]
fn as_ref_str() {
    let s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    assert_eq!(AsRef::<str>::as_ref(&s), "abc");
}

// ---------------------------------------------------------------------------
// AsRef<[NonNullAscii]>
// ---------------------------------------------------------------------------

#[test]
fn as_ref_nonnull_ascii() {
    let s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    let ascii: &[NonNullAscii] = AsRef::<[NonNullAscii]>::as_ref(&s);
    assert_eq!(ascii.len(), 3);
    assert_eq!(ascii[0], NonNullAscii::SmallA);
    assert_eq!(ascii[2], NonNullAscii::SmallC);
}

#[test]
fn as_ref_nonnull_ascii_empty() {
    let s: TinyAscii4 = TinyAscii4::empty();
    let ascii: &[NonNullAscii] = AsRef::<[NonNullAscii]>::as_ref(&s);
    assert!(ascii.is_empty());
}

// ---------------------------------------------------------------------------
// AsRef<[u8]>
// ---------------------------------------------------------------------------

#[test]
fn as_ref_u8() {
    let s: TinyAscii8 = TinyAscii8::try_new("AB").unwrap();
    assert_eq!(AsRef::<[u8]>::as_ref(&s), b"AB");
}

// ---------------------------------------------------------------------------
// AsMut<[NonNullAscii]>
// ---------------------------------------------------------------------------

#[test]
fn as_mut_nonnull_ascii() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    let ascii: &mut [NonNullAscii] = AsMut::<[NonNullAscii]>::as_mut(&mut s);
    ascii[1] = NonNullAscii::Digit9;
    assert_eq!(&s, "a9c");
}

#[test]
fn as_mut_empty() {
    let mut s: TinyAscii4 = TinyAscii4::empty();
    let ascii: &mut [NonNullAscii] = AsMut::<[NonNullAscii]>::as_mut(&mut s);
    assert!(ascii.is_empty());
}

// ---------------------------------------------------------------------------
// Borrow<[NonNullAscii]> / Borrow<str>
// ---------------------------------------------------------------------------

#[test]
fn borrow_nonnull_ascii() {
    use core::borrow::Borrow;

    let s: TinyAscii8 = TinyAscii8::try_new("x").unwrap();
    let borrowed: &[NonNullAscii] = s.borrow();
    assert_eq!(borrowed, &[NonNullAscii::SmallX]);
}

#[test]
fn borrow_str() {
    use core::borrow::Borrow;

    let s: TinyAscii8 = TinyAscii8::try_new("x").unwrap();
    let borrowed: &str = s.borrow();
    assert_eq!(borrowed, "x");
}

// ---------------------------------------------------------------------------
// BorrowMut<[NonNullAscii]>
// ---------------------------------------------------------------------------

#[test]
fn borrow_mut_nonnull_ascii() {
    use core::borrow::BorrowMut;

    let mut s: TinyAscii8 = TinyAscii8::try_new("12").unwrap();
    let borrowed: &mut [NonNullAscii] = BorrowMut::borrow_mut(&mut s);
    borrowed[0] = NonNullAscii::Digit9;
    assert_eq!(&s, "92");
}

// ---------------------------------------------------------------------------
// Default
// ---------------------------------------------------------------------------

#[test]
fn default_is_empty() {
    let s: TinyAscii8 = Default::default();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
    assert_eq!(&s, "");
}

#[test]
fn default_capacity() {
    // Default should work for any capacity
    let s1: TinyAscii4 = Default::default();
    let s2: TinyAscii8 = Default::default();
    assert_eq!(s1.capacity(), 4);
    assert_eq!(s2.capacity(), 8);
}

// ---------------------------------------------------------------------------
// Display
// ---------------------------------------------------------------------------

#[test]
fn display_simple() {
    let s: TinyAscii16 = TinyAscii16::try_new("hello").unwrap();
    assert_eq!(format!("{}", s), "hello");
}

#[test]
fn display_empty() {
    let s: TinyAscii4 = TinyAscii4::empty();
    assert_eq!(format!("{}", s), "");
}

#[test]
fn display_full_capacity() {
    let s: TinyAscii4 = TinyAscii4::try_new("abcd").unwrap();
    assert_eq!(format!("{}", s), "abcd");
}

#[test]
fn display_control_chars() {
    let bytes: [u8; 8] = [b'a', b'b', 0x07, b'c', 0, 0, 0, 0];
    let s = unsafe { TinyAscii8::from_parts(bytes, 4) };
    assert_eq!(format!("{}", s), "ab\x07c");
}

#[test]
fn display_delete_char() {
    let bytes: [u8; 8] = [b'x', 0x7F, b'y', 0, 0, 0, 0, 0];
    let s = unsafe { TinyAscii8::from_parts(bytes, 3) };
    assert_eq!(format!("{}", s), "x\x7Fy");
}

// ---------------------------------------------------------------------------
// Debug
// ---------------------------------------------------------------------------

#[test]
fn debug_simple() {
    let s: TinyAscii16 = TinyAscii16::try_new("hello").unwrap();
    assert_eq!(format!("{:?}", s), "hello");
}

#[test]
fn debug_empty() {
    let s: TinyAscii4 = TinyAscii4::empty();
    assert_eq!(format!("{:?}", s), "");
}

#[test]
fn debug_display_same() {
    // Debug and Display should produce identical output
    let s: TinyAscii16 = TinyAscii16::try_new("test").unwrap();
    assert_eq!(format!("{:?}", s), format!("{}", s));
}

// ---------------------------------------------------------------------------
// Eq / PartialEq (TinyAscii vs TinyAscii) — same capacity
// ---------------------------------------------------------------------------

#[test]
fn eq_same_capacity() {
    let a: TinyAscii8 = TinyAscii8::try_new("foo").unwrap();
    let b: TinyAscii8 = TinyAscii8::try_new("foo").unwrap();
    assert_eq!(a, b);
}

#[test]
fn eq_different_content() {
    let a: TinyAscii8 = TinyAscii8::try_new("foo").unwrap();
    let b: TinyAscii8 = TinyAscii8::try_new("bar").unwrap();
    assert_ne!(a, b);
}

#[test]
fn eq_empty_same_capacity() {
    let a: TinyAscii8 = TinyAscii8::empty();
    let b: TinyAscii8 = TinyAscii8::empty();
    assert_eq!(a, b);
}

#[test]
fn eq_full_capacity() {
    let a: TinyAscii4 = TinyAscii4::try_new("abcd").unwrap();
    let b: TinyAscii4 = TinyAscii4::try_new("abcd").unwrap();
    assert_eq!(a, b);
}

#[test]
fn eq_zero_len_different_trailing() {
    // Two TinyAscii values with same len but different trailing buffer
    let a: TinyAscii8 = TinyAscii8::try_new("ab").unwrap();
    let b: TinyAscii8 = TinyAscii8::try_new("ab").unwrap();
    // They should be equal because trailing bytes are irrelevant
    assert_eq!(a, b);
}

#[test]
fn eq_case_sensitive() {
    let a: TinyAscii8 = TinyAscii8::try_new("Abc").unwrap();
    let b: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    assert_ne!(a, b);
}

#[test]
fn eq_all_printable_ascii_range() {
    // All 95 printable ASCII characters
    let printable: TinyAscii128 =
        TinyAscii128::try_new(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~")
            .unwrap();
    assert_eq!(
        &*printable,
        " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~"
    );
}

// ---------------------------------------------------------------------------
// PartialEq<str> / PartialEq<TinyAscii> for str
// ---------------------------------------------------------------------------

#[test]
fn eq_tinyascii_str() {
    let s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    assert_eq!(&s, "abc");
}

#[test]
fn eq_str_tinyascii() {
    let s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    assert_eq!("abc", &s);
}

#[test]
fn eq_empty_str() {
    let s: TinyAscii4 = TinyAscii4::empty();
    assert_eq!(&s, "");
    assert_eq!("", &s);
}

#[test]
fn eq_empty_tinyascii_empty_str() {
    let s: TinyAscii8 = TinyAscii8::empty();
    assert_eq!(&s, "");
    assert_eq!("", &s);
}

#[test]
fn eq_different_content_str() {
    let s: TinyAscii8 = TinyAscii8::try_new("foo").unwrap();
    assert_ne!(&s, "bar");
    assert_ne!("bar", &s);
}

#[test]
fn eq_different_length() {
    let s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    assert_ne!(&s, "abcd");
    assert_ne!("abcde", &s);
}

#[test]
fn eq_case_sensitive_str() {
    let a: TinyAscii8 = TinyAscii8::try_new("Abc").unwrap();
    assert_ne!(&a, "abc");
    assert_ne!("Abc", "abc");
}

#[test]
fn eq_whitespace() {
    let s: TinyAscii8 = TinyAscii8::try_new("  ").unwrap();
    assert_eq!(&s, "  ");
}

// ---------------------------------------------------------------------------
// PartialEq<[NonNullAscii]> / PartialEq<TinyAscii> for [NonNullAscii] slice
// ---------------------------------------------------------------------------

#[test]
fn eq_tinyascii_nonnull_ascii_slice() {
    let s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    let arr = [
        NonNullAscii::SmallA,
        NonNullAscii::SmallB,
        NonNullAscii::SmallC,
    ];
    assert_eq!(s.as_ascii(), &arr[..]);
}

#[test]
fn eq_nonnull_ascii_slice_tinyascii() {
    let s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    let arr = [
        NonNullAscii::SmallA,
        NonNullAscii::SmallB,
        NonNullAscii::SmallC,
    ];
    assert_eq!(&arr[..], s.as_ascii());
}

#[test]
fn eq_empty_nonnull_slice() {
    let s: TinyAscii4 = TinyAscii4::empty();
    let arr: [NonNullAscii; 0] = [];
    assert_eq!(s.as_ascii(), &arr[..]);
    assert_eq!(&arr[..], s.as_ascii());
}

#[test]
fn eq_nonnull_slice_different_len() {
    let s: TinyAscii8 = TinyAscii8::try_new("ab").unwrap();
    let arr = [
        NonNullAscii::SmallA,
        NonNullAscii::SmallB,
        NonNullAscii::SmallC,
    ];
    assert_ne!(s.as_ascii(), &arr[..]);
}

// ---------------------------------------------------------------------------
// Ord / PartialOrd
// ---------------------------------------------------------------------------

#[test]
fn ord_same_length() {
    let a: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    let b: TinyAscii8 = TinyAscii8::try_new("abd").unwrap();
    assert!(a < b);
    assert!(b > a);
}

#[test]
fn ord_different_length() {
    let a: TinyAscii8 = TinyAscii8::try_new("a").unwrap();
    let b: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    assert!(a < b);
}

#[test]
fn ord_empty_vs_nonempty() {
    let a: TinyAscii8 = TinyAscii8::empty();
    let b: TinyAscii8 = TinyAscii8::try_new("a").unwrap();
    assert!(a < b);
}

#[test]
fn ord_equal() {
    let a: TinyAscii8 = TinyAscii8::try_new("foo").unwrap();
    let b: TinyAscii8 = TinyAscii8::try_new("foo").unwrap();
    assert!(a <= b);
    assert!(a >= b);
    assert!(a.cmp(&b) == core::cmp::Ordering::Equal);
}

#[test]
fn ord_lexicographic() {
    let a: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    let b: TinyAscii8 = TinyAscii8::try_new("abd").unwrap();
    assert!(a < b);
}

#[test]
fn ord_lowercase_vs_uppercase() {
    let upper: TinyAscii8 = TinyAscii8::try_new("Z").unwrap();
    let lower: TinyAscii8 = TinyAscii8::try_new("a").unwrap();
    assert!(upper < lower);
}

#[test]
fn partial_ord_consistent() {
    let a: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    let b: TinyAscii8 = TinyAscii8::try_new("def").unwrap();
    assert!(a.partial_cmp(&b) == Some(core::cmp::Ordering::Less));
    assert!(b.partial_cmp(&a) == Some(core::cmp::Ordering::Greater));
    assert!(a.partial_cmp(&a) == Some(core::cmp::Ordering::Equal));
}

// ---------------------------------------------------------------------------
// FromStr
// ---------------------------------------------------------------------------

#[test]
fn from_str_ok() {
    use core::str::FromStr;
    let s = TinyAscii8::from_str("hello");
    assert!(s.is_ok());
    assert_eq!(&*s.unwrap(), "hello");
}

#[test]
fn from_str_empty() {
    use core::str::FromStr;
    let s = TinyAscii8::from_str("");
    assert!(s.is_ok());
    assert!(s.unwrap().is_empty());
}

#[test]
fn from_str_invalid_ascii() {
    use core::str::FromStr;
    let s = TinyAscii8::from_str("hello \0");
    assert!(s.is_err());
}

#[test]
fn from_str_capacity_exceeded() {
    use core::str::FromStr;
    let s = TinyAscii4::from_str("toolong");
    assert!(s.is_err());
}

// ---------------------------------------------------------------------------
// TryFrom<&str>
// ---------------------------------------------------------------------------

#[test]
fn try_from_str_ok() {
    use core::convert::TryFrom;
    let s = TinyAscii8::try_from("hello");
    assert!(s.is_ok());
    assert_eq!(&*s.unwrap(), "hello");
}

#[test]
fn try_from_str_empty() {
    use core::convert::TryFrom;
    let s = TinyAscii8::try_from("");
    assert!(s.is_ok());
    assert!(s.unwrap().is_empty());
}

#[test]
fn try_from_str_invalid() {
    use core::convert::TryFrom;
    let s = TinyAscii8::try_from("has null \0 here");
    assert!(s.is_err());
}

#[test]
fn try_from_str_capacity() {
    use core::convert::TryFrom;
    let s = TinyAscii4::try_from("this is way too long");
    assert!(s.is_err());
}

// ---------------------------------------------------------------------------
// TryFrom<&[u8]>
// ---------------------------------------------------------------------------

#[test]
fn try_from_bytes_ok() {
    use core::convert::TryFrom;
    let s = TinyAscii8::try_from(&b"hello"[..]);
    assert!(s.is_ok());
    assert_eq!(&*s.unwrap(), "hello");
}

#[test]
fn try_from_bytes_empty() {
    use core::convert::TryFrom;
    let s = TinyAscii8::try_from(&b""[..]);
    assert!(s.is_ok());
    assert!(s.unwrap().is_empty());
}

#[test]
fn try_from_bytes_null() {
    use core::convert::TryFrom;
    let s = TinyAscii8::try_from(&b"ab\0cd"[..]);
    assert!(s.is_err());
}

#[test]
fn try_from_bytes_high_bytes() {
    use core::convert::TryFrom;
    let s = TinyAscii8::try_from(&b"\x80\x81\x82"[..]);
    assert!(s.is_err());
}

#[test]
fn try_from_bytes_capacity() {
    use core::convert::TryFrom;
    let s = TinyAscii4::try_from(&b"toolongbytes"[..]);
    assert!(s.is_err());
}

// ---------------------------------------------------------------------------
// From<TinyAscii<N>> for [u8; N]
// ---------------------------------------------------------------------------

#[test]
fn from_tinyascii_to_array() {
    let s: TinyAscii8 = TinyAscii8::try_new("abcd").unwrap();
    let arr: [u8; 8] = s.into();
    assert_eq!(arr[..4], [b'a', b'b', b'c', b'd']);
}

#[test]
fn from_tinyascii_empty_to_array() {
    let s: TinyAscii4 = TinyAscii4::empty();
    let arr: [u8; 4] = s.into();
    // Empty TinyAscii has buf[0] == 0 (is_empty checks this)
    assert_eq!(arr[0], 0);
}

#[test]
fn from_tinyascii_full_to_array() {
    let s: TinyAscii4 = TinyAscii4::try_new("abcd").unwrap();
    let arr: [u8; 4] = s.into();
    assert_eq!(arr, [b'a', b'b', b'c', b'd']);
}

#[test]
fn from_tinyascii_preserves_data() {
    let s: TinyAscii8 = TinyAscii8::try_new("12345678").unwrap();
    let arr: [u8; 8] = s.into();
    assert_eq!(&arr[..8], b"12345678");
}

#[test]
fn into_parts_roundtrip() {
    let s: TinyAscii16 = TinyAscii16::try_new("hello world!!!").unwrap();
    let (arr, len) = s.clone().into_parts();
    let rebuilt = unsafe { TinyAscii16::from_parts(arr, len) };
    assert_eq!(&rebuilt, "hello world!!!");
}

#[test]
fn into_parts_empty_roundtrip() {
    let s: TinyAscii4 = TinyAscii4::empty();
    let (arr, len) = s.into_parts();
    let rebuilt = unsafe { TinyAscii4::from_parts(arr, len) };
    assert!(rebuilt.is_empty());
    assert_eq!(&rebuilt, "");
}

#[test]
fn into_parts_preserves_capacity() {
    let s: TinyAscii8 = TinyAscii8::try_new("test").unwrap();
    let arr: [u8; 8] = s.into();
    assert_eq!(arr.len(), 8);
}

// ---------------------------------------------------------------------------
// Clone
// ---------------------------------------------------------------------------

#[test]
fn clone_preserves_content() {
    let s: TinyAscii8 = TinyAscii8::try_new("clone me").unwrap();
    let cloned = s.clone();
    assert_eq!(&*cloned, "clone me");
    assert_eq!(cloned.capacity(), 8);
}

#[test]
fn clone_empty() {
    let s: TinyAscii4 = TinyAscii4::empty();
    let cloned = s.clone();
    assert!(cloned.is_empty());
    assert_eq!(cloned.capacity(), 4);
}

#[test]
fn clone_is_deep_copy() {
    let s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    let mut cloned = s.clone();
    cloned.try_push_u8(b'x').unwrap();
    assert_eq!(&*s, "abc");
    assert_eq!(&*cloned, "abcx");
}

// ---------------------------------------------------------------------------
// Default
// ---------------------------------------------------------------------------

#[test]
fn default_is_empty_for_all_sizes() {
    let s4: TinyAscii4 = Default::default();
    let s8: TinyAscii8 = Default::default();
    let s16: TinyAscii16 = Default::default();
    let s32: TinyAscii32 = Default::default();
    let s64: TinyAscii64 = Default::default();
    let s128: TinyAscii128 = Default::default();

    assert!(s4.is_empty());
    assert!(s8.is_empty());
    assert!(s16.is_empty());
    assert!(s32.is_empty());
    assert!(s64.is_empty());
    assert!(s128.is_empty());
}

// ---------------------------------------------------------------------------
// Hash
// ---------------------------------------------------------------------------

#[test]
fn hash_equal_values_equal_hash() {
    use core::hash::{Hash, Hasher};

    let a: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    let b: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();

    let mut hasher_a = std::hash::DefaultHasher::new();
    a.hash(&mut hasher_a);
    let hash_a = hasher_a.finish();

    let mut hasher_b = std::hash::DefaultHasher::new();
    b.hash(&mut hasher_b);
    let hash_b = hasher_b.finish();

    assert_eq!(hash_a, hash_b);
}

#[test]
fn hash_different_values_different_hash() {
    use core::hash::{Hash, Hasher};

    let a: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    let b: TinyAscii8 = TinyAscii8::try_new("def").unwrap();

    let mut hasher_a = std::hash::DefaultHasher::new();
    a.hash(&mut hasher_a);

    let mut hasher_b = std::hash::DefaultHasher::new();
    b.hash(&mut hasher_b);

    // Very unlikely to collide
    assert_ne!(hasher_a.finish(), hasher_b.finish());
}

#[test]
fn hash_empty_consistent() {
    use core::hash::{Hash, Hasher};

    let a: TinyAscii4 = TinyAscii4::empty();
    let b: TinyAscii8 = TinyAscii8::empty();

    let mut hasher_a = std::hash::DefaultHasher::new();
    a.hash(&mut hasher_a);

    let mut hasher_b = std::hash::DefaultHasher::new();
    b.hash(&mut hasher_b);

    // Empty strings of different capacities should hash the same
    assert_eq!(hasher_a.finish(), hasher_b.finish());
}

// ---------------------------------------------------------------------------
// Edge case: special ASCII characters
// ---------------------------------------------------------------------------

#[test]
fn eq_space_only() {
    let s: TinyAscii8 = TinyAscii8::try_new(" ").unwrap();
    assert_eq!(&s, " ");
}

#[test]
fn eq_tab_char() {
    let bytes: [u8; 8] = [b'a', b'\t', b'b', 0, 0, 0, 0, 0];
    let s = unsafe { TinyAscii8::from_parts(bytes, 3) };
    assert_eq!(&s, "a\tb");
}

#[test]
fn eq_newline_char() {
    let bytes: [u8; 8] = [b'a', b'\n', b'b', 0, 0, 0, 0, 0];
    let s = unsafe { TinyAscii8::from_parts(bytes, 3) };
    assert_eq!(&s, "a\nb");
}

#[test]
fn eq_carriage_return() {
    let bytes: [u8; 8] = [b'\r', b'a', b'b', 0, 0, 0, 0, 0];
    let s = unsafe { TinyAscii8::from_parts(bytes, 3) };
    assert_eq!(&s, "\rab");
}

#[test]
fn eq_delete_char_in_string() {
    let bytes: [u8; 8] = [b'a', b'b', b'\x7f', 0, 0, 0, 0, 0];
    let s = unsafe { TinyAscii8::from_parts(bytes, 3) };
    assert_eq!(&s, "ab\x7f");
}

// ---------------------------------------------------------------------------
// Edge case: max-length within capacity
// ---------------------------------------------------------------------------

#[test]
fn eq_max_len_for_capacity() {
    let s: TinyAscii4 = TinyAscii4::try_new("abcd").unwrap();
    assert_eq!(&s, "abcd");
    assert_eq!(s.len(), 4);
    assert_eq!(s.capacity(), 4);
}

#[test]
fn ord_max_len_vs_shorter() {
    let short: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    let long: TinyAscii8 = TinyAscii8::try_new("abcd").unwrap();
    assert!(short < long);
}

// ---------------------------------------------------------------------------
// std feature: AsRef<Path>, AsRef<OsStr>, PartialEq<Path/PathBuf/OsStr>
// ---------------------------------------------------------------------------

#[cfg(feature = "std")]
mod std_tests {
    use std::path::PathBuf;

    use crate::TinyAscii8;

    #[test]
    fn as_ref_path() {
        let s: TinyAscii8 = TinyAscii8::try_new("tmp").unwrap();
        let path: &std::path::Path = s.as_ref();
        assert_eq!(path, std::path::Path::new("tmp"));
    }

    #[test]
    fn as_ref_path_relative() {
        let s: TinyAscii8 = TinyAscii8::try_new("foo").unwrap();
        let path: &std::path::Path = s.as_ref();
        assert_eq!(path, std::path::Path::new("foo"));
    }

    #[test]
    fn as_ref_os_str() {
        let s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
        let os: &std::ffi::OsStr = s.as_ref();
        assert_eq!(os, std::ffi::OsStr::new("hello"));
    }

    #[test]
    fn eq_path() {
        let s: TinyAscii8 = TinyAscii8::try_new("test").unwrap();
        assert_eq!(s, PathBuf::from("test"));
    }

    #[test]
    fn eq_path_not_equal() {
        let s: TinyAscii8 = TinyAscii8::try_new("test").unwrap();
        assert_ne!(s, PathBuf::from("other"));
    }

    #[test]
    fn eq_pathbuf() {
        let s: TinyAscii8 = TinyAscii8::try_new("test").unwrap();
        let pb = PathBuf::from("test");
        assert_eq!(s, pb);
    }

    #[test]
    fn eq_pathbuf_not_equal() {
        let s: TinyAscii8 = TinyAscii8::try_new("test").unwrap();
        let pb = PathBuf::from("other");
        assert_ne!(s, pb);
    }

    #[test]
    fn eq_path_str() {
        let s: TinyAscii8 = TinyAscii8::try_new("test").unwrap();
        assert_eq!("test", &s);
    }

    #[test]
    fn eq_os_string() {
        let s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
        let os = std::ffi::OsStr::new("hello");
        assert_eq!(std::ffi::OsStr::new(s.as_str()), os);
    }

    #[test]
    fn eq_os_string_not_equal() {
        let s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
        let os = std::ffi::OsStr::new("world");
        assert_ne!(std::ffi::OsStr::new(s.as_str()), os);
    }

    #[test]
    fn pathbuf_eq_reverse() {
        let s: TinyAscii8 = TinyAscii8::try_new("test").unwrap();
        assert_eq!(PathBuf::from("test"), s);
    }

    #[test]
    fn os_str_eq_reverse() {
        let s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
        let os = std::ffi::OsStr::new("hello");
        assert_eq!(os, &*s);
    }

    // ---------------------------------------------------------------------------
    // Borrow pattern in collections
    // ---------------------------------------------------------------------------

    #[test]
    fn tinyascii_in_hashmap() {
        use std::collections::HashMap;

        let mut map = HashMap::<TinyAscii8, i32>::new();
        let key = TinyAscii8::try_new("aaa").unwrap();
        map.insert(key, 17);

        assert_eq!(map.get("aaa"), Some(&17));
        assert_eq!(map.get(&TinyAscii8::try_new("aaa").unwrap()), Some(&17));
    }

    #[test]
    fn tinyascii_in_hashset() {
        use std::collections::HashSet;

        let mut set = HashSet::<TinyAscii8>::new();
        set.insert(TinyAscii8::try_new("a").unwrap());
        set.insert(TinyAscii8::try_new("b").unwrap());

        assert!(set.contains("a"));
        assert!(set.contains("b"));
        assert!(!set.contains("c"));
    }

    #[test]
    fn tinyascii_in_btree_map() {
        use alloc::collections::BTreeMap;

        let mut map = BTreeMap::<TinyAscii8, &str>::new();
        map.insert(TinyAscii8::try_new("c").unwrap(), "third");
        map.insert(TinyAscii8::try_new("a").unwrap(), "first");
        map.insert(TinyAscii8::try_new("b").unwrap(), "second");

        assert_eq!(map.get("a"), Some(&"first"));
        assert_eq!(map.get("b"), Some(&"second"));
        assert_eq!(map.get("c"), Some(&"third"));

        // Iterate in order
        let keys: Vec<&str> = map.keys().map(core::convert::AsRef::as_ref).collect();
        assert_eq!(keys, vec!["a", "b", "c"]);
    }

    #[test]
    fn tinyascii_in_btree_set() {
        use alloc::collections::BTreeSet;

        let mut set = BTreeSet::<TinyAscii8>::new();
        set.insert(TinyAscii8::try_new("c").unwrap());
        set.insert(TinyAscii8::try_new("a").unwrap());
        set.insert(TinyAscii8::try_new("b").unwrap());

        let items: Vec<&str> = set.iter().map(core::convert::AsRef::as_ref).collect();
        assert_eq!(items, vec!["a", "b", "c"]);
    }
}

// ---------------------------------------------------------------------------
// alloc feature: alloc tests
// ---------------------------------------------------------------------------
#[cfg(feature = "alloc")]
mod alloc_tests {
    use alloc::borrow::Cow;
    use alloc::string::String;

    use crate::tinyascii::{TinyAscii4, TinyAscii8};

    #[test]
    fn eq_cow_str_borrowed() {
        let s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
        let cow: Cow<str> = Cow::Borrowed("hello");
        assert_eq!(s, cow);
    }

    #[test]
    fn eq_cow_str_owned() {
        let s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
        let cow: Cow<str> = Cow::Owned("hello".to_string());
        assert_eq!(s, cow);
    }

    #[test]
    fn eq_cow_str_not_equal() {
        let s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
        let cow: Cow<str> = Cow::Borrowed("world");
        assert_ne!(s, cow);
    }

    #[test]
    fn eq_cow_str_reverse() {
        let s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
        let cow: Cow<str> = Cow::Borrowed("hello");
        assert_eq!(cow, s);
    }

    #[test]
    fn eq_string() {
        let s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
        let str_val: String = "hello".to_string();
        assert_eq!(s, str_val);
    }

    #[test]
    fn eq_string_not_equal() {
        let s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
        let str_val: String = "world".to_string();
        assert_ne!(s, str_val);
    }

    #[test]
    fn eq_string_reverse() {
        let s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
        let str_val: String = "hello".to_string();
        assert_eq!(str_val, s);
    }

    #[test]
    fn eq_empty_string() {
        let s: TinyAscii8 = TinyAscii8::empty();
        let str_val: String = String::new();
        assert_eq!(s, str_val);
    }

    #[test]
    fn from_tinyascii_to_cow() {
        use alloc::borrow::Cow;
        let s: TinyAscii4 = TinyAscii4::try_new("hi").unwrap();
        let cow: Cow<str> = s.as_str().into();
        assert_eq!(cow, "hi");
    }

    #[test]
    fn from_tinyascii_empty_to_cow() {
        use alloc::borrow::Cow;
        let s: TinyAscii4 = TinyAscii4::empty();
        let cow: Cow<str> = s.as_str().into();
        assert_eq!(cow, "");
    }
}
