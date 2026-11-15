# 0008. Aturan pengembangan dan pemeriksaan arsitektur di macOS: alasan

Catatan keputusan untuk [index.md](index.md). `/develop` tidak perlu membaca file ini.

## Context

[Spec `_root/0007`](../../_root/0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md) memasang pemeriksaan arsitektur yang berjalan di `cargo test --workspace --locked` di host mana pun. Satu hal ia serahkan ke macOS lewat Follow-up: binding Swift harus sama byte demi byte dengan keluaran generator pada setiap perubahan `crates/core/` atau `crates/macos-ffi/`, dan apakah itu menjadi uji atau tetap perintah sampai scope 012. Tabel "Kapan pemeriksaan berjalan" di spec itu juga menyerahkan perintah binding dan `swift test` ke penulis perubahan di Mac.

Dua spec macOS lain menulis Follow-up untuk scope 002:

- [Spec `macos/0005`](../0005-tema-bersama-dan-kontrol-native/index.md) ingin pemeriksaan teks AC-3 dan AC-6 (tanpa warna, bobot, atau jarak literal di kode tampilan; kontrol native tidak diganti; hanya `Presentation` yang mengimpor `DesignSystem`) menjadi pemeriksaan tetap.
- [Spec `macos/0004`](../0004-konfigurasi-publik/index.md) ingin AC-2 (tiga setelan turunan tidak kembali ke `project.pbxproj`) menjadi pemeriksaan tetap.

Keduanya belum dibangun pada 2026-11-15.

Gaya yang membentuk keputusan:

- **Generator binding butuh Mac.** Generator UniFFI membaca pustaka statis `aarch64-apple-darwin`. `swift test` dan `xcodebuild` juga hanya ada di Mac.
- **Aturan netral host.** Perintah yang butuh Mac tidak boleh masuk `cargo test --workspace --locked`, karena perintah itu harus lulus di Windows dan Linux.
- **Perintah yang ada menulis ke git.** Perintah **Membuat ulang binding** di [spec `macos/0001`](../0001-sapaan-di-jendela-macos/index.md) menghapus `bindings/swift/Sources`, menulis ulang, lalu memakai `git diff`. Itu benar sebagai cara memperbaiki, tetapi sebagai pemeriksaan ia mengubah working tree dan hanya bermakna pada tree yang bersih.
- **Satu rumah untuk scope 012.** Skrip dan CI milik spec `_root/0008` yang sedang ditulis. Spec itu butuh perintah yang sudah pasti untuk dibungkus.
- **Tanpa dependensi baru.** Alat `desktop-codegen` tidak punya pustaka regex.
- **Kriteria:** 25 (pemeriksaan kepatuhan arsitektur), 26 (`AGENTS.md`), dan 32, ditambah baris "Integrasi" tabel "Pemeriksaan yang direncanakan" (macOS menghasilkan binding Swift secara konsisten, kriteria 4).

## Options considered

### Opsi 1: Spec macOS sendiri, gerbang berupa perintah, aturan teks di sumber `architecture`

Spec ini menetapkan lima perintah gerbang Mac, pola path yang memicunya, dan empat aturan teks macOS di modul `macos.rs` sumber `architecture`.

**Pros**:

- Semua pemeriksaan macOS untuk scope 002 ada di satu tempat, dengan pemicu yang jelas untuk lead dan CI.
- Cek binding tidak menulis ke file yang dilacak.
- Aturan teks berjalan di setiap host lewat gerbang yang sudah ada.

**Cons**:

- Satu spec lagi untuk dibaca.
- Dua milestone menunggu spec lain dibangun.

### Opsi 2: Satu AC tambahan di spec `macos/0001` dan aturan baru di spec `_root/0007`

AC-14 baru di spec `macos/0001` memuat perintah cek binding. Aturan teks menjadi `A-14` sampai `A-17` di spec bersama.

**Pros**:

- Tidak ada spec baru. Perintah binding tinggal di samping perintah perbaikannya.

**Cons**:

- Spec `macos/0001` berstatus `In Progress` dan dikunci banyak AC, termasuk hitungan path. Menambah AC di sana menyentuh spec yang sedang diverifikasi.
- Aturan khusus macOS berpindah ke spec bersama, berlawanan dengan pola spec `_root/0003` yang menaruh kewajiban OS di spec OS.
- Pemicu (kapan gerbang Mac wajib) tetap tidak punya pemilik.

### Opsi 3: Uji Rust yang hanya berjalan di Mac

Uji di `desktop-macos-ffi` dengan `#[cfg(target_os = "macos")]` yang membangun pustaka dan menjalankan generator, sehingga `cargo test --workspace` di Mac ikut memeriksa binding.

**Pros**:

- Tidak ada perintah yang perlu diingat di Mac.

**Cons**:

- Uji harus memanggil `cargo build` dari dalam `cargo test`. Proses anak itu menunggu kunci build yang sedang dipegang, dan memperlambat setiap `cargo test` di Mac.
- Hasil `cargo test --workspace` berbeda antar host, padahal spec `_root/0001` menginginkan arti yang sama di semua host.
- `swift test` dan `xcodebuild` tetap perintah terpisah.

## Rationale

Opsi 1 dipilih karena gerbang Mac memang terdiri dari perintah yang hanya ada di Mac, dan yang kurang sekarang adalah urutan, pemicu, dan bentuk cek binding yang tidak menulis ke git. Ketiganya tidak cocok sebagai AC di spec `macos/0001`, yang mengunci isi jendela sapaan, dan tidak cocok sebagai uji Rust, karena uji itu memanggil `cargo` dari dalam `cargo`. Perintah `diff -r` terhadap folder di `target/` membandingkan isi, file yang hilang, dan file lebih, tanpa menyentuh `bindings/swift/`.

Aturan teks macOS masuk sumber `architecture` karena isinya hanya membaca file, sama seperti `A-10` dan `A-11`. Menaruhnya di modul milik spec macOS mengikuti pola emitter di spec `_root/0003`: alat bersama, modul per OS, ID milik spec OS. Aturan itu dipasang sesudah kode yang mereka jaga ada, karena `GreetingView.swift` di `main` masih memuat `spacing: 12` dan `padding(20)`, dan `project.pbxproj` masih memuat ketiga setelan.

## Keputusan yang diambil

Mode keputusan koordinasi adalah `auto`: opsi rekomendasi diambil dan pilihan keduanya dicatat. `berat` berarti mahal dibalik sesudah dibangun.

| No. | Pertanyaan | Diambil | Pilihan kedua | Tanda | Yang berubah bila pilihan kedua dipakai |
| --- | --- | --- | --- | --- | --- |
| 1 | Bentuk bagian macOS scope 002 | Spec sendiri | AC-14 di spec `macos/0001` dan `A-14` sampai `A-17` di spec bersama | biasa | Seluruh spec ini; dua spec lain diperbarui |
| 2 | Cara cek binding | Generator menulis ke `target/binding-check/`, lalu `diff -r` | Perintah **Membuat ulang binding** lalu `git diff --exit-code` | biasa | AC-1, AC-2, `G-2` |
| 3 | Uji atau perintah | Perintah shell | Uji Rust `cfg(target_os = "macos")` | biasa | AC-5, invarian, `G-2` |
| 4 | Isi gerbang | `Debug` saja | Tambah build `Release` | biasa | `G-4`, lama gerbang kira kira satu menit lebih |
| 5 | Kapan gerbang wajib | Bila pola path `R` cocok | Setiap perubahan | biasa | AC-4, butir `AGENTS.md` |
| 6 | Rumah aturan teks macOS | Modul `macos.rs` di sumber `architecture`, ID milik spec ini | `A-14` sampai `A-17` di spec bersama | biasa | AC-6, AC-7, langkah 3 dan 4 |
| 7 | Awalan ID | `G-` untuk gerbang, `C-` untuk aturan, `D-` dan `B-` untuk kasus | `M-` | biasa | Teks saja. `M-` sudah dipakai spec `macos/0004` |
| 8 | Format Swift | Tidak ada pemformat atau linter Swift di gerbang | `swift format lint --strict` atas `apps/macos/` | biasa | Gerbang bertambah satu langkah; semua file Swift dirapikan lebih dulu |
| 9 | Kapan `AGENTS.md` diubah | Langkah sendiri sesudah milestone 3 spec bersama | Digabung ke langkah 5 spec bersama | biasa | Milestone 2, AC-8 |
| 10 | Rujukan dan cross check | Tanpa bagian References; cross check oleh subagent baru | Rujukan sumber saja | biasa | Tidak ada |

Alasan singkat per keputusan:

1. Lihat Options considered. Lead memutuskan bila memilih pilihan kedua.
2. Lihat Rationale. Folder `target/` sudah diabaikan `.gitignore`, dan tiap worktree punya `target/` sendiri.
3. Lihat Opsi 3.
4. Gerbang dijalankan pada banyak perubahan, dan `Debug` adalah konfigurasi yang dipakai `swift test` dan pengembangan. AC-4 spec `macos/0001` tetap membuktikan `Release` saat verifikasi.
5. Perubahan yang hanya menyentuh dokumen sering terjadi di repo ini, dan tidak mengubah apa yang dibangun Xcode. Pola yang menyebut path membuat keputusan itu bisa diperiksa.
6. Lihat Rationale.
7. Awalan satu huruf menjaga bentuk pesan `<ID>: ` sama dengan spec bersama.
8. Belum ada aturan gaya Swift yang ditetapkan, dan file Swift yang ada belum pernah diperiksa dengan alat itu. Menambahkannya sekarang berarti memutuskan gaya tanpa kebutuhan.
9. `AGENTS.md` tulisan user, dan milestone 3 spec bersama juga mengubahnya. Langkah terpisah sesudahnya menghindari dua penulis di satu wave.
10. Sumber keputusan adalah spec dan kode repo yang sudah ditautkan.

## Bukti percobaan

Dijalankan pada 2026-11-15 di Mac koordinasi (Apple Silicon, macOS 27.0.1, Xcode 27.1, Rust 1.99.1) pada `main` `9b84e6c`, dengan `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, dari `target/` yang kosong. Keluaran build dihapus sesudahnya.

| Perintah | Hasil |
| --- | --- |
| `G-1` | Kode 0, sekitar 11 detik |
| Dua perintah generator `G-2` ke `target/binding-check/` | Kode 0, sekitar 34 detik termasuk membangun fitur `bindgen` |
| `diff -r bindings/swift/Sources target/binding-check` | Tidak mencetak apa pun, kode 0 |
| `git status --short` sesudah `G-2` | Kosong |
| `diff -rq` pada salinan binding dengan satu baris kosong tambahan | `Files ... desktop_macos_ffi.swift ... differ`, kode 1. Tanpa `-q` (bentuk di `G-2`) baris pertamanya `diff -r <file kiri> <file kanan>`, lalu potongan beda, kode 1; dicoba pada dua folder kecil di scratchpad |
| `diff -rq` pada salinan dengan `stale.h` tambahan dan `desktop_macos_ffi.swift` dihapus | `Only in target/binding-check/DesktopMacosFFI: desktop_macos_ffi.swift` dan `Only in target/binding-copy/desktop_macos_ffiFFI: stale.h`, kode 1 |
| `G-3` | Kode 0, `Test run with 4 tests in 1 suite passed` dan `Test run with 7 tests in 1 suite passed`, sekitar 28 detik |
| `G-4` | `** BUILD SUCCEEDED **`, sekitar 68 detik |
| `G-5` | Kosong. `git status --short --ignored` hanya menampilkan `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/` |
| Perintah AC-4 dengan pola `R` | `0` untuk empat path dokumen, `8` untuk delapan path kode dan build |
| `git grep -nE 'Command::new\("(xcodebuild\|swift\|xcrun\|cargo\|diff)"' -- '*.rs'` | Tidak mencetak apa pun. Tiga uji `*_rules.rs` memang memakai `Command`, tetapi hanya untuk bin `desktop-codegen` sendiri |
| Pola `C-1` pada `apps/macos/Presentation` dan `apps/macos/App` sekarang | Cocok di `GreetingView.swift` baris 12 (`spacing: 12`) dan 20 (`padding(20)`), jadi `C-1` baru bisa dipasang sesudah milestone 2 spec `macos/0005` |
| Tautan Markdown (cara baca `A-12`) termasuk spec ini | 0 rusak |

Percobaan memakai path pustaka relatif `target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a`, sama dengan `G-2`.

## Cross check

Dibaca subagent baru berkonteks bersih pada model `sonnet` (model lain dari penulis, `claude-opus-5-5`, dan bukan Fable), read only, pada 2026-11-15 sebelum spec ini diajukan. Subagent itu tidak menjalankan build; ia membandingkan pola dengan spec `macos/0005` karakter demi karakter, membaca "Isi file" spec `macos/0005` dan `macos/0004`, dan menjalankan `diff -r` pada folder kecil. Temuan berikut diterapkan ke `index.md`.

| Temuan | Tingkat | Perbaikan |
| --- | --- | --- |
| AC-2 (a) mengharapkan `differ`, padahal `diff -r` tanpa `-q` mencetak baris `diff -r <kiri> <kanan>` | blocker | AC-2 (a) memakai baris kepala `diff -r` yang sebenarnya |
| Komentar `AGENTS.md` usulan memuat kata `macos/0008.`, yang dibaca `A-13` sebagai path yang tidak ada | blocker | Komentar ditulis ulang tanpa titik langsung sesudah ID spec |
| Milestone 3 bergantung pada milestone 2 spec bersama, padahal milestone 3 spec itu juga mengubah `RULES` | should fix | Milestone 3 sesudah milestone 3 spec bersama |
| AC-1 spec bersama (`13 rules`) tidak lagi berlaku sesudah aturan `C` masuk | should fix | Follow-up untuk pemilik spec bersama |
| Urutan masalah tidak jelas untuk `A-1` dan `C-1` | should fix | Huruf ID menjadi kunci urutan pertama |
| `G-5` bertentangan dengan klaim gerbang bisa dijalankan di tengah pekerjaan | should fix | `G-1` sampai `G-4` boleh di tengah pekerjaan, `G-5` butuh tree bersih |
| Nama fungsi uji dan hitungan `passed` tidak disebut | should fix | `rejects_macos_violations`, `accepts_macos_shapes`, `2 passed`; angka `<r>` sebelum langkah 3 dicatat |
| Baris `G-3` di AC-3 tidak disebut persis | should fix | Dua baris `Test run with` disebut, dan `grep -c failed` mencetak `0` |
| AC-9 tanpa `wc -l`, path pendek di tabel milestone | nit | Diperbaiki |
| Pola `R` tanpa `.gitignore` dan `.cargo/`, dan `main` lokal atau `origin/main` | nit | `.gitignore` ditambah, `.cargo/` dijelaskan, `main` lokal yang baru diambil |
| `C-4` tanpa perilaku file tidak terbaca | nit | Masalah `cannot read` |
| `C-3` memakai awalan folder, berbeda dengan definisi file uji spec bersama | nit | Memakai definisi spec bersama |

Subagent juga memastikan hal berikut benar: `C-1` dan `C-2` setia pada pola `grep -E` spec `macos/0005`; aturan `C` gagal pada kode sekarang dan lulus pada kode yang akan ditulis spec `macos/0005` dan `macos/0004`; kasus `D` dan `B` memicu aturan yang dimaksud; hitungan AC-4 dan AC-8 benar; AC-5 tidak cocok dengan pemakaian `Command` yang ada; argumen `G-2` sama dengan spec `macos/0001`; hitungan 16 path AC-12 spec `macos/0001` tidak tersentuh.

## Penyelarasan sesudah diterima

Lead menerima spec ini pada 2026-11-15 (commit `a1d0e5c`) dan meminta penyelarasan teks dengan `main` `35432f4`. Keputusan dan ID AC tidak berubah.

| Yang berubah di `main` | Penyesuaian di `index.md` |
| --- | --- |
| Modul `architecture` nyata (`c3448d9`): `mod.rs`, `deps.rs`, `manifest.rs`, `code.rs`, `RULES` berisi `A-1` sampai `A-11`, dan `Findings` yang mengurutkan menurut `rule: u32` | Pendaftaran aturan `C` ditulis terhadap kode itu: `mod macos;`, pemanggilan sesudah `code::check`, entri `RULES` di akhir larik, field `letter` dan `Findings::push_rule`, kunci urutan `(letter, rule, path, line)`, serta `walk` dan `File` yang menjadi `pub(super)`. `scan_lines` tidak dipakai, karena hanya mencocokkan teks persis dan selalu mencatat huruf `'A'`. Baris milestone 3 di tabel file ikut menyebut `code.rs`. |
| Angka `rules` bergantung pada urutan build terhadap milestone 3 spec `_root/0007` | AC-6 dan AC-7 memakai angka relatif ("bertambah tepat tiga", "bertambah tepat satu") dengan batas akhir milestone spec ini. Follow-up spec bersama tidak lagi menyebut `16` dan `17`. |
| Spec `macos/0006` diterima: `LocalizedText.swift`, tombol `Button(LocalizedText.string(.greetingSubmit))`, resource bahasa di `apps/macos/Presentation/Resources/`, dan suite uji baru | Pola `C-1` dan `C-2` dicocokkan dengan isi file spec `macos/0004`, `macos/0005`, dan `macos/0006`: tidak ada yang cocok, jadi aturannya tetap lulus. `macos/0006` hanya mengubah hitungan baris tombol di AC-6 spec `macos/0005`, bukan pola yang disalin. AC-3 tidak lagi mengunci jumlah uji dan suite, dan angka `7` dan `4` hanya disebut sebagai keadaan pada akhir milestone 1. |
| Keluaran generator dan resource macOS dari spec `macos/0004`, `macos/0005`, `macos/0006`, dan spec macOS scope 011 | Satu kalimat di bagian **Kapan gerbang wajib**: semuanya sudah tercakup `apps/macos/`, `design-system/`, `shared/` (termasuk `shared/assets/`), dan `tooling/codegen/`, jadi pola `R` tidak berubah. |
| Spec macOS scope 011 kemungkinan menyentuh `project.pbxproj` | `C-4` dinyatakan hanya mencari tiga teks per baris dan tidak membaca bentuk lain berkas itu. |

Kode `macos/0006` dicocokkan dengan pola `C-1` dan `C-2` memakai skrip sementara di scratchpad sesi, pada blok **Isi file** spec itu (`git show fff3137`). Hasilnya 0 kecocokan.
