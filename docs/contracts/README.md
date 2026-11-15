# Kontrak Rust bersama

Kontrak pertama menghubungkan aksi pengguna di jendela native dengan satu fungsi publik di Rust Core. Ini bentuk yang ditetapkan untuk fondasi dan akan menjadi dasar spec.

```rust
pub fn hello(name: String) -> String {
    format!("Hello, {name}!")
}
```

Dengan input `Ayu`, Core mengembalikan `Hello, Ayu!`. UI setiap OS mengirim nama dan menampilkan nilai yang dikembalikan. Teks antarmuka seperti label tombol bisa memakai katalog bahasa. Format hasil fungsi mengikuti kontrak di atas.

| Konsumen | Jalur pemanggilan yang direncanakan |
| --- | --- |
| macOS | View SwiftUI → state dan ViewModel Swift → adapter → binding Swift hasil UniFFI → `macos-ffi` → Core. |
| Windows | Handler kontrol WinUI 3 dalam aplikasi Rust → API publik crate Core. |
| Linux | Handler kontrol GTK dalam aplikasi Rust → API publik crate Core. |

## Kepemilikan dan kompatibilitas

Core memiliki API publik, tipe data yang melintasi batas, validasi, makna kesalahan, dan ekspektasi perilaku. Detail privat tetap di crate pemilik. Binding Swift berasal dari API yang diekspos penghubung macOS. Windows dan Linux hanya memerlukan dependensi Rust biasa untuk memanggil Core.

Saat kontrak berkembang, spec yang terkait akan mencatat input, output, nilai opsional, kesalahan bertipe, kompatibilitas dan versi. Untuk data yang melintasi UniFFI, catatan juga mencakup pemilik memori, pelepasan resource dan pemetaan kesalahan di Swift. Bentuk sapaan cukup memakai dua string tanpa model data persisten.

## Eksekusi dan uji

Core hanya memakai safe Rust, dengan dependensi minimal dan kepemilikan eksplisit. Hindari state global yang dapat berubah serta alokasi dan kloning yang tidak diperlukan. Input dari luar diperiksa sesuai kontrak. Kode `unsafe` di Core dan di jembatan macOS ditolak saat build lewat `unsafe_code = "forbid"`. Membukanya butuh pembaruan spec `_root` lebih dulu.

Uji unit dan uji API publik akan menjalankan Core secara mandiri. Uji integrasi tiap aplikasi akan membuktikan bahwa nilai yang terlihat berasal dari fungsi yang sama. Pekerjaan yang mahal kelak perlu menjaga thread UI tetap responsif. Kontrak asinkron, pembatalan, keamanan antar thread, dan akses kemampuan OS dapat didokumentasikan saat dibutuhkan tanpa menambah runtime pada contoh sapaan.

[Scope dan urutan kerja](../scope/scope.md) · [Pemilik tiap lapisan](../architecture/README.md)
