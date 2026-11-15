# 0007. Aset merek bersama di macOS: alasan

Catatan keputusan untuk [index.md](index.md). `/develop` tidak perlu membaca file ini.

## Context

Spec [`_root/0005`](../../_root/0005-aset-merek-bersama/index.md) sudah dibangun: `shared/assets/` memuat aturan, manifest `brand.json`, skema hasil generator, dan satu SVG penanda untuk `app-icon`, dan sumber `assets` di `desktop-codegen` memeriksa semuanya. Tabel tujuan ekspor spec itu menetapkan folder keluaran macOS `apps/macos/Resources/` dan tujuan akhirnya bundel aplikasi native. Pada 2026-11-15 aplikasi Mac dari spec `macos/0001` masih berjalan dengan ikon kosong bawaan sistem, dan folder `Resources` belum ada.

Gaya yang membentuk keputusan:

* **Tanpa unduhan.** Aturan koordinasi melarang unduhan di luar dependensi Cargo dan SwiftPM yang spec sebut. Alat yang boleh dipakai adalah yang sudah ada di Mac dengan Xcode 27.1 dan macOS 27.0.1.
* **Alat bersama hanya menulis teks.** Spec `_root/0003` menjaga `desktop-codegen` dengan dua dependensi, `serde` dan `serde_json`, dan `Output` berisi teks. Menambah pengolah gambar berarti pembaruan spec itu.
* **Satu sumber, tanpa salinan tulis tangan.** Aturan **E-1** sampai **E-3** spec `_root/0005` melarang salinan merek yang disunting tangan. Keluaran biner wajib membawa catatan `brand-export.txt` (**E-6**), yang hanya bisa diperbarui di host OS itu.
* **Ikon masih penanda.** Spec ini harus bisa dibangun dan diverifikasi dengan penanda, dan tidak boleh berubah bentuk saat ikon nyata masuk.
* **Berkas bersama satu penulis per wave.** `project.pbxproj`, `.gitattributes`, `tooling/codegen/src/lib.rs`, dan `apps/macos/Package.swift` disentuh banyak spec macOS di wave ini (`macos/0004`, `macos/0005`, `macos/0006`). Perubahan yang sedikit dan tidak saling bergantung lebih murah digabung.
* **Spec `macos/0001` mengunci tata letak.** AC-12 nya mengunci tepat 16 path dan melarang folder `Resources`, jadi setiap spec macOS di wave ini menulis perubahan untuk spec itu sebagai Follow-up.
* **Yang harus tercakup:** baris macOS tabel tujuan ekspor dan aturan **E-1** sampai **E-8** spec `_root/0005`, bagian macOS dari baris **Done when** scope 011, kriteria 21, dan kriteria 32 sebagai batas.

## Options considered

### Opsi 1: Dokumen Icon Composer `.icon` yang ditulis `desktop-codegen`, dikompilasi `actool` saat build

Xcode 26 ke atas menerima dokumen Icon Composer sebagai ikon aplikasi: folder `AppIcon.icon` berisi `icon.json` dan lapisan gambar, yang boleh berupa SVG. Emitter di sumber `assets` menulis kedua berkas teks itu, dan build Xcode memanggil `actool` untuk menghasilkan `AppIcon.icns` dan `Assets.car` di bundel.

**Pros**:

* Semua yang dilacak git berupa teks, jadi `assets check` membandingkannya byte demi byte dan tidak ada catatan ekspor yang bisa basi.
* Tidak ada dependensi baru, tidak ada unduhan, dan tidak ada langkah ekspor yang dijalankan orang.
* `actool` juga menulis `AppIcon.icns` untuk macOS yang lebih lama, dan memetakan tampilan gelap lewat `image-name-specializations`.

**Cons**:

* Butuh Xcode 26 atau lebih baru untuk build.
* Raster di bundel ditentukan versi `actool`, dan format `icon.json` bukan skema publik.
* XML yang tidak utuh baru ketahuan saat build.

### Opsi 2: Katalog aset `AppIcon.appiconset` berisi PNG yang dirender alat native dan dilacak git

Swift script dengan AppKit, `qlmanage`, atau `sips` merender SVG ke PNG di sepuluh ukuran macOS, lalu PNG dan `Contents.json` dilacak git dengan `brand-export.txt` menurut **E-6**.

**Pros**:

* Format katalog aset yang lama dan dikenal semua versi Xcode.
* Raster yang masuk bundel terlihat di repo.

**Cons**:

* Sepuluh berkas biner dan satu catatan ekspor di repo, yang basi setiap kali sumber berubah sampai seseorang mengulang ekspor di Mac.
* Butuh skrip render baru atau langkah manual, padahal skrip milik scope 012. Hasil `qlmanage` dan `sips` untuk SVG tidak dijamin sama antar versi macOS.
* Uji aturan keluaran tidak bisa memeriksa penanda dan `eol=lf` pada PNG, jadi butuh pengecualian.

### Opsi 3: Generator raster di `desktop-codegen`

Alat bersama mendapat pengolah SVG, misalnya `resvg`, dan menulis PNG atau ICNS dari host mana pun.

**Pros**:

* Satu langkah ekspor untuk ketiga OS, bisa dijalankan CI di host mana pun.

**Cons**:

* Dependensi baru dan keluaran biner, dua hal yang dilarang spec `_root/0003` tanpa pembaruan lewat lead. Untuk satu ikon, biayanya jauh di atas kebutuhan fondasi.

### Opsi 4: Ikon ditulis tangan di proyek Xcode

Desainer menaruh ikon langsung di proyek dengan editor Xcode atau Icon Composer, tanpa generator.

**Pros**:

* Paling sedikit kode.

**Cons**:

* Salinan merek tulis tangan, berlawanan langsung dengan **E-1** sampai **E-3**, dan tidak ada yang bisa membuktikan bahwa ikon di Mac sama dengan sumber bersama.

## Rationale

Opsi 1 dipilih karena ia satu satunya yang memenuhi ketiga gaya utama sekaligus: tanpa unduhan, tanpa dependensi baru di alat bersama, dan tanpa salinan atau berkas biner yang bisa basi. Percobaan di salinan `main` membuktikan bahwa `actool` dari Xcode 27.1 menerima SVG penanda apa adanya sebagai lapisan, dan bahwa build Debug dan Release memasukkan ikon ke bundel. Kesalahan yang tidak tertangkap aturan teks spec `_root/0005`, seperti XML yang tidak utuh, membuat build gagal, jadi tidak ada ikon rusak yang diam diam masuk bundel.

Opsi 2 adalah pilihan kedua yang wajar, terutama bila kelak aplikasi harus dibangun dengan Xcode yang lebih lama. Harganya sepuluh berkas biner, catatan ekspor, dan skrip render yang belum ada. Opsi 3 butuh pembaruan spec `_root/0003` dan menambah dependensi pengolah gambar untuk satu ikon. Opsi 4 ditolak karena melanggar aturan satu sumber.

Keuntungan sampingan: grup folder tersinkron `Resources` membuat resource macOS berikutnya tidak perlu mengubah `project.pbxproj`, dan emitter tinggal di dalam modul `assets`, jadi `tooling/codegen/src/lib.rs` tidak disentuh.

## Keputusan yang diambil

Mode keputusan sesi ini `auto`: opsi yang direkomendasikan diambil dan dicatat bersama pilihan keduanya.

| No. | Pertanyaan | Diambil | Pilihan kedua | Tanda | Yang berubah bila pilihan kedua dipakai |
| --- | --- | --- | --- | --- | --- |
| 1 | Format ikon macOS di repo | Dokumen Icon Composer `.icon` berisi teks, dikompilasi `actool` saat build (opsi 1) | `AppIcon.appiconset` berisi PNG yang dilacak dengan `brand-export.txt` (opsi 2) | biasa | Semua AC kecuali AC-7 sampai AC-10; ditambah skrip render, catatan ekspor, dan pengecualian di uji aturan keluaran. |
| 2 | Penulis keluaran | Emitter `macos` di sumber `assets` (**E-4**) | Berkas ditulis tangan dan dibandingkan uji | biasa | "Emitter macOS", AC-1, AC-3, AC-5. |
| 3 | Lokasi | `apps/macos/Resources/AppIcon.icon/` | `apps/macos/App/AppIcon.icon/`, di grup tersinkron yang sudah ada | biasa | AC-2, AC-6 (hanya dua baris `project.pbxproj`), dan pembaruan baris macOS tabel tujuan ekspor spec `_root/0005`. |
| 4 | Cara mendaftarkan ke Xcode | Grup folder tersinkron `Resources` | `PBXFileReference` dan `PBXBuildFile` di fase Resources | biasa | "Perubahan `project.pbxproj`", AC-6; berkas baru di `Resources/` perlu didaftarkan satu per satu. |
| 5 | Penanda di salinan SVG | Komentar XML di baris pertama tanpa `--` | Tanpa penanda, dengan pengecualian O-1 | biasa | "Emitter macOS", AC-9, "Uji aturan keluaran". |
| 6 | Pemetaan `light-dark` | Satu lapisan dengan `image-name-specializations` untuk `dark` | Belum dipetakan sampai ada pasangan | biasa | Uji `light_dark_pair_uses_the_dark_appearance` dan blok JSON kedua. |
| 7 | Platform di `icon.json` | `supported-platforms` berisi `squares: ["macOS"]` | Kunci itu tidak ditulis | biasa | Isi `icon.json` di "Isi file" dan AC-1. |
| 8 | Aset yang diekspor | Hanya `app-icon` | Semua aset di `brand.json` ke katalog aset | biasa | "Emitter macOS" dan uji kontrak. |
| 9 | Berkas biner | Tidak ada di repo; **E-6** tidak dipakai di macOS | `AppIcon.icns` hasil `actool` dilacak dengan `brand-export.txt` | biasa | AC-2, AC-4; satu catatan ekspor dan pemeriksaan basi. |
| 10 | Bukti di bundel | `test -s`, `PlistBuddy`, dan `iconutil` atas bundel hasil build | `assetutil --info` atas `Assets.car` | biasa | AC-4. |
| 11 | ID objek Xcode | `D00000000000000000000020` | ID berikutnya, `D00000000000000000000015` | biasa | "Perubahan `project.pbxproj`", AC-6. Dipilih jauh dari `D…14` milik spec `macos/0004` supaya spec lain di wave ini punya ruang. |
| 12 | Berkas basi di `AppIcon.icon` | Aturan **I-1** di `plan` | Hanya invarian dan **AC-2** | biasa | "Emitter macOS", uji `stale_icon_files_are_reported`, AC-3; berkas basi baru ketahuan saat review. |

Alasan singkat tiap keputusan:

1. Lihat bagian Rationale.
2. Itu pola emitter OS dari spec `_root/0003` (K-1) dan aturan **E-4**.
3. Spec `_root/0005` sudah menetapkan `apps/macos/Resources/` sebagai folder keluaran macOS, dan rujukan macOS memisahkan `App` (siklus hidup) dari `Resources`.
4. Proyek sudah memakai format 77 dengan grup tersinkron untuk `App`. Percobaan menunjukkan grup tersinkron mendaftarkan `.icon` ke fase Resources dengan benar.
5. `actool` menolak SVG yang komentarnya memuat `--` (pesan `Double hyphen within comment`), jadi perintah `cargo run ... --locked -- ...` tidak boleh ada di penanda SVG. Penanda tetap memuat dua teks yang dituntut O-1.
6. Mekanisme tampilan gelap native dari Icon Composer, dan sudah dicoba dengan `actool`. Ditulis sekarang supaya penggantian ke pasangan tidak butuh spec baru.
7. Membatasi keluaran `actool` ke macOS. Tanpa kunci itu `actool` juga lulus, tetapi niatnya kurang jelas.
8. Hanya ikon aplikasi yang dibutuhkan Mac sekarang (kriteria 32).
9. Keluaran biner di repo hanya menambah sesuatu yang bisa basi, padahal `actool` membuatnya ulang setiap build.
10. Ketiga alat ada di setiap Mac dan keluarannya mudah dibaca. `assetutil` mencetak JSON panjang yang bergantung pada versi.
11. Lihat kolom terakhir.
12. Tanpa aturan ini, berkas lama di `Assets/` tetap ikut masuk dokumen ikon dan tidak ada yang menangkapnya, karena `check` hanya mengenal keluaran di `Plan`. Membaca satu folder aplikasi dalam `plan` lebih murah daripada mengandalkan review.

## Bukti percobaan

Dicoba pada 2026-11-15 di Mac koordinasi (macOS 27.0.1, `arm64`, Xcode 27.1 build `27A9275`, SDK `27.0`, `actool` `27.1` build `25098`). Salinan `main` pada `9b84e6c` dibuat dengan `git archive` di folder sementara di luar worktree, lalu diberi dua keluaran dan sembilan baris `project.pbxproj` persis seperti di `index.md`.

| Percobaan | Hasil |
| --- | --- |
| `actool --compile` atas `AppIcon.icon` dengan `icon.json` kanonik berkunci `$comment` dan salinan SVG bertanda | kode 0, menulis `AppIcon.icns` (30184 byte) dan `Assets.car`, tanpa peringatan; `partial.plist` memuat `CFBundleIconFile` dan `CFBundleIconName` bernilai `AppIcon` |
| Sama, tanpa kunci `supported-platforms` | kode 0, keluaran sama |
| Blok `light-dark` dari `index.md` dengan dua salinan SVG | kode 0, `AppIcon.icns` dan `Assets.car`, tanpa peringatan |
| Komentar SVG yang memuat `--` | gagal: `Double hyphen within comment`, tanpa `AppIcon.icns` |
| Kunci `image-name` salah ketik | gagal: `The layer “app-icon” does not have an image name.` |
| `icon.json` menunjuk berkas yang tidak ada | gagal: `references an image named “app-icon.svg” that does not exist` |
| SVG yang terpotong | gagal: `Premature end of data in tag svg` |
| `xcodebuild ... -configuration Debug ... build` di salinan `main` | `** BUILD SUCCEEDED **`; bundel memuat `AppIcon.icns` dan `Assets.car`; `PlistBuddy` mencetak dua baris `AppIcon`; `iconutil -c iconset` lulus dan memuat `icon_16x16.png`; `git status --short` kosong |
| Sama, `-configuration Release` | `** BUILD SUCCEEDED **`, isi bundel sama, `lipo -archs` mencetak `arm64` |
| Build Debug dengan salinan SVG yang komentarnya memuat `--` | `** BUILD FAILED **`, kode 65, `AppIcon.icon: error: Exception Raised during asset import` |
| Build Debug dengan `icon.json` yang menunjuk `nope.svg` | `** BUILD FAILED **`, kode 65 |
| `to_canonical_json` atas blok `icon.json` | sama dengan keluaran `json.dumps(..., indent=2, sort_keys=True)` ditambah newline, bentuk yang sama yang sudah dicocokkan dengan alat asli untuk skema spec `_root/0005` |

Kode Rust emitter belum ditulis. Isi dua keluaran di `index.md` dibuat dengan aturan "Emitter macOS" lewat skrip kecil, lalu dibandingkan byte demi byte dengan berkas yang lulus kedua build di atas.

Peringatan yang muncul di log build hanya `Metadata extraction skipped, no AppIntents.framework dependency found`, yang sudah ada sebelum spec ini.

## Cross check

Spec ini dibaca ulang oleh subagent baru tanpa hak tulis pada Claude Sonnet, berbeda dari model penulis (Claude Opus). Ia mengekstrak kedua keluaran dengan perintah `awk` dari AC-1, menerapkan perubahan `project.pbxproj` ke salinan, menjalankan build Debug sendiri di salinan `git archive` (`** BUILD SUCCEEDED **`, dua kunci `AppIcon` di `Info.plist`), dan mencocokkan `.gitattributes` dengan logika `config_rules.rs`. Tidak ada BUG. Mode keputusan `auto`, jadi perbaikan yang direkomendasikan diterapkan:

| Temuan | Tindak lanjut |
| --- | --- |
| Follow-up untuk spec `macos/0001` melewatkan baris **Decision** "Target aplikasi `DesktopMac` hanya berisi folder `App/`" | Ditambahkan sebagai butir 7, beserta kalimat gabungan AC-12 untuk lead (butir 9). |
| Follow-up untuk spec `_root/0005` hanya menyebut AC-5, padahal klaim di AC-2 nya tentang berkas gambar ikut berubah | Ditambahkan. AC-2 spec ini menjelaskan mengapa pola biner tidak memuat `svg`. |
| Penyimpangan dari **E-8** (daftar ukuran) tidak tertulis terang | Baris "Daftar keluaran menurut **E-8**" ditambahkan di **Decision**. |
| `svg_copy` meninggalkan kasus tepi (`?>` yang hilang, spasi sesudah `?>`) | Ditulis sebagai tiga langkah dengan dua contoh. |
| `outputs` dan `plan` tidak menyebut perilaku saat panjang isi berbeda atau aset tidak ditemukan; `missing_docs` | Ditulis: `assert_eq!`, `.expect`, dan komentar `//!` di `macos.rs`. |
| Berkas basi di `AppIcon.icon` hanya dijaga invarian | Aturan **I-1** dan uji keenam ditambahkan (keputusan 12). |
| `assets_rules.rs` belum ada di `main`, jadi AC-3 tidak pasti | Sesudah merge `main` berkas itu ada dan sudah menerima `apps/`. AC-3 sekarang pasti, dan spec ini tidak mengubah berkas itu. |
| AC-2 memakai `git ls-files` di `HEAD`, yang berubah sesudah merge `main` | AC-2 memakai `git ls-tree` atas `$B` dan `$C`. |
| Detail uji 2, 3, dan 4 kurang persis; AC yang hanya bisa jalan sesudah commit tercatat di langkah 1 | Diperbaiki di tabel uji dan di Build plan. |
| **E-7** tidak punya AC | Ditambahkan ke AC-6 (`INFOPLIST_KEY_` tidak ditambah langkah 1). |
