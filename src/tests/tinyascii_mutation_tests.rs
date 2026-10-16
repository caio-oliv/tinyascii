#![allow(clippy::unwrap_used)]
#![allow(clippy::undocumented_unsafe_blocks)]

//! Tests for `TinyAscii` mutation APIs: push, insert, remove, truncate, clear, retain.

use core::assert_matches;

use crate::error::{
    CapacityLimitError, InsertAsciiError, InsertRangeError, RemoveRangeError, WriteAsciiError,
};
use crate::{NonNullAscii, TinyAscii, TinyAscii4, TinyAscii8, TinyAscii16, TinyAscii32};

// ---------------------------------------------------------------------------
// try_push / try_push_u8 / try_push_str / try_push_bytes
// ---------------------------------------------------------------------------

#[test]
fn try_push_single() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("ab").unwrap();
    s.try_push(NonNullAscii::SmallC).unwrap();
    assert_eq!(&*s, "abc");
}

#[test]
fn try_push_multiple() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("a").unwrap();
    s.try_push(NonNullAscii::SmallB).unwrap();
    s.try_push(NonNullAscii::SmallC).unwrap();
    s.try_push(NonNullAscii::SmallD).unwrap();
    assert_eq!(&*s, "abcd");
}

#[test]
fn try_push_empty() {
    let mut s: TinyAscii4 = TinyAscii4::empty();
    s.try_push(NonNullAscii::SmallA).unwrap();
    assert_eq!(&*s, "a");
}

#[test]
fn try_push_full_rejects() {
    let mut s: TinyAscii4 = TinyAscii4::try_new("abcd").unwrap();
    let err = s.try_push(NonNullAscii::SmallE);
    assert_eq!(err, Err(CapacityLimitError));
    assert_eq!(&*s, "abcd");
}

#[test]
fn try_push_u8_ok() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("a").unwrap();
    s.try_push_u8(b'z').unwrap();
    assert_eq!(&*s, "az");
}

#[test]
fn try_push_u8_null_rejected() {
    let mut s: TinyAscii8 = TinyAscii8::empty();
    let err = s.try_push_u8(0);
    assert_eq!(err, Err(WriteAsciiError::InvalidAscii));
    assert!(s.is_empty());
}

#[test]
fn try_push_u8_high_byte_rejected() {
    let mut s: TinyAscii8 = TinyAscii8::empty();
    let err = s.try_push_u8(200);
    assert_eq!(err, Err(WriteAsciiError::InvalidAscii));
}

#[test]
fn try_push_u8_full_rejects_capacity() {
    let mut s: TinyAscii4 = TinyAscii4::try_new("abcd").unwrap();
    let err = s.try_push_u8(b'e');
    assert!(matches!(err, Err(WriteAsciiError::Capacity(_))));
}

#[test]
fn try_push_str_ok() {
    let mut s = TinyAscii16::try_new("hello").unwrap();
    s.try_push_str(" world").unwrap();
    assert_eq!(&s, "hello world");
}

#[test]
fn try_push_str_empty() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("test").unwrap();
    s.try_push_str("").unwrap();
    assert_eq!(&*s, "test");
}

#[test]
fn try_push_str_invalid_rejected() {
    let mut s: TinyAscii16 = TinyAscii16::try_new("hello").unwrap();
    let err = s.try_push_str(" 😎");
    assert_matches!(err, Err(WriteAsciiError::InvalidAscii));
    assert_eq!(&*s, "hello");
}

#[test]
fn try_push_str_full_rejects() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("hello!!").unwrap();
    let err = s.try_push_str(" x");
    assert_matches!(err, Err(WriteAsciiError::Capacity(_)));
    assert_eq!(&*s, "hello!!");
}

#[test]
fn try_push_bytes_ok() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("a").unwrap();
    s.try_push_bytes(b"bc").unwrap();
    assert_eq!(&*s, "abc");
}

#[test]
fn try_push_bytes_empty() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("test").unwrap();
    s.try_push_bytes(&[]).unwrap();
    assert_eq!(&*s, "test");
}

#[test]
fn try_push_bytes_null_in_rejected() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("a").unwrap();
    let err = s.try_push_bytes(b"\0");
    assert_eq!(err, Err(WriteAsciiError::InvalidAscii));
    assert_eq!(&*s, "a");
}

#[test]
fn try_push_bytes_high_byte_rejected() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("a").unwrap();
    let err = s.try_push_bytes(&[200]);
    assert_eq!(err, Err(WriteAsciiError::InvalidAscii));
}

#[test]
fn try_push_bytes_full_rejects() {
    let mut s: TinyAscii4 = TinyAscii4::try_new("abcd").unwrap();
    let err = s.try_push_bytes(b"e");
    assert_matches!(err, Err(WriteAsciiError::Capacity(_)));
}

// ---------------------------------------------------------------------------
// try_push_slice
// ---------------------------------------------------------------------------

#[test]
fn try_push_slice_ok() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("foo").unwrap();
    let bar = [
        NonNullAscii::SmallB,
        NonNullAscii::SmallA,
        NonNullAscii::SmallR,
    ];
    s.try_push_slice(&bar).unwrap();
    assert_eq!(&*s, "foobar");
}

#[test]
fn try_push_slice_empty() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("test").unwrap();
    s.try_push_slice(&[]).unwrap();
    assert_eq!(&*s, "test");
}

#[test]
fn try_push_slice_full_rejects() {
    let mut s: TinyAscii4 = TinyAscii4::try_new("foo").unwrap();
    let bar = [
        NonNullAscii::SmallB,
        NonNullAscii::SmallA,
        NonNullAscii::SmallR,
    ];
    let err = s.try_push_slice(&bar);
    assert_eq!(err, Err(CapacityLimitError));
    assert_eq!(&*s, "foo");
}

// ---------------------------------------------------------------------------
// try_push_lossy
// ---------------------------------------------------------------------------

#[test]
fn try_push_lossy_ok() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("a").unwrap();
    s.try_push_lossy(&[b'b', 0, b'c'], NonNullAscii::QuestionMark)
        .unwrap();
    assert_eq!(&s, "ab?c");
}

#[test]
fn try_push_lossy_empty() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("test").unwrap();
    s.try_push_lossy(&[], NonNullAscii::QuestionMark).unwrap();
    assert_eq!(&*s, "test");
}

#[test]
fn try_push_lossy_full_rejects() {
    let mut s: TinyAscii4 = TinyAscii4::try_new("abcd").unwrap();
    let err = s.try_push_lossy(b"e", NonNullAscii::QuestionMark);
    assert_eq!(err, Err(CapacityLimitError));
    assert_eq!(&s, "abcd");
}

// ---------------------------------------------------------------------------
// clear
// ---------------------------------------------------------------------------

#[test]
fn clear_basic() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
    s.clear();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
    assert_eq!(s.capacity(), 8);
    assert_eq!(&*s, "");
}

#[test]
fn clear_empty() {
    let mut s: TinyAscii4 = TinyAscii4::empty();
    s.clear();
    assert!(s.is_empty());
}

#[test]
fn clear_full() {
    let mut s: TinyAscii4 = TinyAscii4::try_new("abcd").unwrap();
    s.clear();
    assert!(s.is_empty());
    assert_eq!(s.capacity(), 4);
}

// ---------------------------------------------------------------------------
// truncate
// ---------------------------------------------------------------------------

#[test]
fn truncate_shorter() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
    s.truncate(2);
    assert_eq!(&s, "he");
    assert_eq!(s.len(), 2);
}

#[test]
fn truncate_to_zero() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
    s.truncate(0);
    assert!(s.is_empty());
    assert_eq!(&s, "");
}

#[test]
fn truncate_same_length_noop() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
    s.truncate(5);
    assert_eq!(&*s, "hello");
}

#[test]
fn truncate_longer_noop() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("hi").unwrap();
    s.truncate(10);
    assert_eq!(&*s, "hi");
    assert_eq!(s.len(), 2);
}

#[test]
fn truncate_empty_noop() {
    let mut s: TinyAscii4 = TinyAscii4::empty();
    s.truncate(0);
    assert!(s.is_empty());
}

#[test]
fn truncate_preserves_capacity() {
    let mut s = TinyAscii16::try_new("hello world").unwrap();
    s.truncate(5);
    assert_eq!(s.capacity(), 16);
}

// ---------------------------------------------------------------------------
// try_insert
// ---------------------------------------------------------------------------

#[test]
fn try_insert_at_start() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("bc").unwrap();
    s.try_insert(0, NonNullAscii::SmallA).unwrap();
    assert_eq!(&*s, "abc");
}

#[test]
fn try_insert_in_middle() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("ac").unwrap();
    s.try_insert(1, NonNullAscii::SmallB).unwrap();
    assert_eq!(&*s, "abc");
}

#[test]
fn try_insert_at_end() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    s.try_insert(3, NonNullAscii::SmallD).unwrap();
    assert_eq!(&s, "abcd");
}

#[test]
fn try_insert_empty() {
    let mut s: TinyAscii4 = TinyAscii4::empty();
    s.try_insert(0, NonNullAscii::SmallA).unwrap();
    assert_eq!(&s, "a");
}

#[test]
fn try_insert_full_rejects_capacity() {
    let mut s: TinyAscii4 = TinyAscii4::try_new("abcd").unwrap();
    let err = s.try_insert(0, NonNullAscii::SmallE);
    assert_matches!(err, Err(InsertAsciiError::Capacity(_)));
    assert_eq!(&*s, "abcd");
}

#[test]
fn try_insert_out_of_bounds() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    let err = s.try_insert(4, NonNullAscii::SmallD);
    assert_eq!(err, Err(InsertAsciiError::IndexOutOfBounds));
    assert_eq!(&s, "abc");
}

#[test]
fn try_insert_many_times() {
    let mut s: TinyAscii8 = TinyAscii8::empty();
    s.try_insert(0, NonNullAscii::CapitalC).unwrap();
    s.try_insert(0, NonNullAscii::CapitalB).unwrap();
    s.try_insert(0, NonNullAscii::CapitalA).unwrap();
    assert_eq!(&s, "ABC");
}

// ---------------------------------------------------------------------------
// try_insert_slice
// ---------------------------------------------------------------------------

#[test]
fn try_insert_slice_at_start() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("def").unwrap();
    let abc = [
        NonNullAscii::SmallA,
        NonNullAscii::SmallB,
        NonNullAscii::SmallC,
    ];
    s.try_insert_slice(0, &abc).unwrap();
    assert_eq!(&*s, "abcdef");
}

#[test]
fn try_insert_slice_in_middle() {
    let mut s: TinyAscii16 = TinyAscii16::try_new("ae").unwrap();
    let bc = [
        NonNullAscii::SmallB,
        NonNullAscii::SmallC,
        NonNullAscii::SmallD,
    ];
    s.try_insert_slice(1, &bc).unwrap();
    assert_eq!(&*s, "abcde");
}

#[test]
fn try_insert_slice_at_end() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    let d = [NonNullAscii::SmallD];
    s.try_insert_slice(3, &d).unwrap();
    assert_eq!(&*s, "abcd");
}

#[test]
fn try_insert_slice_empty_slice() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    s.try_insert_slice(1, &[]).unwrap();
    assert_eq!(&*s, "abc");
}

#[test]
fn try_insert_slice_full_rejects_capacity() {
    let mut s: TinyAscii4 = TinyAscii4::try_new("abc").unwrap();
    let d = [NonNullAscii::SmallD, NonNullAscii::SmallE];
    let err = s.try_insert_slice(3, &d);
    assert_matches!(err, Err(InsertAsciiError::Capacity(_)));
    assert_eq!(&*s, "abc");
}

#[test]
fn try_insert_slice_out_of_bounds() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    let d = [NonNullAscii::SmallD];
    let err = s.try_insert_slice(4, &d);
    assert_eq!(err, Err(InsertAsciiError::IndexOutOfBounds));
}

// ---------------------------------------------------------------------------
// extend_from_within
// ---------------------------------------------------------------------------

#[test]
fn extend_from_within_tail() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abcde").unwrap();
    s.extend_from_within(2..).unwrap();
    assert_eq!(&s, "abcdecde");
}

#[test]
fn extend_from_within_head() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abcde").unwrap();
    s.extend_from_within(..2).unwrap();
    assert_eq!(&s, "abcdeab");
}

#[test]
fn extend_from_within_range() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abcde").unwrap();
    s.extend_from_within(1..3).unwrap();
    assert_eq!(&s, "abcdebc");
}

#[test]
fn extend_from_within_all() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abc").unwrap();
    s.extend_from_within(..).unwrap();
    assert_eq!(&s, "abcabc");
}

#[test]
fn extend_from_within_empty_source() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abc").unwrap();
    s.extend_from_within(0..0).unwrap();
    assert_eq!(&s, "abc");
}

#[test]
fn extend_from_within_full_rejects() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abcde").unwrap();
    let err = s.extend_from_within(..);
    assert!(matches!(err, Err(InsertRangeError::Capacity(_))));
}

#[test]
fn extend_from_within_out_of_bounds_start() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abcde").unwrap();
    let err = s.extend_from_within(6..);
    assert_eq!(err, Err(InsertRangeError::RangeOutOfBounds));
}

#[test]
fn extend_from_within_out_of_bounds_end() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abcde").unwrap();
    let err = s.extend_from_within(..7);
    assert_eq!(err, Err(InsertRangeError::RangeOutOfBounds));
}

#[test]
fn extend_from_within_swapped_range() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abcde").unwrap();
    #[allow(clippy::reversed_empty_ranges)]
    let err = s.extend_from_within(3..2);
    assert_eq!(err, Err(InsertRangeError::RangeOutOfBounds));
}

// ---------------------------------------------------------------------------
// replace_range
// ---------------------------------------------------------------------------

#[test]
fn replace_range_middle() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abcde").unwrap();
    let repl = [NonNullAscii::SmallX];
    s.replace_range(1..3, &repl).unwrap();
    assert_eq!(&*s, "axde");
}

#[test]
fn replace_range_head() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abcde").unwrap();
    let repl = [NonNullAscii::SmallX];
    s.replace_range(..2, &repl).unwrap();
    assert_eq!(&*s, "xcde");
}

#[test]
fn replace_range_tail() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abcde").unwrap();
    let repl = [NonNullAscii::SmallX];
    s.replace_range(3.., &repl).unwrap();
    assert_eq!(&*s, "abcx");
}

#[test]
fn replace_range_all() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abc").unwrap();
    let repl = [NonNullAscii::SmallX, NonNullAscii::SmallY];
    s.replace_range(.., &repl).unwrap();
    assert_eq!(&*s, "xy");
}

#[test]
fn replace_range_empty_replacement() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abcde").unwrap();
    s.replace_range(1..3, &[]).unwrap();
    assert_eq!(&*s, "ade");
}

#[test]
fn replace_range_larger_replacement() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abc").unwrap();
    let repl = [NonNullAscii::SmallX; 10];
    s.replace_range(1.., &repl).unwrap();
    assert_eq!(s.len(), 11);
}

#[test]
fn replace_range_out_of_bounds_end() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abcde").unwrap();
    let repl = [NonNullAscii::SmallX];
    let err = s.replace_range(2..6, &repl);
    assert_eq!(err, Err(InsertRangeError::RangeOutOfBounds));
}

#[test]
fn replace_range_swapped() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abcde").unwrap();
    let repl = [NonNullAscii::SmallX];
    #[allow(clippy::reversed_empty_ranges)]
    let err = s.replace_range(3..2, &repl);
    assert_eq!(err, Err(InsertRangeError::RangeOutOfBounds));
}

#[test]
fn replace_range_capacity_exceeded() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abcde").unwrap();
    let repl = [NonNullAscii::SmallX; 10];
    let err = s.replace_range(.., &repl);
    assert!(matches!(err, Err(InsertRangeError::Capacity(_))));
}

#[test]
fn replace_range_empty_target() {
    let mut s: TinyAscii32 = TinyAscii32::empty();
    let repl = [NonNullAscii::SmallA];
    s.replace_range(.., &repl).unwrap();
    assert_eq!(&*s, "a");
}

#[test]
fn replace_range_empty_range() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abc").unwrap();
    let repl = [NonNullAscii::SmallX];
    s.replace_range(1..1, &repl).unwrap();
    assert_eq!(&*s, "axbc");
}

// ---------------------------------------------------------------------------
// remove_range
// ---------------------------------------------------------------------------

#[test]
fn remove_range_middle() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("0123456789").unwrap();
    s.remove_range(2..4).unwrap();
    assert_eq!(&*s, "01456789");
}

#[test]
fn remove_range_head() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("0123456789").unwrap();
    s.remove_range(..4).unwrap();
    assert_eq!(&*s, "456789");
}

#[test]
fn remove_range_tail() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("0123456789").unwrap();
    s.remove_range(7..).unwrap();
    assert_eq!(&*s, "0123456");
}

#[test]
fn remove_range_all() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("hello").unwrap();
    s.remove_range(..).unwrap();
    assert!(s.is_empty());
    assert_eq!(&*s, "");
}

#[test]
fn remove_range_empty_noop() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("hello").unwrap();
    s.remove_range(2..2).unwrap();
    assert_eq!(&*s, "hello");
}

#[test]
fn remove_range_out_of_bounds() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abcde").unwrap();
    let err = s.remove_range(2..6);
    assert_eq!(err, Err(RemoveRangeError));
}

#[test]
fn remove_range_swapped() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abcde").unwrap();
    #[allow(clippy::reversed_empty_ranges)]
    let err = s.remove_range(3..2);
    assert_eq!(err, Err(RemoveRangeError));
}

#[test]
fn remove_range_single() {
    let mut s: TinyAscii32 = TinyAscii32::try_new("abcd").unwrap();
    s.remove_range(1..2).unwrap();
    assert_eq!(&*s, "acd");
}

#[test]
fn remove_range_preserves_capacity() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
    s.remove_range(1..3).unwrap();
    assert_eq!(s.capacity(), 8);
}

// ---------------------------------------------------------------------------
// remove (single)
// ---------------------------------------------------------------------------

#[test]
fn remove_head() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    assert_eq!(s.remove(0), Some(NonNullAscii::SmallA));
    assert_eq!(&*s, "bc");
}

#[test]
fn remove_middle() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    assert_eq!(s.remove(1), Some(NonNullAscii::SmallB));
    assert_eq!(&*s, "ac");
}

#[test]
fn remove_tail() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    assert_eq!(s.remove(2), Some(NonNullAscii::SmallC));
    assert_eq!(&s, "ab");
}

#[test]
fn remove_empty_returns_none() {
    let mut s: TinyAscii4 = TinyAscii4::empty();
    assert_eq!(s.remove(0), None);
}

#[test]
fn remove_out_of_bounds_returns_none() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    assert_eq!(s.remove(3), None);
    assert_eq!(&s, "abc");
}

#[test]
fn remove_index_at_length_boundary() {
    let mut s = TinyAscii32::from_ascii_array([NonNullAscii::CapitalA; 32]);

    for _ in 0..32 {
        let _ = s.remove(2);
    }

    assert_eq!(s.len(), 2);
}

#[test]
fn remove_many() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abcde").unwrap();
    assert_eq!(s.remove(0), Some(NonNullAscii::SmallA));
    assert_eq!(s.remove(0), Some(NonNullAscii::SmallB));
    assert_eq!(&*s, "cde");
}

// ---------------------------------------------------------------------------
// pop
// ---------------------------------------------------------------------------

#[test]
fn pop_single() {
    let mut s: TinyAscii4 = TinyAscii4::try_new("a").unwrap();
    assert_eq!(s.pop(), Some(NonNullAscii::SmallA));
    assert!(s.is_empty());
}

#[test]
fn pop_multiple() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    assert_eq!(s.pop(), Some(NonNullAscii::SmallC));
    assert_eq!(s.pop(), Some(NonNullAscii::SmallB));
    assert_eq!(s.pop(), Some(NonNullAscii::SmallA));
    assert_eq!(s.pop(), None);
}

#[test]
fn pop_empty_returns_none() {
    let mut s: TinyAscii4 = TinyAscii4::empty();
    assert_eq!(s.pop(), None);
}

#[test]
fn pop_preserves_capacity() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("hello").unwrap();
    s.pop();
    assert_eq!(s.capacity(), 8);
}

// ---------------------------------------------------------------------------
// retain
// ---------------------------------------------------------------------------

#[test]
fn retain_keep_all() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    s.retain(|_| true);
    assert_eq!(&*s, "abc");
}

#[test]
fn retain_drop_all() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("abc").unwrap();
    s.retain(|_| false);
    assert!(s.is_empty());
}

#[test]
fn retain_filter() {
    let mut s: TinyAscii16 = TinyAscii16::try_new("a1b2c3").unwrap();
    s.retain(NonNullAscii::is_lowercase);
    assert_eq!(&*s, "abc");
}

#[test]
fn retain_preserves_order() {
    let mut s: TinyAscii16 = TinyAscii16::try_new("cbaabc").unwrap();
    s.retain(|c| c != NonNullAscii::SmallA);
    assert_eq!(&*s, "cbbc");
}

#[test]
fn retain_empty() {
    let mut s: TinyAscii4 = TinyAscii4::empty();
    s.retain(|_| true);
    assert!(s.is_empty());
}

#[test]
fn retain_all_removed() {
    let mut s: TinyAscii8 = TinyAscii8::try_new("123").unwrap();
    s.retain(NonNullAscii::is_digit);
    assert_eq!(&*s, "123");
    s.retain(NonNullAscii::is_lowercase);
    assert!(s.is_empty());
}

#[test]
fn retain_control_chars() {
    let bytes = [b'a', 0x07, b'b', 0x0A, b'c'];
    let mut s = unsafe { TinyAscii::from_parts(bytes, 5) };
    s.retain(|c| c != NonNullAscii::LineFeed);
    assert_eq!(&*s, "a\x07bc");
}
