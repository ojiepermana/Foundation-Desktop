# Verify: Satu contoh lokalisasi di macOS · spec macos/0006 · updated 2026-11-15

_Langkah ini diturunkan dari kriteria penerimaan spec macos/0006. `/check verify` menjalankannya, dan `/test` mengunci langkah yang tahan lama._

Kamu bisa menjalankan semua perintah dari root repo, dengan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0` di shell yang sama. Hasil yang tercatat diambil developer di Mac koordinasi pada 2026-11-15, pada commit langkah 2 (`b4a1bfe`) dengan working tree yang bersih, mulai tanpa `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/`. Host: macOS `27.0.1`, `arm64`, `Xcode 27.1` build `27A9275`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`. Tidak ada jendela yang dibuka. Commit induk langkah 1 adalah `033231a`. Kotak di bawah sengaja dibiarkan kosong untuk checker.

## Milestone 1: Resource dari katalog sampai tombol

Build plan langkah 1 sampai 3: langkah 1 `eb8aaab`, langkah 2 `b4a1bfe`.

### UI / manual

Tidak ada di milestone ini. AC-11 (bahasa di jendela nyata dan VoiceOver) milik milestone 2, `BLOCKED`, `owner: user` (`F010-1`).

### Commands

- [x] `cargo run -p desktop-codegen --locked -- localization check` → kode 0; baris terakhir `localization: ok, 1 keys, 2 locales, 0 missing translations, 3 outputs up to date` → AC-1
- [x] `cargo run -p desktop-codegen --locked -- localization generate` pada working tree bersih → berakhir `3 outputs, 0 written`; `git diff --exit-code -- shared/localization apps/macos` → kode 0 → AC-1
- [x] `rm -r apps/macos/Presentation/Resources`, lalu `localization generate` → `wrote apps/macos/Presentation/Resources/en.lproj/Localizable.strings`, lalu `wrote apps/macos/Presentation/Resources/id.lproj/Localizable.strings`, baris terakhir berakhir `3 outputs, 2 written`; `git status --short` tidak mencetak apa pun → AC-1
- [x] `git diff -U0 033231a HEAD -- .gitattributes | grep -c '^+[^+]'` → `1`, dan baris itu tepat `+/apps/macos/**/*.strings text eol=lf` → AC-1
- [x] `git ls-files --eol 'apps/macos/*.strings'` → dua baris, keduanya `i/lf    w/lf    attr/text eol=lf` → AC-1
- [x] Isi `apps/macos/Presentation/Resources/en.lproj/Localizable.strings` dan `id.lproj/Localizable.strings` dibandingkan dengan blok di **Isi berkas** spec → sama byte demi byte → AC-2
- [x] `cargo test -p desktop-codegen --locked --test localization_macos_contract` → `test result: ok. 7 passed` (`m01_` sampai `m07_`); `cargo test -p desktop-codegen --locked --test localization_rules every_output_follows_the_output_rules` → `1 passed`, tanpa mengubah uji itu; `cargo test -p desktop-codegen --locked` → kode 0, tidak ada `FAILED` → AC-2
- [x] `grep -c 'Button(LocalizedText.string(.greetingSubmit))' apps/macos/Presentation/GreetingView.swift` → `1`; `git grep -nF '"Greet"' -- 'apps/macos/App/*.swift' 'apps/macos/Presentation/*.swift' 'apps/macos/Interop/*.swift'` → tidak mencetak apa pun; `git grep -lF '"greeting.submit"' -- '*.swift'` → tepat `apps/macos/Presentation/LocalizedText.swift` → AC-3
- [x] `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin` (5 detik), lalu `swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"` (90 detik) → kode 0; keluaran memuat `✔ Suite LocalizedTextTests passed` dan `Test run with 10 tests in 2 suites passed` (`GreetingViewModelTests` 4 dan `LocalizedTextTests` 6); 0 `failed` → AC-4, AC-5, AC-6, AC-7
- [x] AC-5, percobaan gagal di worktree sekali pakai: `git worktree add --detach <scratch>/m06-probe HEAD`, tambahkan `    case probeMissing = "probe.missing"` tepat sesudah `case greetingSubmit = "greeting.submit"` di `<scratch>/m06-probe/apps/macos/Presentation/LocalizedText.swift`, lalu `swift test --package-path <scratch>/m06-probe/apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"` → kode 1 (60 detik); keluaran memuat `✘ Test everyKeyHasTextInEveryLanguage() recorded an issue at LocalizedTextTests.swift:33:17: Expectation failed: text != key.rawValue` (dua kali, sekali per bahasa) dan `✘ Test everyKeyHasTextInEveryLanguage() failed after ... with 2 issues`. `git worktree remove --force <scratch>/m06-probe` → worktree hilang dari `git worktree list`, `git status --short` kosong → AC-5
- [x] `git grep -nE 'LocalizedStringKey|String\(localized|NSLocalizedString|Text\("' -- apps/macos/Presentation apps/macos/App` → tidak mencetak apa pun; `percentSignsStayPlain` lulus di `swift test` di atas; `m04_` lulus di uji emitter → AC-6
- [x] `grep -c 'Text(verbatim: model.greeting' apps/macos/Presentation/GreetingView.swift` → `1`; `git grep --untracked -lF 'Hello, ' -- '*.swift'` → tepat tiga baris, semuanya di bawah `apps/macos/Tests/`: `InteropTests/BindingContractTests.swift`, `PresentationTests/GreetingViewModelTests.swift`, `PresentationTests/LocalizedTextTests.swift`; `greetingStaysEnglish` lulus di `swift test` di atas → AC-7
- [x] `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build` (141 detik) dan `-configuration Release` (152 detik, build Rust `--release` dingin) → `** BUILD SUCCEEDED **` untuk keduanya → AC-8
- [x] Untuk `<K>` `Debug` dan `Release`, dengan `<R>=apps/macos/DerivedData/Build/Products/<K>/DesktopMac.app/Contents/Resources/DesktopMacModules_Presentation.bundle/Contents/Resources`: `plutil -convert json -o - <R>/en.lproj/Localizable.strings` → `{"greeting.submit":"Greet"}`; untuk `id.lproj` → `{"greeting.submit":"Sapa"}`; `git diff --quiet 033231a HEAD -- apps/macos/DesktopMac.xcodeproj` → kode 0 → AC-8
- [x] `cargo build --workspace --locked`, `cargo test --workspace --locked` (208 test, 0 ignored), `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, dan `cargo run -p desktop-codegen --locked -- architecture check` (`architecture: ok, 13 rules, 3 members, 0 outputs up to date`) → kode 0 → AC-9
- [x] `swift test` yang sama dengan AC-4 juga memuat `Test run with 7 tests in 1 suite passed` (`BindingContractTests`) dan `Test run with 5 tests in 1 suite passed` (`ThemeTests` spec `macos/0005`) → AC-9
- [x] `cargo tree -p desktop-codegen --depth 1 --prefix none --locked | wc -l` → `3`; `cargo tree -p desktop-core --target all --all-features --prefix none --locked | wc -l` → `1` → AC-9
- [x] Gerbang Mac spec `macos/0008` (pola `R` cocok): `G-1` kode 0 (5 detik); `G-2` dua generator kode 0 dan `diff -r bindings/swift/Sources target/binding-check` kosong (71 detik); `G-3` = `swift test` di atas; `G-4` = build Debug di atas; `G-5` `git status --short` kosong. `/bin/bash scripts/validate.sh` → `rust tests: 208 passed, 0 failed, 0 ignored`, `macos gate: required, 10 changed paths match spec macos/0008 R`, `== validate.sh: PASS, 13 steps, 0 failed` → AC-9
- [x] `git log --name-only --format= --grep='spec macos/0006 step' | sort -u` → hanya sepuluh path di tabel **Tata letak**, `docs/specs/macos/0006-satu-contoh-lokalisasi/index.md`, `verify.md` spec ini (sesudah commit langkah 3), dan empat file `graphify-out/`. Tidak ada `apps/macos/DesktopMac.xcodeproj/`, `apps/macos/App/`, `apps/macos/Interop/`, `tooling/codegen/src/lib.rs`, `crates/`, `bindings/`, `scripts/`, `.github/`, `Cargo.toml`, atau `Cargo.lock` → AC-10
- [x] `git ls-files apps/macos bindings/swift crates/macos-ffi | wc -l | tr -d ' '` → `24`; pada commit induk langkah 1 (`git ls-tree -r --name-only 033231a -- apps/macos bindings/swift crates/macos-ffi | wc -l`) → `20`; bertambah tepat empat → AC-10
- [x] Sesudah semua perintah: `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/` dihapus; `git status --short` kosong → AC-10

### Acceptance-criteria coverage

- AC-1: langkah 1 sampai 5 · AC-2: langkah 6 dan 7 · AC-3: langkah 8 · AC-4: langkah 9 · AC-5: langkah 9 dan 10 · AC-6: langkah 9 dan 11 · AC-7: langkah 9 dan 12 · AC-8: langkah 13 dan 14 · AC-9: langkah 15 sampai 18 · AC-10: langkah 19 sampai 21.
- AC-11 menunggu milestone 2 (sesi GUI, `owner: user`, `F010-1`).
