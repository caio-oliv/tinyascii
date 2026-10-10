#![allow(clippy::unwrap_used)]

use ascii::AsciiChar;

use crate::ascii::{Ascii, NonNullAscii};

// ── From<AsciiChar> for Ascii ────────────────────────────────────────────

#[test]
fn asciichar_to_ascii_null() {
    let ascii = Ascii::from(AsciiChar::Null);
    assert_eq!(ascii, Ascii::Null);
    assert_eq!(ascii.to_u8(), 0);
}

#[test]
fn asciichar_to_ascii_delete() {
    let ascii = Ascii::from(AsciiChar::Tilde);
    assert_eq!(ascii, Ascii::Tilde);
    assert_eq!(ascii.to_u8(), 126);

    // Delete character (0x7F)
    let delete = AsciiChar::new('\x7F');
    let ascii: Ascii = Ascii::from(delete);
    assert_eq!(ascii, Ascii::Delete);
    assert_eq!(ascii.to_u8(), 127);
}

#[test]
fn asciichar_to_ascii_control_chars() {
    let ascii = Ascii::from(AsciiChar::SOH);
    assert_eq!(ascii, Ascii::StartOfHeading);
    assert_eq!(ascii.to_u8(), 1);

    let ascii = Ascii::from(AsciiChar::Bell);
    assert_eq!(ascii, Ascii::Bell);
    assert_eq!(ascii.to_u8(), 7);

    let ascii = Ascii::from(AsciiChar::ESC);
    assert_eq!(ascii, Ascii::Escape);
    assert_eq!(ascii.to_u8(), 27);
}

#[test]
fn asciichar_to_ascii_printable() {
    let ascii = Ascii::from(AsciiChar::Space);
    assert_eq!(ascii, Ascii::Space);
    assert_eq!(ascii.to_u8(), 32);

    let ascii = Ascii::from(AsciiChar::A);
    assert_eq!(ascii, Ascii::CapitalA);
    assert_eq!(ascii.to_u8(), 65);

    let ascii = Ascii::from(AsciiChar::a);
    assert_eq!(ascii, Ascii::SmallA);
    assert_eq!(ascii.to_u8(), 97);

    // Digit: use AsciiChar::new() since ascii crate has no Digit variants
    let ascii = Ascii::from(AsciiChar::new('0'));
    assert_eq!(ascii, Ascii::Digit0);
    assert_eq!(ascii.to_u8(), 48);
}

#[test]
fn asciichar_to_ascii_all_byte_values() {
    // Verify every byte value 0..=127 is convertible
    for byte in 0..=127 {
        let char_val = char::from(byte);
        let ascii_char = AsciiChar::new(char_val);
        let ascii: Ascii = Ascii::from(ascii_char);
        assert_eq!(ascii.to_u8(), byte, "failed for byte {}", byte);
    }
}

// ── From<Ascii> for AsciiChar ────────────────────────────────────────────

#[test]
fn ascii_to_asciichar_null() {
    let ascii_char = AsciiChar::from(Ascii::Null);
    assert_eq!(ascii_char, AsciiChar::Null);
    assert_eq!(ascii_char.as_byte(), 0);
}

#[test]
fn ascii_to_asciichar_delete() {
    let ascii_char = AsciiChar::from(Ascii::Delete);
    assert_eq!(ascii_char.as_byte(), 127);
}

#[test]
fn ascii_to_asciichar_nonnull() {
    // NonNullAscii -> Ascii -> AsciiChar
    let ascii: Ascii = NonNullAscii::SmallA.to_ascii();
    let ascii_char: AsciiChar = ascii.into();
    assert_eq!(ascii_char, AsciiChar::a);

    let ascii: Ascii = NonNullAscii::Delete.to_ascii();
    let ascii_char: AsciiChar = ascii.into();
    assert_eq!(ascii_char.as_byte(), 127);
}

#[test]
fn ascii_to_asciichar_all_values() {
    // Every Ascii value should round-trip through AsciiChar
    for byte in 0..=127 {
        let ascii = Ascii::from_u8(byte).unwrap();
        let ascii_char: AsciiChar = ascii.into();
        assert_eq!(ascii_char.as_byte(), byte, "failed for byte {}", byte);
    }
}

// ── PartialEq<AsciiChar> for Ascii ───────────────────────────────────────

#[test]
fn ascii_eq_asciichar_basic() {
    assert_eq!(Ascii::Null, AsciiChar::Null);
    assert_eq!(Ascii::CapitalA, AsciiChar::A);
    assert_eq!(Ascii::SmallA, AsciiChar::a);
    assert_eq!(Ascii::Space, AsciiChar::Space);
    assert_eq!(Ascii::Delete, AsciiChar::new('\x7F'));
}

#[test]
fn ascii_eq_asciichar_control() {
    assert_eq!(Ascii::CharacterTabulation, AsciiChar::Tab);
    assert_eq!(Ascii::LineFeed, AsciiChar::LineFeed);
    assert_eq!(Ascii::Escape, AsciiChar::ESC);
}

#[test]
fn ascii_eq_asciichar_boundary() {
    // Test boundary values
    assert_eq!(Ascii::StartOfHeading, AsciiChar::new('\x01'));
    assert_eq!(Ascii::Delete, AsciiChar::new('\x7F'));
}

// ── PartialEq<Ascii> for AsciiChar ───────────────────────────────────────

#[test]
fn asciichar_eq_ascii_basic() {
    assert_eq!(AsciiChar::Null, Ascii::Null);
    assert_eq!(AsciiChar::A, Ascii::CapitalA);
    assert_eq!(AsciiChar::a, Ascii::SmallA);
    assert_eq!(AsciiChar::Space, Ascii::Space);
    assert_eq!(AsciiChar::new('\x7F'), Ascii::Delete);
}

#[test]
fn asciichar_eq_ascii_control() {
    assert_eq!(AsciiChar::Tab, Ascii::CharacterTabulation);
    assert_eq!(AsciiChar::LineFeed, Ascii::LineFeed);
    assert_eq!(AsciiChar::ESC, Ascii::Escape);
}

#[test]
fn ascii_asciichar_eq_symmetric() {
    // Equality should be symmetric
    let test_cases = vec![
        Ascii::Null,
        Ascii::Delete,
        Ascii::CapitalA,
        Ascii::SmallZ,
        Ascii::Space,
        Ascii::Escape,
        Ascii::Digit0,
        Ascii::Tilde,
    ];

    for ascii in test_cases {
        let ascii_char: AsciiChar = ascii.into();
        assert!(
            ascii == ascii_char,
            "Ascii({:?}) should equal AsciiChar({:?})",
            ascii,
            ascii_char
        );
        assert!(
            ascii_char == ascii,
            "AsciiChar({:?}) should equal Ascii({:?})",
            ascii_char,
            ascii
        );
    }
}

// ── Round-trip conversions ───────────────────────────────────────────────

#[test]
fn ascii_to_asciichar_roundtrip() {
    for byte in 0..=127 {
        let original = Ascii::from_u8(byte).unwrap();
        let ascii_char: AsciiChar = original.into();
        let back: Ascii = ascii_char.into();
        assert_eq!(
            back, original,
            "round-trip failed for byte {} ({:?})",
            byte, original
        );
    }
}

#[test]
fn asciichar_to_ascii_roundtrip() {
    for byte in 0..=127 {
        let char_val = char::from(byte);
        let original = AsciiChar::new(char_val);
        let ascii: Ascii = Ascii::from(original);
        let back: AsciiChar = ascii.into();
        assert_eq!(back, original, "round-trip failed for byte {}", byte);
    }
}

#[test]
fn ascii_nonnull_to_asciichar_roundtrip() {
    // NonNullAscii -> Ascii -> AsciiChar -> Ascii
    for byte in 1..=127 {
        let nonnull = NonNullAscii::from_u8(byte).unwrap();
        let ascii: Ascii = nonnull.to_ascii();
        let ascii_char: AsciiChar = ascii.into();
        let back: Ascii = ascii_char.into();

        assert_eq!(back.to_u8(), byte, "failed for byte {}", byte);
    }
}

#[test]
fn ascii_to_asciichar_to_u8_identity() {
    // Ascii -> AsciiChar -> u8 should preserve the value
    for byte in 0..=127 {
        let ascii = Ascii::from_u8(byte).unwrap();
        let ascii_char: AsciiChar = ascii.into();
        assert_eq!(ascii_char.as_byte(), byte);
    }
}

#[test]
fn u8_to_ascii_to_asciichar_identity() {
    // u8 -> Ascii -> AsciiChar -> u8
    for byte in 0..=127 {
        let ascii = Ascii::from_u8(byte).unwrap();
        let ascii_char: AsciiChar = ascii.into();
        assert_eq!(byte, ascii_char.as_byte());
    }
}

// ── PartialEq edge cases ─────────────────────────────────────────────────

#[test]
fn ascii_ne_asciichar() {
    // Different values should not be equal
    assert_ne!(Ascii::CapitalA, AsciiChar::b);
    assert_ne!(Ascii::SmallA, AsciiChar::A);
    assert_ne!(Ascii::Null, AsciiChar::A);
    assert_ne!(Ascii::Delete, AsciiChar::LineFeed);
}

#[test]
fn asciichar_ne_ascii() {
    assert_ne!(AsciiChar::A, Ascii::SmallA);
    assert_ne!(AsciiChar::b, Ascii::CapitalA);
    assert_ne!(AsciiChar::Null, Ascii::Delete);
}

// ── Size consistency ─────────────────────────────────────────────────────

#[test]
fn ascii_and_asciichar_same_size() {
    assert_eq!(
        core::mem::size_of::<Ascii>(),
        core::mem::size_of::<AsciiChar>()
    );
}

#[test]
fn ascii_and_asciichar_same_alignment() {
    assert_eq!(
        core::mem::align_of::<Ascii>(),
        core::mem::align_of::<AsciiChar>()
    );
}

#[test]
fn ascii_is_u8_layout() {
    assert_eq!(core::mem::size_of::<Ascii>(), 1);
}

#[test]
fn asciichar_is_u8_layout() {
    assert_eq!(core::mem::size_of::<AsciiChar>(), 1);
}
