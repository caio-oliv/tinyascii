use core::{
    borrow::Borrow,
    fmt,
    hash::Hash,
    ops::Deref,
    str::FromStr,
};

use crate::{
    ascii::NonNullAscii,
    error::{CapacityLimitError, WriteAsciiError},
    tinyascii::TinyAscii,
};

// `Target = str` (not `[NonNullAscii]`) so callers get the full `str` API.
// The ASCII-slice view is available via `as_ascii()`. Revisit if
// `[core::ascii::Char]` stabilizes with comparable string-like support.
impl<const N: usize> Deref for TinyAscii<N> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<const N: usize> AsRef<str> for TinyAscii<N> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> AsRef<[NonNullAscii]> for TinyAscii<N> {
    fn as_ref(&self) -> &[NonNullAscii] {
        self.as_ascii()
    }
}

impl<const N: usize> AsRef<[u8]> for TinyAscii<N> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(feature = "std")]
impl<const N: usize> AsRef<std::path::Path> for TinyAscii<N> {
    fn as_ref(&self) -> &std::path::Path {
        std::path::Path::new(self.as_str())
    }
}

#[cfg(feature = "std")]
impl<const N: usize> AsRef<std::ffi::OsStr> for TinyAscii<N> {
    fn as_ref(&self) -> &std::ffi::OsStr {
        std::ffi::OsStr::new(self.as_str())
    }
}

impl<const N: usize> AsMut<[NonNullAscii]> for TinyAscii<N> {
    fn as_mut(&mut self) -> &mut [NonNullAscii] {
        self.as_mut_ascii()
    }
}

impl<const N: usize> Borrow<str> for TinyAscii<N> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> Default for TinyAscii<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Display for TinyAscii<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<const N: usize> fmt::Debug for TinyAscii<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl<const N: usize> Eq for TinyAscii<N> {}

impl<const N: usize> PartialEq for TinyAscii<N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes().eq(other.as_bytes())
    }
}

impl<const N: usize> PartialEq<str> for TinyAscii<N> {
    fn eq(&self, other: &str) -> bool {
        self.as_bytes().eq(other.as_bytes())
    }
}

impl<const N: usize> PartialEq<TinyAscii<N>> for str {
    fn eq(&self, other: &TinyAscii<N>) -> bool {
        self.as_bytes().eq(other.as_bytes())
    }
}

impl<const N: usize> PartialEq<[NonNullAscii]> for TinyAscii<N> {
    fn eq(&self, other: &[NonNullAscii]) -> bool {
        self.as_ascii().eq(other)
    }
}

impl<const N: usize> PartialEq<TinyAscii<N>> for [NonNullAscii] {
    fn eq(&self, other: &TinyAscii<N>) -> bool {
        self.eq(other.as_ascii())
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> PartialEq<alloc::borrow::Cow<'_, str>> for TinyAscii<N> {
    fn eq(&self, other: &alloc::borrow::Cow<'_, str>) -> bool {
        self.as_str().eq(other.as_ref())
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> PartialEq<TinyAscii<N>> for alloc::borrow::Cow<'_, str> {
    fn eq(&self, other: &TinyAscii<N>) -> bool {
        self.as_ref().eq(other.as_str())
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> PartialEq<alloc::string::String> for TinyAscii<N> {
    fn eq(&self, other: &alloc::string::String) -> bool {
        self.as_str().eq(other)
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> PartialEq<TinyAscii<N>> for alloc::string::String {
    fn eq(&self, other: &TinyAscii<N>) -> bool {
        self.eq(other.as_str())
    }
}

#[cfg(feature = "std")]
impl<const N: usize> PartialEq<std::path::Path> for TinyAscii<N> {
    fn eq(&self, other: &std::path::Path) -> bool {
        self.as_str().eq(other)
    }
}

#[cfg(feature = "std")]
impl<const N: usize> PartialEq<TinyAscii<N>> for std::path::Path {
    fn eq(&self, other: &TinyAscii<N>) -> bool {
        self.eq(other.as_str())
    }
}

#[cfg(feature = "std")]
impl<const N: usize> PartialEq<std::path::PathBuf> for TinyAscii<N> {
    fn eq(&self, other: &std::path::PathBuf) -> bool {
        self.as_str().eq(other)
    }
}

#[cfg(feature = "std")]
impl<const N: usize> PartialEq<TinyAscii<N>> for std::path::PathBuf {
    fn eq(&self, other: &TinyAscii<N>) -> bool {
        self.eq(other.as_str())
    }
}

#[cfg(feature = "std")]
impl<const N: usize> PartialEq<std::ffi::OsStr> for TinyAscii<N> {
    fn eq(&self, other: &std::ffi::OsStr) -> bool {
        self.as_str().eq(other)
    }
}

#[cfg(feature = "std")]
impl<const N: usize> PartialEq<TinyAscii<N>> for std::ffi::OsStr {
    fn eq(&self, other: &TinyAscii<N>) -> bool {
        self.eq(other.as_str())
    }
}

impl<const N: usize> Ord for TinyAscii<N> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_ascii().cmp(other.as_ascii())
    }
}

impl<const N: usize> PartialOrd for TinyAscii<N> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> FromStr for TinyAscii<N> {
    type Err = WriteAsciiError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}

impl<'a, const N: usize> TryFrom<&'a str> for TinyAscii<N> {
    type Error = WriteAsciiError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::try_from_bytes(value.as_bytes())
    }
}

impl<'a, const N: usize> TryFrom<&'a [u8]> for TinyAscii<N> {
    type Error = WriteAsciiError;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        Self::try_from_bytes(value)
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> From<&'a TinyAscii<N>> for alloc::borrow::Cow<'a, str> {
    fn from(s: &'a TinyAscii<N>) -> alloc::borrow::Cow<'a, str> {
        alloc::borrow::Cow::Borrowed(s.as_str())
    }
}

impl<'a, const N: usize> TryFrom<&'a [NonNullAscii]> for TinyAscii<N> {
    type Error = CapacityLimitError;

    fn try_from(value: &'a [NonNullAscii]) -> Result<Self, Self::Error> {
        let mut tiny = Self::new();
        tiny.try_push_slice(value)?;
        Ok(tiny)
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> TryFrom<alloc::string::String> for TinyAscii<N> {
    type Error = WriteAsciiError;

    fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
        Self::try_from_bytes(value.as_bytes())
    }
}

#[cfg(feature = "alloc")]
impl<'a, const N: usize> TryFrom<&'a alloc::string::String> for TinyAscii<N> {
    type Error = WriteAsciiError;

    fn try_from(value: &'a alloc::string::String) -> Result<Self, Self::Error> {
        Self::try_from_bytes(value.as_bytes())
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> TryFrom<alloc::vec::Vec<u8>> for TinyAscii<N> {
    type Error = WriteAsciiError;

    fn try_from(value: alloc::vec::Vec<u8>) -> Result<Self, Self::Error> {
        Self::try_from_bytes(&value)
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> From<TinyAscii<N>> for alloc::string::String {
    fn from(value: TinyAscii<N>) -> Self {
        Self::from(value.as_str())
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> From<TinyAscii<N>> for alloc::vec::Vec<u8> {
    fn from(value: TinyAscii<N>) -> Self {
        value.into_bytes()
    }
}

#[cfg(feature = "alloc")]
impl<const N: usize> From<TinyAscii<N>> for alloc::boxed::Box<str> {
    fn from(value: TinyAscii<N>) -> Self {
        value.into_boxed_str()
    }
}

/// Appends to the string, failing with [`fmt::Error`] if the content does not
/// fit in the spare capacity or is not valid non-null ASCII. A failed write
/// leaves the string unchanged.
impl<const N: usize> fmt::Write for TinyAscii<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.try_push_str(s).map_err(|_| fmt::Error)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        let byte = u8::try_from(c).map_err(|_| fmt::Error)?;
        self.try_push_u8(byte).map_err(|_| fmt::Error)
    }
}

impl<const N: usize> From<TinyAscii<N>> for [u8; N] {
    fn from(value: TinyAscii<N>) -> Self {
        value.into_parts().0
    }
}

impl<const N: usize> Hash for TinyAscii<N> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}
