#!/usr/bin/env bash
set -euo pipefail

: "${TARGET:?TARGET env var is required}"
: "${TAG_NAME:?TAG_NAME env var is required}"

echo "▶ Building target='$TARGET' tag='$TAG_NAME'"

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
    BUILD_CDYLIB=false
    LIB_PREFIX="lib"
    LIB_EXT=".so"
    STATIC_LIB_PREFIX="lib"
    STATIC_LIB_EXT=".a"
    ;;
  *)
    LIB_PREFIX="lib"
    LIB_EXT=".so"
    STATIC_LIB_PREFIX="lib"
    STATIC_LIB_EXT=".a"
    ;;
esac

# ── Patch Cargo.toml ──────────────────────────────────────────────────────────
CRATE_TYPES="\"rlib\", \"staticlib\""
if [ "$BUILD_CDYLIB" = true ]; then
  CRATE_TYPES="\"rlib\", \"cdylib\", \"staticlib\""
fi

# macOS (Darwin) requires an empty string for the -i flag
if [[ "$(uname)" == "Darwin" ]]; then
  sed -i '' "s/crate-type = \[\"rlib\"\]/crate-type = \[$CRATE_TYPES\]/" Cargo.toml
else
  # Linux and Windows (Git Bash)
  sed -i "s/crate-type = \[\"rlib\"\]/crate-type = \[$CRATE_TYPES\]/" Cargo.toml
fi

# ── Build ─────────────────────────────────────────────────────────────────────
# Use native cargo on macOS and Windows runners (where SDKs are present).
# Use cargo-zigbuild on Linux runners for cross-compilation.
if [[ "$(uname)" == "Darwin" ]] || [[ "$(uname)" == *"MINGW"* ]] || [[ "$(uname)" == *"MSYS"* ]]; then
  cargo build --release --target "$TARGET"
else
  cargo zigbuild --release --target "$TARGET"
fi

# ── Collect into staging ─────────────────────────────────────────────────────
STAGING="staging/${TARGET}"
mkdir -p "$STAGING" artifacts

if [ "$BUILD_CDYLIB" = true ]; then
  cp "target/${TARGET}/release/${LIB_PREFIX}spf${LIB_EXT}" "$STAGING/"
fi
cp "target/${TARGET}/release/${STATIC_LIB_PREFIX}spf${STATIC_LIB_EXT}" "$STAGING/"
cp spf.h "$STAGING/"
cp LICENSE-APACHE "$STAGING/"

# ── Package ───────────────────────────────────────────────────────────────────
TARBALL="artifacts/spf.${TAG_NAME}.${TARGET}.tar.gz"
tar -czf "$TARBALL" -C staging "${TARGET}"
rm -rf staging

echo "✅ Artifact ready: $TARBALL"