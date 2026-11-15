# Verify: Sapaan di jendela macOS · spec macos/0001 · updated 2026-11-15

_Langkah ini diturunkan dari kriteria penerimaan spec macos/0001. `/check verify` menjalankannya, dan `/test` mengunci langkah yang tahan lama._

Kamu bisa menjalankan semua perintah dari akar repo, dengan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. Hasil yang tercatat diambil developer pada commit langkah 3 (`aac5336`) di Mac koordinasi (macOS 27.0.1, `arm64`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, Xcode 27.1, Swift 6.4) pada 2026-11-15. Path absolut di keluaran `cargo tree` ikut lokasi checkout, jadi yang dicocokkan cukup awal barisnya. Kotak di bawah sengaja dibiarkan kosong untuk checker.

## Milestone 1: Jembatan, binding, dan uji tanpa jendela

Build plan langkah 1 sampai 3.

### UI / manual

Tidak ada. Milestone ini belum punya jendela, dan semua bukti berupa perintah tanpa jendela. Langkah GUI ada di milestone 4.

### Commands

- [x] `cargo build --workspace --locked` → kode 0 → AC-9
- [x] `cargo test --workspace --locked` → kode 0; tidak ada `FAILED`; uji Core tetap lulus tanpa diubah (`workspace_layout`, `contract_surface`, `hello_contract`, dan lainnya) → AC-9
- [x] `cargo fmt --all --check` → kode 0 → AC-9
- [x] `cargo clippy --workspace --all-targets --locked -- -D warnings` → kode 0 → AC-9
- [x] `cargo clippy -p desktop-macos-ffi --all-targets --all-features --locked -- -D warnings` → kode 0 (ini juga membangun bin `uniffi-bindgen-swift`) → AC-9
- [x] `cargo tree -p desktop-macos-ffi --depth 1 --prefix none --locked` → tepat tiga baris, diawali `desktop-macos-ffi v0.1.0`, `desktop-core v0.1.0`, dan `uniffi v0.32.2` → AC-9
- [x] `cargo tree -p desktop-macos-ffi --target all --all-features --prefix none --locked | grep -cE '^(desktop-windows|desktop-linux|gtk|gtk4|libadwaita|windows-reactor) '` → `0` → AC-9
- [x] `cargo tree -p desktop-core --target all --all-features --prefix none --locked` → tepat satu baris, diawali `desktop-core v0.1.0` → AC-9
- [x] `cargo test -p desktop-macos-ffi --locked` → kode 0; `test tests::forwards_to_core ... ok` → AC-6, AC-7
- [x] `grep -n 'unsafe_code' crates/macos-ffi/Cargo.toml` → `unsafe_code = "forbid"` di bawah `[lints.rust]`; `grep -rn unsafe crates/macos-ffi/src` → tidak mencetak apa pun (kode keluar `grep` 1) → AC-6
- [x] Dokumen crate di `crates/macos-ffi/src/lib.rs` → menyebut `docs/specs/macos/0001-sapaan-di-jendela-macos/index.md` dan merangkum aturan kepemilikan (setiap nilai disalin, Swift menyalin argumen ke buffer milik Rust, Rust mengembalikan buffer milik Rust yang dibebaskan kode Swift hasil generator) → AC-6
- [x] `grep -nE '^[[:space:]]*pub fn ' crates/macos-ffi/src/lib.rs` → tepat satu baris, `18:pub fn hello(name: String) -> String {` → AC-7
- [x] `grep -c 'desktop_core::hello(name)' crates/macos-ffi/src/lib.rs` → `1` → AC-7
- [x] `grep -c '^public func hello(name: String) -> String' bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift` → `1` → AC-7
- [x] Pada working tree yang bersih, jalankan keempat perintah pertama di bagian **Membuat ulang binding** spec (`cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin`, `rm -rf bindings/swift/Sources`, lalu dua perintah `cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- ...`) → keempatnya kode 0 → AC-1
- [x] `git diff --exit-code -- bindings/swift` → kode 0; `git status --short -- bindings/swift` → tidak mencetak apa pun → AC-1
- [x] `git ls-files bindings/swift` → tepat empat path: `bindings/swift/Package.swift`, `bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift`, `bindings/swift/Sources/desktop_macos_ffiFFI/desktop_macos_ffiFFI.h`, `bindings/swift/Sources/desktop_macos_ffiFFI/module.modulemap` → AC-1
- [x] `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin`, lalu `swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"` → kode 0; keluaran memuat `Test run with 7 tests in 1 suite passed` (`BindingContractTests`: `v1AyuReturnsHelloAyu`, `v2EmptyNameIsAccepted`, `v6UnicodeStaysIntact`, `v7NoNormalisation`, `v10NulInTheMiddleIsKept`, `v11LongNameIsNotCut`, `repeatedCallsFromManyThreads`) → AC-2, AC-6
- [x] Keluaran `swift test` yang sama memuat `Test run with 4 tests in 1 suite passed` (`GreetingViewModelTests`: `startsWithoutGreeting`, `showsWhatCoreReturns`, `sendsEmptyNameAsIs`, `sendsNameUnchangedAndShowsResultUnchanged`) → AC-3

### Acceptance-criteria coverage

- AC-1: langkah 15 sampai 17 · AC-2: langkah 18 · AC-3: langkah 19 · AC-6: langkah 9 sampai 11 dan 18 · AC-7: langkah 9 dan 12 sampai 14 · AC-9: langkah 1 sampai 8.
- AC-4 dan AC-8 menunggu milestone 2. AC-10, AC-11, dan AC-12 menunggu milestone 3 (AC-1 juga diulang di sana). AC-5 dan AC-13 menunggu milestone 4 (sesi GUI, `owner: user`).

## Milestone 2: Jendela SwiftUI dan proyek Xcode

Build plan langkah 4. Hasil yang tercatat diambil developer pada commit langkah 4 (`1344f3f`) di host yang sama.

### UI / manual

Sesi ini tidak membuka aplikasi atau Xcode GUI. Dua langkah manual menunggu user di milestone 4:

- **F004-1** (AC-5): langkah 1 sampai 5 di bagian **Rencana verifikasi manual** spec. Status `NOT_RUN`, `owner: user`.
- **F004-2** (AC-13): langkah 6 sampai 9 di bagian yang sama. Status `NOT_RUN`, `owner: user`.

### Commands

- [ ] `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build` → kode 0; `** BUILD SUCCEEDED **` → AC-4
- [ ] Perintah yang sama dengan `-configuration Release` → kode 0; `** BUILD SUCCEEDED **` → AC-4
- [ ] `lipo -archs apps/macos/DerivedData/Build/Products/<K>/DesktopMac.app/Contents/MacOS/DesktopMac`, untuk `<K>` = `Debug` dan `Release` → `arm64` untuk keduanya → AC-4
- [ ] `nm apps/macos/DerivedData/Build/Products/<K>/DesktopMac.app/Contents/MacOS/* 2>/dev/null | grep -c uniffi_desktop_macos_ffi_fn_func_hello`, untuk `Debug` dan `Release` → 1 atau lebih (tercatat `15` untuk Debug, `2` untuk Release) → AC-4
- [ ] `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin`, lalu `swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"` → kode 0; `Test run with 7 tests in 1 suite passed` dan `Test run with 4 tests in 1 suite passed` → AC-2, AC-3 (diulang)
- [ ] `git grep --untracked -lF 'Hello, ' -- '*.swift'` → tepat dua baris, `apps/macos/Tests/InteropTests/BindingContractTests.swift` dan `apps/macos/Tests/PresentationTests/GreetingViewModelTests.swift` → AC-8
- [ ] `git grep --untracked -l 'import DesktopMacosFFI' -- apps/macos` → tepat satu baris, `apps/macos/Interop/Greeter.swift` → AC-8
- [ ] `grep -c 'Text(verbatim: model.greeting' apps/macos/Presentation/GreetingView.swift` → `1` → AC-8
- [ ] `git grep --untracked -lF 'Hello, {' -- '*.rs'` → tepat satu baris, `crates/core/src/greeting.rs` → AC-8
- [ ] `git status --short` sesudah kedua build → tidak mencetak apa pun (`apps/macos/DerivedData/` dan `target/` diabaikan git) → AC-4

### Acceptance-criteria coverage

- AC-4: langkah 1 sampai 4 dan 10 · AC-8: langkah 6 sampai 9 · AC-2 dan AC-3 diulang di langkah 5.
- AC-5 dan AC-13 menunggu milestone 4 (F004-1 dan F004-2, `NOT_RUN`, `owner: user`). AC-1, AC-10, AC-11, dan AC-12 dibuktikan di milestone 3.

## Milestone 3: Bukti build, kombinasi versi, dan kebersihan repo

Build plan langkah 5. Developer menjalankan semua perintah di bawah berurutan pada working tree yang bersih di commit langkah 4 (`1344f3f`), mulai dari `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/` yang sudah dihapus, pada 2026-11-15. Commit sesudahnya hanya mengubah `verify.md` ini. Sesudah semua perintah, ketiga folder itu dihapus lagi.

Host dan arsitektur: Mac Apple Silicon, macOS `27.0.1`, `arm64`.

### UI / manual

Tidak ada untuk milestone ini. F004-1 (AC-5) dan F004-2 (AC-13) tetap `NOT_RUN`, `owner: user`, di milestone 4.

### Commands

- [ ] AC-1: `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin`, `rm -rf bindings/swift/Sources`, lalu kedua perintah `cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- ...` di **Membuat ulang binding** → keempatnya kode 0. `git diff --exit-code -- bindings/swift` → kode 0. `git status --short -- bindings/swift` → kosong. `git ls-files bindings/swift` → empat path: `bindings/swift/Package.swift`, `bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift`, `bindings/swift/Sources/desktop_macos_ffiFFI/desktop_macos_ffiFFI.h`, `bindings/swift/Sources/desktop_macos_ffiFFI/module.modulemap` → AC-1
- [ ] AC-2, AC-3: `swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"` → kode 0; `✔ Test run with 7 tests in 1 suite passed` dan `✔ Test run with 4 tests in 1 suite passed` → AC-2, AC-3
- [ ] AC-4: `xcodebuild ... -configuration Debug ... build` dan `-configuration Release` → keduanya `** BUILD SUCCEEDED **`; `lipo -archs` → `arm64` untuk keduanya; `nm ... | grep -c uniffi_desktop_macos_ffi_fn_func_hello` → `15` (Debug) dan `2` (Release) → AC-4
- [ ] AC-6: `grep -n unsafe_code crates/macos-ffi/Cargo.toml` → `26:unsafe_code = "forbid"`; `grep -rn unsafe crates/macos-ffi/src` → kosong (kode 1); `cargo test -p desktop-macos-ffi --locked` → `test tests::forwards_to_core ... ok` → AC-6
- [ ] AC-7: `grep -nE '^[[:space:]]*pub fn ' crates/macos-ffi/src/lib.rs` → `18:pub fn hello(name: String) -> String {`; `grep -c 'desktop_core::hello(name)' crates/macos-ffi/src/lib.rs` → `1`; `grep -c '^public func hello(name: String) -> String' bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift` → `1` → AC-7
- [ ] AC-8: `git grep --untracked -lF 'Hello, ' -- '*.swift'` → dua path di `apps/macos/Tests/`; `git grep --untracked -l 'import DesktopMacosFFI' -- apps/macos` → `apps/macos/Interop/Greeter.swift`; `grep -c 'Text(verbatim: model.greeting' apps/macos/Presentation/GreetingView.swift` → `1`; `git grep --untracked -lF 'Hello, {' -- '*.rs'` → `crates/core/src/greeting.rs` → AC-8
- [ ] AC-9: `cargo build --workspace --locked`, `cargo test --workspace --locked` (tidak ada `FAILED`), `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo clippy -p desktop-macos-ffi --all-targets --all-features --locked -- -D warnings` → semuanya kode 0. `cargo tree -p desktop-macos-ffi --depth 1 --prefix none --locked` → tiga baris, `desktop-macos-ffi v0.1.0`, `desktop-core v0.1.0`, `uniffi v0.32.2`. `cargo tree -p desktop-macos-ffi --target all --all-features --prefix none --locked | grep -cE '^(desktop-windows|desktop-linux|gtk|gtk4|libadwaita|windows-reactor) '` → `0`. `cargo tree -p desktop-core --target all --all-features --prefix none --locked` → satu baris, `desktop-core v0.1.0` → AC-9
- [ ] AC-10: keluaran perintah kombinasi versi, sama dengan tabel **Kombinasi yang dipakai** spec:
  - `sw_vers -productVersion` → `27.0.1`
  - `uname -m` → `arm64`
  - `xcodebuild -version` → `Xcode 27.1`, `Build version 27A9275`
  - `swift --version` → `Apple Swift version 6.4 (swiftlang-6.4.0.34.1 clang-2100.3.34.1)`, `Target: arm64-apple-macosx27.0.0`
  - `rustc --version` → `rustc 1.99.1 (58cf8f9c3 2026-10-13)`
  - `xcrun --sdk macosx --show-sdk-version` → `27.0`
  - `grep -A1 '^name = "uniffi"$' Cargo.lock` → `name = "uniffi"`, lalu `version = "0.32.2"`
  - `grep -c 'bindings_contract_version = 30' bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift` → `1`
  - `vtool -show-build apps/macos/DerivedData/Build/Products/Release/DesktopMac.app/Contents/MacOS/DesktopMac` → `platform MACOS`, `minos 14.0`, `sdk 27.0` → AC-10
- [ ] AC-11: `git status --short` sesudah perintah AC-1 sampai AC-4 → kosong. `git log --name-only --format= --grep='spec macos/0001 step' | sort -u` → hanya `Cargo.lock`, `Cargo.toml`, enam belas path dari tabel **Tata letak**, `docs/specs/macos/0001-sapaan-di-jendela-macos/index.md`, `docs/specs/macos/0001-sapaan-di-jendela-macos/verify.md`, dan empat berkas `graphify-out/`; tidak ada `.gitignore`, `rust-toolchain.toml`, atau `.graphifyignore`. `Cargo.toml` di commit langkah 1 (`9e14ced`) sama persis dengan bagian **Isi file** (`diff` kosong) → AC-11
- [ ] AC-12: `git ls-files apps/macos bindings/swift crates/macos-ffi` → tepat 16 path, sama dengan tabel **Tata letak**, tanpa `Platform`, `DesignSystem`, atau `Resources`. Daftar AC-11 tidak memuat path di bawah `scripts/` atau `.github/` (hitungan `0`). `grep -c ui-testing apps/macos/DesktopMac.xcodeproj/project.pbxproj` → `0` → AC-12
- [ ] Sesudah semua perintah: `rm -rf target apps/macos/.build apps/macos/DerivedData`, lalu `git status --short` → kosong

### Acceptance-criteria coverage

- AC-1: langkah 1 · AC-10: langkah 8 · AC-11: langkah 9 · AC-12: langkah 10. Langkah 2 sampai 7 mengulang AC-2, AC-3, AC-4, AC-6, AC-7, AC-8, dan AC-9 pada working tree yang bersih.
- AC-5 dan AC-13 menunggu milestone 4 (sesi GUI, `owner: user`).
