[parallel]
ci: audit lint test doctest doc

audit:
    command -v cargo-audit >/dev/null 2>&1 && cargo audit || echo "skipping audit"

[parallel]
lint:
    cargo fmt --all --check
    cargo check --all-targets --all-features --workspace
    cargo clippy --all-targets --all-features --workspace -- -D warnings -D clippy::all

test:
    cargo test --all-features --all-targets --workspace

doctest:
    cargo test --doc

doc:
    cargo doc --no-deps --document-private-items --all-features --workspace --examples

pre-release:
    release-plz update --git-token=$(gh auth token)

package_name := `cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].name'`
package_version := `cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].version'`
asset_name := package_name + "-v" + package_version + "-x86_64-unknown-linux-musl"
release: build archive
    release-plz release
    gh release create v{{package_version}} --verify-tag {{asset_name}}.tar.gz {{asset_name}}.sha256
    just clean-assets

build:
    nix build

archive:
    tar -czf {{asset_name}}.tar.gz -C ./result/bin .
    sha256sum {{asset_name}}.tar.gz > {{asset_name}}.sha256

clean-assets:
    rm -f {{asset_name}}.tar.gz {{asset_name}}.sha256
    rm -rf result
