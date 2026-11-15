# Verify: Aturan pengembangan dan pemeriksaan arsitektur macOS · spec macos/0008 · updated 2026-11-15

_Langkah ini diturunkan dari kriteria penerimaan spec macos/0008. `/check verify` menjalankannya, dan `/test` mengunci langkah yang tahan lama._

Kamu bisa menjalankan semua perintah dari root repo, dengan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. Hasil yang tercatat diambil developer di Mac koordinasi (macOS 27.0.1, `arm64`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, Xcode 27.1, Apple Swift 6.4) pada 2026-11-15, pada working tree yang bersih di `main` `5f7e69d`, mulai tanpa `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/`. Tidak ada jendela yang dibuka. Lama tiap perintah adalah waktu dinding pada build dari kosong. Kotak di bawah sengaja dibiarkan kosong untuk checker.

## Milestone 1: Gerbang Mac terbukti

Build plan langkah 1. Perintah `G-1` sampai `G-5` persis seperti bagian **Gerbang Mac** spec, dijalankan berurutan. Tidak ada kode yang berubah.

### UI / manual

Tidak ada. Semua perintah berjalan tanpa jendela, termasuk `xcodebuild`.

### Commands

- [ ] `git status --short` sebelum `G-1` → tidak mencetak apa pun → AC-1, AC-3
- [ ] `G-1`: `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin` → kode 0, `Finished`, 8 detik → AC-1, AC-3
- [ ] `G-2`: `rm -rf target/binding-check`, dua perintah `cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- ...` dengan argumen spec (`--swift-sources ... target/binding-check/DesktopMacosFFI`, lalu `--headers --modulemap --module-name desktop_macos_ffiFFI --modulemap-filename module.modulemap ... target/binding-check/desktop_macos_ffiFFI`), lalu `diff -r bindings/swift/Sources target/binding-check` → ketiganya kode 0, `diff -r` tidak mencetak apa pun, 20 detik total. `target/binding-check/` berisi `DesktopMacosFFI/desktop_macos_ffi.swift`, `desktop_macos_ffiFFI/desktop_macos_ffiFFI.h`, dan `desktop_macos_ffiFFI/module.modulemap`; `git status --short` tetap kosong, jadi `G-2` tidak menulis ke `bindings/swift/` → AC-1, AC-3
- [ ] AC-2 (a): `printf '\n' >> bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift`, lalu `diff -r bindings/swift/Sources target/binding-check` → kode 1; keluaran memuat `diff -r bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift target/binding-check/DesktopMacosFFI/desktop_macos_ffi.swift` → AC-2
- [ ] AC-2 (b): `touch bindings/swift/Sources/desktop_macos_ffiFFI/stale.h`, lalu perintah yang sama → kode 1; keluaran memuat `Only in bindings/swift/Sources/desktop_macos_ffiFFI: stale.h` → AC-2
- [ ] AC-2 (c): `rm bindings/swift/Sources/desktop_macos_ffiFFI/module.modulemap`, lalu perintah yang sama → kode 1; keluaran memuat `Only in target/binding-check/desktop_macos_ffiFFI: module.modulemap` → AC-2
- [ ] Sesudah tiap perubahan (a), (b), dan (c): `git checkout -- bindings/swift` dan `rm -f bindings/swift/Sources/desktop_macos_ffiFFI/stale.h`, lalu `diff -r bindings/swift/Sources target/binding-check` → kode 0 tiga kali; sesudah ketiganya `git status --short` kosong → AC-2
- [ ] `G-3`: `swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"` → kode 0, 26 detik; keluaran memuat `Test run with 4 tests in 1 suite passed` (`GreetingViewModelTests`) dan `Test run with 7 tests in 1 suite passed` (kontrak sapaan), satu baris untuk tiap target uji; `grep -c failed` atas keluaran → `0` → AC-3
- [ ] `G-4`: `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build` → kode 0, 28 detik; keluaran memuat `** BUILD SUCCEEDED **` → AC-3
- [ ] `G-5`: `git status --short` → tidak mencetak apa pun → AC-1, AC-3
- [ ] AC-4: dengan `R='^(crates/|bindings/|apps/macos/|tooling/codegen/|design-system/|shared/|Cargo\.toml$|Cargo\.lock$|rust-toolchain\.toml$|\.gitattributes$|\.gitignore$)'` (sama dengan blok di bagian **Kapan gerbang wajib**), `printf 'docs/scope/scope.md\nREADME.md\nAGENTS.md\ndocs/specs/macos/0001-sapaan-di-jendela-macos/index.md\n' | grep -cE "$R"` → `0` → AC-4
- [ ] AC-4: `printf 'crates/core/src/lib.rs\nCargo.lock\napps/macos/Package.swift\nbindings/swift/Package.swift\ntooling/codegen/src/tokens/mod.rs\nshared/config/public.json\ndesign-system/tokens/colors.json\nrust-toolchain.toml\n' | grep -cE "$R"` → `8` → AC-4
- [ ] AC-5: `git grep -nE 'Command::new\("(xcodebuild|swift|xcrun|cargo|diff)"' -- '*.rs'` → tidak mencetak apa pun → AC-5
- [ ] AC-5: sesudah `rm -rf target apps/macos/.build apps/macos/DerivedData` dan dengan `git status --short` kosong, `cargo test --workspace --locked` → kode 0, 159 test, tidak ada `FAILED`, 14 detik; sesudahnya `apps/macos/.build/` dan `apps/macos/DerivedData/` tetap tidak ada, jadi uji tidak menjalankan Xcode atau Swift → AC-5
- [ ] AC-9: `cargo tree -p desktop-codegen --depth 1 --prefix none --locked | wc -l` sebelum langkah 1 → `3` (pembanding untuk sesudah langkah 4) → AC-9
- [ ] AC-9: `git log --name-only --format= --grep='spec macos/0008 step' | sort -u` sesudah commit langkah 1 → hanya `docs/specs/macos/0008-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md` (baris `**Status**`), `docs/specs/macos/0008-aturan-pengembangan-dan-pemeriksaan-arsitektur/verify.md`, dan empat file `graphify-out/`. Tidak ada `Cargo.toml`, `Cargo.lock`, `.gitattributes`, `.gitignore`, atau path di bawah `scripts/`, `.github/`, `.githooks/`, `apps/`, `bindings/`, dan `crates/` → AC-9
- [ ] Sesudah semua perintah: `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/` dihapus; `git status --short` kosong → AC-1

### Acceptance-criteria coverage

- AC-1: langkah 1 sampai 3, 10, dan 17 · AC-2: langkah 4 sampai 7 · AC-3: langkah 1 sampai 3 dan 8 sampai 10 · AC-4: langkah 11 dan 12 · AC-5: langkah 13 dan 14 · AC-9: langkah 15 dan 16.
- AC-8 menunggu milestone 2 (persetujuan user untuk `AGENTS.md`). AC-6 menunggu milestone 3, dan AC-7 menunggu milestone 4.
