#!/usr/bin/env bash
# Runs the Rust workspace tests on any host: runs every step, then reports.
# Design: docs/specs/_root/0008-build-dan-penerimaan-per-target/index.md (Skrip, Bentuk keluaran).
set -u
set -o pipefail

cd "$(dirname "$0")/.." || exit 2
. scripts/lib.sh

no_args "$#"

extra_lines() {
  rust_tests_line
}

header
toolchain_step
step "cargo test --workspace --locked" logged cargo-test.log cargo test --workspace --locked
finish
