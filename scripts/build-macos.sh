#!/usr/bin/env bash
# Mac gate: the four source checks with macOS outputs, G-1 to G-5 from spec macos/0008, then a Release build.
# Design: docs/specs/_root/0008-build-dan-penerimaan-per-target/index.md (Skrip, Bentuk keluaran).
# Commands: docs/specs/macos/0008-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md (Gerbang Mac).
set -u
set -o pipefail

cd "$(dirname "$0")/.." || exit 2
. scripts/lib.sh

no_args "$#"
needs_darwin_arm64

# The .icon app icon of spec macos/0007 needs actool from Xcode 26 or newer.
xcode_line=$(xcodebuild -version 2>&1 | head -n 1)
xcode_major=$(printf '%s\n' "$xcode_line" | sed -n 's/^Xcode \([0-9][0-9]*\).*/\1/p')
if [ -z "$xcode_major" ] || [ "$xcode_major" -lt 26 ]; then
  echo "$NAME: needs Xcode 26 or newer (spec macos/0007), this host has $xcode_line" >&2
  exit 2
fi

# setting_value <settings text> <key>: the value of one build setting, or <missing>.
setting_value() {
  value=$(printf '%s\n' "$1" | sed -n "s/^    $2 = //p" | head -n 1)
  [ -n "$value" ] || value="<missing>"
  printf '%s' "$value"
}

# or_missing <command> [arguments...]: the first output line of the command, or <missing>.
or_missing() {
  value=$("$@" 2>/dev/null | head -n 1)
  [ -n "$value" ] || value="<missing>"
  printf '%s' "$value"
}

# plist_value <app> <key>: one Info.plist value of the built app.
plist_value() {
  or_missing plutil -extract "$2" raw -o - "$1/Contents/Info.plist"
}

# minos_value <binary>: the number after "minos " in vtool -show-build.
minos_value() {
  value=$(vtool -show-build "$1" 2>/dev/null | sed -n 's/^ *minos \([^ ]*\).*/\1/p' | head -n 1)
  [ -n "$value" ] || value="<missing>"
  printf '%s' "$value"
}

# facts <configuration>: the settings and bundle lines for one configuration. Not a step.
facts() {
  settings=$(xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration "$1" -showBuildSettings 2>/dev/null)
  echo "settings $1: INFOPLIST_KEY_CFBundleDisplayName=$(setting_value "$settings" INFOPLIST_KEY_CFBundleDisplayName), MARKETING_VERSION=$(setting_value "$settings" MARKETING_VERSION), PRODUCT_BUNDLE_IDENTIFIER=$(setting_value "$settings" PRODUCT_BUNDLE_IDENTIFIER), PRODUCT_NAME=$(setting_value "$settings" PRODUCT_NAME), CURRENT_PROJECT_VERSION=$(setting_value "$settings" CURRENT_PROJECT_VERSION)"
  app=apps/macos/DerivedData/Build/Products/$1/DesktopMac.app
  echo "bundle $1: archs $(or_missing lipo -archs "$app/Contents/MacOS/DesktopMac"), minos $(minos_value "$app/Contents/MacOS/DesktopMac"), CFBundleDisplayName $(plist_value "$app" CFBundleDisplayName), CFBundleIdentifier $(plist_value "$app" CFBundleIdentifier), CFBundleShortVersionString $(plist_value "$app" CFBundleShortVersionString), CFBundleVersion $(plist_value "$app" CFBundleVersion), CFBundleName $(plist_value "$app" CFBundleName)"
}

extra_lines() {
  if [ "$FAILED_STEPS" = " 10" ]; then
    echo "G-2: bindings/swift differs from the generator output, run bash scripts/generate-bindings.sh and commit bindings/swift with the Rust change (spec macos/0008)"
  fi
  if [ "$STEPS" -eq 14 ] && [ "$FAILED" -eq 0 ]; then
    facts Debug
    facts Release
  fi
}

header
toolchain_step || finish
for source in tokens config localization assets; do
  step "cargo run -p desktop-codegen --locked -- $source check" cargo run -p desktop-codegen --locked -- "$source" check || finish
done
# G-1
step "cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin" \
  cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin || finish
# G-2
step "rm -rf target/binding-check" rm -rf target/binding-check || finish
step "cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- --swift-sources target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a target/binding-check/DesktopMacosFFI" \
  cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- \
  --swift-sources target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a \
  target/binding-check/DesktopMacosFFI || finish
step "cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- --headers --modulemap --module-name desktop_macos_ffiFFI --modulemap-filename module.modulemap target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a target/binding-check/desktop_macos_ffiFFI" \
  cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- \
  --headers --modulemap --module-name desktop_macos_ffiFFI --modulemap-filename module.modulemap \
  target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a \
  target/binding-check/desktop_macos_ffiFFI || finish
step "diff -r bindings/swift/Sources target/binding-check" diff -r bindings/swift/Sources target/binding-check || finish
# G-3
step 'swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"' \
  swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug" || finish
# G-4
step "xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build" \
  xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac \
  -configuration Debug -derivedDataPath apps/macos/DerivedData build || finish
# G-5
step "git status --short" must_print_nothing git status --short || finish
# G-4 with the Release configuration
step "xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Release -derivedDataPath apps/macos/DerivedData build" \
  xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac \
  -configuration Release -derivedDataPath apps/macos/DerivedData build || finish
finish
