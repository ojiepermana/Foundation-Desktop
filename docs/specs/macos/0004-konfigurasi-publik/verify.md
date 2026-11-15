# Verify: Konfigurasi publik di macOS · spec macos/0004 · updated 2026-11-15

_Langkah ini diturunkan dari kriteria penerimaan spec macos/0004. `/check verify` menjalankannya, dan `/test` mengunci langkah yang tahan lama._

Kamu bisa menjalankan semua perintah dari root repo, dengan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0` dan tiga nilai acuan dari bagian "Acceptance criteria":

```bash
NAME=$(jq -r '.build.product_name' shared/config/public.json)
BASE=$(jq -r '.build.app_identifier' shared/config/public.json)
VERSION=$(sed -n '/^\[workspace\.package\]$/,/^\[/s/^version = "\(.*\)"$/\1/p' Cargo.toml)
```

Dengan nilai sementara ketiganya `Desktop Foundation`, `com.example.desktop-foundation`, dan `0.1.0`. Hasil yang tercatat diambil developer di Mac koordinasi pada 2026-11-15, pada commit langkah 2 (`e187745`) dengan working tree yang bersih, mulai tanpa `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/`. Host: `sw_vers -productVersion` `27.0.1`, `uname -m` `arm64`, `xcodebuild -version` `Xcode 27.1`, `rustc --version` `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. Tidak ada jendela yang dibuka. Kotak di bawah sengaja dibiarkan kosong untuk checker.

## Milestone 1: Setelan Xcode dari sumber bersama

Build plan langkah 1 sampai 3. Langkah 1 di `352bde1`, langkah 2 di `e187745`.

### UI / manual

Tidak ada di milestone ini. AC-7 (judul jendela dan menu bar di sesi GUI) milik milestone 2, `BLOCKED`, `owner: user`.

### Commands

- [ ] `git ls-files apps/macos/Config` → tepat satu baris, `apps/macos/Config/Shared.xcconfig` → AC-1
- [ ] ``awk '/^```xcconfig$/{f=1;next} /^```$/{f=0} f' docs/specs/macos/0004-konfigurasi-publik/index.md | diff - apps/macos/Config/Shared.xcconfig`` → tidak mencetak apa pun, kode 0 → AC-1
- [ ] `cargo run -p desktop-codegen --locked -- config check` → kode 0; `config: ok, schema version 1, 2 build values, 2 outputs up to date` → AC-1
- [ ] `cargo run -p desktop-codegen --locked -- config generate` → `config: ok, schema version 1, 2 build values, 2 outputs, 0 written`; lalu `git status --short` tidak mencetak apa pun → AC-1
- [ ] `grep -c '^/apps/macos/Config/\*\.xcconfig text eol=lf$' .gitattributes` → `1` → AC-1
- [ ] `grep -c 'INFOPLIST_KEY_CFBundleDisplayName\|MARKETING_VERSION\|PRODUCT_BUNDLE_IDENTIFIER' apps/macos/DesktopMac.xcodeproj/project.pbxproj` → `0`; `grep -c 'baseConfigurationReference = D00000000000000000000014' apps/macos/DesktopMac.xcodeproj/project.pbxproj` → `2` → AC-2
- [ ] `grep -cxF "INFOPLIST_KEY_CFBundleDisplayName = $NAME" apps/macos/Config/Shared.xcconfig` → `1`; `grep -cxF "PRODUCT_BUNDLE_IDENTIFIER = $BASE.macos" apps/macos/Config/Shared.xcconfig` → `1` → AC-2
- [ ] `git grep -lF "$NAME" -- apps/macos` dan `git grep -lF "$BASE" -- apps/macos` → masing masing tepat satu baris, `apps/macos/Config/Shared.xcconfig` → AC-2
- [ ] `git show --stat --format= 352bde1 -- apps/macos/DesktopMac.xcodeproj/project.pbxproj` → `4 insertions(+), 6 deletions(-)` → AC-2, AC-9
- [ ] AC-3, untuk `<K>` `Debug` lalu `Release`: `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration <K> -showBuildSettings 2>/dev/null | grep -cxF -e "    INFOPLIST_KEY_CFBundleDisplayName = $NAME" -e "    MARKETING_VERSION = $VERSION" -e "    PRODUCT_BUNDLE_IDENTIFIER = $BASE.macos" -e "    PRODUCT_NAME = DesktopMac" -e "    CURRENT_PROJECT_VERSION = 1"` → `5` untuk keduanya → AC-3
- [ ] AC-4: sesudah `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration <K> -derivedDataPath apps/macos/DerivedData build` (`Debug` 29 detik, `Release` 44 detik, keduanya `** BUILD SUCCEEDED **`), dengan `P=apps/macos/DerivedData/Build/Products/<K>/DesktopMac.app/Contents/Info.plist`, untuk `Debug` dan `Release`: `plutil -extract CFBundleDisplayName raw -o - "$P"` → `Desktop Foundation`; `CFBundleIdentifier` → `com.example.desktop-foundation.macos`; `CFBundleShortVersionString` → `0.1.0`; `CFBundleVersion` → `1`; `CFBundleName` → `DesktopMac` → AC-4
- [ ] `cargo test -p desktop-codegen --locked --test config_macos_contract` → kode 0, `test result: ok. 11 passed`; `-- --list` → sebelas uji `x01_` sampai `x11_`; tiap uji tolak memeriksa ID aturan (`M-1` atau `M-2`) dan path `Problem` (`shared/config/public.json` atau `Cargo.toml`) → AC-5
- [ ] ``awk '/^```swift$/{f=1;next} /^```$/{f=0} f' docs/specs/macos/0004-konfigurasi-publik/index.md | diff - apps/macos/App/DesktopMacApp.swift`` → tidak mencetak apa pun; `grep -c 'WindowGroup("' apps/macos/App/DesktopMacApp.swift` → `0`; `grep -c 'forInfoDictionaryKey: "CFBundleDisplayName"' apps/macos/App/DesktopMacApp.swift` → `1` → AC-6
- [ ] `git grep -n 'shared/config\|public\.json' -- '*.swift' apps/macos/DesktopMac.xcodeproj crates/macos-ffi` → tidak mencetak apa pun; `grep -c 'shellScript = ' apps/macos/DesktopMac.xcodeproj/project.pbxproj` → `1` → AC-8
- [ ] `grep -vc '^//' apps/macos/Config/Shared.xcconfig` → `3`; untuk `MACOSX_DEPLOYMENT_TARGET = 14.0;`, `CURRENT_PROJECT_VERSION = 1;`, `PRODUCT_NAME = "$(TARGET_NAME)";`, `CODE_SIGN_IDENTITY = "-";`, `CODE_SIGN_STYLE = Automatic;`, dan `DEVELOPMENT_TEAM = "";`, `grep -cF '<teks>' apps/macos/DesktopMac.xcodeproj/project.pbxproj` → `2` masing masing → AC-9
- [ ] AC-10, perintah spec 004: `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin` (7 detik), lalu `swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"` (20 detik) → kode 0; keluaran memuat `✔ Suite BindingContractTests passed`, `Test run with 7 tests in 1 suite passed`, `✔ Suite GreetingViewModelTests passed`, dan `Test run with 4 tests in 1 suite passed` → AC-10
- [ ] AC-10, AC-4 spec 004: untuk `Debug` dan `Release`, `lipo -archs apps/macos/DerivedData/Build/Products/<K>/DesktopMac.app/Contents/MacOS/DesktopMac` → `arm64`; `nm apps/macos/DerivedData/Build/Products/<K>/DesktopMac.app/Contents/MacOS/* 2>/dev/null | grep -c uniffi_desktop_macos_ffi_fn_func_hello` → `15` (Debug) dan `2` (Release), keduanya 1 atau lebih → AC-10
- [ ] AC-10: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, dan `cargo test --workspace --locked` → kode 0; 189 test (178 sebelumnya ditambah 11), 0 ignored, termasuk `hand_written_swift_uses_no_pointers` dan keempat `committed_outputs_match_the_source` → AC-10
- [ ] Gerbang Mac spec `macos/0008` (pola `R` cocok dengan path fitur ini): `G-1` kode 0, 7 detik; `G-2` dua generator kode 0 dan `diff -r bindings/swift/Sources target/binding-check` kosong, 21 detik; `G-3` kode 0, 20 detik, 0 `failed`; `G-4` Debug `** BUILD SUCCEEDED **`, 29 detik; `G-5` `git status --short` kosong → AC-10
- [ ] Lima sumber: `cargo run -p desktop-codegen --locked -- <tokens|config|localization|assets|architecture> check` → semuanya kode 0; `architecture: ok, 13 rules, 3 members, 0 outputs up to date`; `git grep --untracked -lF 'Hello, {' -- '*.rs'` → tepat satu baris, `crates/core/src/greeting.rs` → AC-10
- [ ] `git diff --name-only main...HEAD` di branch developer → hanya `.gitattributes`, `apps/macos/App/DesktopMacApp.swift`, `apps/macos/Config/Shared.xcconfig`, `apps/macos/DesktopMac.xcodeproj/project.pbxproj`, `docs/specs/macos/0004-konfigurasi-publik/index.md`, `verify.md` spec ini (sesudah commit langkah 3), empat file `graphify-out/`, `tooling/codegen/src/config/macos.rs`, `tooling/codegen/src/config/mod.rs`, dan `tooling/codegen/tests/config_macos_contract.rs` → AC-11
- [ ] `git log --diff-filter=A --name-only --format= --grep='spec macos/0004 step' -- apps/macos bindings/swift crates/macos-ffi | sort -u` → tepat satu baris, `apps/macos/Config/Shared.xcconfig`; `git diff --quiet main...HEAD -- Cargo.toml Cargo.lock .gitignore shared/config tooling/codegen/Cargo.toml` → kode 0 → AC-11
- [ ] Sesudah semua perintah: `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/` dihapus; `git status --short` kosong → AC-11

### Acceptance-criteria coverage

- AC-1: langkah 1 sampai 5 · AC-2: langkah 6 sampai 9 · AC-3: langkah 10 · AC-4: langkah 11 · AC-5: langkah 12 · AC-6: langkah 13 · AC-8: langkah 14 · AC-9: langkah 9 dan 15 · AC-10: langkah 16 sampai 20 · AC-11: langkah 21 sampai 23.
- AC-7 menunggu milestone 2 (sesi GUI, `owner: user`, `F009-1`).
