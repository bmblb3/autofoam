[parallel]
ci: bootstrap audit lint test doctest doc

audit:
    cargo audit

[parallel]
lint:
    cargo fmt --all --check
    cargo check --all-targets --all-features --workspace
    cargo clippy --all-targets --all-features --workspace -- -D warnings -D clippy::all

test:
    cargo nextest run --all-features --all-targets --workspace

doctest:
    cargo test --doc

doc:
    cargo doc --no-deps --document-private-items --all-features --workspace --examples

bootstrap:
    if ! command -v cargo-audit > /dev/null; then cargo install cargo-audit; fi
    if ! command -v cargo-nextest > /dev/null; then cargo install cargo-nextest; fi
