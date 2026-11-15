//! Fixed-capacity, inline ASCII strings.
//!
//! [`TinyAscii<N>`] stores up to `N` non-null ASCII characters in a `[u8; N]`
//! on the stack, with no allocation and no separate length field. The capacity
//! `N` must be in `1..=`[`MAX_LEN`] (128), which is checked at compile time.
//! Because the content is always valid ASCII, a [`TinyAscii`] can be viewed as
//! a `&str`, as bytes, or as a slice of [`NonNullAscii`] at no cost.
//!
//! Operations that may not fit or may receive invalid input never panic: they
//! return a `Result` with one of the types in the [`error`] module.
//!
//! # Examples
//!
//! ```
//! use tinyascii::{TinyAscii16, NonNullAscii};
//!
//! let mut s = TinyAscii16::try_from_str("hello").unwrap();
//! s.try_push(NonNullAscii::Space).unwrap();
//! s.try_push_str("world").unwrap();
//!
//! assert_eq!(&s, "hello world");
//! assert_eq!(s.len(), 11);
//! assert_eq!(s.available_capacity(), 5);
//! ```
//!
//! # Features
//!
//! This crate is `no_std` by default.
//!
//! - `alloc`: conversions from and to `String`, `Vec<u8>` and `Box<str>`.
//! - `std`: implies `alloc`, and adds `std`-only trait impls (e.g. for `Path`).
//! - `dep_tinystr`: conversions to and from the [`tinystr`] crate.
//! - `dep_ascii`: conversions to and from the [`ascii`] crate.
//!
//! [`tinystr`]: https://docs.rs/tinystr
//! [`ascii`]: https://docs.rs/ascii

#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod ascii;
mod ascii_impls;
mod interop;
mod simd;
mod tinyascii;
mod tinyascii_impls;

pub mod error;

pub use ascii::*;
pub use tinyascii::*;

#[cfg(test)]
mod tests;
