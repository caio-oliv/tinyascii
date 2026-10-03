#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(any(feature = "alloc", feature = "std"))]
extern crate alloc;

mod ascii;
mod ascii_impls;
mod tinyascii;
mod tinyascii_impls;

pub mod error;

pub use ascii::*;
pub use tinyascii::*;

#[cfg(test)]
mod ascii_tests;
