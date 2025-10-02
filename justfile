set dotenv-load := true
set shell := ["bash", "-euo", "pipefail", "-c"]

cargo +args='':
    cargo {{args}}

pre-commit:
    @just cargo fmt --all --check
    @just cargo check --all-targets --all-features --workspace
    @just cargo clippy --all-targets --all-features --workspace -- -D warnings -D clippy::all
    @just cargo nextest run --all-features --all-targets --workspace
    @just cargo test --doc

pre-push:
    @just cargo doc --no-deps --document-private-items --all-features --workspace --examples
    @just cargo audit
