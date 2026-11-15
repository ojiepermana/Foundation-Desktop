# Verify: Aset merek bersama · spec _root/0005 · updated 2026-11-15

_Langkah ini diturunkan dari kriteria penerimaan spec _root/0005. `/check verify` menjalankannya, dan `/test` mengunci langkah yang tahan lama._

Kamu bisa menjalankan semua perintah dari root repo, dengan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0` dan `SPEC=docs/specs/_root/0005-aset-merek-bersama/index.md`. Hasil yang tercatat diambil developer di Mac koordinasi (macOS 27.0.1, `arm64`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`) pada 2026-11-15. Kotak di bawah sengaja dibiarkan kosong untuk checker.

## Milestone 1: Sumber, manifest, aturan, dan pemeriksaan aset merek

Build plan langkah 1 dan 2. Hasil tercatat pada commit langkah 1 (`82890be`), pada working tree yang bersih.

### UI / manual

Tidak ada. Milestone ini tidak punya tampilan, dan semua bukti berupa perintah.

### Commands

- [ ] ``awk '/^````markdown$/{f=1;next} /^````$/{f=0} f' "$SPEC" | diff - shared/assets/README.md`` → kode 0 tanpa keluaran → AC-1
- [ ] `grep '^## ' shared/assets/README.md | tr '\n' '|'` → `## Pemilik|## Struktur|## Format sumber|## Penamaan|## Varian terang dan gelap|## Resolusi|## Versi|## Tujuan ekspor|## Pemeriksaan|## Mengganti atau menambah aset|` → AC-1
- [ ] `sed -n '/^## Pemilik$/,/^## Struktur$/p' shared/assets/README.md | grep -c '^| '` → `7` → AC-1
- [ ] `git ls-files shared/assets` → tepat empat path: `shared/assets/README.md`, `shared/assets/brand.json`, `shared/assets/brand.schema.json`, `shared/assets/source/app-icon.svg` → AC-2
- [ ] ``awk '/^```json$/{n++;f=(n==1);next} /^```$/{f=0} f' "$SPEC" | diff - shared/assets/brand.json`` dan ``awk '/^```xml$/{f=1;next} /^```$/{f=0} f' "$SPEC" | diff - shared/assets/source/app-icon.svg`` → masing masing kode 0 tanpa keluaran → AC-2
- [ ] `git ls-files | grep -v '^graphify-out/' | grep -ciE '\.(svg|png|ico|icns|jpe?g|gif|webp|pdf)$'` → `1` → AC-2
- [ ] `cargo run -p desktop-codegen --locked -- assets check` → kode 0; baris terakhir `assets: ok, 1 assets, 1 source files, 1 placeholder values, 1 outputs up to date` → AC-3
- [ ] `cargo test -p desktop-codegen --locked --test assets_contract reads_the_committed_brand` → `1 passed` → AC-3
- [ ] `cargo test -p desktop-codegen --locked --test assets_contract` → kode 0; `committed_outputs_match_the_source`, `generation_is_deterministic`, `reads_the_committed_brand`, `rejects_broken_manifests`, `rejects_broken_svgs`, `rejects_wrong_folder_contents`, dan `accepts_valid_sources` semuanya `ok`; `test result: ok. 7 passed` → AC-4, AC-5
- [ ] Baca `tooling/codegen/tests/assets_contract.rs` → `BROKEN_MANIFESTS` memuat M-1 sampai M-14, `BROKEN_SVGS` memuat S-1 sampai S-27 (S-7 memeriksa S-5 dan S-3), `WRONG_FOLDERS` memuat F-1 sampai F-13, dan `accepts_valid_sources` memeriksa M-15 sampai M-17, S-28 sampai S-31, dan F-14 sampai F-17 dengan `summary` dari tabel spec; setiap kasus menolak dengan ID aturan dari tabel → AC-4
- [ ] `touch shared/assets/notes.txt`, lalu `assets check` → kode 1; stderr `shared/assets/notes.txt: A-1: unexpected entry` lalu `assets: failed, 1 problems`. Hapus dengan `rm shared/assets/notes.txt` → AC-4
- [ ] `cargo run -p desktop-codegen --locked -- assets generate` pada working tree bersih → baris terakhir `assets: ok, 1 assets, 1 source files, 1 placeholder values, 1 outputs, 0 written`; `git diff --exit-code -- shared/assets` → kode 0 → AC-5
- [ ] `rm shared/assets/brand.schema.json`, lalu `assets generate` → `wrote shared/assets/brand.schema.json` dan baris terakhir berakhir `1 outputs, 1 written`; `git status --short` tidak mencetak apa pun → AC-5
- [ ] ``awk '/^```json$/{n++;f=(n==2);next} /^```$/{f=0} f' "$SPEC" | diff - shared/assets/brand.schema.json`` → kode 0 tanpa keluaran; `grep -c '"additionalProperties": false' shared/assets/brand.schema.json` → `3` → AC-5
- [ ] `git ls-files -s -- shared/assets/source | grep -cE '^100644 [0-9a-f]{40,64} 0[[:space:]]shared/assets/source/app-icon\.svg$'` → `1` → AC-6
- [ ] Di folder sementara di luar repo: `git ls-files -s -- shared/assets/source > <tmp>/brand-export.txt`, lalu `git ls-files -s -- shared/assets/source | diff - <tmp>/brand-export.txt` → kode 0 tanpa keluaran; sesudah satu digit id di berkas itu diubah, perintah yang sama → kode 1 → AC-6
- [ ] `git ls-files --eol shared/assets | grep -c 'attr/text eol=lf'` → `3` → AC-6
- [ ] `sed -n '/^## Tujuan ekspor$/,/^## Pemeriksaan$/p' shared/assets/README.md | grep -cE '^\| (macOS|Windows|Linux) \|'` → `3`; `grep -c '^\* \*\*E-[1-8]\*\*' shared/assets/README.md` → `8` → AC-7
- [ ] `git ls-files | grep -c 'brand-export\.txt$'` → `0` (kode keluar `grep` 1) → AC-7
- [ ] `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, dan `cargo clippy --workspace --all-targets --locked -- -D warnings` → semuanya kode 0; `cargo test --workspace` menjalankan 123 test (116 yang sudah ada ditambah 7) tanpa `FAILED` → AC-9
- [ ] `git log --format=%s main..HEAD` → setiap subject memuat `spec _root/0005 step <n>` → AC-9
- [ ] `git log --name-only --format= --grep='spec _root/0005 step' | sed '/^$/d' | sort -u` sesudah commit langkah 2 → hanya `.gitattributes`, `docs/specs/_root/0005-aset-merek-bersama/index.md`, `docs/specs/_root/0005-aset-merek-bersama/verify.md`, empat file `graphify-out/`, empat file `shared/assets/`, tiga file `tooling/codegen/src/assets/`, `tooling/codegen/src/lib.rs`, dan `tooling/codegen/tests/assets_contract.rs`. Tidak ada `Cargo.toml`, `Cargo.lock`, `.gitignore`, `.graphifyignore`, atau path di bawah `crates/`, `apps/`, `bindings/`, `scripts/`, `.github/` → AC-9

### Acceptance-criteria coverage

- AC-1: langkah 1 sampai 3 · AC-2: langkah 4 sampai 6 · AC-3: langkah 7 dan 8 · AC-4: langkah 9 sampai 11 · AC-5: langkah 9 dan 12 sampai 14 · AC-6: langkah 15 sampai 17 · AC-7: langkah 18 dan 19 · AC-9: langkah 20 sampai 22.
- AC-8 menunggu milestone 2. AC-10 menunggu milestone 3 (berkas merek nyata dari user, `owner: user`).

## Milestone 2: Dokumen rujukan menunjuk aturan aset

Build plan langkah 3. Hasil grep dan tautan tercatat sebelum commit langkah 3, di atas commit langkah 2 (`01fffbd`). Perintah `git log` dijalankan lagi sesudah commit langkah 3.

### UI / manual

Tidak ada. Milestone ini hanya mengubah dokumen, dan semua bukti berupa perintah.

### Commands

- [ ] Paragraf di bawah `## Aset merek` di `docs/shared-resources/README.md` → diawali tiga kalimat dari butir 1 bagian "Perubahan dokumen rujukan" spec, disusul tiga kalimat lama ("Ikon sistem boleh mengikuti OS. ...") tanpa perubahan → AC-8
- [ ] `grep -c 'shared/assets/README.md' docs/shared-resources/README.md` → `1`; `grep -c '0005-aset-merek-bersama' docs/shared-resources/README.md` → `1`; `grep -c 'akan memiliki berkas sumber' docs/shared-resources/README.md` → `0` (kode keluar `grep` 1) → AC-8
- [ ] `grep -c 'Root workspace.*`shared/assets/`.*sudah ada' docs/architecture/README.md` → `1`; kalimatnya kini ``..., `tooling/codegen/`, `shared/assets/`, dan `shared/localization/` sudah ada.`` (sisipan tepat sebelum `dan` terakhir; lokasi lain tidak berubah) → AC-8
- [ ] `grep -c '^| `shared/assets/` |' docs/architecture/README.md` → `1`; baris itu adalah baris terakhir tabel `### Yang sudah ada`, sesudah baris `shared/localization/`, sama dengan butir 3 spec → AC-8
- [ ] `test -f shared/assets/README.md && test -f docs/specs/_root/0005-aset-merek-bersama/index.md; echo $?` → `0`; dari `docs/shared-resources/`, path relatif `../../shared/assets/README.md` dan `../specs/_root/0005-aset-merek-bersama/index.md` ada, dan dari `docs/architecture/` path `../specs/_root/0005-aset-merek-bersama/index.md` ada → AC-8
- [ ] `git diff 01fffbd -- docs/architecture/README.md` → satu baris diganti dan satu baris ditambah; tabel "Aturan arah dependensi" tidak berubah. `cargo test -p desktop-core --locked` → kode 0, tidak ada `FAILED` (`workspace_layout` 10 passed) → AC-8
- [ ] `git log --name-only --format= --grep='spec _root/0005 step' | sed '/^$/d' | sort -u` sesudah commit langkah 3 → daftar milestone 1 ditambah `docs/architecture/README.md` dan `docs/shared-resources/README.md`; tetap tanpa `Cargo.toml`, `Cargo.lock`, `.gitignore`, `.graphifyignore`, atau path di bawah `crates/`, `apps/`, `bindings/`, `scripts/`, `.github/` → AC-9

### Acceptance-criteria coverage

- AC-8: langkah 1 sampai 6 · AC-9: langkah 7.
- AC-1 sampai AC-7 dan AC-9 (gerbang) dibuktikan di milestone 1. AC-10 menunggu milestone 3 (`BLOCKED`, `owner: user`).
