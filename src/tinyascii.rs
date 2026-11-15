use core::{
    mem::MaybeUninit,
    ops::{Bound, RangeBounds},
    range::Range,
};

use crate::ascii::NonNullAscii;
use crate::error::{
    CapacityLimitError, InsertAsciiError, InsertRangeError, RemoveRangeError, WriteAsciiError,
};

/// A fixed-capacity ASCII string stored inline, holding up to `N` non-null
/// ASCII characters.
///
/// `N` must be greater than 0 and less than or equal to [`MAX_LEN`] (128),
/// otherwise compilation fails. The type aliases [`TinyAscii4`], [`TinyAscii8`],
/// [`TinyAscii16`], [`TinyAscii32`], [`TinyAscii64`] and [`TinyAscii128`] cover
/// the common sizes.
///
/// # Layout
///
/// The string is backed by `[u8; N]` and has no separate length field. The
/// content never contains NUL, and the last byte of the buffer doubles as the
/// length marker: if its top bit is set, the low 7 bits are the length;
/// otherwise the string is full and that byte is real content.
/// The size of a `TinyAscii<N>` is therefore exactly `N` bytes.
///
/// # Examples
///
/// ```
/// # use tinyascii::TinyAscii8;
/// let mut s = TinyAscii8::try_from_str("tiny").unwrap();
///
/// s.try_push_str("str").unwrap();
///
/// assert_eq!(&s, "tinystr");
/// assert_eq!(core::mem::size_of::<TinyAscii8>(), 8);
/// ```
#[derive(Clone)]
pub struct TinyAscii<const N: usize> {
    buf: [MaybeUninit<u8>; N],
}

/// The maximum capacity (`N`) of a [`TinyAscii`], in bytes.
///
/// The length marker uses the low 7 bits of the last byte, so a length
/// cannot be greater than 128.
pub const MAX_LEN: usize = 128;

/// A [`TinyAscii`] with a capacity of 4 bytes.
pub type TinyAscii4 = TinyAscii<4>;
/// A [`TinyAscii`] with a capacity of 8 bytes.
pub type TinyAscii8 = TinyAscii<8>;
/// A [`TinyAscii`] with a capacity of 16 bytes.
pub type TinyAscii16 = TinyAscii<16>;
/// A [`TinyAscii`] with a capacity of 32 bytes.
pub type TinyAscii32 = TinyAscii<32>;
/// A [`TinyAscii`] with a capacity of 64 bytes.
pub type TinyAscii64 = TinyAscii<64>;
/// A [`TinyAscii`] with the maximum capacity of 128 bytes ([`MAX_LEN`]).
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

impl<const N: usize> TinyAscii<N> {
    #[inline(always)]
    const fn last_byte(&self) -> u8 {
        // SAFETY: `buf[N - 1]` is always initialized.
        unsafe { self.buf[N - 1].assume_init() }
    }

    #[inline(always)]
    const unsafe fn set_len(&mut self, new_len: usize) {
        self.buf[N - 1].write(encode_len(new_len));
    }

    /// Creates a new empty [`TinyAscii`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii64;
    /// let s = TinyAscii64::new();
    ///
    /// assert_eq!(&s, "");
    /// ```
    #[inline(always)]
    pub const fn new() -> Self {
        const { assert_capacity::<N>() }

        let mut this = Self {
            buf: [MaybeUninit::uninit(); N],
        };
        this.buf[N - 1].write(LEN_MASK);
        this
    }

    /// Creates a new [`TinyAscii`] from a byte slice `&[u8]`.
    ///
    /// The byte slice must contain only non-null ASCII characters and
    /// its length must be less than or equal to the capacity (`N`).
    ///
    /// # Safety
    ///
    /// Caller must ensure that:
    ///
    /// * `bytes.len()` is less than or equal to `N`.
    /// * All bytes (`u8`) are valid [`NonNullAscii`] characters.
    ///
    /// Use the safe version, [`try_from_bytes`], if bytes may be invalid.
    ///
    /// Violating these requirements **will** cause UB.
    ///
    /// [`try_from_bytes`]: TinyAscii::try_from_bytes
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii32;
    /// let s = unsafe { TinyAscii32::from_bytes_unchecked("Hello world!".as_bytes()) };
    /// assert_eq!(&s, "Hello world!");
    /// ```
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> Self {
        const { assert_capacity::<N>() }
        let mut tiny = Self::new();

        // SAFETY: Caller must ensure `bytes.len()` is less or equal
        // than `N`.
        unsafe { tiny.set_len(bytes.len()) };

        // SAFETY:
        // - `bytes.len()` is less than or equal to `N` as
        // ensured by the caller
        // - `tiny.buf.len()` is greater or equal than `bytes`
        // length as ensured by the caller
        unsafe {
            crate::simd::copy_tinyascii(bytes, &mut tiny.buf);
        }

        tiny
    }

    /// Creates a new [`TinyAscii`] from a `&str`.
    ///
    /// The `&str` must contain only non-null ASCII characters and
    /// its length must be less than or equal to the capacity (`N`).
    ///
    /// # Errors
    ///
    /// Returns [`WriteAsciiError::InvalidAscii`] if the `&str` contains a null
    /// or non-ASCII character, or [`WriteAsciiError::Capacity`] if its length
    /// is greater than the capacity (`N`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii32;
    /// let s = TinyAscii32::try_from_str("Hello world!").unwrap();
    /// assert_eq!(&s, "Hello world!");
    /// ```
    ///
    /// Using invalid ASCII characters:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii32, error::WriteAsciiError};
    /// let err = TinyAscii32::try_from_str("invalid 😎 ascii").unwrap_err();
    /// assert_eq!(err, WriteAsciiError::InvalidAscii);
    ///
    /// let err = TinyAscii32::try_from_str("null present \0").unwrap_err();
    /// assert_eq!(err, WriteAsciiError::InvalidAscii);
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, error::{WriteAsciiError, CapacityLimitError}};
    /// let err = TinyAscii8::try_from_str("not so tiny string").unwrap_err();
    /// assert_eq!(err, WriteAsciiError::Capacity(CapacityLimitError));
    /// ```
    #[inline(always)]
    pub fn try_from_str(s: &str) -> Result<Self, WriteAsciiError> {
        const { assert_capacity::<N>() }

        Self::try_from_bytes(s.as_bytes())
    }

    /// Creates a new [`TinyAscii`] from a byte slice `&[u8]`.
    ///
    /// The byte slice must contain only non-null ASCII characters and
    /// its length must be less than or equal to the capacity (`N`).
    ///
    /// # Errors
    ///
    /// Returns [`WriteAsciiError::InvalidAscii`] if the slice contains a null
    /// or non-ASCII byte, or [`WriteAsciiError::Capacity`] if its length is
    /// greater than the capacity (`N`).
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
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, WriteAsciiError> {
        const { assert_capacity::<N>() }

        if bytes.len() > N {
            return Err(WriteAsciiError::Capacity(CapacityLimitError));
        }

        let mut out = const {
            Ok(Self {
                buf: [MaybeUninit::uninit(); N],
            })
        };
        // SAFETY: `out` has an `Ok` variant
        let tiny = unsafe { out.as_mut().ok().unwrap_unchecked() };
        // SAFETY: if tiny is valid, it must have a length equal to bytes.
        unsafe { tiny.set_len(bytes.len()) };

        // SAFETY:
        // - `bytes.len()` is less or equal to `N` as verified above
        // - `tiny.buf` length is greater than or equal `bytes.len()`
        if !unsafe { crate::simd::copy_valid_tinyascii(bytes, &mut tiny.buf) } {
            out = Err(WriteAsciiError::InvalidAscii);
        }

        out
    }

    /// Creates a new [`TinyAscii`] from a slice of non-null ASCII characters.
    ///
    /// The slice length must be less than or equal to the capacity (`N`).
    ///
    /// # Errors
    ///
    /// Returns a [`CapacityLimitError`] if the slice length is greater than the
    /// capacity (`N`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii16, NonNullAscii};
    /// let slice = [NonNullAscii::SmallO, NonNullAscii::SmallK];
    /// let s = TinyAscii16::try_from_slice(&slice).unwrap();
    /// assert_eq!(&s, "ok");
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii, error::CapacityLimitError};
    /// let big = [NonNullAscii::SmallA; 20];
    /// let err = TinyAscii8::try_from_slice(&big).unwrap_err();
    /// assert_eq!(err, CapacityLimitError);
    /// ```
    pub fn try_from_slice(slice: &[NonNullAscii]) -> Result<Self, CapacityLimitError> {
        const { assert_capacity::<N>() }

        if slice.len() > N {
            return Err(CapacityLimitError);
        }

        let mut tiny = Self::new();
        // SAFETY: `slice.len()` is less than or equal to `N` as verified above.
        unsafe { tiny.set_len(slice.len()) };

        // SAFETY: `NonNullAscii` is `#[repr(u8)]`, so it has the same size, alignment
        // and valid bit patterns subset as `u8`; the pointer and length come from a
        // valid slice.
        let bytes =
            unsafe { core::slice::from_raw_parts(slice.as_ptr().cast::<u8>(), slice.len()) };

        // SAFETY:
        // - `bytes.len()` is less than or equal to `N` (and thus `MAX_LEN`) as verified above
        // - `tiny.buf.len()` is `N`, greater than or equal to `bytes.len()`
        unsafe { crate::simd::copy_tinyascii(bytes, &mut tiny.buf) };

        Ok(tiny)
    }

    /// Creates a new [`TinyAscii`] from a non-null ASCII array.
    ///
    /// The array length `M` can be less than or equal to the capacity (`N`).
    ///
    /// # Panics
    ///
    /// Fails to compile (const panic) if `M` is greater than the capacity (`N`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii};
    /// let arr = [NonNullAscii::SmallY, NonNullAscii::SmallE, NonNullAscii::SmallS];
    /// let s = TinyAscii8::from_ascii_array(arr);
    /// assert_eq!(&s, "yes");
    /// ```
    ///
    /// An array larger than the capacity fails to compile:
    ///
    /// ```compile_fail
    /// # use tinyascii::{TinyAscii4, NonNullAscii};
    /// let _ = TinyAscii4::from_ascii_array([NonNullAscii::SmallA; 5]);
    /// ```
    pub fn from_ascii_array<const M: usize>(arr: [NonNullAscii; M]) -> TinyAscii<N> {
        const {
            assert_capacity::<N>();
            assert!(
                M <= N,
                "array length (M) must be less than or equal to TinyAscii capacity (N)"
            );
        }

        // `M <= N` is verified at compile time above, so this cannot fail.
        match Self::try_from_slice(&arr) {
            Ok(tiny) => tiny,
            // SAFETY: `M <= N` is enforced by the const block above.
            Err(_) => unsafe { core::hint::unreachable_unchecked() },
        }
    }

    /// Creates a new [`TinyAscii`] from a byte slice `&[u8]`.
    ///
    /// The byte slice length must be less than or equal to the capacity (`N`). During the
    /// conversion, any null or non-ASCII byte is replaced with the provided replacement.
    ///
    /// # Errors
    ///
    /// Returns a [`CapacityLimitError`] if the slice length is greater than the capacity (`N`).
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
    /// let err = TinyAscii8::from_ascii_lossy(bytes, NonNullAscii::QuestionMark).unwrap_err();
    /// assert_eq!(err, CapacityLimitError);
    /// ```
    pub fn from_ascii_lossy(
        bytes: &[u8],
        replacement: NonNullAscii,
    ) -> Result<Self, CapacityLimitError> {
        const { assert_capacity::<N>() }

        if bytes.len() > N {
            return Err(CapacityLimitError);
        }

        let mut tiny = Self::new();
        // SAFETY: `tiny` length must be equal to bytes after copy.
        unsafe { tiny.set_len(bytes.len()) };

        // SAFETY:
        // - `dst` length is greater than or equal `src` length
        unsafe { crate::simd::fill_ascii(bytes, &mut tiny.buf, replacement) };

        Ok(tiny)
    }

    /// Converts this [`TinyAscii`] into one with a different capacity `M`.
    ///
    /// If `M` is greater than or equal to the current length, the content is
    /// preserved and the extra capacity is left unused. Otherwise, the content is
    /// truncated to its first `M` characters.
    ///
    /// # Examples
    ///
    /// Extending (`M` > `N`):
    ///
    /// ```
    /// # use tinyascii::TinyAscii;
    /// let s = TinyAscii::<20>::try_from_str("to be extended ...").unwrap();
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
    /// # use tinyascii::TinyAscii;
    /// let s = TinyAscii::<20>::try_from_str("to be truncated ...").unwrap();
    /// assert_eq!(s.capacity(), 20);
    ///
    /// let s = s.resize::<8>();
    /// assert_eq!(s.capacity(), 8);
    /// assert_eq!(&s, "to be tr");
    /// ```
    pub const fn resize<const M: usize>(self) -> TinyAscii<M> {
        const { assert_capacity::<M>() }

        let mut buf = const { [MaybeUninit::uninit(); M] };
        // use `M.min(N)` when const stable
        let cp_len = const { if N < M { N } else { M } };

        // SAFETY:
        // - `self.buf` is initialized with valid `NonNullAscii` values up to `final_len`
        // - `tiny.buf` can be written up to `final_len`
        // - `self.buf` and `tiny.buf` do not overlap
        unsafe {
            core::ptr::copy_nonoverlapping(self.buf.as_ptr(), buf.as_mut_ptr().cast(), cp_len);
        }

        let len = self.len();
        if len < M {
            buf[M - 1].write(encode_len(len));
        }

        TinyAscii { buf }
    }

    /// Returns this [`TinyAscii`]'s capacity, in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii;
    /// let s = TinyAscii::<10>::new();
    ///
    /// assert_eq!(s.capacity(), 10);
    /// ```
    #[inline(always)]
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns the length of this [`TinyAscii`], in bytes.
    ///
    /// Since every ASCII character occupies exactly 1 byte, this is also the
    /// number of characters.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii8;
    /// let a = TinyAscii8::try_from_str("foo.bar").unwrap();
    /// assert_eq!(a.len(), 7);
    /// ```
    pub const fn len(&self) -> usize {
        let maybe_len = self.last_byte();

        let len = if (maybe_len & LEN_MASK) == LEN_MASK {
            (maybe_len & !LEN_MASK) as usize
        } else {
            N
        };

        // SAFETY: `self.len()` is always less or equal to N, since N is the
        // total capacity.
        unsafe {
            core::hint::assert_unchecked(len <= N);
        }

        len
    }

    /// Returns the available capacity, in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii8;
    /// let s = TinyAscii8::try_from_str("foo").unwrap();
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
    /// let mut v = TinyAscii128::new();
    /// assert_eq!(v.len(), 0);
    /// assert!(v.is_empty());
    ///
    /// v.try_push(NonNullAscii::SmallA).unwrap();
    /// assert!(!v.is_empty());
    /// ```
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if this [`TinyAscii`] capacity is filled, and `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii;
    /// let mut v = TinyAscii::<6>::try_from_str("foobar").unwrap();
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
        let maybe_len = self.last_byte();
        maybe_len & LEN_MASK == 0
    }

    /// Converts a [`TinyAscii`] to a byte slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii8;
    /// let s = TinyAscii8::try_from_str("foobar").unwrap();
    /// assert_eq!(s.as_bytes(), b"foobar");
    /// ```
    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8] {
        // SAFETY:
        // - `self.len()` is always less that or equal N
        // - `buf[..n]` points to initialized valid ASCII data
        unsafe { core::slice::from_raw_parts(self.buf.as_ptr().cast(), self.len()) }
    }

    /// Returns the [`TinyAscii`] as a string slice `&str`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii16;
    /// let s = TinyAscii16::try_from_str("hello world").unwrap();
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
    /// let s = TinyAscii8::try_from_str("abc").unwrap();
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
    /// let mut s = TinyAscii8::try_from_str("111").unwrap();
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
    /// let mut s = TinyAscii4::try_from_str("baz").unwrap();
    ///
    /// s.clear();
    ///
    /// assert!(s.is_empty());
    /// assert_eq!(s.len(), 0);
    /// assert_eq!(s.capacity(), 4);
    /// ```
    #[inline(always)]
    pub const fn clear(&mut self) {
        self.buf[N - 1].write(LEN_MASK);
    }

    /// Shortens this [`TinyAscii`] to the specified length.
    ///
    /// If `new_len` is greater than or equal to the [`TinyAscii`]'s current length, this has no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii8;
    /// let mut s = TinyAscii8::try_from_str("hello").unwrap();
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

        // SAFETY: `new_len` is less than or equal previous `len`,
        // so `self.buf[..new_len]` remains valid and initialized.
        unsafe { self.set_len(new_len) };
    }

    /// Appends the given [`NonNullAscii`] to the end of this [`TinyAscii`].
    ///
    /// # Errors
    ///
    /// Returns a [`CapacityLimitError`] if there is no spare capacity.
    /// The string is not modified in this case.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii16, NonNullAscii};
    /// let mut s = TinyAscii16::try_from_str("hello").unwrap();
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
    /// let mut s: TinyAscii<5> = TinyAscii::try_from_str("hello").unwrap();
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

        // SAFETY: length increased by 1 as next available
        // slot is written with `ascii` byte.
        unsafe { self.set_len(len + 1) };

        self.buf[len].write(ascii.to_u8());

        Ok(())
    }

    /// Appends the given [`NonNullAscii`] slice to the end of this [`TinyAscii`].
    ///
    /// # Errors
    ///
    /// Returns a [`CapacityLimitError`] if the slice does not fit in the spare
    /// capacity. The string is not modified in this case.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii};
    /// let mut s = TinyAscii8::try_from_str("foo").unwrap();
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
    /// let mut s = TinyAscii4::try_from_str("foo").unwrap();
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
        if ascii.is_empty() {
            return Ok(());
        }

        let len = self.len();
        let final_len = len + ascii.len();

        if final_len > N {
            return Err(CapacityLimitError);
        }

        // SAFETY: increase length including new `ascii` slice copied bellow.
        unsafe { self.set_len(final_len) };

        // SAFETY:
        // - `ascii` is initialized with valid `NonNullAscii` bytes up to `ascii.len()`
        // - `self.buf` has available capacity >= `ascii.len()`
        // - `self.buf` and `ascii` do not overlap
        unsafe {
            core::ptr::copy_nonoverlapping(
                ascii.as_ptr().cast(),
                self.buf.as_mut_ptr().add(len),
                ascii.len(),
            );
        }

        Ok(())
    }

    /// Appends the given byte to the end of this [`TinyAscii`].
    ///
    /// # Errors
    ///
    /// Returns [`WriteAsciiError::InvalidAscii`] if the byte is not a valid non-null
    /// ASCII character, or [`WriteAsciiError::Capacity`] if there is no spare capacity.
    /// The string is not modified in either case.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii16;
    /// let mut s = TinyAscii16::try_from_str("correct").unwrap();
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
    /// let mut s = TinyAscii8::try_from_str("invalid").unwrap();
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
    /// let mut s: TinyAscii<5> = TinyAscii::try_from_str("hello").unwrap();
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
    /// # Errors
    ///
    /// Returns [`WriteAsciiError::Capacity`] if the slice does not fit in the spare
    /// capacity, or [`WriteAsciiError::InvalidAscii`] if it contains a null or
    /// non-ASCII byte. The string is not modified in either case.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii16;
    /// let mut s = TinyAscii16::try_from_str("correct").unwrap();
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
    /// let mut s = TinyAscii16::try_from_str("invalid").unwrap();
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
    /// let mut s: TinyAscii<5> = TinyAscii::try_from_str("hello").unwrap();
    ///
    /// let err = s.try_push_bytes(" world".as_bytes()).unwrap_err();
    ///
    /// assert_eq!(err, WriteAsciiError::Capacity(CapacityLimitError));
    /// assert_eq!(&s, "hello");
    /// ```
    pub fn try_push_bytes(&mut self, bytes: &[u8]) -> Result<(), WriteAsciiError> {
        if bytes.is_empty() {
            return Ok(());
        }

        let len = self.len();
        let final_len = len + bytes.len();

        if final_len > N {
            return Err(WriteAsciiError::Capacity(CapacityLimitError));
        }

        // SAFETY: `bytes.len()` is less than `N` as verified above
        if !unsafe { crate::simd::is_valid_tinyascii(bytes) } {
            return Err(WriteAsciiError::InvalidAscii);
        }

        // SAFETY: increase length including new `bytes` slice copied bellow.
        unsafe { self.set_len(final_len) };

        // SAFETY:
        // - `bytes` is initialized with valid `NonNullAscii` values
        // - `self.buf` can be written up to `bytes.len()`
        // - `bytes` and `self.buf` do not overlap
        unsafe {
            core::ptr::copy_nonoverlapping(
                bytes.as_ptr(),
                self.buf.as_mut_ptr().cast::<u8>().add(len),
                bytes.len(),
            );
        }

        Ok(())
    }

    /// Appends the given byte slice to the end of this [`TinyAscii`].
    ///
    /// The byte slice length must fit in the spare capacity. During the conversion, any
    /// null or non-ASCII byte is replaced with the provided replacement.
    ///
    /// # Errors
    ///
    /// Returns a [`CapacityLimitError`] if the slice does not fit in the spare
    /// capacity. The string is not modified in this case.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii16, NonNullAscii};
    /// let mut s = TinyAscii16::try_from_str("123").unwrap();
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
    /// let mut s: TinyAscii<5> = TinyAscii::try_from_str("hello").unwrap();
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
        if bytes.is_empty() {
            return Ok(());
        }

        let len = self.len();
        let final_len = len + bytes.len();

        if final_len > N {
            return Err(CapacityLimitError);
        }

        // SAFETY: increase length including new `bytes` slice filled bellow.
        unsafe { self.set_len(final_len) };

        // SAFETY:
        // - `dst` length is greater than or equal `src` length
        unsafe { crate::simd::fill_ascii(bytes, &mut self.buf[len..], replacement) };

        Ok(())
    }

    /// Appends the given string `&str` to the end of this [`TinyAscii`].
    ///
    /// # Errors
    ///
    /// Returns [`WriteAsciiError::Capacity`] if the `&str` does not fit in the spare
    /// capacity, or [`WriteAsciiError::InvalidAscii`] if it contains a null or
    /// non-ASCII character. The string is not modified in either case.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii16;
    /// let mut s = TinyAscii16::try_from_str("foo").unwrap();
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
    /// let mut s = TinyAscii16::try_from_str("non-ascii").unwrap();
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
    /// let mut s: TinyAscii<6> = TinyAscii::try_from_str("full").unwrap();
    ///
    /// let err = s.try_push_str(" house").unwrap_err();
    ///
    /// assert_eq!(err, WriteAsciiError::Capacity(CapacityLimitError));
    /// assert_eq!(&s, "full");
    /// ```
    pub fn try_push_str(&mut self, s: &str) -> Result<(), WriteAsciiError> {
        self.try_push_bytes(s.as_bytes())
    }

    /// Inserts a character into this [`TinyAscii`] at byte position `idx`.
    ///
    /// Makes space for the insertion by copying all bytes of
    /// `&self[idx..]` to new positions.
    ///
    /// Note that calling this in a loop can result in quadratic behavior.
    ///
    /// # Errors
    ///
    /// Returns [`InsertAsciiError::Capacity`] if there is no spare capacity, or
    /// [`InsertAsciiError::IndexOutOfBounds`] if `idx` is larger than the
    /// `TinyAscii`'s length. The string is not modified in either case.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii4, NonNullAscii};
    /// let mut s = TinyAscii4::new();
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
    /// let mut s = TinyAscii4::try_from_str("full").unwrap();
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
    /// # use tinyascii::{TinyAscii8, NonNullAscii, error::InsertAsciiError};
    /// let mut s = TinyAscii8::try_from_str("12345").unwrap();
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

        // SAFETY: increase length by 1 for new written `ascii`.
        unsafe { self.set_len(len + 1) };

        if idx < len {
            // SAFETY: Move the bytes starting from `idx` to their new location 1 byte
            // ahead. This is safe because sufficient capacity is ensured by checks
            // above.
            unsafe {
                let ptr = self.buf.as_mut_ptr();
                core::ptr::copy(
                    // `buf` is valid for reads from `idx` up to `len - idx`.
                    ptr.cast_const().add(idx),
                    // `buf` is valid for writes from `idx + 1` up to `len - idx`.
                    ptr.add(idx + 1),
                    len - idx,
                );
            }
        }

        self.buf[idx].write(ascii.to_u8());

        Ok(())
    }

    /// Inserts a non-null ASCII slice into this [`TinyAscii`] at byte position `idx`.
    ///
    /// Makes space for the insertion by copying all bytes of `&self[idx..]` to new
    /// positions.
    ///
    /// Note that calling this in a loop can result in quadratic behavior.
    ///
    /// # Errors
    ///
    /// Returns [`InsertAsciiError::Capacity`] if the slice does not fit in the spare
    /// capacity, or [`InsertAsciiError::IndexOutOfBounds`] if `idx` is larger than
    /// the `TinyAscii`'s length. The string is not modified in either case.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii};
    /// let mut s = TinyAscii8::try_from_str("bar").unwrap();
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
    /// let mut s = TinyAscii4::try_from_str("bar").unwrap();
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
    /// # use tinyascii::{TinyAscii8, NonNullAscii, error::InsertAsciiError};
    /// let mut s = TinyAscii8::try_from_str("bar").unwrap();
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

        if new_len != N {
            // SAFETY: required to set length marker since `self.buf` is not full.
            unsafe { self.set_len(new_len) };
        }

        // SAFETY: Move the bytes starting from `idx` to their new location `ascii.len()` bytes
        // ahead. This is safe because sufficient capacity is ensured.
        unsafe {
            let ptr = self.buf.as_mut_ptr();
            core::ptr::copy(
                // `buf` is valid for reads from `idx` up to `len - idx`.
                ptr.cast_const().add(idx),
                // `buf` is valid for writes from `idx + ascii.len()` up to `len - idx`.
                ptr.add(idx + ascii.len()),
                len - idx,
            );
        }

        // SAFETY: Copy the new slice into the vacated region if `idx != len`,
        // or into the spare capacity otherwise.
        unsafe {
            core::ptr::copy_nonoverlapping(
                ascii.as_ptr().cast(),
                self.buf.as_mut_ptr().add(idx),
                ascii.len(),
            );
        }

        Ok(())
    }

    /// Copies characters from the `src` range to the end of the string.
    ///
    /// # Errors
    ///
    /// Returns [`InsertRangeError::RangeOutOfBounds`] if the range has
    /// `start_bound > end_bound` or `end_bound > self.len()`, or
    /// [`InsertRangeError::Capacity`] if the copied characters exceed the spare
    /// capacity. The string is not modified in either case.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii32, NonNullAscii};
    /// let mut s = TinyAscii32::try_from_str("abcde").unwrap();
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
    /// # use tinyascii::{TinyAscii32, error::InsertRangeError};
    /// let mut s = TinyAscii32::try_from_str("abcde").unwrap();
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
    /// # use tinyascii::{TinyAscii8, error::{InsertRangeError, CapacityLimitError}};
    /// let mut s = TinyAscii8::try_from_str("abcde").unwrap();
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

        let new_len = len + count;
        if new_len != N {
            // SAFETY: set new length marker as `self.buf` is not full.
            unsafe { self.set_len(new_len) };
        }

        // SAFETY:
        // - Both pointers are offset by `range.start` and `len` and, as checked
        // above, both memory regions do not overlap.
        // - `count` is less or equal to the len of `source`, so source
        // is valid for `count` reads
        // - `count <= self.available_capacity()` guarantees that `dst.len() >= count`
        // so `dst` is valid for `count` writes
        unsafe {
            let ptr = self.buf.as_mut_ptr();
            core::ptr::copy_nonoverlapping(ptr.cast_const().add(range.start), ptr.add(len), count);
        }

        Ok(())
    }

    /// Removes the specified range in the [`TinyAscii`], and replaces it with the given
    /// `&[NonNullAscii]`. The given slice doesn't need to be the same length as the range.
    ///
    /// # Errors
    ///
    /// Returns [`InsertRangeError::RangeOutOfBounds`] if the range has
    /// `start_bound > end_bound` or `end_bound > self.len()`, or
    /// [`InsertRangeError::Capacity`] if the resulting length would exceed the
    /// capacity. The string is not modified in either case.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii32, NonNullAscii};
    /// let mut s = TinyAscii32::try_from_str("abcde").unwrap();
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
    /// let mut s = TinyAscii16::try_from_str("abcde").unwrap();
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
    /// let mut s = TinyAscii8::try_from_str("abcde").unwrap();
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

        if replace_with.len() > self.available_capacity() + range_len {
            return Err(InsertRangeError::Capacity(CapacityLimitError));
        }

        let new_len = len + replace_with.len() - range_len;

        // Adjust post-range data to its final place.
        let adjust_count = len - arange.end;
        if range_len != replace_with.len() && adjust_count != 0 {
            // SAFETY:
            // - `src` can be read up to `ajust_count`, as `ajust_count` is `len - arange.end` and `len <= N`.
            // - `dst` can be written up to `ajust_count`, as `new_len <= N` was checked above.
            // - `ptr::copy` supports overlapping ranges.
            unsafe {
                let ptr = self.buf.as_mut_ptr();
                core::ptr::copy(
                    ptr.cast_const().add(arange.end),
                    ptr.add(arange.start + replace_with.len()),
                    adjust_count,
                );
            }
        }

        // SAFETY:
        // - `dst` can be written up to `replace_with.len()` as checked above.
        // - `replace_with.as_ptr()` is valid for reads up to `replace_with.len()`.
        // - `self.buf` and `replace_with` do not overlap, as `self` is borrowed mutably.
        unsafe {
            core::ptr::copy_nonoverlapping(
                replace_with.as_ptr().cast::<u8>(),
                self.buf.as_mut_ptr().cast::<u8>().add(arange.start),
                replace_with.len(),
            );
        }

        if new_len != N {
            // SAFETY: set new length marker as `self.buf` is not full.
            unsafe { self.set_len(new_len) };
        }

        Ok(())
    }

    /// Removes the specified range in the [`TinyAscii`].
    ///
    /// # Errors
    ///
    /// Returns a [`RemoveRangeError`] if the range has `start_bound > end_bound`, or if
    /// `end_bound > self.len()`. The string is not modified in this case.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii32;
    /// let mut s = TinyAscii32::try_from_str("0123456789").unwrap();
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
    /// let mut s = TinyAscii32::try_from_str("abcde").unwrap();
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

        // SAFETY:
        // - `src` can be read up to `len - arange.end`
        // - `dst` can be written up to `len - arange.end`
        unsafe {
            let ptr = self.buf.as_mut_ptr();
            core::ptr::copy(
                ptr.cast_const().add(arange.end),
                ptr.add(arange.start),
                len - arange.end,
            );
        }

        // SAFETY: decrease length by the removed range length.
        unsafe { self.set_len(len - range_len) };

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
    /// # use tinyascii::{TinyAscii8, NonNullAscii};
    /// let mut s = TinyAscii8::try_from_str("abcde").unwrap();
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
    /// # use tinyascii::{TinyAscii8, NonNullAscii};
    /// let mut s = TinyAscii8::try_from_str("abcde").unwrap();
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
        if idx >= len {
            return None;
        }

        // SAFETY: Shift bytes from `idx..` to `idx - 1`
        // - all pointers come from the same `self.buf`
        // - `self.buf` is valid for reads and writes.
        let ascii = unsafe {
            let ptr_idx = self.buf.as_mut_ptr().cast::<u8>().add(idx);

            // copy it out, unsafely having a copy of the value on
            // the stack and in the vector at the same time.
            let byte = core::ptr::read(ptr_idx);

            // Shift everything down to fill in that spot.
            core::ptr::copy(ptr_idx.add(1), ptr_idx, len - idx - 1);
            byte
        };

        // SAFETY: decrease length by 1 given `len >= 1`.
        unsafe { self.set_len(len - 1) };

        // SAFETY: `ascii` is a valid non-null ASCII
        Some(unsafe { NonNullAscii::from_u8_unchecked(ascii) })
    }

    /// Removes the last [`NonNullAscii`] character from this [`TinyAscii`] and returns it,
    /// or [`None`] if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii32, NonNullAscii};
    /// let mut s = TinyAscii32::try_from_str("101").unwrap();
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

        let idx = self.len() - 1;

        // SAFETY: `buf[idx]` is initialized as every byte in `buf[..len]`.
        let code = unsafe { self.buf[idx].assume_init() };
        // SAFETY: decrease length by 1 given `len >= 1`.
        unsafe { self.set_len(idx) };

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
    /// let mut s = TinyAscii16::try_from_str("f_o_ob_ar").unwrap();
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
    /// let mut s = TinyAscii8::try_from_str("abcde").unwrap();
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
        self.retain_mut(|c| f(*c));
    }

    /// Retains only the characters specified by the predicate, passing a
    /// mutable reference to it.
    ///
    /// In other words, remove all ASCII characters `c` such that `f(&mut c)` returns `false`.
    /// This method operates in place, visiting each character exactly once in the
    /// original order, and preserves the order of the retained characters.
    ///
    /// Characters modified by `f` keep their new value if they are retained.
    /// If `f` panics, the character being visited at that moment is kept
    /// unmodified.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii16, NonNullAscii};
    /// let mut s = TinyAscii16::try_from_str("f_o_ob_ar").unwrap();
    ///
    /// s.retain_mut(|c| {
    ///     if *c == '_' {
    ///         false
    ///     } else {
    ///         if *c == 'o' {
    ///             *c = NonNullAscii::Digit0;
    ///         }
    ///         true
    ///     }
    /// });
    ///
    /// assert_eq!(&s, "f00bar");
    /// ```
    pub fn retain_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut NonNullAscii) -> bool,
    {
        // Invariants while the guard is alive:
        // - `buf[..idx - deleted]` holds the retained characters,
        // - `buf[idx..original_len]` holds the not yet visited characters,
        // - `buf[idx - deleted..idx]` is a gap of stale (already visited) bytes.
        //
        // The length marker (`buf[N - 1]`) is only written on drop, so when the
        // string is full, the last content byte is intact while being visited.
        struct BackshiftOnDrop<'a, const N: usize> {
            tiny: &'a mut TinyAscii<N>,
            idx: usize,
            deleted: usize,
            original_len: usize,
        }

        impl<const N: usize> Drop for BackshiftOnDrop<'_, N> {
            fn drop(&mut self) {
                if self.deleted == 0 {
                    // Nothing changed, so the length marker is still valid.
                    return;
                }

                // Move the unvisited tail over the gap. This also runs if the
                // predicate panics, so no character is lost or duplicated.
                //
                // SAFETY:
                // - `idx <= original_len <= N`, so the source is valid for
                //   reads of `original_len - idx` bytes.
                // - `deleted <= idx`, so the destination is within the buffer
                //   and ends at `original_len - deleted <= N`.
                // - `ptr::copy` supports overlapping ranges.
                unsafe {
                    let ptr = self.tiny.buf.as_mut_ptr();
                    core::ptr::copy(
                        ptr.add(self.idx),
                        ptr.add(self.idx - self.deleted),
                        self.original_len - self.idx,
                    );
                }

                // SAFETY: `deleted > 0`, so `new_len < N` and the marker must be written.
                unsafe { self.tiny.set_len(self.original_len - self.deleted) };
            }
        }

        fn process_loop<F, const N: usize, const DELETED: bool>(
            original_len: usize,
            f: &mut F,
            g: &mut BackshiftOnDrop<'_, N>,
        ) where
            F: FnMut(&mut NonNullAscii) -> bool,
        {
            while g.idx < original_len {
                // SAFETY: `g.idx < original_len <= N`, and the byte is a valid
                // initialized `NonNullAscii`, as all visited bytes are part of
                // the content.
                let mut ascii = unsafe {
                    let byte = g.tiny.buf.get_unchecked(g.idx).assume_init();
                    NonNullAscii::from_u8_unchecked(byte)
                };

                if !f(&mut ascii) {
                    // Advance only after `f` returned, so a panic in `f` keeps
                    // the current character in the unvisited tail.
                    g.idx += 1;
                    g.deleted += 1;
                    if DELETED {
                        continue;
                    }
                    // First deletion found, switch to the shifting loop.
                    return;
                }

                // Write back, as `f` may have modified the character. When
                // `DELETED` is false this is the character's own slot.
                //
                // SAFETY: `g.deleted <= g.idx < N`, so the destination is in bounds.
                unsafe {
                    g.tiny
                        .buf
                        .get_unchecked_mut(g.idx - g.deleted)
                        .write(ascii.to_u8());
                }
                g.idx += 1;
            }
        }

        let original_len = self.len();
        let mut g = BackshiftOnDrop {
            tiny: self,
            idx: 0,
            deleted: 0,
            original_len,
        };

        // Stage 1: nothing deleted yet, so no data needs to be moved.
        process_loop::<F, N, false>(original_len, &mut f, &mut g);

        // Stage 2: at least one deletion, retained characters are shifted back.
        process_loop::<F, N, true>(original_len, &mut f, &mut g);

        drop(g);
    }

    /// Removes and returns the last character if the predicate returns `true`,
    /// or [`None`] if the predicate returns `false` or the string is empty.
    ///
    /// The predicate receives a mutable reference to the last character, which
    /// it may modify even when it returns `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii};
    /// let mut s = TinyAscii8::try_from_str("abc1").unwrap();
    ///
    /// assert_eq!(s.pop_if(|c| c.is_digit()), Some(NonNullAscii::Digit1));
    /// assert_eq!(s.pop_if(|c| c.is_digit()), None);
    /// assert_eq!(&s, "abc");
    ///
    /// s.clear();
    /// assert_eq!(s.pop_if(|_| true), None);
    /// ```
    pub fn pop_if<F>(&mut self, pred: F) -> Option<NonNullAscii>
    where
        F: FnOnce(&mut NonNullAscii) -> bool,
    {
        let last = self.as_mut_ascii().last_mut()?;

        if pred(last) { self.pop() } else { None }
    }

    /// Fills `buf[start..end]` with `value`.
    ///
    /// # Safety
    ///
    /// `start <= end <= N`.
    #[inline]
    unsafe fn fill_range(&mut self, start: usize, end: usize, value: NonNullAscii) {
        // SAFETY: `start <= end <= N <= MAX_LEN`, as ensured by the caller.
        let dst = unsafe { self.buf.get_unchecked_mut(start..end) };

        // SAFETY: `dst.len() <= MAX_LEN`
        unsafe { crate::simd::fill_value(dst, value) };
    }

    /// Resizes this [`TinyAscii`] in place so that its length is equal to `new_len`.
    ///
    /// If `new_len` is greater than the current length, the string is extended
    /// with copies of `value`. Otherwise, it is truncated.
    ///
    /// # Errors
    ///
    /// Returns a [`CapacityLimitError`] if `new_len` is greater than the capacity
    /// of this `TinyAscii`. The string is not modified in this case.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii};
    /// let mut s = TinyAscii8::try_from_str("ab").unwrap();
    ///
    /// s.try_resize(5, NonNullAscii::Digit0).unwrap();
    /// assert_eq!(&s, "ab000");
    ///
    /// s.try_resize(1, NonNullAscii::Digit0).unwrap();
    /// assert_eq!(&s, "a");
    /// ```
    ///
    /// Not enough capacity:
    ///
    /// ```
    /// # use tinyascii::{TinyAscii4, NonNullAscii, error::CapacityLimitError};
    /// let mut s = TinyAscii4::try_from_str("ab").unwrap();
    ///
    /// let err = s.try_resize(5, NonNullAscii::Digit0).unwrap_err();
    ///
    /// assert_eq!(err, CapacityLimitError);
    /// assert_eq!(&s, "ab");
    /// ```
    pub fn try_resize(
        &mut self,
        new_len: usize,
        value: NonNullAscii,
    ) -> Result<(), CapacityLimitError> {
        if new_len > N {
            return Err(CapacityLimitError);
        }

        let len = self.len();
        if new_len <= len {
            self.truncate(new_len);
            return Ok(());
        }

        // The marker is written first: when `new_len == N` the fill below
        // overwrites it with the last content byte, which clears the marker.
        //
        // SAFETY: `buf[len..new_len]` is filled just below.
        unsafe { self.set_len(new_len) };

        // SAFETY: `len < new_len <= N`.
        unsafe { self.fill_range(len, new_len, value) };

        Ok(())
    }

    /// Fills the spare capacity of this [`TinyAscii`] with `value`, making it full.
    ///
    /// Existing content is left intact. This differs from [`slice::fill`], which
    /// overwrites every element: only the unused part (`len()..capacity()`)
    /// is written, and the length grows to the capacity.
    /// If the string is already full, this has no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, NonNullAscii};
    /// let mut s = TinyAscii8::try_from_str("abc").unwrap();
    ///
    /// s.fill_remaining(NonNullAscii::HyphenMinus);
    ///
    /// assert_eq!(&s, "abc-----");
    /// assert!(s.is_full());
    ///
    /// s.fill_remaining(NonNullAscii::Digit0);
    /// assert_eq!(&s, "abc-----");
    /// ```
    pub fn fill_remaining(&mut self, value: NonNullAscii) {
        let len = self.len();
        if len == N {
            return;
        }

        // The length is implicitly updated: `buf[N - 1]` becomes a content
        // byte (< 0x80), which clears the length marker.
        //
        // SAFETY: `len < N`.
        unsafe { self.fill_range(len, N, value) };
    }

    /// Converts this [`TinyAscii`] into a boxed string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii8;
    /// let s = TinyAscii8::try_from_str("abc").unwrap();
    ///
    /// let boxed: Box<str> = s.into_boxed_str();
    ///
    /// assert_eq!(&*boxed, "abc");
    /// ```
    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn into_boxed_str(self) -> alloc::boxed::Box<str> {
        alloc::boxed::Box::from(self.as_str())
    }

    /// Converts this [`TinyAscii`] into a byte vector with its content.
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii8;
    /// let s = TinyAscii8::try_from_str("abc").unwrap();
    ///
    /// assert_eq!(s.into_bytes(), b"abc");
    /// ```
    #[cfg(feature = "alloc")]
    #[must_use]
    pub fn into_bytes(self) -> alloc::vec::Vec<u8> {
        self.as_bytes().to_vec()
    }

    /// Decomposes a [`TinyAscii`] into its raw components: `(byte_array, length)`.
    ///
    /// Returns the array with the ASCII string data, and the length of the string (in bytes).
    /// These are the same arguments in the same order as the arguments to [`from_parts`].
    /// Every byte of the array past the returned length is zero.
    ///
    /// [`from_parts`]: TinyAscii::from_parts
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::{TinyAscii8, TinyAscii};
    /// let s = TinyAscii8::try_from_str("foobar").unwrap();
    ///
    /// let (mut arr, len) = s.into_parts();
    ///
    /// assert_eq!(len, 6);
    /// assert_eq!(arr[6..], [0, 0]);
    ///
    /// arr[3] = b'B';
    ///
    /// let rebuilt = unsafe { TinyAscii::from_parts(arr, len) };
    ///
    /// assert_eq!(&rebuilt, "fooBar");
    /// ```
    pub fn into_parts(mut self) -> ([u8; N], usize) {
        let len = self.len();

        // Zero everything past `len`, including the length marker in the last
        // byte, which is not part of the content.
        self.buf[len..].fill(MaybeUninit::zeroed());

        // TODO: use `array_assume_init` when stable
        (
            // SAFETY: `self.buf` was initialized from `0..len` and `len..N`
            // was just zero-filled, so it is completely initialized.
            unsafe { core::mem::transmute_copy::<[MaybeUninit<u8>; N], [u8; N]>(&self.buf) },
            len,
        )
    }

    /// Creates a [`TinyAscii`] directly from a byte array and a length.
    ///
    /// # Safety
    ///
    /// Caller must ensure that:
    ///
    /// * `len` is less than or equal to `N` (the array length).
    /// * The first `len` bytes are valid [`NonNullAscii`] characters.
    ///
    /// These requirements are always upheld by any `arr` and `len` pair
    /// returned from [`into_parts`]. Other sources are allowed if the invariants
    /// are upheld.
    ///
    /// Violating these requirements **will** cause UB.
    ///
    /// [`into_parts`]: TinyAscii::into_parts
    ///
    /// # Examples
    ///
    /// ```
    /// # use tinyascii::TinyAscii128;
    /// let ascii = TinyAscii128::try_from_str("bar").unwrap();
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
    pub unsafe fn from_parts(mut arr: [u8; N], len: usize) -> Self {
        const { assert_capacity::<N>() }

        if len != N {
            arr[N - 1] = encode_len(len);
        }

        Self {
            buf: arr.map(MaybeUninit::new),
        }
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
