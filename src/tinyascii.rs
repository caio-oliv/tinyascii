use core::{
    ops::{Bound, RangeBounds},
    range::Range,
};

use crate::ascii::NonNullAscii;
use crate::error::{
    CapacityLimitError, InsertAsciiError, InsertRangeError, RemoveRangeError, WriteAsciiError,
};

#[derive(Clone)]
pub struct TinyAscii<const N: usize> {
    buf: [u8; N],
}

pub const MAX_LEN: usize = 128;

pub type TinyAscii4 = TinyAscii<4>;
pub type TinyAscii8 = TinyAscii<8>;
pub type TinyAscii16 = TinyAscii<16>;
pub type TinyAscii32 = TinyAscii<32>;
pub type TinyAscii64 = TinyAscii<64>;
pub type TinyAscii128 = TinyAscii<MAX_LEN>;

/// Static assert that [`TinyAscii`] capacity `N` is greater
/// than 0 and less than or equal 128.
///
/// # Panics
///
/// Panics if the const generic `N` is equal 0 or greater than 128.
const fn assert_capacity<const N: usize>() {
    assert!(N != 0, "TinyAscii capacity (N) must not be equal 0");
    assert!(
        N <= MAX_LEN,
        "TinyAscii capacity (N) must be less than or equal 128"
    );
}

const LEN_MASK: u8 = 0b1000_0000;

/// Encode [`TinyAscii`] length as a byte.
#[inline(always)]
const fn encode_len(len: usize) -> u8 {
    len.to_le_bytes()[0] | LEN_MASK
}

/// Validates [`TinyAscii`] valid content.
///
/// Returns a [`WriteAsciiError`] if the `src` input has bytes equal 0 or greater than 127.
const fn validate_tinyascii(src: &[u8]) -> Result<(), WriteAsciiError> {
    // TODO: optimize
    let mut i = 0;
    while i < src.len() {
        let byte = src[i];

        if byte == 0 || byte > 127 {
            return Err(WriteAsciiError::InvalidAscii);
        };

        i += 1;
    }

    Ok(())
}

/// Fill destination (`dst`) with valid bytes from source (`src`) using
/// a replacement character for invalid bytes in source.
///
/// # Safety
///
/// `dst` length must be greater or equal to `src` length (`dst.len() >= src.len()`).
///
/// Failing to uphold this invariant will cause a write out of bounds.
const unsafe fn fill_non_null_ascii(dst: &mut [u8], src: &[u8], replacement: NonNullAscii) {
    // SAFETY: `dst` length is greater or equal than `src` length
    // as ensured by the caller.
    unsafe { core::hint::assert_unchecked(dst.len() >= src.len()) };

    // TODO: optimize
    let mut i = 0;
    while i < src.len() {
        let byte = src[i];

        dst[i] = if byte == 0 || byte > 127 {
            replacement.to_u8()
        } else {
            byte
        };

        i += 1;
    }
}

impl<const N: usize> TinyAscii<N> {
    /// Creates a new empty [`TinyAscii`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii64;
    /// let s = TinyAscii64::empty();
    ///
    /// assert_eq!(&s, "");
    /// ```
    #[inline(always)]
    pub const fn empty() -> Self {
        const { assert_capacity::<N>() }

        let mut this = Self { buf: [0; N] };
        this.buf[N - 1] = LEN_MASK;
        this
    }

    /// Creates a new [`TinyAscii`] from a `&str`.
    ///
    /// The `&str` must contain only non-null ASCII characters and
    /// length less than capacity (`N`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii32;
    /// let s = TinyAscii32::try_new("Hello world!").unwrap();
    /// assert_eq!(&s, "Hello world!");
    /// ```
    ///
    /// Using invalid ASCII characters:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii32, error::WriteAsciiError};
    /// let err = TinyAscii32::try_new("invalid 😎 ascii").unwrap_err();
    /// assert_eq!(err, WriteAsciiError::InvalidAscii);
    ///
    /// let err = TinyAscii32::try_new("null present \0").unwrap_err();
    /// assert_eq!(err, WriteAsciiError::InvalidAscii);
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, error::{WriteAsciiError, CapacityLimitError}};
    /// let err = TinyAscii8::try_new("not soo tiny string").unwrap_err();
    /// assert_eq!(err, WriteAsciiError::Capacity(CapacityLimitError));
    /// ```
    #[inline(always)]
    pub const fn try_new(s: &str) -> Result<Self, WriteAsciiError> {
        const { assert_capacity::<N>() }

        Self::try_from_bytes(s.as_bytes())
    }

    /// Creates a new [`TinyAscii`] from a byte slice `&[u8]`.
    ///
    /// The byte slice must contain only non-null ASCII characters and
    /// length less than capacity (`N`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii32;
    /// let bytes = "Hello world!".as_bytes();
    /// let s = TinyAscii32::try_from_bytes(bytes).unwrap();
    /// assert_eq!(&s, "Hello world!");
    /// ```
    ///
    /// Using invalid ASCII characters:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii32, error::{WriteAsciiError}};
    /// let b1 = [200, 201, 202, 203];
    /// let err = TinyAscii32::try_from_bytes(&b1).unwrap_err();
    /// assert_eq!(err, WriteAsciiError::InvalidAscii);
    ///
    /// let b2 = [b'a', b'a', b'a', 0, b'1'];
    /// let err = TinyAscii32::try_from_bytes(&b2).unwrap_err();
    /// assert_eq!(err, WriteAsciiError::InvalidAscii);
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, error::{WriteAsciiError, CapacityLimitError}};
    /// let big = [b'a'; 500];
    /// let err = TinyAscii8::try_from_bytes(&big).unwrap_err();
    /// assert_eq!(err, WriteAsciiError::Capacity(CapacityLimitError));
    /// ```
    pub const fn try_from_bytes(bytes: &[u8]) -> Result<Self, WriteAsciiError> {
        const { assert_capacity::<N>() }

        if bytes.len() > N {
            return Err(WriteAsciiError::Capacity(CapacityLimitError));
        }

        let mut tiny = Self::empty();
        tiny.buf[N - 1] = encode_len(bytes.len());

        if let Err(err) = validate_tinyascii(bytes) {
            return Err(err);
        }

        // SAFETY:
        // - `bytes` is initialized with valid `NonNullAscii` values
        // - `tiny.buf` can be written up to `bytes.len()`
        // - `bytes` and `tiny.buf` do not overlap
        unsafe {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), tiny.buf.as_mut_ptr(), bytes.len());
        }

        Ok(tiny)
    }

    /// Creates a new [`TinyAscii`] from a non-null ASCII array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii, NonNullAscii};
    /// let arr = [NonNullAscii::SmallY, NonNullAscii::SmallE, NonNullAscii::SmallS];
    /// let s = TinyAscii::from_ascii_array(arr);
    /// assert_eq!(&s, "yes");
    /// ```
    pub fn from_ascii_array(ascii: [NonNullAscii; N]) -> TinyAscii<N> {
        const { assert_capacity::<N>() }

        Self {
            buf: ascii.map(NonNullAscii::to_u8),
        }
    }

    /// Creates a new [`TinyAscii`] from a byte slice `&[u8]`.
    ///
    /// The byte slice length must be less than capacity (`N`). During the conversion, it will
    /// replace any invalid ASCII characters with the provided replacement.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii16, NonNullAscii};
    /// let arr = [65, 66, 67, 0, 32, 200, 49, 50, 51, 255];
    /// let s = TinyAscii16::from_ascii_lossy(&arr, NonNullAscii::QuestionMark).unwrap();
    /// assert_eq!(&s, "ABC? ?123?");
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii, error::CapacityLimitError};
    /// let bytes = "string too large".as_bytes();
    /// let err = TinyAscii8::from_ascii_lossy(&bytes, NonNullAscii::QuestionMark).unwrap_err();
    /// assert_eq!(err, CapacityLimitError);
    /// ```
    pub const fn from_ascii_lossy(
        bytes: &[u8],
        replacement: NonNullAscii,
    ) -> Result<Self, CapacityLimitError> {
        const { assert_capacity::<N>() }

        if bytes.len() > N {
            return Err(CapacityLimitError);
        }

        let mut tiny = Self::empty();
        tiny.buf[N - 1] = encode_len(bytes.len());

        // SAFETY:
        // - `dst` length is greater than or equal `src` length
        unsafe { fill_non_null_ascii(&mut tiny.buf, bytes, replacement) };

        Ok(tiny)
    }

    /// Resizes [`TinyAscii`] capacity.
    ///
    /// If `M` is greater than `N`, the `TinyAscii` is extended by
    /// the difference. If `M` is less than `N`, the `TinyAscii` is
    /// simply truncated.
    ///
    /// # Examples
    ///
    /// Extending (`M` > `N`):
    ///
    /// ```
    /// # use tinyascii::{TinyAscii, NonNullAscii, error::CapacityLimitError};
    /// let s = TinyAscii::<20>::try_new("to be extended ...").unwrap();
    /// assert_eq!(s.capacity(), 20);
    ///
    /// let s = s.resize::<100>();
    /// assert_eq!(s.capacity(), 100);
    /// assert_eq!(&s, "to be extended ...");
    /// ```
    ///
    /// Truncating (`M` < `N`):
    ///
    /// ```
    /// # use tinyascii::{TinyAscii, NonNullAscii, error::CapacityLimitError};
    /// let s = TinyAscii::<20>::try_new("to be truncated ...").unwrap();
    /// assert_eq!(s.capacity(), 20);
    ///
    /// let s = s.resize::<8>();
    /// assert_eq!(s.capacity(), 8);
    /// assert_eq!(&s, "to be tr");
    /// ```
    pub const fn resize<const M: usize>(self) -> TinyAscii<M> {
        const { assert_capacity::<M>() }

        let mut tiny = TinyAscii::<M>::empty();

        let len = self.len();
        let final_len = if len < M {
            tiny.buf[M - 1] = encode_len(len);
            len
        } else {
            M
        };

        // SAFETY:
        // - `self.buf` is initialized with valid `NonNullAscii` values up to `final_len`
        // - `tiny.buf` can be written up to `final_len`
        // - `self.buf` and `tiny.buf` do not overlap
        unsafe {
            core::ptr::copy_nonoverlapping(self.buf.as_ptr(), tiny.buf.as_mut_ptr(), final_len);
        }

        tiny
    }

    /// Returns this [`TinyAscii`]'s capacity, in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii;
    /// let s = TinyAscii::<10>::empty();
    ///
    /// assert_eq!(s.capacity(), 10);
    /// ```
    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns the length in bytes, and given ASCII occupies 1 byte, it's also
    /// the ASCII string length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii8;
    /// let a = TinyAscii8::try_new("foo.bar").unwrap();
    /// assert_eq!(a.len(), 7);
    /// ```
    pub const fn len(&self) -> usize {
        let maybe_len = self.buf[N - 1];

        let len = if (maybe_len & LEN_MASK) == LEN_MASK {
            (maybe_len & !LEN_MASK) as usize
        } else {
            N
        };

        // SAFETY: `self.len()` is always less or equal to N, since N is the
        // total capacity.
        unsafe { core::hint::assert_unchecked(len <= N) };

        len
    }

    /// Returns the available capacity, in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii8;
    /// let s = TinyAscii8::try_new("foo").unwrap();
    ///
    /// assert_eq!(s.available_capacity(), 5);
    /// assert_eq!(s.len() + s.available_capacity(), s.capacity());
    /// ```
    #[inline(always)]
    pub const fn available_capacity(&self) -> usize {
        N - self.len()
    }

    /// Returns `true` if this [`TinyAscii`] has a length of zero, and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii128, NonNullAscii};
    /// let mut v = TinyAscii128::empty();
    /// dbg!(v.len());
    /// assert!(v.is_empty());
    ///
    /// v.try_push(NonNullAscii::SmallA).unwrap();
    /// assert!(!v.is_empty());
    /// ```
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.buf[0] == 0
    }

    /// Returns `true` if this [`TinyAscii`] capacity is filled, and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii;
    /// let mut v = TinyAscii::<6>::try_new("foobar").unwrap();
    ///
    /// assert_eq!(v.capacity(), v.len());
    /// assert!(v.is_full());
    ///
    /// v.pop();
    ///
    /// assert_ne!(v.capacity(), v.len());
    /// assert!(!v.is_full());
    /// ```
    #[inline(always)]
    pub const fn is_full(&self) -> bool {
        let maybe_len = self.buf[N - 1];
        maybe_len & LEN_MASK == 0
    }

    /// Converts a [`TinyAscii`] to a byte slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii8;
    /// let s = TinyAscii8::try_new("foobar").unwrap();
    /// assert_eq!(s.as_bytes(), b"foobar");
    /// ```
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8] {
        // SAFETY: `self.len()` is always less that or equal N and
        // points to valid ASCII data
        unsafe { core::slice::from_raw_parts(self.buf.as_ptr(), self.len()) }
    }

    /// Returns the [`TinyAscii`] as a string slice `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii16;
    /// let s = TinyAscii16::try_new("hello world").unwrap();
    /// assert_eq!(s.as_str(), "hello world");
    /// ```
    #[inline(always)]
    pub const fn as_str(&self) -> &str {
        // SAFETY: `TinyAscii` is only constructed from valid ASCII (hence, UTF-8) bytes.
        unsafe { core::str::from_utf8_unchecked(self.as_bytes()) }
    }

    /// Returns the [`TinyAscii`] as a [`NonNullAscii`] slice `&[NonNullAscii]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii};
    /// let s = TinyAscii8::try_new("abc").unwrap();
    /// let arr = [NonNullAscii::SmallA, NonNullAscii::SmallB, NonNullAscii::SmallC];
    /// assert_eq!(s.as_ascii(), &arr);
    /// ```
    #[inline(always)]
    pub const fn as_ascii(&self) -> &[NonNullAscii] {
        // SAFETY: `self.len()` is always less that or equal N and
        // points to a valid `NonNullAscii` slice
        unsafe { core::slice::from_raw_parts(self.buf.as_ptr().cast(), self.len()) }
    }

    /// Returns the [`TinyAscii`] as a mutable [`NonNullAscii`] slice `&mut [NonNullAscii]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii};
    /// let mut s = TinyAscii8::try_new("111").unwrap();
    ///
    /// let ascii = s.as_mut_ascii();
    /// ascii[1] = NonNullAscii::Digit0;
    ///
    /// assert_eq!(&s, "101");
    /// ```
    #[inline(always)]
    pub const fn as_mut_ascii(&mut self) -> &mut [NonNullAscii] {
        // SAFETY: `self.len()` is always less that or equal N and
        // points to a valid `NonNullAscii` slice
        unsafe { core::slice::from_raw_parts_mut(self.buf.as_mut_ptr().cast(), self.len()) }
    }

    /// Truncates this [`TinyAscii`], removing all contents.
    ///
    /// While this means the [`TinyAscii`] will have a length of zero, it does not touch its capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii4;
    /// let mut s = TinyAscii4::try_new("baz").unwrap();
    ///
    /// s.clear();
    ///
    /// assert!(s.is_empty());
    /// assert_eq!(s.len(), 0);
    /// assert_eq!(s.capacity(), 4);
    /// ```
    #[inline(always)]
    pub const fn clear(&mut self) {
        self.buf = Self::empty().buf;
    }

    /// Shortens this [`TinyAscii`] to the specified length.
    ///
    /// If `new_len` is greater than or equal to the [`TinyAscii`]'s current length, this has no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii8;
    /// let mut s = TinyAscii8::try_new("hello").unwrap();
    ///
    /// s.truncate(2);
    ///
    /// assert_eq!(s.len(), 2);
    /// assert_eq!(&s, "he");
    ///
    /// s.truncate(7);
    ///
    /// assert_eq!(s.len(), 2);
    /// ```
    #[inline(always)]
    pub const fn truncate(&mut self, new_len: usize) {
        if new_len >= self.len() {
            return;
        }

        self.buf[N - 1] = encode_len(new_len);
    }

    /// Appends the given [`NonNullAscii`] to the end of this [`TinyAscii`].
    ///
    /// Returns an [`CapacityLimitError`] if there is no spare capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii16, NonNullAscii};
    /// let mut s = TinyAscii16::try_new("hello").unwrap();
    ///
    /// s.try_push(NonNullAscii::ExclamationMark).unwrap();
    ///
    /// assert_eq!(&s, "hello!");
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii, NonNullAscii, error::CapacityLimitError};
    /// let mut s: TinyAscii<5> = TinyAscii::try_new("hello").unwrap();
    ///
    /// let err = s.try_push(NonNullAscii::ExclamationMark).unwrap_err();
    ///
    /// assert_eq!(err, CapacityLimitError);
    /// assert_eq!(&s, "hello");
    /// ```
    pub const fn try_push(&mut self, ascii: NonNullAscii) -> Result<(), CapacityLimitError> {
        let len = self.len();
        if len == N {
            return Err(CapacityLimitError);
        }

        let final_len = len + 1;
        self.buf[N - 1] = encode_len(final_len);

        self.buf[len] = ascii.to_u8();

        Ok(())
    }

    /// Appends the given [`NonNullAscii`] slice to the end of this [`TinyAscii`].
    ///
    /// Returns an [`CapacityLimitError`] if there is no spare capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii};
    /// let mut s = TinyAscii8::try_new("foo").unwrap();
    ///
    /// let bar = [NonNullAscii::SmallB, NonNullAscii::SmallA, NonNullAscii::SmallR];
    /// s.try_push_slice(&bar).unwrap();
    ///
    /// assert_eq!(&s, "foobar");
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii4, NonNullAscii, error::CapacityLimitError};
    /// let mut s = TinyAscii4::try_new("foo").unwrap();
    ///
    /// let bar = [NonNullAscii::SmallB, NonNullAscii::SmallA, NonNullAscii::SmallR];
    /// let err = s.try_push_slice(&bar).unwrap_err();
    ///
    /// assert_eq!(err, CapacityLimitError);
    /// assert_eq!(&s, "foo");
    /// ```
    pub const fn try_push_slice(
        &mut self,
        ascii: &[NonNullAscii],
    ) -> Result<(), CapacityLimitError> {
        let len = self.len();
        let final_len = len + ascii.len();

        if final_len > N {
            return Err(CapacityLimitError);
        }

        self.buf[N - 1] = encode_len(final_len);

        // SAFETY: `buf + len` is within the array bounds.
        let dst = unsafe { self.buf.as_mut_ptr().add(len) };

        // SAFETY:
        // - `ascii` is initialized with valid `NonNullAscii` bytes up to `ascii.len()`
        // - `self.buf` has available capacity >= `ascii.len()`
        // - `self.buf` and `ascii` do not overlap
        unsafe {
            core::ptr::copy_nonoverlapping(ascii.as_ptr().cast(), dst, ascii.len());
        }

        Ok(())
    }

    /// Appends the given byte to the end of this [`TinyAscii`].
    ///
    /// Returns an [`WriteAsciiError`] if the byte is an invalid non-null
    /// ASCII or if there is no spare capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii16;
    /// let mut s = TinyAscii16::try_new("correct").unwrap();
    ///
    /// s.try_push_u8(b'!').unwrap();
    ///
    /// assert_eq!(&s, "correct!");
    /// ```
    ///
    /// Invalid ASCII:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, error::WriteAsciiError};
    /// let mut s = TinyAscii8::try_new("invalid").unwrap();
    ///
    /// let err = s.try_push_u8(201).unwrap_err();
    ///
    /// assert_eq!(err, WriteAsciiError::InvalidAscii);
    /// assert_eq!(&s, "invalid");
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii, error::{WriteAsciiError, CapacityLimitError}};
    /// let mut s: TinyAscii<5> = TinyAscii::try_new("hello").unwrap();
    ///
    /// let err = s.try_push_u8(b'!').unwrap_err();
    ///
    /// assert_eq!(err, WriteAsciiError::Capacity(CapacityLimitError));
    /// assert_eq!(&s, "hello");
    /// ```
    pub const fn try_push_u8(&mut self, byte: u8) -> Result<(), WriteAsciiError> {
        let Some(ascii) = NonNullAscii::from_u8(byte) else {
            return Err(WriteAsciiError::InvalidAscii);
        };
        match self.try_push(ascii) {
            Ok(()) => Ok(()),
            Err(err) => Err(WriteAsciiError::Capacity(err)),
        }
    }

    /// Appends the given byte slice to the end of this [`TinyAscii`].
    ///
    /// Returns an [`WriteAsciiError`] if the slice contains an invalid non-null
    /// ASCII or if there is no spare capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii16;
    /// let mut s = TinyAscii16::try_new("correct").unwrap();
    ///
    /// let bytes = [b' ', b'a', b'n', b's', b'w', b'e', b'r'];
    /// s.try_push_bytes(&bytes).unwrap();
    ///
    /// assert_eq!(&s, "correct answer");
    /// ```
    ///
    /// Invalid ASCII:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii16, error::WriteAsciiError};
    /// let mut s = TinyAscii16::try_new("invalid").unwrap();
    ///
    /// let bytes = [65, 66, 67, 0];
    /// let err = s.try_push_bytes(&bytes).unwrap_err();
    ///
    /// assert_eq!(err, WriteAsciiError::InvalidAscii);
    /// assert_eq!(&s, "invalid");
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii, error::{WriteAsciiError, CapacityLimitError}};
    /// let mut s: TinyAscii<5> = TinyAscii::try_new("hello").unwrap();
    ///
    /// let err = s.try_push_bytes(" world".as_bytes()).unwrap_err();
    ///
    /// assert_eq!(err, WriteAsciiError::Capacity(CapacityLimitError));
    /// assert_eq!(&s, "hello");
    /// ```
    pub const fn try_push_bytes(&mut self, bytes: &[u8]) -> Result<(), WriteAsciiError> {
        let len = self.len();
        let final_len = len + bytes.len();

        if final_len > N {
            return Err(WriteAsciiError::Capacity(CapacityLimitError));
        }

        if let Err(err) = validate_tinyascii(bytes) {
            return Err(err);
        }

        self.buf[N - 1] = encode_len(final_len);

        // SAFETY: `buf + len` is within the array bounds.
        let dst = unsafe { self.buf.as_mut_ptr().add(len) };

        // SAFETY:
        // - `bytes` is initialized with valid `NonNullAscii` values
        // - `self.buf` can be written up to `bytes.len()`
        // - `bytes` and `self.buf` do not overlap
        unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), dst, bytes.len()) };

        Ok(())
    }

    /// Appends the given byte slice to the end of this [`TinyAscii`].
    ///
    /// The byte slice length must fit in the spare capacity. During the conversion, it will
    /// replace any invalid ASCII characters with the provided replacement.
    ///
    /// Returns an [`CapacityLimitError`] if there is no spare capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii16, NonNullAscii};
    /// let mut s = TinyAscii16::try_new("123").unwrap();
    ///
    /// let bytes = [181, 65, 66, 67, 0];
    /// s.try_push_lossy(&bytes, NonNullAscii::QuestionMark).unwrap();
    ///
    /// assert_eq!(&s, "123?ABC?");
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii, NonNullAscii, error::CapacityLimitError};
    /// let mut s: TinyAscii<5> = TinyAscii::try_new("hello").unwrap();
    ///
    /// let err = s.try_push_lossy(" world".as_bytes(), NonNullAscii::CommercialAt).unwrap_err();
    ///
    /// assert_eq!(err, CapacityLimitError);
    /// assert_eq!(&s, "hello");
    /// ```
    pub fn try_push_lossy(
        &mut self,
        bytes: &[u8],
        replacement: NonNullAscii,
    ) -> Result<(), CapacityLimitError> {
        let len = self.len();
        let final_len = len + bytes.len();

        if final_len > N {
            return Err(CapacityLimitError);
        }

        // SAFETY:
        // - `dst` length is greater than or equal `src` length
        unsafe { fill_non_null_ascii(&mut self.buf[len..], bytes, replacement) };

        if final_len != N {
            self.buf[N - 1] = encode_len(final_len);
        }

        Ok(())
    }

    /// Appends the given string `&str` to the end of this [`TinyAscii`].
    ///
    /// Returns an [`WriteAsciiError`] if the str contains an invalid non-null
    /// ASCII or if there is no spare capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii16;
    /// let mut s = TinyAscii16::try_new("foo").unwrap();
    ///
    /// s.try_push_str("bar").unwrap();
    ///
    /// assert_eq!(&s, "foobar");
    /// ```
    ///
    /// Invalid ASCII:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii16, error::WriteAsciiError};
    /// let mut s = TinyAscii16::try_new("non-ascii").unwrap();
    ///
    /// let err = s.try_push_str("⚡️").unwrap_err();
    ///
    /// assert_eq!(err, WriteAsciiError::InvalidAscii);
    /// assert_eq!(&s, "non-ascii");
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii, error::{WriteAsciiError, CapacityLimitError}};
    /// let mut s: TinyAscii<6> = TinyAscii::try_new("full").unwrap();
    ///
    /// let err = s.try_push_str(" house").unwrap_err();
    ///
    /// assert_eq!(err, WriteAsciiError::Capacity(CapacityLimitError));
    /// assert_eq!(&s, "full");
    /// ```
    pub const fn try_push_str(&mut self, s: &str) -> Result<(), WriteAsciiError> {
        self.try_push_bytes(s.as_bytes())
    }

    /// Inserts a character into this [`TinyAscii`] at byte position `idx`.
    ///
    /// Makes space for the insertion by copying all bytes of
    /// `&self[idx..]` to new positions.
    ///
    /// Note that calling this in a loop can result in quadratic behavior.
    ///
    /// Returns [`InsertAsciiError`] if `idx` is larger than the `TinyAscii`'s
    /// length, or if there is no spare capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii4, NonNullAscii};
    /// let mut s = TinyAscii4::empty();
    ///
    /// s.try_insert(0, NonNullAscii::SmallF).unwrap();
    /// s.try_insert(1, NonNullAscii::SmallO).unwrap();
    /// s.try_insert(2, NonNullAscii::SmallO).unwrap();
    ///
    /// assert_eq!(&s, "foo");
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii4, NonNullAscii, error::{InsertAsciiError, CapacityLimitError}};
    /// let mut s = TinyAscii4::try_new("full").unwrap();
    ///
    /// let err = s.try_insert(2, NonNullAscii::HyphenMinus).unwrap_err();
    ///
    /// assert_eq!(err, InsertAsciiError::Capacity(CapacityLimitError));
    /// assert_eq!(&s, "full");
    /// ```
    ///
    /// Index out of bounds:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii, error::{InsertAsciiError, CapacityLimitError}};
    /// let mut s = TinyAscii8::try_new("12345").unwrap();
    ///
    /// let err = s.try_insert(8, NonNullAscii::PercentSign).unwrap_err();
    ///
    /// assert_eq!(err, InsertAsciiError::IndexOutOfBounds);
    /// ```
    pub const fn try_insert(
        &mut self,
        idx: usize,
        ascii: NonNullAscii,
    ) -> Result<(), InsertAsciiError> {
        let len = self.len();

        if len == N {
            return Err(InsertAsciiError::Capacity(CapacityLimitError));
        }
        if idx > len {
            return Err(InsertAsciiError::IndexOutOfBounds);
        }

        // SAFETY: `buf` is valid for reads from `idx` up to `len - idx`.
        let src = unsafe { self.buf.as_ptr().add(idx) };
        // SAFETY: `buf` is valid for writes from `idx + 1` up to `len - idx`.
        let dst = unsafe { self.buf.as_mut_ptr().add(idx + 1) };
        // SAFETY: Move the bytes starting from `idx` to their new location 1 byte
        // ahead. This is safe because sufficient capacity is ensured by checks
        // above.
        unsafe {
            core::ptr::copy(src, dst, len - idx);
        }

        self.buf[idx] = ascii.to_u8();
        self.buf[N - 1] = encode_len(len + 1);

        Ok(())
    }

    /// Inserts a non-null ASCII slice into this [`TinyAscii`] at byte position `idx`.
    ///
    /// Makes space for the insertion by copying all bytes of `&self[idx..]` to new
    /// positions.
    ///
    /// Note that calling this in a loop can result in quadratic behavior.
    ///
    /// Returns [`InsertAsciiError`] if `idx` is larger than the `TinyAscii`'s
    /// length, or if there is no spare capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii};
    /// let mut s = TinyAscii8::try_new("bar").unwrap();
    /// let foo = [NonNullAscii::SmallF, NonNullAscii::SmallO, NonNullAscii::SmallO];
    ///
    /// s.try_insert_slice(0, &foo).unwrap();
    ///
    /// assert_eq!(&s, "foobar");
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii4, NonNullAscii, error::{InsertAsciiError, CapacityLimitError}};
    /// let mut s = TinyAscii4::try_new("bar").unwrap();
    /// let foo = [NonNullAscii::SmallF, NonNullAscii::SmallO, NonNullAscii::SmallO];
    ///
    /// let err = s.try_insert_slice(3, &foo).unwrap_err();
    ///
    /// assert_eq!(err, InsertAsciiError::Capacity(CapacityLimitError));
    /// assert_eq!(&s, "bar");
    /// ```
    ///
    /// Index out of bounds:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii, error::{InsertAsciiError, CapacityLimitError}};
    /// let mut s = TinyAscii8::try_new("bar").unwrap();
    /// let foo = [NonNullAscii::SmallF, NonNullAscii::SmallO, NonNullAscii::SmallO];
    ///
    /// let err = s.try_insert_slice(4, &foo).unwrap_err();
    ///
    /// assert_eq!(err, InsertAsciiError::IndexOutOfBounds);
    /// assert_eq!(&s, "bar");
    /// ```
    pub const fn try_insert_slice(
        &mut self,
        idx: usize,
        ascii: &[NonNullAscii],
    ) -> Result<(), InsertAsciiError> {
        let len = self.len();
        let new_len = len + ascii.len();

        if new_len > N {
            return Err(InsertAsciiError::Capacity(CapacityLimitError));
        }
        if idx > len {
            return Err(InsertAsciiError::IndexOutOfBounds);
        }

        {
            // SAFETY: `buf` is valid for reads from `idx` up to `len - idx`.
            let src = unsafe { self.buf.as_ptr().add(idx) };
            // SAFETY: `buf` is valid for writes from `idx + ascii.len()` up to `len - idx`.
            let dst = unsafe { self.buf.as_mut_ptr().add(idx + ascii.len()) };
            // SAFETY: Move the bytes starting from `idx` to their new location `ascii.len()` bytes
            // ahead. This is safe because sufficient capacity is ensured.
            unsafe {
                core::ptr::copy(src, dst, len - idx);
            }
        }

        // SAFETY: `buf` is valid for writes from `idx` up to `len - idx`.
        let dst = unsafe { self.buf.as_mut_ptr().add(idx) };
        // SAFETY: Copy the new slice into the vacated region if `idx != len`,
        // or into the spare capacity otherwise.
        unsafe {
            core::ptr::copy_nonoverlapping(ascii.as_ptr().cast(), dst, ascii.len());
        }

        self.buf[N - 1] = encode_len(new_len);

        Ok(())
    }

    /// Copies elements from `src` range to the end of the string.
    ///
    /// Returns [`InsertRangeError`] if the range has `start_bound > end_bound`, or if
    /// the it exceeds the available capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use tinyascii::{TinyAscii32, NonNullAscii};
    /// let mut s = TinyAscii32::try_new("abcde").unwrap();
    ///
    /// s.extend_from_within(2..).unwrap();
    /// assert_eq!(&s, "abcdecde");
    ///
    /// s.extend_from_within(..2).unwrap();
    /// assert_eq!(&s, "abcdecdeab");
    ///
    /// s.extend_from_within(4..8).unwrap();
    /// assert_eq!(&s, "abcdecdeabecde");
    ///
    /// s.try_push(NonNullAscii::Colon).unwrap();
    ///
    /// s.extend_from_within(..).unwrap();
    /// assert_eq!(&s, "abcdecdeabecde:abcdecdeabecde:");
    /// ```
    ///
    /// Range out of bounds:
    ///
    /// ```
    /// use tinyascii::{TinyAscii32, error::InsertRangeError};
    /// let mut s = TinyAscii32::try_new("abcde").unwrap();
    ///
    /// let err = s.extend_from_within(2..9).unwrap_err();
    /// assert_eq!(err, InsertRangeError::RangeOutOfBounds);
    ///
    /// let err = s.extend_from_within(..7).unwrap_err();
    /// assert_eq!(err, InsertRangeError::RangeOutOfBounds);
    ///
    /// let err = s.extend_from_within(6..).unwrap_err();
    /// assert_eq!(err, InsertRangeError::RangeOutOfBounds);
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// use tinyascii::{TinyAscii8, error::{InsertRangeError, CapacityLimitError}};
    /// let mut s = TinyAscii8::try_new("abcde").unwrap();
    ///
    /// let err = s.extend_from_within(..).unwrap_err();
    /// assert_eq!(err, InsertRangeError::Capacity(CapacityLimitError));
    ///
    /// let err = s.extend_from_within(1..).unwrap_err();
    /// assert_eq!(err, InsertRangeError::Capacity(CapacityLimitError));
    ///
    /// let err = s.extend_from_within(..5).unwrap_err();
    /// assert_eq!(err, InsertRangeError::Capacity(CapacityLimitError));
    /// ```
    pub fn extend_from_within<R>(&mut self, src: R) -> Result<(), InsertRangeError>
    where
        R: RangeBounds<usize>,
    {
        let len = self.len();
        let range = normalize_range(src, len);

        if range.start > range.end || range.end > len {
            return Err(InsertRangeError::RangeOutOfBounds);
        }

        let count = range.end - range.start;

        if count > self.available_capacity() {
            return Err(InsertRangeError::Capacity(CapacityLimitError));
        }

        self.buf[N - 1] = encode_len(len + count);

        // SAFETY:
        // - `range.start` index in a filled range in `self.buf`
        let source = unsafe { self.buf.as_ptr().add(range.start) };
        // SAFETY:
        // - `range.end` index at the start of the unfilled range in `self.buf`
        let dst = unsafe { self.buf.as_mut_ptr().add(len) };

        // SAFETY:
        // - Both pointers are offset by `range.start` and `len` and, as checked
        // above, both memory regions do not overlap.
        // - `count` is less or equal to the len of `source`, so source
        // is valid for `count` reads
        // - `count <= self.available_capacity()` guarantees that `dst.len() >= count`
        // so `dst` is valid for `count` writes
        unsafe {
            core::ptr::copy_nonoverlapping(source, dst, count);
        }

        Ok(())
    }

    /// Removes the specified range in the [`TinyAscii`], and replaces it with the given
    /// `&[NonNullAscii]`. The given string doesn’t need to be the same length as the range.
    ///
    /// Returns [`InsertRangeError`] if the range has `start_bound > end_bound`, if
    /// `end_bound > self.len()`, or if it exceeds the available capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii32, NonNullAscii};
    /// let mut s = TinyAscii32::try_new("abcde").unwrap();
    ///
    /// let replacement = [NonNullAscii::Digit1, NonNullAscii::Digit2, NonNullAscii::Digit3];
    ///
    /// s.replace_range(2.., &replacement).unwrap();
    /// assert_eq!(&s, "ab123");
    ///
    /// s.replace_range(..4, &replacement).unwrap();
    /// assert_eq!(&s, "1233");
    ///
    /// s.replace_range(3..4, &replacement).unwrap();
    /// assert_eq!(&s, "123123");
    ///
    /// s.replace_range(.., &replacement).unwrap();
    /// assert_eq!(&s, "123");
    /// ```
    ///
    /// Range out of bounds:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii16, NonNullAscii, error::InsertRangeError};
    /// let mut s = TinyAscii16::try_new("abcde").unwrap();
    ///
    /// let err = s.replace_range(2..6, &[NonNullAscii::QuestionMark]).unwrap_err();
    /// assert_eq!(err, InsertRangeError::RangeOutOfBounds);
    ///
    /// let err = s.replace_range(..7, &[NonNullAscii::QuestionMark]).unwrap_err();
    /// assert_eq!(err, InsertRangeError::RangeOutOfBounds);
    ///
    /// let err = s.replace_range(6.., &[NonNullAscii::QuestionMark]).unwrap_err();
    /// assert_eq!(err, InsertRangeError::RangeOutOfBounds);
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii, error::{InsertRangeError, CapacityLimitError}};
    /// let mut s = TinyAscii8::try_new("abcde").unwrap();
    ///
    /// let replacement = [NonNullAscii::NumberSign; 10];
    ///
    /// let err = s.replace_range(.., &replacement).unwrap_err();
    /// assert_eq!(err, InsertRangeError::Capacity(CapacityLimitError));
    ///
    /// let err = s.replace_range(1.., &replacement).unwrap_err();
    /// assert_eq!(err, InsertRangeError::Capacity(CapacityLimitError));
    ///
    /// let err = s.replace_range(..5, &replacement).unwrap_err();
    /// assert_eq!(err, InsertRangeError::Capacity(CapacityLimitError));
    /// ```
    pub fn replace_range<R>(
        &mut self,
        range: R,
        replace_with: &[NonNullAscii],
    ) -> Result<(), InsertRangeError>
    where
        R: RangeBounds<usize>,
    {
        let len = self.len();
        let arange = normalize_range(range, len);

        if arange.start > arange.end || arange.end > len {
            return Err(InsertRangeError::RangeOutOfBounds);
        }

        let range_len = arange.end - arange.start;

        if replace_with.len() as isize - range_len as isize > self.available_capacity() as isize {
            return Err(InsertRangeError::Capacity(CapacityLimitError));
        }

        let new_len = len + replace_with.len() - range_len;
        self.buf[N - 1] = encode_len(new_len);

        // Ajust post range data to its final place.
        let ajust_count = len - arange.end;
        if range_len != replace_with.len() && ajust_count != 0 {
            // SAFETY: `arange.end < len`, so it is within bounds of `self.buf`
            let src = unsafe { self.buf.as_ptr().add(arange.end) };
            // SAFETY: `arange.start + replace_with.len()` is `< self.buf.len` given the checks
            // above, so it is within bounds of `self.buf`
            let dst = unsafe { self.buf.as_mut_ptr().add(arange.start + replace_with.len()) };
            // SAFETY:
            // - `src` can be read up to `ajust_count`, as `ajust_count` is `len - arange.end`
            // - `dst` can be written up to `ajust_count`
            unsafe {
                core::ptr::copy(src, dst, ajust_count);
            }
        }

        // SAFETY: `arange.start` is at the start of the replacement section.
        let dst = unsafe { self.buf.as_mut_ptr().add(arange.start) };
        // SAFETY:
        // - `dst` can be written up to `replace_with.len()` as checked above.
        // - `replace_with.as_ptr()` is a valid shared reference valid for
        // reads up to `replace_with.len()`
        // - `self.buf` and `replace_with` does not overlap, as `dst` comes from a
        // mutable reference of `self.buf`
        unsafe {
            core::ptr::copy_nonoverlapping(
                replace_with.as_ptr().cast::<u8>(),
                dst,
                replace_with.len(),
            );
        }

        Ok(())
    }

    /// Removes the specified range in the [`TinyAscii`].
    ///
    /// Returns [`InsertRangeError`] if the range has `start_bound > end_bound`, or if
    /// `end_bound > self.len()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii32;
    /// let mut s = TinyAscii32::try_new("0123456789").unwrap();
    ///
    /// s.remove_range(2..4).unwrap();
    /// assert_eq!(&s, "01456789");
    ///
    /// s.remove_range(..4).unwrap();
    /// assert_eq!(&s, "6789");
    ///
    /// s.remove_range(3..).unwrap();
    /// assert_eq!(&s, "678");
    ///
    /// s.remove_range(..).unwrap();
    /// assert_eq!(&s, "");
    /// ```
    ///
    /// Range out of bounds:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii32, error::RemoveRangeError};
    /// let mut s = TinyAscii32::try_new("abcde").unwrap();
    ///
    /// let err = s.remove_range(2..6).unwrap_err();
    /// assert_eq!(err, RemoveRangeError);
    ///
    /// let err = s.remove_range(..7).unwrap_err();
    /// assert_eq!(err, RemoveRangeError);
    ///
    /// let err = s.remove_range(6..).unwrap_err();
    /// assert_eq!(err, RemoveRangeError);
    /// ```
    pub fn remove_range<R>(&mut self, range: R) -> Result<(), RemoveRangeError>
    where
        R: RangeBounds<usize>,
    {
        let len = self.len();
        let arange = normalize_range(range, len);

        if arange.start > arange.end || arange.end > len {
            return Err(RemoveRangeError);
        }

        let range_len = arange.end - arange.start;

        if range_len == 0 {
            return Ok(());
        }

        self.buf[N - 1] = encode_len(len - range_len);

        // SAFETY: `arange.end < len`, so it is within bounds of `self.buf`
        let src = unsafe { self.buf.as_ptr().add(arange.end) };

        // SAFETY: `arange.start < self.buf.len` ensure by the checks
        // above, so it is within bounds of `self.buf`
        let dst = unsafe { self.buf.as_mut_ptr().add(arange.start) };

        // SAFETY:
        // - `src` can be read up to `len - arange.end`
        // - `dst` can be written up to `len - arange.end`
        unsafe {
            core::ptr::copy(src, dst, len - arange.end);
        }

        Ok(())
    }

    /// Removes a [`NonNullAscii`] from this [`TinyAscii`] at byte
    /// position `idx` and returns it.
    ///
    /// Copies all bytes after the removed ASCII to new positions.
    ///
    /// Note that calling this in a loop can result in quadratic
    /// behavior.
    ///
    /// Returns [`None`] if `idx` is larger than or equal to the
    /// `TinyAscii`'s length.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii, error::RemoveRangeError};
    /// let mut s = TinyAscii8::try_new("abcde").unwrap();
    ///
    /// assert_eq!(s.remove(0), Some(NonNullAscii::SmallA));
    /// assert_eq!(s.remove(1), Some(NonNullAscii::SmallC));
    /// assert_eq!(s.remove(0), Some(NonNullAscii::SmallB));
    ///
    /// assert_eq!(&s, "de");
    /// ```
    ///
    /// Index out of bounds:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii, error::RemoveRangeError};
    /// let mut s = TinyAscii8::try_new("abcde").unwrap();
    ///
    /// assert_eq!(s.remove(9), None);
    ///
    /// s.clear();
    /// assert_eq!(s.remove(0), None);
    ///
    /// assert_eq!(&s, "");
    /// ```
    pub const fn remove(&mut self, idx: usize) -> Option<NonNullAscii> {
        let len = self.len();
        if len == 0 || idx >= len {
            return None;
        }

        let ascii = self.buf[idx];
        self.buf[N - 1] = encode_len(len - 1);

        // SAFETY: `arange.end < len`, so it is within bounds of `self.buf`
        let src = unsafe { self.buf.as_ptr().add(idx + 1) };

        // SAFETY: `arange.start < self.buf.len` ensure by the checks
        // above, so it is within bounds of `self.buf`
        let dst = unsafe { self.buf.as_mut_ptr().add(idx) };

        // SAFETY:
        // - `src` can be read up to `len - idx`
        // - `dst` can be written up to `len - idx`
        unsafe {
            core::ptr::copy(src, dst, len - idx);
        }

        // SAFETY: `ascii` is a valid non-null ASCII
        Some(unsafe { NonNullAscii::from_u8_unchecked(ascii) })
    }

    /// Returns the last [`NonNullAscii`] character from a [`TinyAscii`] and returns it, or [`None`] if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii32, NonNullAscii};
    /// let mut s = TinyAscii32::try_new("101").unwrap();
    ///
    /// assert_eq!(s.pop(), Some(NonNullAscii::Digit1));
    /// assert_eq!(&s, "10");
    ///
    /// s.clear();
    ///
    /// assert_eq!(s.pop(), None);
    /// assert_eq!(&s, "");
    /// ```
    ///
    /// # Time complexity
    ///
    /// Takes *O*(1) time.
    pub const fn pop(&mut self) -> Option<NonNullAscii> {
        if self.is_empty() {
            return None;
        }

        let remove_idx = self.len() - 1;

        let code = self.buf[remove_idx];
        self.buf[N - 1] = encode_len(remove_idx);

        // SAFETY: code is a valid ASCII variant, as enforced by the `TinyAscii`.
        Some(unsafe { NonNullAscii::from_u8_unchecked(code) })
    }

    /// Retains only the characters specified by the predicate.
    ///
    /// In other words, remove all ASCII characters `c` such that `f(c)` returns `false`.
    /// This method operates in place, visiting each character exactly once in the
    /// original order, and preserves the order of the retained characters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii16;
    /// let mut s = TinyAscii16::try_new("f_o_ob_ar").unwrap();
    ///
    /// s.retain(|c| c != '_');
    ///
    /// assert_eq!(&s, "foobar");
    /// ```
    ///
    /// Because the elements are visited exactly once in the original order,
    /// external state may be used to decide which elements to keep.
    ///
    /// ```
    /// # use tinyascii::TinyAscii8;
    /// let mut s = TinyAscii8::try_new("abcde").unwrap();
    /// let keep = [false, true, true, false, true];
    ///
    /// let mut iter = keep.iter();
    /// s.retain(|_| *iter.next().unwrap());
    ///
    /// assert_eq!(&s, "bce");
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(NonNullAscii) -> bool,
    {
        struct SetLenOnDrop<'a, const N: usize> {
            tiny: &'a mut TinyAscii<N>,
            idx: usize,
            deleted: usize,
        }

        impl<'a, const N: usize> Drop for SetLenOnDrop<'a, N> {
            fn drop(&mut self) {
                let new_len = self.idx - self.deleted;
                // SAFETY: after deleting characters, the `new_len` will always
                // be less or equal than previous `tiny.len()`
                unsafe { core::hint::assert_unchecked(new_len <= self.tiny.len()) };
                self.tiny.buf[N - 1] = encode_len(new_len);
            }
        }

        let len = self.len();
        let mut guard = SetLenOnDrop {
            tiny: self,
            idx: 0,
            deleted: 0,
        };

        while guard.idx < len {
            // SAFETY: `byte` is a valid `NonNullAscii`
            let ascii = unsafe { NonNullAscii::from_u8_unchecked(guard.tiny.buf[guard.idx]) };

            if !f(ascii) {
                guard.deleted += 1;
            } else if guard.deleted > 0 {
                let free_idx = guard.idx - guard.deleted;
                guard.tiny.buf[free_idx] = ascii.to_u8();
            }

            // Point idx to the next char
            guard.idx += 1;
        }

        drop(guard);
    }

    /// Decomposes a [`TinyAscii`] into its raw components: `(byte_array, length)`.
    ///
    /// Returns the array with the ASCII string data, and the length of the string (in bytes).
    /// These are the same arguments in the same order as the arguments to [`from_parts`].
    ///
    /// [`from_parts`]: TinyAscii::from_parts
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, TinyAscii};
    /// let s = TinyAscii8::try_new("foobar").unwrap();
    ///
    /// let (mut arr, len) = s.into_parts();
    ///
    /// arr[3] = b'B';
    ///
    /// let rebuilt = unsafe { TinyAscii::from_parts(arr, len) };
    ///
    /// assert_eq!(&rebuilt, "fooBar");
    /// ```
    pub const fn into_parts(self) -> ([u8; N], usize) {
        let len = self.len();
        (self.buf, len)
    }

    /// Creates a [`TinyAscii`] directly from a byte array, and length.
    ///
    /// # Safety
    ///
    /// Caller must ensure that:
    ///
    /// * `length` is less than or equal to `arr.len()`.
    /// * The first `length` bytes must be valid [`NonNullAscii`] characters.
    ///
    /// These requirements are always upheld by any `arr` and `length` pair
    /// returned from [`into_parts`]. Other sources are allowed if the invariants
    /// are upheld.
    ///
    /// Violating these may cause UB when reading the created [`TinyAscii`] content.
    ///
    /// [`into_parts`]: TinyAscii::into_parts
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii128;
    /// let ascii = TinyAscii128::try_new("bar").unwrap();
    ///
    /// let (mut arr, len) = ascii.into_parts();
    ///
    /// arr[3..6].copy_from_slice("baz".as_bytes());
    ///
    /// // Put back together
    /// let rebuilt = unsafe { TinyAscii128::from_parts(arr, len + 3) };
    ///
    /// assert_eq!(&rebuilt, "barbaz");
    /// ```
    ///
    /// Using an array that was created elsewhere:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii128, MAX_LEN};
    /// let mut arr = [0; MAX_LEN];
    ///
    /// arr[..20].copy_from_slice("safe and valid usage".as_bytes());
    ///
    /// // Put together
    /// let rebuilt = unsafe { TinyAscii128::from_parts(arr, 20) };
    ///
    /// assert_eq!(&rebuilt, "safe and valid usage");
    /// ```
    pub const unsafe fn from_parts(mut arr: [u8; N], len: usize) -> Self {
        const { assert_capacity::<N>() }

        if len != N {
            arr[N - 1] = encode_len(len);
        }
        Self { buf: arr }
    }
}

// Taken from https://github.com/rust-lang/rust/issues/76393#issuecomment-2699090574
#[inline]
fn normalize_range(range: impl RangeBounds<usize>, len: usize) -> Range<usize> {
    use Bound::*;

    let start = match range.start_bound() {
        Included(&start) => start,
        Excluded(&start) => start.saturating_add(1),
        Unbounded => 0,
    };
    let end = match range.end_bound() {
        Included(&end) => end.saturating_add(1),
        Excluded(&end) => end,
        Unbounded => len,
    };

    Range { start, end }
}
