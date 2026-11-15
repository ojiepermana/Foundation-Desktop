# Verify: Batas workspace dan pemilik · spec _root/0001 · updated 2026-11-15

_Langkah ini diturunkan dari kriteria penerimaan spec _root/0001. `/check verify` menjalankannya, dan `/test` mengunci langkah yang tahan lama._

## Milestone 1: Workspace dan kerangka Core

Kamu bisa menjalankan semua perintah dari root repo, dengan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0` dan `target/` yang bersih. Keluaran yang tercatat diambil pada commit langkah 1 (`09c6f0c`) di Mac koordinasi (macOS 27.0.1, `arm64`, `rustc 1.99.1`) pada 2026-11-15. Path absolut di keluaran `cargo tree` ikut lokasi checkout, jadi yang dicocokkan cukup awal barisnya.

### UI / manual

Tidak ada. Milestone ini tidak punya tampilan, dan semua bukti berupa perintah.

### Commands

- [x] `cargo build --workspace --locked` → kode 0 → AC-1
- [x] `cargo tree --workspace --depth 0 --prefix none --locked` → tepat satu baris tidak kosong, `desktop-core v0.1.0 (<root>/crates/core)` → AC-1
- [x] `cargo build -p desktop-core --locked` → kode 0 → AC-2
- [x] `cargo test -p desktop-core --locked` → kode 0; uji unit dan uji dokumen masing masing `test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`; tidak ada `FAILED`. Repo tidak memuat proyek Xcode, binding Swift, crate penghubung, atau crate aplikasi → AC-2
- [x] `cargo tree -p desktop-core --target all --all-features --prefix none --locked` → tepat satu baris, `desktop-core v0.1.0 (<root>/crates/core)` → AC-3
- [x] `grep -cE '^\[(dev-dependencies|build-dependencies|features)\]|^\[target\.' crates/core/Cargo.toml` → `0`; `grep -c '^\[dependencies\]$' crates/core/Cargo.toml` → `1` (tabel kosong di akhir file); `test ! -e crates/core/build.rs` → kode 0 → AC-3
- [x] `rust-toolchain.toml` sama persis dengan "Isi file"; `rustc --version` → `rustc 1.99.1 (58cf8f9c3 2026-10-13)` → AC-4
- [x] `git ls-files --error-unmatch Cargo.lock` → kode 0; `grep -c '^\[\[package\]\]' Cargo.lock` → `1` → AC-4
- [x] Sesudah perintah AC-1 dan AC-2: `git status --short` → tidak mencetak apa pun; `git check-ignore -q target` → kode 0; `git ls-files graphify-out` → empat path (`GRAPH_REPORT.md`, `graph.html`, `graph.json`, `manifest.json`) → AC-5
- [x] `cargo fmt --all --check` → kode 0 → AC-6
- [x] `cargo clippy --workspace --all-targets --locked -- -D warnings` → kode 0 → AC-6
- [x] `git diff --name-only main...HEAD` di branch developer, atau `git log --name-only --format= --grep='spec _root/0001 step [12]' | sort -u` sesudah merge → hanya `.gitignore`, `.graphifyignore`, `Cargo.lock`, `Cargo.toml`, `crates/core/Cargo.toml`, `crates/core/src/lib.rs`, `rust-toolchain.toml`, empat file `graphify-out/`, serta (sesudah commit langkah 2) `verify.md` dan `index.md` spec ini → AC-7
- [x] Isi tujuh file pada commit langkah 1 dibandingkan dengan blok kode di bagian "Isi file" spec ini → ketujuhnya sama persis byte demi byte → AC-7
- [x] `grep -cE '^\[workspace\.(lints|dependencies)\]|^default-members|^exclude|^\[profile' Cargo.toml` → `0` → AC-7
- [x] `test ! -e xtask && test ! -e crates/xtask && test ! -e .cargo/config.toml` → kode 0 → AC-7

### Acceptance-criteria coverage

- AC-1: langkah 1 dan 2 · AC-2: langkah 3 dan 4 · AC-3: langkah 5 dan 6 · AC-4: langkah 7 dan 8 · AC-5: langkah 9 · AC-6: langkah 10 dan 11 · AC-7: langkah 12 sampai 15.
- AC-8 menunggu milestone 2 (langkah 3 Build plan, `docs/architecture/README.md`).
- AC-9 menunggu milestone 3 (langkah 4 Build plan, `BLOCKED`, `owner: user`, host Windows dan Linux).

## Milestone 2: Peta arsitektur mencatat tata letak nyata

Kamu bisa menjalankan semua perintah dari root repo. Perintah perbandingan tabel memakai process substitution, jadi jalankan di `bash` atau `zsh`. Hasil yang diharapkan di bawah dicoba developer pada commit langkah 3 di Mac koordinasi pada 2026-11-15. Checker yang mencentang kotaknya.

### UI / manual

Tidak ada. Milestone ini hanya mengubah dokumen, dan semua bukti berupa perintah.

### Commands

- [x] `grep -c 'desktop-core\|desktop-macos-ffi\|desktop-windows\|desktop-linux' docs/architecture/README.md` → angka 4 atau lebih (developer melihat `8`) → AC-8
- [x] `grep -c '1\.99\.1' docs/architecture/README.md` → angka 1 atau lebih (developer melihat `1`) → AC-8
- [x] `grep -c '^### Yang sudah ada$' docs/architecture/README.md` → `1`; `grep -c '^### Aturan arah dependensi$' docs/architecture/README.md` → `1` → AC-8
- [x] `diff <(awk '/^## Aturan arah dependensi$/{f=1;next} /^\*\*Aturan netral host/{f=0} f && /^\|/' docs/specs/_root/0001-batas-workspace-dan-pemilik/index.md) <(awk '/^### Aturan arah dependensi$/{f=1;next} /^## /{f=0} f && /^\|/' docs/architecture/README.md)` → tidak mencetak apa pun, kode 0; perintah `awk` kedua sendiri mencetak 7 baris (header, pemisah, lima aturan) → AC-8
- [x] `grep -o '(\.\./specs/_root/0001-batas-workspace-dan-pemilik/index\.md)' docs/architecture/README.md` → satu baris; `test -f docs/architecture/../specs/_root/0001-batas-workspace-dan-pemilik/index.md` → kode 0 → AC-8
- [x] `grep -c 'Pada tahap scope, folder implementasi masih berupa rencana\.' docs/architecture/README.md` → `0` → AC-8
- [x] `grep -c 'Root workspace nantinya memuat manifest Cargo, lockfile dan konfigurasi toolchain\.' docs/architecture/README.md` → `0` → AC-8
- [x] Bandingkan isi `docs/architecture/README.md` dengan bagian "Perubahan dokumen arsitektur" spec ini: dua kalimat pengganti dan dua subbagian baru (sebelum heading "Aturan pengembangan yang akan dicatat") sama teksnya, dan `git diff <commit sebelum langkah 3> -- docs/architecture/README.md` tidak mengubah baris lain, termasuk tabel "Pemilik dan arah dependensi" → AC-8

### Acceptance-criteria coverage

- AC-8: langkah 1 sampai 8 di bagian ini.
- AC-1 sampai AC-7 tercakup di Milestone 1. AC-9 menunggu milestone 3 (langkah 4 Build plan, `BLOCKED`, `owner: user`).
