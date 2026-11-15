# 0002. Batas workspace dan pemilik di macOS

**Scope**: [001 - Batas workspace dan pemilik](../../scope/001%20-%20Batas%20workspace%20dan%20pemilik.md)
**Date**: 2026-11-15
**Status**: In Progress

## Summary

Spec ini menjelaskan bagaimana keputusan workspace dari [spec `_root/0001`](../_root/0001-batas-workspace-dan-pemilik/index.md) berlaku di Mac, dan bukti apa yang menunjukkannya. Spec ini tidak memutuskan ulang apa pun. Hampir semua bukti sudah ada atau akan dihasilkan oleh spec `_root/0001` dan [spec macOS 0001](0001-sapaan-di-jendela-macos/index.md), jadi di sini cukup dirujuk. Satu satunya pekerjaan baru adalah menambah tiga lokasi macOS ke tabel "Yang sudah ada" di peta arsitektur, sesudah lokasi itu benar benar ada.

## Context

[Panduan scope](../../scope/000%20-%20Panduan%20scope.md) meminta setiap fitur bersama punya spec per OS yang merujuk keputusan bersama dan menjelaskan penerapan serta bukti lokalnya. Untuk scope 001, keputusan bersamanya sudah lengkap: satu Cargo Workspace, pemilik tiap lokasi, dan arah dependensi. Bagian macOS dari keputusan itu diisi oleh scope 004, yang menambah jembatan `crates/macos-ffi/`, binding di `bindings/swift/`, dan aplikasi di `apps/macos/`.

Yang belum tertulis di satu tempat adalah jawaban atas pertanyaan pemeriksa: untuk kriteria 1, 2, 27, dan 32, mana bukti yang diambil di Mac, pada host dan versi apa, dan bagian mana yang sengaja tidak berlaku di Mac. Tanpa itu, kotak scope 001 untuk macOS tidak punya kontrak yang bisa diperiksa. Selain itu, peta arsitektur masih menyebut tiga lokasi macOS sebagai rencana, dan itu tidak lagi benar sesudah scope 004 dibangun.

Hanya ada satu host: Mac Apple Silicon (`arm64`) dengan macOS 27.0.1 dan Rust 1.99.1. Bukti yang butuh host Windows atau Linux bukan urusan spec ini.

## Requirements

**User stories**:

* Sebagai pemeriksa, saya ingin satu daftar yang menunjuk bukti scope 001 di Mac, supaya saya tidak perlu menebak catatan mana yang berlaku untuk target ini.
* Sebagai developer aplikasi macOS, saya ingin tahu aturan workspace mana yang mengikat kode saya dan lewat file apa, tanpa membaca ulang seluruh spec bersama.

**Acceptance criteria**:

Angka kriteria merujuk tabel di [Penerimaan dan bukti fondasi](../../quality/README.md). Setiap AC berlaku untuk host macOS pada `arm64`. "Spec bersama" berarti spec `_root/0001`, dan "spec 004" berarti spec macOS 0001. Semua perintah dijalankan dari akar repo.

Kotak `[x]` di `verify.md` dicentang oleh `/check verify` spec pemilik berkas itu. Spec ini tidak punya `verify.md` sendiri. Hasil perintah yang dijalankan langsung untuk spec ini dicatat `/check verify` di `docs/reviews/`, bersama host, arsitektur, versi, perintah, dan hasilnya.

* **AC-1** (kriteria 1): workspace dibangun di Mac, sebelum dan sesudah anggota macOS bergabung. (a) Di `docs/specs/_root/0001-batas-workspace-dan-pemilik/verify.md` bagian Milestone 1, langkah untuk AC-1 dan AC-6 spec bersama bertanda `[x]`. (b) Sesudah `desktop-macos-ffi` bergabung, lima perintah pertama **AC-9** spec 004 (`cargo build`, `cargo test`, `cargo fmt`, dan dua `cargo clippy`) tercatat `[x]` di `verify.md` spec 004. (c) Saat spec ini diverifikasi, `cargo build --workspace --locked` dijalankan ulang di Mac dan keluar dengan kode 0.
* **AC-2** (kriteria 2): Core dibangun dan diuji sendiri di Mac. (a) Di `verify.md` spec bersama bagian Milestone 1, langkah untuk AC-2 dan AC-3 bertanda `[x]`. Catatan itu diambil pada commit `09c6f0c`, saat repo belum memuat proyek Xcode, binding Swift, atau crate penghubung. (b) Sesudah anggota macOS bergabung, `cargo tree -p desktop-core --target all --all-features --prefix none --locked` tetap mencetak tepat satu baris. Itu perintah terakhir **AC-9** spec 004. (c) Saat spec ini diverifikasi, `cargo test -p desktop-core --locked` dijalankan ulang di Mac dan keluar dengan kode 0.
* **AC-3** (Done when "arah dependensi tiap aplikasi sesuai peta arsitektur", kriteria 1): dua baris macOS di tabel "Aturan arah dependensi" spec bersama terbukti. Baris `desktop-macos-ffi` dibuktikan oleh dua perintah `cargo tree -p desktop-macos-ffi` di **AC-9** spec 004 (tiga baris, dan `0` untuk pola larangan). Baris `apps/macos/` dibuktikan oleh **AC-8** spec 004 (hanya `apps/macos/Interop/Greeter.swift` yang mengimpor `DesktopMacosFFI`). Keduanya tercatat `[x]` di `verify.md` spec 004.
* **AC-4** (kriteria 27): peta arsitektur mencatat lokasi macOS yang sudah ada. `test -d crates/macos-ffi && test -d bindings/swift && test -d apps/macos/DesktopMac.xcodeproj` keluar dengan kode 0. ``awk '/^### Yang sudah ada$/{f=1;next} /^### /{f=0} f && /^\| `/' docs/architecture/README.md`` mencetak tepat lima baris: baris manifest yang lama, baris `crates/core/` yang baru, lalu baris `crates/macos-ffi/`, `bindings/swift/`, dan `apps/macos/`. Empat baris terakhir itu, dibaca berdampingan, sama dengan empat baris di bagian **Perubahan dokumen arsitektur**. `grep -c 'kerangka Rust Core' docs/architecture/README.md` mencetak `0`. ``grep -c 'masih rencana: `desktop-windows`' docs/architecture/README.md`` mencetak `1`, ``grep -c 'masih rencana: `desktop-macos-ffi`' docs/architecture/README.md`` mencetak `0`, dan ``grep -c 'Root workspace dan `crates/core/` sudah ada' docs/architecture/README.md`` mencetak `0`. `cargo test -p desktop-codegen --locked --test workspace_layout` keluar dengan kode 0, jadi tabel aturan yang dikunci spec bersama tidak ikut berubah. Uji itu pindah dari `crates/core/tests/` ke `tooling/codegen/tests/` lewat spec `_root/0007`; sebelum pemindahan perintahnya `-p desktop-core`.
* **AC-5** (kriteria 32): bagian macOS hanya memuat yang dibutuhkan fondasi. **AC-12** spec 004 tercatat `[x]` di `verify.md` spec 004. Uji `repo_has_no_cargo_config` di `workspace_layout` lulus sesudah scope 004 milestone 2, jadi anggota macOS tidak membawa konfigurasi Cargo.
* **AC-6** (kriteria 1): anggota macOS mematuhi aturan manifest akar spec bersama, dan file bersama tidak disentuh. **AC-11** spec 004 tercatat `[x]` di `verify.md` spec 004. `cargo test -p desktop-codegen --locked --test workspace_layout` (sebelum spec `_root/0007` memindahkannya, `-p desktop-core`) lulus sesudah `crates/macos-ffi` masuk `members`. Uji itu memeriksa setiap anggota saat berjalan: nama diawali `desktop-`, empat field diwarisi dari `[workspace.package]`, tidak ada glob, dan hanya ada satu `Cargo.lock`.
* **AC-7** (catatan per target, mendukung kriteria 1 dan 2): bukti yang dirujuk menyebut host, arsitektur, dan versi. ``grep -c 'macOS 27.0.1, `arm64`, `rustc 1.99.1`' docs/specs/_root/0001-batas-workspace-dan-pemilik/verify.md`` mencetak `1`. `awk '/^## Milestone 1/{f=1} /^## Milestone 3/{f=0} f' docs/specs/_root/0001-batas-workspace-dan-pemilik/verify.md | grep -c '^- \[ \]'` mencetak `0`. `verify.md` spec 004 bagian `## Milestone 3: Bukti build, kombinasi versi, dan kebersihan repo` memuat keluaran perintah **AC-10** spec 004 (versi macOS, arsitektur, Xcode, Swift, Rust, dan UniFFI).

## Options considered

### Opsi 1: Rujuk bukti yang ada, tambah satu langkah untuk peta arsitektur

Spec ini memetakan tiap kriteria ke catatan bukti yang sudah ada atau yang akan dibuat spec 004, dan hanya menambah satu langkah build: memperbarui tabel "Yang sudah ada".

**Pros**:
* Tidak ada perintah yang dijalankan dua kali dan tidak ada dua catatan untuk bukti yang sama.
* Peta arsitektur punya pemilik yang jelas untuk baris macOS.

**Cons**:
* Sebagian AC baru bisa diperiksa sesudah milestone spec 004 selesai.
* Pemeriksa harus membuka `verify.md` spec lain.

### Opsi 2: Jalankan ulang semua perintah di Mac dan catat di berkas bukti sendiri

Spec ini mengulang perintah spec bersama dan spec 004, lalu menyimpan hasilnya sendiri.

**Pros**:
* Semua bukti scope 001 untuk Mac ada di satu berkas.

**Cons**:
* Perintah yang sama dicatat di dua atau tiga tempat, dan catatan itu akan saling menyimpang.
* Host bukti spec bersama memang Mac ini, jadi pengulangan tidak menambah keyakinan.

### Opsi 3: Tanpa langkah build, peta arsitektur diperbarui lewat spec bersama

Perubahan peta arsitektur diminta sebagai pembaruan spec `_root/0001`, karena spec itu yang pertama menulis bagian tersebut.

**Pros**:
* Satu spec memiliki seluruh teks bagian "Yang sudah ada".

**Cons**:
* Setiap scope yang menambah lokasi (004, 006, 007, 008, dan seterusnya) harus mengubah spec bersama lebih dulu.
* Kriteria 27 untuk Mac menunggu pekerjaan di spec lain.

## Decision

**Chosen option**: Opsi 1: rujuk bukti yang ada, tambah satu langkah untuk peta arsitektur.

Bukti scope 001 di Mac adalah catatan `verify.md` spec `_root/0001` dan spec macOS 0001, dan spec ini hanya menambah tiga baris lokasi macOS ke peta arsitektur.

## Rationale

Host yang menghasilkan bukti spec bersama adalah Mac koordinasi itu sendiri, dan catatannya sudah menyebut versi macOS, arsitektur, dan versi Rust. Mengulangnya (Opsi 2) hanya menambah tempat yang bisa usang. Peta arsitektur berbeda: tidak ada spec lain yang memilikinya untuk baris macOS, dan aturan repo meminta dokumen yang terdampak ikut diperbarui. Opsi 3 menjaga satu pemilik teks, tetapi membuat setiap scope bergantung pada pembaruan spec bersama untuk hal yang murni lokal. Jadi langkah itu ditaruh di sini, dibatasi pada tambahan baris, satu baris, dan dua kalimat yang tidak lagi benar.

Cross check oleh model lain (Claude Opus, hanya membaca, 2026-11-15) membenarkan semua rujukan dan perintah, lalu meminta empat hal yang sudah diterapkan: kalimat ketiga yang ikut usang masuk ke langkah 1, batas waktu AC-8 spec bersama menjadi prasyarat dan bukan saran, tiap AC yang bersandar pada catatan lama mendapat satu perintah yang dijalankan ulang, dan tempat mencatat hasil perintah disebut.

Pembaruan 2026-11-15, atas permintaan lead: AC-8 spec bersama sudah dibatasi, jadi prasyarat itu dihapus dari Build plan, dan baris `crates/core/` yang masih berkata "kerangka" ikut diganti di langkah 1.

## Feature design

**Data model sketch**: tidak ada.

**State transitions**: tidak ada.

**API surface**: tidak ada API baru. Tabel ini menunjukkan bagian keputusan spec bersama yang berlaku di Mac dan file yang menerapkannya.

| Keputusan spec bersama | Penerapan di Mac | File |
| --- | --- | --- |
| Satu workspace, anggota ditulis satu per satu | `crates/macos-ffi` menjadi anggota kedua | `Cargo.toml` akar (spec 004 langkah 1) |
| Nama paket `desktop-` ditambah peran, empat field diwarisi | Paket `desktop-macos-ffi` | `crates/macos-ffi/Cargo.toml` |
| Konsumen pertama Core menambah `[workspace.dependencies]` | `desktop-core = { path = "crates/core" }`, dipakai jembatan lewat `desktop-core.workspace = true` | `Cargo.toml` akar, `crates/macos-ffi/Cargo.toml` |
| Dependensi luar milik satu anggota tinggal di anggota itu | `uniffi` hanya ada di manifest jembatan | `crates/macos-ffi/Cargo.toml` |
| Satu `Cargo.lock`, semua perintah bukti memakai `--locked` | Lockfile yang sama memuat UniFFI | `Cargo.lock` |
| Baris aturan `desktop-macos-ffi`: boleh Core dan alat binding | Dependensi langsung hanya `desktop-core` dan `uniffi` | `crates/macos-ffi/Cargo.toml` |
| Baris aturan `apps/macos/`: hanya lewat `bindings/swift/` dan pustaka jembatan | Hanya modul `Interop` yang mengimpor binding | `apps/macos/Package.swift`, `apps/macos/Interop/Greeter.swift` |
| Pemilik lokasi | `crates/macos-ffi/`, `bindings/swift/`, dan `apps/macos/` milik Aplikasi macOS | Tabel "Lokasi dan pemilik" spec bersama |
| Keluaran build tidak dilacak | `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/` sudah tertutup lima baris umum `.gitignore` | `.gitignore` (tidak berubah) |
| Pin toolchain | Target `aarch64-apple-darwin` adalah target host, jadi `rust-toolchain.toml` tidak butuh `targets` | `rust-toolchain.toml` (tidak berubah) |

Yang sengaja tidak berlaku di Mac:

* AC-9 dan milestone 3 spec bersama (Core di host Windows dan Linux, langkah manual `F001-1`).
* Tabel `target` dan atribut `cfg` dari aturan netral host. Belum ada anggota macOS yang memakai pustaka satu OS. `desktop-windows` dan `desktop-linux` belum ada.
* Mac Intel (`x86_64-apple-darwin`). Spec 004 tidak mendukungnya di fase ini.
* Lokasi `design-system/`, `shared/`, `tooling/`, `tests/` di akar, `scripts/`, dan `.github/workflows/`. Itu milik scope 002 dan 008 sampai 012.

**Perubahan dokumen arsitektur**:

Langkah 1 Build plan mengubah `docs/architecture/README.md` sebagai berikut. Bagian lain dokumen itu, termasuk tabel "Aturan arah dependensi", tidak diubah.

1. Di bagian "Tata letak yang dituju", ganti kalimat "Root workspace dan `crates/core/` sudah ada. Lokasi lain masih berupa rencana sampai scope pemiliknya dikerjakan." dengan:

   ```markdown
   Root workspace, `crates/core/`, `crates/macos-ffi/`, `bindings/swift/`, dan `apps/macos/` sudah ada. Lokasi lain masih berupa rencana sampai scope pemiliknya dikerjakan.
   ```

2. Di tabel bagian "Yang sudah ada", ganti baris `crates/core/` dengan baris pertama di bawah ini, lalu tambahkan tiga baris berikutnya sesudahnya. Core kini memuat `hello`, jadi kata "kerangka" tidak lagi benar:

   ```markdown
   | `crates/core/` | Paket `desktop-core`, Rust Core tanpa dependensi, berisi fungsi publik `hello` dari [spec 0002](../specs/_root/0002-kontrak-sapaan-bersama/index.md). |
   | `crates/macos-ffi/` | Paket `desktop-macos-ffi`, jembatan UniFFI tipis yang meneruskan API publik Core ke Swift, beserta generator binding. |
   | `bindings/swift/` | Paket Swift lokal berisi binding hasil UniFFI yang terlacak git. |
   | `apps/macos/` | Proyek Xcode `DesktopMac` dan paket Swift lokal dengan modul `Interop` dan `Presentation`. |
   ```

3. Ganti kalimat sesudah tabel itu, yang diawali "Nama paket yang dicadangkan", dengan:

   ```markdown
   Nama paket yang dicadangkan untuk lokasi yang masih rencana: `desktop-windows` di `apps/windows/` dan `desktop-linux` di `apps/linux/`. Rincian lokasi macOS ada di [spec macOS 0001](../specs/macos/0001-sapaan-di-jendela-macos/index.md).
   ```

**Value sourcing**:

| Aksi | Nilai yang dihasilkan atau ditampilkan | Sumber |
| --- | --- | --- |
| Pemeriksaan AC-1, AC-2, AC-7 | Host, arsitektur, dan versi Rust bukti workspace | Paragraf pembuka Milestone 1 di `verify.md` spec bersama |
| Pemeriksaan AC-1, AC-3, AC-5, AC-6, AC-7 | Hasil perintah untuk anggota macOS | `verify.md` spec 004, yang ditulis `/develop` per milestone |
| Langkah 1 | Teks empat baris tabel dan dua kalimat pengganti | Bagian **Perubahan dokumen arsitektur** spec ini |
| Langkah 1 | Nama paket, nama proyek, dan nama modul di baris itu | Spec 004, bagian Decision dan Tata letak |

**Key invariants**:

* Spec ini tidak punya keputusan sendiri tentang workspace. Bila isinya berbeda dari spec bersama atau spec 004, kedua spec itu yang berlaku.
* Tabel "Yang sudah ada" hanya memuat lokasi yang benar benar ada di `main`.
* Tabel "Aturan arah dependensi" di peta arsitektur tetap sama baris demi baris dengan spec bersama.

**Security model**: tidak ada perubahan. Spec ini tidak menambah rahasia, kredensial, atau akses.

**Configuration required**: tidak ada.

**Critical test scenarios**:

* Workspace di Mac dengan anggota macOS: perintah **AC-9** spec 004 pada host `arm64`, memeriksa **AC-1**, **AC-2**, **AC-3**.
* Aturan anggota: `cargo test -p desktop-codegen --locked --test workspace_layout` sesudah `crates/macos-ffi` bergabung, memeriksa **AC-4**, **AC-6**. Uji itu ada di `tooling/codegen/tests/` sejak spec `_root/0007`.
* Peta arsitektur: perintah `awk` dan `grep` di **AC-4**, memeriksa **AC-4**.
* Kelengkapan catatan: perintah `grep` di **AC-7** dan pembacaan `verify.md` spec 004, memeriksa **AC-5**, **AC-7**.

Tidak ada uji baru. Spec ini memakai uji dan perintah yang sudah ada.

## Build plan

Pendekatan build proyek adalah Tracer Bullet. Benang tipis untuk scope 001 sudah dibangun spec bersama dan spec 004, jadi yang tersisa di sini satu langkah dokumen. Satu langkah menjadi satu commit, dengan subject yang memuat `spec macos/0002 step 1`.

**Milestone 1: Peta arsitektur mencatat lokasi macOS** (langkah 1, needs scope 004 milestone 2)

1. Perbarui `docs/architecture/README.md` sesuai bagian **Perubahan dokumen arsitektur**, lalu jalankan perintah **AC-4**, satisfies **AC-4**

Langkah ini mengubah teks yang pertama ditulis spec bersama. AC-8 spec bersama sudah dibatasi "pada akhir milestone 2" pada 2026-11-15, dan AC itu menyebut bahwa tabel "Yang sudah ada" dan kalimat di sekitarnya disesuaikan oleh spec yang membangun lokasinya. Jadi langkah ini tidak menunggu spec bersama.

AC lain tidak punya langkah build di spec ini. Yang memenuhinya adalah build spec lain. Kotak `[x]` di `verify.md` spec 004 baru ada sesudah `/check verify` scope 004 berjalan, jadi verifikasi penuh spec ini needs kotak Verify it scope 004:

| AC | Dipenuhi oleh | Bisa diperiksa sesudah |
| --- | --- | --- |
| **AC-1** | Spec bersama milestone 1 (sudah selesai) dan spec 004 langkah 1 | Scope 004 milestone 1 |
| **AC-2** | Spec bersama milestone 1 (sudah selesai) dan spec 004 langkah 1 | Scope 004 milestone 1 |
| **AC-3** | Spec 004 langkah 1 dan 4 | Scope 004 milestone 2 |
| **AC-5** | Spec 004 langkah 5 | Scope 004 milestone 3 |
| **AC-6** | Spec 004 langkah 1 dan 5 | Scope 004 milestone 3 |
| **AC-7** | Spec bersama milestone 1 dan 2 (sudah selesai) dan spec 004 langkah 5 | Scope 004 milestone 3 |

File yang boleh diubah langkah 1: `docs/architecture/README.md`, baris `**Status**` spec ini, dan empat berkas `graphify-out/` dari hook. `docs/architecture/` biasanya terlarang bagi sesi developer, jadi langkah ini butuh izin eksplisit di brief.

## Consequences

**Positive**:

* Kotak scope 001 untuk macOS punya AC yang bisa diperiksa tanpa menjalankan ulang pekerjaan spec lain.
* Peta arsitektur berhenti menyebut lokasi macOS sebagai rencana.

**Negative / tradeoffs**:

* Bukti tersebar di tiga berkas (spec ini dan dua `verify.md`). Bila heading atau teks di `verify.md` itu berubah, perintah `grep` di **AC-7** ikut perlu disesuaikan.
* Verifikasi penuh spec ini menunggu kotak Verify it scope 004.
* Hanya macOS 27.0.1 pada `arm64` yang punya bukti. Mac Intel dan macOS yang lebih lama tidak tercakup.

**Neutral**:

* Sesudah langkah 1, satu kalimat di "Tata letak yang dituju" dan isi bagian "Yang sudah ada" tidak lagi sama kata demi kata dengan teks di bagian "Perubahan dokumen arsitektur" spec bersama. Itu sesuai AC-8 spec bersama, yang membatasi pemeriksaan teks pada akhir milestone 2. Bagian AC-8 yang berlaku terus tetap lulus, begitu juga uji `workspace_layout`.

## Follow-up

* [x] Lead dan pemilik spec `_root/0001` (prasyarat langkah 1): AC-8 spec itu berkata peta arsitektur "memuat teks" dari bagian "Perubahan dokumen arsitektur". Sesudah langkah 1 spec ini, dua kalimat berubah dan tabel "Yang sudah ada" punya baris tambahan. Tambahkan kata "pada akhir milestone 2" di AC-8, seperti perbaikan AC-4. Diterapkan di spec `_root/0001` (2026-11-15).
* [x] Lead dan pemilik spec `_root/0001`: baris `crates/core/` di tabel "Yang sudah ada" masih berkata "kerangka Rust Core tanpa dependensi", padahal Core sudah memuat `hello`. Perbarui lewat spec pemiliknya. Diterapkan di spec ini, langkah 1 (2026-11-15).
* [ ] Scope 006 dan 007 (menunggu host Windows dan Linux): spec penerapan scope 001 untuk Windows dan Linux menulis bukti host masing masing, termasuk hasil `cargo build --workspace --locked` dengan `desktop-macos-ffi` di dalam workspace.
* [ ] Scope 002: bila pemeriksaan arsitektur otomatis sudah ada, **AC-3** dan **AC-6** cukup merujuk pemeriksaan itu.
