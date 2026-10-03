use crate::ascii::{Ascii, NonNullAscii};

#[test]
fn display_ascii_as_char() {
    let null = Ascii::Null.to_string();
    assert_eq!(null, '\0'.to_string());

    let esc = Ascii::Escape.to_string();
    assert_eq!(esc, char::from(0x1B).to_string());

    let esc = Ascii::CommercialAt.to_string();
    assert_eq!(esc, '@'.to_string());

    let esc = NonNullAscii::Escape.to_string();
    assert_eq!(esc, char::from(0x1B).to_string());

    let esc = NonNullAscii::CommercialAt.to_string();
    assert_eq!(esc, '@'.to_string());
}

#[test]
fn ascii_as_ref_str() {
    assert_eq!(Ascii::CapitalJ.as_ref(), "J");
    assert_eq!(Ascii::SmallF.as_ref(), "f");
    assert_eq!(Ascii::Space.as_ref(), " ");
    assert_eq!(Ascii::DollarSign.as_ref(), "$");

    assert_eq!(NonNullAscii::CapitalJ.as_ref(), "J");
    assert_eq!(NonNullAscii::SmallF.as_ref(), "f");
    assert_eq!(NonNullAscii::Space.as_ref(), " ");
    assert_eq!(NonNullAscii::DollarSign.as_ref(), "$");
}

#[test]
fn conversions_char_from_ascii() {
    assert_eq!(char::from(Ascii::Ampersand), '&');
    assert_eq!(char::from(Ascii::NumberSign), '#');
    assert_eq!(char::from(Ascii::FullStop), '.');
    assert_eq!(char::from(Ascii::Digit8), '8');

    assert_eq!(char::from(NonNullAscii::Ampersand), '&');
    assert_eq!(char::from(NonNullAscii::NumberSign), '#');
    assert_eq!(char::from(NonNullAscii::FullStop), '.');
    assert_eq!(char::from(NonNullAscii::Digit8), '8');
}

#[test]
fn conversions_num_from_ascii() {
    assert_eq!(u8::from(Ascii::Null), 0);
    assert_eq!(u8::from(Ascii::Bell), 7);
    assert_eq!(u8::from(Ascii::Cancel), 24);
    assert_eq!(u8::from(Ascii::Delete), 127);

    assert_eq!(u8::from(NonNullAscii::LeftCurlyBracket), 123);
    assert_eq!(u8::from(NonNullAscii::SmallA), 97);
    assert_eq!(u8::from(NonNullAscii::EqualsSign), 61);
    assert_eq!(u8::from(NonNullAscii::LineFeed), 10);

    assert_eq!(u16::from(Ascii::Null), 0);
    assert_eq!(u16::from(Ascii::Bell), 7);
    assert_eq!(u16::from(Ascii::Cancel), 24);
    assert_eq!(u16::from(Ascii::Delete), 127);

    assert_eq!(u16::from(NonNullAscii::LeftCurlyBracket), 123);
    assert_eq!(u16::from(NonNullAscii::SmallA), 97);
    assert_eq!(u16::from(NonNullAscii::EqualsSign), 61);
    assert_eq!(u16::from(NonNullAscii::LineFeed), 10);

    assert_eq!(u32::from(Ascii::Null), 0);
    assert_eq!(u32::from(Ascii::Bell), 7);
    assert_eq!(u32::from(Ascii::Cancel), 24);
    assert_eq!(u32::from(Ascii::Delete), 127);

    assert_eq!(u32::from(NonNullAscii::LeftCurlyBracket), 123);
    assert_eq!(u32::from(NonNullAscii::SmallA), 97);
    assert_eq!(u32::from(NonNullAscii::EqualsSign), 61);
    assert_eq!(u32::from(NonNullAscii::LineFeed), 10);

    assert_eq!(u64::from(Ascii::Null), 0);
    assert_eq!(u64::from(Ascii::Bell), 7);
    assert_eq!(u64::from(Ascii::Cancel), 24);
    assert_eq!(u64::from(Ascii::Delete), 127);

    assert_eq!(u64::from(NonNullAscii::LeftCurlyBracket), 123);
    assert_eq!(u64::from(NonNullAscii::SmallA), 97);
    assert_eq!(u64::from(NonNullAscii::EqualsSign), 61);
    assert_eq!(u64::from(NonNullAscii::LineFeed), 10);

    assert_eq!(u128::from(Ascii::Null), 0);
    assert_eq!(u128::from(Ascii::Bell), 7);
    assert_eq!(u128::from(Ascii::Cancel), 24);
    assert_eq!(u128::from(Ascii::Delete), 127);

    assert_eq!(u128::from(NonNullAscii::LeftCurlyBracket), 123);
    assert_eq!(u128::from(NonNullAscii::SmallA), 97);
    assert_eq!(u128::from(NonNullAscii::EqualsSign), 61);
    assert_eq!(u128::from(NonNullAscii::LineFeed), 10);
}

#[test]
fn comparisons_eq_ascii() {
    assert_eq!(Ascii::Asterisk, '*');
    assert_eq!(Ascii::LowLine, '_');
    assert_eq!(Ascii::Digit9, '9');

    assert_eq!(NonNullAscii::Asterisk, '*');
    assert_eq!(NonNullAscii::LowLine, '_');
    assert_eq!(NonNullAscii::Digit9, '9');

    assert_eq!('*', Ascii::Asterisk);
    assert_eq!('_', Ascii::LowLine);
    assert_eq!('9', Ascii::Digit9);

    assert_eq!('*', NonNullAscii::Asterisk);
    assert_eq!('_', NonNullAscii::LowLine);
    assert_eq!('9', NonNullAscii::Digit9);

    assert_eq!(Ascii::Asterisk, b'*');
    assert_eq!(Ascii::LowLine, b'_');
    assert_eq!(Ascii::Digit9, b'9');

    assert_eq!(NonNullAscii::Asterisk, b'*');
    assert_eq!(NonNullAscii::LowLine, b'_');
    assert_eq!(NonNullAscii::Digit9, b'9');

    assert_eq!(b'*', Ascii::Asterisk);
    assert_eq!(b'_', Ascii::LowLine);
    assert_eq!(b'9', Ascii::Digit9);

    assert_eq!(b'*', NonNullAscii::Asterisk);
    assert_eq!(b'_', NonNullAscii::LowLine);
    assert_eq!(b'9', NonNullAscii::Digit9);
}
