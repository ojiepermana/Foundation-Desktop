# Aturan commit dan integrasi

1. Sebelum **setiap commit**, tinjau diff dan perbarui dokumentasi yang terdampak. Jalankan `/graphify . --update` dari root repo setelah perubahan terakhir, termasuk penambahan, penggantian nama, atau penghapusan berkas. Hasil “tidak ada perubahan” memenuhi langkah ini bila graf sudah mutakhir.
2. Gunakan sumber yang akan di-commit sebagai masukan graf. Jika ada perubahan lain atau riwayat lokal di `graphify-out/memory/`, jalankan pembaruan pada salinan berkas dari indeks Git. Sertakan hasil Graphify yang berubah dalam commit yang sama, periksa `git diff --cached`, dan ulangi pembaruan jika sumber berubah lagi.
3. Saat mengirim perubahan ke `main`, ambil `origin/main` terbaru, gabungkan commit dari branch kerja ke `main`, lalu push ke `origin/main`. Jika integrasi mengubah isi sumber, perbarui graf sebelum menyelesaikan commit hasil penggabungan. Pastikan SHA pada remote sama dengan SHA lokal yang dikirim.
