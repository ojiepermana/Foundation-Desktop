# Resource bersama

Konfigurasi publik, teks antarmuka, dan aset merek masing masing memiliki satu sumber berversi. Spec per OS nantinya menjelaskan bagaimana aplikasi mengonsumsi sumber itu.

## Konfigurasi

`shared/config/public.json` adalah sumber nilai publik yang wajib sama di ketiga aplikasi. Skemanya tertutup dan bernomor versi, dan `shared/config/README.md` menjelaskan tiap kunci serta cara mengubahnya. Fondasi saat ini hanya butuh dua nilai, yaitu nama produk dan dasar pengenal aplikasi. Versi aplikasi tetap bersumber dari `[workspace.package]` di `Cargo.toml` root. Semua nilai dipakai saat build. Belum ada nilai yang dibaca saat aplikasi berjalan dan belum ada definisi environment; keduanya ditambah lewat kenaikan versi skema saat ada kebutuhan nyata. Pengaturan khusus OS, termasuk izin, tinggal di proyek native tiap aplikasi. Pengaturan lokal tinggal di file yang diabaikan git. Kunci privat dan kredensial penandatanganan tetap di luar repo. Layanan konfigurasi jarak jauh dan mesin feature flag ditunda. Rincian dan alasannya ada di [spec 0004](../specs/_root/0004-konfigurasi-publik/index.md).

## Lokalisasi

`shared/localization/` akan memiliki katalog istilah dan teks bersama dengan kunci stabil. Konvensi mencakup nama locale, fallback bila teks bahasa tujuan belum ada, placeholder yang harus cocok, dan deteksi entri hilang. Satu contoh teks UI cukup untuk membuktikan alurnya. Format angka, tanggal, dan bentuk jamak mengikuti OS.

## Aset merek

`shared/assets/` akan memiliki berkas sumber dan aturan penamaan, format, varian terang dan gelap, resolusi, optimasi, versi, serta tujuan ekspor. Ikon sistem boleh mengikuti OS. Aset merek yang sama diekspor dari sumbernya. Tambahan ilustrasi atau artwork menunggu kebutuhan produk.

| Target | Teks antarmuka | Tujuan aset |
| --- | --- | --- |
| macOS | Resource lokalisasi Apple | Bundle aplikasi native |
| Windows | Adapter lokalisasi Rust untuk presentasi WinUI | Resource aplikasi WinUI |
| Linux | Resource lokalisasi yang sesuai GTK | Ikon dan resource sesuai konvensi desktop Linux |

Prosedur pembuatan ulang nantinya menjelaskan sumber, pemilik, versi alat, lokasi keluaran, dan cara memeriksa hasilnya. Definisi turunan disimpan terpisah dari sumber. Detail yang khusus untuk satu host ditempatkan dalam spec OS terkait.

[Scope dan urutan kerja](../scope/scope.md) · [Kontrak teks sapaan](../contracts/README.md)
