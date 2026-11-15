# Verify: Tema bersama dan kontrol native · spec _root/0003 · updated 2026-11-15

_Langkah ini diturunkan dari kriteria penerimaan spec _root/0003. `/check verify` menjalankannya, dan `/test` mengunci langkah yang tahan lama._

Kamu bisa menjalankan semua perintah dari root repo, dengan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. Hasil yang tercatat diambil developer pada commit pertama langkah 2 (`ed52dbd`) di Mac koordinasi (macOS 27.0.1, `arm64`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`) pada 2026-11-15, pada working tree yang bersih. Path absolut di keluaran `cargo tree` ikut lokasi checkout, jadi yang dicocokkan cukup awal barisnya. Kotak di bawah sengaja dibiarkan kosong untuk checker.

## Milestone 1: Alat dan sumber token

Build plan langkah 1 dan 2.

### UI / manual

Tidak ada. Milestone ini tidak punya tampilan, dan semua bukti berupa perintah.

### Commands

- [x] `git ls-files design-system` → tepat enam path: `design-system/schemas/colors.schema.json`, `spacing.schema.json`, `typography.schema.json`, lalu `design-system/tokens/colors.json`, `spacing.json`, `typography.json` → AC-1
- [x] `git ls-files tooling` → tepat tujuh path: `tooling/codegen/Cargo.toml`, `src/json.rs`, `src/lib.rs`, `src/main.rs`, `src/tokens/mod.rs`, `src/tokens/schema.rs`, `tests/tokens_contract.rs` → AC-1
- [x] `ls design-system/tokens/radius.json design-system/tokens/motion.json tooling/validation` → ketiganya `No such file or directory` → AC-1
- [x] Isi tiga file token, tiga file skema, dan `.gitattributes` dibandingkan dengan blok di bagian **Isi file** spec → sama persis (tercatat: ketujuh file sama byte demi byte) → AC-1, AC-7
- [x] `cargo run -p desktop-codegen --locked -- tokens check` → kode 0; baris terakhir `tokens: ok, 6 semantic tokens, 3 placeholder values, 3 outputs up to date` → AC-2, AC-8
- [x] `cargo test -p desktop-codegen --locked --test tokens_contract resolves_light_and_dark` → `1 passed` (nilai tabel **Nilai hasil**, urutan `accent`, `background`, `text.primary`, dan `placeholders == 3`) → AC-2, AC-8
- [x] Ganti `"#111111"` dengan `"#999999"` di `design-system/tokens/colors.json`, lalu `cargo run -p desktop-codegen --locked -- tokens check` → kode 1; stderr `design-system/tokens/colors.json: semantic.text.primary: contrast on background is 2.72 in light, needs 4.50 or more`, baris yang sama untuk `dark`, lalu `tokens: failed, 2 problems`. Kembalikan dengan `git checkout -- design-system` → AC-3, AC-4
- [x] Dengan sumber rusak yang sama, `rm design-system/schemas/colors.schema.json`, lalu `tokens generate` → kode 1, dua masalah yang sama, dan `ls design-system/schemas` hanya mencetak `spacing.schema.json` dan `typography.schema.json` (tidak ada yang ditulis). Kembalikan dengan `git checkout -- design-system` → AC-4
- [x] Hapus baris `"dark": "neutral.50",` dari `text.primary` di `colors.json`, lalu `tokens check` → kode 1; ``design-system/tokens/colors.json: missing field `dark` at line 24 column 5``, lalu `tokens: failed, 1 problems`. Kembalikan dengan `git checkout -- design-system` → AC-3, AC-4
- [x] Ganti `"schemaVersion": 1,` dengan `"schemaVersion": 1, "focus": {},` di `colors.json`, lalu `tokens generate` → kode 1; ``design-system/tokens/colors.json: unknown field `focus`, expected one of `$schema`, `schemaVersion`, `base`, `semantic` at line 3 column 29``, lalu `tokens: failed, 1 problems`. Kembalikan dengan `git checkout -- design-system`, lalu `git status --short` tidak mencetak apa pun → AC-4, AC-7
- [x] `cargo run -p desktop-codegen --locked -- tokens` dan `cargo run -p desktop-codegen --locked -- nope check` → masing masing kode 2; stderr `usage: desktop-codegen <source> <check|generate>` lalu `sources: tokens` → AC-4
- [x] `cargo test -p desktop-codegen --locked --test tokens_contract` → kode 0; `committed_outputs_match_the_source`, `generation_is_deterministic`, `resolves_light_and_dark`, `rejects_broken_sources` (18 kasus), dan `check_reports_stale_and_missing_outputs_and_generate_repairs_them` semuanya `ok`; `test result: ok. 5 passed` → AC-3, AC-4, AC-5, AC-6, AC-7
- [x] `cargo run -p desktop-codegen --locked -- tokens generate` pada working tree bersih → baris terakhir `tokens: ok, 6 semantic tokens, 3 placeholder values, 3 outputs, 0 written`; lalu `git diff --exit-code -- design-system` → kode 0 → AC-6
- [x] `rm design-system/schemas/*.schema.json`, lalu `tokens generate` → tiga baris `wrote design-system/schemas/colors.schema.json`, `wrote design-system/schemas/spacing.schema.json`, `wrote design-system/schemas/typography.schema.json`, baris terakhir berakhir `3 outputs, 3 written`; lalu `git status --short` tidak mencetak apa pun → AC-6
- [x] `git ls-files --eol design-system` → enam baris, semuanya `i/lf    w/lf    attr/text eol=lf` → AC-6
- [x] `grep -c '"additionalProperties": false' design-system/schemas/colors.schema.json` → `3`; untuk `spacing.schema.json` → `3`; untuk `typography.schema.json` → `2` → AC-7
- [x] `grep -c '"\$schema": "../schemas/' design-system/tokens/*.json` → `1` untuk tiap file → AC-7
- [x] `grep -c '"placeholder": true' design-system/tokens/colors.json` → `3` → AC-8
- [x] `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, dan `cargo clippy --workspace --all-targets --locked -- -D warnings` → semuanya kode 0; `cargo test --workspace` mencetak 13 baris `test result: ok` dan tidak ada `FAILED` (uji `workspace_layout`, `contract_surface`, dan `macos_bridge_ownership` tetap lulus tanpa diubah, dan 18 kasus sumber rusak lulus juga lewat `--workspace`) → AC-5, AC-9
- [x] `cargo tree -p desktop-codegen --depth 1 --prefix none --locked` → tepat tiga baris, diawali `desktop-codegen v0.1.0`, `serde v1.0.229`, `serde_json v1.0.154` → AC-9
- [x] `cargo tree -p desktop-codegen --target all --all-features --prefix none --locked | grep -cE '^(desktop-core|desktop-macos-ffi|desktop-windows|desktop-linux|uniffi|gtk|gtk4|libadwaita|windows-reactor) '` → `0` (kode keluar `grep` 1 karena tidak ada yang cocok) → AC-9
- [x] `cargo tree --workspace --locked -i desktop-codegen --prefix none` → tepat satu baris, diawali `desktop-codegen v0.1.0` → AC-9
- [x] `cargo tree -p desktop-core --target all --all-features --prefix none --locked` → tepat satu baris, diawali `desktop-core v0.1.0` → AC-9
- [x] `git diff f262895~1 f262895 -- Cargo.toml` (commit langkah 1) → hanya baris `members` berubah, menjadi `["crates/core", "crates/macos-ffi", "tooling/codegen"]` → AC-9
- [x] `git show f262895~1:Cargo.lock | grep -c '^\[\[package\]\]'` → `77`; `git show f262895:Cargo.lock | grep -c '^\[\[package\]\]'` → `78`; blok baru hanya `desktop-codegen` dengan dependensi `serde` dan `serde_json` → AC-9
- [x] `RUSTDOCFLAGS="-D warnings" cargo doc -p desktop-codegen --no-deps --locked` → kode 0 → AC-9
- [x] `git log --format=%s main..HEAD` → setiap subject memuat `spec _root/0003 step 1` atau `spec _root/0003 step 2` → AC-10
- [x] `git log --name-only --format= --grep='spec _root/0003 step [12]' | sort -u` → hanya `.gitattributes`, `Cargo.lock`, `Cargo.toml`, enam path `design-system/`, `docs/specs/_root/0003-tema-bersama-dan-kontrol-native/index.md`, empat file `graphify-out/`, dan tujuh path `tooling/codegen/` (tercatat sebelum commit `verify.md`); sesudah commit kedua langkah 2 hanya bertambah `docs/specs/_root/0003-tema-bersama-dan-kontrol-native/verify.md`. Tidak ada path di bawah `scripts/`, `.github/`, `apps/`, `crates/`, atau `bindings/` → AC-10
- [x] `git ls-files apps/macos bindings/swift crates/macos-ffi | wc -l` → `16` → AC-10

### Acceptance-criteria coverage

- AC-1: langkah 1 sampai 4 · AC-2: langkah 5 dan 6 · AC-3: langkah 7, 9, dan 12 · AC-4: langkah 7 sampai 12 · AC-5: langkah 12 dan 19 · AC-6: langkah 12 sampai 15 · AC-7: langkah 4, 10, 12, 16, dan 17 · AC-8: langkah 5, 6, dan 18 · AC-9: langkah 19 sampai 26 · AC-10: langkah 27 sampai 29.
- AC-11 menunggu milestone 2 (dokumen). AC-12 menunggu milestone 3 (host Windows dan Linux, `owner: user`).

## Milestone 2: Dokumen mengikuti keadaan nyata

Build plan langkah 3. Hasil yang tercatat diambil developer sebelum commit langkah 3, di atas `main` `abd3a43`, pada mesin yang sama.

### UI / manual

Tidak ada. Milestone ini hanya mengubah dokumen, dan semua bukti berupa perintah.

### Commands

- [ ] Blok `## Yang sudah ada` di `docs/design-system/README.md` dibandingkan dengan blok di bagian **Perubahan dokumen** spec → sama persis, kecuali kata `spec 0003` di kalimat terakhir yang menjadi tautan `[spec 0003](../specs/_root/0003-tema-bersama-dan-kontrol-native/index.md)`; letaknya sesudah paragraf "Bukti minimum kelak" dan sebelum baris `[Scope dan urutan kerja]` → AC-11
- [ ] Di `docs/architecture/README.md` bagian "Tata letak yang dituju", kalimat kedua berbunyi ``Root workspace, `crates/core/`, `crates/macos-ffi/`, `bindings/swift/`, `apps/macos/`, `design-system/`, dan `tooling/codegen/` sudah ada.`` → AC-11
- [ ] Di tabel "Yang sudah ada" dokumen arsitektur, tepat sesudah baris `apps/macos/`, ada baris `design-system/tokens/`, `design-system/schemas/` (dengan tautan `spec 0003`) lalu baris `tooling/codegen/`, sama dengan teks spec → AC-11
- [ ] `git diff abd3a43 -- docs/architecture/README.md` → hanya satu baris diganti dan dua baris ditambah; tabel "Aturan arah dependensi" tidak berubah → AC-11
- [ ] `grep -c 'desktop-codegen' docs/design-system/README.md` → `3` (2 atau lebih) → AC-11
- [ ] `grep -c 'tooling/codegen' docs/architecture/README.md` → `3` (2 atau lebih) → AC-11
- [ ] `(cd docs/design-system && test -f ../specs/_root/0003-tema-bersama-dan-kontrol-native/index.md) && (cd docs/architecture && test -f ../specs/_root/0003-tema-bersama-dan-kontrol-native/index.md)` → kode 0; tautan di kedua dokumen mengarah ke file yang ada → AC-11
- [ ] `cargo test -p desktop-core --locked` → kode 0; tidak ada `FAILED`, termasuk `workspace_layout` (`10 passed`) dan `architecture_rule_table_matches_the_spec_row_by_row` → AC-11

### Acceptance-criteria coverage

- AC-11: langkah 1 sampai 8.
- AC-1 sampai AC-10 sudah dibuktikan di milestone 1. AC-12 menunggu milestone 3 (host Windows dan Linux, `owner: user`).
