# Verify: Kontrak sapaan bersama · spec _root/0002 · updated 2026-11-15

_Langkah ini diturunkan dari kriteria penerimaan spec _root/0002. `/check verify` menjalankannya, dan `/test` mengunci langkah yang tahan lama._

Kamu bisa menjalankan semua perintah dari root repo, dengan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. Hasil yang tercatat diambil developer pada commit langkah 5 (`bfb12a9`) di Mac koordinasi (macOS 27.0.1, `arm64`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`) pada 2026-11-15, mulai dari `target/` yang bersih. Path absolut di keluaran `cargo tree` ikut lokasi checkout, jadi yang dicocokkan cukup awal barisnya. Kotak di bawah sengaja dibiarkan kosong untuk checker.

## Milestone 1: fungsi hello dan uji Done when

Build plan langkah 1 dan 2.

### UI / manual

Tidak ada. Milestone ini tidak punya tampilan, dan semua bukti berupa perintah.

### Commands

- [x] `cargo test -p desktop-core --locked --test hello_contract` → kode 0; `ayu_returns_hello_ayu ... ok` dan `signature_is_pinned ... ok` → AC-1, AC-4
- [x] `cargo build -p desktop-core --locked` → kode 0 → AC-2
- [x] `cargo test -p desktop-core --locked` → kode 0; tidak ada `FAILED` → AC-2
- [x] `cargo tree -p desktop-core -e normal,build,dev --prefix none --locked` → tepat satu baris, diawali `desktop-core v` → AC-2
- [x] `grep -E '^(pub|mod) ' crates/core/src/lib.rs` → tepat dua baris, `mod greeting;` lalu `pub use greeting::hello;` → AC-4
- [x] `crates/core/tests/hello_contract.rs` hanya memuat `use desktop_core::hello;` dari crate Core, dan `signature_is_pinned` berisi `let pinned: fn(String) -> String = hello;` → AC-1, AC-4

### Acceptance-criteria coverage

- AC-1: langkah 1 dan 6 · AC-2: langkah 2 sampai 4 · AC-4: langkah 1, 5, dan 6.
- AC-2 (fmt dan clippy), AC-3, AC-5, AC-6, dan AC-7 menunggu milestone 2.

## Milestone 2: masukan tepi, aturan kontrak, dan bukti

Build plan langkah 3 sampai 6.

### UI / manual

Tidak ada. Milestone ini tidak punya tampilan, dan semua bukti berupa perintah.

### Commands

- [x] `cargo test -p desktop-core --locked` → kode 0; uji unit `5 passed` (`greets_by_name`, `accepts_empty_name`, `keeps_whitespace_as_given`, `same_input_gives_same_output`, `adds_exactly_eight_bytes`), uji kontrak `4 passed` (`ayu_returns_hello_ayu`, `signature_is_pinned`, `vectors_match_contract`, `long_name_is_not_cut`), doc test `1 passed` → AC-2, AC-5, AC-6
- [x] `cargo fmt -p desktop-core --check` → kode 0 → AC-2
- [x] `cargo clippy -p desktop-core --all-targets --locked -- -D warnings` → kode 0 → AC-2
- [x] `cargo tree -p desktop-core -e normal,build,dev --prefix none --locked` → tepat satu baris, diawali `desktop-core v` → AC-2
- [x] `git grep --untracked -lF 'Hello, {' -- '*.rs'` → tepat satu baris, `crates/core/src/greeting.rs` → AC-3
- [x] Baca `crates/core/tests/hello_contract.rs` dan `mod tests` di `greeting.rs` → harapan berupa literal atau pemeriksaan awalan, akhiran, panjang, dan bagian tengah; tidak ada `format!` yang membentuk ulang sapaan → AC-3
- [x] `grep -E '^(pub|mod) ' crates/core/src/lib.rs` → tepat dua baris, `mod greeting;` lalu `pub use greeting::hello;` → AC-4
- [x] Tabel `VECTORS` di `hello_contract.rs` dibandingkan dengan tabel **Vektor kontrak** spec → V-1 sampai V-10 sama persis, termasuk pemeriksaan tidak sama V-7; `long_name_is_not_cut` memeriksa panjang 1.048.584, awalan, akhiran, dan bagian tengah → AC-5
- [x] `sed '/#\[cfg(test)\]/,$d' crates/core/src/greeting.rs | grep -nE '^[[:space:]]*(pub )?(use|static|extern|unsafe)[[:space:]]'` → tidak mencetak apa pun (kode keluar `grep` 1) → AC-6
- [x] `grep -E '^(unsafe_code|missing_docs)' crates/core/Cargo.toml` → `unsafe_code = "forbid"` dan `missing_docs = "warn"`, keduanya di bawah `[lints.rust]` → AC-6, AC-7
- [x] Dokumen crate di `crates/core/src/lib.rs` memuat `# Public contract`, path `docs/specs/_root/0002-kontrak-sapaan-bersama/index.md`, dan ringkasan R-1 sampai R-9; empat baris pertama sama dengan kerangka spec `_root/0001` → AC-7
- [x] `cargo test -p desktop-core --locked --doc` → kode 0; `crates/core/src/greeting.rs - greeting::hello ... ok`, `1 passed` → AC-7
- [x] `RUSTDOCFLAGS="-D warnings" cargo doc -p desktop-core --no-deps --locked` → kode 0 → AC-7
- [x] `cargo fmt --all --check` dan `cargo clippy --workspace --all-targets --locked -- -D warnings` → kode 0 → AC-2
- [x] `cargo tree -p desktop-core --target all --all-features --prefix none --locked` → tepat satu baris; `git diff --quiet main -- Cargo.lock` → kode 0 (batas spec `_root/0001` tetap terjaga) → AC-2

### Acceptance-criteria coverage

- AC-2: langkah 1 sampai 4, 14, dan 15 · AC-3: langkah 5 dan 6 · AC-4: langkah 7 · AC-5: langkah 1 dan 8 · AC-6: langkah 1, 9, dan 10 · AC-7: langkah 10 sampai 13.
- AC-1 sudah dibuktikan di milestone 1 dan ikut lulus di langkah 1 milestone ini.
- Semua AC spec ini tercakup; tidak ada yang menunggu milestone lain.
