# Verify: Build dan penerimaan per target · spec _root/0008 · updated 2026-11-15

_Langkah ini diturunkan dari kriteria penerimaan spec _root/0008. `/check verify` menjalankannya, dan `/test` mengunci langkah yang tahan lama._

Kamu bisa menjalankan semua perintah dari akar repo, dengan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, dan skrip selalu dipanggil dengan `/bin/bash` (bash `3.2.57(1)-release (arm64-apple-darwin26)` bawaan macOS) supaya kecocokan bash `3.2` benar benar teruji. Hasil yang tercatat diambil developer di Mac koordinasi (macOS `27.0.1`, `arm64`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1 (5f94df478 2026-08-27)`, Xcode 27.1 build 27A9275) pada 2026-11-15, dengan build hangat. AC-1 sampai AC-3 diambil pada commit langkah 1 (`b5a9005`), AC-4, AC-15, dan AC-16 pada commit langkah 2 (`f0c166e`), semuanya pada working tree yang bersih. Kotak di bawah sengaja dibiarkan kosong untuk checker.

## Milestone 1: Gerbang bersama

Build plan langkah 1 dan 2.

### UI / manual

Tidak ada untuk milestone ini. Langkah `F012-1` (host Windows dan Linux) dan `F012-2` (run GitHub Actions) milik user dan dicatat di milestone 3.

### Commands

- [x] `/bin/bash scripts/validate.sh | grep -c '^commit: [0-9a-f]\{40\} (clean)$'` → `1` (tercatat `commit: b5a900547b3c94e417dab6d36bfeca051c53c880 (clean)`) → AC-1
- [x] `/bin/bash scripts/validate.sh --nope` → kode `2`, stdout kosong (0 byte), stderr tepat satu baris `usage: bash scripts/validate.sh` → AC-1
- [x] `/bin/bash scripts/build-rust.sh --x` dan `/bin/bash scripts/test.sh --x` → masing masing kode `2`, stdout kosong, stderr `usage: bash scripts/build-rust.sh` dan `usage: bash scripts/test.sh` → AC-1
- [x] Keluaran ketiga skrip dibandingkan dengan bagian **Bentuk keluaran** spec → baris kepala `== <nama>.sh`, `host: Darwin arm64`, `commit: ... (clean)`, `rustc: ...`, `cargo: ...`, `macos: 27.0.1`, `xcode: Xcode 27.1 Build version 27A9275`, `swift: ...`; tiap langkah diapit `-- step <n>: <label>` dan `-- step <n>: PASS (exit 0, <detik>s)`; label langkah 1 `toolchain matches rust-toolchain.toml (1.99.1)` dengan keluaran `rustc 1.99.1 (58cf8f9c3 2026-10-13)`; baris penutup selalu baris terakhir → AC-1
- [x] `/bin/bash scripts/validate.sh` → kode `0`; baris terakhir `== validate.sh: PASS, 13 steps, 0 failed`. Rumus: 8 + 5 nama di baris `sources: tokens, config, localization, assets, architecture` + 0 berkas `git ls-files '*brand-export.txt'` = 13 → AC-2
- [x] Keluaran yang sama memuat lima baris sumber: `tokens: ok, 6 semantic tokens, 3 placeholder values, 3 outputs up to date`, `config: ok, schema version 1, 2 build values, 1 outputs up to date`, `localization: ok, 1 keys, 2 locales, 0 missing translations, 1 outputs up to date`, `assets: ok, 1 assets, 1 source files, 1 placeholder values, 1 outputs up to date`, `architecture: ok, 13 rules, 3 members, 0 outputs up to date` → AC-2
- [x] Keluaran yang sama memuat `desktop-core v0.1.0 (...)` (langkah 6), keluaran langkah 7 tepat satu baris `crates/core/src/greeting.rs`, `rust tests: 178 passed, 0 failed, 0 ignored` (sama dengan jumlah `passed` dan `ignored` di 22 baris `test result:` pada `target/scripts/cargo-test.log`), `brand exports: 0 files`, dan `macos gate: required, 1 changed paths match spec macos/0008 R` (path itu `.gitattributes`) → AC-2
- [x] `git status --short` sesudah `validate.sh` → kosong → AC-2
- [x] `printf '\n' >> design-system/schemas/colors.schema.json`, lalu `/bin/bash scripts/validate.sh` → kode `1`, ke 13 langkah tetap dijalankan, baris terakhir `== validate.sh: FAIL, 13 steps, 2 failed (steps 5 9)`; langkah 5 (`cargo test --workspace --locked`) `FAIL (exit 101, ...)` dan langkah 9 (`tokens check`) `FAIL (exit 1, ...)` → AC-3
- [x] `git checkout -- design-system/schemas/colors.schema.json`, lalu skrip yang sama → kode `0`, `== validate.sh: PASS, 13 steps, 0 failed` → AC-3
- [x] `printf '// Hello, {name}\n' > probe.rs`, lalu skrip yang sama → kode `1`, baris terakhir `== validate.sh: FAIL, 13 steps, 1 failed (steps 7)`, keluaran langkah 7 `crates/core/src/greeting.rs` dan `probe.rs` → AC-3
- [x] `rm probe.rs`, lalu skrip yang sama → kode `0`, `== validate.sh: PASS, 13 steps, 0 failed`; `git status --short` kosong → AC-3
- [x] `/bin/bash scripts/build-rust.sh` → kode `0`, berakhir `== build-rust.sh: PASS, 2 steps, 0 failed` → AC-4
- [x] `/bin/bash scripts/test.sh` → kode `0`, berakhir `rust tests: 178 passed, 0 failed, 0 ignored` lalu `== test.sh: PASS, 2 steps, 0 failed` → AC-4
- [x] `grep -nE '^/(scripts/\*\.sh|\.github/workflows/\*\.yml) text eol=lf$' .gitattributes` → baris 6 `/scripts/*.sh text eol=lf` dan baris 7 `/.github/workflows/*.yml text eol=lf`, sesudah lima baris yang sudah ada → AC-15
- [x] `git ls-files --eol -- 'scripts/*.sh' .github | grep -vc 'attr/text eol=lf'` → `0` → AC-15
- [x] `git ls-files -s -- 'scripts/*.sh' | awk '$4!="scripts/lib.sh" && $1!="100755"'` dan `git ls-files -s -- scripts/lib.sh | awk '$1!="100644"'` → keduanya kosong (`build-rust.sh`, `test.sh`, `validate.sh` bermode `100755`, `lib.sh` `100644`) → AC-15
- [x] `/bin/bash -n` untuk `scripts/build-rust.sh`, `scripts/lib.sh`, `scripts/test.sh`, dan `scripts/validate.sh` → keempatnya kode `0` → AC-15
- [x] `git log --name-only --format= --grep='spec _root/0008 step' | grep -v '^graphify-out/' | sort -u` → hanya `.gitattributes`, `docs/specs/_root/0008-build-dan-penerimaan-per-target/index.md`, `docs/specs/_root/0008-build-dan-penerimaan-per-target/verify.md`, `scripts/build-rust.sh`, `scripts/lib.sh`, `scripts/test.sh`, `scripts/validate.sh`; tanpa `Cargo.toml`, `Cargo.lock`, `.githooks/`, `scripts/update_graphify.py`, atau `.cargo/` → AC-16
- [x] `grep -nE 'git (commit|push|config|stash|add)|graphify|githooks|--no-verify|curl|wget' scripts/*.sh` → kosong (kode keluar `grep` 1) → AC-16
- [x] `test ! -e scripts/build-windows.ps1 && test ! -e scripts/build-linux.sh` → kode `0` → AC-16

### Acceptance-criteria coverage

- AC-1: langkah 1 sampai 4 · AC-2: langkah 5 sampai 8 · AC-3: langkah 9 sampai 12 · AC-4: langkah 13 dan 14 · AC-15: langkah 15 sampai 18 · AC-16: langkah 19 sampai 21 (bagian milestone 1).
- AC-1 untuk ketiga skrip Mac, AC-5 sampai AC-9, dan AC-15 untuk skrip Mac menunggu milestone 2. AC-10, AC-13, AC-14, dan AC-16 untuk milestone 3 menunggu milestone 3. AC-11 dan AC-12 menunggu milestone 4 (`F012-1` dan `F012-2`, `owner: user`).

## Milestone 2: Skrip macOS

Build plan langkah 3. Hasil tercatat diambil developer di Mac yang sama pada 2026-11-15, pada commit langkah 3 (`f88607c`) dengan working tree bersih dan build hangat (satu run percobaan sebelum commit sudah membangun jembatan, `swift test`, dan Debug). Kalau kamu menjalankannya dari build dingin, lama langkah 6, 8, 11, 12, dan 14 akan lebih panjang. Kotak sengaja dibiarkan kosong untuk checker.

### UI / manual

Tidak ada. Tidak ada jendela yang dibuka; `xcodebuild` hanya membangun.

### Commands

- [x] `/bin/bash scripts/build-macos.sh` → kode `0`; baris terakhir `== build-macos.sh: PASS, 14 steps, 0 failed`; baris kepala `host: Darwin arm64`, `commit: f88607c60092ba0c529416f971262b9a172f9977 (clean)`, `rustc: rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo: cargo 1.99.1 (5f94df478 2026-08-27)`, `macos: 27.0.1`, `xcode: Xcode 27.1 Build version 27A9275`, `swift: swift-driver version: 1.168.6 Apple Swift version 6.4 (swiftlang-6.4.0.34.1 clang-2100.3.34.1)` → AC-1, AC-5
- [x] Keluaran yang sama: langkah 2 sampai 5 berlabel `cargo run -p desktop-codegen --locked -- <sumber> check` untuk `tokens`, `config`, `localization`, `assets`, berurutan, sebelum `xcodebuild` pertama; label langkah 6 sampai 14 berikut, dengan lama tercatat → AC-5
  - 6 `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin` (`0s`)
  - 7 `rm -rf target/binding-check` (`0s`)
  - 8 `cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- --swift-sources target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a target/binding-check/DesktopMacosFFI` (`1s`)
  - 9 `cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- --headers --modulemap --module-name desktop_macos_ffiFFI --modulemap-filename module.modulemap target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a target/binding-check/desktop_macos_ffiFFI` (`0s`)
  - 10 `diff -r bindings/swift/Sources target/binding-check` (`0s`)
  - 11 `swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"` (`3s`)
  - 12 `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build` (`3s`)
  - 13 `git status --short` (`0s`)
  - 14 `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Release -derivedDataPath apps/macos/DerivedData build` (`40s`)
- [x] Keluaran yang sama: `grep -cE 'Test run with [0-9]+ tests? in [0-9]+ suites? passed'` → `3` (`10 tests in 2 suites`, `7 tests in 1 suite`, `5 tests in 1 suite`); `grep -c '\*\* BUILD SUCCEEDED \*\*'` → `2`; baris `-- step 13: git status --short` langsung diikuti `-- step 13: PASS (exit 0, 0s)` → AC-5
- [x] Keluaran yang sama memuat empat baris fakta, tepat sebelum baris penutup → AC-5
  - `settings Debug: INFOPLIST_KEY_CFBundleDisplayName=Desktop Foundation, MARKETING_VERSION=0.1.0, PRODUCT_BUNDLE_IDENTIFIER=com.example.desktop-foundation.macos, PRODUCT_NAME=DesktopMac, CURRENT_PROJECT_VERSION=1`
  - `bundle Debug: archs arm64, minos 14.0, CFBundleDisplayName Desktop Foundation, CFBundleIdentifier com.example.desktop-foundation.macos, CFBundleShortVersionString 0.1.0, CFBundleVersion 1, CFBundleName DesktopMac`
  - `settings Release: ...` dan `bundle Release: ...` dengan nilai yang sama persis
- [x] `git status --short` sesudah `build-macos.sh` → kosong → AC-5
- [x] `printf '\n' >> bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift`, lalu `/bin/bash scripts/build-macos.sh` → kode `1`; baris terakhir `== build-macos.sh: FAIL, 10 steps, 1 failed (steps 10)`; keluaran langkah 10 memuat `diff -r bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift target/binding-check/DesktopMacosFFI/desktop_macos_ffi.swift`; baris `G-2: bindings/swift differs from the generator output, run bash scripts/generate-bindings.sh and commit bindings/swift with the Rust change (spec macos/0008)` tepat sebelum baris penutup; tidak ada baris `-- step 11:`. Lalu `git checkout -- bindings/swift` → `git status --short` kosong → AC-6
- [x] `/bin/bash scripts/generate-bindings.sh` → kode `0`; memuat `bindings/swift: unchanged`; baris terakhir `== generate-bindings.sh: PASS, 5 steps, 0 failed`; `git status --short` kosong → AC-7
- [x] `printf '\n' >> bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift`, lalu `/bin/bash scripts/generate-bindings.sh` → kode `0`; memuat `bindings/swift: unchanged`; `== generate-bindings.sh: PASS, 5 steps, 0 failed`; `git status --short` kosong → AC-7
- [x] `/bin/bash scripts/generate-design-tokens.sh` → kode `0`; keluaran langkah 2 `tokens: ok, 6 semantic tokens, 3 placeholder values, 4 outputs, 0 written`; baris terakhir `== generate-design-tokens.sh: PASS, 2 steps, 0 failed`; `git status --short` kosong → AC-8
- [x] `/bin/bash scripts/generate-bindings.sh --x` dan `/bin/bash scripts/generate-design-tokens.sh --x` → masing masing kode `2`, stdout kosong, stderr `usage: bash scripts/generate-bindings.sh` dan `usage: bash scripts/generate-design-tokens.sh` → AC-1
- [x] `uname` palsu (`d=$(mktemp -d); printf '#!/bin/sh\necho Linux\n' > "$d/uname"; chmod +x "$d/uname"`): `PATH="$d:$PATH" /bin/bash scripts/build-macos.sh` dan `PATH="$d:$PATH" /bin/bash scripts/generate-bindings.sh` → masing masing kode `2`, stdout 0 byte, stderr satu baris `build-macos.sh: needs macOS on arm64, this host is Linux Linux` dan `generate-bindings.sh: needs macOS on arm64, this host is Linux Linux` (`uname` palsu mencetak `Linux` untuk `-s` dan `-m`) → AC-9
- [x] `PATH="$d:$PATH" /bin/bash scripts/build-macos.sh --nope` → kode `2`, stdout kosong, stderr `usage: bash scripts/build-macos.sh` → AC-9
- [x] `xcodebuild` palsu (`x=$(mktemp -d); printf '#!/bin/sh\necho "Xcode 25.4"\necho "Build version 25E1"\n' > "$x/xcodebuild"; chmod +x "$x/xcodebuild"`): `PATH="$x:$PATH" /bin/bash scripts/build-macos.sh` → kode `2`, stdout 0 byte, stderr tepat `build-macos.sh: needs Xcode 26 or newer (spec macos/0007), this host has Xcode 25.4`. Lalu `rm -rf "$d" "$x"`. Folder sementara dibuat di scratchpad sesi, bukan di repo → AC-9
- [x] `git ls-files --eol -- 'scripts/*.sh' .github | grep -vc 'attr/text eol=lf'` → `0` → AC-15
- [x] `git ls-files -s -- 'scripts/*.sh' | awk '$4!="scripts/lib.sh" && $1!="100755"'` dan `git ls-files -s -- scripts/lib.sh | awk '$1!="100644"'` → keduanya kosong (`build-macos.sh`, `generate-bindings.sh`, `generate-design-tokens.sh` bermode `100755`) → AC-15
- [x] `/bin/bash -n` untuk ketujuh berkas `scripts/*.sh` → semuanya kode `0` → AC-15
- [x] `grep -nE 'git (commit|push|config|stash|add)|graphify|githooks|--no-verify|curl|wget' scripts/*.sh` → kosong (kode `grep` 1); `git log --name-only --format= --grep='spec _root/0008 step' | grep -v '^graphify-out/' | sort -u` → hanya path milestone 1 ditambah `scripts/build-macos.sh`, `scripts/generate-bindings.sh`, `scripts/generate-design-tokens.sh` → AC-16 (penjaga batas, dicatat untuk milestone ini)
- [x] `/bin/bash scripts/validate.sh` pada commit yang sama → kode `0`, `== validate.sh: PASS, 13 steps, 0 failed`, `rust tests: 223 passed, 0 failed, 0 ignored`, `macos gate: not required` (base `main` `80f360f`; perubahan hanya di `scripts/` dan dokumen) → gerbang bersama tetap lulus

### Acceptance-criteria coverage

- AC-1 (ketiga skrip Mac): langkah 1, 10 sampai 12 · AC-5: langkah 1 sampai 5 · AC-6: langkah 6 · AC-7: langkah 7 dan 8 · AC-8: langkah 9 · AC-9: langkah 11 sampai 13 · AC-15: langkah 14 sampai 16.
- AC-10, AC-13, AC-14, dan AC-16 milestone 3 menunggu milestone 3. AC-11 dan AC-12 menunggu milestone 4.
