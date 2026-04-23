#!/usr/bin/env bash
# Uses plain `cargo build` on native macOS runners.
# Uses `cargo zigbuild` on Linux runners for all other cross targets.

set -euo pipefail

: "${TARGET:?TARGET env var is required}"
: "${TAG_NAME:?TAG_NAME env var is required}"

echo "▶ Building target='$TARGET' tag='$TAG_NAME'"

# By default, we build both. musl targets will set this to false.
BUILD_CDYLIB=true

# ── Per-target naming conventions ────────────────────────────────────────────
case "$TARGET" in
  *windows-msvc*)
    LIB_PREFIX=""
    LIB_EXT=".dll"
    STATIC_LIB_PREFIX=""
    STATIC_LIB_EXT=".lib"
    ;;
  *windows-gnu*)
    LIB_PREFIX=""
    LIB_EXT=".dll"
    STATIC_LIB_PREFIX="lib"
    STATIC_LIB_EXT=".a"
    ;;
  *apple*)
    LIB_PREFIX="lib"
    LIB_EXT=".dylib"
    STATIC_LIB_PREFIX="lib"
    STATIC_LIB_EXT=".a"
    ;;
  *musl*)
    BUILD_CDYLIB=true
    LIB_PREFIX="lib"
    LIB_EXT=".so"
    STATIC_LIB_PREFIX="lib"
    STATIC_LIB_EXT=".a"
    ;;
  *)  # Standard Linux, FreeBSD
    LIB_PREFIX="lib"
    LIB_EXT=".so"
    STATIC_LIB_PREFIX="lib"
    STATIC_LIB_EXT=".a"
    ;;
esac

# Patch Cargo.toml based on whether we can build a cdylib
CRATE_TYPES="\"rlib\", \"staticlib\""
if [ "$BUILD_CDYLIB" = true ]; then
  CRATE_TYPES="\"rlib\", \"cdylib\", \"staticlib\""
fi

# macOS sed vs GNU sed
if [[ "$(uname)" == "Darwin" ]]; then
  sed -i '' "s/crate-type = \[\"rlib\"\]/crate-type = \[$CRATE_TYPES\]/" Cargo.toml
else
  sed -i "s/crate-type = \[\"rlib\"\]/crate-type = \[$CRATE_TYPES\]/" Cargo.toml
fi

# ── Build ─────────────────────────────────────────────────────────────────────
if [[ "$(uname)" == "Darwin" ]]; then
  cargo build --release --target "$TARGET"
else
  cargo zigbuild --release --target "$TARGET"
fi

# ── Collect binary into a staging dir ────────────────────────────────────────
STAGING="staging/${TARGET}"
mkdir -p "$STAGING" artifacts

# Copy dynamic library if built
if [ "$BUILD_CDYLIB" = true ]; then
  cp "target/${TARGET}/release/${LIB_PREFIX}spf${LIB_EXT}" "$STAGING/"
fi

# Always copy static library
cp "target/${TARGET}/release/${STATIC_LIB_PREFIX}spf${STATIC_LIB_EXT}" "$STAGING/"
cp spf.h "$STAGING/"
cp LICENSE-APACHE "$STAGING/"

# ── Package ───────────────────────────────────────────────────────────────────
TARBALL="artifacts/spf.${TAG_NAME}.${TARGET}.tar.gz"
tar -czf "$TARBALL" -C staging "${TARGET}"
rm -rf staging

echo "✅ Artifact ready: $TARBALL"