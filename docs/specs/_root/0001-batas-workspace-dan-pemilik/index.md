# 0001. Batas workspace dan pemilik

**Scope**: [001 - Batas workspace dan pemilik](../../../scope/001%20-%20Batas%20workspace%20dan%20pemilik.md)
**Date**: 2026-11-15
**Status**: Proposed

## Summary

Spec ini menetapkan satu Cargo Workspace (kumpulan paket Rust yang dibangun bersama dan berbagi satu lockfile) di root repo, dengan Rust Core sebagai anggota pertama bernama `desktop-core` di `crates/core/`. Setiap lokasi kode mendapat satu pemilik, dan arah dependensi ditulis sebagai tabel yang bisa dibuktikan dengan perintah `cargo tree`. Hasil build spec ini adalah workspace dengan kerangka Core yang bisa dibangun dan diuji sendiri tanpa UI, sehingga scope 003 dan 004 punya tempat yang pasti untuk mulai.

## Requirements

**User stories**:

- Sebagai pengembang Core, kamu ingin membangun dan menguji Core tanpa menyiapkan proyek Xcode, pustaka GTK, atau Windows App SDK, supaya perilaku bersama bisa dibuktikan di host mana pun.
- Sebagai pengembang aplikasi satu OS, kamu ingin tahu di mana kodemu tinggal dan apa saja yang boleh kamu impor, supaya perubahanmu tidak mematahkan build di host lain.
- Sebagai lead, kamu ingin satu perintah `--workspace` yang hijau di Mac sekarang dan tetap hijau saat aplikasi Windows dan Linux bergabung.

**Acceptance criteria** (kontrak; nomor kriteria merujuk [Penerimaan dan bukti fondasi](../../../quality/README.md); semua perintah dijalankan dari root repo):

- **AC-1** (kriteria 1): Pada host macOS, `cargo build --workspace --locked` keluar dengan kode 0. `cargo tree --workspace --depth 0 --prefix none --locked` mencetak satu baris untuk tiap anggota; pada akhir milestone 1 itu tepat satu baris yang tidak kosong, diawali `desktop-core v0.1.0`.
- **AC-2** (kriteria 2): `cargo build -p desktop-core --locked` dan `cargo test -p desktop-core --locked` keluar dengan kode 0 pada repo yang tidak memuat proyek Xcode, binding Swift, crate penghubung, atau crate aplikasi. Keluaran test memuat `test result: ok` untuk uji unit dan uji dokumen dan tidak memuat `FAILED`. Kerangka dari spec ini tidak membawa test, jadi sebelum scope 003 masuk kedua baris itu melaporkan 0 test.
- **AC-3** (kriteria 2, Done when): `cargo tree -p desktop-core --target all --all-features --prefix none --locked` mencetak tepat satu baris, yaitu `desktop-core` sendiri. Perintah itu mencakup dependensi biasa, build, dan test untuk semua OS. Pada akhir milestone 1 `crates/core/Cargo.toml` memuat tabel `[dependencies]` yang kosong dan tidak memuat `[dev-dependencies]`, `[build-dependencies]`, `[target.*]`, atau `[features]`, dan `crates/core/build.rs` tidak ada.
- **AC-4** (kriteria 1): `rust-toolchain.toml` berisi persis seperti di bagian "Isi file", dan `rustc --version` mencetak baris yang diawali `rustc 1.99.1`. `git ls-files --error-unmatch Cargo.lock` keluar dengan kode 0, dan `grep -c '^\[\[package\]\]' Cargo.lock` mencetak `1`.
- **AC-5** (kriteria 1): Sesudah perintah AC-1 dan AC-2 dijalankan pada working tree yang bersih, `git status --short` tidak mencetak apa pun dan `git check-ignore -q target` keluar dengan kode 0. `git ls-files graphify-out` tetap mencetak empat path.
- **AC-6** (gerbang merge lead): `cargo fmt --all --check` dan `cargo clippy --workspace --all-targets --locked -- -D warnings` keluar dengan kode 0.
- **AC-7** (kriteria 32): Milestone 1 hanya menyentuh file berikut: `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.gitignore`, `crates/core/Cargo.toml`, `crates/core/src/lib.rs`, `verify.md` spec ini, baris `**Status**` di `index.md` spec ini, dan empat file `graphify-out/` dari hook. Daftar itu diperiksa dengan `git diff --name-only main...HEAD` di branch developer sebelum merge, atau sesudah merge dengan `git log --name-only --format= --grep='spec _root/0001 step [12]' | sort -u`. Pada commit langkah 1, isi enam file pertama sama persis dengan bagian "Isi file", jadi kerangka Core tidak memuat item, atribut, atau test. Pada akhir milestone 1 `grep -cE '^\[workspace\.(lints|dependencies)\]|^default-members|^exclude|^\[profile' Cargo.toml` mencetak `0`. Tidak ada crate `xtask` dan tidak ada `.cargo/config.toml`.
- **AC-8** (kriteria 27, Done when): `docs/architecture/README.md` memuat teks dari bagian "Perubahan dokumen arsitektur". `grep -c 'desktop-core\|desktop-macos-ffi\|desktop-windows\|desktop-linux' docs/architecture/README.md` mencetak angka 4 atau lebih, `grep -c '1\.99\.1' docs/architecture/README.md` mencetak 1 atau lebih, heading `### Yang sudah ada` dan `### Aturan arah dependensi` ada, tabel aturannya sama baris demi baris dengan tabel di bagian "Aturan arah dependensi" spec ini, dan tautan ke `docs/specs/_root/0001-batas-workspace-dan-pemilik/index.md` mengarah ke file yang ada. Kalimat "Pada tahap scope, folder implementasi masih berupa rencana." dan "Root workspace nantinya memuat manifest Cargo, lockfile dan konfigurasi toolchain." sudah tidak ada.
- **AC-9** (kriteria 2 pada host lain; `BLOCKED`, `owner: user`): Pada host Windows dengan target `x86_64-pc-windows-msvc` dan host Linux dengan target `x86_64-unknown-linux-gnu`, `cargo build -p desktop-core --locked` dan `cargo test -p desktop-core --locked` keluar dengan kode 0 dengan toolchain `1.99.1`. Prasyaratnya hanya `rustup` dan linker sistem (MSVC Build Tools di Windows, `cc` di Linux), tanpa pustaka UI. Hasil dicatat per host dengan OS, arsitektur, versi, perintah, dan keluaran.

## Decision

**Chosen option**: Opsi 1: satu workspace, anggota eksplisit, aturan netral host.

Repo memakai satu Cargo Workspace dengan manifest virtual (manifest root tanpa paket sendiri) di root, daftar `members` yang ditulis satu per satu, satu `Cargo.lock` yang terlacak git, dan pin toolchain `1.99.1`. Setiap anggota khusus OS wajib tetap bisa dibangun di host lain, sehingga `cargo build --workspace` hijau di ketiga host.

Keputusan turunannya:

| Hal | Keputusan |
| --- | --- |
| Manifest root | Virtual, `resolver = "3"`, `members = ["crates/core"]`. Tanpa glob. |
| Nama paket | Awalan `desktop-` ditambah peran. Core adalah paket `desktop-core`, nama lib `desktop_core`. |
| Nama yang dicadangkan | `desktop-macos-ffi` di `crates/macos-ffi/`, `desktop-windows` di `apps/windows/`, `desktop-linux` di `apps/linux/`. |
| Edisi dan versi | Edisi `2024`, `rust-version = "1.99"`, `version = "0.1.0"`, `publish = false`, semuanya diwarisi dari `[workspace.package]`. |
| Toolchain | `rust-toolchain.toml` dengan `channel = "1.99.1"`, `profile = "minimal"`, komponen `rustfmt` dan `clippy`. |
| Lockfile | `Cargo.lock` terlacak git. Setiap perintah bukti dan gerbang memakai `--locked`. |
| Keluaran build | `/target` dan entri umum Xcode, SwiftPM, dan macOS masuk `.gitignore`. |
| Bukti arah dependensi | Perintah `cargo tree` yang tertulis di spec ini. Tanpa crate atau skrip baru. |
| Folder rencana | Tidak dibuat sebelum ada tanggung jawab nyata. Spec ini hanya mencadangkan path dan nama. |

## Rationale

Alasan, opsi yang ditimbang, dan daftar keputusan beserta pilihan keduanya ada di [rationale.md](rationale.md).

## Rancangan workspace

### Lokasi dan pemilik

Pemilik di sini adalah lapisan arsitektur dari [peta arsitektur](../../../architecture/README.md), bukan orang. Setiap lokasi punya tepat satu pemilik.

| Lokasi | Paket Cargo | Keadaan sesudah spec ini | Pemilik | Scope yang membuat atau mengisinya |
| --- | --- | --- | --- | --- |
| `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.gitignore` di root | bukan paket | dibuat (`.gitignore` ditambah) | Tooling bersama | 001 (spec ini) |
| `crates/core/` | `desktop-core` | kerangka dibuat | Rust Core | 001 kerangka, 003 isi |
| `crates/macos-ffi/` | `desktop-macos-ffi` | rencana | Aplikasi macOS | 004 |
| `bindings/swift/` | bukan paket (keluaran generator, sumbernya `crates/macos-ffi/`) | rencana | Aplikasi macOS | 004 |
| `apps/macos/` | bukan paket (proyek Xcode) | rencana | Aplikasi macOS | 004 |
| `apps/windows/` | `desktop-windows` | rencana | Aplikasi Windows | 006, sesudah 005 |
| `apps/linux/` | `desktop-linux` | rencana | Aplikasi Linux | 007 |
| `design-system/tokens/`, `design-system/schemas/` | bukan paket | rencana | Sistem desain | 008 |
| `shared/config/`, `shared/localization/`, `shared/assets/` | bukan paket | rencana | Resource bersama | 009, 010, 011 |
| `shared/specifications/` | bukan paket | rencana, belum ada scope yang membutuhkannya | Resource bersama | belum ada; spec pekerjaan tinggal di `docs/specs/` |
| `tooling/codegen/`, `tooling/validation/` | ditetapkan scope pembuatnya | rencana | Tooling bersama | scope pertama di antara 002, 004, dan 008 yang membutuhkannya |
| `tests/fixtures/`, `tests/contracts/` | bukan paket | rencana | Tooling bersama | saat konsumen kedua butuh fixture yang sama |
| `scripts/`, `.github/workflows/` | bukan paket | `scripts/update_graphify.py` sudah ada | Tooling bersama | 012 |

Uji API publik Core tinggal di `crates/core/tests/`, supaya `cargo test -p desktop-core` menjalankannya. `tests/` di root bukan folder target Cargo pada manifest virtual, jadi folder itu hanya untuk fixture yang dipakai lebih dari satu aplikasi.

### Isi file

Developer menulis file berikut persis seperti ini.

`Cargo.toml` di root:

```toml
[workspace]
resolver = "3"
members = ["crates/core"]

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.99"
publish = false
```

`rust-toolchain.toml` di root:

```toml
[toolchain]
channel = "1.99.1"
profile = "minimal"
components = ["rustfmt", "clippy"]
```

`crates/core/Cargo.toml`:

```toml
[package]
name = "desktop-core"
description = "Perilaku bersama fondasi desktop, tanpa pustaka UI dan tanpa SDK khusus OS."
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true

[dependencies]
```

`crates/core/src/lib.rs`:

```rust
//! Rust Core fondasi desktop.
//!
//! Crate ini memiliki perilaku bersama untuk ketiga aplikasi native. Ia dibangun
//! dan diuji tanpa pustaka UI dan tanpa SDK khusus OS.
```

`.gitignore` sesudah perubahan (lima baris pertama sudah ada dan tidak diubah, lima baris terakhir ditambahkan tanpa baris kosong di antaranya):

```gitignore
/graphify-out/*
!/graphify-out/graph.json
!/graphify-out/graph.html
!/graphify-out/GRAPH_REPORT.md
!/graphify-out/manifest.json
/target
DerivedData/
xcuserdata/
.build/
.DS_Store
```

`Cargo.lock`, dibuat dengan `cargo generate-lockfile` di root dan tidak disunting dengan tangan. Tanpa dependensi, perintah itu tidak mengunduh apa pun dan hasilnya:

```toml
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 4

[[package]]
name = "desktop-core"
version = "0.1.0"
```

### Aturan untuk manifest root

- Daftar `members` ditulis satu per satu. Cargo otomatis menjadikan setiap dependensi `path` di dalam root sebagai anggota, jadi setiap folder crate di repo wajib tercantum di `members` dan wajib mematuhi aturan spec ini.
- Spec yang menambah anggota boleh mengubah manifest root sebatas menambah baris `members` miliknya dan entri `[workspace.dependencies]` yang ia butuhkan. Perubahan lain pada `Cargo.toml` root, `rust-toolchain.toml`, dan `.gitignore` lewat pembaruan spec `_root`.
- Konsumen pertama Core (scope 004) menambah `[workspace.dependencies]` dengan baris `desktop-core = { path = "crates/core" }`. Setiap konsumen lalu menulis `desktop-core.workspace = true` di manifestnya. Dependensi luar yang dipakai dua anggota atau lebih juga ditulis di tabel itu; yang dipakai satu anggota tinggal di manifest anggota itu.
- Setiap paket baru mewarisi `version`, `edition`, `rust-version`, dan `publish` dari `[workspace.package]`, dan namanya `desktop-` ditambah peran sesuai tabel "Lokasi dan pemilik".
- Sesudah manifest berubah, jalankan `cargo build` tanpa `--locked` sekali dan sertakan `Cargo.lock` di commit yang sama. `--locked` yang gagal berarti lockfile belum ikut diperbarui.
- Kenaikan patch pin di minor yang sama (`1.99.x`) cukup satu commit yang lulus AC-1 sampai AC-6. Kenaikan minor mengubah `channel` dan `rust-version` bersama sama lewat pembaruan spec ini, sesudah kombinasi compiler, Xcode, dan alat binding diperiksa di host yang tersedia.

### Nilai dan sumbernya

| Nilai | Sumber |
| --- | --- |
| Nama paket `desktop-core`, nama lib `desktop_core` | Keputusan spec ini. Cargo menurunkan nama lib dari nama paket. |
| `version`, `edition`, `rust-version`, `publish` tiap paket | `[workspace.package]` di manifest root. |
| Pin `1.99.1` | Toolchain stable yang aktif dan lengkap (`rustfmt`, `clippy`) di Mac koordinasi pada 2026-11-15. |
| Daftar anggota workspace | Baris `members` di manifest root. |
| Isi `Cargo.lock` | Dihasilkan `cargo generate-lockfile` dari manifest. |
| Keluaran yang diharapkan tiap perintah bukti | Dicoba pada workspace percobaan dengan `rustc 1.99.1` pada 2026-11-15, tercatat di [rationale.md](rationale.md). |

### Invarian

- Core hanya membutuhkan toolchain Rust yang dipin dan linker sistem host (Command Line Tools di macOS, MSVC Build Tools di Windows, `cc` di Linux). Core tidak pernah membutuhkan SDK UI.
- `cargo build --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, dan `cargo test --workspace --locked` keluar dengan kode 0 di setiap host yang didukung, berapa pun anggota yang sudah bergabung.
- Satu `Cargo.lock` untuk seluruh workspace. Tidak ada lockfile lain di repo.
- `target/` tidak pernah terlacak git. Repo tidak mengatur `CARGO_TARGET_DIR`, jadi tiap worktree memakai `target/` sendiri.

### Keamanan

Spec ini tidak menambah rahasia, kredensial, atau variabel lingkungan. `publish = false` pada setiap paket mencegah paket terkirim ke registry publik tanpa sengaja. Kredensial penandatanganan tetap di luar repo (scope 004 dan 012).

### Kasus gagal yang sudah diputuskan

| Kejadian | Perilaku yang diharapkan |
| --- | --- |
| Toolchain `1.99.1` belum terpasang di sebuah mesin | `rustup` memasangnya saat perintah `cargo` pertama. Di Mac koordinasi versi itu sudah terpasang, jadi tidak ada unduhan. |
| Mesin tanpa `rustup` memakai compiler yang lebih lama | Cargo menolak build dengan pesan `rust-version` yang jelas. Compiler yang lebih baru bisa membangun, tetapi bukan kombinasi yang dibuktikan. |
| `--locked` gagal sesudah manifest berubah | Perbarui lockfile dengan `cargo build`, lalu sertakan di commit yang sama dengan manifest. |
| Konflik `Cargo.lock` saat merge | Ambil satu sisi, jalankan `cargo build`, lalu commit hasilnya. |
| Anggota khusus OS gagal dibangun di host lain | Pelanggaran aturan netral host. Anggota itu diperbaiki; perintah `--workspace` tidak diganti dengan daftar `-p`. |

### Skenario uji kritis

- Jalur utama: clone bersih, `cargo build --workspace --locked` lalu `cargo test -p desktop-core --locked` hijau, memverifikasi **AC-1**, **AC-2**.
- Arah dependensi: `cargo tree -p desktop-core --target all --all-features --prefix none --locked` mencetak satu baris, memverifikasi **AC-3**.
- Toolchain dan lockfile: `rustc --version` dan `grep -c '^\[\[package\]\]' Cargo.lock`, memverifikasi **AC-4**.
- Kebersihan repo: `git status --short` kosong sesudah build dan test, memverifikasi **AC-5**.
- Gerbang: `cargo fmt --all --check` dan `cargo clippy --workspace --all-targets --locked -- -D warnings`, memverifikasi **AC-6**.
- Cakupan: daftar file yang disentuh milestone 1 sama dengan daftar di AC-7, memverifikasi **AC-7**.

## Aturan arah dependensi

Dependensi mengarah ke Core dan tidak pernah membentuk siklus. Aturan berlaku untuk dependensi biasa, dependensi build, dan dependensi test.

| Dari | Boleh bergantung pada | Dilarang |
| --- | --- | --- |
| `desktop-core` | Tidak ada anggota workspace. | Anggota lain mana pun; pustaka UI; SDK khusus OS; alat binding seperti `uniffi`. Berlaku langsung maupun transitif. |
| `desktop-macos-ffi` | `desktop-core`, alat binding. | Crate aplikasi; pustaka UI. |
| `desktop-windows` | `desktop-core`; pustaka UI dan SDK Windows di bawah tabel `target` untuk Windows. | `desktop-macos-ffi`, `desktop-linux`. |
| `desktop-linux` | `desktop-core`; GTK dan libadwaita di bawah tabel `target` untuk Linux. | `desktop-macos-ffi`, `desktop-windows`. |
| `apps/macos/` (Xcode) | `bindings/swift/` dan pustaka hasil build `desktop-macos-ffi`. | Memanggil Core lewat jalur lain. |

Tidak ada anggota yang bergantung pada crate aplikasi.

**Aturan netral host.** Anggota yang butuh pustaka satu OS wajib tetap terkompilasi di host lain, sehingga ketiga perintah `--workspace` di bagian "Invarian" keluar dengan kode 0 di sana. Caranya:

- Dependensi OS ditulis di bawah `[target.'cfg(target_os = "...")'.dependencies]`, dan dependensi build OS di bawah `[target.'cfg(target_os = "...")'.build-dependencies]`.
- Kode yang memakai dependensi itu membawa atribut `#[cfg(target_os = "...")]` yang sama.
- Skrip `build.rs` anggota itu tidak melakukan pekerjaan OS saat `CARGO_CFG_TARGET_OS` bukan OS miliknya, karena skrip build berjalan di setiap host.

Dengan begitu pustaka GTK atau Windows tidak pernah masuk graf build di Mac, tetapi tetap tercatat di `Cargo.lock` yang sama. Apa yang dilakukan titik masuk aplikasi di host lain diputuskan spec aplikasi itu (scope 006 dan 007).

**Bukti minimal.** Sampai scope 002 memasang pemeriksaan otomatis, bukti arah dependensi adalah perintah berikut.

| Kapan | Perintah | Hasil yang diharapkan |
| --- | --- | --- |
| Sekarang dan sesudah setiap perubahan Core | `cargo tree -p desktop-core --target all --all-features --prefix none --locked` | Tepat satu baris, `desktop-core` sendiri. |
| Saat anggota baru bergabung | `cargo tree -p <paket> --target all --all-features --prefix none --locked` | Tidak memuat paket dari kolom "Dilarang" baris anggota itu. |
| Saat anggota khusus OS bergabung, di host lain | `cargo build --workspace --locked`, lalu `cargo tree -p <paket> --prefix none --locked` | Kode 0, dan pohon tanpa `--target all` tidak memuat pustaka UI atau SDK OS mana pun. |

Spec yang menambah anggota mengutip barisnya dari tabel aturan dan menjalankan bukti di atas untuk anggotanya. Bukti "tepat satu baris" berlaku selama Core tidak punya dependensi; spec yang memberi Core dependensi pertamanya mengganti bukti itu dengan daftar larangan dari scope 002.

## Perubahan dokumen arsitektur

Langkah 3 Build plan mengubah `docs/architecture/README.md` sebagai berikut. Bagian lain dokumen itu, termasuk tabel "Pemilik dan arah dependensi", tidak diubah.

1. Di bagian "Tata letak yang dituju", ganti kalimat "Pada tahap scope, folder implementasi masih berupa rencana." dengan:

   ```markdown
   Root workspace dan `crates/core/` sudah ada. Lokasi lain masih berupa rencana sampai scope pemiliknya dikerjakan.
   ```

2. Di bagian yang sama, ganti kalimat "Root workspace nantinya memuat manifest Cargo, lockfile dan konfigurasi toolchain." dengan:

   ```markdown
   Root workspace memuat manifest Cargo virtual, `Cargo.lock` yang terlacak git, dan `rust-toolchain.toml` yang mengunci Rust `1.99.1` dengan edisi `2024`.
   ```

3. Di akhir bagian "Tata letak yang dituju", sesudah paragraf yang diawali "Target bersama adalah Rust stable." dan sebelum heading "Aturan pengembangan yang akan dicatat", tambahkan dua subbagian ini berurutan:

   ```markdown
   ### Yang sudah ada

   | Lokasi | Isi |
   | --- | --- |
   | `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` | Manifest workspace virtual dengan daftar anggota eksplisit, lockfile bersama, dan pin toolchain. |
   | `crates/core/` | Paket `desktop-core`, kerangka Rust Core tanpa dependensi. |

   Nama paket yang dicadangkan untuk lokasi yang masih rencana: `desktop-macos-ffi` di `crates/macos-ffi/`, `desktop-windows` di `apps/windows/`, dan `desktop-linux` di `apps/linux/`.

   ### Aturan arah dependensi

   Dependensi mengarah ke Core dan tidak pernah membentuk siklus. Aturan berlaku untuk dependensi biasa, dependensi build, dan dependensi test.

   TABEL

   Tidak ada anggota yang bergantung pada crate aplikasi. Anggota yang butuh pustaka satu OS menaruh dependensi itu di bawah tabel `target` untuk OS tersebut dan memberi kode yang memakainya atribut `cfg` yang sama, sehingga `cargo build --workspace --locked` berhasil di ketiga host. Bukti untuk Core adalah `cargo tree -p desktop-core --target all --all-features --prefix none --locked`, yang mencetak tepat satu baris. Rincian dan alasannya ada di spec 0001.
   ```

   Ganti kata `TABEL` dengan salinan persis tabel lima baris dari bagian "Aturan arah dependensi" spec ini. Jadikan kata `spec 0001` di kalimat terakhir tautan ke `docs/specs/_root/0001-batas-workspace-dan-pemilik/index.md`; dari dokumen arsitektur path relatifnya diawali `../specs/_root/`.

## Build plan

Pendekatan build proyek adalah Tracer Bullet. Untuk fitur ini benang tipisnya adalah workspace yang bisa membangun dan menguji Core, lalu dokumen arsitektur mengikuti keadaan nyata. Satu langkah menjadi satu commit.

**Milestone 1: Workspace dan kerangka Core** (langkah 1 dan 2)

1. Buat `Cargo.toml`, `rust-toolchain.toml`, `crates/core/Cargo.toml`, dan `crates/core/src/lib.rs`, ubah `.gitignore`, lalu jalankan `cargo generate-lockfile`, semuanya persis seperti bagian "Isi file" dan dalam satu commit. Sebelum commit, perintah AC-1 sampai AC-6 harus lulus, satisfies **AC-1**, **AC-2**, **AC-3**, **AC-4**, **AC-5**, **AC-7**
2. Dengan `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, jalankan ulang setiap perintah AC-1 sampai AC-7 pada commit langkah 1, catat perintah dan keluarannya di `verify.md` bagian `## Milestone 1: Workspace dan kerangka Core`, hapus `target/`, lalu commit `verify.md`, satisfies **AC-1**, **AC-2**, **AC-3**, **AC-4**, **AC-5**, **AC-6**, **AC-7**

**Milestone 2: Peta arsitektur mencatat tata letak nyata** (langkah 3, sesudah milestone 1)

3. Perbarui `docs/architecture/README.md` sesuai bagian "Perubahan dokumen arsitektur" dan catat pemeriksaan AC-8 di `verify.md` bagian `## Milestone 2: Peta arsitektur mencatat tata letak nyata`, satisfies **AC-8**

**Milestone 3: Bukti Core di host Windows dan Linux** (langkah 4, `BLOCKED`, `owner: user`)

4. Pada host Windows `x86_64-pc-windows-msvc` dan host Linux `x86_64-unknown-linux-gnu`, jalankan `cargo build -p desktop-core --locked` dan `cargo test -p desktop-core --locked`, lalu catat OS, arsitektur, versi, perintah, dan keluaran di `verify.md` bagian `## Milestone 3: Bukti Core di host Windows dan Linux`. Sesi koordinasi di Mac tidak menjalankan langkah ini, satisfies **AC-9**

## Consequences

**Positive**:

- Satu perintah `--workspace` berlaku di semua host, jadi gerbang lead dan CI kelak tidak butuh daftar paket per host.
- Core terbukti berdiri sendiri sejak commit pertama, dan buktinya satu baris keluaran yang mudah dibaca.
- Satu lockfile membuat ketiga aplikasi memakai versi dependensi yang sama.
- Nama dan path sudah pasti sebelum scope 003 dan 004 menulis kode.

**Negative / tradeoffs**:

- Aplikasi Windows dan Linux wajib membawa atribut `cfg` dan titik masuk yang terkompilasi di host lain. Itu sedikit kode tambahan di tiap aplikasi, dan build di host lain hanya membuktikan bahwa kerangkanya terkompilasi, bukan bahwa aplikasinya benar.
- `Cargo.lock` memuat dependensi semua OS, jadi perubahan dependensi satu aplikasi menyentuh file yang dipakai semua tim.
- Pin persis `1.99.1` harus dinaikkan dengan sengaja. Versi itu bukan stable terbaru (1.100.0 terbit 2026-11-09), dan Rust hanya memperbaiki stable terbaru.
- Bukti arah dependensi masih dijalankan orang sampai scope 002 mengotomatiskannya, dan bukti "tepat satu baris" berhenti berlaku begitu Core mendapat dependensi pertamanya.
- Core tidak boleh memakai `uniffi`, jadi `desktop-macos-ffi` mendeklarasikan ulang API yang diekspos ke Swift.

**Neutral**:

- Mengganti nama paket sesudah scope 004 berarti mengubah manifest, `use`, dan nama modul binding Swift. Sebelum itu biayanya kecil.
- `docs/architecture/README.md` biasanya terlarang bagi sub sesi. Milestone 2 butuh izin eksplisit di brief developer.
- `/develop` menulis `verify.md` saat build. Spec ini tidak membuatnya.

## Follow-up

- [ ] Scope 002: pemeriksaan arsitektur otomatis yang lengkap (daftar larangan untuk Core termasuk dependensi transitif, sisi yang diizinkan per anggota, keluaran generator yang menyimpang), kebijakan `[workspace.lints]`, konfigurasi `rustfmt` dan `clippy` bila diperlukan, dan isi `AGENTS.md` (aturan manifest root, penamaan, netral host). Scope 002 juga memutuskan apakah spec di `docs/specs/` menggantikan ADR di `docs/decisions/` yang disebut dokumen arsitektur.
- [ ] Scope 003 (spec `_root/0002`): isi Core. `hello`, modul, lint khusus Core lewat `[lints.rust]` di `crates/core/Cargo.toml`, dan uji kontrak di `crates/core/tests/`.
- [ ] Scope 004: `crates/macos-ffi/` bergabung sebagai `desktop-macos-ffi` dan menambah `[workspace.dependencies]`; versi UniFFI yang cocok dengan Rust `1.99.1`; jenis crate; apakah `bindings/swift/` dilacak git; proyek Xcode di `apps/macos/`; target Intel `x86_64-apple-darwin` dan apakah masuk `targets` di `rust-toolchain.toml`; entri `.gitignore` di luar lima baris umum.
- [ ] Scope 005 dan 006 (menunggu host Windows): `apps/windows/` bergabung sebagai `desktop-windows` di bawah aturan netral host, termasuk perilaku titik masuknya di host lain. Bila `windows-reactor` butuh toolchain lain atau tidak bisa mematuhi aturan itu, perbarui spec ini lebih dulu.
- [ ] Scope 007 (menunggu host Linux): `apps/linux/` bergabung sebagai `desktop-linux` di bawah aturan netral host, termasuk perilaku titik masuknya di host lain.
- [ ] Scope 012: skrip `scripts/` dan CI membungkus perintah yang sama dengan `--locked` dan membaca `rust-toolchain.toml`; pengaturan `[profile.release]` bila dibutuhkan.
- [ ] AC-9 dan milestone 3 menunggu host Windows dan Linux (`owner: user`).
- [ ] `README.md` di root masih berkata repo hanya berisi dokumentasi perencanaan. Kalimat itu usang sesudah milestone 1; `/sync` atau lead memperbaruinya.
- [ ] Field `license` sengaja tidak diisi karena lisensinya hanya user yang tahu. `publish = false` menjaga paket tetap privat sampai nilai itu ada.
