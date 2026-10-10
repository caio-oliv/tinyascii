#![allow(clippy::unwrap_used)]

use tinystr::{ParseError, TinyAsciiStr};

use crate::error::{CapacityLimitError, WriteAsciiError};
use crate::tinyascii::TinyAscii;

#[test]
fn tinyascii_to_tinyascii_str_empty() {
    let tinystr: TinyAsciiStr<8> = TinyAsciiStr::try_from_str("").unwrap();
    let tinyascii: TinyAscii<8> = TinyAscii::from(tinystr);
    assert_eq!(tinyascii.len(), 0);
    assert!(tinyascii.is_empty());
    assert_eq!(&tinyascii, "");

    // Round-trip back
    let back: TinyAsciiStr<8> = TinyAsciiStr::from(tinyascii);
    assert_eq!(back.as_str(), "");
}

#[test]
fn tinyascii_to_tinyascii_str_simple() {
    let tinystr: TinyAsciiStr<8> = TinyAsciiStr::try_from_str("hello").unwrap();
    let tinyascii: TinyAscii<8> = TinyAscii::from(tinystr);
    assert_eq!(&tinyascii, "hello");
    assert_eq!(tinyascii.len(), 5);
}

#[test]
fn tinyascii_to_tinyascii_str_max_len() {
    // Fill capacity exactly (N-1 chars for TinyAscii)
    let s: String = "abcdefgh".to_string();
    let tinystr: TinyAsciiStr<8> = TinyAsciiStr::try_from_str(&s).unwrap();
    let tinyascii: TinyAscii<8> = TinyAscii::from(tinystr);
    assert_eq!(&tinyascii, "abcdefgh");
    assert_eq!(tinyascii.len(), 8);

    // Round-trip
    let back: TinyAsciiStr<8> = TinyAsciiStr::from(tinyascii);
    assert_eq!(back.as_str(), "abcdefgh");
}

#[test]
fn tinyascii_to_tinyascii_str_control_chars() {
    // Control characters are valid in TinyAsciiStr (1..=127)
    let tinystr: TinyAsciiStr<16> = TinyAsciiStr::try_from_str("\x01\x02\x03\x04").unwrap();
    let tinyascii: TinyAscii<16> = TinyAscii::from(tinystr);
    assert_eq!(tinyascii.len(), 4);
    assert_eq!(tinyascii.as_bytes(), &[1, 2, 3, 4]);
}

#[test]
fn tinyascii_to_tinyascii_str_all_ascii_chars() {
    // Test a representative sample of all ASCII categories
    let chars = [
        '\x01', // StartOfHeading (control)
        '\t',   // CharacterTabulation
        '\n',   // LineFeed
        ' ',    // Space
        '0',    // Digit
        'A',    // Uppercase
        'a',    // Lowercase
        '~',    // Tilde
        '\x7F', // Delete
    ];
    let s: String = chars.iter().collect();
    let tinystr: TinyAsciiStr<16> = TinyAsciiStr::try_from_str(&s).unwrap();
    let tinyascii: TinyAscii<16> = TinyAscii::from(tinystr);
    assert_eq!(tinyascii.as_bytes(), s.as_bytes());

    // Round-trip
    let back: TinyAsciiStr<16> = TinyAsciiStr::from(tinyascii);
    assert_eq!(back.as_str(), s);
}

#[test]
fn tinyascii_to_tinyascii_str_sizes_4() {
    let s = "a";
    let tinystr: TinyAsciiStr<4> = TinyAsciiStr::try_from_str(s).unwrap();
    let tinyascii: TinyAscii<4> = TinyAscii::from(tinystr);
    assert_eq!(&tinyascii, s);

    let back: TinyAsciiStr<4> = TinyAsciiStr::from(tinyascii);
    assert_eq!(back.as_str(), s);
}

#[test]
fn tinyascii_to_tinyascii_str_sizes_8() {
    let s = "test";
    let tinystr: TinyAsciiStr<8> = TinyAsciiStr::try_from_str(s).unwrap();
    let tinyascii: TinyAscii<8> = TinyAscii::from(tinystr);
    assert_eq!(&tinyascii, s);

    let back: TinyAsciiStr<8> = TinyAsciiStr::from(tinyascii);
    assert_eq!(back.as_str(), s);
}

#[test]
fn tinyascii_to_tinyascii_str_sizes_16() {
    let s = "medium string";
    let tinystr: TinyAsciiStr<16> = TinyAsciiStr::try_from_str(s).unwrap();
    let tinyascii: TinyAscii<16> = TinyAscii::from(tinystr);
    assert_eq!(&tinyascii, s);

    let back: TinyAsciiStr<16> = TinyAsciiStr::from(tinyascii);
    assert_eq!(back.as_str(), s);
}

#[test]
fn tinyascii_to_tinyascii_str_sizes_32() {
    let s = "a large string for testing";
    let tinystr: TinyAsciiStr<32> = TinyAsciiStr::try_from_str(s).unwrap();
    let tinyascii: TinyAscii<32> = TinyAscii::from(tinystr);
    assert_eq!(&tinyascii, s);

    let back: TinyAsciiStr<32> = TinyAsciiStr::from(tinyascii);
    assert_eq!(back.as_str(), s);
}

#[test]
fn tinyascii_to_tinyascii_str_sizes_64() {
    let s = "a very large string for testing purposes";
    let tinystr: TinyAsciiStr<64> = TinyAsciiStr::try_from_str(s).unwrap();
    let tinyascii: TinyAscii<64> = TinyAscii::from(tinystr);
    assert_eq!(&tinyascii, s);

    let back: TinyAsciiStr<64> = TinyAsciiStr::from(tinyascii);
    assert_eq!(back.as_str(), s);
}

#[test]
fn tinyascii_to_tinyascii_str_sizes_128() {
    let s = "a very large string for testing purposes that fills sixty-four bytes";
    let tinystr: TinyAsciiStr<128> = TinyAsciiStr::try_from_str(s).unwrap();
    let tinyascii: TinyAscii<128> = TinyAscii::from(tinystr);
    assert_eq!(&tinyascii, s);

    let back: TinyAsciiStr<128> = TinyAsciiStr::from(tinyascii);
    assert_eq!(back.as_str(), s);
}

#[test]
fn tinyascii_to_tinyascii_str_from_ascii_array_roundtrip() {
    let expected = "test";
    let tinystr: TinyAsciiStr<8> = TinyAsciiStr::try_from_str(expected).unwrap();
    let tinyascii: TinyAscii<8> = TinyAscii::from(tinystr);

    // Verify we can reconstruct the original TinyAsciiStr
    let roundtrip: TinyAsciiStr<8> = TinyAsciiStr::from(tinyascii);
    assert_eq!(roundtrip.as_str(), expected);
    assert_eq!(roundtrip.len(), expected.len());
}

// ── From<TinyAscii<N>> for TinyAsciiStr<N> ───────────────────────────────

#[test]
fn tinyascii_str_from_tinyascii_empty() {
    let tinyascii = TinyAscii::<8>::empty();
    let tinystr: TinyAsciiStr<8> = TinyAsciiStr::from(tinyascii);
    assert_eq!(tinystr.as_str(), "");
    assert_eq!(tinystr.len(), 0);
}

#[test]
fn tinyascii_str_from_tinyascii_full_capacity() {
    let tinyascii = TinyAscii::<4>::try_new("abcd").unwrap();
    let tinystr: TinyAsciiStr<4> = TinyAsciiStr::from(tinyascii);
    assert_eq!(tinystr.as_str(), "abcd");
    assert_eq!(tinystr.len(), 4);
}

#[test]
fn tinyascii_str_from_tinyascii_partial() {
    let tinyascii = TinyAscii::<8>::try_new("abc").unwrap();
    let tinystr: TinyAsciiStr<8> = TinyAsciiStr::from(tinyascii);
    assert_eq!(tinystr.as_str(), "abc");
    assert_eq!(tinystr.len(), 3);
}

#[test]
fn tinyascii_str_from_tinyascii_roundtrip() {
    let original = TinyAscii::<16>::try_new("roundtrip test").unwrap();
    let expected = original.as_str().to_owned();
    let tinystr: TinyAsciiStr<16> = TinyAsciiStr::from(original);
    let back: TinyAscii<16> = TinyAscii::from(tinystr);

    assert_eq!(back.as_str(), expected);
    assert_eq!(back.len(), expected.len());
}

#[test]
fn tinyascii_str_from_tinyascii_preserves_all_bytes() {
    let tinyascii = TinyAscii::<8>::try_new("12345678").unwrap();
    let tinystr: TinyAsciiStr<8> = TinyAsciiStr::from(tinyascii);
    // TinyAsciiStr stores all N bytes; verify content
    assert_eq!(tinystr.len(), 8);
    assert_eq!(tinystr.as_str(), "12345678");
}

// ── From<ParseError> for WriteAsciiError ─────────────────────────────────

#[test]
fn parse_error_too_long_maps_to_capacity() {
    let err = ParseError::TooLong { max: 4, len: 5 };
    let write_err: WriteAsciiError = WriteAsciiError::from(err);
    assert!(matches!(write_err, WriteAsciiError::Capacity(_)));
    assert_eq!(write_err, WriteAsciiError::Capacity(CapacityLimitError));
}

#[test]
fn parse_error_contains_null_maps_to_invalid() {
    let err = ParseError::ContainsNull;
    let write_err: WriteAsciiError = WriteAsciiError::from(err);
    assert!(matches!(write_err, WriteAsciiError::InvalidAscii));
    assert_eq!(write_err, WriteAsciiError::InvalidAscii);
}

#[test]
fn parse_error_non_ascii_maps_to_invalid() {
    let err = ParseError::NonAscii;
    let write_err: WriteAsciiError = WriteAsciiError::from(err);
    assert!(matches!(write_err, WriteAsciiError::InvalidAscii));
    assert_eq!(write_err, WriteAsciiError::InvalidAscii);
}

#[test]
fn parse_error_variants_exhaustive() {
    // Ensure all ParseError variants are handled
    let too_long = ParseError::TooLong { max: 1, len: 2 };
    let contains_null = ParseError::ContainsNull;
    let non_ascii = ParseError::NonAscii;

    match WriteAsciiError::from(too_long) {
        WriteAsciiError::Capacity(_) => {}
        WriteAsciiError::InvalidAscii => panic!("expected Capacity"),
    }

    match WriteAsciiError::from(contains_null) {
        WriteAsciiError::Capacity(_) => panic!("expected InvalidAscii"),
        WriteAsciiError::InvalidAscii => {}
    }

    match WriteAsciiError::from(non_ascii) {
        WriteAsciiError::Capacity(_) => panic!("expected InvalidAscii"),
        WriteAsciiError::InvalidAscii => {}
    }
}

// ── Cross-feature round-trips ────────────────────────────────────────────

#[test]
fn full_tinystr_roundtrip() {
    let original = "full roundtrip";
    let tinystr: TinyAsciiStr<32> = TinyAsciiStr::try_from_str(original).unwrap();
    let tinyascii: TinyAscii<32> = TinyAscii::from(tinystr);
    let back: TinyAsciiStr<32> = TinyAsciiStr::from(tinyascii);

    assert_eq!(back.as_str(), original);
    assert_eq!(back.len(), original.len());
}

#[test]
fn tinystr_special_chars_roundtrip() {
    let special = "!@#$%^&*()_+-=[]{}|;':\",./<>?";
    let tinystr: TinyAsciiStr<32> = TinyAsciiStr::try_from_str(special).unwrap();
    let tinyascii: TinyAscii<32> = TinyAscii::from(tinystr);
    let back: TinyAsciiStr<32> = TinyAsciiStr::from(tinyascii);

    assert_eq!(back.as_str(), special);
}

// ── Edge cases ───────────────────────────────────────────────────────────

#[test]
fn tinyascii_to_tinyascii_str_trailing_null_in_tinystr() {
    // TinyAsciiStr stores trailing nulls for padding; TinyAscii handles
    // the length separately, so the roundtrip should be clean.
    let tinystr: TinyAsciiStr<8> = TinyAsciiStr::try_from_str("abc").unwrap();
    let tinyascii: TinyAscii<8> = TinyAscii::from(tinystr);
    assert_eq!(tinyascii.len(), 3);

    let back: TinyAsciiStr<8> = TinyAsciiStr::from(tinyascii);
    assert_eq!(back.as_str(), "abc");
}

#[test]
fn tinyascii_str_from_empty_then_fill() {
    let tinyascii = TinyAscii::<8>::empty();
    let tinystr: TinyAsciiStr<8> = TinyAsciiStr::from(tinyascii);
    assert!(tinystr.as_str().is_empty());

    // Convert back to TinyAscii and fill
    let mut ta: TinyAscii<8> = TinyAscii::from(tinystr);
    ta.try_push_str("filled").unwrap();
    let filled: TinyAsciiStr<8> = TinyAsciiStr::from(ta);
    assert_eq!(filled.as_str(), "filled");
}
