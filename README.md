# Fondasi desktop native

Repositori ini merencanakan tiga aplikasi desktop dengan satu Rust Core dan antarmuka native pada setiap sistem operasi. Contoh pertamanya kecil. Pengguna mengetik `Ayu`, menekan tombol, lalu melihat `Hello, Ayu!` yang dikembalikan oleh fungsi Rust yang sama.

**Mulai dari [indeks 12 fitur fondasi desktop](docs/scope/scope.md).** Indeks menunjukkan urutan kerja dan menautkan tiap fitur ke berkas yang memuat kriteria selesai dan tujuan spec. Workspace Rust, Rust Core, dan aplikasi macOS sudah ada di repositori ini. Windows dan Linux menunggu host masing masing. Status tiap fitur tercatat di indeks itu.

## Bekerja dari satu platform

| OS | Alur yang direncanakan | Panduan untuk menyusun spec |
| --- | --- | --- |
| macOS | SwiftUI memanggil Core melalui binding Swift dari UniFFI. | [Fondasi macOS](docs/platforms/macos/README.md) |
| Windows | WinUI 3 dalam Rust memanggil Core langsung. Kelayakan `windows-reactor` dibuktikan lebih dulu di Windows. | [Fondasi Windows](docs/platforms/windows/README.md) |
| Linux | GTK 4 dan libadwaita memanggil Core langsung. Target utamanya Wayland native dengan kompatibilitas X11. | [Fondasi Linux](docs/platforms/linux/README.md) |

Dari Mac, titik awalnya adalah [scope 001, batas workspace dan pemilik](docs/scope/001%20-%20Batas%20workspace%20dan%20pemilik.md), dilanjutkan hingga [sapaan di jendela macOS](docs/scope/004%20-%20Sapaan%20di%20jendela%20macOS.md). Windows dan Linux dapat mengambil scope yang sama untuk hal bersama, lalu membuktikan integrasi pada host masing masing.

Saat satu fitur siap dirancang, spec bersama masuk ke `docs/specs/_root/`. Spec penerapan OS masuk ke `docs/specs/macos/`, `docs/specs/windows/`, atau `docs/specs/linux/` dan merujuk nomor scope yang sama. Spec yang sudah ditulis ada di folder itu.

## Rujukan bersama

| Untuk mencari | Baca |
| --- | --- |
| Pemilik tiap lapisan, arah dependensi, struktur repo yang dituju, serta batas fitur mendatang | [Arsitektur](docs/architecture/README.md) |
| Fungsi sapaan, API publik, memori, dan batas penghubung Swift | [Kontrak Rust](docs/contracts/README.md) |
| Token warna, tipografi, jarak, dan tema native | [Sistem desain](docs/design-system/README.md) |
| Konfigurasi, teks antarmuka, dan aset merek | [Resource bersama](docs/shared-resources/README.md) |
| 32 kriteria penerimaan, build per arsitektur, tes dan bukti runtime | [Penerimaan dan bukti fondasi](docs/quality/README.md) |
| Aturan pengembangan, perintah uji, dan pemeriksaan arsitektur | [Aturan pengembangan](AGENTS.md) |
| Cara memperbarui graf sebelum commit dan mengirim perubahan ke `main` | [Aturan commit dan integrasi](AGENTS.md) |
