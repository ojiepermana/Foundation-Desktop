#!/usr/bin/env bash
# Rewrites the token outputs from design-system/ on any host, then reports.
# Design: docs/specs/_root/0008-build-dan-penerimaan-per-target/index.md (Skrip, Bentuk keluaran).
set -u
set -o pipefail

cd "$(dirname "$0")/.." || exit 2
. scripts/lib.sh

no_args "$#"

header
toolchain_step || finish
step "cargo run -p desktop-codegen --locked -- tokens generate" cargo run -p desktop-codegen --locked -- tokens generate || finish
finish
