#!/usr/bin/env bash
# Shared merge gate for every host: runs every step, then reports.
# Design: docs/specs/_root/0008-build-dan-penerimaan-per-target/index.md (Skrip, Bentuk keluaran).
set -u
set -o pipefail

cd "$(dirname "$0")/.." || exit 2
. scripts/lib.sh

no_args "$#"

# Pattern R from spec macos/0008, section "Kapan gerbang wajib", copied as is.
R='^(crates/|bindings/|apps/macos/|tooling/codegen/|design-system/|shared/|Cargo\.toml$|Cargo\.lock$|rust-toolchain\.toml$|\.gitattributes$|\.gitignore$)'

SOURCES=""

# single_greeting_source: the greeting text has one source (spec _root/0002 AC-3).
single_greeting_source() {
  greeting_files=$(git grep --untracked -lF 'Hello, {' -- '*.rs' 2>&1)
  [ -n "$greeting_files" ] && printf '%s\n' "$greeting_files"
  [ "$greeting_files" = "crates/core/src/greeting.rs" ]
}

# list_sources: desktop-codegen without arguments exits with code 2 and names its sources on a "sources: " line.
list_sources() {
  sources_out=$(cargo run -p desktop-codegen --locked 2>&1)
  sources_code=$?
  [ -n "$sources_out" ] && printf '%s\n' "$sources_out"
  [ "$sources_code" -eq 2 ] || return 1
  SOURCES=$(printf '%s\n' "$sources_out" | sed -n 's/^sources: //p' | head -n 1 | tr ',' ' ')
  [ -n "$(printf '%s' "$SOURCES" | tr -d ' ')" ]
}

# brand_export_check <file>: the committed export record matches the source files (spec _root/0005, E-6).
brand_export_check() {
  git ls-files -s -- shared/assets/source | diff - "$1"
}

extra_lines() {
  rust_tests_line
  echo "brand exports: $(printf '%s' "$BRAND_FILES" | grep -c .) files"
  if git rev-parse --verify -q main >/dev/null; then
    gate_paths=$(git diff --name-only main...HEAD | grep -cE "$R")
    if [ "$gate_paths" -gt 0 ]; then
      echo "macos gate: required, $gate_paths changed paths match spec macos/0008 R"
    else
      echo "macos gate: not required"
    fi
  else
    echo "macos gate: unknown, no local main branch"
  fi
}

header
toolchain_step
step "cargo fmt --all --check" cargo fmt --all --check
step "cargo clippy --workspace --all-targets --locked -- -D warnings" cargo clippy --workspace --all-targets --locked -- -D warnings
step "cargo build --workspace --locked" cargo build --workspace --locked
step "cargo test --workspace --locked" logged cargo-test.log cargo test --workspace --locked
step "cargo tree -p desktop-core --target all --all-features --prefix none --locked" cargo tree -p desktop-core --target all --all-features --prefix none --locked
step "git grep --untracked -lF 'Hello, {' -- '*.rs'" single_greeting_source
if step "cargo run -p desktop-codegen --locked" list_sources; then
  for source in $SOURCES; do
    step "cargo run -p desktop-codegen --locked -- $source check" cargo run -p desktop-codegen --locked -- "$source" check
  done
fi
BRAND_FILES=$(git ls-files '*brand-export.txt')
old_ifs=$IFS
IFS='
'
for brand_file in $BRAND_FILES; do
  IFS=$old_ifs
  step "git ls-files -s -- shared/assets/source | diff - $brand_file" brand_export_check "$brand_file"
done
IFS=$old_ifs
finish
