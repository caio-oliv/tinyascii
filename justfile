set default-list

@build-matrix:
    cargo build --no-default-features
    cargo build --features alloc
    cargo build --features std
    cargo build --features dep_tinystr
    cargo build --features dep_ascii

[group('test')]
[positional-arguments]
@test:
    cargo test --all-features "$@"

[group('test')]
[positional-arguments]
[env("RUSTFLAGS", "-C target_feature=+avx2")]
@test-avx2:
    @just test "$@"

[group('test')]
@test-cov:
    cargo llvm-cov clean
    cargo +nightly llvm-cov test --doctests --all-features --html

[group('test')]
@test-cov-open:
    python -m http.server 9010 -d target/llvm-cov/html

@bench-compare feature base='main':
    git switch {{ base }}
    cargo bench --bench tinyascii_ctor_bench -- --save-baseline {{ base }}
    cargo bench --bench tinyascii_mutation_bench -- --save-baseline {{ base }}
    cargo bench --bench tinyascii_range_bench -- --save-baseline {{ base }}

    git switch {{ feature }}
    cargo bench --bench tinyascii_ctor_bench -- --save-baseline {{ feature }}
    cargo bench --bench tinyascii_mutation_bench -- --save-baseline {{ feature }}
    cargo bench --bench tinyascii_range_bench -- --save-baseline {{ feature }}

    # compare benchs
    cargo bench --bench tinyascii_ctor_bench -- --load-baseline {{ feature }} --baseline {{ base }}
    cargo bench --bench tinyascii_mutation_bench -- --load-baseline {{ feature }} --baseline {{ base }}
    cargo bench --bench tinyascii_range_bench -- --load-baseline {{ feature }} --baseline {{ base }}

@bench-open:
    python -m http.server 9020 -d target/criterion

@bench-clear:
    rm -rf target/criterion
