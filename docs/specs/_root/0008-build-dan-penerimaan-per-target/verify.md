# Verify: Build dan penerimaan per target · spec _root/0008 · updated 2026-11-15

_Langkah ini diturunkan dari kriteria penerimaan spec _root/0008. `/check verify` menjalankannya, dan `/test` mengunci langkah yang tahan lama._

Kamu bisa menjalankan semua perintah dari akar repo, dengan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, dan skrip selalu dipanggil dengan `/bin/bash` (bash `3.2.57(1)-release (arm64-apple-darwin26)` bawaan macOS) supaya kecocokan bash `3.2` benar benar teruji. Hasil yang tercatat diambil developer di Mac koordinasi (macOS `27.0.1`, `arm64`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1 (5f94df478 2026-08-27)`, Xcode 27.1 build 27A9275) pada 2026-11-15, dengan build hangat. AC-1 sampai AC-3 diambil pada commit langkah 1 (`b5a9005`), AC-4, AC-15, dan AC-16 pada commit langkah 2 (`f0c166e`), semuanya pada working tree yang bersih. Kotak di bawah sengaja dibiarkan kosong untuk checker.

## Milestone 1: Gerbang bersama

Build plan langkah 1 dan 2.

### UI / manual

Tidak ada untuk milestone ini. Langkah `F012-1` (host Windows dan Linux) dan `F012-2` (run GitHub Actions) milik user dan dicatat di milestone 3.

### Commands

- [ ] `/bin/bash scripts/validate.sh | grep -c '^commit: [0-9a-f]\{40\} (clean)$'` → `1` (tercatat `commit: b5a900547b3c94e417dab6d36bfeca051c53c880 (clean)`) → AC-1
- [ ] `/bin/bash scripts/validate.sh --nope` → kode `2`, stdout kosong (0 byte), stderr tepat satu baris `usage: bash scripts/validate.sh` → AC-1
- [ ] `/bin/bash scripts/build-rust.sh --x` dan `/bin/bash scripts/test.sh --x` → masing masing kode `2`, stdout kosong, stderr `usage: bash scripts/build-rust.sh` dan `usage: bash scripts/test.sh` → AC-1
- [ ] Keluaran ketiga skrip dibandingkan dengan bagian **Bentuk keluaran** spec → baris kepala `== <nama>.sh`, `host: Darwin arm64`, `commit: ... (clean)`, `rustc: ...`, `cargo: ...`, `macos: 27.0.1`, `xcode: Xcode 27.1 Build version 27A9275`, `swift: ...`; tiap langkah diapit `-- step <n>: <label>` dan `-- step <n>: PASS (exit 0, <detik>s)`; label langkah 1 `toolchain matches rust-toolchain.toml (1.99.1)` dengan keluaran `rustc 1.99.1 (58cf8f9c3 2026-10-13)`; baris penutup selalu baris terakhir → AC-1
- [ ] `/bin/bash scripts/validate.sh` → kode `0`; baris terakhir `== validate.sh: PASS, 13 steps, 0 failed`. Rumus: 8 + 5 nama di baris `sources: tokens, config, localization, assets, architecture` + 0 berkas `git ls-files '*brand-export.txt'` = 13 → AC-2
- [ ] Keluaran yang sama memuat lima baris sumber: `tokens: ok, 6 semantic tokens, 3 placeholder values, 3 outputs up to date`, `config: ok, schema version 1, 2 build values, 1 outputs up to date`, `localization: ok, 1 keys, 2 locales, 0 missing translations, 1 outputs up to date`, `assets: ok, 1 assets, 1 source files, 1 placeholder values, 1 outputs up to date`, `architecture: ok, 13 rules, 3 members, 0 outputs up to date` → AC-2
- [ ] Keluaran yang sama memuat `desktop-core v0.1.0 (...)` (langkah 6), keluaran langkah 7 tepat satu baris `crates/core/src/greeting.rs`, `rust tests: 178 passed, 0 failed, 0 ignored` (sama dengan jumlah `passed` dan `ignored` di 22 baris `test result:` pada `target/scripts/cargo-test.log`), `brand exports: 0 files`, dan `macos gate: required, 1 changed paths match spec macos/0008 R` (path itu `.gitattributes`) → AC-2
- [ ] `git status --short` sesudah `validate.sh` → kosong → AC-2
- [ ] `printf '\n' >> design-system/schemas/colors.schema.json`, lalu `/bin/bash scripts/validate.sh` → kode `1`, ke 13 langkah tetap dijalankan, baris terakhir `== validate.sh: FAIL, 13 steps, 2 failed (steps 5 9)`; langkah 5 (`cargo test --workspace --locked`) `FAIL (exit 101, ...)` dan langkah 9 (`tokens check`) `FAIL (exit 1, ...)` → AC-3
- [ ] `git checkout -- design-system/schemas/colors.schema.json`, lalu skrip yang sama → kode `0`, `== validate.sh: PASS, 13 steps, 0 failed` → AC-3
- [ ] `printf '// Hello, {name}\n' > probe.rs`, lalu skrip yang sama → kode `1`, baris terakhir `== validate.sh: FAIL, 13 steps, 1 failed (steps 7)`, keluaran langkah 7 `crates/core/src/greeting.rs` dan `probe.rs` → AC-3
- [ ] `rm probe.rs`, lalu skrip yang sama → kode `0`, `== validate.sh: PASS, 13 steps, 0 failed`; `git status --short` kosong → AC-3
- [ ] `/bin/bash scripts/build-rust.sh` → kode `0`, berakhir `== build-rust.sh: PASS, 2 steps, 0 failed` → AC-4
- [ ] `/bin/bash scripts/test.sh` → kode `0`, berakhir `rust tests: 178 passed, 0 failed, 0 ignored` lalu `== test.sh: PASS, 2 steps, 0 failed` → AC-4
- [ ] `grep -nE '^/(scripts/\*\.sh|\.github/workflows/\*\.yml) text eol=lf$' .gitattributes` → baris 6 `/scripts/*.sh text eol=lf` dan baris 7 `/.github/workflows/*.yml text eol=lf`, sesudah lima baris yang sudah ada → AC-15
- [ ] `git ls-files --eol -- 'scripts/*.sh' .github | grep -vc 'attr/text eol=lf'` → `0` → AC-15
- [ ] `git ls-files -s -- 'scripts/*.sh' | awk '$4!="scripts/lib.sh" && $1!="100755"'` dan `git ls-files -s -- scripts/lib.sh | awk '$1!="100644"'` → keduanya kosong (`build-rust.sh`, `test.sh`, `validate.sh` bermode `100755`, `lib.sh` `100644`) → AC-15
- [ ] `/bin/bash -n` untuk `scripts/build-rust.sh`, `scripts/lib.sh`, `scripts/test.sh`, dan `scripts/validate.sh` → keempatnya kode `0` → AC-15
- [ ] `git log --name-only --format= --grep='spec _root/0008 step' | grep -v '^graphify-out/' | sort -u` → hanya `.gitattributes`, `docs/specs/_root/0008-build-dan-penerimaan-per-target/index.md`, `docs/specs/_root/0008-build-dan-penerimaan-per-target/verify.md`, `scripts/build-rust.sh`, `scripts/lib.sh`, `scripts/test.sh`, `scripts/validate.sh`; tanpa `Cargo.toml`, `Cargo.lock`, `.githooks/`, `scripts/update_graphify.py`, atau `.cargo/` → AC-16
- [ ] `grep -nE 'git (commit|push|config|stash|add)|graphify|githooks|--no-verify|curl|wget' scripts/*.sh` → kosong (kode keluar `grep` 1) → AC-16
- [ ] `test ! -e scripts/build-windows.ps1 && test ! -e scripts/build-linux.sh` → kode `0` → AC-16

### Acceptance-criteria coverage

- AC-1: langkah 1 sampai 4 · AC-2: langkah 5 sampai 8 · AC-3: langkah 9 sampai 12 · AC-4: langkah 13 dan 14 · AC-15: langkah 15 sampai 18 · AC-16: langkah 19 sampai 21 (bagian milestone 1).
- AC-1 untuk ketiga skrip Mac, AC-5 sampai AC-9, dan AC-15 untuk skrip Mac menunggu milestone 2. AC-10, AC-13, AC-14, dan AC-16 untuk milestone 3 menunggu milestone 3. AC-11 dan AC-12 menunggu milestone 4 (`F012-1` dan `F012-2`, `owner: user`).
