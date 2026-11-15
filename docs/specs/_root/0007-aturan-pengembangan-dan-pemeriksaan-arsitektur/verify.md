# Verify: Aturan pengembangan dan pemeriksaan arsitektur · spec _root/0007 · updated 2026-11-15

_Langkah ini diturunkan dari kriteria penerimaan spec _root/0007. `/check verify` menjalankannya, dan `/test` mengunci langkah yang tahan lama._

Kamu bisa menjalankan semua perintah dari root repo, dengan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. Hasil yang tercatat diambil developer di Mac koordinasi (macOS 27.0.1, `arm64`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`) pada 2026-11-15. Kotak di bawah sengaja dibiarkan kosong untuk checker.

## Milestone 1: Sumber architecture dan aturan dependensi

Build plan langkah 1 dan 2. Hasil tercatat pada commit uji langkah 2 (`076b806`), pada working tree yang bersih. Angka aturan pada akhir milestone ini `4 rules`.

### UI / manual

Tidak ada. Milestone ini tidak punya tampilan, dan semua bukti berupa perintah.

### Commands

- [ ] `cargo run -p desktop-codegen --locked -- architecture check` → kode 0; baris terakhir `architecture: ok, 4 rules, 3 members, 0 outputs up to date` → AC-1
- [ ] `cargo run -p desktop-codegen --locked -- architecture generate` → `architecture: ok, 4 rules, 3 members, 0 outputs, 0 written`; lalu `git status --short` tidak mencetak apa pun → AC-1
- [ ] `cargo run -p desktop-codegen --locked -- nope check` → kode 2; baris kedua stderr `sources: tokens, config, localization, assets, architecture` → AC-1
- [ ] `cargo test -p desktop-codegen --locked --test architecture_contract rejects_broken_repos` → `1 passed`; kasus `N-1` (`A-1`), `N-2` dan `N-3` (`A-2`), `N-4` dan `N-5` (`A-3`), `N-6` (`A-4`, pesan memuat `desktop-codegen`, `gtk4`, `class ui`) masing masing memberi masalah yang diawali ID itu → AC-2
- [ ] Baca `REJECTED` di `tooling/codegen/tests/architecture_contract.rs` → perubahan tiap kasus sama dengan kolom Perubahan tabel **Kasus yang ditolak** untuk `N-1` sampai `N-6`; tiap kasus dijalankan pada salinan repo di `CARGO_TARGET_TMPDIR` dan dipulihkan sebelum kasus berikutnya → AC-2
- [ ] `cargo test -p desktop-codegen --locked --test architecture_contract accepts_allowed_shapes` → `1 passed` (`P-1`, salinan tanpa perubahan) → AC-2
- [ ] `printf '\n' >> design-system/schemas/colors.schema.json`, lalu `cargo test -p desktop-codegen --locked --test architecture_contract every_source_passes_check_on_the_repo` → kode bukan 0; keluaran memuat `tokens: design-system/schemas/colors.schema.json: differs from the generator output, run: cargo run -p desktop-codegen --locked -- tokens generate` dan `test result: FAILED` → AC-3
- [ ] `git checkout -- design-system/schemas/colors.schema.json`, lalu perintah yang sama → `1 passed` → AC-3
- [ ] `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, dan `cargo clippy --workspace --all-targets --locked -- -D warnings` → semuanya kode 0; `cargo test --workspace` menjalankan 140 test (137 yang sudah ada ditambah 3) tanpa `FAILED` → AC-3, AC-10
- [ ] `git log --format=%s main..HEAD` → setiap subject memuat `spec _root/0007 step <n>` → AC-10
- [ ] `git log --name-only --format= --grep='spec _root/0007 step' | sort -u` → hanya `docs/specs/_root/0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md`, `verify.md` spec ini (sesudah commit verify), empat file `graphify-out/`, `tooling/codegen/src/architecture/deps.rs`, `manifest.rs`, `mod.rs`, `tooling/codegen/src/lib.rs`, dan `tooling/codegen/tests/architecture_contract.rs`. Tidak ada `Cargo.toml`, `Cargo.lock`, `.gitattributes`, atau path di bawah `scripts/`, `.github/`, `.githooks/`, `apps/`, dan `bindings/` → AC-10

### Acceptance-criteria coverage

- AC-1: langkah 1 sampai 3 (angka `4 rules`; angka `13 rules` menunggu milestone 3) · AC-2: langkah 4 sampai 6 · AC-3: langkah 7 sampai 9 · AC-10: langkah 9 sampai 11.
- AC-4, AC-7, dan AC-8 menunggu milestone 2. AC-5, AC-6, dan AC-9 menunggu milestone 3. AC-11 menunggu milestone 4 (`BLOCKED`, `owner: user`).
