use ascii::AsciiChar;

use crate::ascii::Ascii;

impl From<AsciiChar> for Ascii {
    fn from(value: AsciiChar) -> Self {
        // SAFETY: `AsciiChar` guarantees the same representation as `Ascii`
        // - `u8` layout
        // - `0..128` pattern
        unsafe { core::mem::transmute::<AsciiChar, Ascii>(value) }
    }
}

impl From<Ascii> for AsciiChar {
    fn from(value: Ascii) -> Self {
        // SAFETY: `Ascii` guarantees the same representation as `AsciiChar`
        // - `u8` layout
        // - `0..128` pattern
        unsafe { core::mem::transmute::<Ascii, AsciiChar>(value) }
    }
}

impl PartialEq<AsciiChar> for Ascii {
    fn eq(&self, other: &AsciiChar) -> bool {
        self.to_u8().eq(&other.as_byte())
    }
}

impl PartialEq<Ascii> for AsciiChar {
    fn eq(&self, other: &Ascii) -> bool {
        self.as_byte().eq(&other.to_u8())
    }
}
