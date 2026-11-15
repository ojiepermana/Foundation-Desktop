# 0005. Aset merek bersama: alasan

Catatan keputusan untuk [index.md](index.md). `/develop` tidak perlu membaca file ini.

## Context

Tiga aplikasi native di repo ini harus membawa identitas merek yang sama. Pada 2026-11-15 `main` belum punya satu pun berkas merek. Aplikasi macOS dari [spec `macos/0001`](../../macos/0001-sapaan-di-jendela-macos/index.md) berjalan tanpa ikon, dan Follow-up spec itu menyerahkan ikon aplikasi dan aset merek lain ke scope 011. Lokasi `shared/assets/` sudah dicadangkan [spec `_root/0001`](../0001-batas-workspace-dan-pemilik/index.md) untuk lapisan Resource bersama, tetapi masih rencana.

[Resource bersama](../../../shared-resources/README.md) meminta satu sumber berversi dengan aturan penamaan, format, varian terang dan gelap, resolusi, optimasi, versi, dan tujuan ekspor, serta keluaran turunan yang disimpan terpisah dari sumber. Baris **Done when** scope 011 meminta aturan itu menjelaskan bagaimana aset masuk ke bundle macOS, resource Windows, atau direktori ikon Linux. [Kriteria 21](../../../quality/README.md) meminta kepemilikan aset bersama terdokumentasi.

Gaya yang membentuk keputusan:

* **Identitas merek hanya dimiliki user.** Bentuk ikon dan warnanya tidak boleh dibuat atau ditebak oleh developer maupun agen. Saat spec ini ditulis, user belum menyerahkan berkas apa pun.
* **Tiga OS, tiga kemasan.** macOS membawa ikon di dalam bundel aplikasi, Windows di resource aplikasi WinUI, dan Linux di direktori ikon tema. Format, ukuran, dan nama keluarannya berbeda di tiap OS, dan sebagian berupa berkas biner.
* **Satu host yang tersedia.** Hanya Mac yang ada sekarang. Apa pun yang diputuskan harus bisa dibuktikan di Mac tanpa menulis spec, kode, atau bukti untuk Windows dan Linux.
* **Satu alat untuk sumber bersama.** Spec `_root/0003` (scope 008) menetapkan paket `desktop-codegen` sebagai satu satunya alat pemeriksa dan generator sumber bersama, dan meminta scope 009 sampai 011 menambah modul di sana. Alat itu hanya membaca dan menulis teks, dan dependensinya hanya `serde` dan `serde_json`.
* **Nama produk sudah punya pemilik.** [Spec `_root/0004`](../0004-konfigurasi-publik/index.md) menetapkan `build.product_name` di `shared/config/public.json` sebagai sumber nama produk, dan menyatakan aset merek hanya memegang ikon dan berkas gambar.
* **Batas fase fondasi.** Kriteria 32 melarang layanan untuk fitur mendatang, dan dokumen resource bersama menunda ilustrasi serta artwork tambahan. Alur sapaan tidak menampilkan logo di dalam jendela.
* **File root punya satu pemilik.** Spec `_root/0001` melarang perubahan pada manifest root, toolchain, dan `.gitignore` di luar pembaruan spec `_root`, dan tidak mengizinkan `.cargo/config.toml`.
* **Hook commit membaca setiap sumber baru.** Sumber yang gagal dibaca Graphify menolak commit.
* **Yang harus tercakup:** kriteria 21, baris **Done when** scope 011, dan kriteria 32 sebagai batas.

Bila keputusan ini ditunda, spec macOS untuk scope 011 akan memilih sendiri tempat ikon, lalu Windows dan Linux mengulanginya dengan cara lain, dan tidak ada yang bisa menjawab berkas mana yang asli.

## Options considered

### Opsi 1: Dokumen aturan saja

Hanya `shared/assets/README.md` berisi pemilik dan aturan. Tidak ada manifest, tidak ada berkas sumber, dan tidak ada pemeriksaan sampai user menyerahkan merek nyata.

**Pros**:

* Paling sedikit berkas, dan sudah memenuhi kata "terdokumentasi" di kriteria 21.
* Tidak ada penanda sementara yang bisa terbawa ke aplikasi, dan tidak bergantung pada spec `_root/0003`.

**Cons**:

* Aturannya tidak pernah dijalankan, jadi kesalahan aturan baru terlihat saat merek nyata tiba.
* Spec macOS tidak punya apa pun untuk diekspor, sehingga ikon aplikasi tetap menggantung.
* Sumber bersama ini menjadi satu satunya yang tidak dijaga gerbang.

### Opsi 2: Aturan, manifest, dan satu sumber penanda, diperiksa `desktop-codegen`, dengan ekspor biner oleh alat native tiap OS

`shared/assets/` memuat dokumen aturan, manifest `brand.json`, dan satu SVG penanda untuk `app-icon`. Sumber baru `assets` di `desktop-codegen` memeriksa folder, manifest, dan isi tiap SVG. Keluaran teks tiap OS ditulis alat itu; keluaran biner dibuat alat native di host OS itu dan membawa catatan tanda versi sumber.

**Pros**:

* Aturan terbukti jalan sejak hari pertama dengan satu aset, dan dijaga `cargo test --workspace` tanpa langkah tambahan.
* Mengikuti pola yang sama dengan token dan konfigurasi, tanpa paket atau dependensi baru.
* Merek nyata masuk kelak dengan mengganti isi berkas, tanpa mengubah nama atau langkah ekspor.

**Cons**:

* Build menunggu milestone 1 spec `_root/0003` dan tidak bisa paralel dengan sumber lain di alat itu.
* Ekspor biner tetap tiga langkah yang berbeda, dan satu host tidak bisa memperbarui keluaran OS lain.
* Repo membawa ikon penanda yang harus diganti sebelum aplikasi dibagikan.

### Opsi 3: Generator ekspor biner bersama

Alat bersama juga merender PNG, ICNS, dan ICO untuk ketiga OS dari host mana pun, misalnya dengan crate pengolah SVG seperti `resvg`. Keluaran diperiksa dengan membuat ulang lalu membandingkan.

**Pros**:

* Satu langkah ekspor, hasil yang sama di semua host, dan sumber yang berubah bisa langsung diekspor ke ketiga OS dari satu mesin.

**Cons**:

* Menambah dependensi pengolah gambar ke workspace untuk satu ikon, yang dibangun di setiap host. Itu lebih dari yang dibutuhkan fondasi.
* Butuh pembaruan spec `_root/0003`, karena alat itu hanya menulis teks dan aturan keluarannya meminta hasil yang sama byte demi byte di setiap host.
* Format kemasan tiap OS tetap perlu pengetahuan dan pembuktian di host OS itu.

### Opsi 4: Setiap aplikasi menyimpan asetnya sendiri

Tidak ada sumber bersama. Tiap aplikasi menyimpan ikon dalam format OS nya, dan `shared/assets/` hanya berisi pedoman merek.

**Pros**:

* Tidak ada langkah ekspor. Tiap OS memakai berkas yang paling cocok untuknya.

**Cons**:

* Tiga salinan merek yang bisa saling menyimpang, berlawanan dengan kalimat "aset merek yang sama diekspor dari sumbernya" di dokumen resource bersama.
* Tidak ada jawaban atas pertanyaan berkas mana yang asli.

## Rationale

Opsi 2 dipilih karena ia satu satunya yang memenuhi semua gaya sekaligus: kepemilikan dan aturan bisa dibuktikan sekarang di Mac, tidak ada identitas merek yang ditebak, tidak ada alat kedua, dan tidak ada dependensi baru. Satu aset penanda cukup untuk menjalankan setiap aturan dari ujung ke ujung, sesuai pendekatan Tracer Bullet proyek ini, dan memberi spec macOS sesuatu yang nyata untuk diekspor.

Opsi 1 memenuhi kata "terdokumentasi" tetapi meninggalkan aturan yang belum pernah diuji dan membuat spec macOS menunggu user. Opsi 3 adalah arah yang wajar bila kelak ada lebih banyak aset atau CI perlu mengekspor di host mana pun, tetapi untuk satu ikon biayanya tidak sebanding, kriteria 32 meminta fondasi hanya memuat yang dibutuhkan, dan alat bersama memang dibatasi pada teks. Berpindah dari opsi 2 ke opsi 3 murah: manifest dan aturan sumber tetap, hanya langkah ekspor biner yang berganti. Opsi 4 ditolak karena berlawanan langsung dengan aturan satu sumber.

Draf pertama spec ini memeriksa aturan dengan perintah `git`, `grep`, dan Python yang ditulis di `README.md`, tanpa alat, dan menyimpan nama produk di manifest. Dua hal mengubahnya sebelum spec ini diserahkan: spec `_root/0004` masuk `main` dan memegang nama produk, dan spec `_root/0003` diterima dengan aturan bahwa scope 011 menambah modul di `desktop-codegen`. Pemeriksaan silang atas draf pertama juga menunjukkan bahwa pemeriksaan tanpa alat itu rapuh. Rancangan sekarang mengikuti kedua spec itu.

Catatan `brand-export.txt` adalah harga dari membiarkan alat native membuat keluaran biner. Keluaran itu tidak bisa dibandingkan byte demi byte, jadi bukti bahwa sebuah ekspor mutakhir adalah tanda versi sumber yang ikut dicatat. Tanda itu memakai id blob git, karena git sudah ada di setiap host dan tidak ada angka yang bisa lupa dinaikkan.

## Keputusan yang diambil

Mode keputusan sesi ini `auto`: opsi yang direkomendasikan diambil dan dicatat bersama pilihan keduanya.

| No. | Pertanyaan | Diambil | Pilihan kedua | Tanda | Yang berubah bila pilihan kedua dipakai |
| --- | --- | --- | --- | --- | --- |
| 1 | Bentuk hasil spec | Aturan, manifest, satu sumber penanda, dan pemeriksaan (opsi 2) | Dokumen aturan saja (opsi 1) | biasa | AC-2 sampai AC-7 dan AC-10 hilang atau menyusut; langkah 1 hanya menulis README. |
| 2 | Format sumber | SVG saja, hanya bentuk vektor | PNG 1024 piksel sebagai induk, SVG bila ada | berat | Bagian "Format sumber" dan "Resolusi", aturan S, AC-4. Butuh pembaruan spec `_root/0003` karena alat hanya membaca teks. Bergantung pada berkas yang user punya. |
| 3 | Cara membuat keluaran biner tiap OS | Alat native di host OS itu, dipilih spec OS | Generator biner bersama (opsi 3) | biasa | Aturan E-5 dan E-6, tambahan dependensi, dan pembaruan spec `_root/0003`. |
| 4 | Nasib keluaran ekspor | Dilacak git di folder aplikasi | Dibuat saat build dan tidak dilacak | biasa | Aturan E-2 dan E-6; setiap build butuh alat ekspor. |
| 5 | Bentuk versi | `schemaVersion` untuk bentuk; versi isi adalah riwayat git dan id blob | Bilangan `version` di manifest yang dinaikkan tangan, dengan `brand_version` di catatan ekspor | berat | Kunci manifest, bagian "Versi", aturan E-6, AC-6, dan satu aturan A baru. |
| 6 | Alat pemeriksa | Sumber `assets` di `desktop-codegen` | Perintah `git`, `grep`, dan Python di `README.md`, tanpa alat | biasa | Bagian "Bentuk alat" dan "Uji kontrak" hilang; AC-3 sampai AC-5 menjadi perintah tangan; build tidak menunggu spec `_root/0003`. Berlawanan dengan arahan spec itu. |
| 7 | Cara memeriksa isi SVG | Pemeriksaan teks dengan daftar elemen yang diizinkan, tanpa dependensi | Pengurai XML sebagai dependensi baru alat | biasa | Aturan S-2 sampai S-7 menjadi pemeriksaan pohon XML; `Cargo.lock` bertambah; spec `_root/0003` diperbarui. |
| 8 | Varian terang dan gelap | `variants` bernilai `any` atau `light-dark`, menurut tampilan pemakaian | Selalu pasangan untuk setiap aset | biasa | Skema, aturan A-3, penanda menjadi dua berkas. |
| 9 | Aset awal | Hanya `app-icon`, dan wajib ada | `app-icon` dan `logo` | biasa | AC-2, isi manifest; berlawanan dengan kriteria 32 karena belum ada yang memakai logo. |
| 10 | Isi manifest | Id, `variants`, dan `placeholder`; path diturunkan | Path tiap sumber ditulis di manifest | biasa | Skema, aturan A-2 dan A-3, satu aturan baru untuk mencocokkan path dengan nama. |
| 11 | Tanda penanda sementara | `"placeholder": true` pada aset, seperti token di spec `_root/0003` | Daftar nama di tingkat atas manifest | biasa | Skema, `Brand.placeholders`, AC-10. |
| 12 | Bentuk khusus satu OS di sumber | Tidak ada; langkah ekspor OS yang menambahkannya | Varian sumber per OS | biasa | Bagian "Resolusi", skema dengan `schemaVersion` baru, aturan A-3. |
| 13 | Rumah dokumen kepemilikan | `shared/assets/README.md`, ditunjuk dari `docs/` | Seluruh aturan di `docs/shared-resources/README.md` | biasa | AC-1, AC-8, aturan A-1, dan langkah 3. |
| 14 | Bukti ekspor biner mutakhir | `brand-export.txt` berisi id blob git sumber | Berkas kunci hasil generator berisi hash yang dihitung alat | biasa | Aturan E-6, AC-6, satu keluaran baru di `Plan`, dan satu algoritme hash di alat. |

Alasan singkat tiap keputusan:

1. Lihat bagian Rationale.
2. Vektor bisa dipakai langsung oleh Linux dan dirender ke ukuran berapa pun untuk macOS dan Windows tanpa kehilangan ketajaman. SVG juga teks, jadi cocok dengan alat bersama yang hanya membaca teks. Larangan tulisan membuat hasil tidak bergantung pada font di host. Tanda `berat` karena hanya user yang tahu berkas apa yang mereka punya.
3. Lihat bagian Rationale.
4. Build di host mana pun tidak butuh alat ekspor, proyek Xcode butuh berkasnya ada, dan perubahan ikon terlihat saat review. Ini sama dengan keputusan binding Swift di spec `macos/0001` dan aturan keluaran O-6 spec `_root/0003`.
5. Spec `_root/0003` sudah memilih "versi isi adalah riwayat git" untuk token, dan nomor yang dinaikkan tangan bisa terlupa tanpa ada yang tahu. Id blob git berubah setiap kali isi berubah, tersedia di setiap host, dan tidak butuh kode. Tanda `berat` karena baris **Done when** menyebut versi, dan nomor versi yang terbaca orang adalah pilihan yang juga masuk akal.
6. Brief sesi ini dan spec `_root/0003` meminta scope 011 mengikuti bentuk alat bersama. Selain itu, pemeriksaan di gerbang `cargo test` berjalan tanpa diingat orang.
7. Alat bersama tidak punya pengurai XML, dan menambahnya berarti dependensi baru di setiap host. Pemeriksaan teks dengan daftar izin sudah menolak apa pun yang tidak dikenal. Harganya: XML yang tidak utuh tidak tertangkap di sini.
8. Satu berkas cukup untuk ikon aplikasi. Nama menurut tampilan pemakaian menghindari salah baca "ikon gelap" sebagai ikon berwarna gelap. Nilai `light-dark` tetap ada walau belum dipakai, karena baris **Done when** dan dokumen resource bersama meminta aturan varian terang dan gelap. Itu aturan, bukan layanan, jadi tidak melanggar kriteria 32.
9. Hanya ikon aplikasi yang dibutuhkan ketiga OS sekarang, dan ketiganya membutuhkannya.
10. Path yang diturunkan tidak bisa berbeda dari manifest, jadi satu kelas kesalahan hilang.
11. Mengikuti pola yang sudah ada membuat baris sukses ketiga sumber terbaca sama.
12. Satu sumber untuk semua OS adalah inti aturan. Pengecualian masuk lewat pembaruan spec bila user memang menyerahkan desain per OS.
13. Aturan tinggal di samping berkas yang diaturnya, dan `docs/` terlarang bagi sebagian besar sesi developer. Spec `_root/0004` melakukan hal yang sama untuk `shared/config/`.
14. Hash yang dihitung alat butuh algoritme di kode alat dan satu keluaran baru. Id blob git memberi hal yang sama tanpa kode.

## Bukti percobaan

Dicoba pada 2026-11-15 di Mac koordinasi (macOS 27.0.1, `arm64`, Python 3.9.6, git 2.54.0, Graphify 0.9.48), di repo git sementara di luar worktree yang berisi empat berkas dari bagian "Isi file" dan `.gitattributes` dengan dua baris baru.

Paket `desktop-codegen` masuk `main` saat spec ini hampir selesai. Modul `assets` sendiri tidak ditulis dalam Rust untuk percobaan. Yang dicoba adalah logika aturannya, lewat prototipe Python yang menjalankan aturan **A-2** dan **S-1** sampai **S-7** seperti tertulis di `index.md`:

| Yang dicoba | Hasil |
| --- | --- |
| Kasus S-1 sampai S-27 | Semuanya ditolak, dan salah satu pesannya membawa ID di kolom Aturan. Kasus S-7 ditolak **S-5** dan **S-3** |
| Kasus S-28 sampai S-31 | Semuanya diterima tanpa pesan |
| Kasus M-1 sampai M-14 | Semuanya ditolak dengan **A-2** |
| Kasus M-15 sampai M-17 | Semuanya diterima |

Aturan **A-1** dan **A-3** serta kasus F tidak diprototipekan; isinya hanya membandingkan daftar nama.

Perintah yang dicoba langsung:

| Perintah | Keluaran |
| --- | --- |
| `git ls-files shared/assets` | empat path dari AC-2 |
| `grep '^## ' shared/assets/README.md \| tr '\n' '\|'` | sepuluh heading di AC-1, berurutan |
| `sed -n '/^## Pemilik$/,/^## Struktur$/p' shared/assets/README.md \| grep -c '^\| '` | `7` |
| Dua `grep` di AC-7 | `3`, `8` |
| `grep -c '"additionalProperties": false' shared/assets/brand.schema.json` | `3` |
| `git ls-files -s -- shared/assets/source \| grep -cE '...'` dari AC-6 | `1` |
| Pemeriksaan ekspor basi dengan catatan yang baru disimpan | tidak ada keluaran, kode 0 |
| Pemeriksaan yang sama sesudah satu digit id di catatan diubah | kode 1 |
| Pemeriksaan yang sama sesudah sumber diubah dan di-stage | kode 1 |
| `test -z "$(git status --porcelain -- shared/assets)"` pada folder yang bersih | kode 0 |
| Perintah yang sama sesudah berkas baru yang belum dilacak ditaruh di `source/` | kode 1 |
| Empat perintah `awk ... \| diff` di AC-1, AC-2, dan AC-5 terhadap berkas yang sama | tidak ada keluaran, kode 0 |
| `git ls-files --eol shared/assets \| grep -c 'attr/text eol=lf'` | `3` |
| `wc -c shared/assets/source/app-icon.svg` | `334` |

Isi `brand.schema.json` di "Isi file" dicoba dengan alat yang asli: program kecil di luar repo membaca teks itu, memberikannya ke `desktop_codegen::json::to_canonical_json` dari `main`, dan hasilnya sama byte demi byte dengan teks itu. Jadi generator yang membangun nilai JSON yang sama akan menulis berkas yang sama.

Hook commit: fungsi `detect` dan `extract` Graphify 0.9.48 dijalankan pada keempat berkas itu di folder sementara. Dua berkas JSON terbaca sebagai sumber kode, `README.md` sebagai dokumen, `app-icon.svg` sebagai gambar, dan `failed_sources` kosong. Jadi commit langkah 1 tidak butuh entri `.graphifyignore`.

Di `main` pada saat yang sama, `git ls-files | grep -v '^graphify-out/' | grep -ciE '\.(svg|png|ico|icns|jpe?g|gif|webp|pdf)$'` mencetak `0`, jadi angka `1` di AC-2 hanya berasal dari penanda.

## Cross check

Spec ini dibaca ulang dua kali oleh model lain (Claude Opus) tanpa hak tulis. Mode keputusan `auto`, jadi perbaikan yang direkomendasikan diterapkan.

Putaran pertama membaca draf pertama, yang memeriksa aturan dengan perintah `git`, `grep`, dan Python di `README.md`. Temuannya: pemeriksaan versi gagal pada commit merge, pemeriksaan isi SVG dengan `grep` bisa dilewati enam cara, larangan `DOCTYPE` diperiksa sesudah berkas diurai, dan beberapa AC tidak punya perintah. Draf itu lalu diganti rancangan sekarang (lihat bagian Rationale), yang membuang mekanisme yang bermasalah: nomor versi tulis tangan, pemeriksaan lewat `grep`, dan pengurai XML.

Putaran kedua membaca rancangan sekarang, menulis ulang aturan S sebagai skrip, dan menjalankan setiap kasus. Semua kasus berperilaku seperti tabelnya, isi skema sama dengan keluaran `to_canonical_json` menurut uraian spec `_root/0003`, permukaan Rust cocok dengan `Source`, `Plan`, `Problem`, `check`, `generate`, dan `UniqueMap` di spec itu, dan tidak ada pertentangan dengan spec `_root/0001`, `_root/0003`, atau `_root/0004`. Sesudah putaran ini `main` memuat alat yang sudah dibangun, dan tanda tangan di "Bentuk alat" dicocokkan dengan `tooling/codegen/src/lib.rs` dan `json.rs` yang nyata. Temuannya dan tindak lanjutnya:

| Temuan | Tindak lanjut |
| --- | --- |
| `<rect/onload="..."` lolos, karena S-3 menerima `/` sesudah nama dan S-5 hanya melihat `on` sesudah spasi putih. `HREF=` dan `xml:base` juga lolos | Diperbaiki. S-3 hanya menerima `/>`, S-5 melihat karakter sebelum `on`, S-6 tidak membedakan huruf besar kecil dan menolak `xml:base`. Kasus S-7, S-10, dan S-11 ditambahkan. |
| Deklarasi `<?xml ?>` tidak didefinisikan persis, jadi `<?xml-stylesheet ...?>` di awal berkas bisa ikut dibuang | Diperbaiki di S-2, dengan kasus S-17. |
| `brand-export.txt` dengan akhir baris CRLF membuat pemeriksaan ekspor basi selalu gagal | Diperbaiki di E-6, "Kontrak untuk spec OS", dan kasus gagal. |
| Syarat `git diff --quiet` tidak melihat berkas baru yang belum dilacak | Diperbaiki. E-6 memakai `git status --porcelain -- shared/assets`. |
| Kata "persis" di AC-1, AC-2, dan AC-5 tidak punya perintah | Diperbaiki dengan empat perintah `awk ... \| diff`. |
| Jalur gagal `plan` belum diputuskan: folder hilang, jenis entri salah, berkas tidak terbaca | Diperbaiki dengan tabel "Jalur gagal `plan`" dan kasus F-6 dan F-7. Ekspor ulang `check_svg`, nama `serde` untuk `Variants`, dan teks usage juga ditulis. |
| Contoh ` on = ` di kasus gagal tidak ditolak S-5 | Diperbaiki menjadi ` onclick=`. |
| Dua kalimat "Format sumber" tidak berlaku untuk keluaran teks | Diperbaiki di `README.md`, dan penempatan penanda O-1 sesudah deklarasi ditulis di "Kontrak untuk spec OS". |
| A-1 membaca disk, jadi `Thumbs.db` dan `desktop.ini` di Windows akan menggagalkan gerbang | Diperbaiki. Keduanya diabaikan seperti `.DS_Store`. |
| Langkah 1 Build plan meminta AC-9 sebelum commit ada, dan `--format=` mencetak baris kosong | Diperbaiki. |
| Bagian **Done when** tentang cara aset masuk ke tiap OS hanya dipenuhi sebagai aturan | Ditulis terang di bawah daftar AC: mekanisme dan bukti per OS adalah kewajiban spec OS. |
| Catatan ekspor menandai semua OS basi untuk perubahan apa pun, dan tidak menangkap keluaran biner yang disunting tangan | Dicatat di Consequences dan kasus gagal. |
| Pengurai HTML membaca komentar berbeda dari XML | Dicatat di bagian Keamanan. Tidak ada aturan baru, karena alat ekspor dan aplikasi native membaca SVG sebagai XML. |
