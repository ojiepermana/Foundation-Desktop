# Verify: Konfigurasi publik · spec _root/0004 · updated 2026-11-15

_Langkah ini diturunkan dari kriteria penerimaan spec _root/0004. `/check verify` menjalankannya, dan `/test` mengunci langkah yang tahan lama._

Kamu bisa menjalankan semua perintah dari root repo, dengan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. Perintah `jq` memakai `/usr/bin/jq` bawaan macOS. Hasil yang tercatat diambil developer di Mac koordinasi (macOS 27.0.1, `arm64`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`) pada 2026-11-15. Kotak di bawah sengaja dibiarkan kosong untuk checker.

## Milestone 1: Sumber nilai, pemeriksaan, dan skema hasil generator

Build plan langkah 1 dan 2. Hasil tercatat pada commit langkah 1 (`0fa35bd`), pada working tree yang bersih.

### UI / manual

Tidak ada. Milestone ini tidak punya tampilan, dan semua bukti berupa perintah.

### Commands

- [x] `git ls-files shared/config` → tepat tiga baris: `shared/config/README.md`, `shared/config/public.json`, `shared/config/public.schema.json` → AC-1, AC-6
- [x] `awk '/^````markdown$/{f=1;next} /^````$/{f=0} f' docs/specs/_root/0004-konfigurasi-publik/index.md | diff - shared/config/README.md` → tidak mencetak apa pun, kode 0 → AC-1
- [x] `awk '/^```json$/{n++;f=(n==3);next} /^```$/{f=0} f' docs/specs/_root/0004-konfigurasi-publik/index.md | diff - shared/config/public.schema.json` → tidak mencetak apa pun, kode 0 → AC-1
- [x] `grep -cxF '/shared/config/**/*.json text eol=lf' .gitattributes` → `1` → AC-1
- [x] `jq -c . shared/config/public.json` → `{"$schema":"./public.schema.json","schemaVersion":1,"build":{"product_name":"Desktop Foundation","app_identifier":"com.example.desktop-foundation"}}` (nilai sementara dari "Nilai dari user") → AC-2
- [x] `awk '/^```json$/{n++;f=(n==2);next} /^```$/{f=0} f' docs/specs/_root/0004-konfigurasi-publik/index.md | diff - shared/config/public.json` → tidak mencetak apa pun, kode 0 → AC-2
- [x] `jq -r '.schemaVersion' shared/config/public.json` → `1`; `jq -r '.properties.schemaVersion.const' shared/config/public.schema.json` → `1` → AC-3
- [x] `grep '^| [0-9]' shared/config/README.md | tail -1` → `| 1 | 2026-11-15 | Skema pertama: ...| 0004 |`, diawali `| 1 |` → AC-3
- [x] `grep -c '"version"' shared/config/public.json` → `0` (kode keluar `grep` 1); `grep -c '^version = ' Cargo.toml` → `1` → AC-3
- [x] `cargo run -p desktop-codegen --locked -- config check` → kode 0; `config: ok, schema version 1, 2 build values, 1 outputs up to date` → AC-4
- [x] `touch shared/config/local.json`, lalu `config check` → kode 1; stderr `shared/config/local.json: R-1: unexpected entry` lalu `config: failed, 1 problems`. Sesudah `rm shared/config/local.json`, `config check` kembali kode 0 → AC-4, AC-6
- [x] `cargo run -p desktop-codegen --locked -- config generate` → kode 0; `config: ok, schema version 1, 2 build values, 1 outputs, 0 written`; lalu `git status --short` tidak mencetak apa pun → AC-4
- [x] `cargo test -p desktop-codegen --locked --test config_contract` → kode 0; `test result: ok. 33 passed` → AC-3, AC-5
- [x] `cargo test -p desktop-codegen --locked --test config_contract -- --list` → `committed_outputs_match_the_source` dan 32 uji `t01_` sampai `t32_` (`grep -c '^t[0-3][0-9]_'` mencetak `32`); uji `t25_`, `t26_`, `t27_` menolak dengan `R-3` → AC-3, AC-5
- [x] Baca `tooling/codegen/tests/config_contract.rs` → tiap uji `tNN_` membuat root sementara `config_contract_tNN` di bawah `CARGO_TARGET_TMPDIR` dan tidak menyunting `shared/config/`; uji tolak memakai `assert_rejected` dengan ID aturan dari kolom "Aturan"; uji `committed_outputs_match_the_source` memanggil `check(&repo_root(), &config::SOURCE)` → AC-5
- [x] `grep -n '^## Yang tidak boleh ada di folder ini' shared/config/README.md` → satu baris; `git ls-files shared/config | wc -l | tr -d ' '` → `3` → AC-6
- [x] `git grep -n -i 'DEVELOPMENT_TEAM\|CODE_SIGN\|password\|secret\|token' -- shared/config/public.json shared/config/public.schema.json` → tidak mencetak apa pun → AC-6
- [x] `grep -n '^## Cara aplikasi memakai nilai' shared/config/README.md` → satu baris; `grep -c '^- \*\*K-[1-7]\*\*' shared/config/README.md` → `7` → AC-7
- [x] `git grep -n 'shared/config\|public\.json' -- crates/core/src crates/macos-ffi/src bindings` → tidak mencetak apa pun → AC-7
- [x] `jq -c 'keys' shared/config/public.json` → `["$schema","build","schemaVersion"]` → AC-9
- [x] `git diff --name-only main...HEAD` di branch developer sebelum commit `verify.md` → hanya `.gitattributes`, `docs/specs/_root/0004-konfigurasi-publik/index.md`, empat file `graphify-out/`, tiga file `shared/config/`, `tooling/codegen/src/config/mod.rs`, `tooling/codegen/src/lib.rs`, dan `tooling/codegen/tests/config_contract.rs`; `git diff --quiet main...HEAD -- Cargo.toml Cargo.lock tooling/codegen/Cargo.toml apps crates bindings` → kode 0 → AC-9
- [x] `git show 0fa35bd -- tooling/codegen/src/lib.rs` → hanya baris `pub mod config;` ditambah dan baris `SOURCES` menjadi `&[tokens::SOURCE, config::SOURCE]` → AC-9
- [x] `cargo fmt --all --check` → kode 0; `cargo clippy --workspace --all-targets --locked -- -D warnings` → kode 0; `cargo test --workspace --locked` → kode 0, 14 baris `test result: ok`, tidak ada `FAILED` (`config_contract` 33 passed, `tokens_contract` 5 passed, uji `crates/core/tests/` tidak diubah) → AC-5, AC-10

### Acceptance-criteria coverage

- AC-1: langkah 1 sampai 4 · AC-2: langkah 5 dan 6 · AC-3: langkah 7 sampai 9, 13, dan 14 · AC-4: langkah 10 sampai 12 · AC-5: langkah 13 sampai 15 dan 23 · AC-6: langkah 1, 11, 16, dan 17 · AC-7: langkah 18 dan 19 · AC-9: langkah 20 sampai 22 · AC-10: langkah 23.
- AC-8 menunggu milestone 2. AC-9 diperiksa lagi di milestone 2 sesudah dokumen berubah.

## Milestone 2: Dokumen rujukan mencatat sumber yang nyata

Build plan langkah 3. Hasil tercatat sebelum commit langkah 3, di atas commit langkah 2 (`75fa02a`).

### UI / manual

Tidak ada. Milestone ini hanya mengubah dokumen, dan semua bukti berupa perintah.

### Commands

- [x] `grep -n '^## Batas pengaturan khusus OS' shared/config/README.md` → satu baris; di bawahnya ada tiga pertanyaan penentu bernomor dan tabel contoh untuk macOS, Windows, dan Linux → AC-8
- [x] Paragraf di bawah `## Konfigurasi` di `docs/shared-resources/README.md` → sama persis dengan blok pertama "Perubahan dokumen" spec, dengan `spec 0004` di kalimat terakhir menjadi tautan `[spec 0004](../specs/_root/0004-konfigurasi-publik/index.md)` → AC-8
- [x] `grep -c '0004-konfigurasi-publik' docs/shared-resources/README.md` → `1` → AC-8
- [x] `grep -c 'akan memiliki skema untuk nilai publik' docs/shared-resources/README.md` → `0` (kode keluar `grep` 1) → AC-8
- [x] `grep -c '^Path ini menunjukkan.*.shared/config/.*sudah ada\.' docs/architecture/README.md` → `1`; kalimatnya kini ``Root workspace, `crates/core/`, `crates/macos-ffi/`, `bindings/swift/`, `apps/macos/`, `design-system/`, `shared/config/`, dan `tooling/codegen/` sudah ada.`` (sisipan tepat sebelum `dan` terakhir; lokasi lain tidak berubah) → AC-8
- [x] `grep -c '^| .shared/config/. |' docs/architecture/README.md` → `1`; baris itu adalah baris terakhir tabel `### Yang sudah ada`, sesudah baris `tooling/codegen/`, dengan tautan `spec 0004` → AC-8
- [x] `(cd docs/shared-resources && test -f ../specs/_root/0004-konfigurasi-publik/index.md) && (cd docs/architecture && test -f ../specs/_root/0004-konfigurasi-publik/index.md)` → kode 0; kedua tautan mengarah ke file yang ada → AC-8
- [x] `git diff 75fa02a -- docs/architecture/README.md` → satu baris diganti dan satu baris ditambah; tabel "Aturan arah dependensi" tidak berubah. `cargo test -p desktop-core --locked` → kode 0, tidak ada `FAILED` (`workspace_layout` 10 passed) → AC-8
- [x] `jq -c 'keys' shared/config/public.json` → `["$schema","build","schemaVersion"]` → AC-9
- [x] `git diff --name-only main...HEAD` di branch developer → hanya `.gitattributes`, `docs/architecture/README.md`, `docs/shared-resources/README.md`, `docs/specs/_root/0004-konfigurasi-publik/index.md`, `docs/specs/_root/0004-konfigurasi-publik/verify.md`, empat file `graphify-out/`, tiga file `shared/config/`, `tooling/codegen/src/config/mod.rs`, `tooling/codegen/src/lib.rs`, dan `tooling/codegen/tests/config_contract.rs`; `git diff --quiet main...HEAD -- Cargo.toml Cargo.lock tooling/codegen/Cargo.toml apps crates bindings` → kode 0 → AC-9

### Acceptance-criteria coverage

- AC-8: langkah 1 sampai 8 · AC-9: langkah 9 dan 10.
- AC-1 sampai AC-7 dan AC-10 dibuktikan di milestone 1. Semua AC spec ini tercakup.
