use tinystr::{ParseError, TinyAsciiStr};

use crate::error::{CapacityLimitError, WriteAsciiError};
use crate::tinyascii::TinyAscii;

impl<const N: usize> From<TinyAsciiStr<N>> for TinyAscii<N> {
    fn from(value: TinyAsciiStr<N>) -> Self {
        let len = value.len();

        // SAFETY: `TinyAsciiStr<N>` representation ensure that:
        // - `len` is less than or equal `N`
        // - first length bytes are valid `NonNullAscii` characters.
        unsafe { TinyAscii::<N>::from_parts(*value.all_bytes(), len) }
    }
}

impl<const N: usize> From<TinyAscii<N>> for TinyAsciiStr<N> {
    fn from(value: TinyAscii<N>) -> Self {
        let (mut buf, len) = value.into_parts();
        buf[len..].fill(0);

        // SAFETY: buf array is made of valid non-null ASCII bytes
        // terminated by null ('\0').
        unsafe { Self::from_utf8_unchecked(buf) }
    }
}

impl From<ParseError> for WriteAsciiError {
    fn from(value: ParseError) -> Self {
        match value {
            ParseError::TooLong { .. } => Self::Capacity(CapacityLimitError),
            _ => Self::InvalidAscii,
        }
    }
}
