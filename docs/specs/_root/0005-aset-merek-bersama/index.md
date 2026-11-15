# 0005. Aset merek bersama

**Scope**: [011 - Aset merek bersama](../../../scope/011%20-%20Aset%20merek%20bersama.md)
**Date**: 2026-11-15
**Status**: In Progress

## Summary

Spec ini menetapkan satu tempat untuk berkas merek, yaitu `shared/assets/`, beserta pemiliknya dan aturan format, penamaan, varian terang dan gelap, resolusi, versi, dan tujuan ekspor ke tiap OS. Isinya dokumen aturan, manifest `brand.json` (daftar isi yang bisa dibaca program), dan satu sumber ikon aplikasi berupa SVG penanda sementara yang jelas bukan merek. Aturannya diperiksa alat bersama `desktop-codegen` lewat sumber baru bernama `assets`, jadi tidak ada alat kedua. Berkas merek nyata hanya datang dari user, dan cara aset masuk ke bundel macOS, resource Windows, serta direktori ikon Linux dirinci spec tiap OS dengan mengikuti aturan ekspor di sini.

## Requirements

**User stories**:

* Sebagai pemilik produk, kamu ingin satu tempat untuk menyerahkan berkas merek, supaya ketiga aplikasi memakai identitas yang sama.
* Sebagai pengembang aplikasi satu OS, kamu ingin tahu dari mana ikon diambil, ke folder mana hasilnya ditaruh, dan bagaimana tahu hasil itu sudah basi.
* Sebagai pemeriksa, kamu ingin satu perintah yang berkata apakah folder merek, manifest, dan setiap sumber sah, tanpa membuka jendela.
* Sebagai lead, kamu ingin pemilik setiap berkas merek tertulis di satu tempat.

**Acceptance criteria** (kontrak; nomor kriteria merujuk [Penerimaan dan bukti fondasi](../../../quality/README.md); semua perintah dijalankan dari akar repo pada Mac koordinasi):

* **AC-1** (kriteria 21): `shared/assets/README.md` sama persis dengan bagian "Isi file". Dengan `SPEC=docs/specs/_root/0005-aset-merek-bersama/index.md`, perintah ``awk '/^````markdown$/{f=1;next} /^````$/{f=0} f' "$SPEC" | diff - shared/assets/README.md`` keluar dengan kode 0 tanpa keluaran. `grep '^## ' shared/assets/README.md | tr '\n' '|'` mencetak `## Pemilik|## Struktur|## Format sumber|## Penamaan|## Varian terang dan gelap|## Resolusi|## Versi|## Tujuan ekspor|## Pemeriksaan|## Mengganti atau menambah aset|`. Tabel pemilik memuat lima baris isi, jadi `sed -n '/^## Pemilik$/,/^## Struktur$/p' shared/assets/README.md | grep -c '^| '` mencetak `7`.
* **AC-2** (Done when "aturan sumber", kriteria 32): `git ls-files shared/assets` mencetak tepat empat path: `shared/assets/README.md`, `shared/assets/brand.json`, `shared/assets/brand.schema.json`, dan `shared/assets/source/app-icon.svg`. Dua sumber tulis tangan sama persis dengan "Isi file": dengan `SPEC` seperti di **AC-1**, ``awk '/^```json$/{n++;f=(n==1);next} /^```$/{f=0} f' "$SPEC" | diff - shared/assets/brand.json`` dan ``awk '/^```xml$/{f=1;next} /^```$/{f=0} f' "$SPEC" | diff - shared/assets/source/app-icon.svg`` keluar dengan kode 0 tanpa keluaran. Pada akhir milestone 1 spec ini, `git ls-files | grep -v '^graphify-out/' | grep -ciE '\.(svg|png|ico|icns|jpe?g|gif|webp|pdf)$'` mencetak `1`, jadi pada akhir milestone 1 spec ini tidak ada berkas gambar lain di repo. Sesudah spec `macos/0007` dibangun, salinan SVG di `apps/macos/Resources/AppIcon.icon/Assets/` menambah satu berkas.
* **AC-3** (Done when "aturan sumber" dan "penamaan"): `cargo run -p desktop-codegen --locked -- assets check` keluar dengan kode 0. Pada akhir milestone 1 baris terakhirnya tepat `assets: ok, 1 assets, 1 source files, 1 placeholder values, 1 outputs up to date`. Uji `reads_the_committed_brand` lulus.
* **AC-4** (Done when "aturan sumber" dan "penamaan"; tabel "Pemeriksaan yang direncanakan" baris Sumber bersama): aturan **A-1** sampai **A-3** dan **S-1** sampai **S-7** ditegakkan. `cargo test -p desktop-codegen --locked --test assets_contract` mencetak `test result: ok. 7 passed`. Uji `rejects_broken_manifests`, `rejects_broken_svgs`, dan `rejects_wrong_folder_contents` memuat satu kasus untuk setiap baris tabel M-1 sampai M-14, S-1 sampai S-27, dan F-1 sampai F-13, dan uji `accepts_valid_sources` untuk setiap baris tabel kasus yang diterima. Sesudah `touch shared/assets/notes.txt`, `cargo run -p desktop-codegen --locked -- assets check` keluar dengan kode 1 dan stderr memuat baris yang diawali `shared/assets/notes.txt: A-1:` lalu baris `assets: failed, 1 problems`. Hapus berkas itu sesudahnya.
* **AC-5** (kriteria 16 untuk keluaran sumber ini): pada akhir milestone 1 spec ini, pada working tree yang bersih, `cargo run -p desktop-codegen --locked -- assets generate` mencetak baris terakhir `assets: ok, 1 assets, 1 source files, 1 placeholder values, 1 outputs, 0 written`, lalu `git diff --exit-code -- shared/assets` keluar dengan kode 0. Sesudah `rm shared/assets/brand.schema.json`, perintah yang sama mencetak `wrote shared/assets/brand.schema.json` dan baris terakhir yang berakhir `1 outputs, 1 written`, dan `git status --short` tidak mencetak apa pun. Angka `1 outputs` berlaku pada akhir milestone 1 spec ini; sesudah spec `macos/0007` dibangun angkanya `3 outputs`, dan bagian lain AC ini tetap. Isi `brand.schema.json` sama persis dengan bagian "Isi file": dengan `SPEC` seperti di **AC-1**, ``awk '/^```json$/{n++;f=(n==2);next} /^```$/{f=0} f' "$SPEC" | diff - shared/assets/brand.schema.json`` keluar dengan kode 0 tanpa keluaran, dan `grep -c '"additionalProperties": false' shared/assets/brand.schema.json` mencetak `3`. Uji `committed_outputs_match_the_source` dan `generation_is_deterministic` lulus.
* **AC-6** (Done when "versi"): tanda versi sumber bisa dicetak dan dibandingkan. `git ls-files -s -- shared/assets/source | grep -cE '^100644 [0-9a-f]{40,64} 0[[:space:]]shared/assets/source/app-icon\.svg$'` mencetak `1`. Bila keluaran `git ls-files -s -- shared/assets/source` disimpan ke `brand-export.txt` di folder sementara di luar repo, perintah pemeriksaan ekspor basi di bagian "Tujuan ekspor" `README.md` keluar dengan kode 0 tanpa keluaran; sesudah satu digit id di berkas itu diubah, perintah yang sama keluar dengan kode 1. `git ls-files --eol shared/assets | grep -c 'attr/text eol=lf'` mencetak `3`.
* **AC-7** (Done when "tujuan ekspor"): bagian `## Tujuan ekspor` di `shared/assets/README.md` memuat satu baris tabel untuk tiap OS dan delapan aturan ekspor. `sed -n '/^## Tujuan ekspor$/,/^## Pemeriksaan$/p' shared/assets/README.md | grep -cE '^\| (macOS|Windows|Linux) \|'` mencetak `3`, dan `grep -c '^\* \*\*E-[1-8]\*\*' shared/assets/README.md` mencetak `8`. Pada akhir milestone 1, `git ls-files | grep -c 'brand-export\.txt$'` mencetak `0`, karena ekspor adalah milik spec OS.
* **AC-8** (kriteria 21): dokumen rujukan menunjuk aturan yang nyata. Sesudah milestone 2, `grep -c 'shared/assets/README.md' docs/shared-resources/README.md` dan `grep -c '0005-aset-merek-bersama' docs/shared-resources/README.md` masing masing mencetak `1` atau lebih, `grep -c 'akan memiliki berkas sumber' docs/shared-resources/README.md` mencetak `0`, `grep -c '^| `shared/assets/` |' docs/architecture/README.md` mencetak `1`, dan `grep -c 'Root workspace.*`shared/assets/`.*sudah ada' docs/architecture/README.md` mencetak `1`. Kedua tujuan tautan baru ada, jadi `test -f shared/assets/README.md && test -f docs/specs/_root/0005-aset-merek-bersama/index.md; echo $?` mencetak `0`. `cargo test -p desktop-core --locked` tetap lulus, jadi tabel aturan arah dependensi tidak ikut berubah.
* **AC-9** (kriteria 32, dan aturan spec [`_root/0001`](../0001-batas-workspace-dan-pemilik/index.md)): workspace tetap hijau dan spec ini tidak menambah paket, dependensi, skrip, atau job CI. `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, dan `cargo clippy --workspace --all-targets --locked -- -D warnings` keluar dengan kode 0. Setiap commit fitur ini memakai subject yang memuat `spec _root/0005 step <n>`. `git log --name-only --format= --grep='spec _root/0005 step' | sed '/^$/d' | sort -u`, dijalankan sesudah commit terakhir tiap milestone, hanya mencetak path dari tabel "File yang boleh diubah tiap milestone". Jadi daftar itu tidak memuat `Cargo.toml`, `Cargo.lock`, `.gitignore`, `.graphifyignore`, atau path di bawah `crates/`, `apps/`, `bindings/`, `scripts/`, dan `.github/`.
* **AC-10** (Done when "aturan sumber"; nilai dari user; langkah manual, `owner: user`): sesudah user menyerahkan berkas ikon aplikasi nyata dalam SVG, langkah 1 sampai 5 di bagian "Mengganti atau menambah aset" `README.md` dijalankan. Hasilnya `cargo run -p desktop-codegen --locked -- assets check` keluar dengan kode 0 dan baris terakhirnya memuat `0 placeholder values`, dan `grep -c '"placeholder"' shared/assets/brand.json` mencetak `0`.

Bagian **Done when** tentang bagaimana aset masuk ke bundle macOS, resource Windows, atau direktori ikon Linux dipenuhi di sini sebagai aturan: tabel tujuan ekspor dan aturan **E-1** sampai **E-8**. Mekanisme dan buktinya di tiap OS bukan AC di sini. Itu kewajiban spec OS, didaftar di bagian "Kontrak untuk spec OS". Jadi bagian bersama scope 011 selesai dengan spec ini, bagian macOS selesai dengan spec macOS nya, dan bagian Windows serta Linux tetap terbuka sampai hostnya ada.

## Decision

**Chosen option**: Opsi 2: aturan, manifest, dan satu sumber penanda di `shared/assets/`, diperiksa sumber `assets` di `desktop-codegen`, dengan ekspor biner memakai alat native tiap OS.

`shared/assets/` menjadi satu satunya sumber aset merek. Aturannya tinggal di `shared/assets/README.md`, isinya dicatat di `brand.json`, alat bersama dari spec `_root/0003` memeriksanya, dan setiap spec OS mengekspor dari sana ke folder aplikasinya dengan mengikuti aturan **E-1** sampai **E-8**.

Keputusan turunannya:

| Hal | Keputusan |
| --- | --- |
| Pemilik | Identitas merek milik user. Folder `shared/assets/` milik lapisan Resource bersama. Alat pemeriksa dan skema milik Tooling bersama. Keluaran ekspor milik aplikasi OS itu. |
| Isi awal | Satu aset, `app-icon`. Logo, ilustrasi, huruf merek, dan artwork lain menunggu kebutuhan produk. |
| Format sumber | SVG saja, teks UTF-8 dengan akhir baris LF, hanya bentuk vektor dari daftar 19 elemen yang diizinkan. Tanpa tulisan, gambar raster tertanam, gaya tertanam, skrip, atau rujukan ke luar berkas. Paling besar 262144 byte. |
| Penamaan | Id huruf kecil dengan tanda hubung. Berkas diturunkan dari id: `source/<id>.svg`, atau `source/<id>-light.svg` dan `source/<id>-dark.svg`. Tanpa versi, ukuran, atau nama OS. |
| Varian terang dan gelap | `variants` bernilai `any` atau `light-dark`, dinamai menurut tampilan tempat aset dipakai. `app-icon` memakai `any`. |
| Resolusi | Sumber vektor. `app-icon` di kanvas `0 0 1024 1024`. Raster dirender langsung dari vektor pada ukuran tujuan. Daftar ukuran ditetapkan spec OS. |
| Versi | `schemaVersion` untuk bentuk manifest. Versi isi adalah riwayat git, dan tanda versi sebuah sumber adalah id blob git nya. Tidak ada nomor yang dinaikkan dengan tangan. |
| Manifest | `shared/assets/brand.json`, JSON yang dibuka `$schema` dan `schemaVersion`, dengan kunci tertutup. Path berkas tidak ditulis di manifest. |
| Penanda sementara | `"placeholder": true` pada aset. `check` melaporkan jumlahnya dan tidak gagal. Penanda awal adalah ikon kotak abu abu bersilang. |
| Nama produk | Bukan aset merek. Sumbernya `build.product_name` di spec `_root/0004`. |
| Alat pemeriksa | Sumber baru bernama `assets` di paket `desktop-codegen` dari spec `_root/0003`. Tanpa alat kedua dan tanpa dependensi baru. |
| Pemeriksaan isi SVG | Pemeriksaan teks dengan daftar elemen yang diizinkan. Alat tidak mengurai XML. |
| Keluaran alat | Satu: `shared/assets/brand.schema.json`. |
| Tujuan ekspor | Folder aplikasi tiap OS, dilacak git. Keluaran teks lewat `desktop-codegen`. Keluaran biner lewat alat native tiap OS, dengan catatan `brand-export.txt`. |
| Dokumen kepemilikan | `shared/assets/README.md`, ditunjuk dari `docs/shared-resources/README.md` dan `docs/architecture/README.md`. |

Nilai dari spec dan dokumen lain yang dipakai di sini, tanpa diputuskan ulang:

| Nilai | Isi | Pemilik |
| --- | --- | --- |
| Lokasi dan lapisan pemilik | `shared/assets/`, bukan paket Cargo, milik Resource bersama, diisi scope 011 | Spec `_root/0001`, tabel "Lokasi dan pemilik" |
| Batas file root | Tanpa `.cargo/config.toml`, paket berawalan `desktop-`, aturan netral host. Spec ini tidak mengubah `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, dan `.gitignore` | Spec `_root/0001` |
| Bentuk alat | Paket `desktop-codegen` di `tooling/codegen/`, pola `Source`, perintah `<sumber> check` dan `<sumber> generate`, kode keluar, bentuk baris sukses dan gagal, pembuka `$schema` dan `schemaVersion`, aturan keluaran O-1 sampai O-7, dan `.gitattributes` per jenis file | Spec `_root/0003` (scope 008) |
| Nama produk | `build.product_name` di `shared/config/public.json` | Spec [`_root/0004`](../0004-konfigurasi-publik/index.md) |
| Folder resource aplikasi | `Resources` di `apps/macos/`, `resources` di `apps/windows/` dan `apps/linux/` | [Peta arsitektur](../../../architecture/README.md), "Tata letak yang dituju" |
| Tujuan aset per OS | Bundle aplikasi native, resource aplikasi WinUI, ikon dan resource sesuai konvensi desktop Linux | [Resource bersama](../../../shared-resources/README.md) |

## Rationale

Alasan, opsi yang ditimbang, daftar keputusan beserta pilihan keduanya, dan bukti percobaan ada di [rationale.md](rationale.md).

## Rancangan aset merek

### Lokasi dan pemilik

Pemilik di sini adalah lapisan dari [peta arsitektur](../../../architecture/README.md), kecuali identitas merek yang dimiliki user sebagai orang.

| Lokasi | Keadaan sesudah spec ini | Pemilik | Yang mengisinya |
| --- | --- | --- | --- |
| `shared/assets/README.md` | dibuat | Resource bersama | Spec ini |
| `shared/assets/brand.json` | dibuat | Resource bersama | Spec ini |
| `shared/assets/source/app-icon.svg` | dibuat, berupa penanda | Resource bersama. Isi gambar milik user | Spec ini, lalu user lewat **AC-10** |
| `shared/assets/brand.schema.json` | dibuat generator | Tooling bersama | `assets generate` |
| `tooling/codegen/src/assets/` dan `tooling/codegen/tests/assets_contract.rs` | dibuat | Tooling bersama | Spec ini |
| Folder keluaran di `apps/macos/Resources/` | rencana | Aplikasi macOS | Spec macOS untuk scope 011 |
| Folder `resources` di `apps/windows/` dan `apps/linux/` | rencana | Aplikasi Windows, Aplikasi Linux | Spec OS itu, menunggu host |

### Isi file

Developer menulis `brand.json`, `app-icon.svg`, dan `README.md` persis seperti ini, masing masing diakhiri satu baris baru. `brand.schema.json` tidak ditulis tangan: generator yang menulisnya, dan hasilnya harus sama persis dengan blok di bawah.

`shared/assets/brand.json`:

```json
{
  "$schema": "./brand.schema.json",
  "schemaVersion": 1,
  "assets": {
    "app-icon": {
      "variants": "any",
      "placeholder": true
    }
  }
}
```

`shared/assets/source/app-icon.svg`. Ini penanda sementara, yaitu kotak abu abu dengan tanda silang, supaya tidak ada yang mengiranya merek:

```xml
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024">
  <title>Placeholder app icon, not a brand asset</title>
  <rect x="64" y="64" width="896" height="896" rx="160" fill="#d9d9d9" stroke="#8c8c8c" stroke-width="16"/>
  <path d="M192 192 L832 832 M832 192 L192 832" fill="none" stroke="#8c8c8c" stroke-width="16"/>
</svg>
```

`shared/assets/brand.schema.json`, keluaran `cargo run -p desktop-codegen --locked -- assets generate` (JSON Schema draft-07, kunci terurut, dua spasi, satu baris baru di akhir; teks berbahasa Inggris mengikuti bahasa komentar kode):

```json
{
  "$comment": "Generated by desktop-codegen from tooling/codegen/src/assets. Do not edit. To regenerate, run: cargo run -p desktop-codegen --locked -- assets generate",
  "$schema": "http://json-schema.org/draft-07/schema#",
  "additionalProperties": false,
  "description": "Manifest of the shared brand assets. An editor aid; the check in desktop-codegen is the authority. Rules: docs/specs/_root/0005-aset-merek-bersama/index.md.",
  "properties": {
    "$schema": {
      "const": "./brand.schema.json",
      "description": "Relative path to this generated schema."
    },
    "assets": {
      "additionalProperties": false,
      "description": "One entry per brand asset, keyed by asset id. An id may not end with -light or -dark.",
      "patternProperties": {
        "^[a-z][a-z0-9]*(-[a-z0-9]+)*$": {
          "additionalProperties": false,
          "description": "One brand asset. Its source files are derived from the id: source/<id>.svg for any, source/<id>-light.svg and source/<id>-dark.svg for light-dark.",
          "properties": {
            "placeholder": {
              "description": "True while the source files are temporary markers and not the real brand. Defaults to false.",
              "type": "boolean"
            },
            "variants": {
              "description": "any: one source file for both light and dark appearance. light-dark: one source file per appearance.",
              "enum": [
                "any",
                "light-dark"
              ]
            }
          },
          "required": [
            "variants"
          ],
          "type": "object"
        }
      },
      "required": [
        "app-icon"
      ],
      "type": "object"
    },
    "schemaVersion": {
      "const": 1,
      "description": "Version of this manifest shape. Goes up by one on every shape change."
    }
  },
  "required": [
    "$schema",
    "schemaVersion",
    "assets"
  ],
  "title": "Brand asset manifest of the desktop foundation, schema version 1",
  "type": "object"
}
```

Skema itu bantuan untuk editor. Ia memuat kunci tertutup, kunci wajib, tipe, nilai tetap `$schema` dan `schemaVersion`, pola id, dan dua nilai `variants`. Larangan id yang diakhiri `-light` atau `-dark`, isi folder, dan isi tiap sumber hanya ditegakkan kode pemeriksaan. Generator membangun skema dari konstanta yang sama dengan pemeriksaan, lalu menulisnya dengan `desktop_codegen::json::to_canonical_json`.

Dua baris ditambahkan ke `.gitattributes` di akar, sesudah baris yang sudah ada:

```gitattributes
/shared/assets/**/*.json text eol=lf
/shared/assets/**/*.svg text eol=lf
```

`shared/assets/README.md`:

````markdown
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
````

### Aturan sumber

Yang diperiksa `plan` sumber `assets`:

| ID | Aturan |
| --- | --- |
| **A-1** | `shared/assets/` ada dan berupa folder, dan memuat berkas `README.md`, berkas `brand.json`, dan folder `source/`. Ketiganya wajib ada dengan jenis itu. Selain itu hanya berkas `brand.schema.json` yang boleh ada. Entri bernama `.DS_Store`, `Thumbs.db`, dan `desktop.ini` diabaikan di sini dan di `source/`. Berkas lain dan subfolder lain ditolak. Ada tidaknya `brand.schema.json` dan isinya diperiksa `check` sebagai keluaran generator, bukan oleh aturan ini. |
| **A-2** | `brand.json` adalah teks UTF-8 berisi satu objek JSON yang memenuhi tabel "Isi `brand.json`" di `README.md`: kunci tertutup tanpa kunci ganda, kunci wajib ada, tipe benar, `$schema` tepat `./brand.schema.json`, `schemaVersion` tepat `1`, `assets` memuat `app-icon`, setiap id cocok dengan `^[a-z][a-z0-9]*(-[a-z0-9]+)*$` dan tidak diakhiri `-light` atau `-dark`, dan `variants` bernilai `any` atau `light-dark`. |
| **A-3** | `source/` berisi tepat berkas yang diturunkan dari manifest: `<id>.svg` untuk `any`, `<id>-light.svg` dan `<id>-dark.svg` untuk `light-dark`. Berkas yang hilang, berkas tambahan, dan subfolder ditolak. |
| **S-1** | Sumber paling besar 262144 byte, berupa teks UTF-8 tanpa BOM, dan tidak memuat karakter `\r`. Aturan ini memberi satu masalah S-1 per berkas, untuk syarat pertama yang gagal dalam urutan itu: ukuran, UTF-8, BOM, lalu `\r`. Bila aturan ini gagal, aturan S lain dilewati untuk berkas itu. |
| **S-2** | Deklarasi adalah teks `<?xml` yang diikuti spasi putih, mulai di byte pertama berkas, sampai `?>` pertama. Deklarasi dibuang lebih dulu bila ada, lalu setiap komentar `<!-- ... -->` dibuang, masing masing sampai `-->` pertama. Sesudah itu teks tidak memuat `<!` dan tidak memuat `<?`. Deklarasi atau komentar yang tidak ditutup ditolak, dan aturan **S-3** sampai **S-7** dilewati untuk berkas itu. |
| **S-3** | Setiap `<` diikuti `/` yang boleh ada atau tidak, lalu nama dari daftar elemen yang diizinkan, lalu salah satu dari: spasi putih, `>`, atau `/>`. Nama adalah deretan huruf ASCII terpanjang di posisi itu dan dibandingkan dengan membedakan huruf besar kecil. `<` yang diikuti `!` atau `?` dilaporkan **S-2** saja. |
| **S-4** | Elemen pertama, yaitu `<` pertama yang bukan tag penutup, adalah `svg`. Tag pembukanya, yaitu teks sampai `>` pertama sesudahnya, memuat `xmlns="http://www.w3.org/2000/svg"` dan `viewBox="0 0 <lebar> <tinggi>"`, dengan lebar dan tinggi bilangan bulat positif tanpa nol di depan. Untuk aset `app-icon`, nilainya tepat `viewBox="0 0 1024 1024"`. |
| **S-5** | Tidak ada atribut peristiwa: teks tidak memuat `on`, lalu satu huruf ASCII atau lebih, lalu spasi putih yang boleh ada, lalu `=`, bila karakter tepat sebelum `on` bukan huruf ASCII, angka, `_`, `.`, `:`, atau `-`. Awal teks dihitung sebagai bukan karakter itu. Huruf besar kecil tidak dibedakan. Jadi `on` di tengah nama atribut, misalnya `data-onload=`, `x_onload=`, `a.onload=`, dan `a:onload=`, sengaja diterima: nama seperti itu bukan atribut peristiwa SVG, yang selalu tanpa awalan. |
| **S-6** | Setiap `href` diikuti spasi putih yang boleh ada, `=`, spasi putih yang boleh ada, tanda kutip `"` atau `'`, lalu `#`. Setiap `url(` diikuti spasi putih yang boleh ada, tanda kutip `"` atau `'` yang boleh ada, lalu `#`. Teks `xml:base` tidak boleh ada. Ketiganya dicari tanpa membedakan huruf besar kecil. |
| **S-7** | Setiap `&` membuka salah satu dari `&amp;`, `&lt;`, `&gt;`, `&quot;`, atau `&apos;`. |

Spasi putih berarti spasi, tab, atau baris baru. Aturan **S-3** sampai **S-7** bekerja pada teks sesudah komentar dan deklarasi dibuang, termasuk isi `title` dan `desc`. Daftar elemen yang diizinkan, berurutan: `svg`, `g`, `path`, `rect`, `circle`, `ellipse`, `line`, `polyline`, `polygon`, `defs`, `linearGradient`, `radialGradient`, `stop`, `clipPath`, `mask`, `use`, `symbol`, `title`, `desc`.

Syarat untuk pemeriksaan:

* `plan` menerima akar repo sebagai masukan dan hanya membaca `<akar>/shared/assets/`, sehingga uji bisa memberinya akar sementara.
* `plan` menjalankan semua aturan dan mengembalikan satu `Problem` untuk setiap pelanggaran, bukan hanya yang pertama, dengan urutan lewat di tabel "Jalur gagal `plan`" di bawah.
* Tiap `Problem` memakai path dari akar repo dengan pemisah `/`: berkas atau folder yang salah untuk **A-1** dan **A-3**, `shared/assets/brand.json` untuk **A-2**, dan path sumber untuk aturan S. Pesannya diawali ID aturan, titik dua, dan satu spasi, lalu keterangan berbahasa Inggris, misalnya `shared/assets/notes.txt: A-1: unexpected entry`. Kesalahan baca dari `serde_json` dilaporkan sebagai **A-2** diikuti pesan `serde_json`. Uji hanya mengunci ID di awal pesan.
* Semua pembacaan memakai isi berkas di disk pada `root`, bukan indeks git.
* Pemeriksaan berjalan di ketiga host tanpa SDK UI dan tanpa jaringan.

Jalur gagal `plan`:

| Keadaan | Masalah yang dilaporkan | Yang dilewati |
| --- | --- | --- |
| `shared/assets/` tidak ada atau bukan folder | Satu masalah di path `shared/assets`: `A-1: missing folder` | Semua aturan lain |
| `README.md`, `brand.json`, atau `source/` tidak ada, atau jenisnya salah (folder di tempat berkas, atau sebaliknya) | Satu masalah **A-1** di path entri itu | Untuk `brand.json`: **A-2**, **A-3**, dan aturan S. Untuk `source/`: **A-3** dan aturan S |
| `brand.json` ada tetapi tidak terbaca atau bukan teks UTF-8 | Satu masalah di path itu: `A-2: cannot read: <kesalahan>` | **A-3** dan aturan S |
| `parse` gagal | Masalah **A-2** dari `parse` | **A-3** dan aturan S |
| Sumber yang diturunkan dari manifest tidak ada di `source/` | Satu masalah **A-3** di path sumber itu | Aturan S untuk sumber itu |
| Sumber ada tetapi tidak terbaca | Satu masalah di path itu: `S-1: cannot read: <kesalahan OS>` | Aturan S lain untuk sumber itu |
| Entri di `source/` yang bukan sumber turunan, termasuk subfolder | Satu masalah **A-3** di path entri itu | Tidak ada |

### Bentuk alat

Nilai di bagian ini datang dari spec `_root/0003`. Bila spec itu berubah, bagian ini ikut diperbarui. Spec ini menambah satu sumber dan tidak membuat alat, paket, skrip, atau dependensi sendiri.

Tata letak:

| Path | Isi |
| --- | --- |
| `tooling/codegen/src/assets/mod.rs` | `SOURCE`, konstanta, tipe, `parse`, fungsi `plan`, aturan **A-1** sampai **A-3**, dan `pub use svg::check_svg;` |
| `tooling/codegen/src/assets/svg.rs` | Modul privat. `check_svg` dan aturan **S-1** sampai **S-7** |
| `tooling/codegen/src/assets/schema.rs` | Modul privat. Penulis `brand.schema.json` |
| `tooling/codegen/src/lib.rs` | Dua perubahan: baris `pub mod assets;`, dan entri `assets::SOURCE` di akhir larik `SOURCES` (`rustfmt` boleh memecah larik itu ke beberapa baris). Teks usage `sources: ...` ikut memuat `assets`, karena alat menyusunnya dari `SOURCES` |
| `tooling/codegen/tests/assets_contract.rs` | Tujuh uji kontrak |

Permukaan Rust. Tulis tanda tangan ini persis. Setiap item publik punya rustdoc berbahasa Inggris. `Asset` dan `Brand` menurunkan `Debug`, `Clone`, `PartialEq`, dan `Eq`. `Variants` juga menurunkan `Copy`.

```rust
// tooling/codegen/src/assets/mod.rs
pub const SOURCE: Source;                              // name: "assets"
pub const SCHEMA_VERSION: u64 = 1;
pub const DIR: &str = "shared/assets";
pub const README_PATH: &str = "shared/assets/README.md";
pub const MANIFEST_PATH: &str = "shared/assets/brand.json";
pub const SCHEMA_PATH: &str = "shared/assets/brand.schema.json";
pub const SOURCE_DIR: &str = "shared/assets/source";
pub const REQUIRED_ASSET: &str = "app-icon";
pub const APP_ICON_VIEW_BOX: &str = "0 0 1024 1024";
pub const MAX_SOURCE_BYTES: usize = 262_144;
pub const ALLOWED_ELEMENTS: [&str; 19];                // urutan di bagian "Aturan sumber"
pub enum Variants { Any, LightDark }
pub struct Asset { pub id: String, pub variants: Variants, pub placeholder: bool }
impl Asset { pub fn source_paths(&self) -> Vec<String> }   // path dari akar repo; light lalu dark
pub struct Brand { pub assets: Vec<Asset>, pub placeholders: usize }
pub fn parse(manifest: &str) -> Result<Brand, Vec<Problem>>;
pub fn check_svg(path: &str, asset_id: &str, bytes: &[u8]) -> Vec<Problem>;
```

Perilaku tiap fungsi:

| Fungsi | Perilaku |
| --- | --- |
| `parse` | Fungsi murni atas teks manifest. Menjalankan **A-2** dan mengumpulkan semua masalah. Teks dibaca ke tipe `serde` privat yang memakai `deny_unknown_fields` di objek teratas dan di tiap aset; `assets` dibaca lewat `desktop_codegen::json::UniqueMap`, jadi id ganda ditolak. `Variants` dibaca dengan `#[serde(rename_all = "kebab-case")]`, jadi `Any` ditulis `any` dan `LightDark` ditulis `light-dark`. `placeholder` memakai `#[serde(default)]`. Bila pembacaan itu gagal, hasilnya tepat satu masalah. `Brand.assets` terurut menurut id dalam urutan byte. `placeholders` adalah jumlah aset yang `placeholder` nya `true`. |
| `Asset::source_paths` | `["shared/assets/source/<id>.svg"]` untuk `Any`. `["shared/assets/source/<id>-light.svg", "shared/assets/source/<id>-dark.svg"]` untuk `LightDark`. |
| `check_svg` | Fungsi murni atas isi satu sumber. Menjalankan **S-1** sampai **S-7** untuk aset `asset_id` dan memakai `path` sebagai path tiap masalah. Daftar kosong berarti sah. |
| `SOURCE.plan` | Membaca isi `shared/assets/` dan menjalankan **A-1**. Membaca `brand.json` lalu memanggil `parse`. Bila lulus, membaca isi `source/`, menjalankan **A-3**, dan memanggil `check_svg` untuk setiap sumber yang ada. Bila ada masalah, mengembalikan semuanya. Bila tidak, mengembalikan `Plan` dengan satu `Output` untuk `SCHEMA_PATH` dan `summary` berbentuk `<a> assets, <f> source files, <p> placeholder values`. Ketiga kata itu selalu jamak. Keluaran spec OS menurut **E-4** ditambahkan sesudah skema. |

Baris perintah, dengan bentuk yang ditetapkan spec `_root/0003`:

| Perintah | Keluaran saat sukses (stdout) | Kode | Saat gagal (stderr) |
| --- | --- | --- | --- |
| `cargo run -p desktop-codegen --locked -- assets check` | `assets: ok, <ringkasan>, <n> outputs up to date` | `0` | Satu baris `<path>: <pesan>` per masalah, lalu `assets: failed, <n> problems`, kode `1`. Tidak menulis apa pun. |
| `cargo run -p desktop-codegen --locked -- assets generate` | Satu baris `wrote <path>` per berkas yang ditulis, lalu `assets: ok, <ringkasan>, <n> outputs, <k> written` | `0` | Sama, kode `1`, tidak ada yang ditulis |

Skema hasil generator mematuhi aturan keluaran O-1 sampai O-7 spec `_root/0003`: penanda di kunci `$comment`, isi yang hanya bergantung pada kode alat, UTF-8 dengan akhir baris `\n`, kunci terurut, path tetap di kode alat, dan satu pintu lewat `Plan.outputs`.

### Uji kontrak

`tooling/codegen/tests/assets_contract.rs` hanya memakai path publik `desktop_codegen` dan pustaka standar, dan dibuka dengan komentar dokumen `//!`.

| Uji | Yang dibuktikan |
| --- | --- |
| `committed_outputs_match_the_source` | `check(&repo_root(), assets)` lulus dan barisnya diawali `assets: ok, `. |
| `generation_is_deterministic` | Dua panggilan `plan` pada repo memberi `Plan` yang sama. Tiap keluaran berakhir `\n`, tidak memuat `\r`, tidak memuat path absolut akar repo, dan memuat `Generated by desktop-codegen` serta `Do not edit`. |
| `reads_the_committed_brand` | `parse` atas `brand.json` yang ada di repo memberi tepat satu aset: id `app-icon`, `Variants::Any`, `placeholder` `true`, dan `placeholders == 1`. `source_paths` aset itu adalah `["shared/assets/source/app-icon.svg"]`. |
| `rejects_broken_manifests` | Kasus M-1 sampai M-14. |
| `rejects_broken_svgs` | Kasus S-1 sampai S-27. |
| `rejects_wrong_folder_contents` | Kasus F-1 sampai F-13. |
| `accepts_valid_sources` | Kasus M-15 sampai M-17, S-28 sampai S-31, dan F-14 sampai F-17. |

Kasus manifest. Tiap kasus mengganti kemunculan pertama teks di kolom Cari dengan teks di kolom Ganti pada `brand.json` dari "Isi file", lalu memanggil `parse`. `\n` berarti satu baris baru, dan spasi sesudahnya adalah indentasi di "Isi file". Salah satu pesan wajib diawali `A-2:`.

| No. | Cari | Ganti |
| --- | --- | --- |
| M-1 | `"schemaVersion": 1` | `"schemaVersion": 2` |
| M-2 | `"schemaVersion": 1` | `"schemaVersion": "1"` |
| M-3 | `"./brand.schema.json"` | `"./other.schema.json"` |
| M-4 | `"schemaVersion": 1,` | `"schemaVersion": 1,\n  "product_name": "X",` |
| M-5 | `"variants": "any",` | `"variants": "any",\n      "path": "x.svg",` |
| M-6 | `"variants": "any"` | `"variants": "both"` |
| M-7 | `"variants": "any",\n      ` | teks kosong |
| M-8 | `"app-icon"` | `"App-Icon"` |
| M-9 | `"app-icon"` | `"app-icon-dark"` |
| M-10 | `"app-icon"` | `"logo"` |
| M-11 | `"assets": {` | `"assets": {\n    "app-icon": {\n      "variants": "any"\n    },` |
| M-12 | `"placeholder": true` | `"placeholder": "yes"` |
| M-13 | seluruh isi | `[]` |
| M-14 | seluruh isi | `{` |

Manifest yang diterima (`parse` lulus):

| No. | Masukan | Hasil |
| --- | --- | --- |
| M-15 | `brand.json` dari "Isi file" | Satu aset, `placeholders == 1` |
| M-16 | Cari `,\n      "placeholder": true`, Ganti teks kosong | Satu aset, `placeholder` `false`, `placeholders == 0` |
| M-17 | Cari `"assets": {`, Ganti `"assets": {\n    "logo": {\n      "variants": "light-dark"\n    },` | Dua aset berurutan `app-icon` lalu `logo`. `source_paths` untuk `logo` adalah `shared/assets/source/logo-light.svg` lalu `shared/assets/source/logo-dark.svg` |

Kasus sumber. Tiap kasus berangkat dari `app-icon.svg` di "Isi file", diubah seperti kolom Perubahan, lalu diberikan ke `check_svg` dengan `asset_id` `app-icon`. Salah satu pesan wajib diawali ID di kolom Aturan; pesan lain boleh ada.

| No. | Perubahan | Aturan |
| --- | --- | --- |
| S-1 | Cari `</svg>`, Ganti `<script/></svg>` | **S-3** |
| S-2 | Cari `</svg>`, Ganti `<s:script xmlns:s="http://www.w3.org/2000/svg"/></svg>` | **S-3** |
| S-3 | Cari `</svg>`, Ganti `<text>hi</text></svg>` | **S-3** |
| S-4 | Cari `</svg>`, Ganti `<image href="#a"/></svg>` | **S-3** |
| S-5 | Cari `</svg>`, Ganti `<style>@import url(x.css);</style></svg>` | **S-3** |
| S-6 | Cari `<rect `, Ganti `<rect onload = "x()" ` | **S-5** |
| S-7 | Cari `<rect `, Ganti `<rect/onload="x()" ` | **S-5**, dan juga **S-3** |
| S-8 | Cari `</svg>`, Ganti `<use href='http://example.invalid/a.svg#b'/></svg>` | **S-6** |
| S-9 | Cari `</svg>`, Ganti `<use xlink:href="b.svg#x"/></svg>` | **S-6** |
| S-10 | Cari `</svg>`, Ganti `<use HREF="http://example.invalid/a.svg#b"/></svg>` | **S-6** |
| S-11 | Cari `<svg `, Ganti `<svg xml:base="http://example.invalid/" ` | **S-6** |
| S-12 | Cari `fill="#d9d9d9"`, Ganti `fill="url(http://example.invalid/p)"` | **S-6** |
| S-13 | Cari `fill="#d9d9d9"`, Ganti `style="fill: URL( 'http://example.invalid/p' )"` | **S-6** |
| S-14 | Cari `fill="#d9d9d9"`, Ganti `style="fill: u&#114;l(http://example.invalid/p)"` | **S-7** |
| S-15 | Awali berkas dengan `<!DOCTYPE svg [<!ENTITY a "b">]>\n` | **S-2** |
| S-16 | Awali berkas dengan `<?xml version="1.0"?>\n<?xml-stylesheet href="a.css"?>\n` | **S-2** |
| S-17 | Awali berkas dengan `<?xml-stylesheet href="#x"?>\n` | **S-2** |
| S-18 | Cari `</svg>`, Ganti `<![CDATA[x]]></svg>` | **S-2** |
| S-19 | Cari `</svg>`, Ganti `<!-- belum ditutup </svg>` | **S-2** |
| S-20 | Cari `viewBox="0 0 1024 1024"`, Ganti `viewBox="0 0 512 512"` | **S-4** |
| S-21 | Cari ` viewBox="0 0 1024 1024"`, Ganti teks kosong | **S-4** |
| S-22 | Cari ` xmlns="http://www.w3.org/2000/svg"`, Ganti teks kosong | **S-4** |
| S-23 | Cari `<svg `, Ganti `<g/><svg ` | **S-4** |
| S-24 | Baris baru pertama diganti `\r\n` | **S-1** |
| S-25 | Awali berkas dengan BOM (U+FEFF) | **S-1** |
| S-26 | Tambahkan di akhir berkas `<!--`, 262144 huruf `a`, lalu `-->\n` | **S-1** |
| S-27 | Isi berkas adalah lima byte `FF FE 3C 00 73` | **S-1** |

Sumber yang diterima (`check_svg` mengembalikan daftar kosong):

| No. | Masukan | `asset_id` |
| --- | --- | --- |
| S-28 | `app-icon.svg` dari "Isi file" | `app-icon` |
| S-29 | Diawali `<?xml version="1.0" encoding="UTF-8"?>\n<!-- catatan <script> -->\n` | `app-icon` |
| S-30 | Cari `</svg>`, Ganti `<defs><linearGradient id="g"><stop offset="0" stop-color="#fff"/></linearGradient></defs><rect fill="url(#g)"/><rect fill="url('#g')"/><use href="#g"/><desc>catatan &amp; keterangan</desc></svg>` | `app-icon` |
| S-31 | Cari `viewBox="0 0 1024 1024"`, Ganti `viewBox="0 0 300 80"` | `logo` |

Kasus folder. Tiap kasus membuat akar sementara di bawah `env!("CARGO_TARGET_TMPDIR")` bernama `assets_contract_` ditambah nomor kasus dalam huruf kecil, misalnya `assets_contract_f1`, seperti pola di `tokens_contract.rs`. Folder lama dihapus di awal kasus lalu dibuat ulang. Kasus itu menyalin ke `shared/assets/` di dalamnya empat berkas yang ada di repo, mengubahnya seperti kolom Perubahan, lalu memanggil `(assets::SOURCE.plan)(&akar)`. Salah satu pesan wajib diawali ID di kolom Aturan.

| No. | Perubahan | Aturan |
| --- | --- | --- |
| F-1 | Berkas tambahan `shared/assets/notes.txt` | **A-1** |
| F-2 | Subfolder tambahan `shared/assets/export/` | **A-1** |
| F-3 | `README.md` dihapus | **A-1** |
| F-4 | `brand.json` dihapus | **A-1** |
| F-5 | Folder `source/` dihapus beserta isinya | **A-1** |
| F-6 | Folder `shared/assets/` tidak dibuat sama sekali | **A-1** |
| F-7 | `brand.json` diganti folder kosong bernama sama | **A-1** |
| F-8 | Berkas tambahan `source/logo.svg`, salinan `app-icon.svg` | **A-3** |
| F-9 | `source/app-icon.svg` dihapus | **A-3** |
| F-10 | Subfolder tambahan `source/old/` | **A-3** |
| F-11 | Berkas tambahan `source/app-icon.png` berisi teks `x` | **A-3** |
| F-12 | `"variants": "any"` di `brand.json` diganti `"variants": "light-dark"`, isi `source/` tetap | **A-3** |
| F-13 | `source/app-icon.svg` diubah seperti kasus S-1 | **S-3** |

Folder yang diterima (`plan` lulus):

| No. | Perubahan | `summary` |
| --- | --- | --- |
| F-14 | Tidak ada | `1 assets, 1 source files, 1 placeholder values` |
| F-15 | Berkas `.DS_Store`, `Thumbs.db`, dan `desktop.ini` ditambahkan di `shared/assets/` dan di `source/` | `1 assets, 1 source files, 1 placeholder values` |
| F-16 | `brand.schema.json` dihapus. Ketiadaan keluaran ditangkap `check`, bukan `plan` | `1 assets, 1 source files, 1 placeholder values` |
| F-17 | `"variants": "any"` diganti `"variants": "light-dark"`, dan `source/app-icon.svg` diganti dua salinannya bernama `app-icon-light.svg` dan `app-icon-dark.svg` | `1 assets, 2 source files, 1 placeholder values` |

### Kontrak untuk spec OS

Spec OS untuk scope 011 merujuk spec ini dan tidak mengulang aturannya. Supaya developer OS itu tidak perlu bertanya lagi, spec OS menetapkan hal berikut:

| Hal | Aturan yang diikuti |
| --- | --- |
| Path persis folder keluaran | **E-2** |
| Keluaran teks: modul emitter di `tooling/codegen/src/assets/<target>.rs` yang menambah `Output` ke `Plan` sumber `assets`, mematuhi O-1 sampai O-7 spec `_root/0003`. Untuk salinan SVG, penanda O-1 ditulis sebagai komentar sesudah deklarasi `<?xml ... ?>` bila ada | **E-4** |
| Keluaran biner: alat ekspor dan versinya di host itu, perintah ekspor ulang, dan lokasi `brand-export.txt` | **E-5**, **E-6**, **E-8** |
| Daftar keluaran: nama, format, ukuran piksel, dan skala | **E-8**, bagian "Resolusi" dan "Penamaan" |
| Pemetaan `any`, `light`, dan `dark` ke mekanisme tampilan native | Bagian "Varian terang dan gelap" |
| Baris `.gitattributes` untuk jenis berkas keluaran teksnya, dan baris `text eol=lf` untuk `brand-export.txt`, supaya pemeriksaan ekspor basi tidak gagal karena akhir baris | Aturan O-6 spec `_root/0003`, **E-6** |
| Bukti bahwa aset tampil di aplikasi yang berjalan | Langkah manual di sesi GUI, milik user |

Spec ini tidak menetapkan apa pun di dalam folder aplikasi. Ia hanya menetapkan dari mana ekspor membaca, di lapisan mana keluarannya tinggal, dan apa yang wajib dicatat.

### Nilai dan sumbernya

| Nilai | Sumber |
| --- | --- |
| Lapisan pemilik `shared/assets/` | Spec `_root/0001` dan peta arsitektur. |
| Gambar `app-icon.svg` awal | Penanda di bagian "Isi file". Gambar nyata hanya dari user. |
| `"placeholder": true` pada `app-icon` | Keputusan spec ini, karena gambarnya belum nyata. |
| Path tiap sumber | Diturunkan dari id dan `variants` oleh `Asset::source_paths`. |
| Jumlah di baris sukses | `<a>` panjang `Brand.assets`, `<f>` jumlah seluruh `source_paths`, `<p>` `Brand.placeholders`, `<n>` panjang `Plan.outputs`. |
| Versi skema yang dikenal | Konstanta `SCHEMA_VERSION`, sekarang `1`. |
| Kanvas `0 0 1024 1024` | Keputusan spec ini. Sisi 1024 piksel adalah raster terbesar yang diminta ikon aplikasi macOS. |
| Batas 262144 byte | Keputusan spec ini. Penanda berukuran 334 byte. |
| Daftar elemen yang diizinkan | Keputusan spec ini: bentuk dasar, kelompok, gradasi, potongan, dan keterangan. Diperluas lewat pembaruan spec ini. |
| Isi `brand.schema.json` | Dibangun `assets/schema.rs` dari konstanta di atas, ditulis `to_canonical_json`. |
| Akar yang diperiksa | Argumen `root` fungsi `plan`. Perintah repo memberinya akar repo dari alat; uji memberinya akar sementara. |
| Tanda versi sumber | Id blob dari `git ls-files -s -- shared/assets/source`. |
| Isi `brand-export.txt` | Keluaran perintah yang sama pada saat ekspor, ditulis langkah ekspor spec OS sesudah `git status --porcelain -- shared/assets` tidak mencetak apa pun. |
| Folder keluaran per OS | Peta arsitektur dan rujukan platform. Path persis ditetapkan spec OS. |
| Bentuk alat, perintah, dan baris keluaran | Spec `_root/0003`. |
| Keluaran yang diharapkan tiap perintah git dan `grep` | Dicoba di Mac koordinasi pada 2026-11-15, tercatat di [rationale.md](rationale.md). |

### Invarian

* Setiap berkas merek di repo adalah sumber di `shared/assets/source/`, atau keluaran ekspor di folder aplikasi satu OS.
* `shared/assets/` hanya memuat empat entri: `README.md`, `brand.json`, `brand.schema.json`, dan `source/`.
* Isi `source/` selalu sama dengan yang diturunkan dari `brand.json`.
* Nama berkas sumber tidak berubah saat isinya diganti, jadi langkah ekspor tidak perlu diubah saat merek nyata tiba.
* Aset merek tidak pernah digambar, ditebak, atau diunduh oleh developer atau agen. Sumbernya hanya user.
* `desktop-codegen` tidak mendapat dependensi baru dan tidak menulis berkas biner.

### Keamanan

SVG adalah berkas teks yang bisa membawa skrip, gaya, dan rujukan ke alamat luar. Karena itu aturan **S-3** memakai daftar elemen yang diizinkan, bukan daftar larangan: apa pun yang tidak dikenal ditolak, termasuk elemen berawalan namespace. **S-2** menolak `DOCTYPE`, `ENTITY`, dan `CDATA`, **S-5** dan **S-6** menolak atribut peristiwa, `xml:base`, dan rujukan ke luar, dan **S-7** menutup cara menyamarkan `url(` dengan rujukan karakter. Pemeriksaan ini membaca teks dan tidak mengurai XML, jadi tidak ada pengurai yang bisa dibebani berkas jahat. Komentar dibaca seperti XML membacanya, yaitu sampai `-->` pertama; itu cocok dengan cara alat ekspor dan aplikasi native membaca SVG. Tujuannya menjaga kebersihan sumber yang datang dari pemilik merek. Ia bukan penyaring untuk berkas dari pihak yang tidak dipercaya, dan aplikasi tidak pernah memuat SVG dari pengguna lewat jalur ini. Spec ini tidak menambah rahasia, kredensial, variabel lingkungan, atau unduhan. Berkas merek hanya diterima dari user, jadi hak pakainya jelas.

### Kasus gagal yang sudah diputuskan

| Kejadian | Perilaku yang diharapkan |
| --- | --- |
| User menyerahkan PNG atau format lain, bukan SVG | Berkas itu tidak masuk `source/` (**A-3** menolaknya). Minta versi SVG. Bila memang tidak ada, perbarui spec ini lebih dulu (lihat Follow-up). |
| SVG dari alat desain memuat `text`, `image`, `style`, atau rujukan luar | `assets check` mencetak satu baris per pelanggaran. Minta pemilik merek mengekspor ulang dengan tulisan diubah menjadi path dan gaya ditulis sebagai atribut. Bentuk merek tidak diperbaiki sendiri. |
| SVG memakai elemen vektor yang sah tetapi di luar daftar, misalnya `filter` atau `pattern` | **S-3** menolaknya. Minta ekspor yang lebih sederhana. Bila merek memang membutuhkannya, perluas daftar lewat pembaruan spec ini sesudah elemen itu terbukti dirender sama di ketiga OS. |
| Keterangan di `title` atau `desc` memuat teks seperti `href` atau ` onclick=` | **S-6** atau **S-5** menolaknya, karena aturan bekerja pada seluruh teks. Tulis ulang keterangannya. |
| Sumber lolos pemeriksaan tetapi bukan XML yang utuh | Alat tidak mengurai XML, jadi ini baru terlihat saat alat ekspor atau aplikasi satu OS membacanya. Perbaiki sumber lewat pemilik merek. |
| `check` gagal karena `brand.schema.json` berbeda dari keluaran generator | Jalankan `assets generate` dan commit hasilnya. Skema tidak disunting tangan. |
| Sumber berubah sesudah ekspor biner dibuat, termasuk karena aset baru ditambah | Pemeriksaan ekspor basi keluar dengan kode 1 untuk setiap OS, karena catatan memuat semua sumber. Ekspor diulang di host OS itu. Di host lain hal itu dicatat sebagai pekerjaan terbuka, bukan diperbaiki dengan alat lain. |
| Ekspor dijalankan saat ada perubahan di `shared/assets/` yang belum masuk commit, termasuk berkas baru yang belum dilacak | Catatan ekspor akan memuat id yang salah atau tidak memuat berkas baru itu. Karena itu **E-6** mensyaratkan `git status --porcelain -- shared/assets` tidak mencetak apa pun lebih dulu. |
| `brand-export.txt` tersimpan dengan akhir baris CRLF di Windows | Pemeriksaan ekspor basi selalu keluar dengan kode 1. Spec OS menambah baris `text eol=lf` untuk berkas itu di `.gitattributes`. |
| Keluaran biner disunting tangan sesudah ekspor | Catatan ekspor tidak menangkapnya. Aturan **E-3** dijaga lewat review. |
| Checkout di Windows mengubah akhir baris | Dicegah dua baris `.gitattributes`. Bila tetap terjadi, **S-1** menolak sumber itu. Itu temuan untuk host Windows, bukan alasan melonggarkan aturan. |
| Bentuk alat di spec `_root/0003` berubah sebelum build spec ini | Bagian "Bentuk alat" diperbarui lebih dulu. Developer tidak membuat alat kedua. |
| Hook commit gagal dengan `Graphify extraction failed` untuk berkas di `shared/assets/` | Tidak terjadi pada Graphify 0.9.48 (sudah dicoba, lihat rationale). Bila versi lain gagal, ikuti kasus yang sama di spec `_root/0001`: entri `.graphifyignore` ditambah lewat pemilik berkas itu dan dicatat di spec ini. |

### Skenario uji kritis

* Jalur utama: repo dengan empat berkas dari "Isi file" lulus `assets check`, dan `generate` tidak mengubah apa pun, memverifikasi **AC-2**, **AC-3**, **AC-5**.
* Kepemilikan: daftar heading dan baris tabel pemilik sesuai, memverifikasi **AC-1**.
* Manifest rusak: kunci tak dikenal seperti `product_name`, id yang salah, dan `variants` yang tidak dikenal ditolak dengan **A-2**, memverifikasi **AC-4**.
* Isi terlarang: `script`, elemen berawalan namespace, `href` ke luar, dan `DOCTYPE` ditolak dengan aturan S, memverifikasi **AC-4**.
* Berkas tersesat: `notes.txt`, subfolder, atau PNG di `source/` ditolak dengan **A-1** atau **A-3**, memverifikasi **AC-4**.
* Keluaran menyimpang: skema yang dihapus atau disunting tertangkap `check` dan uji `committed_outputs_match_the_source`, memverifikasi **AC-5**.
* Versi: catatan ekspor contoh di luar repo cocok dengan sumber, lalu tidak cocok sesudah diubah, memverifikasi **AC-6**.
* Dokumen: tautan dari dua dokumen rujukan mengarah ke berkas yang ada, memverifikasi **AC-8**.
* Gerbang: `cargo fmt`, `cargo clippy`, dan `cargo test --workspace`, memverifikasi **AC-9**.
* Tidak ada skenario izin atau peran, karena spec ini tidak punya permukaan yang berjalan. Yang boleh mengubah apa diatur tabel pemilik dan ditinjau saat review.

## Perubahan dokumen rujukan

Langkah 3 Build plan mengubah dua dokumen berikut. Bagian lain kedua dokumen itu tidak diubah. Spec lain di wave yang sama juga mengubah dokumen ini, jadi ubah hanya kalimat dan baris yang disebut di bawah.

1. Di `docs/shared-resources/README.md` bagian "Aset merek", ganti kalimat pertama, yang diawali "`shared/assets/` akan memiliki berkas sumber", dengan tiga kalimat ini. Tiga kalimat sesudahnya tetap.

   ```markdown
   `shared/assets/` memiliki berkas sumber, manifest `brand.json`, dan aturan pemilik, penamaan, format, optimasi, varian terang dan gelap, resolusi, versi, serta tujuan ekspor di [shared/assets/README.md](../../shared/assets/README.md). Sumber itu diperiksa `cargo run -p desktop-codegen --locked -- assets check`. Keputusan dan alasannya ada di [spec 0005](../specs/_root/0005-aset-merek-bersama/index.md).
   ```

2. Di `docs/architecture/README.md` bagian "Tata letak yang dituju", di paragraf yang diawali "Path ini menunjukkan", ada kalimat berisi daftar lokasi yang berakhir "sudah ada.". Sisipkan `` `shared/assets/`, `` tepat sebelum kata `dan` terakhir di kalimat itu. Lokasi lain yang sudah tercantum di kalimat itu, termasuk yang ditambah spec lain, tidak diubah.

3. Di dokumen yang sama, tambahkan satu baris sesudah baris terakhir tabel `### Yang sudah ada`:

   ```markdown
   | `shared/assets/` | Sumber aset merek bersama dari [spec 0005](../specs/_root/0005-aset-merek-bersama/index.md): aturan di `README.md`, manifest `brand.json`, dan berkas sumber di `source/`. |
   ```

## Build plan

Pendekatan build proyek adalah Tracer Bullet. Benang tipis fitur ini adalah satu aset penanda yang melewati seluruh aturan: ada di `source/`, tercatat di manifest, lulus pemeriksaan alat bersama, dan punya tujuan ekspor yang tertulis. Ekspor ke aplikasi menyusul di spec OS. Satu langkah menjadi satu commit.

File yang boleh diubah tiap milestone:

| Milestone | Path |
| --- | --- |
| 1 | Empat berkas di `shared/assets/`, tiga berkas di `tooling/codegen/src/assets/`, `tooling/codegen/src/lib.rs` (dua perubahan), `tooling/codegen/tests/assets_contract.rs`, `.gitattributes` (dua baris), `docs/specs/_root/0005-aset-merek-bersama/verify.md`, baris `**Status**` di `index.md` spec ini |
| 2 | `docs/shared-resources/README.md`, `docs/architecture/README.md`, `verify.md` spec ini |
| 3 | `shared/assets/brand.json`, berkas di `shared/assets/source/`, `verify.md` spec ini |

Empat berkas `graphify-out/` yang ditulis hook commit selalu boleh ikut.

**Milestone 1: Sumber, manifest, aturan, dan pemeriksaan aset merek** (langkah 1 dan 2; needs milestone 1 spec `_root/0003`, yang sudah ada di `main`; tidak dibangun bersamaan dengan milestone 1 spec `_root/0003`, `_root/0004`, atau `_root/0006`, karena `tooling/codegen/src/lib.rs` dan `.gitattributes` dipakai bersama)

1. Dalam satu commit: buat `shared/assets/README.md`, `shared/assets/brand.json`, dan `shared/assets/source/app-icon.svg` persis seperti "Isi file"; tambahkan sumber `assets` ke `desktop-codegen` menurut "Bentuk alat" dengan aturan **A-1** sampai **A-3** dan **S-1** sampai **S-7**; tambahkan dua baris `.gitattributes`; jalankan `cargo run -p desktop-codegen --locked -- assets generate` untuk menulis `shared/assets/brand.schema.json`; dan tulis `tooling/codegen/tests/assets_contract.rs` dengan tujuh uji di "Uji kontrak". Sebelum commit, perintah **AC-3**, **AC-4**, dan empat perintah gerbang di **AC-9** harus lulus, satisfies **AC-1**, **AC-2**, **AC-3**, **AC-4**, **AC-5**, **AC-6**, **AC-7**, **AC-9**
2. Dengan `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, jalankan ulang setiap perintah **AC-1** sampai **AC-7** dan **AC-9** pada commit langkah 1, catat perintah dan keluarannya di `verify.md` bagian `## Milestone 1: Sumber, manifest, aturan, dan pemeriksaan aset merek`, hapus `target/`, lalu commit `verify.md`. Jalankan ulang perintah `git log` di **AC-9** sesudah commit itu, satisfies **AC-1**, **AC-2**, **AC-3**, **AC-4**, **AC-5**, **AC-6**, **AC-7**, **AC-9**

**Milestone 2: Dokumen rujukan menunjuk aturan aset** (langkah 3, sesudah milestone 1)

3. Perbarui `docs/shared-resources/README.md` dan `docs/architecture/README.md` sesuai bagian "Perubahan dokumen rujukan", lalu catat perintah **AC-8** dan perintah `git log` di **AC-9** beserta keluarannya di `verify.md` bagian `## Milestone 2: Dokumen rujukan menunjuk aturan aset`, satisfies **AC-8**, **AC-9**

**Milestone 3: Berkas merek nyata dari user** (langkah 4, `BLOCKED`, `owner: user`)

4. Sesudah user menyerahkan ikon aplikasi dalam SVG, jalankan langkah 1 sampai 5 di bagian "Mengganti atau menambah aset" `README.md`, lalu catat perintah dan keluarannya di `verify.md` bagian `## Milestone 3: Berkas merek nyata dari user`. Sesi koordinasi tidak menjalankan langkah ini tanpa berkas dari user. Ekspor ulang per OS (langkah 6 di bagian itu) mengikuti spec OS, satisfies **AC-10**

## Consequences

**Positive**:

* Pemilik setiap berkas merek tertulis di satu tempat, dan aturan sumber ditegakkan gerbang `cargo test --workspace --locked` yang sudah ada.
* Satu alat untuk semua sumber bersama. Developer mengenal satu pola perintah untuk token, konfigurasi, dan aset.
* Manifest dan nama berkas tidak bisa berbeda, karena path diturunkan dari id.
* Tidak ada nomor versi yang bisa lupa dinaikkan. Ekspor basi ketahuan dengan satu perintah git yang sama di semua OS.
* Merek nyata bisa masuk kelak tanpa mengubah nama berkas atau langkah ekspor.
* Tidak ada paket atau dependensi baru. `Cargo.toml` dan `Cargo.lock` tidak berubah.

**Negative / tradeoffs**:

* Build spec ini tidak bisa berjalan bersamaan dengan build sumber lain di `desktop-codegen` (spec `_root/0004` dan `_root/0006`), karena `tooling/codegen/src/lib.rs` dan `.gitattributes` dipakai bersama.
* Ekspor biner punya satu langkah per OS dengan alat sendiri. Hasil raster bisa sedikit berbeda antar OS, dan satu host tidak bisa memperbarui keluaran OS lain.
* Pemeriksaan ekspor basi adalah perintah git di luar `desktop-codegen`, jadi belum masuk gerbang `cargo test`. Spec OS dan scope 012 yang menjalankannya. Catatan itu memuat semua sumber, jadi satu perubahan menandai ekspor setiap OS sebagai basi, dan ia tidak menangkap keluaran biner yang disunting tangan.
* Pemeriksaan isi SVG membaca teks dan tidak mengurai XML. Sumber yang bukan XML utuh bisa lolos, dan keterangan di `title` atau `desc` yang memuat kata seperti `href` ikut ditolak.
* Daftar elemen yang diizinkan sengaja sempit. SVG sah yang memakai `filter` atau `pattern` ditolak sampai spec ini diperluas.
* Sumber wajib SVG tanpa tulisan dan tanpa gaya tertanam. Bila user hanya punya PNG, spec ini harus diperbarui lebih dulu.
* Aturan bahwa nama sumber tidak memuat versi, ukuran, atau nama OS tidak diperiksa mesin. Itu ditinjau orang.
* Repo memuat ikon penanda sampai user menyerahkan yang nyata, dan aplikasi belum boleh dibagikan selama masih ada aset bertanda `placeholder`.
* Aturan ditulis di tiga tempat: spec ini, kode pemeriksa, dan `shared/assets/README.md`. Perubahan aturan wajib menyentuh ketiganya.

**Neutral**:

* `brand.json` dan skemanya terbaca hook Graphify sebagai sumber, dan SVG tercatat sebagai gambar. Keduanya tidak butuh entri `.graphifyignore`.
* `docs/shared-resources/README.md` dan `docs/architecture/README.md` biasanya terlarang bagi sub sesi. Milestone 2 butuh izin eksplisit di brief developer.
* Nama produk bukan milik spec ini. Baris Follow-up "Scope 011" di spec `macos/0001` yang menyebut nama tampil bundel sudah dijawab spec `_root/0004`.
* `/develop` menulis `verify.md` saat build. Spec ini tidak membuatnya.

## Follow-up

* [x] Jawaban atas Follow-up Scope 011 di spec `_root/0003` dan arahan sesi penulisnya: sumber aset menjadi modul `assets` di `desktop-codegen` dengan pola `Source`, uji `assets_contract.rs`, aturan keluaran O-1 sampai O-7, dan baris `.gitattributes` per jenis berkas.
* [x] Jawaban atas Follow-up Scope 011 di spec `_root/0004`: aset merek hanya memegang ikon dan berkas gambar. Nama produk tidak ada di `brand.json`, dan uji M-4 menolaknya.
* [x] Dijawab spec [`macos/0007`](../../macos/0007-aset-merek-bersama/index.md) (2026-11-15). Spec macOS untuk scope 011 (sesudah spec ini): ekspor `app-icon` ke `apps/macos/Resources/`, ikon aplikasi di proyek Xcode, alat ekspor dan `brand-export.txt` bila keluarannya biner, dan hal lain di tabel "Kontrak untuk spec OS". Spec itu juga memperbarui **AC-12** spec `macos/0001`, yang sekarang mengunci tepat 16 path dan tanpa folder `Resources`. Ini menjawab bagian ikon dari baris Follow-up "Scope 011" di spec `macos/0001`.
* [ ] Windows (menunggu host Windows): spec Windows untuk scope 011 menetapkan hal di tabel "Kontrak untuk spec OS" untuk resource aplikasi WinUI. `cargo test -p desktop-codegen --locked` di host itu (AC-12 spec `_root/0003`) ikut menjalankan `assets_contract`.
* [ ] Linux (menunggu host Linux): spec Linux untuk scope 011 menetapkan hal yang sama untuk direktori ikon tema `hicolor`. Salinan SVG adalah keluaran teks, jadi mengikuti **E-4**. Nama berkas ikon mengikuti id aplikasi yang diturunkan spec Linux dari `build.app_identifier`.
* [ ] Nilai dari user: berkas ikon aplikasi nyata dalam SVG (**AC-10**, milestone 3).
* [ ] Bila user hanya punya PNG, atau menyerahkan desain ikon yang berbeda untuk satu OS, perbarui spec ini lebih dulu: aturan format sumber, atau varian per OS di manifest dengan `schemaVersion` baru.
* [ ] Pemilik spec `_root/0003`: sumber `assets` menambah empat path di `tooling/codegen/` dan dua baris `.gitattributes`, jadi angka path di AC-1 spec itu dan baris `sources:` di usage ikut berubah. Bagian "Bentuk alat" di sini memakai API yang ada di `main` pada 2026-11-15. Bila nama paket, perintah, API `Source`, atau aturan keluaran berubah, bagian itu, **AC-3**, **AC-4**, dan **AC-5** diperbarui di sini.
* [ ] Bila kelak dibutuhkan keluaran biner dari alat bersama, atau pemeriksaan XML yang utuh, itu pembaruan spec `_root/0003` lebih dulu (alat sekarang hanya menulis teks dan tidak punya pengurai XML), lalu pembaruan spec ini.
* [ ] Huruf merek (Follow-up Scope 011 di spec `_root/0003`), logo di dalam jendela, ilustrasi, dan artwork lain menunggu kebutuhan produk. Berkas bukan SVG butuh pembaruan spec ini.
* [x] Scope 012: `scripts/validate.sh` dan CI memanggil `cargo run -p desktop-codegen --locked -- assets check`, dan menjalankan pemeriksaan ekspor basi untuk tiap folder yang punya `brand-export.txt`. Dijawab [spec `_root/0008`](../0008-build-dan-penerimaan-per-target/index.md): `validate.sh` menjalankan `assets check` dan pemeriksaan ekspor basi untuk tiap `brand-export.txt` yang dilacak (2026-11-15).
* [ ] Scope 002, belum dijawab spec [`_root/0007`](../0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md): aturan `A-1` sampai `A-13` di sana tidak memeriksa berkas gambar merek. Jadikan invarian pertama (tidak ada berkas gambar merek di luar `shared/assets/source/` dan folder keluaran aplikasi) pemeriksaan tetap.
* [ ] `/sync` atau lead, sesudah milestone 1: `AGENTS.md` root masih berkata sumber bersama adalah rencana. Cantumkan `shared/assets/`, perintah `assets check`, dan aturan "aset merek hanya dari user" di sana, dengan spec ini sebagai pemiliknya.
* [x] Lead: tambahkan tautan balik ke spec ini di berkas scope 011. Sudah ada di baris **Pointer** berkas scope 011.
* [ ] Opsional, tidak wajib: kolom "Dirinci di" baris macOS di tabel tujuan ekspor `shared/assets/README.md` boleh menaut ke spec `macos/0007`. Itu mengubah blok README di bagian "Isi file" spec ini dan berkas yang dikunci **AC-1** byte demi byte, jadi hanya lewat pembaruan spec ini bersama perubahan berkasnya, misalnya saat build macOS scope 011 sudah masuk.
