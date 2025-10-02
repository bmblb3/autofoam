[parallel]
ci: lint test doctest doc

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
    release-plz update

package_name := `cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].name'`
package_version := `cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].version'`
asset_name := package_name + "-v" + package_version + "-x86_64-unknown-linux-musl"
release: build
    git push
    release-plz release --git-token=$(gh auth token)
    @TMPDIR=$(mktemp -d) && \
        tar -czf "$TMPDIR/{{asset_name}}.tar.gz" -C ./result/bin . && \
        (cd "$TMPDIR" && sha256sum "{{asset_name}}.tar.gz" > "{{asset_name}}.sha256") && \
        gh release upload v{{package_version}} \
            "$TMPDIR/{{asset_name}}.tar.gz" \
            "$TMPDIR/{{asset_name}}.sha256"

build:
    nix build
