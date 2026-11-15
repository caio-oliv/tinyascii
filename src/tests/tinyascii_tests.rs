#![allow(clippy::unwrap_used)]
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
    assert_eq!(TinyAscii4::new().len(), 0);
    assert_eq!(TinyAscii8::new().len(), 0);
    assert_eq!(TinyAscii16::new().len(), 0);
}

#[test]
fn empty_is_empty_true() {
    assert!(TinyAscii4::new().is_empty());
    assert!(TinyAscii8::new().is_empty());
    assert!(TinyAscii16::new().is_empty());
}

#[test]
fn empty_as_str() {
    assert_eq!(TinyAscii4::new().as_str(), "");
    assert_eq!(TinyAscii8::new().as_str(), "");
}

#[test]
fn empty_as_bytes() {
    assert_eq!(TinyAscii4::new().as_bytes(), &[] as &[u8]);
    assert_eq!(TinyAscii8::new().as_bytes(), &[] as &[u8]);
}

#[test]
fn empty_as_ascii() {
    let empty = TinyAscii4::new();
    let ascii: &[NonNullAscii] = empty.as_ascii();
    assert!(ascii.is_empty());
}

#[test]
fn empty_available_capacity() {
    assert_eq!(TinyAscii4::new().available_capacity(), 4);
    assert_eq!(TinyAscii8::new().available_capacity(), 8);
    assert_eq!(TinyAscii16::new().available_capacity(), 16);
}

#[test]
fn empty_is_full_false() {
    assert!(!TinyAscii4::new().is_full());
    assert!(!TinyAscii8::new().is_full());
}

#[test]
fn full_is_full_true() {
    let s: TinyAscii4 = TinyAscii4::try_from_str("abcd").unwrap();
    assert!(s.is_full());
    assert!(!s.is_empty());
    assert_eq!(s.available_capacity(), 0);
}

#[test]
fn try_from_str_various_sizes() {
    let s4: TinyAscii4 = TinyAscii4::try_from_str("ab").unwrap();
    assert_eq!(&s4, "ab");

    let s8: TinyAscii8 = TinyAscii8::try_from_str(&"hello world"[..5]).unwrap();
    assert_eq!(&s8, "hello");

    let s16: TinyAscii16 = TinyAscii16::try_from_str("test string").unwrap();
    assert_eq!(&s16, "test string");

    let s32: TinyAscii32 = TinyAscii32::try_from_str("a longer test string here").unwrap();
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
// from_bytes_unchecked
// ---------------------------------------------------------------------------

#[test]
fn from_bytes_unchecked_basic() {
    let s = unsafe { TinyAscii32::from_bytes_unchecked(b"Hello world!") };
    assert_eq!(&s, "Hello world!");
    assert_eq!(s.len(), 12);
    assert_eq!(s.as_bytes(), b"Hello world!");
}

#[test]
fn from_bytes_unchecked_empty() {
    let s = unsafe { TinyAscii4::from_bytes_unchecked(&[]) };
    assert!(s.is_empty());
    assert!(!s.is_full());
    assert_eq!(s.len(), 0);
    assert_eq!(&s, "");
}

#[test]
fn from_bytes_unchecked_single_byte() {
    let s = unsafe { TinyAscii4::from_bytes_unchecked(b"a") };
    assert_eq!(&s, "a");
    assert_eq!(s.len(), 1);
    assert_eq!(s.available_capacity(), 3);
}

#[test]
fn from_bytes_unchecked_full() {
    let s = unsafe { TinyAscii4::from_bytes_unchecked(b"abcd") };
    assert_eq!(&s, "abcd");
    assert!(s.is_full());
    assert_eq!(s.len(), 4);
    assert_eq!(s.available_capacity(), 0);
}

#[test]
fn from_bytes_unchecked_one_below_full() {
    let s = unsafe { TinyAscii4::from_bytes_unchecked(b"abc") };
    assert_eq!(&s, "abc");
    assert!(!s.is_full());
    assert_eq!(s.len(), 3);
    assert_eq!(s.available_capacity(), 1);
}

#[test]
fn from_bytes_unchecked_max_capacity() {
    let bytes = [b'x'; MAX_LEN];
    let s = unsafe { TinyAscii128::from_bytes_unchecked(&bytes) };
    assert!(s.is_full());
    assert_eq!(s.len(), MAX_LEN);
    assert_eq!(s.as_bytes(), &bytes);

    let s = unsafe { TinyAscii128::from_bytes_unchecked(&bytes[..MAX_LEN - 1]) };
    assert!(!s.is_full());
    assert_eq!(s.len(), MAX_LEN - 1);
    assert_eq!(s.as_bytes(), &bytes[..MAX_LEN - 1]);
}

#[test]
fn from_bytes_unchecked_all_lengths() {
    // Exercises every length (including SIMD block boundaries) for several capacities.
    let src: [u8; MAX_LEN] = core::array::from_fn(|i| b'!' + u8::try_from(i % 90).unwrap());

    macro_rules! check {
        ($ty:ty, $n:expr) => {
            for len in 0..=$n {
                let s = unsafe { <$ty>::from_bytes_unchecked(&src[..len]) };
                assert_eq!(s.len(), len);
                assert_eq!(s.as_bytes(), &src[..len]);
                assert_eq!(s.is_full(), len == $n);
            }
        };
    }

    check!(TinyAscii4, 4);
    check!(TinyAscii8, 8);
    check!(TinyAscii16, 16);
    check!(TinyAscii32, 32);
    check!(TinyAscii64, 64);
    check!(TinyAscii128, 128);
}

#[test]
fn from_bytes_unchecked_odd_capacities() {
    let s = unsafe { TinyAscii::<1>::from_bytes_unchecked(b"z") };
    assert_eq!(&s, "z");
    assert!(s.is_full());

    let s = unsafe { TinyAscii::<1>::from_bytes_unchecked(b"") };
    assert!(s.is_empty());

    let s = unsafe { TinyAscii::<7>::from_bytes_unchecked(b"abcdefg") };
    assert_eq!(&s, "abcdefg");

    let s = unsafe { TinyAscii::<33>::from_bytes_unchecked(b"0123456789abcdef0123456789abcdef") };
    assert_eq!(&s, "0123456789abcdef0123456789abcdef");
    assert_eq!(s.len(), 32);
    assert!(!s.is_full());
}

#[test]
fn from_bytes_unchecked_matches_try_from_bytes() {
    let inputs: [&[u8]; 5] = [b"", b"a", b"hello", b"with spaces & symbols!~", b"~~~~~~~~~~~~~~~~"];
    for bytes in inputs {
        let a = unsafe { TinyAscii32::from_bytes_unchecked(bytes) };
        let b = TinyAscii32::try_from_bytes(bytes).unwrap();
        assert_eq!(a.as_bytes(), b.as_bytes());
        assert_eq!(a.len(), b.len());
        assert_eq!(a.is_full(), b.is_full());
    }
}

#[test]
fn from_bytes_unchecked_boundary_ascii_values() {
    // Lowest (0x01) and highest (0x7F) valid non-null ASCII values.
    let bytes = [0x01, 0x7F, 0x20, 0x7E];
    let s = unsafe { TinyAscii8::from_bytes_unchecked(&bytes) };
    assert_eq!(s.as_bytes(), &bytes);
    assert_eq!(s.len(), 4);

    let full = unsafe { TinyAscii4::from_bytes_unchecked(&bytes) };
    assert_eq!(full.as_bytes(), &bytes);
    assert!(full.is_full());
}

#[test]
fn from_bytes_unchecked_then_mutate() {
    let mut s = unsafe { TinyAscii8::from_bytes_unchecked(b"foo") };
    s.try_push_str("bar").unwrap();
    assert_eq!(&s, "foobar");

    let mut s = unsafe { TinyAscii4::from_bytes_unchecked(b"abcd") };
    assert_eq!(s.pop(), Some(NonNullAscii::SmallD));
    assert_eq!(&s, "abc");
}

#[test]
fn from_bytes_unchecked_into_parts_roundtrip() {
    let s = unsafe { TinyAscii16::from_bytes_unchecked(b"roundtrip") };
    let (arr, len) = s.into_parts();
    assert_eq!(len, 9);
    let rebuilt = unsafe { TinyAscii16::from_parts(arr, len) };
    assert_eq!(&rebuilt, "roundtrip");
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
    let s: TinyAscii4 = TinyAscii4::try_from_str("hi").unwrap();
    let s16: TinyAscii16 = s.resize::<16>();
    assert_eq!(&s16, "hi");
    assert_eq!(s16.capacity(), 16);
    assert_eq!(s16.available_capacity(), 14);
}

#[test]
fn resize_truncate() {
    let s: TinyAscii32 = TinyAscii32::try_from_str("hello world!!!").unwrap();
    let s8: TinyAscii8 = s.resize::<8>();
    assert_eq!(&s8, "hello wo");
    assert_eq!(s8.capacity(), 8);
}

#[test]
fn resize_empty() {
    let s: TinyAscii32 = TinyAscii32::new();
    let s8: TinyAscii8 = s.resize::<8>();
    assert!(s8.is_empty());
    assert_eq!(s8.capacity(), 8);
}

#[test]
fn resize_full_to_full() {
    let s: TinyAscii4 = TinyAscii4::try_from_str("abcd").unwrap();
    let s4: TinyAscii4 = s.resize::<4>();
    assert_eq!(&*s4, "abcd");
}

// ---------------------------------------------------------------------------
// into_parts / from_parts roundtrip
// ---------------------------------------------------------------------------

#[test]
fn into_parts_roundtrip_basic() {
    let s: TinyAscii8 = TinyAscii8::try_from_str("hello").unwrap();
    let (arr, len) = s.clone().into_parts();
    let rebuilt = unsafe { TinyAscii8::from_parts(arr, len) };
    assert_eq!(&rebuilt, "hello");
}

#[test]
fn into_parts_roundtrip_empty() {
    let s: TinyAscii4 = TinyAscii4::new();
    let (arr, len) = s.into_parts();
    let rebuilt = unsafe { TinyAscii4::from_parts(arr, len) };
    assert!(rebuilt.is_empty());
    assert_eq!(&rebuilt, "");
}

#[test]
fn into_parts_roundtrip_full() {
    let s: TinyAscii4 = TinyAscii4::try_from_str("abcd").unwrap();
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
fn into_parts_zero_fills_bytes_past_len() {
    // Fresh (partially uninitialized) buffer.
    let s: TinyAscii8 = TinyAscii8::try_from_str("abc").unwrap();
    let (arr, len) = s.into_parts();
    assert_eq!(len, 3);
    assert_eq!(&arr[..3], b"abc");
    assert_eq!(&arr[3..], &[0; 5]);

    // Empty string: the whole array is zero, including the length marker.
    let (arr, len) = TinyAscii8::new().into_parts();
    assert_eq!(len, 0);
    assert_eq!(arr, [0; 8]);

    // Capacity of 1.
    let (arr, len) = TinyAscii::<1>::new().into_parts();
    assert_eq!((arr, len), ([0], 0));

    // Full string: the last byte is content, not a marker.
    let s: TinyAscii4 = TinyAscii4::try_from_str("abcd").unwrap();
    let (arr, len) = s.into_parts();
    assert_eq!((&arr, len), (b"abcd", 4));
}

#[test]
fn into_parts_zero_fills_stale_bytes_after_shrinking() {
    // Stale bytes left in the buffer by `pop`, `truncate` and `clear`
    // must not leak into the returned array.
    let mut s: TinyAscii8 = TinyAscii8::try_from_str("abcdefgh").unwrap();
    s.pop();
    s.truncate(5);
    let (arr, len) = s.clone().into_parts();
    assert_eq!(len, 5);
    assert_eq!(&arr[..5], b"abcde");
    assert_eq!(&arr[5..], &[0; 3]);

    s.clear();
    let (arr, len) = s.into_parts();
    assert_eq!((arr, len), ([0; 8], 0));
}

#[test]
fn into_parts_zero_fills_max_capacity() {
    let mut s = TinyAscii128::try_from_str("xyz").unwrap();
    s.try_push(NonNullAscii::SmallW).unwrap();
    let (arr, len) = s.into_parts();
    assert_eq!(len, 4);
    assert_eq!(&arr[..4], b"xyzw");
    assert!(arr[4..].iter().all(|&b| b == 0));
}

#[test]
fn into_parts_from_parts_modify() {
    let s: TinyAscii8 = TinyAscii8::try_from_str("foo").unwrap();
    let (mut arr, len) = s.into_parts();
    arr[1] = b'O';
    let rebuilt = unsafe { TinyAscii8::from_parts(arr, len) };
    assert_eq!(&rebuilt, "fOo");
}
