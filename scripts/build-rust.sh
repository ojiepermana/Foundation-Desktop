#!/usr/bin/env bash
# Builds the Rust workspace on any host: runs every step, then reports.
# Design: docs/specs/_root/0008-build-dan-penerimaan-per-target/index.md (Skrip, Bentuk keluaran).
set -u
set -o pipefail

cd "$(dirname "$0")/.." || exit 2
. scripts/lib.sh

no_args "$#"

header
toolchain_step
step "cargo build --workspace --locked" cargo build --workspace --locked
finish
