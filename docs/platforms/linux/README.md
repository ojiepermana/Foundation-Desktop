# Fondasi Linux

Untuk menyusun spec Linux, kamu bisa mengambil nomor dan tujuan fitur dari [scope bersama](../../scope/scope.md). Folder tujuan spec adalah `docs/specs/linux/`. Panduan ini memuat target awal. Versi pustaka, dukungan arsitektur, dan hasil pengujian masih perlu dibuktikan pada host Linux.

## Jendela native dan Rust Core

Target pertama adalah satu jendela GTK 4 dan libadwaita. Jendela menampilkan judul aplikasi bersama, kolom nama, tombol, dan label hasil. Saat kamu memasukkan `Ayu` lalu menekan tombol, label menampilkan `Hello, Ayu!` dari fungsi `hello` di Rust Core.

| Bagian | Tanggung jawab |
| --- | --- |
| Aplikasi Linux | Crate di `apps/linux/` memakai `gtk-rs`. Presentasi mengelola widget GTK, jendela libadwaita, navigasi, pintasan, fokus, aksesibilitas, tema, dan keadaan layar. |
| Rust Core | Aplikasi mengimpor API publik `crates/core/` langsung melalui Cargo. Algoritma sapaan hanya dimiliki Core. Dependensi GTK, libadwaita, dan crate FFI macOS tetap di luar Core. |
| Integrasi desktop | Adaptor Linux menangani aktivasi aplikasi dan API desktop. Akses berkas, notifikasi, dan integrasi sistem jendela kelak memiliki pemilik di sini. |

Saat aplikasi berjalan, GTK menangani aktivasi dan penutupan jendela. Pembaruan widget berjalan pada konteks utama GTK agar antarmuka tetap responsif. Saat aplikasi ditutup, aplikasi melepaskan sumber daya yang dimilikinya.

## Sesi desktop

Wayland adalah target utama. Dalam sesi Wayland normal, GTK menggunakan backend Wayland asli tanpa memerlukan XWayland. Compositor mengatur posisi jendela. Aplikasi mengikuti skala layar, termasuk HiDPI, serta memakai abstraksi GTK untuk input dan papan klip. Interaksi perlu bekerja tanpa koordinat jendela global.

Backend X11 tetap tersedia untuk sesi X11 yang kompatibel. Lingkungan sasaran mencakup GNOME, KDE Plasma, dan desktop lain yang mendukung GTK. libadwaita menentukan tampilan aplikasi, sedangkan dekorasi jendela dan layanan yang tersedia dapat berbeda antarlingkungan.

## Tema dan sumber daya

| Sumber bersama | Penerapan di Linux |
| --- | --- |
| Token desain | Warna semantik, tipografi, dan jarak berasal dari token bersama. Pemetaan ke CSS GTK dan definisi Rust bila diperlukan bisa dihasilkan ulang. Jendela mengikuti pengaturan terang atau gelap serta mempertahankan aksesibilitas kontrol GTK. |
| Lokalisasi | Contoh teks antarmuka memakai kunci bersama melalui sumber daya yang kompatibel dengan GTK. Format lokal dan aturan jamak mengikuti bahasa pengguna. |
| Konfigurasi | Nilai publik lintas OS mengikuti skema bersama. Preferensi yang hanya relevan di Linux boleh dimiliki aplikasi Linux. |
| Aset | Ikon aplikasi mengikuti konvensi desktop Linux dan berasal dari aset merek bersama. |

## Bukti yang diperlukan di Linux

| Yang perlu dibuktikan | Bukti untuk spec terkait |
| --- | --- |
| Build | Perintah build yang dapat diulang pada host Linux, versi Rust dan `gtk-rs`, versi minimum GTK 4 dan libadwaita, serta pustaka sistem yang dibutuhkan. Target awal adalah `x86_64`, dengan `aarch64` bila didukung. Catat hasil untuk setiap target yang dicoba. |
| Integrasi | Aplikasi GTK dapat dibuka, menerima nama, menampilkan hasil panggilan langsung ke Core, memakai tema bersama dalam kedua mode, dan ditutup dengan bersih. |
| Sesi grafis | Catat backend yang digunakan untuk uji Wayland native dan untuk uji kompatibilitas X11. Bila sesi yang dibutuhkan belum tersedia, hasil untuk sesi itu tetap belum terverifikasi. |
| CI | Runner Linux di GitHub Actions memasang dependensi GTK, membangun aplikasi, dan menjalankan pemeriksaan yang dapat dilakukan tanpa layar. Verifikasi interaksi Wayland memerlukan compositor atau sesi desktop Linux. Hasil build dan hasil uji jendela dicatat terpisah. |

## Portal dan distribusi mendatang

Untuk fitur mendatang, XDG Desktop Portal menjadi jalur yang dipertimbangkan untuk pemilih berkas, membuka URI, notifikasi, berbagi layar, dan izin desktop. Aplikasi mengikuti izin sistem berkas dan menggunakan hak akses secukupnya. Fondasi hanya mencatat batas ini.

Flatpak adalah kandidat utama untuk distribusi portabel. Evaluasinya mencakup sandbox, portal, GTK, dan dependensi runtime. DEB, RPM, serta AppImage juga dapat dipertimbangkan sesuai kebutuhan distribusi. Paket produksi berada di luar tahap fondasi.
