# Sistem desain bersama

Ketiga aplikasi akan memakai arti warna, tipografi, dan jarak yang sama. Bentuk kontrol, navigasi, fokus dan perilaku jendela mengikuti sistem operasi pengguna.

| Sumber yang direncanakan | Isi |
| --- | --- |
| `design-system/tokens/colors.json` | Nilai dasar merek, primer dan sekunder serta alias semantik untuk latar, permukaan, teks, garis, sukses, peringatan, dan kesalahan. Ada pemetaan terang dan gelap. |
| `typography.json`, `spacing.json` dalam folder token | Peran tipografi, bobot huruf, dan skala jarak. |
| `radius.json`, `motion.json` dalam folder token | Radius sudut dan makna gerak. |
| `design-system/schemas/` | Bentuk token yang diterima generator. |

Token dasar menyimpan nilai. Alias semantik menjelaskan peran nilai pada layar, misalnya teks utama di atas permukaan. Satu sumber berversi menghasilkan keluaran yang sama saat dibuat ulang. Perubahan dilakukan di sumber, kemudian skema dan hasil generator diperiksa.

| Target | Pemetaan ke UI native |
| --- | --- |
| macOS | Definisi Swift dipakai tema SwiftUI. Kontrol mengikuti Apple Human Interface Guidelines, penskalaan teks, fokus, dan aksesibilitas. |
| Windows | Definisi Rust atau pemetaan yang bisa diulang memakai API tema WinUI 3 yang didukung versi terpilih. Pemetaan ini tidak mengharuskan generator XAML. Tampilan mengikuti Fluent dan mendukung kontras tinggi sejauh didukung. |
| Linux | Resource atau CSS yang sesuai GTK, disertai definisi Rust jika perlu. Kontrol GTK dan libadwaita menghormati preferensi desktop, aksesibilitas dan interaksi Wayland. |

Bukti minimum kelak adalah jendela sapaan dengan warna semantik, tipografi dan jarak dari sumber bersama, diperiksa dalam tampilan terang dan gelap. Fokus keyboard, aksesibilitas dan penskalaan diperiksa melalui kontrol asli pada host target. Perpustakaan komponen lengkap berada di luar fase ini.

[Scope dan urutan kerja](../scope/scope.md)
