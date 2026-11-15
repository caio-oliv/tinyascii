# tinyascii

Fixed-capacity, inline ASCII strings for Rust. No allocation, `no_std` by default.

## Features

- **Inline storage**: `TinyAscii<N>` lives entirely on the stack in a `[u8; N]` (`N` in `1..=128`, checked at compile time).
- **Zero overhead length**: no separate length field, the length is encoded in the buffer itself.
- **Free views**: borrow the content as `&str`, `&[u8]` or `&[NonNullAscii]` at no cost.
- **Never panics**: operations that may not fit or receive invalid input return a `Result` with a descriptive error type.

## How it works

A `TinyAscii<N>` stores up to `N` non-null ASCII characters in a `[u8; N]`. Since NUL is never part of the content, the last byte doubles as the length marker: if its top bit is set, the low 7 bits hold the length; otherwise the buffer is full and that byte is real content. This is why `N` is capped at 128.

Because the content is always valid ASCII, a `TinyAscii` can be viewed as a `&str`, as bytes, or as a slice of `NonNullAscii`.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tinyascii = "0.1.0"
```

### Cargo features

This crate is `no_std` by default.

| Feature       | Description                                                         |
| ------------- | ------------------------------------------------------------------- |
| `alloc`       | Conversions from and to `String`, `Vec<u8>` and `Box<str>`.         |
| `std`         | Implies `alloc`, and adds `std`-only trait impls (e.g. for `Path`). |
| `dep_tinystr` | Conversions to and from the `tinystr` crate.                        |
| `dep_ascii`   | Conversions to and from the `ascii` crate.                          |

## Quick Start

### Basic usage

```rust
use tinyascii::{TinyAscii16, NonNullAscii};

let mut s = TinyAscii16::try_from_str("hello").unwrap();
s.try_push(NonNullAscii::Space).unwrap();
s.try_push_str("world").unwrap();

assert_eq!(&s, "hello world");
assert_eq!(s.len(), 11);
assert_eq!(s.available_capacity(), 5);
```

### Handling errors

```rust
use tinyascii::TinyAscii4;

// Too long for the capacity.
assert!(TinyAscii4::try_from_str("hello").is_err());

// Not ASCII.
assert!(TinyAscii4::try_from_str("héy").is_err());

// NUL is not allowed.
assert!(TinyAscii4::try_from_bytes(b"a\0b").is_err());
```

### Borrowing the content

```rust
use tinyascii::TinyAscii8;

let s = TinyAscii8::try_from_str("tiny").unwrap();

let text: &str = s.as_str();
let bytes: &[u8] = s.as_bytes();
assert_eq!(text, "tiny");
assert_eq!(bytes, b"tiny");
```

## Documentation

The full API documentation is available at [docs.rs/tinyascii](https://docs.rs/tinyascii/latest/tinyascii/).

## License

Licensed under either of [Apache 2.0 License](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
