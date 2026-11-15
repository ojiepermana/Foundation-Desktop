# Verify: Aset merek bersama di macOS · spec macos/0007 · updated 2026-11-15

_Langkah ini diturunkan dari kriteria penerimaan spec macos/0007. `/check verify` menjalankannya, dan `/test` mengunci langkah yang tahan lama._

Kamu bisa menjalankan semua perintah dari root repo, dengan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0` dan:

```bash
SPEC=docs/specs/macos/0007-aset-merek-bersama/index.md
C=$(git log --format=%H --grep='spec macos/0007 step 1' | tail -1)
B=$C^
```

Pada build ini `C` adalah `d032ccd` (commit langkah 1) dan `B` adalah `ea807cc` (`main` sebelum langkah 1). Hasil yang tercatat diambil developer di Mac koordinasi pada 2026-11-15, pada commit langkah 1 dengan working tree yang bersih, mulai tanpa `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/`. Tidak ada jendela yang dibuka. Kotak di bawah sengaja dibiarkan kosong untuk checker.

## Milestone 1: Ikon penanda dari sumber bersama sampai bundel Mac

Build plan langkah 1 (`d032ccd`) dan langkah 2.

### UI / manual

Tidak ada di milestone ini. AC-11 (ikon di Dock, Finder, Get Info, dan About) milik milestone 2, `owner: user` (`F011-2`); langkahnya ada di bagian "Rencana verifikasi manual" spec. AC-12 (ikon nyata) milik milestone 3, `BLOCKED` (`F011-3`).

### Commands

- [x] AC-10: `sw_vers -productVersion` → `27.0.1`; `uname -m` → `arm64`; `xcodebuild -version` → `Xcode 27.1`, `Build version 27A9275`; `xcrun --sdk macosx --show-sdk-version` → `27.0`; `rustc --version` → `rustc 1.99.1 (58cf8f9c3 2026-10-13)`; `xcrun actool --version` → `short-bundle-version` `27.1`, `bundle-version` `25098` → AC-10
- [x] `cargo run -p desktop-codegen --locked -- assets check` → kode 0; baris terakhir `assets: ok, 1 assets, 1 source files, 1 placeholder values, 3 outputs up to date` → AC-1
- [x] `cargo run -p desktop-codegen --locked -- assets generate` pada working tree bersih → baris terakhir `assets: ok, 1 assets, 1 source files, 1 placeholder values, 3 outputs, 0 written`; `git diff --exit-code -- apps/macos/Resources shared/assets` → kode 0 → AC-1
- [x] `rm -r apps/macos/Resources/AppIcon.icon`, lalu `assets generate` → `wrote apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg` dan `wrote apps/macos/Resources/AppIcon.icon/icon.json`; `git status --short` tidak mencetak apa pun → AC-1
- [x] ``awk '/^```json$/{n++;f=(n==1);next} /^```$/{f=0} f' "$SPEC" | diff - apps/macos/Resources/AppIcon.icon/icon.json`` dan ``awk '/^```xml$/{f=1;next} /^```$/{f=0} f' "$SPEC" | diff - apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg`` → keduanya kode 0 tanpa keluaran → AC-1
- [x] `git ls-tree -r --name-only "$C" -- apps/macos/Resources` dan `git ls-files apps/macos/Resources` → tepat `apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg` dan `apps/macos/Resources/AppIcon.icon/icon.json` → AC-2
- [x] `git ls-tree -r --name-only "$C" -- apps/macos bindings/swift crates/macos-ffi | wc -l` → `26`, dan untuk `"$B"` → `24`; selisih `2` → AC-2
- [x] `git ls-files | grep -v '^graphify-out/' | grep -ciE '\.(png|icns|ico|car|jpe?g|gif|webp|pdf|tiff?)$'` → `0`; `git ls-files | grep -c 'brand-export\.txt$'` → `0` → AC-2
- [x] `cargo test -p desktop-codegen --locked --test assets_macos_contract` → `test result: ok. 6 passed`; `--test assets_contract` → `7 passed`; `--test assets_rules` → `19 passed`, dengan `git diff --quiet "$B" "$C" -- tooling/codegen/tests/assets_rules.rs` → kode 0 (berkas tidak diubah) → AC-3
- [x] `touch apps/macos/Resources/AppIcon.icon/Assets/old.svg`, lalu `assets check` → kode 1; stderr `apps/macos/Resources/AppIcon.icon/Assets/old.svg: I-1: not produced by the generator, remove it` lalu `assets: failed, 1 problems`. Hapus dengan `rm apps/macos/Resources/AppIcon.icon/Assets/old.svg` → AC-3
- [x] `git ls-files --eol apps/macos/Resources | grep -c 'attr/text eol=lf'` → `2` → AC-3
- [x] `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build` (27 detik) dan `-configuration Release` (32 detik) → `** BUILD SUCCEEDED **` untuk keduanya → AC-4
- [x] Untuk `<K>` `Debug` dan `Release`, dengan `A=apps/macos/DerivedData/Build/Products/<K>/DesktopMac.app`: `test -s "$A/Contents/Resources/AppIcon.icns" && test -s "$A/Contents/Resources/Assets.car"; echo $?` → `0`; `/usr/libexec/PlistBuddy -c 'Print :CFBundleIconFile' -c 'Print :CFBundleIconName' "$A/Contents/Info.plist"` → dua baris `AppIcon`; `iconutil -c iconset -o "$TMPDIR/AppIcon-<K>.iconset" "$A/Contents/Resources/AppIcon.icns"` → kode 0 dan foldernya memuat `icon_16x16.png`; `lipo -archs "$A/Contents/MacOS/DesktopMac"` → `arm64` → AC-4
- [x] Sesudah kedua build: `git status --short` tidak mencetak apa pun (Xcode tidak menulis ke `apps/macos/Resources/AppIcon.icon/`) → AC-4
- [x] `printf '<svg' > apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg`, lalu `assets check` → kode 1; stderr diawali `apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg: differs from the generator output`; build Debug di AC-4 → kode 65, `** BUILD FAILED **`, dengan galat `.../apps/macos/Resources/AppIcon.icon: error: Exception Raised during asset import (2)` → AC-5
- [x] `cargo run -p desktop-codegen --locked -- assets generate` → `... 3 outputs, 1 written`; `assets check` → kode 0; `git status --short` tidak mencetak apa pun → AC-5
- [x] `git diff --numstat "$B" "$C" -- apps/macos/DesktopMac.xcodeproj/project.pbxproj` → `9` tab `0`; `grep -c 'ASSETCATALOG_COMPILER_APPICON_NAME = AppIcon;' apps/macos/DesktopMac.xcodeproj/project.pbxproj` → `2`; `grep -c 'D00000000000000000000020 /\* Resources \*/' ...` → `3`; `grep -c 'path = Resources;' ...` → `1` → AC-6
- [x] `git diff "$B" "$C" -- apps/macos/DesktopMac.xcodeproj/project.pbxproj | grep -c '^+.*INFOPLIST_KEY_'` → `0`; `git diff --quiet "$B" "$C" -- apps/macos/Package.swift tooling/codegen/src/lib.rs; echo $?` → `0` → AC-6
- [x] `cargo build --workspace --locked`, `cargo test --workspace --locked` (223 test, 0 ignored), `cargo fmt --all --check`, dan `cargo clippy --workspace --all-targets --locked -- -D warnings` → kode 0; `cargo tree -p desktop-codegen --depth 1 --prefix none --locked | wc -l` → `3`; `git diff --quiet "$B" "$C" -- Cargo.toml Cargo.lock tooling/codegen/Cargo.toml; echo $?` → `0` → AC-7
- [x] `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin`, lalu `swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"` (18 detik) → kode 0; `Test run with 10 tests in 2 suites passed` (`GreetingViewModelTests`, `LocalizedTextTests`), `Test run with 7 tests in 1 suite passed` (`BindingContractTests`), `Test run with 5 tests in 1 suite passed` (`ThemeTests`); 0 `failed` → AC-7
- [x] `git log --name-only --format= --grep='spec macos/0007 step' | sed '/^$/d' | sort -u` sesudah commit langkah 2 → hanya `.gitattributes`, `apps/macos/DesktopMac.xcodeproj/project.pbxproj`, `apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg`, `apps/macos/Resources/AppIcon.icon/icon.json`, `docs/specs/macos/0007-aset-merek-bersama/index.md`, `verify.md` spec ini, empat file `graphify-out/`, `tooling/codegen/src/assets/macos.rs`, `tooling/codegen/src/assets/mod.rs`, dan `tooling/codegen/tests/assets_macos_contract.rs` → AC-8
- [x] `head -1 apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg | grep -c 'Generated by desktop-codegen from shared/assets/source/app-icon.svg. Do not edit.'` → `1`; `sed -n 2p apps/macos/Resources/AppIcon.icon/icon.json | grep -c 'Generated by desktop-codegen from tooling/codegen/src/assets/macos.rs. Do not edit.'` → `1` → AC-9
- [x] Gerbang Mac spec `macos/0008` (pola `R` cocok): `G-1` kode 0 (2 detik sesudah build cargo sebelumnya); `G-2` dua generator kode 0 dan `diff -r bindings/swift/Sources target/binding-check` kosong (20 detik); `G-3` = `swift test` di atas; `G-4` = build Debug di atas; `G-5` `git status --short` kosong → AC-7
- [x] `/bin/bash scripts/validate.sh` → kode 0; `rust tests: 223 passed, 0 failed, 0 ignored`, `brand exports: 0 files`, `macos gate: required, 7 changed paths match spec macos/0008 R`, `== validate.sh: PASS, 13 steps, 0 failed` → AC-7
- [x] Lima sumber `check` → semuanya kode 0 (`architecture: ok, 13 rules, 3 members, 0 outputs up to date`) → AC-7
- [x] Sesudah semua perintah: `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/` dihapus; `git status --short` kosong → AC-8

### Acceptance-criteria coverage

- AC-10: langkah 1 · AC-1: langkah 2 sampai 5 · AC-2: langkah 6 sampai 8 · AC-3: langkah 9 sampai 11 · AC-4: langkah 12 sampai 14 · AC-5: langkah 15 dan 16 · AC-6: langkah 17 dan 18 · AC-7: langkah 19, 20, dan 23 sampai 25 · AC-8: langkah 21 dan 26 · AC-9: langkah 22.
- AC-11 menunggu milestone 2 (sesi GUI, `owner: user`, `F011-2`). AC-12 menunggu milestone 3 (`BLOCKED`, ikon nyata dari user, `F011-3`).
