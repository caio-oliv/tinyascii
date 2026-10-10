default:
	@just --list

build-matrix:
	cargo build --no-default-features
	cargo build --features alloc
	cargo build --features std

test:
	cargo test --all-features

test-cov:
	cargo llvm-cov clean
	cargo +nightly llvm-cov test --doctests --all-features --html

test-cov-open:
	python -m http.server 9010 -d target/llvm-cov/html
