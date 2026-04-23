#!/usr/bin/env bash
# Uses plain `cargo build` on native macOS runners (full Apple SDK present).
# Uses `cargo zigbuild` on Linux runners for all other cross targets.
#
# Required env vars:
#   TARGET     — Rust target triple, e.g. "x86_64-unknown-linux-gnu"
#   TAG_NAME   — e.g. "v1.0.0" (used only for the output filename)

set -euo pipefail

: "${TARGET:?TARGET env var is required}"
: "${TAG_NAME:?TAG_NAME env var is required}"


echo "▶ Building target='$TARGET' tag='$TAG_NAME'"

# ── Per-target naming conventions ────────────────────────────────────────────
case "$TARGET" in
  *windows*)
    LIB_PREFIX=""
    LIB_EXT=".dll"
    ;;
  *apple*)
    LIB_PREFIX="lib"
    LIB_EXT=".dylib"
    ;;
  *)  # Linux, FreeBSD
    LIB_PREFIX="lib"
    LIB_EXT=".so"
    ;;
esac

# Patch Cargo.toml to also build staticlib and cdylib
# macOS requires an empty string argument for the -i flag (BSD sed vs GNU sed)
if [[ "$(uname)" == "Darwin" ]]; then
  sed -i '' 's/crate-type = \["rlib"\]/crate-type = \["rlib", "cdylib", "staticlib"\]/' Cargo.toml
else
  sed -i 's/crate-type = \["rlib"\]/crate-type = \["rlib", "cdylib", "staticlib"\]/' Cargo.toml
fi

# ── Build ─────────────────────────────────────────────────────────────────────
# On macOS runners, aws-lc-sys needs the real Apple SDK (CoreServices.h etc.)
# which zigbuild's bundled minimal SDK doesn't include. Build natively there.
# On Linux runners, use zigbuild for all cross targets.
if [[ "$(uname)" == "Darwin" ]]; then
  cargo build --release --target "$TARGET"
else
  cargo zigbuild --release --target "$TARGET"
fi

# ── Collect binary into a staging dir ────────────────────────────────────────
STAGING="staging/${TARGET}"
mkdir -p "$STAGING" artifacts

cp "target/${TARGET}/release/${LIB_PREFIX}spf${LIB_EXT}" "$STAGING/"
cp spf.h "$STAGING/"
cp LICENSE-APACHE "$STAGING/"

# ── Package ───────────────────────────────────────────────────────────────────
TARBALL="artifacts/spf.${TAG_NAME}.${TARGET}.tar.gz"
tar -czf "$TARBALL" -C staging "${TARGET}"
rm -rf staging

echo "✅ Artifact ready: $TARBALL"