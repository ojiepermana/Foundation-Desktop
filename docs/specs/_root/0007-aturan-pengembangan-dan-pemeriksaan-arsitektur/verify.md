# Verify: Aturan pengembangan dan pemeriksaan arsitektur · spec _root/0007 · updated 2026-11-15

_Langkah ini diturunkan dari kriteria penerimaan spec _root/0007. `/check verify` menjalankannya, dan `/test` mengunci langkah yang tahan lama._

Kamu bisa menjalankan semua perintah dari root repo, dengan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. Hasil yang tercatat diambil developer di Mac koordinasi (macOS 27.0.1, `arm64`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`) pada 2026-11-15. Kotak di bawah sengaja dibiarkan kosong untuk checker.

## Milestone 1: Sumber architecture dan aturan dependensi

Build plan langkah 1 dan 2. Hasil tercatat pada commit uji langkah 2 (`076b806`), pada working tree yang bersih. Angka aturan pada akhir milestone ini `4 rules`.

### UI / manual

Tidak ada. Milestone ini tidak punya tampilan, dan semua bukti berupa perintah.

### Commands

- [x] `cargo run -p desktop-codegen --locked -- architecture check` → kode 0; baris terakhir `architecture: ok, 4 rules, 3 members, 0 outputs up to date` → AC-1
- [x] `cargo run -p desktop-codegen --locked -- architecture generate` → `architecture: ok, 4 rules, 3 members, 0 outputs, 0 written`; lalu `git status --short` tidak mencetak apa pun → AC-1
- [x] `cargo run -p desktop-codegen --locked -- nope check` → kode 2; baris kedua stderr `sources: tokens, config, localization, assets, architecture` → AC-1
- [x] `cargo test -p desktop-codegen --locked --test architecture_contract rejects_broken_repos` → `1 passed`; kasus `N-1` (`A-1`), `N-2` dan `N-3` (`A-2`), `N-4` dan `N-5` (`A-3`), `N-6` (`A-4`, pesan memuat `desktop-codegen`, `gtk4`, `class ui`) masing masing memberi masalah yang diawali ID itu → AC-2
- [x] Baca `REJECTED` di `tooling/codegen/tests/architecture_contract.rs` → perubahan tiap kasus sama dengan kolom Perubahan tabel **Kasus yang ditolak** untuk `N-1` sampai `N-6`; tiap kasus dijalankan pada salinan repo di `CARGO_TARGET_TMPDIR` dan dipulihkan sebelum kasus berikutnya → AC-2
- [x] `cargo test -p desktop-codegen --locked --test architecture_contract accepts_allowed_shapes` → `1 passed` (`P-1`, salinan tanpa perubahan) → AC-2
- [x] `printf '\n' >> design-system/schemas/colors.schema.json`, lalu `cargo test -p desktop-codegen --locked --test architecture_contract every_source_passes_check_on_the_repo` → kode bukan 0; keluaran memuat `tokens: design-system/schemas/colors.schema.json: differs from the generator output, run: cargo run -p desktop-codegen --locked -- tokens generate` dan `test result: FAILED` → AC-3
- [x] `git checkout -- design-system/schemas/colors.schema.json`, lalu perintah yang sama → `1 passed` → AC-3
- [x] `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, dan `cargo clippy --workspace --all-targets --locked -- -D warnings` → semuanya kode 0; `cargo test --workspace` menjalankan 140 test (137 yang sudah ada ditambah 3) tanpa `FAILED` → AC-3, AC-10
- [x] `git log --format=%s main..HEAD` → setiap subject memuat `spec _root/0007 step <n>` → AC-10
- [x] `git log --name-only --format= --grep='spec _root/0007 step' | sort -u` → hanya `docs/specs/_root/0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md`, `verify.md` spec ini (sesudah commit verify), empat file `graphify-out/`, `tooling/codegen/src/architecture/deps.rs`, `manifest.rs`, `mod.rs`, `tooling/codegen/src/lib.rs`, dan `tooling/codegen/tests/architecture_contract.rs`. Tidak ada `Cargo.toml`, `Cargo.lock`, `.gitattributes`, atau path di bawah `scripts/`, `.github/`, `.githooks/`, `apps/`, dan `bindings/` → AC-10

### Acceptance-criteria coverage

- AC-1: langkah 1 sampai 3 (angka `4 rules`; angka `13 rules` menunggu milestone 3) · AC-2: langkah 4 sampai 6 · AC-3: langkah 7 sampai 9 · AC-10: langkah 9 sampai 11.
- AC-4, AC-7, dan AC-8 menunggu milestone 2. AC-5, AC-6, dan AC-9 menunggu milestone 3. AC-11 menunggu milestone 4 (`BLOCKED`, `owner: user`).

## Milestone 2: Aturan kode dan uji lintas crate

Build plan langkah 3 dan 4. Hasil tercatat pada commit kode langkah 4 (`d951fa6`), pada working tree yang bersih. Pemindahan uji ada di commit langkah 3 (`f9dbf0b`). Angka aturan pada akhir milestone ini `11 rules`.

### UI / manual

Tidak ada. Milestone ini tidak punya tampilan, dan semua bukti berupa perintah.

### Commands

- [x] `cargo run -p desktop-codegen --locked -- architecture check` → kode 0; baris terakhir `architecture: ok, 11 rules, 3 members, 0 outputs up to date`; `architecture generate` → `architecture: ok, 11 rules, 3 members, 0 outputs, 0 written`, lalu `git status --short` kosong → AC-4
- [x] `cargo test -p desktop-codegen --locked --test architecture_contract` → `3 passed`; `rejects_broken_repos` memuat `N-1` sampai `N-19` (`N-7`, `N-8` → `A-5`; `N-9`, `N-10` → `A-6`; `N-11` → `A-7`; `N-12` → `A-8`; `N-13` → `A-8` dan `A-9`; `N-14` sampai `N-17` → `A-9`; `N-18` → `A-10`; `N-19` → `A-11`), dan `accepts_allowed_shapes` memuat `P-1`, `P-2`, `P-3` → AC-4, AC-8
- [x] `printf '\n// shared/config\n' >> crates/macos-ffi/src/lib.rs`, lalu `architecture check` → kode 1; stderr `crates/macos-ffi/src/lib.rs: A-7: Core and the bridge do not read configuration (shared/config)` lalu `architecture: failed, 1 problems` → AC-4
- [x] `git checkout -- crates/macos-ffi/src/lib.rs`, lalu `architecture check` → kode 0 → AC-4
- [x] `git ls-files crates/core/tests` → tepat tiga path: `crates/core/tests/contract_surface.rs`, `crates/core/tests/hello_contract.rs`, `crates/core/tests/hello_invariants.rs` → AC-7
- [x] `git ls-files tooling/codegen/tests` → memuat `tooling/codegen/tests/workspace_layout.rs` dan `tooling/codegen/tests/macos_bridge_ownership.rs` → AC-7
- [x] `git grep -n '\.\./\.\./\|CARGO_MANIFEST_DIR' -- 'crates/core/*.rs'` → tidak mencetak apa pun → AC-7
- [x] `cargo test -p desktop-core --locked` → kode 0 (unit 5, `contract_surface` 7, `hello_contract` 4, `hello_invariants` 5, doc 1); `cargo test -p desktop-codegen --locked` → kode 0, termasuk `workspace_layout` 10 passed dan `macos_bridge_ownership` 4 passed → AC-7
- [x] `git show --stat=200 -M --format= f9dbf0b` (commit langkah 3) → `{crates/core => tooling/codegen}/tests/macos_bridge_ownership.rs` dan `{crates/core => tooling/codegen}/tests/workspace_layout.rs`. Dengan lebar bawaan 80 kolom di luar terminal, `git show --stat -M --format=` memotong path menjadi `.../codegen}/tests/...`; `git show -M --summary --format= f9dbf0b` mencetak `rename ... (96%)` dan `rename ... (100%)` → AC-7
  - Keterangan (2026-11-15, pembaruan spec lewat `architect (update)`): teks AC-7 kini memakai `git show -M --summary --format= HEAD`, bentuk yang sudah dicatat di baris ini. Centang dan hasilnya tidak berubah.
- [x] Jumlah `#[test]`: `git show f9dbf0b~1:crates/core/tests/workspace_layout.rs | grep -c '#\[test\]'` → `10` dan untuk `macos_bridge_ownership.rs` → `4`; di lokasi baru `grep -c '#\[test\]' tooling/codegen/tests/workspace_layout.rs tooling/codegen/tests/macos_bridge_ownership.rs` → `10` dan `4`. `git diff f9dbf0b~1 f9dbf0b -M -- crates/core/tests tooling/codegen/tests` hanya mengubah dua baris `include_str!` di `macos_bridge_ownership.rs` → AC-7
- [x] `grep -c '^\[workspace\.lints' Cargo.toml` → `0`; `git ls-files | grep -cE '(^|/)\.?(rustfmt|clippy)\.toml$'` → `0` (kode keluar `grep` 1 untuk keduanya) → AC-8
- [x] `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, dan `cargo clippy --workspace --all-targets --locked -- -D warnings` → semuanya kode 0; `cargo test --workspace` menjalankan 140 test tanpa `FAILED` → AC-8
- [x] `git log --name-only --format= --grep='spec _root/0007 step' | sort -u` → hanya `docs/specs/_root/0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md`, `verify.md` spec ini, empat file `graphify-out/`, `tooling/codegen/src/architecture/code.rs`, `deps.rs`, `manifest.rs`, `mod.rs`, `tooling/codegen/src/lib.rs`, `tooling/codegen/tests/architecture_contract.rs`, `tooling/codegen/tests/macos_bridge_ownership.rs`, dan `tooling/codegen/tests/workspace_layout.rs`. Path lama dua uji tidak tercetak, karena `git log --name-only` tanpa `--follow` hanya menyebut path tujuan. Tidak ada `Cargo.toml`, `Cargo.lock`, `.gitattributes`, atau path di bawah `scripts/`, `.github/`, `.githooks/`, `apps/`, dan `bindings/` → AC-10

### Acceptance-criteria coverage

- AC-4: langkah 1 sampai 4 · AC-7: langkah 5 sampai 10 · AC-8: langkah 2, 11, dan 12 · AC-10: langkah 13.
- AC-1 dengan angka `13 rules`, AC-5, AC-6, dan AC-9 menunggu milestone 3. AC-11 menunggu milestone 4 (`BLOCKED`, `owner: user`).

## Milestone 3: Panduan dan dokumen mengikuti

Build plan langkah 5. Hasil tercatat pada commit langkah 5 (`4fa6f20`), pada working tree yang bersih. Angka aturan pada akhir milestone ini `13 rules`. Semua teks "Cari" di bagian **Perubahan dokumen** ditemukan tepat satu kali, jadi tidak ada beda yang perlu dicatat. Penggantian yang diterapkan: `AGENTS.md` 3, `crates/core/AGENTS.md` 5 butir (6 suntingan, karena butir 1 menghapus dua baris), `README.md` 3, `docs/architecture/README.md` 4, `docs/contracts/README.md` 2.

### UI / manual

Tidak ada. Milestone ini tidak punya tampilan, dan semua bukti berupa perintah.

### Commands

- [x] `cargo run -p desktop-codegen --locked -- architecture check` → kode 0; baris terakhir `architecture: ok, 13 rules, 3 members, 0 outputs up to date` → AC-1
- [x] `cargo run -p desktop-codegen --locked -- architecture generate` → `architecture: ok, 13 rules, 3 members, 0 outputs, 0 written`; lalu `git status --short` tidak mencetak apa pun → AC-1
- [x] `cargo run -p desktop-codegen --locked -- nope check` → kode 2; baris kedua stderr `sources: tokens, config, localization, assets, architecture` → AC-1
- [x] `grep -c 'cargo run -p desktop-codegen --locked -- architecture check' AGENTS.md` → `1`; `grep -c 'tooling/codegen/tests/workspace_layout.rs' AGENTS.md` → `1`; `grep -c 'crates/core/tests/workspace_layout.rs' AGENTS.md` → `0`; `grep -c 'workspace_layout\|macos_bridge_ownership' crates/core/AGENTS.md` → `0` → AC-5
- [x] `cargo test -p desktop-codegen --locked --test architecture_contract` → `3 passed`; `rejects_broken_repos` memuat `N-20` (`A-12`), `N-21`, `N-22`, `N-23` (`A-13`), dan `accepts_allowed_shapes` memuat `P-4` dan `P-5`, di samping kasus milestone 1 dan 2 → AC-5
- [x] Baca diff kelima dokumen (`git show 4fa6f20 -- AGENTS.md crates/core/AGENTS.md README.md docs/architecture/README.md docs/contracts/README.md`) → hanya teks dari bagian **Perubahan dokumen**: `AGENTS.md` (dua baris di blok `## Commands`, tujuh butir `## Rules` menggantikan butir "Uji mengunci aturan ini.", bagian `## Tinjauan manusia` sebelum `## Agent skills`), `crates/core/AGENTS.md` (dua baris `Key files` dihapus, satu butir `Conventions`, dua butir `Gotchas` diganti, satu butir `Related specs`), `README.md` (dua kalimat diganti, satu baris tabel sebelum baris terakhir), `docs/architecture/README.md` (baris `tooling/codegen/`, paragraf sesudah tautan spec 0001 dengan tautan `spec 0007`, dua kalimat), `docs/contracts/README.md` (dua kalimat) → AC-5, AC-9
- [x] `git show --stat=200 --format= 4fa6f20 -- AGENTS.md crates/core/AGENTS.md README.md docs/architecture/README.md docs/contracts/README.md` → `AGENTS.md | 26 +++++++++++++++++++++++++-`, `README.md | 5 +++--`, `crates/core/AGENTS.md | 8 ++++----`, `docs/architecture/README.md | 8 +++++---`, `docs/contracts/README.md | 2 +-`, `5 files changed, 38 insertions(+), 11 deletions(-)` → AC-5, AC-9
- [x] `grep -c '^## Tinjauan manusia' AGENTS.md` → `1`; `grep -c '^- \*\*H-[0-9]*\*\*' AGENTS.md` → `10` → AC-6
- [x] `grep -c 'Semua fitur berstatus' README.md`, `grep -c 'docs/decisions' docs/architecture/README.md`, `grep -c 'tooling/validation' docs/architecture/README.md`, dan ``grep -c 'Jika bagian `unsafe` diperlukan' docs/contracts/README.md`` → masing masing `0`; `test ! -e docs/decisions` → kode 0 → AC-9
- [x] `cargo test -p desktop-codegen --locked --test workspace_layout` → `10 passed`; tabel aturan arah dependensi tidak berubah → AC-9
- [x] `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, dan `cargo clippy --workspace --all-targets --locked -- -D warnings` → semuanya kode 0; `cargo test --workspace` menjalankan 159 test tanpa `FAILED` → AC-10
- [x] `cargo tree -p desktop-codegen --depth 1 --prefix none --locked` → tepat tiga baris, diawali `desktop-codegen v0.1.0`, `serde v1.0.229`, `serde_json v1.0.154`; `test ! -e tooling/validation` → kode 0 → AC-10
- [x] `git log --name-only --format= --grep='spec _root/0007 step' | sort -u` → hanya path dari tabel **File yang boleh diubah tiap milestone** baris 1 sampai 3 (lima dokumen, empat file `graphify-out/`, file di `tooling/codegen/src/architecture/`, `tooling/codegen/src/lib.rs`, tiga file uji di `tooling/codegen/tests/`, `index.md` dan `verify.md` spec ini). Tidak ada `Cargo.toml`, `Cargo.lock`, `.gitattributes`, atau path di bawah `scripts/`, `.github/`, `.githooks/`, `apps/`, dan `bindings/` → AC-10

### Acceptance-criteria coverage

- AC-1: langkah 1 sampai 3 · AC-5: langkah 4 sampai 7 · AC-6: langkah 8 · AC-9: langkah 6, 7, 9, dan 10 · AC-10: langkah 11 sampai 13.
- Catatan historis milestone 3: saat verifikasi ini dilakukan, salinan repo di uji ikut menyalin `.claude/` (yang tidak dipindai alat), karena `AGENTS.md` akar menautkan `.claude/skills/coordinate/`; tanpa itu `P-1` gagal dengan `A-12` pada salinan. Lihat laporan build. Skill lokal dan tautan itu dihapus pada 2026-11-17; salinan uji kini melewati `.claude/` sesuai aturan alat.
- AC-11 menunggu milestone 4 (`BLOCKED`, `owner: user`).
