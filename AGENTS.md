# Aturan commit dan integrasi

Aktifkan hook sekali di setiap clone dengan `git config --local core.hooksPath .githooks`. Hook memerlukan Python 3 dan CLI `graphify` di PATH. Untuk memasang Graphify, gunakan `uv tool install graphifyy`.

1. Sebelum commit, perbarui dokumentasi yang terdampak, stage sumber yang ingin disimpan, lalu periksa `git diff --cached`.
2. Saat `git commit`, hook memperbarui Graphify dari salinan indeks Git dan menyertakan perubahan JSON, HTML, laporan, dan manifest dalam commit yang sama. Perubahan unstaged serta riwayat kueri lokal tetap di mesin. Jika pembaruan gagal, perbaiki penyebab yang dilaporkan dan ulangi commit.
3. Pembaruan lokal membaca struktur kode serta judul dan tautan Markdown. Untuk memperbarui konsep dan hubungan semantik dalam isi dokumen, jalankan `/graphify . --update` dari salinan sumber yang akan di-commit dan stage keempat hasilnya.
4. Saat mengirim perubahan ke `main`, ambil `origin/main` terbaru, gabungkan branch kerja, lalu push ke `origin/main`. Jika Git perlu membuat commit penggabungan, gunakan `git merge --no-commit <branch>` lalu `git commit` agar hook menyertakan graf hasil penggabungan. Pastikan SHA pada remote sama dengan SHA lokal yang dikirim.

# Aturan pengembangan

Tiga aplikasi desktop native memakai satu Rust Core. Yang sudah ada: Cargo Workspace, `crates/core/`, jembatan macOS di `crates/macos-ffi/`, binding Swift di `bindings/swift/`, dan aplikasi macOS di `apps/macos/`. Sumber bersama di `design-system/` dan `shared/`, alat `tooling/codegen/`, dan skrip di `scripts/` juga sudah ada. Lokasi Windows dan Linux masih rencana. Setiap aturan di bawah menyebut spec pemiliknya di `docs/specs/`. Spec itu sumbernya, bagian ini hanya penunjuk.

## Stack

- **Bahasa dan toolchain**: Rust `1.99.1` (pin di `rust-toolchain.toml`, membawa `rustfmt` dan `clippy`), edisi `2024`, `rust-version = "1.99"`.
- **Workspace**: satu Cargo Workspace dengan manifest virtual di root, `resolver = "3"`, dan satu `Cargo.lock` yang terlacak git.
- **Anggota**: `desktop-core` di `crates/core/`, tanpa dependensi, dan `desktop-macos-ffi` di `crates/macos-ffi/`, jembatan UniFFI `=0.32.2` ke Swift.
- **Alat**: `desktop-codegen` di `tooling/codegen/`, generator dan pemeriksaan untuk sumber `tokens`, `config`, `localization`, `assets`, dan `architecture`. Dependensinya hanya `serde` dan `serde_json` (spec `_root/0003`).
- **macOS**: paket Swift lokal di `apps/macos/` (modul `Interop`, `Presentation`, dan `DesignSystem`) dan proyek Xcode `DesktopMac`, target `aarch64-apple-darwin`. Versi yang dipakai ada di spec `macos/0001` bagian "Kombinasi yang dipakai".
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

# Pemeriksaan arsitektur saja, satu baris per pelanggaran. cargo test --workspace sudah menjalankannya
cargo run -p desktop-codegen --locked -- architecture check

# Semua gerbang bersama dalam satu perintah, termasuk check tiap sumber (spec _root/0008)
bash scripts/validate.sh

# Bukti arah dependensi Core: harus mencetak tepat satu baris, desktop-core sendiri
cargo tree -p desktop-core --target all --all-features --prefix none --locked

# macOS, dari spec macos/0001 bagian "Perintah build dan uji". swift test butuh jembatan yang sudah dibangun
cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin
swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"
xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build
```

## Specs

Keputusan bersama ada di `docs/specs/_root/NNNN-judul/` (`index.md`, `rationale.md`, `verify.md`). Penerapan macOS ada di `docs/specs/macos/`. Penerapan Windows dan Linux masuk ke `docs/specs/windows/` dan `docs/specs/linux/` saat hostnya tersedia. Scope fitur ada di `docs/scope/` dengan indeks di `docs/scope/scope.md`. Kriteria penerimaan bernomor ada di `docs/quality/README.md`. Catatan `/check verify` untuk spec tanpa `verify.md` sendiri ada di `docs/reviews/`.

## Rules

- **Satu lokasi, satu pemilik.** Tabel lokasi, paket, dan pemilik ada di spec `_root/0001` bagian "Lokasi dan pemilik". Folder rencana baru dibuat saat scope pemiliknya dikerjakan.
- **Dependensi mengarah ke Core.** Tabel boleh dan dilarang ada di spec `_root/0001` bagian "Aturan arah dependensi". Sesudah mengubah Core atau menambah anggota, jalankan perintah di tabel "Bukti minimal" bagian itu.
- **Netral host.** Anggota khusus OS menaruh dependensi OS di bawah `[target.'cfg(target_os = "...")'.dependencies]` dan memberi kodenya atribut `cfg` yang sama, supaya perintah `--workspace` tetap lulus di host lain (spec `_root/0001`).
- **Manifest root.** `members` ditulis satu per satu tanpa glob. Spec yang menambah anggota hanya menambah baris `members` miliknya dan entri `[workspace.dependencies]` yang ia butuhkan. Perubahan lain pada `Cargo.toml` root, `rust-toolchain.toml`, dan `.gitignore` lewat pembaruan spec `_root` (spec `_root/0001` bagian "Aturan untuk manifest root").
- **Nama paket.** `desktop-` ditambah peran. Setiap paket mewarisi `version`, `edition`, `rust-version`, dan `publish` dari `[workspace.package]` (spec `_root/0001`).
- **Lockfile.** Sesudah manifest berubah, jalankan `cargo build` tanpa `--locked` sekali dan sertakan `Cargo.lock` di commit yang sama. Tidak ada lockfile lain dan tidak ada `.cargo/config.toml` (spec `_root/0001`).
- **API publik Core.** Ikuti R-1 sampai R-9 di spec `_root/0002` bagian "Aturan kontrak publik bersama". API baru atau perubahan butuh spec `_root` yang diterima lebih dulu.
- **Binding Swift.** Isi `bindings/swift/Sources/` dibuat generator. Bila API jembatan, rustdoc fungsi yang diekspor, atau versi UniFFI berubah, jalankan perintah di spec `macos/0001` bagian "Membuat ulang binding" dan sertakan hasilnya di commit yang sama.
- **Satu jalur ke Core di macOS.** Hanya `apps/macos/Interop/` yang mengimpor `DesktopMacosFFI`. Jembatan hanya meneruskan panggilan ke Core, dan teks sapaan di Swift hanya ada di uji (spec `macos/0001`, AC-7 dan AC-8).
- **`.graphifyignore`.** Bila sumber baru membuat hook gagal dengan `Graphify extraction failed`, tambahkan entri untuk sumber itu dan catat alasannya di spec pemiliknya. Hook dan `scripts/update_graphify.py` tidak dilonggarkan (spec `_root/0001` bagian "Kasus gagal yang sudah diputuskan").
- **Uji mengunci aturan ini.** `tooling/codegen/tests/workspace_layout.rs` gagal bila aturan manifest di atas dilanggar, atau bila tabel arah dependensi di `docs/architecture/README.md` berbeda dari tabel di spec `_root/0001`. Ubah kedua tabel itu bersama sama. `tooling/codegen/tests/macos_bridge_ownership.rs` mengunci kepemilikan di batas FFI (spec `macos/0001`, AC-6).
- **Pemeriksaan arsitektur.** Aturan `A-1` sampai `A-13` di spec `_root/0007` diperiksa sumber `architecture` di `desktop-codegen`. Pelanggaran menggagalkan `cargo test --workspace --locked`. Aturan baru masuk lewat pembaruan spec itu, bersama kasus ujinya.
- **Anggota dan dependensi baru.** Sisi antar anggota dan dependensi langsung tiap anggota tercatat di `tooling/codegen/src/architecture/mod.rs`. Menambah satu butuh spec pemilik anggota itu lebih dulu, lalu barisnya diubah di commit yang sama dengan manifest (spec `_root/0007`, bagian "Cara menambah atau mengubah aturan").
- **Keluaran generator.** File turunan tidak disunting tangan. Sunting sumbernya, jalankan `cargo run -p desktop-codegen --locked -- <sumber> generate`, lalu commit sumber bersama keluarannya (spec `_root/0003`, O-1 sampai O-7).
- **Lint dan format.** Setiap anggota membawa `[lints.rust]` dengan `unsafe_code = "forbid"` dan `missing_docs = "warn"`. Tidak ada `[workspace.lints]`, `rustfmt.toml`, atau `clippy.toml`. Gaya dan lint mengikuti bawaan toolchain yang dipin (spec `_root/0007`, `A-5`).
- **Bahasa.** Komentar kode, rustdoc, nama uji, deskripsi paket, dan pesan alat berbahasa Inggris. Dokumen di `docs/`, `AGENTS.md`, dan `README.md` berbahasa Indonesia (spec `_root/0007`, `H-6`).
- **Keputusan.** Keputusan arsitektur dicatat sebagai spec di `docs/specs/`, dengan alasannya di `rationale.md` spec itu. Repo ini tidak memakai folder ADR.

## Tinjauan manusia

Aturan berikut butuh penilaian orang. Penulis perubahan memeriksanya sebelum meminta merge, dan pemeriksa membacanya lagi. Rinciannya ada di spec `_root/0007` bagian "Aturan yang diperiksa manusia".

- **H-1** Perilaku bersama tinggal di Core. Aplikasi dan jembatan tidak menyalin logika Core dalam bentuk apa pun.
- **H-2** API publik Core, anggota baru, dan dependensi langsung baru punya spec yang diterima sebelum kodenya ditulis.
- **H-3** Versi `desktop-core` dan tabel "Riwayat kontrak" ikut berubah saat kontrak berubah.
- **H-4** Vektor kontrak di spec sama dengan yang diuji di Rust dan di Swift.
- **H-5** Tabel arah dependensi di spec `_root/0001` dan di peta arsitektur bermakna sama dengan tabel di alat.
- **H-6** Kode berbahasa Inggris, dokumen berbahasa Indonesia.
- **H-7** Setiap entri `.graphifyignore` punya alasan di spec pemilik file itu.
- **H-8** Tidak ada kode, dependensi, folder, atau skrip untuk fitur di luar fase fondasi.
- **H-9** Setiap `#[allow]` dan `#[expect]` membawa `reason` yang menyebut spec.
- **H-10** Interaksi native yang tidak tercakup uji otomatis ditulis sebagai langkah manual di spec OS.

## Agent skills

Alur `/scope`, `/architect`, `/develop`, `/check verify`, `/test`, `/audit`, dan `/sync` yang disebut di `docs/scope/` adalah skill tingkat pengguna. Skill itu tidak tersimpan di repo ini.

## Context files

- [crates/core/AGENTS.md](crates/core/AGENTS.md) (Rust Core: kontrak publik, larangan dependensi, dan aturan uji)
- [tooling/codegen/AGENTS.md](tooling/codegen/AGENTS.md) (generator sumber bersama, emitter macOS, dan pemeriksaan arsitektur)
