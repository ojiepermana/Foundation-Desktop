# Langkah manual: `000 - Panduan scope`

Langkah yang butuh mata, tangan, akun, perangkat, atau izin kamu. Tidak ada sesi yang menjalankannya. Kirim hasilnya ke lead (ID langkah, `PASS` atau `FAIL`, dan output bila `FAIL`); lead meneruskannya ke checker yang mencentang `verify.md`. `FAIL` membuka kembali kotaknya.

Keputusan yang diambil atas namamu ada di `000 - Panduan scope.log.md` bagian "Keputusan delegasi"; baris bertanda `berat` paling perlu kamu baca. Untuk membatalkan satu baris, sebut barisnya ke lead.

## Ringkasan

| ID | Langkah | AC | Kotak yang dicentang dengan langkah ini `NOT_RUN` | Bisa dijalankan sesudah | Hasil |
| --- | --- | --- | --- | --- | --- |
| `F001-1` | Bangun dan uji Core di host Windows dan host Linux | spec `_root/0001` AC-9 | `Verify it` fitur 001 (bukti Mac). Milestone 3 dan `Build it` fitur 001 tetap terbuka sampai langkah ini `PASS` | 001 milestone 1 di `origin/main` (sesudah push penutupan) | `NOT_RUN` |
| `F008-1` | Bangun, uji, dan periksa token dengan `desktop-codegen` di host Windows dan host Linux | spec `_root/0003` AC-12 | `Verify it` fitur 008 (bukti Mac). Milestone 3 dan `Build it` fitur 008 tetap terbuka sampai langkah ini `PASS` | 008 milestone 1 di `origin/main` | `NOT_RUN` |
| `F002-1` | Bangun, uji, dan jalankan pemeriksaan arsitektur di host Windows dan host Linux | spec `_root/0007` AC-11 | belum ada | 002 milestone 3 di `main`, dan host Windows serta Linux | `NOT_RUN` |
| `F008-2` | Tampilan terang dan gelap dari token, fokus, keyboard, ukuran teks, VoiceOver, Increase contrast (langkah 1 sampai 10) | spec `macos/0005` AC-7, AC-8 | belum ada | 008 macOS milestone 2 di `main` | `NOT_RUN` |
| `F009-1` | Judul jendela dari konfigurasi publik dan nama menu aplikasi (langkah 1 sampai 4) | spec `macos/0004` AC-7 | belum ada | 009 macOS milestone 1 di `main` | `NOT_RUN` |
| `F010-1` | Teks tombol mengikuti bahasa: `Sapa` untuk `id` dan `id-ID`, `Greet` untuk `fr` dan Inggris, VoiceOver (langkah 1 sampai 7) | spec `macos/0006` AC-11 | belum ada | 010 macOS milestone 1 di `main` | `NOT_RUN` |
| `F011-1` | Mengganti ikon penanda dengan ikon aplikasi nyata dalam SVG, lalu `assets generate` dan `assets check` | spec `_root/0005` AC-10 | belum ada | 011 milestone 1 di `main`, dan berkas ikon darimu (`N9`) | `NOT_RUN` |
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
| `N8` | Bahasa yang didukung selain bahasa sumber, dan teks terjemahan yang disetujui. Sekarang: sumber `en` (`Greet`), bahasa kedua `id` (`Sapa`), usulan architect. | spec `_root/0006` AC-1 (lulus dengan usulan itu) | `shared/localization/catalog.json` |
| `N9` | Berkas ikon aplikasi nyata dalam SVG (dan berkas merek lain bila ada). Sekarang `shared/assets/source/app-icon.svg` akan berupa penanda kotak abu abu bersilang bertanda `placeholder`. | spec `_root/0005` AC-10 (`BLOCKED`, `owner: user`) | `shared/assets/source/`, `shared/assets/brand.json` |

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

## Fitur `002`: `Aturan pengembangan dan pemeriksaan arsitektur` (spec `_root/0007`)

### `F002-1` · Bangun, uji, dan jalankan pemeriksaan arsitektur di host Windows dan host Linux

Dari laporan `arc-f002-spec` (2026-11-15). Membuktikan spec `_root/0007` `AC-11` (kriteria 25 pada host lain); ini milestone 4 fitur 002.

- **Prasyarat:** milestone 3 fitur 002 sudah di `main` dan di-clone di host itu. Host Windows `x86_64-pc-windows-msvc` dengan `rustup` dan MSVC Build Tools; host Linux `x86_64-unknown-linux-gnu` dengan `rustup` dan `cc`; toolchain `1.99.1`. Tanpa pustaka UI.
- **Jalankan:** dari akar repo, pada tiap host:
  - `rustc --version`
  - `cargo test -p desktop-codegen --locked`
  - `cargo run -p desktop-codegen --locked -- architecture check`
- **Lulus bila:** `rustc --version` diawali `rustc 1.99.1`, kedua perintah `cargo` keluar dengan kode 0, keluaran test tidak memuat `FAILED`, dan baris terakhir perintah ketiga diawali `architecture: ok, 13 rules, 3 members` (angka sesuai `main` saat itu). Kirim OS, arsitektur, versi, perintah, dan keluarannya.
- **Hanya langkah ini yang bisa membuktikan:** pemeriksaan teks (akhir baris `\r\n`, path dengan `/`) memberi hasil yang sama di luar macOS.

Pertanyaan untukmu dari spec yang sama, bukan langkah uji: apakah hook `pre-merge-commit`, penggabungan `manifest.json`, atau hook `pre-push` yang menjalankan gerbang perlu diubah. Folder `.githooks/` terlarang bagi sub sesi, jadi itu hanya berubah atas katamu.

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

## Fitur `008`: `Tema bersama dan kontrol native` (spec `_root/0003`)

### `F008-1` · Bangun, uji, dan periksa token di host Windows dan host Linux

Dari laporan `arc-f004-spec` (2026-11-15). Membuktikan spec `_root/0003` `AC-12` (kriteria 16 pada host lain); ini milestone 3 fitur 008.

- **Prasyarat:** host Windows `x86_64-pc-windows-msvc` dan host Linux `x86_64-unknown-linux-gnu`, `rustup` dengan toolchain `1.99.1`, checkout `main` sesudah milestone 1 fitur 008.
- **Jalankan:** dari akar repo, pada tiap host:
  - `cargo build -p desktop-codegen --locked`
  - `cargo test -p desktop-codegen --locked`
  - `cargo run -p desktop-codegen --locked -- tokens check`
- **Lulus bila:** ketiganya keluar dengan kode 0, dan baris terakhir perintah ketiga `tokens: ok, 6 semantic tokens, 3 placeholder values, 3 outputs up to date`. Kirim OS, arsitektur, versi, perintah, dan keluarannya.
- **Hanya langkah ini yang bisa membuktikan:** bahwa alat generator dan token terbangun dan memberi keluaran yang sama di luar macOS.

### `F008-2` · Tampilan terang dan gelap dari token dan kontrol native

Dari laporan `arc-f004-spec` (2026-11-15). Membuktikan spec `macos/0005` `AC-7` (langkah 1 sampai 5) dan `AC-8` (langkah 6 sampai 10); ini milestone 3 bagian macOS fitur 008. Teks lengkap dan sama persis ada di spec `macos/0005` bagian "Rencana verifikasi manual".

- **Prasyarat:** Mac Apple Silicon dengan macOS 14 atau lebih baru (yang dipakai membangun 27.0.1), Xcode 27.1, sesi desktop yang sedang login, repo pada commit yang lulus AC-5 spec `macos/0005` (milestone 2 bagian macOS sudah di-merge). Nilai warna dibaca dari `apps/macos/DesignSystem/Tokens.swift` pada commit itu; angka di bawah nilai sementara. Bilah judul milik sistem dan tidak dinilai. Warna tombol hanya dinilai saat jendela `DesktopMac` aktif, jadi klik jendela itu dulu.
- **Jalankan:**
  1. Dari akar repo: `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build`. Harapan: `** BUILD SUCCEEDED **`.
  2. System Settings, Appearance: catat Appearance dan Accent color sekarang. Pilih Appearance Light dan Accent color Orange. Jalankan `open apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app`, ketik `Ayu`, klik `Greet`. Harapan: area isi hampir putih (`#FAFAFA`), teks `Hello, Ayu!` hampir hitam (`#111111`), tombol `Greet` berisi biru (`#3366CC`), bukan oranye, kolom `Name` seperti kolom teks Mac biasa.
  3. Buka Digital Color Meter, pilih "Display in sRGB". Klik jendela `DesktopMac`, arahkan kursor ke area isi dan ke isi tombol `Greet`. Harapan: sekitar `250 250 250` dan `51 102 204`.
  4. Tanpa menutup aplikasi, ganti Appearance ke Dark, klik jendela. Harapan: jendela berubah tanpa dibuka ulang; area isi hampir hitam (`#111111`), teks hampir putih (`#FAFAFA`), tombol tetap biru. Ukur lagi: sekitar `17 17 17` dan `51 102 204`.
  5. Command Q, lalu `open` lagi saat Appearance masih Dark, ketik `Ayu`, klik `Greet`. Harapan: jendela langsung gelap, teks hampir putih. Biarkan Dark dan Orange sampai langkah 6 selesai.
  6. Klik kolom `Name`. Harapan: cincin fokus sistem terlihat dan berwarna oranye, bukan biru token. Hapus isi, ketik `Budi`, tekan Return tanpa mouse. Harapan: `Hello, Budi!`. Ganti Appearance ke Light, klik kolom lagi, amati cincin fokus yang sama.
  7. System Settings, Keyboard: nyalakan Keyboard navigation. Klik kolom `Name`, hapus isi, ketik `Citra`, tekan Tab. Harapan: fokus pindah ke tombol `Greet` dengan cincin fokus. Tekan Spasi. Harapan: `Hello, Citra!`. Kembalikan pengaturan.
  8. System Settings, Accessibility, Display: perbesar Text Size. Lihat jendela aplikasi (buka ulang bila perlu, ketik `Ayu`, klik `Greet`). Harapan: tidak ada teks terpotong atau bertumpuk, label hasil seukuran dengan teks tombol dan kolom. Catat apakah teks ikut membesar (boleh tidak; itu dicatat, bukan gagal). Kembalikan pengaturan.
  9. Hapus isi kolom, ketik `Budi`, tekan Return. Nyalakan VoiceOver (Command F5), pindah ke kolom, tombol, dan label. Harapan: `Name`, `Greet`, dan `Hello, Budi!` dibacakan. Matikan VoiceOver.
  10. System Settings, Accessibility, Display: nyalakan Increase contrast, klik jendela. Harapan: area isi dan teks hasil tetap memakai warna token dan mudah dibaca, kontrol bergaris tepi lebih tegas. Matikan Increase contrast, kembalikan Appearance dan Accent color, Command Q.
- **Lulus bila:** setiap harapan di atas terpenuhi. Kirim host, arsitektur, versi macOS, tampilan yang aktif, dan hasil tiap langkah.
- **Hanya langkah ini yang bisa membuktikan:** bahwa jendela nyata menggambar warna token di tampilan terang dan gelap, dan bahwa fokus, keyboard, dan VoiceOver tetap milik kontrol native.

## Fitur `009`: `Konfigurasi publik` (spec `macos/0004`)

### `F009-1` · Judul jendela dari konfigurasi publik dan nama menu aplikasi

Dari laporan `arc-f001-spec` (2026-11-15). Membuktikan spec `macos/0004` `AC-7`; ini milestone 2 bagian macOS fitur 009.

- **Prasyarat:** Mac Apple Silicon dengan macOS 14 atau lebih baru (yang diuji 27.0.1), Xcode 27.1, sesi desktop yang sedang login, repo pada commit yang lulus AC-4 spec `macos/0004` (milestone 1 bagian macOS sudah di-merge).
- **Jalankan:**
  1. Dari akar repo: `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build`
  2. `jq -r '.build.product_name' shared/config/public.json`, catat keluarannya, lalu `open apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app`
  3. Lihat menu bar selagi aplikasi di depan. Catat juga nama di atas ikon Dock.
  4. Tekan Command Q.
- **Lulus bila:** (1) `** BUILD SUCCEEDED **`; (2) satu jendela terbuka dan judulnya sama persis dengan keluaran `jq` (nilai sementara `Desktop Foundation`); (3) menu aplikasi di sebelah menu Apple bernama `DesktopMac` (nama di Dock dicatat saja, tidak menentukan lulus atau gagal); (4) aplikasi tertutup tanpa dialog. Kirim host, arsitektur, versi macOS, versi Xcode, dan hasil tiap nomor.
- **Hanya langkah ini yang bisa membuktikan:** bahwa judul jendela nyata berasal dari `shared/config/public.json` lewat bundel, bukan dari literal.

## Fitur `010`: `Satu contoh lokalisasi` (spec `macos/0006`)

### `F010-1` · Teks tombol mengikuti bahasa di jendela nyata

Dari laporan `arc-f010-spec` (2026-11-15). Membuktikan spec `macos/0006` `AC-11` (OS-9); ini milestone 2 bagian macOS fitur 010.

- **Prasyarat:** Mac Apple Silicon dengan macOS 14 atau lebih baru (yang diuji 27.0.1), Xcode 27.1, sesi desktop yang sedang login, repo pada commit yang lulus AC-8 spec `macos/0006` (milestone 1 bagian macOS sudah di-merge), dan aplikasi DesktopMac tidak sedang berjalan.
- **Jalankan:**
  1. Dari akar repo: `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build`. Harapan: `** BUILD SUCCEEDED **`.
  2. `open -n apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app --args -AppleLanguages '(id)'`. Harapan: tombol bertuliskan `Sapa`, kolom tetap `Name`. Ketik `Ayu`, klik `Sapa`. Harapan: label `Hello, Ayu!`. Command Q.
  3. `open -n apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app --args -AppleLanguages '(id-ID)'`. Harapan: tombol `Sapa`. Command Q.
  4. `open -n apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app --args -AppleLanguages '(fr)'`. Harapan: tombol `Greet`. Command Q.
  5. System Settings, General, Language & Region: tambahkan Bahasa Indonesia bila belum ada dan seret ke urutan pertama. `open apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app` tanpa argumen. Harapan: tombol `Sapa`. Catat apakah menu bawaan macOS ikut berbahasa Indonesia atau tetap Inggris; keduanya tidak menggagalkan langkah ini.
  6. Dengan bahasa Indonesia masih pertama, nyalakan VoiceOver (Command F5), pindah ke tombol. Harapan: `Sapa` dibacakan. Ketik `Ayu`, tekan Return, pindah ke label. Harapan: `Hello, Ayu!` dibacakan. Matikan VoiceOver, Command Q.
  7. Kembalikan urutan bahasa, buka aplikasi lagi. Harapan: tombol `Greet`. Command Q, lalu `git status --short`. Harapan: tidak ada keluaran.
- **Lulus bila:** setiap harapan di atas terpenuhi. Kirim host, arsitektur, versi macOS, dan hasil tiap langkah.
- **Hanya langkah ini yang bisa membuktikan:** bahwa jendela nyata menampilkan teks dari katalog dalam bahasa yang dipilih sistem, dan bahwa hasil `hello` tetap bahasa Inggris.

## Fitur `011`: `Aset merek bersama` (spec `_root/0005`)

### `F011-1` · Mengganti ikon penanda dengan ikon aplikasi nyata

Dari laporan `arc-f011-spec` (2026-11-15). Membuktikan spec `_root/0005` `AC-10`; ini milestone 3 fitur 011.

- **Prasyarat:** milestone 1 spec `_root/0005` sudah di `main` (`shared/assets/` dan sumber `assets` di `desktop-codegen` ada). Kamu punya ikon aplikasi asli dalam SVG yang memenuhi "Format sumber" di `shared/assets/README.md`: teks UTF-8, akhir baris LF, paling besar 262144 byte, elemen pertama `svg` dengan `xmlns="http://www.w3.org/2000/svg"` dan `viewBox="0 0 1024 1024"`, hanya bentuk vektor dari daftar elemen, tanpa tulisan, gambar tertanam, gaya, skrip, atau rujukan luar.
- **Jalankan:** dari akar repo:
  1. Salin berkasmu ke `shared/assets/source/app-icon.svg` (menimpa penanda).
  2. Di `shared/assets/brand.json`, hapus baris `"placeholder": true` dari aset `app-icon` (dan koma sebelumnya).
  3. `cargo run -p desktop-codegen --locked -- assets generate`
  4. `cargo run -p desktop-codegen --locked -- assets check`
  5. `grep -c '"placeholder"' shared/assets/brand.json`
  6. Commit sumber, manifest, dan keluaran bersama.
- **Lulus bila:** langkah 4 keluar dengan kode 0 dan baris terakhirnya memuat `0 placeholder values`; langkah 5 mencetak `0`. Bila langkah 4 menolak sumber (aturan S), ekspor ulang ikonnya; bentuk merek tidak diperbaiki oleh sesi mana pun.
- **Hanya langkah ini yang bisa membuktikan:** bahwa berkas merek nyata lolos aturan sumber. Bila kamu hanya punya PNG atau desain berbeda per OS, spec `_root/0005` diperbarui lebih dulu.
