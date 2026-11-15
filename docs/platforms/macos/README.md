# Fondasi macOS

Saat kamu menyusun spec macOS, gunakan nomor fitur dari [scope fondasi desktop](../../scope/scope.md). Lokasi spec mendatang adalah `docs/specs/macos/`. Catatan ini merangkum perilaku yang dituju dan bukti yang perlu dikumpulkan pada Mac.

## Jendela dan interaksi

Pengguna membuka jendela SwiftUI, mengetik nama, lalu menekan tombol. Untuk masukan `Ayu`, label menampilkan `Hello, Ayu!` dari Rust Core. Judul, kolom nama, tombol, dan label hasil memakai kontrol native sesuai Apple Human Interface Guidelines.

Swift menangani peluncuran, aktivasi, jendela, menu bar, navigasi, fokus keyboard, pintasan, dialog, animasi, aksesibilitas, izin, dan penghentian aplikasi. AppKit digunakan saat SwiftUI belum menyediakan kemampuan macOS yang dibutuhkan. Targetnya adalah aplikasi macOS native dengan SwiftUI. Mac Catalyst di luar lingkup fondasi ini.

## Batas Swift dan Rust

| Bagian yang direncanakan | Tanggung jawab |
|---|---|
| `apps/macos/` | `App` mengelola siklus hidup. `Presentation` memuat tampilan, state dan ViewModel. `Interop` menghubungkan ViewModel ke Rust. `Platform` menangani API macOS. `DesignSystem` dan `Resources` menyediakan tema dan sumber daya native. |
| `bindings/swift/` | Antarmuka Swift yang dihasilkan UniFFI untuk dipakai adapter. Perubahan berasal dari kontrak Rust dan proses generasi yang dapat diulang. |
| `crates/macos-ffi/` | Jembatan tipis yang mengekspos kontrak publik Core ke Swift. Spec menjelaskan kepemilikan memori dan sumber daya serta pemetaan galat Rust ke Swift. |
| `crates/core/` | Pemilik fungsi sapaan dan perilaku bersama. Core dapat dibangun dan diuji tanpa SwiftUI atau AppKit. |

Alur panggilan dimulai di ViewModel, melewati adapter dan binding Swift, lalu jembatan memanggil `hello` di Core. Swift menyimpan hasil untuk ditampilkan. Perubahan UI mengikuti aturan thread utama SwiftUI dan AppKit. Operasi yang dapat memblokir dijalankan di luar thread tersebut.

## Tema dan sumber daya

Warna semantik, tipografi dan jarak berasal dari token bersama yang dihasilkan untuk Swift. Jendela mengikuti tampilan terang dan gelap, skala teks, fokus keyboard dan perilaku aksesibilitas native.

Contoh lokalisasi menggunakan kunci bersama dan sumber daya Apple, dengan format lokal serta bentuk jamak native. Nilai konfigurasi bersama mengikuti skema bersama, sedangkan pengaturan khusus macOS dapat tetap native. Aset merek diekspor dari sumber bersama untuk bundel aplikasi. Kredensial penandatanganan disimpan di luar repositori.

## Bukti penerimaan pada Mac

| Hal yang diperiksa | Bukti untuk spec terkait |
|---|---|
| Kompatibilitas | Versi minimum macOS serta kombinasi Xcode, Swift, Rust dan UniFFI yang digunakan. Target utama adalah Apple Silicon ARM64. Intel `x86_64` dicatat bila didukung. |
| Build | Perintah yang dapat diulang pada Mac dan runner macOS di GitHub Actions untuk membangun pustaka Rust, menghasilkan binding Swift, lalu membangun aplikasi melalui Xcode. Hasil build dicatat per arsitektur. |
| Integrasi | Uji Core dapat dijalankan sendiri. Binding dapat dihasilkan ulang dan aplikasi Swift berhasil memanggil fungsi Rust melalui jembatan. |
| Penggunaan nyata | Pada sesi GUI yang tersedia, jalankan uji singkat dari membuka jendela hingga menampilkan sapaan. Periksa token tema dalam tampilan terang dan gelap, fokus keyboard, serta aksesibilitas kontrol. |

Saat verifikasi dilakukan, catat host, arsitektur, perintah dan hasil aktual. Uji antarmuka atau target yang belum dijalankan tetap tercatat sebagai belum diverifikasi.

## Distribusi

Format aplikasi yang dituju adalah bundel `.app`. Dokumentasi distribusi kelak mencakup sandbox, penandatanganan kode dan notarisasi. DMG, PKG dan Mac App Store adalah pilihan distribusi lanjutan. Pembuatan installer produksi berada di luar fase fondasi.
