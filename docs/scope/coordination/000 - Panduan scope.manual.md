# Langkah manual: `000 - Panduan scope`

Langkah yang butuh mata, tangan, akun, perangkat, atau izin kamu. Tidak ada sesi yang menjalankannya. Kirim hasilnya ke lead (ID langkah, `PASS` atau `FAIL`, dan output bila `FAIL`); lead meneruskannya ke checker yang mencentang `verify.md`. `FAIL` membuka kembali kotaknya.

Keputusan yang diambil atas namamu ada di `000 - Panduan scope.log.md` bagian "Keputusan delegasi"; baris bertanda `berat` paling perlu kamu baca. Untuk membatalkan satu baris, sebut barisnya ke lead.

## Ringkasan

| ID | Langkah | AC | Kotak yang dicentang dengan langkah ini `NOT_RUN` | Bisa dijalankan sesudah | Hasil |
| --- | --- | --- | --- | --- | --- |
| `F001-1` | Bangun dan uji Core di host Windows dan host Linux | spec `_root/0001` AC-9 | `Verify it` fitur 001 (bukti Mac). Milestone 3 dan `Build it` fitur 001 tetap terbuka sampai langkah ini `PASS` | 001 milestone 1 di `origin/main` (sesudah push penutupan) | `NOT_RUN` |

## Nilai dari user

Nilai yang hanya kamu tahu. AC di kolom ketiga `BLOCKED` sampai nilainya ada.

| ID | Nilai yang dibutuhkan | AC yang menunggu | File yang menunggu |
| --- | --- | --- | --- |
| `N1` | Lisensi paket Rust (field `license` di `[workspace.package]`). Sampai ada, `publish = false` menjaga paket tetap privat. | tidak ada; Follow-up spec `_root/0001` | `Cargo.toml` root, lewat pembaruan spec `_root/0001` |
| `N2` | Bahasa komentar kode dan rustdoc (sekarang Inggris di Core, dokumen repo Indonesia). Architect scope 002 akan mengambil rekomendasinya bila kamu tidak menyebut pilihan. | tidak ada; Follow-up spec `_root/0001` dan `_root/0002` | spec scope 002 |
| `N3` | Gaya pesan commit, awalan branch, dan preferensi gaya di luar bawaan `rustfmt` dan `clippy`. Audit tidak menebaknya dan tidak menuliskannya di `AGENTS.md`. | tidak ada | `AGENTS.md`, bila kamu ingin aturannya tertulis |

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
