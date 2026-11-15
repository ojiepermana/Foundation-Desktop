# Aturan commit dan integrasi

Aktifkan hook sekali di setiap clone dengan `git config --local core.hooksPath .githooks`. Hook memerlukan Python 3 dan CLI `graphify` di PATH. Untuk memasang Graphify, gunakan `uv tool install graphifyy`.

1. Sebelum commit, perbarui dokumentasi yang terdampak, stage sumber yang ingin disimpan, lalu periksa `git diff --cached`.
2. Saat `git commit`, hook memperbarui Graphify dari salinan indeks Git dan menyertakan perubahan JSON, HTML, laporan, dan manifest dalam commit yang sama. Perubahan unstaged serta riwayat kueri lokal tetap di mesin. Jika pembaruan gagal, perbaiki penyebab yang dilaporkan dan ulangi commit.
3. Pembaruan lokal membaca struktur kode serta judul dan tautan Markdown. Untuk memperbarui konsep dan hubungan semantik dalam isi dokumen, jalankan `/graphify . --update` dari salinan sumber yang akan di-commit dan stage keempat hasilnya.
4. Saat mengirim perubahan ke `main`, ambil `origin/main` terbaru, gabungkan branch kerja, lalu push ke `origin/main`. Jika Git perlu membuat commit penggabungan, gunakan `git merge --no-commit <branch>` lalu `git commit` agar hook menyertakan graf hasil penggabungan. Pastikan SHA pada remote sama dengan SHA lokal yang dikirim.
