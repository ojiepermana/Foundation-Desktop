# Resource bersama

Konfigurasi publik, teks antarmuka, dan aset merek masing masing memiliki satu sumber berversi. Spec per OS nantinya menjelaskan bagaimana aplikasi mengonsumsi sumber itu.

## Konfigurasi

`shared/config/` akan memiliki skema untuk nilai publik yang perlu konsisten. Bedakan nilai saat build, nilai saat aplikasi berjalan, definisi environment, dan pengaturan khusus OS. Perubahan skema perlu dapat divalidasi dan ditelusuri versinya. Izin, kunci privat, dan kredensial penandatanganan diurus lewat mekanisme yang sesuai, di luar resource publik. Layanan konfigurasi jarak jauh dan mesin feature flag ditunda.

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
