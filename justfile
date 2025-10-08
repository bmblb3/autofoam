default:
    @just --list

[group('ci')]
[parallel]
ci: lint test doctest doc

[group('ci')]
[parallel]
lint:
    cargo fmt --all --check
    cargo check --all-targets --all-features --workspace
    cargo clippy --all-targets --all-features --workspace -- -D warnings -D clippy::all

[group('ci')]
test:
    cargo test --all-features --all-targets --workspace

[group('ci')]
doctest:
    cargo test --doc

[group('ci')]
doc:
    cargo doc --no-deps --document-private-items --all-features --workspace --examples

[group('build')]
build:
    nix build

[group('release')]
pre-release:
    release-plz update

[group('release')]
release: build
    #!/usr/bin/env bash
    set -euo pipefail
    PACKAGE_NAME=$(cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].name')
    PACKAGE_VERSION=$(cargo metadata --no-deps --format-version=1 | jq -r '.packages[0].version')
    ASSET_NAME="${PACKAGE_NAME}-v${PACKAGE_VERSION}-x86_64-unknown-linux-musl"
    git push
    release-plz release --git-token="$(gh auth token)"
    TMPDIR=$(mktemp -d)
    trap "rm -rf '$TMPDIR'" EXIT
    tar -czf "$TMPDIR/${ASSET_NAME}.tar.gz" -C ./result/bin .
    (cd "$TMPDIR" && sha256sum "${ASSET_NAME}.tar.gz" > "${ASSET_NAME}.sha256")
    gh release upload "v${PACKAGE_VERSION}" \
        "$TMPDIR/${ASSET_NAME}.tar.gz" \
        "$TMPDIR/${ASSET_NAME}.sha256"
