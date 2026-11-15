# Rust Core (`desktop-core`)

## Overview

Paket `desktop-core` memegang perilaku yang dipakai bersama oleh ketiga aplikasi native. Saat ini isinya satu fungsi publik, `desktop_core::hello`. Crate ini dibangun dan diuji sendiri, tanpa pustaka UI, SDK khusus OS, atau dependensi apa pun.

## Key files

| File | Owns |
|---|---|
| `Cargo.toml` | Manifest paket. `[dependencies]` kosong, dan `[lints.rust]` memuat `unsafe_code = "forbid"` serta `missing_docs = "warn"`. |
| `src/lib.rs` | Akar crate dan satu satunya permukaan publik. Isinya hanya dokumen crate, `mod`, dan `pub use`. |
| `src/greeting.rs` | Modul privat berisi `hello` dan uji unitnya. Pola format sapaan hanya ada di sini. |
| `tests/hello_contract.rs` | Uji kontrak lewat path publik: vektor V-1 sampai V-11 dan tanda tangan `fn(String) -> String`. |
| `tests/hello_invariants.rs` | Uji invarian `hello` di luar vektor literal: panjang, awalan, akhiran, dan pemanggilan dari banyak thread. |
| `tests/contract_surface.rs` | Membaca `Cargo.toml`, `lib.rs`, dan `greeting.rs` sebagai teks untuk mengunci R-1, larangan dependensi, dan lint. |
| `tests/workspace_layout.rs` | Mengunci aturan workspace dari spec `_root/0001`, termasuk file di root repo dan tabel di `docs/architecture/README.md`. |
| `tests/macos_bridge_ownership.rs` | Mengunci aturan kepemilikan FFI dari spec `macos/0001` (AC-6): membaca sumber jembatan di `crates/macos-ffi/` dan file Swift tulisan tangan di `apps/macos/` sebagai teks. |

## Commands

Jalankan dari root repo.

```bash
cargo test -p desktop-core --locked
cargo test -p desktop-core --locked --doc
RUSTDOCFLAGS="-D warnings" cargo doc -p desktop-core --no-deps --locked

# Core tanpa dependensi, juga untuk uji: harus mencetak tepat satu baris
cargo tree -p desktop-core -e normal,build,dev --prefix none --locked

# Pola format sapaan di satu tempat: harus mencetak hanya crates/core/src/greeting.rs
git grep --untracked -lF 'Hello, {' -- '*.rs'
```

## Conventions

- Aturan API publik adalah R-1 sampai R-9 di spec `_root/0002` bagian "Aturan kontrak publik bersama". Baca bagian itu sebelum menyentuh `src/`. File ini tidak menyalinnya.
- Sebelum menambah atau mengubah item publik, spec `_root` yang mencatatnya harus sudah diterima (R-2). Perubahan itu lalu dicatat di tabel "Riwayat kontrak" spec `_root/0002`, dan versinya mengikuti R-5.
- Core tidak punya dependensi, termasuk `[dev-dependencies]`. Uji hanya memakai harness bawaan `cargo test`. Dependensi pertama diputuskan di spec yang menambah API yang membutuhkannya.
- Setiap fungsi publik punya uji unit di `mod tests` dalam file modulnya, dan uji kontrak di `tests/` yang hanya memakai path publik `desktop_core::...` (R-9).
- Nilai harapan di uji ditulis sebagai literal, atau diperiksa lewat awalan, akhiran, dan panjang. Uji tidak membentuk ulang harapan dengan pola format yang sama (spec `_root/0002`, AC-3).
- Setiap file di `tests/` diawali komentar dokumen `//!`, dan setiap item publik punya rustdoc dengan satu contoh yang berjalan (R-7).
- Uji di `tests/` menyebut ID vektor (`V-1` dan seterusnya) atau AC yang dibuktikannya, di komentar dokumen atau di pesan gagal.

## Gotchas

- `tests/contract_surface.rs`, `tests/workspace_layout.rs`, dan `tests/macos_bridge_ownership.rs` membaca file sumber, manifest, spec `_root/0001`, `docs/architecture/README.md`, sumber jembatan di `crates/macos-ffi/`, dan file Swift di `apps/macos/` sebagai teks. Mengubah baris `mod` atau `pub use` di `lib.rs`, tabel lint, rujukan spec di dokumen crate, salah satu tabel arah dependensi, dokumen crate atau lint jembatan, atau memakai pointer di Swift tulisan tangan bisa membuat `cargo test -p desktop-core` gagal walau kode Core benar. Perbarui spec lebih dulu, lalu ujinya.
- `missing_docs` juga memeriksa crate uji. File uji tanpa `//!` di awal gagal di `cargo clippy --workspace --all-targets --locked -- -D warnings` dengan pesan `missing documentation for the crate`.
- `hello` sengaja menerima `String` yang dimiliki (R-3). Atribut `#[allow(clippy::needless_pass_by_value, reason = ...)]` mencatat alasannya. Jangan diganti dengan `&str`.
- Rustdoc dan deskripsi paket Core berbahasa Inggris untuk sementara, sampai scope 002 memutuskan kebijakan bahasa komentar (spec `_root/0001` bagian "Isi file", spec `_root/0002` bagian "Follow-up"). Ikuti bahasa yang sudah ada di file yang kamu sentuh.

## Related specs

- [0001. Batas workspace dan pemilik](../../docs/specs/_root/0001-batas-workspace-dan-pemilik/index.md)
- [0002. Kontrak sapaan bersama](../../docs/specs/_root/0002-kontrak-sapaan-bersama/index.md)

_Drafted by /audit from the repo, worth a quick human pass. Edit freely: once a line stops matching this draft, later runs treat it as curated and will flag rather than overwrite it._
