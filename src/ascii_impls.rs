use core::fmt::{self, Write};

use crate::ascii::{Ascii, NonNullAscii};

impl fmt::Display for Ascii {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(self.to_char())
    }
}
impl fmt::Display for NonNullAscii {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(self.to_char())
    }
}

impl fmt::Debug for Ascii {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(self.to_char())
    }
}
impl fmt::Debug for NonNullAscii {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char(self.to_char())
    }
}

impl AsRef<str> for Ascii {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl AsRef<str> for NonNullAscii {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<Ascii> for char {
    fn from(value: Ascii) -> Self {
        value.to_char()
    }
}
impl From<NonNullAscii> for char {
    fn from(value: NonNullAscii) -> Self {
        value.to_char()
    }
}

impl From<Ascii> for u8 {
    fn from(value: Ascii) -> Self {
        value.to_u8()
    }
}
impl From<NonNullAscii> for u8 {
    fn from(value: NonNullAscii) -> Self {
        value.to_u8()
    }
}

impl From<Ascii> for u16 {
    fn from(value: Ascii) -> Self {
        u16::from(value.to_u8())
    }
}
impl From<NonNullAscii> for u16 {
    fn from(value: NonNullAscii) -> Self {
        u16::from(value.to_u8())
    }
}

impl From<Ascii> for u32 {
    fn from(value: Ascii) -> Self {
        u32::from(value.to_u8())
    }
}
impl From<NonNullAscii> for u32 {
    fn from(value: NonNullAscii) -> Self {
        u32::from(value.to_u8())
    }
}

impl From<Ascii> for u64 {
    fn from(value: Ascii) -> Self {
        u64::from(value.to_u8())
    }
}
impl From<NonNullAscii> for u64 {
    fn from(value: NonNullAscii) -> Self {
        u64::from(value.to_u8())
    }
}

impl From<Ascii> for u128 {
    fn from(value: Ascii) -> Self {
        u128::from(value.to_u8())
    }
}
impl From<NonNullAscii> for u128 {
    fn from(value: NonNullAscii) -> Self {
        u128::from(value.to_u8())
    }
}

impl PartialEq<Ascii> for char {
    fn eq(&self, other: &Ascii) -> bool {
        self.eq(&other.to_char())
    }
}
impl PartialEq<NonNullAscii> for char {
    fn eq(&self, other: &NonNullAscii) -> bool {
        self.eq(&other.to_char())
    }
}

impl PartialEq<Ascii> for u8 {
    fn eq(&self, other: &Ascii) -> bool {
        self.eq(&other.to_u8())
    }
}
impl PartialEq<NonNullAscii> for u8 {
    fn eq(&self, other: &NonNullAscii) -> bool {
        self.eq(&other.to_u8())
    }
}

impl PartialEq<char> for Ascii {
    fn eq(&self, other: &char) -> bool {
        self.to_char().eq(other)
    }
}
impl PartialEq<char> for NonNullAscii {
    fn eq(&self, other: &char) -> bool {
        self.to_char().eq(other)
    }
}

impl PartialEq<u8> for Ascii {
    fn eq(&self, other: &u8) -> bool {
        self.to_u8().eq(other)
    }
}
impl PartialEq<u8> for NonNullAscii {
    fn eq(&self, other: &u8) -> bool {
        self.to_u8().eq(other)
    }
}
