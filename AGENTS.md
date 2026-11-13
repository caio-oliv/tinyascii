# AGENTS.md — tinyascii

## Commands

Task runner is `just` (see `justfile`):

```
just test            # cargo test --all-features, run twice: default and RUSTFLAGS="-C target_feature=+avx2" (extra args are forwarded)
just test-miri       # nightly miri (--lib, strict provenance), also run twice (default + avx2); needs miri
just doc             # cargo doc --all-features
just build-matrix    # builds every feature combo (none, alloc, std, dep_tinystr, dep_ascii)
just test-cov        # cargo +nightly llvm-cov test --doctests --all-features --html (needs cargo-llvm-cov)
just test-cov-open   # serves target/llvm-cov/html on :9010
cargo test --all-features <name>   # single test by name filter
cargo test --all-features --test tinyascii_proptest   # property tests only (tests/)
cargo bench          # criterion benches in benches/tinyascii_*_bench.rs
just bench-compare <feature-branch> [base=main]   # switches branches (!), saves criterion baselines and compares
```

Always test with `--all-features`; the interop tests are gated behind `dep_tinystr` / `dep_ascii`. SIMD code paths are chosen at compile time, so the AVX2 impl is only exercised with `+avx2` (which `just test` does).

## Architecture

A `no_std` (by default) crate, edition 2024, providing a fixed-capacity inline ASCII string.

- `Ascii` / `NonNullAscii` (`src/ascii.rs`, `ascii_impls.rs`): `#[repr(u8)]` enums of the 128 ASCII code points (the latter excludes NUL). `NonNullAscii` is the element type of the string.
- `TinyAscii<const N: usize>` (`src/tinyascii.rs`, `tinyascii_impls.rs`): backed by `[u8; N]`, N in 1..=`MAX_LEN` (128), enforced at compile time via `assert_capacity::<N>()`. Aliases `TinyAscii4/8/16/32/64/128`.
  - **Length encoding**: no separate length field. Content is NUL-free, and the last byte `buf[N-1]` doubles as the length marker. If its top bit (`LEN_MASK = 0x80`) is set, the low 7 bits are the length; otherwise the buffer is full (`len == N`) and that byte is real content (always < 0x80). So `is_full()` is "top bit clear" and `len()` must go through that decode. Any mutation must keep this invariant.
  - Heavy use of `const fn` and `unsafe` (`from_utf8_unchecked`, `assert_unchecked`); each unsafe site needs a `// SAFETY:` comment.
- `src/simd/` (`mod.rs`, `avx2.rs`, `scalar.rs`): internal unsafe helpers (`is_valid_tinyascii`, `copy_valid_tinyascii`, `fill_ascii`) used by `TinyAscii` constructors/mutators. `mod.rs` dispatches via `cfg_select!` on `target_feature = "avx2"`; both impls must keep identical safety contracts (e.g. `src.len() <= MAX_LEN`).
- `src/error.rs`: error types (`CapacityLimitError`, `WriteAsciiError`, `InsertAsciiError`, `InsertRangeError`, `RemoveRangeError`).
- `src/interop/`: optional conversions to/from `tinystr` and `ascii` crates, behind `dep_tinystr` / `dep_ascii` features, with `*_test.rs` siblings.
- Features: `alloc`, `std` (implies `alloc`), `dep_tinystr`, `dep_ascii`.

Tests: unit tests in `src/tests/` (incl. mutation and trait-impl suites), proptests in `tests/tinyascii_proptest.rs` (regressions persisted in `tests/*.proptest-regressions`), doctests on `TinyAscii` methods.

## Lint rules (Cargo.toml) that bite

- **deny**: `unwrap_used`, `get_unwrap` (use `expect` or handle errors), `undocumented_unsafe_blocks`, `missing_safety_doc`, `missing_panics_doc` (document `# Panics`/`# Safety`), `transmute_undefined_repr`, `std_instead_of_core`, `std_instead_of_alloc`, `alloc_instead_of_core` (import from `core`/`alloc`, never `std`).
- **warn**: `missing_docs` / `missing_crate_level_docs` (rustc), `missing_errors_doc` (document `# Errors` on fallible pub fns), plus several pedantic clippy ones (`manual_let_else`, `match_same_arms`, `semicolon_if_nothing_returned`, ...). `as_conversions` and `unsafe_op_in_unsafe_fn` are not configured, but still prefer `From`/`try_from` over `as` and wrap unsafe ops in explicit `unsafe {}` blocks; doc paragraphs should end with punctuation.
