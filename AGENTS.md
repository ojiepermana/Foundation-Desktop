# Aturan commit dan integrasi

Aktifkan hook sekali di setiap clone dengan `git config --local core.hooksPath .githooks`. Hook memerlukan Python 3 dan CLI `graphify` di PATH. Untuk memasang Graphify, gunakan `uv tool install graphifyy`.

1. Sebelum commit, perbarui dokumentasi yang terdampak, stage sumber yang ingin disimpan, lalu periksa `git diff --cached`.
2. Saat `git commit`, hook memperbarui Graphify dari salinan indeks Git dan menyertakan perubahan JSON, HTML, laporan, dan manifest dalam commit yang sama. Perubahan unstaged serta riwayat kueri lokal tetap di mesin. Jika pembaruan gagal, perbaiki penyebab yang dilaporkan dan ulangi commit.
3. Pembaruan lokal membaca struktur kode serta judul dan tautan Markdown. Untuk memperbarui konsep dan hubungan semantik dalam isi dokumen, jalankan `/graphify . --update` dari salinan sumber yang akan di-commit dan stage keempat hasilnya.
4. Saat mengirim perubahan ke `main`, ambil `origin/main` terbaru, gabungkan branch kerja, lalu push ke `origin/main`. Jika Git perlu membuat commit penggabungan, gunakan `git merge --no-commit <branch>` lalu `git commit` agar hook menyertakan graf hasil penggabungan. Pastikan SHA pada remote sama dengan SHA lokal yang dikirim.

# Aturan pengembangan

Tiga aplikasi desktop native memakai satu Rust Core. Yang sudah ada baru Cargo Workspace dan `crates/core/`; lokasi lain masih rencana. Setiap aturan di bawah menyebut spec pemiliknya di `docs/specs/`. Spec itu sumbernya, bagian ini hanya penunjuk.

## Stack

- **Bahasa dan toolchain**: Rust `1.99.1` (pin di `rust-toolchain.toml`, membawa `rustfmt` dan `clippy`), edisi `2024`, `rust-version = "1.99"`.
- **Workspace**: satu Cargo Workspace dengan manifest virtual di root, `resolver = "3"`, dan satu `Cargo.lock` yang terlacak git.
- **Anggota**: `desktop-core` di `crates/core/`, tanpa dependensi.
- **Package manager**: `cargo`. Setiap perintah bukti memakai `--locked`.

## Build approach

Tracer Bullet. Buktikan satu alur dari input sampai hasil Core di macOS. Ulangi alur itu di host Windows dan Linux ketika tersedia.

## Commands

Jalankan dari root repo. Keempat perintah pertama harus keluar dengan kode 0 (spec `_root/0001`, AC-6 dan bagian "Invarian").

```bash
cargo build --workspace --locked
cargo test --workspace --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings

# Bukti arah dependensi Core: harus mencetak tepat satu baris, desktop-core sendiri
cargo tree -p desktop-core --target all --all-features --prefix none --locked
```

## Specs

Keputusan bersama ada di `docs/specs/_root/NNNN-judul/` (`index.md`, `rationale.md`, `verify.md`). Penerapan per OS masuk ke `docs/specs/macos/`, `docs/specs/windows/`, dan `docs/specs/linux/`. Scope fitur ada di `docs/scope/` dengan indeks di `docs/scope/scope.md`. Kriteria penerimaan bernomor ada di `docs/quality/README.md`.

## Rules

- **Satu lokasi, satu pemilik.** Tabel lokasi, paket, dan pemilik ada di spec `_root/0001` bagian "Lokasi dan pemilik". Folder rencana baru dibuat saat scope pemiliknya dikerjakan.
- **Dependensi mengarah ke Core.** Tabel boleh dan dilarang ada di spec `_root/0001` bagian "Aturan arah dependensi". Sesudah mengubah Core atau menambah anggota, jalankan perintah di tabel "Bukti minimal" bagian itu.
- **Netral host.** Anggota khusus OS menaruh dependensi OS di bawah `[target.'cfg(target_os = "...")'.dependencies]` dan memberi kodenya atribut `cfg` yang sama, supaya perintah `--workspace` tetap lulus di host lain (spec `_root/0001`).
- **Manifest root.** `members` ditulis satu per satu tanpa glob. Spec yang menambah anggota hanya menambah baris `members` miliknya dan entri `[workspace.dependencies]` yang ia butuhkan. Perubahan lain pada `Cargo.toml` root, `rust-toolchain.toml`, dan `.gitignore` lewat pembaruan spec `_root` (spec `_root/0001` bagian "Aturan untuk manifest root").
- **Nama paket.** `desktop-` ditambah peran. Setiap paket mewarisi `version`, `edition`, `rust-version`, dan `publish` dari `[workspace.package]` (spec `_root/0001`).
- **Lockfile.** Sesudah manifest berubah, jalankan `cargo build` tanpa `--locked` sekali dan sertakan `Cargo.lock` di commit yang sama. Tidak ada lockfile lain dan tidak ada `.cargo/config.toml` (spec `_root/0001`).
- **API publik Core.** Ikuti R-1 sampai R-9 di spec `_root/0002` bagian "Aturan kontrak publik bersama". API baru atau perubahan butuh spec `_root` yang diterima lebih dulu.
- **`.graphifyignore`.** Bila sumber baru membuat hook gagal dengan `Graphify extraction failed`, tambahkan entri untuk sumber itu dan catat alasannya di spec pemiliknya. Hook dan `scripts/update_graphify.py` tidak dilonggarkan (spec `_root/0001` bagian "Kasus gagal yang sudah diputuskan").
- **Uji mengunci aturan ini.** `crates/core/tests/workspace_layout.rs` gagal bila aturan manifest di atas dilanggar, atau bila tabel arah dependensi di `docs/architecture/README.md` berbeda dari tabel di spec `_root/0001`. Ubah kedua tabel itu bersama sama.

## Agent skills

- [coordinate](.claude/skills/coordinate/): lokal di repo ini, membagi satu scope ke sub sesi (architect, developer, checker, tester, debugger) dan menjaga gerbang merge.

Alur `/scope`, `/architect`, `/develop`, `/check verify`, `/test`, `/audit`, dan `/sync` yang disebut di `docs/scope/` adalah skill tingkat pengguna. Skill itu tidak tersimpan di repo ini.

## Context files

- [crates/core/AGENTS.md](crates/core/AGENTS.md) (Rust Core: kontrak publik, larangan dependensi, dan aturan uji)
