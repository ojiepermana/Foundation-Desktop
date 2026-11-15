# Verify: Satu contoh lokalisasi · spec _root/0006 · updated 2026-11-15

_Langkah ini diturunkan dari kriteria penerimaan spec _root/0006. `/check verify` menjalankannya, dan `/test` mengunci langkah yang tahan lama._

Kamu bisa menjalankan semua perintah dari root repo, dengan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. Hasil yang tercatat diambil developer di Mac koordinasi (macOS 27.0.1, `arm64`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`) pada 2026-11-15. Path absolut di keluaran `cargo tree` ikut lokasi checkout, jadi yang dicocokkan cukup awal barisnya. Kotak di bawah sengaja dibiarkan kosong untuk checker.

## Milestone 1: Katalog dan sumber localization

Build plan langkah 1 dan 2. Hasil tercatat pada working tree bersih di atas uji langkah 2, sebelum `verify.md` ini masuk commit yang sama.

### UI / manual

Tidak ada. Milestone ini tidak punya tampilan, dan semua bukti berupa perintah.

### Commands

- [x] `git ls-files shared/localization` → tepat tiga path: `shared/localization/README.md`, `shared/localization/catalog.json`, `shared/localization/catalog.schema.json` → AC-1
- [x] Isi `catalog.json`, `README.md`, dan `catalog.schema.json` dibandingkan dengan blok di bagian **Isi berkas** spec → sama byte demi byte, masing masing diakhiri satu baris baru → AC-1, AC-8
- [x] `python3 -c 'import json;d=json.load(open("shared/localization/catalog.json"));print(d["schemaVersion"],d["sourceLocale"],",".join(d["locales"]),",".join(sorted(d["entries"])))'` → `1 en en,id greeting.submit` → AC-1
- [x] `cargo run -p desktop-codegen --locked -- localization check` → kode 0; baris terakhir `localization: ok, 1 keys, 2 locales, 0 missing translations, 1 outputs up to date` → AC-2. Angka `1 outputs` adalah keadaan pada akhir milestone 1; build spec `macos/0006` menaikkannya ke `3 outputs`
- [x] `cargo test -p desktop-codegen --locked --test localization_contract` → kode 0; `committed_outputs_match_the_source`, `committed_catalog_is_complete`, `real_catalog_resolves`, `generation_is_deterministic`, `placeholders_may_reorder`, `missing_translations_are_listed_and_fall_back`, dan `rejects_broken_catalogs` semuanya `ok`; `test result: ok. 7 passed` → AC-2, AC-3, AC-4, AC-5, AC-6, AC-7, AC-8, AC-9
- [x] Baca `rejects_broken_catalogs` di `tooling/codegen/tests/localization_contract.rs` → tabel `CASES` memuat 31 baris yang sama dengan tabel **Kasus sumber rusak** (Cari, Ganti, dan pesan), termasuk teks `serde` kasus 2, 4, dan 6 → AC-3 (kasus 7 sampai 11), AC-4 (12 sampai 19), AC-5 (20 sampai 30), AC-8 (1 sampai 6 dan 31)
- [x] `real_catalog_resolves` memeriksa `Entry.key` `greeting.submit`, teks `en` `Greet` dan `id` `Sapa` berasal `Origin::Own`, dan `missing` kosong → AC-3, AC-7
- [x] `placeholders_may_reorder` (K-1) memeriksa `placeholders` `name`, `app` → AC-5
- [x] `missing_translations_are_listed_and_fall_back` (K-2) memeriksa teks `id` dan `ms` bernilai `Greet` berasal `Origin::Fallback`, `en` berasal `Origin::Own`, `missing` `greeting.submit id` lalu `greeting.submit ms`, dan `summary` `plan` pada salinan di `CARGO_TARGET_TMPDIR` tepat `1 keys, 3 locales, 2 missing translations (greeting.submit id; greeting.submit ms)` → AC-6, AC-7
- [x] `cargo test --workspace --locked --test localization_contract` → `test result: ok. 7 passed` (31 kasus juga lulus dengan fitur crate yang disatukan) → AC-8
- [x] `grep -c '"additionalProperties": false' shared/localization/catalog.schema.json` → `2`; `grep -c '"\$schema": "./catalog.schema.json"' shared/localization/catalog.json` → `1` → AC-8
- [x] `cargo run -p desktop-codegen --locked -- localization generate` pada working tree bersih → baris terakhir `localization: ok, 1 keys, 2 locales, 0 missing translations, 1 outputs, 0 written`; lalu `git diff --exit-code -- shared/localization` → kode 0 → AC-9. Angka `1 outputs` adalah keadaan pada akhir milestone 1; build spec `macos/0006` menaikkannya ke `3 outputs`
- [x] `rm shared/localization/catalog.schema.json`, lalu `localization generate` → `wrote shared/localization/catalog.schema.json` dan baris terakhir berakhir `1 outputs, 1 written`; lalu `git status --short` tidak mencetak apa pun → AC-9. Angka `1 outputs` adalah keadaan pada akhir milestone 1; build spec `macos/0006` menaikkannya ke `3 outputs`
- [x] `git ls-files --eol 'shared/localization/*.json'` → dua baris, keduanya `i/lf    w/lf    attr/text eol=lf` → AC-9
- [x] `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, dan `cargo clippy --workspace --all-targets --locked -- -D warnings` → semuanya kode 0; `cargo test --workspace` menjalankan 99 test (92 yang sudah ada ditambah 7) dan tidak ada `FAILED` → AC-10
- [x] `cargo tree -p desktop-codegen --depth 1 --prefix none --locked` → tepat tiga baris, diawali `desktop-codegen v0.1.0`, `serde v1.0.229`, `serde_json v1.0.154` → AC-10
- [x] `cargo tree -p desktop-core -e normal,build,dev --prefix none --locked` → tepat satu baris, diawali `desktop-core v0.1.0` → AC-10
- [x] `git grep --untracked -lF 'Hello, {' -- '*.rs'` → tepat satu baris, `crates/core/src/greeting.rs` → AC-10
- [x] `git log --name-only --format= --grep='spec _root/0006 step' | sort -u` → hanya `.gitattributes`, `docs/specs/_root/0006-satu-contoh-lokalisasi/index.md`, empat file `graphify-out/`, tiga file `shared/localization/`, `tooling/codegen/src/lib.rs`, `tooling/codegen/src/localization/mod.rs`, `tooling/codegen/src/localization/schema.rs`, `tooling/codegen/tests/localization_contract.rs` (tercatat sebelum `verify.md` ikut di commit langkah 2; sesudahnya bertambah `verify.md` ini). `git diff --quiet main...HEAD -- Cargo.toml Cargo.lock` → kode 0 → AC-10
- [x] `cargo run -p desktop-codegen --locked -- tokens check` dan `-- config check` → tetap kode 0 dengan baris ok masing masing; `-- nope check` → kode 2, `sources: tokens, config, localization` → AC-10

### Acceptance-criteria coverage

- AC-1: langkah 1 sampai 3 · AC-2: langkah 4 dan 5 · AC-3: langkah 5 sampai 7 · AC-4: langkah 5 dan 6 · AC-5: langkah 5, 6, dan 8 · AC-6: langkah 5 dan 9 · AC-7: langkah 5, 7, dan 9 · AC-8: langkah 2, 5, 6, 10, dan 11 · AC-9: langkah 5 dan 12 sampai 14 · AC-10: langkah 15 sampai 20.
- AC-11 menunggu milestone 2.

## Milestone 2: Dokumen rujukan mencatat keadaan nyata

Build plan langkah 3. Hasil tercatat sebelum commit langkah 3, di atas commit langkah 2 (`0257e46`).

### UI / manual

Tidak ada. Milestone ini hanya mengubah dokumen, dan semua bukti berupa perintah.

### Commands

- [x] Paragraf di bawah `## Lokalisasi` di `docs/shared-resources/README.md` → sama persis dengan blok butir 1 bagian **Perubahan dokumen rujukan** spec; paragraf lama ("akan memiliki katalog istilah") tidak ada lagi → AC-11
- [x] `grep -c '0006-satu-contoh-lokalisasi' docs/shared-resources/README.md` → `1` (1 atau lebih) → AC-11
- [x] Kalimat "sudah ada." di bagian "Tata letak yang dituju" `docs/architecture/README.md` → ``Root workspace, `crates/core/`, `crates/macos-ffi/`, `bindings/swift/`, `apps/macos/`, `design-system/`, `shared/config/`, `tooling/codegen/`, dan `shared/localization/` sudah ada.`` (`shared/localization/` sesudah lokasi terakhir, `tooling/codegen/`) → AC-11
- [x] Baris terakhir tabel `### Yang sudah ada` di dokumen arsitektur → `| `shared/localization/` | Katalog bahasa bersama ... [spec 0006](../specs/_root/0006-satu-contoh-lokalisasi/index.md). |`, sesudah baris `shared/config/`, sama dengan butir 3 spec → AC-11
- [x] `(cd docs/shared-resources && test -f ../specs/_root/0006-satu-contoh-lokalisasi/index.md) && (cd docs/architecture && test -f ../specs/_root/0006-satu-contoh-lokalisasi/index.md)` → kode 0; tautan di kedua dokumen mengarah ke berkas yang ada → AC-11
- [x] `git diff 0257e46 -- docs/architecture/README.md` → satu baris diganti dan satu baris ditambah; tabel "Aturan arah dependensi" tidak berubah → AC-11
- [x] `cargo test -p desktop-core --locked` → kode 0, tidak ada `FAILED` (`workspace_layout` 10 passed) → AC-11. Sejak spec `_root/0007` (`c3448d9`) uji `workspace_layout` ada di `tooling/codegen/tests/`, jadi perintahnya sekarang `cargo test -p desktop-codegen --locked --test workspace_layout`

### Acceptance-criteria coverage

- AC-11: langkah 1 sampai 7.
- AC-1 sampai AC-10 dibuktikan di milestone 1. Semua AC spec ini tercakup.
