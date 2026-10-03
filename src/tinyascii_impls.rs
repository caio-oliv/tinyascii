use core::{
    borrow::{Borrow, BorrowMut},
    fmt,
    hash::Hash,
    ops::Deref,
    str::FromStr,
};

use crate::{ascii::NonNullAscii, error::WriteAsciiError, tinyascii::TinyAscii};

// This impl should be on `type Target = [NonNullAscii]`, but
// ASCII slice does not have the same support as `&str`.
// Maybe when std has a stable `&[ascii]` type.
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

impl<const N: usize> Borrow<[NonNullAscii]> for TinyAscii<N> {
    fn borrow(&self) -> &[NonNullAscii] {
        self.as_ascii()
    }
}

impl<const N: usize> Borrow<str> for TinyAscii<N> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> BorrowMut<[NonNullAscii]> for TinyAscii<N> {
    fn borrow_mut(&mut self) -> &mut [NonNullAscii] {
        self.as_mut_ascii()
    }
}

impl<const N: usize> Default for TinyAscii<N> {
    fn default() -> Self {
        Self::empty()
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
        Self::try_new(s)
    }
}

impl<'a, const N: usize> TryFrom<&'a str> for TinyAscii<N> {
    type Error = WriteAsciiError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::try_new(value)
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

impl<const N: usize> From<TinyAscii<N>> for [u8; N] {
    fn from(value: TinyAscii<N>) -> Self {
        value.into_parts().0
    }
}

impl<const N: usize> Hash for TinyAscii<N> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_bytes().hash(state);
    }
}
