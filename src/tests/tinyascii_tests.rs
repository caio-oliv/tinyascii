#![allow(clippy::unwrap_used)]
#![allow(clippy::as_conversions)]
#![allow(clippy::undocumented_unsafe_blocks)]

use crate::{
    MAX_LEN, NonNullAscii, TinyAscii, TinyAscii4, TinyAscii8, TinyAscii16, TinyAscii32,
    TinyAscii64, TinyAscii128,
};

// ---------------------------------------------------------------------------
// Size / Memory layout
// ---------------------------------------------------------------------------

#[test]
fn size_of_tinyascii() {
    // TinyAscii<N> should be exactly N bytes (plus any padding from the enum if needed,
    // but since it's just [u8; N], it should be N).
    assert_eq!(core::mem::size_of::<TinyAscii4>(), 4);
    assert_eq!(core::mem::size_of::<TinyAscii8>(), 8);
    assert_eq!(core::mem::size_of::<TinyAscii16>(), 16);
    assert_eq!(core::mem::size_of::<TinyAscii32>(), 32);
    assert_eq!(core::mem::size_of::<TinyAscii64>(), 64);
    assert_eq!(core::mem::size_of::<TinyAscii128>(), 128);
}

#[test]
fn align_of_tinyascii() {
    // TinyAscii<N> should have align of 1 (same as u8).
    assert_eq!(core::mem::align_of::<TinyAscii4>(), 1);
    assert_eq!(core::mem::align_of::<TinyAscii8>(), 1);
    assert_eq!(core::mem::align_of::<TinyAscii16>(), 1);
    assert_eq!(core::mem::align_of::<TinyAscii32>(), 1);
    assert_eq!(core::mem::align_of::<TinyAscii64>(), 1);
    assert_eq!(core::mem::align_of::<TinyAscii128>(), 1);
}

#[test]
fn tinyascii_is_packed() {
    // Verify there's no padding inside the struct.
    assert_eq!(
        core::mem::size_of::<TinyAscii8>(),
        core::mem::size_of::<[u8; 8]>()
    );
}

// ---------------------------------------------------------------------------
// empty
// ---------------------------------------------------------------------------

#[test]
fn empty_len_is_zero() {
    assert_eq!(TinyAscii4::empty().len(), 0);
    assert_eq!(TinyAscii8::empty().len(), 0);
    assert_eq!(TinyAscii16::empty().len(), 0);
}

#[test]
fn empty_is_empty_true() {
    assert!(TinyAscii4::empty().is_empty());
    assert!(TinyAscii8::empty().is_empty());
    assert!(TinyAscii16::empty().is_empty());
}

#[test]
fn empty_as_str() {
    assert_eq!(TinyAscii4::empty().as_str(), "");
    assert_eq!(TinyAscii8::empty().as_str(), "");
}

#[test]
fn empty_as_bytes() {
    assert_eq!(TinyAscii4::empty().as_bytes(), &[] as &[u8]);
    assert_eq!(TinyAscii8::empty().as_bytes(), &[] as &[u8]);
}

#[test]
fn empty_as_ascii() {
    let empty = TinyAscii4::empty();
    let ascii: &[NonNullAscii] = empty.as_ascii();
    assert!(ascii.is_empty());
}

#[test]
fn empty_available_capacity() {
    assert_eq!(TinyAscii4::empty().available_capacity(), 4);
    assert_eq!(TinyAscii8::empty().available_capacity(), 8);
    assert_eq!(TinyAscii16::empty().available_capacity(), 16);
}

#[test]
fn empty_is_full_false() {
    assert!(!TinyAscii4::empty().is_full());
    assert!(!TinyAscii8::empty().is_full());
}

#[test]
fn full_is_full_true() {
    let s: TinyAscii4 = TinyAscii4::try_new("abcd").unwrap();
    assert!(s.is_full());
    assert!(!s.is_empty());
    assert_eq!(s.available_capacity(), 0);
}

#[test]
fn try_new_various_sizes() {
    let s4: TinyAscii4 = TinyAscii4::try_new("ab").unwrap();
    assert_eq!(&s4, "ab");

    let s8: TinyAscii8 = TinyAscii8::try_new(&"hello world"[..5]).unwrap();
    assert_eq!(&s8, "hello");

    let s16: TinyAscii16 = TinyAscii16::try_new("test string").unwrap();
    assert_eq!(&s16, "test string");

    let s32: TinyAscii32 = TinyAscii32::try_new("a longer test string here").unwrap();
    assert_eq!(&s32, "a longer test string here");
}

#[test]
fn try_from_bytes_various() {
    let s: TinyAscii8 = TinyAscii8::try_from_bytes(b"xyz".as_slice()).unwrap();
    assert_eq!(&s, "xyz");

    let s: TinyAscii16 = TinyAscii16::try_from_bytes(b"abc").unwrap();
    assert_eq!(&s, "abc");
}

// ---------------------------------------------------------------------------
// from_ascii_array
// ---------------------------------------------------------------------------

#[test]
fn from_ascii_array_basic() {
    let arr = [
        NonNullAscii::SmallY,
        NonNullAscii::SmallE,
        NonNullAscii::SmallS,
    ];
    let s: TinyAscii<3> = TinyAscii::from_ascii_array(arr);
    assert_eq!(&s, "yes");
}

#[test]
fn from_ascii_array_full() {
    let arr = [NonNullAscii::SmallA; 4];
    let s: TinyAscii4 = TinyAscii4::from_ascii_array(arr);
    assert_eq!(&s, "aaaa");
    assert!(s.is_full());
}

// ---------------------------------------------------------------------------
// from_ascii_lossy
// ---------------------------------------------------------------------------

#[test]
fn from_ascii_lossy_clean() {
    let bytes = b"hello";
    let s: TinyAscii8 = TinyAscii8::from_ascii_lossy(bytes, NonNullAscii::QuestionMark).unwrap();
    assert_eq!(&s, "hello");
}

#[test]
fn from_ascii_lossy_replaces_null() {
    let bytes = [b'h', b'i', 0, b'!'];
    let s: TinyAscii8 = TinyAscii8::from_ascii_lossy(&bytes, NonNullAscii::QuestionMark).unwrap();
    assert_eq!(&s, "hi?!");
}

#[test]
fn from_ascii_lossy_replaces_high_bytes() {
    let bytes = [65, 200, 201, 66];
    let s: TinyAscii8 = TinyAscii8::from_ascii_lossy(&bytes, NonNullAscii::Tilde).unwrap();
    assert_eq!(&s, "A~~B");
}

#[test]
fn from_ascii_lossy_capacity_exceeded() {
    let bytes = [b'a'; 500];
    let err = TinyAscii8::from_ascii_lossy(&bytes, NonNullAscii::QuestionMark);
    assert!(err.is_err());
}

#[test]
fn from_ascii_lossy_empty() {
    let bytes: [u8; 0] = [];
    let s: TinyAscii4 = TinyAscii4::from_ascii_lossy(&bytes, NonNullAscii::QuestionMark).unwrap();
    assert!(s.is_empty());
}

// ---------------------------------------------------------------------------
// resize
// ---------------------------------------------------------------------------

#[test]
fn resize_extend() {
    let s: TinyAscii4 = TinyAscii4::try_new("hi").unwrap();
    let s16: TinyAscii16 = s.resize::<16>();
    assert_eq!(&s16, "hi");
    assert_eq!(s16.capacity(), 16);
    assert_eq!(s16.available_capacity(), 14);
}

#[test]
fn resize_truncate() {
    let s: TinyAscii32 = TinyAscii32::try_new("hello world!!!").unwrap();
    let s8: TinyAscii8 = s.resize::<8>();
    assert_eq!(&s8, "hello wo");
    assert_eq!(s8.capacity(), 8);
}

#[test]
fn resize_empty() {
    let s: TinyAscii32 = TinyAscii32::empty();
    let s8: TinyAscii8 = s.resize::<8>();
    assert!(s8.is_empty());
    assert_eq!(s8.capacity(), 8);
}

#[test]
fn resize_full_to_full() {
    let s: TinyAscii4 = TinyAscii4::try_new("abcd").unwrap();
    let s4: TinyAscii4 = s.resize::<4>();
    assert_eq!(&*s4, "abcd");
}

// ---------------------------------------------------------------------------
// into_parts / from_parts roundtrip
// ---------------------------------------------------------------------------

#[test]
fn into_parts_roundtrip_basic() {
    let s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
    let (arr, len) = s.clone().into_parts();
    let rebuilt = unsafe { TinyAscii8::from_parts(arr, len) };
    assert_eq!(&rebuilt, "hello");
}

#[test]
fn into_parts_roundtrip_empty() {
    let s: TinyAscii4 = TinyAscii4::empty();
    let (arr, len) = s.into_parts();
    let rebuilt = unsafe { TinyAscii4::from_parts(arr, len) };
    assert!(rebuilt.is_empty());
    assert_eq!(&rebuilt, "");
}

#[test]
fn into_parts_roundtrip_full() {
    let s: TinyAscii4 = TinyAscii4::try_new("abcd").unwrap();
    let (arr, len) = s.into_parts();
    let rebuilt = unsafe { TinyAscii4::from_parts(arr, len) };
    assert_eq!(&rebuilt, "abcd");
}

#[test]
fn into_parts_roundtrip_max_capacity() {
    let max: [u8; MAX_LEN] = [b'x'; MAX_LEN];
    let s = unsafe { TinyAscii128::from_parts(max, MAX_LEN) };
    assert_eq!(s.len(), MAX_LEN);
    assert!(s.is_full());

    let (arr2, len2) = s.into_parts();
    let rebuilt = unsafe { TinyAscii128::from_parts(arr2, len2) };
    assert_eq!(&rebuilt, "x".repeat(MAX_LEN).as_str());
}

#[test]
fn into_parts_from_parts_modify() {
    let s: TinyAscii8 = TinyAscii8::try_new("foo").unwrap();
    let (mut arr, len) = s.into_parts();
    arr[1] = b'O';
    let rebuilt = unsafe { TinyAscii8::from_parts(arr, len) };
    assert_eq!(&rebuilt, "fOo");
}
