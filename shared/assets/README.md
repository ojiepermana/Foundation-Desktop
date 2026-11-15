# Aset merek bersama

Folder ini adalah satu satunya sumber aset merek untuk aplikasi macOS, Windows, dan Linux. `brand.schema.json` dibuat generator untuk editor; jangan disunting dengan tangan. Aturan lengkap dan alasannya ada di [spec 0005](../../docs/specs/_root/0005-aset-merek-bersama/index.md). Bila aturan perlu berubah, perbarui spec itu lebih dulu, lalu alat pemeriksa dan berkas ini.

## Pemilik

| Hal | Pemilik | Cara berubah |
| --- | --- | --- |
| Identitas merek: bentuk ikon dan logo serta warna di dalam berkas | User sebagai pemilik produk | Hanya user yang menyerahkan atau menyetujui berkas sumber. |
| Folder `shared/assets/`: berkas ini, `brand.json`, dan `source/` | Resource bersama | Lewat langkah di bagian "Mengganti atau menambah aset". |
| `brand.schema.json` dan aturan pemeriksaan di `tooling/codegen/src/assets/` | Tooling bersama | Lewat pembaruan spec 0005, lalu `assets generate`. |
| Keluaran ekspor di folder aplikasi tiap OS | Aplikasi OS itu | Hanya lewat ekspor ulang dari sumber. Tidak disunting tangan. |
| Ikon sistem bawaan OS, nama produk, dan warna merek sebagai token tema | OS masing masing, konfigurasi publik (`shared/config/`), dan sistem desain (`design-system/tokens/`) | Tidak disimpan di sini. |

## Struktur

| Path | Isi |
| --- | --- |
| `README.md` | Aturan ini. |
| `brand.json` | Manifest (daftar isi yang bisa dibaca program): aset apa saja yang ada, variannya, dan mana yang masih penanda sementara. |
| `brand.schema.json` | Skema manifest untuk editor. Keluaran generator. |
| `source/` | Berkas sumber. Satu berkas SVG per aset dan per tampilan. |

Folder ini tertutup: berkas atau subfolder lain ditolak pemeriksaan, kecuali berkas sistem `.DS_Store`, `Thumbs.db`, dan `desktop.ini` yang diabaikan. Keluaran ekspor tidak pernah disimpan di sini.

Isi `brand.json`:

| Kunci | Tipe | Wajib | Aturan |
| --- | --- | --- | --- |
| `$schema` | teks | ya | Tepat `./brand.schema.json`. |
| `schemaVersion` | bilangan bulat | ya | Tepat `1`. Naik satu setiap kali bentuk manifest berubah. |
| `assets` | objek | ya | Satu entri per aset, dengan id aset sebagai kunci. Entri `app-icon` wajib ada. |
| `variants` di tiap aset | teks | ya | `any` atau `light-dark`. Lihat bagian "Varian terang dan gelap". |
| `placeholder` di tiap aset | boolean | tidak | `true` selama berkas sumbernya masih penanda sementara. Bila tidak ditulis, artinya `false`. |

Manifest tidak menyimpan path berkas. Path sumber diturunkan dari id dan varian, jadi nama berkas dan manifest tidak bisa berbeda.

## Format sumber

* Setiap sumber adalah berkas SVG (gambar vektor berbasis teks): teks UTF-8 tanpa BOM, akhir baris LF, paling besar 262144 byte (256 KiB).
* Elemen pertama adalah `svg` dengan `xmlns="http://www.w3.org/2000/svg"` dan `viewBox="0 0 <lebar> <tinggi>"`.
* Isinya hanya bentuk vektor. Elemen yang diizinkan: `svg`, `g`, `path`, `rect`, `circle`, `ellipse`, `line`, `polyline`, `polygon`, `defs`, `linearGradient`, `radialGradient`, `stop`, `clipPath`, `mask`, `use`, `symbol`, `title`, dan `desc`. Elemen lain ditolak, termasuk `text`, `image`, `style`, `script`, `foreignObject`, `filter`, elemen animasi, dan elemen berawalan namespace.
* Tulisan diubah menjadi path supaya hasilnya tidak bergantung pada font yang terpasang. Gaya ditulis sebagai atribut.
* Atribut peristiwa seperti `onload` dan atribut `xml:base` dilarang. `href` dan `url(...)` hanya boleh menunjuk id di berkas yang sama, misalnya `href="#id"` dan `url(#id)`.
* `DOCTYPE`, `ENTITY`, bagian `CDATA`, dan instruksi pemrosesan dilarang. Deklarasi `<?xml ... ?>` di awal berkas dan komentar boleh. Entitas yang boleh hanya `&amp;`, `&lt;`, `&gt;`, `&quot;`, dan `&apos;`.
* Sumber disimpan seperti yang diserahkan pemilik merek, selama memenuhi aturan di atas. Langkah ekspor boleh mengecilkan ukuran keluarannya selama tampilannya tidak berubah.

Pemeriksaan membaca berkas sebagai teks dan tidak mengurai XML. Berkas yang lolos tetapi bukan XML yang utuh baru ketahuan saat alat ekspor atau aplikasi membacanya.

## Penamaan

* Id aset memakai huruf kecil ASCII, angka, dan tanda hubung, diawali huruf, dan tidak diakhiri `-light` atau `-dark`. Contoh: `app-icon`.
* Aset dengan varian `any` punya satu berkas, `source/<id>.svg`. Aset dengan varian `light-dark` punya dua berkas, `source/<id>-light.svg` dan `source/<id>-dark.svg`.
* Nama sumber tidak memuat versi, ukuran, atau nama OS. Nama itu tetap sama saat isinya diganti.
* Nama keluaran ekspor mengikuti konvensi OS tujuan dan ditetapkan spec OS itu.

## Varian terang dan gelap

* `light` dipakai saat OS berada di tampilan terang, `dark` saat tampilan gelap. Nama mengikuti tampilan tempat aset dipakai, bukan warna aset itu sendiri.
* `any` berarti satu berkas untuk kedua tampilan. Berkas itu harus tetap terbaca di latar terang maupun gelap.
* `app-icon` memakai `any` sampai pemilik merek menyerahkan pasangan.
* Spec OS memetakan pasangan ke mekanisme tampilan native OS itu.

## Resolusi

* Sumber berupa vektor, jadi tidak punya resolusi. `app-icon` memakai kanvas persegi dengan `viewBox="0 0 1024 1024"`.
* Setiap gambar raster (gambar berbasis piksel) dirender langsung dari sumber vektor pada ukuran piksel tujuannya. Raster tidak pernah dibuat dengan memperbesar raster lain.
* Daftar ukuran dan skala mengikuti konvensi OS tujuan dan ditetapkan spec OS itu.
* Sumber tidak memuat bentuk yang hanya berlaku di satu OS, seperti topeng sudut atau bayangan. Bila pedoman satu OS memintanya, langkah ekspor OS itu yang menambahkannya.

## Versi

* Versi bentuk manifest adalah `schemaVersion`.
* Versi isi adalah riwayat git folder ini. Tidak ada nomor versi yang dinaikkan dengan tangan, dan versi lama tidak disimpan sebagai berkas terpisah.
* Tanda versi sebuah sumber adalah id blob git nya, yang dicetak perintah ini:

  ```bash
  git ls-files -s -- shared/assets/source
  ```

* Keluaran ekspor berupa teks dibandingkan dengan sumbernya oleh `assets check`, jadi tidak bisa tertinggal.
* Keluaran ekspor biner membawa catatan ekspor berisi tanda versi semua sumber pada saat ekspor. Lihat aturan **E-6**. Perubahan pada sumber mana pun membuat setiap ekspor biner dianggap basi.
* `placeholder` menandai aset yang masih penanda sementara. Selama masih ada aset bertanda, aplikasi belum boleh dibagikan ke luar tim.

## Tujuan ekspor

| OS | Folder keluaran di repo | Tujuan akhir | Dirinci di |
| --- | --- | --- | --- |
| macOS | `apps/macos/Resources/` | Bundel aplikasi native | Spec macOS untuk scope 011 |
| Windows | Folder `resources` di `apps/windows/` | Resource aplikasi WinUI | Spec Windows untuk scope 011, menunggu host Windows |
| Linux | Folder `resources` di `apps/linux/` | Direktori ikon tema `hicolor` dan resource sesuai konvensi desktop Linux | Spec Linux untuk scope 011, menunggu host Linux |

Aturan yang berlaku di ketiga OS:

* **E-1** Ekspor hanya membaca berkas di `shared/assets/source/` yang tercatat di `brand.json`. Tidak ada salinan tulis tangan sumber merek di tempat lain.
* **E-2** Keluaran ekspor tinggal di folder aplikasi OS itu sesuai tabel di atas, dan dilacak git.
* **E-3** Keluaran ekspor tidak disunting tangan. Perubahan dilakukan pada sumber, lalu ekspor diulang.
* **E-4** Keluaran berupa teks, misalnya salinan SVG, ditulis alat `desktop-codegen` lewat sumber `assets`, sehingga `assets generate` menulisnya dan `assets check` membandingkannya.
* **E-5** Keluaran biner, misalnya PNG, ICNS, dan ICO, dibuat alat native di host OS itu. Alat itu dipilih spec OS. `desktop-codegen` tidak menulis berkas biner.
* **E-6** Folder yang memuat keluaran biner juga memuat catatan ekspor bernama `brand-export.txt`. Isinya keluaran `git ls-files -s -- shared/assets/source` pada saat ekspor, dengan akhir baris LF. Ekspor hanya dijalankan saat `git status --porcelain -- shared/assets` tidak mencetak apa pun, yaitu saat semua perubahan di folder ini sudah masuk commit.
* **E-7** Ikon sistem bawaan OS dan nama produk bukan aset merek. Nama produk bersumber dari `build.product_name` di `shared/config/public.json`.
* **E-8** Spec OS menulis perintah ekspor ulang, alat dan versinya, daftar keluaran beserta nama, format, dan ukurannya, serta cara memeriksa hasilnya di host itu.

Perintah berikut memeriksa apakah sebuah ekspor biner basi. Ganti `<folder>` dengan folder keluaran. Kode keluar 0 dan tanpa keluaran berarti ekspor dibuat dari sumber yang sekarang. Kode 1 berarti sumber sudah berubah dan ekspor perlu diulang.

```bash
git ls-files -s -- shared/assets/source | diff - <folder>/brand-export.txt
```

## Pemeriksaan

Jalankan dari akar repo. Perintah pertama memeriksa folder ini, manifest, dan setiap sumber, lalu membandingkan setiap keluaran. Perintah kedua menulis ulang keluaran yang hilang atau berbeda.

```bash
cargo run -p desktop-codegen --locked -- assets check
cargo run -p desktop-codegen --locked -- assets generate
```

`cargo test --workspace --locked` menjalankan pemeriksaan yang sama, jadi sumber yang tidak sah atau keluaran yang menyimpang menggagalkan gerbang.

## Mengganti atau menambah aset

1. Terima berkas dari pemilik merek. Jangan menggambar atau menebak aset merek sendiri.
2. Simpan berkas di `source/` dengan nama sesuai bagian "Penamaan". Untuk aset baru, tambahkan entrinya di `assets`. Aset baru hanya ditambah bila sudah ada aplikasi yang memakainya.
3. Hapus `"placeholder": true` dari aset yang berkasnya sudah nyata.
4. Jalankan `assets generate`, lalu `assets check`. Bila sebuah sumber ditolak, minta pemilik merek mengekspor ulang berkasnya. Bentuk merek tidak diperbaiki sendiri.
5. Commit sumber, manifest, dan keluaran bersama.
6. Ulangi ekspor biner di tiap OS sesuai spec OS itu, sampai pemeriksaan ekspor basi keluar dengan kode 0.
