# Langkah manual: `000 - Panduan scope`

Langkah yang butuh mata, tangan, akun, perangkat, atau izin kamu. Tidak ada sesi yang menjalankannya. Kirim hasilnya ke lead (ID langkah, `PASS` atau `FAIL`, dan output bila `FAIL`); lead meneruskannya ke checker yang mencentang `verify.md`. `FAIL` membuka kembali kotaknya.

Keputusan yang diambil atas namamu ada di `000 - Panduan scope.log.md` bagian "Keputusan delegasi"; baris bertanda `berat` paling perlu kamu baca. Untuk membatalkan satu baris, sebut barisnya ke lead.

## Ringkasan

| ID | Langkah | AC | Kotak yang dicentang dengan langkah ini `NOT_RUN` | Bisa dijalankan sesudah | Hasil |
| --- | --- | --- | --- | --- | --- |
| `F001-1` | Bangun dan uji Core di host Windows dan host Linux | spec `_root/0001` AC-9 | `Verify it` fitur 001 (bukti Mac). Milestone 3 dan `Build it` fitur 001 tetap terbuka sampai langkah ini `PASS` | 001 milestone 1 di `origin/main` (sesudah push penutupan) | `NOT_RUN` |
| `F004-1` | Alur sapaan di jendela: build, buka aplikasi, `Ayu`, kolom kosong, teks Unicode (langkah 1 sampai 5) | spec `macos/0001` AC-5 | `Verify it` fitur 004 (bukti tanpa jendela) dan `Verify it` bagian macOS fitur 003 (spec `macos/0003` AC-4 bagian jendela). Milestone 4 dan `Build it` fitur 004 tetap terbuka sampai langkah ini `PASS` | 004 milestone 2 di `main` (terpenuhi di `main` lokal, `c969a82`) | `NOT_RUN` |
| `F004-2` | Penggunaan nyata: Return tanpa mouse, VoiceOver, Command Q, Run dari Xcode (langkah 6 sampai 9) | spec `macos/0001` AC-13 | sama dengan `F004-1` | 004 milestone 2 di `main` (terpenuhi di `main` lokal, `c969a82`) | `NOT_RUN` |

## Nilai dari user

Nilai yang hanya kamu tahu. AC di kolom ketiga `BLOCKED` sampai nilainya ada.

| ID | Nilai yang dibutuhkan | AC yang menunggu | File yang menunggu |
| --- | --- | --- | --- |
| `N1` | Lisensi paket Rust (field `license` di `[workspace.package]`). Sampai ada, `publish = false` menjaga paket tetap privat. | tidak ada; Follow-up spec `_root/0001` | `Cargo.toml` root, lewat pembaruan spec `_root/0001` |
| `N2` | Bahasa komentar kode dan rustdoc (sekarang Inggris di Core, dokumen repo Indonesia). Architect scope 002 akan mengambil rekomendasinya bila kamu tidak menyebut pilihan. | tidak ada; Follow-up spec `_root/0001` dan `_root/0002` | spec scope 002 |
| `N3` | Gaya pesan commit, awalan branch, dan preferensi gaya di luar bawaan `rustfmt` dan `clippy`. Audit tidak menebaknya dan tidak menuliskannya di `AGENTS.md`. | tidak ada | `AGENTS.md`, bila kamu ingin aturannya tertulis |
| `N4` | Pengenal bundel akhir aplikasi macOS. Spec memakai nilai sementara `com.example.desktop-foundation.macos`, cukup untuk build lokal, wajib diganti sebelum penandatanganan atau distribusi. | tidak ada; Follow-up spec `macos/0001` | `apps/macos/DesktopMac.xcodeproj/project.pbxproj`, lewat pembaruan spec `macos/0001` |
| `N5` | `NAMA_PRODUK`: nama produk yang tampil di ketiga OS. Nilai sementara `Desktop Foundation`. | spec `_root/0004` AC-2 (lulus dengan nilai sementara) | `shared/config/public.json` (`build.product_name`) |
| `N6` | `DASAR_PENGENAL`: dasar pengenal reverse DNS dari domain milik pemilik produk. Nilai sementara `com.example.desktop-foundation`. `N4` (pengenal bundel macOS) diturunkan darinya dengan aturan spec macOS. | spec `_root/0004` AC-2 (lulus dengan nilai sementara) | `shared/config/public.json` (`build.app_identifier`) |
| `N7` | Warna merek: warna utama, latar terang dan gelap, teks terang dan gelap, dan apakah tampilan gelap memakai rona merek yang berbeda. Tiga warna dasar sekarang (`brand.primary`, `neutral.50`, `neutral.950`) adalah contoh bertanda `placeholder`. | spec `_root/0003` AC-8 (lulus dengan nilai sementara; alat melaporkan `3 placeholder values`) | `design-system/tokens/colors.json` |

## Fitur `001`: `Batas workspace dan pemilik` (spec `_root/0001`)

### `F001-1` · Bangun dan uji Core di host Windows dan host Linux

Dari laporan `arc-f001-spec` (2026-11-15). Membuktikan spec `_root/0001` `AC-9` (kriteria 2 pada host lain); ini milestone 3 fitur 001.

- **Prasyarat:** milestone 1 fitur 001 sudah di `main` dan sudah di-clone di host itu. Host Windows `x86_64-pc-windows-msvc` dengan `rustup` dan MSVC Build Tools; host Linux `x86_64-unknown-linux-gnu` dengan `rustup` dan `cc`. Tanpa pustaka UI.
- **Jalankan:** di root repo, pada tiap host:
  - `rustc --version`
  - `cargo build -p desktop-core --locked`
  - `cargo test -p desktop-core --locked`
- **Lulus bila:** `rustc --version` diawali `rustc 1.99.1`, kedua perintah `cargo` keluar dengan kode 0, dan keluaran test memuat `test result: ok` tanpa `FAILED`. Kirim OS, arsitektur, versi, perintah, dan keluarannya; checker mencatatnya di `verify.md` bagian `## Milestone 3: Bukti Core di host Windows dan Linux`.
- **Hanya langkah ini yang bisa membuktikan:** bahwa Core terbangun dan teruji di luar macOS. Semua bukti lain fitur 001 dijalankan di Mac.

## Fitur `004`: `Sapaan di jendela macOS` (spec `macos/0001`)

Kedua langkah di bawah adalah milestone 4 fitur 004. Catat host, arsitektur, versi macOS, dan hasil tiap nomor. Yang belum dijalankan tetap tercatat belum diverifikasi.

### `F004-1` · Alur sapaan di jendela

Dari laporan `arc-f004-spec` (2026-11-15). Membuktikan spec `macos/0001` `AC-5` (Done when scope 004, kriteria 5 dan 14).

- **Prasyarat:** Mac Apple Silicon dengan macOS 14 atau lebih baru (yang sudah diuji: 27.0.1), Xcode 27.1, sesi desktop yang sedang login, dan repo pada commit yang lulus AC-4 (004 milestone 2 sudah di-merge).
- **Jalankan:**
  1. Dari akar repo: `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build`
  2. `open apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app`
  3. Ketik `Ayu` di kolom, lalu klik `Greet`.
  4. Hapus isi kolom, lalu klik `Greet`.
  5. Tempel teks `Łukasz 李 👩🏽‍💻` ke kolom, lalu klik `Greet`.
- **Lulus bila:** (1) `** BUILD SUCCEEDED **`; (2) satu jendela berjudul `Desktop Foundation` dengan kolom `Name`, tombol `Greet`, dan label hasil yang kosong; (3) label menampilkan `Hello, Ayu!`; (4) label menampilkan `Hello, !`; (5) label menampilkan `Hello, Łukasz 李 👩🏽‍💻!` dengan emoji yang utuh.
- **Hanya langkah ini yang bisa membuktikan:** bahwa nilai yang terlihat di jendela nyata berasal dari Core. Uji `swift test` membuktikan jalur yang sama tanpa jendela.

### `F004-2` · Penggunaan nyata

Dari laporan `arc-f004-spec` (2026-11-15). Membuktikan spec `macos/0001` `AC-13`.

- **Prasyarat:** sama dengan `F004-1`, dan aplikasi masih terbuka dari langkah 2.
- **Jalankan:**
  6. Hapus isi kolom, ketik `Budi`, lalu tekan Return tanpa memakai mouse.
  7. Nyalakan VoiceOver dengan Command F5 (pada sebagian laptop Fn Command F5), lalu pindah ke kolom, tombol, dan label. Matikan VoiceOver sesudahnya.
  8. Tekan Command Q.
  9. `open apps/macos/DesktopMac.xcodeproj`, tunggu paket selesai dimuat, lalu tekan Command R. Tutup Xcode, lalu jalankan `git status --short`.
- **Lulus bila:** (6) label menampilkan `Hello, Budi!`; (7) VoiceOver menyebut `Name` untuk kolom, `Greet` untuk tombol, dan membacakan `Hello, Budi!` untuk label; (8) aplikasi tertutup tanpa dialog; (9) build berhasil dan jendela yang sama terbuka. Bila `git status --short` mencetak sesuatu, catat setiap path dan jangan di-commit; keluaran itu tidak menggagalkan langkah 9.
- **Hanya langkah ini yang bisa membuktikan:** fokus keyboard, aksesibilitas kontrol native, dan bahwa proyek tulisan tangan terbuka benar di Xcode.
