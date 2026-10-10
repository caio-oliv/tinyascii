#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod ascii;
mod ascii_impls;
mod tinyascii;
mod tinyascii_impls;

pub mod error;

pub use ascii::*;
pub use tinyascii::*;

#[cfg(feature = "dep_tinystr")]
mod tinystr;

#[cfg(feature = "dep_ascii")]
mod dep_ascii;

#[cfg(test)]
mod ascii_tests;
