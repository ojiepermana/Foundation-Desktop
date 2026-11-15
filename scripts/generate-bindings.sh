#!/usr/bin/env bash
# Rewrites the Swift binding in bindings/swift/Sources from the bridge library, then reports whether it changed.
# Design: docs/specs/_root/0008-build-dan-penerimaan-per-target/index.md (Skrip, Bentuk keluaran).
# Commands: docs/specs/macos/0001-sapaan-di-jendela-macos/index.md (Membuat ulang binding).
set -u
set -o pipefail

cd "$(dirname "$0")/.." || exit 2
. scripts/lib.sh

no_args "$#"
needs_darwin_arm64

extra_lines() {
  if [ -z "$(git status --porcelain -- bindings/swift 2>&1)" ]; then
    echo "bindings/swift: unchanged"
  else
    echo "bindings/swift: changed, review and commit it with the bridge change (spec macos/0001)"
  fi
}

header
toolchain_step || finish
step "cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin" \
  cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin || finish
step "rm -rf bindings/swift/Sources" rm -rf bindings/swift/Sources || finish
step "cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- --swift-sources target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a bindings/swift/Sources/DesktopMacosFFI" \
  cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- \
  --swift-sources target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a \
  bindings/swift/Sources/DesktopMacosFFI || finish
step "cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- --headers --modulemap --module-name desktop_macos_ffiFFI --modulemap-filename module.modulemap target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a bindings/swift/Sources/desktop_macos_ffiFFI" \
  cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- \
  --headers --modulemap --module-name desktop_macos_ffiFFI --modulemap-filename module.modulemap \
  target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a \
  bindings/swift/Sources/desktop_macos_ffiFFI || finish
finish
