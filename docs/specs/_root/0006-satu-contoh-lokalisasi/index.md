# 0006. Satu contoh lokalisasi

**Scope**: [010 - Satu contoh lokalisasi](../../../scope/010%20-%20Satu%20contoh%20lokalisasi.md)
**Date**: 2026-11-15
**Status**: In Progress

## Summary

Spec ini menetapkan satu katalog bahasa bersama di `shared/localization/catalog.json` dan aturan yang dipakai ketiga aplikasi: kunci yang stabil, nama locale (penanda bahasa seperti `en` atau `id`), fallback (teks pengganti saat terjemahan belum ada), placeholder (bagian teks yang diisi saat aplikasi berjalan), deteksi terjemahan yang hilang, dan versi. Sebagai contoh, satu teks di layar sapaan pindah ke katalog: label tombol dengan kunci `greeting.submit`, `Greet` dalam bahasa Inggris dan `Sapa` dalam bahasa Indonesia. Katalog diperiksa oleh alat `desktop-codegen` dari spec `_root/0003`, dan resource bahasa di tiap OS dibuat alat itu, bukan disunting dengan tangan. Hasil `hello` tetap bahasa Inggris dan tidak ikut dilokalkan.

## Requirements

**User stories**:

* Sebagai developer aplikasi macOS, Windows, atau Linux, saya ingin satu katalog dengan aturan yang tertulis, supaya teks antarmuka saya memakai kunci dan terjemahan yang sama dengan dua aplikasi lain.
* Sebagai penerjemah atau penulis teks, saya ingin tahu persis apa yang boleh ada di sebuah teks dan apa yang terjadi bila terjemahan belum ada, supaya saya bisa menambah bahasa tanpa membaca kode.
* Sebagai pemeriksa, saya ingin satu pemeriksaan yang menyebut setiap terjemahan yang hilang dan setiap placeholder yang tidak cocok, tanpa membuka jendela.

**Acceptance criteria**:

Angka kriteria merujuk ke tabel di [Penerimaan dan bukti fondasi](../../../quality/README.md). Semua perintah dijalankan dari akar repo pada Mac koordinasi. Angka yang bertanda "pada akhir milestone 1" bertambah saat spec OS menambah keluaran.

* **AC-1** (kriteria 20, Done when "kunci yang sama"): katalog ada dan berisi tepat satu contoh. `git ls-files shared/localization` mencetak tepat tiga path: `shared/localization/README.md`, `shared/localization/catalog.json`, dan `shared/localization/catalog.schema.json`. Isi `catalog.json` dan `README.md` sama byte demi byte dengan bagian **Isi berkas**, diakhiri satu baris baru. Perintah `python3 -c 'import json;d=json.load(open("shared/localization/catalog.json"));print(d["schemaVersion"],d["sourceLocale"],",".join(d["locales"]),",".join(sorted(d["entries"])))'` mencetak tepat `1 en en,id greeting.submit`.
* **AC-2** (kriteria 19, baris "Sumber bersama" di tabel pemeriksaan): `cargo run -p desktop-codegen --locked -- localization check` keluar dengan kode 0. Pada akhir milestone 1 baris terakhirnya tepat `localization: ok, 1 keys, 2 locales, 0 missing translations, 1 outputs up to date`. Uji `real_catalog_resolves` lulus: katalog nyata terurai menjadi nilai di tabel **Nilai hasil**.
* **AC-3** (kriteria 19, kunci stabil): kunci yang melanggar aturan L-3 ditolak. Kasus 7 sampai 11 di tabel **Kasus sumber rusak** lulus. Kunci yang sah tampil apa adanya: `Entry.key` untuk katalog nyata adalah `greeting.submit`.
* **AC-4** (kriteria 19, nama locale): nama locale yang melanggar aturan L-4 atau L-5 ditolak. Kasus 12 sampai 19 lulus.
* **AC-5** (kriteria 19, placeholder; baris "Sumber bersama"): teks dan placeholder mengikuti aturan L-6 dan L-7. Kasus 20 sampai 30 lulus. Uji `placeholders_may_reorder` lulus: terjemahan yang urutan placeholder nya berbeda dari teks sumber diterima, dan `Entry.placeholders` adalah `name`, `app`, yaitu urutan kemunculan pertama di teks sumber.
* **AC-6** (Done when "deteksi terjemahan yang hilang"): setiap terjemahan yang hilang disebut, bukan hanya yang pertama, dan katalog yang tidak lengkap tidak bisa masuk `main`. Uji `missing_translations_are_listed_and_fall_back` lulus: untuk katalog K-2, `Catalog.missing` memuat tepat dua pasangan, `greeting.submit` dengan `id` dan `greeting.submit` dengan `ms`, dan ringkasan `plan` tepat `1 keys, 3 locales, 2 missing translations (greeting.submit id; greeting.submit ms)`. Uji `committed_catalog_is_complete` lulus untuk katalog nyata, jadi `cargo test --workspace --locked` gagal begitu satu terjemahan hilang.
* **AC-7** (Done when "aturan fallback"): teks yang hilang diganti teks bahasa sumber menurut aturan L-9, tidak pernah dengan kunci mentah atau teks kosong. Dalam uji `missing_translations_are_listed_and_fall_back`, teks `id` dan `ms` untuk `greeting.submit` bernilai `Greet` dengan asal `Origin::Fallback`, dan teks `en` berasal `Origin::Own`. Dalam uji `real_catalog_resolves`, semua teks berasal `Origin::Own`.
* **AC-8** (kriteria 19, versi dan bentuk): bentuk katalog tertutup dan versinya diperiksa. Kasus 1 sampai 6 dan kasus 31 lulus, lewat `cargo test -p desktop-codegen --locked` maupun `cargo test --workspace --locked`. Isi `shared/localization/catalog.schema.json` sama persis dengan bagian **Isi berkas**. `grep -c '"additionalProperties": false' shared/localization/catalog.schema.json` mencetak `2`, dan `grep -c '"\$schema": "./catalog.schema.json"' shared/localization/catalog.json` mencetak `1`.
* **AC-9** (baris "Sumber bersama", konsistensi keluaran generator): keluaran bisa dibuat ulang dan penyimpangan tertangkap. Angka keluaran dan angka uji di AC ini adalah keadaan pada akhir milestone 1 spec ini; sesudah spec [`macos/0006`](../../macos/0006-satu-contoh-lokalisasi/index.md) dibangun, `1 outputs` menjadi `3 outputs`. Pada working tree yang bersih, `cargo run -p desktop-codegen --locked -- localization generate` mencetak baris terakhir `localization: ok, 1 keys, 2 locales, 0 missing translations, 1 outputs, 0 written`, lalu `git diff --exit-code -- shared/localization` keluar dengan kode 0. Sesudah `rm shared/localization/catalog.schema.json`, perintah yang sama mencetak `wrote shared/localization/catalog.schema.json` dan baris terakhir yang berakhir `1 outputs, 1 written`, dan `git status --short` tidak mencetak apa pun. `git ls-files --eol 'shared/localization/*.json'` mencetak dua baris yang keduanya memuat `i/lf`, `w/lf`, dan `attr/text eol=lf`. Uji `committed_outputs_match_the_source` dan `generation_is_deterministic` lulus. `cargo test -p desktop-codegen --locked --test localization_contract` mencetak `test result: ok. 7 passed`.
* **AC-10** (kriteria 32): spec ini hanya menambah yang dibutuhkan contoh. Katalog nyata memuat tepat satu entri. `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, dan `cargo clippy --workspace --all-targets --locked -- -D warnings` keluar dengan kode 0. `cargo tree -p desktop-codegen --depth 1 --prefix none --locked` tetap mencetak tepat tiga baris, jadi alat tidak mendapat dependensi baru. `cargo tree -p desktop-core -e normal,build,dev --prefix none --locked` tetap mencetak tepat satu baris. `git grep --untracked -lF 'Hello, {' -- '*.rs'` tetap mencetak tepat satu baris, `crates/core/src/greeting.rs`. `git log --name-only --format= --grep='spec _root/0006 step' | sort -u` hanya mencetak path dari tabel **File yang boleh diubah tiap milestone**, jadi tidak ada path di bawah `crates/`, `bindings/`, `apps/`, `scripts/`, atau `.github/`, dan `Cargo.toml` serta `Cargo.lock` tidak berubah.
* **AC-11** (kriteria 19, "tersedia aturan"): aturan bisa ditemukan dari dokumen rujukan. `docs/shared-resources/README.md` dan `docs/architecture/README.md` memuat teks dari bagian **Perubahan dokumen rujukan**. `grep -c '0006-satu-contoh-lokalisasi' docs/shared-resources/README.md` mencetak 1 atau lebih, dan tautan itu mengarah ke berkas yang ada. Uji `workspace_layout`, yang mengunci tabel aturan arah dependensi, tetap lulus. Pada akhir milestone 2 spec ini uji itu ada di `crates/core/tests/` dan dijalankan dengan `cargo test -p desktop-core --locked`. Sejak spec [`_root/0007`](../0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md) (`c3448d9`) uji itu ada di `tooling/codegen/tests/workspace_layout.rs`, dan perintahnya `cargo test -p desktop-codegen --locked --test workspace_layout`.

Bagian Done when yang berbunyi "ketiga aplikasi menampilkan teks UI" hanya bisa dibuktikan di tiap OS. Bagian itu ditulis sebagai kewajiban spec OS di bagian **Kewajiban spec OS**, bukan sebagai AC di sini.

## Decision

**Chosen option**: Option 1: Satu katalog JSON netral untuk semua bahasa, resource native dibuat dari katalog itu

Semua teks antarmuka yang dilokalkan tinggal di satu berkas, `shared/localization/catalog.json`, yang memuat setiap kunci beserta teksnya dalam semua bahasa. Katalog itu menjadi sumber kedua bernama `localization` di alat `desktop-codegen`. Alat memeriksa katalog, mengisi terjemahan yang hilang dengan teks bahasa sumber, dan menulis berkas turunan yang dilacak git. Satu uji di `cargo test --workspace` menolak katalog di repo yang terjemahannya belum lengkap.

Keputusan turunannya:

| Hal | Keputusan |
| --- | --- |
| Teks contoh | Label tombol sapaan, kunci `greeting.submit`. `Name` tetap literal bahasa Inggris. `Desktop Foundation` adalah nama produk dan bukan isi katalog (spec `_root/0004`). |
| Bahasa | Bahasa sumber `en`. Bahasa contoh kedua `id`. Pasangan ini usulan, user boleh menggantinya (lihat Follow-up). |
| Bentuk kunci | Dua sampai lima segmen huruf kecil dan angka, dipisah titik. Kunci menyebut peran, bukan isi teks. |
| Nama locale | Tag bahasa gaya BCP 47 (standar penamaan bahasa) yang dipersempit: bahasa, lalu wilayah bila perlu, misalnya `id` atau `pt-BR`. |
| Placeholder | Bernama, ditulis `{nama}`. Nilainya selalu teks. Kurung kurawal literal belum didukung. |
| Terjemahan yang hilang | Dihitung dan disebut di ringkasan `check` dan `generate`, tanpa menggagalkan keduanya. Uji `committed_catalog_is_complete` menggagalkan `cargo test --workspace --locked`. |
| Fallback | Per teks: teks bahasa sumber, diisikan alat. Per pengguna: bahasa pilihan pengguna dicocokkan dengan memotong wilayah, lalu jatuh ke bahasa sumber. |
| Versi | `schemaVersion` bilangan bulat, sekarang `1`, sama dengan sumber lain di alat. Perubahan isi yang memutus dicatat di tabel **Riwayat katalog**. |
| Skema | `shared/localization/catalog.schema.json`, JSON Schema draft-07, keluaran generator. Hanya mencakup bentuk. |
| Angka, tanggal, bentuk jamak | Mengikuti OS dan bukan isi katalog. |
| Hasil `hello` | Tidak dilokalkan (spec `_root/0002`). Core tidak membaca katalog. |
| Keluaran per OS | Tiap spec OS menambah satu modul emitter (penulis berkas untuk satu target) di sumber `localization`. Spec ini hanya menulis skema. |

Nilai dari spec lain yang dipakai di sini, tanpa diputuskan ulang:

| Nilai | Isi | Pemilik |
| --- | --- | --- |
| Alat | `tooling/codegen/`, paket `desktop-codegen`, perintah `cargo run -p desktop-codegen --locked -- <sumber> <check\|generate>`, kode keluar `0`, `1`, `2` | Spec [`_root/0003`](../0003-tema-bersama-dan-kontrol-native/index.md) |
| Pola sumber | Modul `tooling/codegen/src/<sumber>/mod.rs` dengan `pub const SOURCE`, dua baris di `lib.rs`, uji di `tooling/codegen/tests/<sumber>_contract.rs` | Spec `_root/0003` |
| Tipe dan fungsi alat | `Source`, `Plan`, `Output`, `Problem`, `check`, `generate`, `repo_root`, `json::UniqueMap`, `json::to_canonical_json` | Spec `_root/0003`, bagian API surface |
| Aturan keluaran | O-1 sampai O-7 (penanda, isi tetap, UTF-8 dengan `\n`, urutan tetap, lokasi tetap, dilacak git, satu pintu) | Spec `_root/0003` |
| Dua anggota pembuka sumber | `"$schema"` dan `"schemaVersion": 1` | Spec `_root/0003` |
| Pola deskripsi | `^[A-Za-z0-9 ,.()-]{1,120}$` | Spec `_root/0003`, aturan T-4 |
| Lokasi katalog | `shared/localization/` | [Arsitektur](../../../architecture/README.md), tabel "Tata letak yang dituju" |
| Teks bahasa Inggris tombol | `Greet` | Spec [macos/0001](../../macos/0001-sapaan-di-jendela-macos/index.md), baris "Teks antarmuka" |
| Hasil `hello` selalu bahasa Inggris dan tampil sebagai teks biasa | `Hello, {name}!`, aturan R-8 | Spec [_root/0002](../0002-kontrak-sapaan-bersama/index.md) |
| Aturan workspace | Tanpa `.cargo/config.toml`, paket berawalan `desktop-`, aturan netral host | Spec [_root/0001](../0001-batas-workspace-dan-pemilik/index.md) |

Spec [`_root/0003`](../0003-tema-bersama-dan-kontrol-native/index.md) sudah ada di `main` dengan status `Proposed`. Alatnya sendiri baru ada sesudah milestone 1 spec itu dibangun.

## Rationale

Alasan, pilihan lain, dan daftar keputusan beserta pilihan keduanya: lihat [rationale.md](rationale.md).

## Feature design

**Data model sketch**:

Satu berkas JSON, `shared/localization/catalog.json`. Tidak ada basis data dan tidak ada data pengguna. Setiap objek tertutup: anggota di luar tabel ini ditolak.

| Anggota | Tipe | Wajib | Isi |
| --- | --- | --- | --- |
| `$schema` | teks | ya | Tepat `./catalog.schema.json`, sama bentuknya dengan `shared/config/public.json` |
| `schemaVersion` | bilangan bulat | ya | Tepat `1` (L-10) |
| `sourceLocale` | teks | ya | Nama locale yang sah dan terdaftar di `locales` (L-4, L-5) |
| `locales` | larik teks | ya | Paling sedikit satu, tanpa pengulangan, setiap isinya nama locale yang sah (L-4). Urutannya menjadi urutan keluaran. |
| `entries` | objek | ya | Nama anggota adalah kunci (L-3). Boleh kosong. |
| `entries.<kunci>.description` | teks | ya | Peran teks di layar, untuk penerjemah. Bahasa Inggris, cocok dengan pola deskripsi. |
| `entries.<kunci>.text` | objek | ya | Nama anggota adalah locale yang terdaftar. Nilainya teks (L-6, L-7). Wajib memuat `sourceLocale` (L-5). |

**Nilai hasil** (katalog nyata sesudah diurai, pada akhir milestone 1):

| Kunci | Placeholder | `en` | `id` |
| --- | --- | --- | --- |
| `greeting.submit` | tidak ada | `Greet`, asal `Own` | `Sapa`, asal `Own` |

`Catalog.source_locale` adalah `en`, `Catalog.locales` adalah `en`, `id`, dan `Catalog.missing` kosong.

**Isi berkas**:

`shared/localization/catalog.json`:

```json
{
  "$schema": "./catalog.schema.json",
  "schemaVersion": 1,
  "sourceLocale": "en",
  "locales": ["en", "id"],
  "entries": {
    "greeting.submit": {
      "description": "Label of the button that sends the name and shows the greeting.",
      "text": {
        "en": "Greet",
        "id": "Sapa"
      }
    }
  }
}
```

`shared/localization/README.md`:

```markdown
# Katalog bahasa bersama

`catalog.json` adalah satu satunya sumber teks antarmuka yang dilokalkan untuk aplikasi macOS, Windows, dan Linux. `catalog.schema.json` dan resource bahasa di tiap aplikasi dibuat dari berkas ini oleh `desktop-codegen`. Jangan menyunting hasil generator.

Cara mengubah teks: sunting `catalog.json`, jalankan `cargo run -p desktop-codegen --locked -- localization generate`, lalu `cargo run -p desktop-codegen --locked -- localization check`, dan commit katalog bersama berkas turunannya.

Aturan kunci, nama locale, placeholder, fallback, deteksi terjemahan yang hilang, dan versi ada di [spec 0006](../../docs/specs/_root/0006-satu-contoh-lokalisasi/index.md), bagian "Aturan lokalisasi bersama".
```

`shared/localization/catalog.schema.json` adalah hasil `localization generate`. Kamu tidak mengetiknya. Cocokkan hasil generatormu dengan blok ini. Modul privat `localization/schema.rs` membangunnya dari konstanta `SCHEMA_VERSION` dan tiga pola yang juga dipakai `parse`, lalu menulisnya dengan `to_canonical_json`:

```json
{
  "$comment": "Generated by desktop-codegen from tooling/codegen/src/localization. Do not edit. To regenerate, run: cargo run -p desktop-codegen --locked -- localization generate",
  "$schema": "http://json-schema.org/draft-07/schema#",
  "additionalProperties": false,
  "properties": {
    "$schema": {
      "const": "./catalog.schema.json"
    },
    "entries": {
      "additionalProperties": {
        "additionalProperties": false,
        "properties": {
          "description": {
            "pattern": "^[A-Za-z0-9 ,.()-]{1,120}$",
            "type": "string"
          },
          "text": {
            "additionalProperties": {
              "minLength": 1,
              "type": "string"
            },
            "propertyNames": {
              "pattern": "^[a-z]{2,3}(-[A-Z]{2})?$"
            },
            "type": "object"
          }
        },
        "required": [
          "description",
          "text"
        ],
        "type": "object"
      },
      "propertyNames": {
        "pattern": "^[a-z][a-z0-9]*(\\.[a-z][a-z0-9]*){1,4}$"
      },
      "type": "object"
    },
    "locales": {
      "items": {
        "pattern": "^[a-z]{2,3}(-[A-Z]{2})?$",
        "type": "string"
      },
      "minItems": 1,
      "type": "array",
      "uniqueItems": true
    },
    "schemaVersion": {
      "const": 1
    },
    "sourceLocale": {
      "pattern": "^[a-z]{2,3}(-[A-Z]{2})?$",
      "type": "string"
    }
  },
  "required": [
    "$schema",
    "schemaVersion",
    "sourceLocale",
    "locales",
    "entries"
  ],
  "title": "Localization catalog",
  "type": "object"
}
```

Skema hanya mencakup bentuk. Aturan teks dan placeholder (L-6, L-7) serta aturan lintas nilai (L-5) hanya diperiksa `localization check`.

`.gitattributes` di akar mendapat satu baris baru di akhir. Baris yang sudah ada tidak diubah. Polanya per jenis berkas, mengikuti aturan O-6 spec `_root/0003`:

```gitattributes
/shared/localization/**/*.json text eol=lf
```

`tooling/codegen/src/lib.rs` mendapat dua perubahan. Baris `pub mod localization;` ditambahkan di antara deklarasi `pub mod` yang sudah ada, dalam urutan abjad. `localization::SOURCE` ditambahkan sebagai entri terakhir `SOURCES`. Tidak ada baris lain yang berubah.

Teks `Sapa` dan pasangan bahasa `en` dan `id` adalah usulan. User boleh mengganti nilainya tanpa mengubah aturan. Bila nilainya diganti, isi katalog, tabel **Nilai hasil**, **AC-1**, **AC-2**, dan kasus uji yang menyebut teks itu mengikuti isi yang baru lewat pembaruan spec ini.

**State transitions**:

Tidak ada. Katalog adalah berkas statis. Bahasa yang dipakai aplikasi dipilih saat aplikasi dibuka dari pengaturan OS dan tidak disimpan aplikasi.

**Aturan lokalisasi bersama**:

Aturan ini berlaku untuk katalog, alat, dan ketiga aplikasi. Kata "teks" berarti nilai di dalam `text` untuk satu locale. Setiap pola dicocokkan dengan seluruh string. Alat tidak punya crate regex, jadi pola diperiksa dengan kode biasa.

* **L-1 Satu sumber.** Teks antarmuka yang dilokalkan hanya ditulis di `catalog.json`. Resource bahasa di tiap OS adalah turunan. Teks antarmuka yang belum masuk katalog tetap literal dalam bahasa sumber di kode UI. Hasil fungsi Core, termasuk `hello`, bukan teks antarmuka dan tidak pernah masuk katalog. Nama produk juga tidak masuk katalog: nilainya sama untuk semua bahasa dan dimiliki konfigurasi publik (spec [`_root/0004`](../0004-konfigurasi-publik/index.md), `build.product_name`).
* **L-2 Bentuk berkas.** Berkas adalah satu objek JSON dengan anggota persis seperti tabel **Data model sketch**. Anggota yang tidak dikenal, tipe yang salah, anggota wajib yang tidak ada, dan nama anggota yang ditulis dua kali ditolak. Bilangan bulat berarti angka JSON tanpa pecahan: `1` sah, sedangkan `1.0`, `"1"`, dan `true` tidak.
* **L-3 Kunci stabil.** Kunci cocok dengan `^[a-z][a-z0-9]*(\.[a-z][a-z0-9]*){1,4}$`: dua sampai lima segmen, tiap segmen huruf kecil ASCII dan angka, diawali huruf, dipisah titik. Segmen pertama menyebut layar atau area, segmen berikutnya menyebut elemen dan perannya, bukan isi teksnya (`greeting.submit`, bukan `greeting.greet`). Kunci tidak diganti namanya dan tidak dipakai ulang untuk makna lain. Mengubah kata kata tanpa mengubah makna mempertahankan kunci. Makna baru berarti kunci baru. Ketiga aplikasi mencari teks dengan kunci yang sama (lihat L-11 untuk bentuk pengenalnya di resource).
* **L-4 Nama locale.** Nama locale cocok dengan `^[a-z]{2,3}(-[A-Z]{2})?$`: kode bahasa huruf kecil, lalu bila perlu tanda `-` dan kode wilayah dua huruf besar. Contoh yang sah: `en`, `id`, `pt-BR`, `zh-TW`. Aksara, wilayah berupa angka, varian, dan ekstensi belum didukung. Emitter memetakan nama ini ke kebiasaan tiap OS (misalnya `pt_BR` untuk gettext). Pemetaan itu ditulis di spec OS dan wajib satu ke satu: dua nama locale katalog tidak boleh menjadi nama OS yang sama.
* **L-5 Bahasa sumber dan daftar bahasa.** `locales` adalah daftar bahasa yang didukung aplikasi. `sourceLocale` wajib ada di daftar itu dan menjadi bahasa bawaan setiap aplikasi. Setiap entri wajib punya teks untuk `sourceLocale`. Teks untuk locale yang tidak terdaftar ditolak.
* **L-6 Teks biasa.** Teks tidak boleh kosong dan tidak boleh hanya berisi spasi U+0020. Teks tidak boleh memuat karakter kendali U+0000 sampai U+001F atau U+007F, termasuk tab dan baris baru. Teks adalah teks biasa: bukan Markdown, bukan markup, bukan pola format milik satu OS. Karakter seperti `*`, `_`, `<`, `&`, dan `%` tampil apa adanya. Emitter dan aplikasi wajib menjaga hal itu, sehingga teks yang terlihat sama dengan teks katalog sesudah placeholder diisi.
* **L-7 Placeholder.** Placeholder ditulis `{nama}` dengan nama yang cocok dengan `^[a-z][a-z0-9_]*$`. Setiap `{` wajib mengawali placeholder yang sah dan setiap `}` wajib menutupnya. Kurung kurawal lain ditolak, jadi kurung kurawal literal belum bisa ditulis. Himpunan nama placeholder di setiap terjemahan wajib sama dengan himpunan di teks sumber. Urutan boleh berbeda dan satu nama boleh muncul lebih dari sekali. Nilai placeholder selalu teks. Aplikasi mengubah angka dan tanggal menjadi teks dengan API OS sebelum mengisinya. Untuk format native yang memakai nomor urut, urutan argumen adalah urutan kemunculan pertama tiap nama di teks sumber (`Entry.placeholders`). Nilai diisikan sebagai teks biasa dan tidak pernah ditafsirkan sebagai pola format atau markup.
* **L-8 Terjemahan yang hilang.** Sebuah teks hilang untuk locale `L` bila `L` terdaftar di `locales`, `L` bukan `sourceLocale`, dan entri itu tidak punya anggota `L` di dalam `text`. Terjemahan yang hilang bukan masalah bentuk: `parse` tetap berhasil dan mencatat setiap pasangan di `Catalog.missing`. Ringkasan `check` dan `generate` menyebut jumlahnya dan setiap pasangannya. Katalog di `main` wajib tanpa terjemahan yang hilang, dan uji `committed_catalog_is_complete` menjaganya.
* **L-9 Fallback.** Ada dua tingkat.
  * Per teks, dikerjakan alat: untuk kunci `K` dan locale `L`, `parse` memakai teks `L` dengan asal `Own`. Bila teks itu hilang (L-8), `parse` memakai teks `sourceLocale` dengan asal `Fallback`. Jadi setiap entri di `Catalog` punya satu teks untuk setiap locale, emitter menulis resource yang selalu lengkap, dan aplikasi tidak pernah menampilkan kunci mentah atau teks kosong.
  * Per pengguna, saat aplikasi dibuka: aplikasi mendaftarkan `locales` sebagai bahasa yang didukung dan `sourceLocale` sebagai bahasa bawaan. Hasil minimumnya sama di setiap OS. Untuk setiap bahasa pilihan pengguna menurut urutannya, dipakai locale katalog yang sama persis. Bila tidak ada, wilayahnya dipotong dan dicoba lagi (`id-ID` memakai `id`). Pilihan pertama yang cocok menang. Bila tidak ada satu pun yang cocok, dipakai `sourceLocale`. OS yang punya negosiasi bahasa sendiri untuk bentuk resource nya memakai negosiasi itu, selama dua hasil ini terpenuhi: `id-ID` memakai `id`, dan pengguna tanpa pilihan yang cocok melihat `sourceLocale`. OS atau bentuk resource yang tidak punya negosiasi (misalnya adapter Rust, atau mekanisme yang mengembalikan kunci saat bahasa tidak cocok) menerapkan langkah di atas sendiri dengan daftar pilihan bahasa dari OS. Perilaku di luar itu dicatat di spec OS. Aplikasi tidak punya pengalih bahasa sendiri.
* **L-10 Versi.** `schemaVersion` menyebut versi bentuk berkas dan tata tulis teks (L-2 sampai L-7). Nilainya sekarang `1`, dan alat menolak angka lain. Perubahan yang membuat katalog lama tidak lagi sah menaikkan `SCHEMA_VERSION` dan `schemaVersion` dalam commit yang sama. Riwayat isi dilacak git. Perubahan isi yang memutus adalah menghapus kunci, mengubah himpunan placeholder sebuah kunci, menghapus locale dari `locales`, dan mengganti `sourceLocale`. Perubahan seperti itu dicatat di tabel **Riwayat katalog** dan menyertakan perubahan di setiap aplikasi yang memakai kunci itu.
* **L-11 Keluaran per OS.** Resource bahasa tiap OS ditulis emitter dari `Catalog` (hasil L-9 tingkat per teks) dan memenuhi lima hal, di samping aturan keluaran O-1 sampai O-7 spec `_root/0003`:
  1. Memuat setiap kunci untuk setiap locale di `locales`, termasuk `sourceLocale`. Resource bahasa sumber selalu ditulis, tidak dianggap sudah ada.
  2. Memakai kunci katalog sebagai pengenal, apa adanya. Bila format OS tidak menerima titik di pengenal, setiap `.` diganti `_`. Tidak ada pemetaan lain.
  3. Mengubah placeholder ke bentuk native menurut L-7.
  4. Meloloskan (escape) karakter yang punya arti khusus di format native, misalnya `%`, supaya L-6 terpenuhi.
  5. Tinggal di folder aplikasi OS itu, bukan di `shared/localization/`.

  Format resource, lokasi persisnya, dan cara aplikasi membacanya ditulis di spec OS.
* **L-12 Yang mengikuti OS.** Format angka, tanggal, dan bentuk jamak memakai API OS dan tidak ditulis di katalog. Teks yang kata katanya bergantung pada jumlah (bentuk jamak) belum boleh masuk katalog. Core tidak mengenal locale: `desktop-core` tidak membaca katalog dan tidak mendapat parameter bahasa.

| Target | Bentuk keluaran (dari [Resource bersama](../../../shared-resources/README.md)) | Yang diputuskan spec OS |
| --- | --- | --- |
| macOS | Resource lokalisasi Apple | Format berkas, lokasi di `apps/macos/`, cara SwiftUI membaca kunci, pemetaan nama locale |
| Windows | Adapter lokalisasi Rust untuk presentasi WinUI | Bentuk adapter, lokasi di `apps/windows/`, cara membaca daftar bahasa pilihan dari Windows |
| Linux | Resource lokalisasi yang sesuai GTK | Format berkas, lokasi di `apps/linux/`, pemetaan nama locale, cara memuat bahasa sumber sebagai pilihan terakhir |

**API surface**:

Baris perintah. Perilaku `check`, `generate`, kode keluar, dan bentuk baris gagal adalah milik spec `_root/0003` dan sama untuk setiap sumber.

| Perintah | Masukan | Keluaran saat sukses (stdout) | Kode | Saat gagal (stderr) |
| --- | --- | --- | --- | --- |
| `localization check` | `catalog.json` dan berkas keluaran di disk | `localization: ok, <ringkasan>, <n> outputs up to date` | `0` | Satu baris per masalah, lalu `localization: failed, <n> problems`, kode `1` |
| `localization generate` | `catalog.json` | Satu baris `wrote <path>` per berkas yang ditulis, lalu `localization: ok, <ringkasan>, <n> outputs, <k> written` | `0` | Sama, kode `1`, tidak ada yang ditulis |

`<ringkasan>` adalah `<k> keys, <l> locales, <m> missing translations`, dengan `<k>` jumlah entri, `<l>` jumlah isi `locales`, dan `<m>` jumlah pasangan di `Catalog.missing`. Kata `keys`, `locales`, dan `translations` selalu jamak. Bila `<m>` lebih dari nol, ringkasan disambung satu spasi dan daftar dalam kurung: tiap pasangan ditulis `<kunci> <locale>`, dipisah `; `, dalam urutan `Catalog.missing`. Contoh: `1 keys, 3 locales, 2 missing translations (greeting.submit id; greeting.submit ms)`.

Terjemahan yang hilang tidak membuat `check` atau `generate` gagal (L-8). Yang membuat keduanya gagal adalah masalah di tabel **Aturan sumber**, dan untuk `check` juga keluaran yang menyimpang.

Permukaan Rust. Tulis tanda tangan ini persis. Setiap item publik punya rustdoc berbahasa Inggris (`missing_docs`). `Text`, `Entry`, `Missing`, dan `Catalog` menurunkan `Debug`, `Clone`, `PartialEq`, dan `Eq`. `Origin` juga menurunkan `Copy`.

```rust
// tooling/codegen/src/localization/mod.rs
pub const SOURCE: Source;                            // name: "localization"
pub const SCHEMA_VERSION: u64 = 1;
pub const CATALOG_PATH: &str = "shared/localization/catalog.json";
pub const SCHEMA_PATH: &str = "shared/localization/catalog.schema.json";
pub enum Origin { Own, Fallback }
pub struct Text { pub locale: String, pub value: String, pub origin: Origin }
pub struct Entry { pub key: String, pub description: String, pub placeholders: Vec<String>, pub texts: Vec<Text> }
pub struct Missing { pub key: String, pub locale: String }
pub struct Catalog { pub source_locale: String, pub locales: Vec<String>, pub entries: Vec<Entry>, pub missing: Vec<Missing> }
pub fn parse(catalog: &str) -> Result<Catalog, Vec<Problem>>;
```

Perilaku:

| Item | Perilaku |
| --- | --- |
| `parse` | Fungsi murni atas teks `catalog.json`. Menjalankan **Urutan pemeriksaan**. Setiap `Problem` memakai `CATALOG_PATH` sebagai path. Bila tanpa masalah, mengembalikan `Catalog` yang sudah diisi fallback. |
| `Catalog.locales` | Isi `locales` dalam urutan berkas. |
| `Catalog.entries` | Terurut menurut `key` dalam urutan byte. |
| `Entry.texts` | Tepat satu `Text` untuk tiap locale, dalam urutan `Catalog.locales`. `origin` adalah `Own` bila entri punya teks untuk locale itu. Bila tidak, `value` adalah teks `sourceLocale` dan `origin` adalah `Fallback`. |
| `Entry.placeholders` | Nama placeholder teks sumber, tiap nama sekali, dalam urutan kemunculan pertama. |
| `Catalog.missing` | Satu `Missing` untuk tiap teks berasal `Fallback`, terurut menurut `key` dalam urutan byte, lalu menurut urutan `Catalog.locales`. |
| `SOURCE.plan` | Membaca `CATALOG_PATH` dari `root`. Berkas yang tidak terbaca memberi `<path>: cannot read: <kesalahan OS>`. Lalu `parse`, lalu menyusun keluaran: satu berkas, `SCHEMA_PATH`. Emitter OS ditambahkan sesudah skema. `Plan.summary` adalah `<ringkasan>` di atas. |

Kedua berkas modul adalah `tooling/codegen/src/localization/mod.rs` dan `tooling/codegen/src/localization/schema.rs` (modul privat, penulis skema).

**Urutan pemeriksaan**:

Urutan ini menentukan masalah mana yang muncul dan urutannya di `Vec<Problem>`.

1. Teks diurai ke tipe `serde` tertutup (`deny_unknown_fields`, `u64` untuk `schemaVersion`, `UniqueMap` untuk `entries` dan `text`). Gagal: satu masalah berisi pesan `serde_json`, selesai.
2. `$schema` lalu `schemaVersion` diperiksa. Bila ada masalah di langkah ini, selesai.
3. Nama locale: tiap isi `locales` menurut urutannya (pola, lalu pengulangan), `locales` yang kosong, lalu `sourceLocale` (pola, dan bila polanya cocok, keanggotaan di `locales`). Bila ada masalah di langkah ini, entri tidak diperiksa.
4. Tiap entri menurut urutan byte kuncinya, dengan butir berikut menurut urutannya:
   1. Kunci tidak cocok dengan pola. Entri itu tidak diperiksa lebih jauh.
   2. `description` tidak cocok dengan pola.
   3. Tiap teks untuk locale yang tidak terdaftar, menurut urutan byte nama locale. Teks itu diabaikan di butir berikutnya.
   4. Tidak ada teks `sourceLocale`. Entri itu tidak diperiksa lebih jauh.
   5. Tiap teks untuk locale yang terdaftar, menurut urutan `locales`: teks kosong, atau bila tidak, karakter kendali, atau bila tidak, kurung kurawal yang salah. Paling banyak satu masalah untuk satu teks. Teks yang kena salah satunya tidak ikut butir 6.
   6. Bila teks sumber lolos butir 5: tiap terjemahan yang lolos butir 5, menurut urutan `locales`, dibandingkan himpunan placeholder nya dengan teks sumber.

**Aturan sumber** (pesan ditulis persis, `<...>` diganti nilainya; setiap pesan diawali path berkas, misalnya `shared/localization/catalog.json: schemaVersion: expected 1, found 2`):

| Aturan | Yang ditolak | Pesan |
| --- | --- | --- |
| L-2 | Bentuk di luar tabel **Data model sketch** | Pesan `serde_json` apa adanya, misalnya ``unknown field `notes`, expected one of ...``, ``missing field `text` ``, ``invalid type: floating point `1.0`, expected u64``. Untuk kunci atau locale yang ditulis dua kali: ``duplicate key `<nama>` `` |
| L-10 | `$schema` atau `schemaVersion` tidak tepat | `$schema: expected ./catalog.schema.json, found <nilai>` dan `schemaVersion: expected 1, found <angka>` |
| L-4 | Nama locale tidak cocok dengan pola | `locales.<nama>: locale must match ^[a-z]{2,3}(-[A-Z]{2})?$` dan `sourceLocale: locale must match ^[a-z]{2,3}(-[A-Z]{2})?$` |
| L-5 | `locales` kosong atau berulang | `locales: needs at least one locale` dan `locales: <nama> is listed twice` (sekali untuk tiap nama yang berulang) |
| L-5 | `sourceLocale` tidak terdaftar | `sourceLocale: <nama> is not listed in locales` |
| L-3 | Kunci tidak cocok dengan pola | `entries.<kunci>: key must match ^[a-z][a-z0-9]*(\.[a-z][a-z0-9]*){1,4}$` |
| T-4 spec `_root/0003` | Deskripsi tidak cocok dengan pola | `entries.<kunci>.description: must match ^[A-Za-z0-9 ,.()-]{1,120}$` |
| L-5 | Teks untuk locale yang tidak terdaftar | `entries.<kunci>.text.<locale>: locale is not listed in locales` |
| L-5 | Entri tanpa teks bahasa sumber | `entries.<kunci>.text: missing source text for <sourceLocale>` |
| L-6 | Teks kosong atau hanya spasi | `entries.<kunci>.text.<locale>: text is empty` |
| L-6 | Teks memuat karakter kendali | `entries.<kunci>.text.<locale>: text contains a control character` |
| L-7 | Kurung kurawal yang tidak membentuk placeholder | `entries.<kunci>.text.<locale>: braces must form {name} placeholders` |
| L-7 | Himpunan placeholder berbeda dari teks sumber | `entries.<kunci>.text.<locale>: placeholders differ from <sourceLocale>: expected <daftar>, found <daftar>` |

`<daftar>` adalah nama placeholder tanpa kurung, tiap nama sekali, terurut dalam urutan byte, dipisah `, `. Himpunan kosong ditulis `none`.

**Uji kontrak** (`tooling/codegen/tests/localization_contract.rs`, hanya memakai path publik `desktop_codegen`, dibuka dengan komentar dokumen `//!`; "katalog nyata" berarti teks `shared/localization/catalog.json` yang dibaca saat uji berjalan):

| Uji | Yang dibuktikan | Langkah |
| --- | --- | --- |
| `committed_outputs_match_the_source` | `check(&repo_root(), localization)` lulus dan barisnya diawali `localization: ok, ` | 1 |
| `committed_catalog_is_complete` | `parse` atas katalog nyata memberi `missing` yang kosong. Pesan gagalnya menyebut setiap pasangan kunci dan locale yang hilang. | 1 |
| `real_catalog_resolves` | `parse` atas katalog nyata memberi nilai di tabel **Nilai hasil** | 1 |
| `generation_is_deterministic` | Dua panggilan `plan` memberi `Plan` yang sama. Tiap keluaran berakhir `\n`, tidak memuat `\r`, tidak memuat path absolut akar repo, dan memuat `Generated by desktop-codegen` serta `Do not edit` | 1 |
| `placeholders_may_reorder` | `parse` atas katalog K-1 lulus tanpa masalah, `placeholders` adalah `name`, `app`, dan `missing` kosong | 2 |
| `missing_translations_are_listed_and_fall_back` | `parse` atas katalog K-2 lulus tanpa masalah dan memberi nilai di **Katalog uji**. Di salinan sumber di bawah `env!("CARGO_TARGET_TMPDIR")` yang berisi K-2, `(SOURCE.plan)(root)` memberi `summary` yang tertulis di **Katalog uji**. | 2 |
| `rejects_broken_catalogs` | 31 kasus di tabel **Kasus sumber rusak** | 2 |

**Katalog uji**. Tidak ada folder fixture. Tiap katalog uji dibuat di dalam uji dengan mengganti potongan teks katalog nyata (kemunculan pertama).

| ID | Perubahan pada katalog nyata | Hasil `parse` |
| --- | --- | --- |
| K-1 | `"Greet"` menjadi `"{name}, welcome to {app}."`, dan `"Sapa"` menjadi `"Selamat datang di {app}, {name}."` | Tanpa masalah. Satu entri, `placeholders` `name`, `app`. Kedua teks berasal `Own`. `missing` kosong. |
| K-2 | `["en", "id"]` menjadi `["en", "id", "ms"]`, dan baris `"id": "Sapa"` dihapus bersama koma di akhir baris `"en": "Greet",` | Tanpa masalah. `locales` `en`, `id`, `ms`. Teks `en` `Greet` berasal `Own`. Teks `id` dan `ms` bernilai `Greet` dan berasal `Fallback`. `missing` berisi `greeting.submit` dengan `id`, lalu `greeting.submit` dengan `ms`. Ringkasan `plan`: `1 keys, 3 locales, 2 missing translations (greeting.submit id; greeting.submit ms)`. |

**Kasus sumber rusak**. Tiap kasus mengganti satu potong teks di katalog nyata (kemunculan pertama), memanggil `parse`, dan mencari masalah yang `Display` nya memuat teks di kolom terakhir. Kasus 2, 4, dan 6 memakai teks dari `serde` dan `serde_json`. Bila teks itu berubah saat versinya naik, perbarui tabel ini.

| No. | Perubahan | Pesan memuat |
| --- | --- | --- |
| 1 | `"schemaVersion": 1` menjadi `"schemaVersion": 2` | `catalog.json: schemaVersion: expected 1, found 2` |
| 2 | `"schemaVersion": 1` menjadi `"schemaVersion": 1.0` | ``catalog.json: invalid type: floating point `1.0`, expected u64`` |
| 3 | `"$schema": "./catalog.schema.json"` menjadi `"$schema": "other.json"` | `catalog.json: $schema: expected ./catalog.schema.json, found other.json` |
| 4 | tambah `"notes": "x",` sesudah `"schemaVersion": 1,` | ``catalog.json: unknown field `notes` `` |
| 5 | `"id": "Sapa"` menjadi `"en": "Sapa"` | ``catalog.json: duplicate key `en` `` |
| 6 | `["en", "id"]` menjadi `"en"` | `catalog.json: invalid type: string "en", expected a sequence` |
| 7 | `"greeting.submit"` menjadi `"Greeting.submit"` | `catalog.json: entries.Greeting.submit: key must match` |
| 8 | `"greeting.submit"` menjadi `"greeting"` | `catalog.json: entries.greeting: key must match` |
| 9 | `"greeting.submit"` menjadi `"greeting..submit"` | `catalog.json: entries.greeting..submit: key must match` |
| 10 | `"greeting.submit"` menjadi `"greeting.submit_label"` | `catalog.json: entries.greeting.submit_label: key must match` |
| 11 | `"greeting.submit"` menjadi `"greeting.1st"` | `catalog.json: entries.greeting.1st: key must match` |
| 12 | `["en", "id"]` menjadi `["en", "ID"]` | `catalog.json: locales.ID: locale must match` |
| 13 | `["en", "id"]` menjadi `["en", "pt_BR"]` | `catalog.json: locales.pt_BR: locale must match` |
| 14 | `["en", "id"]` menjadi `["en", "zh-Hans"]` | `catalog.json: locales.zh-Hans: locale must match` |
| 15 | `["en", "id"]` menjadi `["en", "en"]` | `catalog.json: locales: en is listed twice` |
| 16 | `["en", "id"]` menjadi `[]` | `catalog.json: locales: needs at least one locale` |
| 17 | `"sourceLocale": "en"` menjadi `"sourceLocale": "fr"` | `catalog.json: sourceLocale: fr is not listed in locales` |
| 18 | `"id": "Sapa"` menjadi `"fr": "Saluer"` | `catalog.json: entries.greeting.submit.text.fr: locale is not listed in locales` |
| 19 | baris `"en": "Greet",` dihapus | `catalog.json: entries.greeting.submit.text: missing source text for en` |
| 20 | `"Sapa"` menjadi `""` | `catalog.json: entries.greeting.submit.text.id: text is empty` |
| 21 | `"Sapa"` menjadi `"   "` (tiga spasi) | `catalog.json: entries.greeting.submit.text.id: text is empty` |
| 22 | `"Sapa"` menjadi `"Sa\tpa"` (escape JSON untuk tab) | `catalog.json: entries.greeting.submit.text.id: text contains a control character` |
| 23 | `"Greet"` menjadi `"Hi {"` | `catalog.json: entries.greeting.submit.text.en: braces must form {name} placeholders` |
| 24 | `"Greet"` menjadi `"Hi }"` | sama dengan kasus 23 |
| 25 | `"Greet"` menjadi `"Hi {Name}"` | sama dengan kasus 23 |
| 26 | `"Greet"` menjadi `"Hi {}"` | sama dengan kasus 23 |
| 27 | `"Greet"` menjadi `"Hi {na me}"` | sama dengan kasus 23 |
| 28 | `"Greet"` menjadi `"Hi {{name}}"` | sama dengan kasus 23 |
| 29 | `"Greet"` menjadi `"Hi {name}"` | `catalog.json: entries.greeting.submit.text.id: placeholders differ from en: expected name, found none` |
| 30 | `"Sapa"` menjadi `"Hai {user}"` | `catalog.json: entries.greeting.submit.text.id: placeholders differ from en: expected none, found user` |
| 31 | `shows the greeting.` menjadi `shows the greeting */` | `catalog.json: entries.greeting.submit.description: must match` |

**Value sourcing**:

| Aksi | Nilai yang dihasilkan atau ditampilkan | Sumber |
| --- | --- | --- |
| Aplikasi menampilkan label tombol | Teks untuk `greeting.submit` dalam bahasa pengguna | Resource OS yang ditulis emitter dari `Catalog` menurut L-9 dan L-11 |
| Katalog | Teks `en` `Greet` | Literal yang sekarang ada di `apps/macos/Presentation/GreetingView.swift` (spec macos/0001) |
| Katalog | Teks `id` `Sapa` | Usulan spec ini. Nilai yang disetujui hanya user yang tahu (lihat Follow-up). |
| Katalog | Daftar bahasa `en`, `id` dan bahasa sumber `en` | Keputusan spec ini, usulan yang boleh diganti user |
| Aplikasi memilih bahasa | Urutan pilihan bahasa pengguna | Pengaturan OS. Aplikasi tidak menyimpan pilihan bahasa. |
| `localization check`, baris sukses | Jumlah kunci | Jumlah anggota `entries` |
| `localization check`, baris sukses | Jumlah locale | Jumlah isi `locales` |
| `localization check`, baris sukses | Jumlah dan daftar terjemahan yang hilang | `Catalog.missing` |
| `localization check`, baris sukses | Jumlah keluaran | Panjang `Plan.outputs` |
| `parse` | Teks dan asal tiap locale | `entries.<kunci>.text`, lalu aturan L-9 tingkat per teks |
| `parse` | Urutan placeholder | Urutan kemunculan pertama tiap nama di teks sumber (L-7) |
| `parse` | Teks dan urutan masalah | Tabel **Aturan sumber** dan **Urutan pemeriksaan** |
| Keluaran skema | Pola kunci, pola locale, pola deskripsi, versi | Konstanta di `localization/mod.rs` yang juga dipakai `parse` |
| Keluaran skema | Teks penanda | Konstanta di `localization/schema.rs`, tertulis di **Isi berkas** |
| Emitter OS | Pengenal tiap teks di resource | `Entry.key`, menurut L-11 butir 2 |
| Emitter OS | Nama locale dalam kebiasaan OS | Pemetaan di spec OS, dari nama L-4 |
| Semua perintah | Akar repo dan path katalog | `repo_root()` dari spec `_root/0003` dan `CATALOG_PATH` |
| Bentuk baris sukses dan gagal, kode keluar | | Spec `_root/0003`, bagian API surface |

**Kewajiban spec OS**:

Spec macOS, Windows, dan Linux untuk scope 010 merujuk aturan di atas dan membuktikan hal berikut di host masing masing. ID ini stabil supaya spec OS bisa merujuknya.

* **OS-1** Label tombol sapaan dibaca dari resource lewat kunci `greeting.submit`. Literal `Greet` untuk tombol itu tidak lagi ada di kode UI.
* **OS-2** Emitter. Tambah satu modul `tooling/codegen/src/localization/<target>.rs` yang mengubah `&Catalog` menjadi satu `Output` atau lebih, dan panggil dari `localization::SOURCE.plan` sesudah skema. Keluaran memenuhi kelima butir L-11 dan aturan O-1 sampai O-7 spec `_root/0003`. AC spec OS memuat `cargo run -p desktop-codegen --locked -- localization check` dan `git diff --exit-code -- <folder keluaran>` sesudah `generate`.
* **OS-3** Aplikasi mendaftarkan `locales` sebagai bahasa yang didukung dan `sourceLocale` sebagai bahasa bawaan. Bila mekanisme OS mengembalikan kunci saat bahasa pengguna tidak cocok, aplikasi memuat resource `sourceLocale` sebagai pilihan terakhir.
* **OS-4** Uji tanpa jendela membuktikan teks per bahasa: `Greet` untuk `en`, `Sapa` untuk `id`, `Sapa` untuk pilihan `id-ID`, dan `Greet` untuk pilihan yang tidak didukung seperti `fr`.
* **OS-5** Kunci yang dipakai kode UI tetapi tidak ada di resource gagal saat build atau uji, bukan tampil sebagai kunci mentah saat aplikasi berjalan.
* **OS-6** Teks katalog tampil sebagai teks biasa (L-6), dan nilai placeholder diisikan sebagai teks biasa (L-7).
* **OS-7** Hasil `hello` tetap tampil apa adanya dan tetap bahasa Inggris di setiap bahasa antarmuka: dengan bahasa `id`, masukan `Ayu` tetap menampilkan `Hello, Ayu!`.
* **OS-8** Pemetaan nama locale ke kebiasaan OS dicatat dan satu ke satu (L-4), dan perilaku pemilihan bahasa di luar hasil minimum L-9 dicatat.
* **OS-9** Langkah manual di jendela nyata: mengganti bahasa sistem ke bahasa Indonesia menampilkan `Sapa`. Langkah ini milik user.

**Riwayat katalog**:

| Tanggal | `schemaVersion` | Perubahan | Spec |
| --- | --- | --- | --- |
| 2026-11-15 | `1` | Katalog dibuat dengan `greeting.submit` dalam `en` dan `id`. | 0006 |

**Key invariants**:

* Teks antarmuka yang dilokalkan hanya ditulis tangan di `shared/localization/catalog.json`. Berkas lain yang memuat teks itu adalah keluaran generator dan sama byte demi byte dengan hasil `generate`.
* `catalog.json` di `main` lulus `localization check` dan tidak punya terjemahan yang hilang.
* Setiap entri punya teks bahasa sumber, jadi setiap pasangan kunci dan locale selalu punya teks sesudah L-9.
* Kunci yang sama menunjuk makna yang sama di ketiga aplikasi dan sepanjang waktu.
* Himpunan placeholder sebuah kunci sama di semua bahasa.
* Skema dan `parse` memakai konstanta yang sama.
* `desktop-core` tidak bergantung pada katalog, alat, atau pustaka lokalisasi, dan `hello` tidak berubah. Aplikasi memakai berkas hasil generator, bukan crate `desktop-codegen`.

**Security model**:

Tidak ada autentikasi, peran, atau data yang diatur regulasi.

* Katalog adalah isi repo. Siapa pun yang boleh mengubah repo boleh mengubah teks, dan perubahannya terlihat di tinjauan git. Katalog tidak pernah diambil dari jaringan saat build atau saat aplikasi berjalan.
* Katalog tidak memuat rahasia, kredensial, atau data pribadi.
* Teks katalog dan nilai placeholder tampil sebagai teks biasa (L-6, L-7). Nilai placeholder bisa berasal dari pengguna, jadi nilai itu tidak pernah menjadi pola format, markup, atau kunci.
* Kunci dan deskripsi dibatasi ke karakter aman (L-3 dan pola deskripsi), jadi keduanya tidak bisa keluar dari komentar atau pengenal di berkas hasil generator. Teks boleh memuat karakter apa pun selain karakter kendali, jadi emitter wajib meloloskannya menurut format target (L-11 butir 4).
* Alat membaca dan menulis path tetap di bawah akar repo, tanpa argumen path.

**Configuration required**:

Tidak ada. Tidak ada variabel lingkungan, kredensial, atau pengaturan aplikasi baru. Bahasa antarmuka mengikuti pengaturan OS.

**File yang boleh diubah tiap milestone**:

| Milestone | File |
| --- | --- |
| 1 | `shared/localization/catalog.json`, `shared/localization/README.md`, `shared/localization/catalog.schema.json`, `tooling/codegen/src/localization/mod.rs`, `tooling/codegen/src/localization/schema.rs`, `tooling/codegen/src/lib.rs` (dua perubahan di **Isi berkas**), `tooling/codegen/tests/localization_contract.rs`, `.gitattributes` (satu baris), `verify.md` spec ini, baris `**Status**` di `index.md` spec ini, dan empat file `graphify-out/` dari hook |
| 2 | `docs/shared-resources/README.md`, `docs/architecture/README.md`, `verify.md` spec ini, dan empat file `graphify-out/` dari hook |

`Cargo.toml` akar dan `Cargo.lock` tidak berubah, karena tidak ada anggota atau dependensi baru. Bila baris usage atau uji sumber lain di `desktop-codegen` menyebut daftar sumber secara tetap dan gagal sesudah `localization` masuk, jangan mengubahnya sendiri. Minta pemilik spec `_root/0003` memperbaruinya.

**Kasus gagal yang sudah diputuskan**:

| Kejadian | Perilaku |
| --- | --- |
| Terjemahan belum ada saat developer membangun aplikasi dari working tree | `generate` tetap menulis resource dengan teks bahasa sumber (L-9) dan menyebut pasangan yang hilang di ringkasan. `cargo test --workspace --locked` gagal sampai terjemahan ditambahkan, jadi perubahan itu belum boleh masuk `main`. |
| Katalog diubah tanpa `generate` | `check` dan `cargo test --workspace --locked` gagal dan menyebut perintah `generate` (perilaku spec `_root/0003`). Katalog dan keluarannya masuk commit yang sama. |
| Dua branch menambah kunci di tempat yang sama dan konflik saat merge | Selesaikan konflik di `catalog.json`, jalankan `generate`, lalu commit hasilnya. Jangan menyelesaikan konflik di berkas hasil generator. |
| Teks sumber diubah sesudah diterjemahkan | Tidak ada deteksi otomatis. Orang yang mengubah teks sumber meninjau terjemahannya dalam perubahan yang sama. Bila maknanya berubah, buat kunci baru (L-3). |
| Teks butuh kurung kurawal literal, baris baru, atau bentuk jamak | Belum didukung. Perbarui spec ini lebih dulu (L-6, L-7, L-12). |
| Bahasa yang dibutuhkan tidak bisa ditulis dengan L-4 (misalnya butuh aksara) | Belum didukung. Perbarui L-4 lewat spec ini, bersama pemetaan di spec OS. |
| Hook Graphify menolak commit karena berkas baru tidak menghasilkan node | Ikuti aturan di spec `_root/0001`: tambahkan entri sempit ke `.graphifyignore` dan sebut entrinya di laporan. Jangan memakai `--no-verify`. |

**Critical test scenarios**:

* Jalur utama: katalog nyata lulus `check`, terurai ke nilai yang benar, dan `generate` menulis nol berkas, memverifikasi **AC-1**, **AC-2**, **AC-9**.
* Kunci dan nama locale: kasus 7 sampai 19 ditolak dengan pesan yang menyebut tempatnya, memverifikasi **AC-3**, **AC-4**.
* Teks dan placeholder: kasus 20 sampai 30 ditolak, dan urutan placeholder yang berbeda diterima, memverifikasi **AC-5**.
* Terjemahan hilang: K-2 memberi dua pasangan di `missing` dan ringkasan yang menyebut keduanya, dan katalog nyata dijaga uji gerbang, memverifikasi **AC-6**.
* Fallback: teks yang hilang di K-2 memakai teks bahasa sumber dengan asal `Fallback`, memverifikasi **AC-7**.
* Versi dan bentuk: kasus 1 sampai 6 dan 31 ditolak, dan skema hasil generator sama dengan **Isi berkas**, memverifikasi **AC-8**.
* Keluaran menyimpang: skema yang hilang atau disunting tertangkap `check` dan `cargo test`, lalu diperbaiki `generate`, memverifikasi **AC-9**.
* Cakupan: hanya file di tabel milestone yang berubah, Core dan alat tidak mendapat dependensi, memverifikasi **AC-10**.
* Dokumen: dokumen rujukan menautkan aturan, memverifikasi **AC-11**.
* Akses: tidak ada peran atau izin di fitur ini. Yang ditolak adalah katalog yang tidak sah, memverifikasi **AC-8**.

## Perubahan dokumen rujukan

Langkah 3 Build plan mengubah dua dokumen berikut. Bagian lain kedua dokumen itu tidak diubah. Kedua dokumen biasanya terlarang bagi sub sesi, jadi langkah ini butuh izin tertulis di brief developer.

1. Di `docs/shared-resources/README.md`, bagian "Lokalisasi", ganti seluruh paragrafnya dengan:

   ```markdown
   `shared/localization/catalog.json` memuat katalog teks bersama dengan kunci stabil. Aturan nama locale, fallback bila teks bahasa tujuan belum ada, placeholder yang harus cocok, deteksi entri hilang, dan versi ada di [spec 0006](../specs/_root/0006-satu-contoh-lokalisasi/index.md). Satu contoh teks UI, label tombol sapaan dengan kunci `greeting.submit`, membuktikan alurnya. Katalog diperiksa dengan `cargo run -p desktop-codegen --locked -- localization check`. Format angka, tanggal, dan bentuk jamak mengikuti OS.
   ```

2. Di `docs/architecture/README.md`, bagian "Tata letak yang dituju", tambahkan `shared/localization/` ke daftar lokasi di kalimat yang berakhir dengan "sudah ada." (paragraf pertama bagian itu), sesudah lokasi terakhir yang tercantum saat itu.

3. Di `docs/architecture/README.md`, tabel "Yang sudah ada", tambahkan satu baris sesudah baris terakhir:

   ```markdown
   | `shared/localization/` | Katalog bahasa bersama `catalog.json` dan skema hasil generator. Aturannya ada di [spec 0006](../specs/_root/0006-satu-contoh-lokalisasi/index.md). |
   ```

Tabel "Aturan arah dependensi" tidak diubah. Uji `workspace_layout` hanya mengunci tabel itu, jadi perubahan di atas tidak menyentuhnya. Uji itu ada di `crates/core/tests/workspace_layout.rs` saat milestone 2 dibangun, dan di `tooling/codegen/tests/workspace_layout.rs` sejak spec `_root/0007`.

## Build plan

Pendekatan build proyek adalah Tracer Bullet dari [Panduan scope](../../../scope/000%20-%20Panduan%20scope.md). Benang tipisnya: katalog nyata, lewat `parse` dan `check`, sampai skema yang dibandingkan oleh uji. Sesudah itu jalur gagal dan fallback ditebalkan, lalu dokumen rujukan mengikuti keadaan nyata.

Prasyarat: milestone 1 spec `_root/0003` ("Alat dan sumber token") sudah ada di `main`. Build spec ini tidak berjalan bersamaan dengan build spec `_root/0003` atau spec `_root/0004`, karena `tooling/codegen/src/lib.rs` dan `.gitattributes` dipakai bersama.

Satu langkah menjadi satu commit dengan subject `type(scope): spec _root/0006 step <n>, ...` (**AC-10** mencari teks itu). Hasil pemeriksaan dicatat di `docs/specs/_root/0006-satu-contoh-lokalisasi/verify.md`, di bagian `## Milestone <n>: <judul>` dengan judul seperti di bawah. Developer mengubah baris `**Status**` spec ini menjadi `In Progress` pada commit pertama. Pakai `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, dan hapus `target/` sesudah mencatat hasil.

**Milestone 1: Katalog dan sumber `localization`** (langkah 1 dan 2)

1. Buat `catalog.json` dan `README.md` di `shared/localization/`, modul `localization` (`mod.rs` dengan tipe, `SOURCE`, dan `parse` yang menjalankan urutan pemeriksaan langkah 1 dan 2 serta mengisi `texts`, `placeholders`, dan `missing`; `schema.rs` dengan penulis skema), dua perubahan di `lib.rs`, baris `.gitattributes`, dan berkas uji dengan `committed_outputs_match_the_source`, `committed_catalog_is_complete`, `real_catalog_resolves`, dan `generation_is_deterministic`. Jalankan `cargo run -p desktop-codegen --locked -- localization generate` untuk menulis skema, lalu cocokkan dengan **Isi berkas**. Semua masuk satu commit, satisfies **AC-1**, **AC-2**, **AC-8**, **AC-9**, **AC-10**
2. Lengkapi `parse` dengan urutan pemeriksaan langkah 3 dan 4 dan semua pesan di **Aturan sumber**. Tambah uji `rejects_broken_catalogs` (31 kasus), `placeholders_may_reorder`, dan `missing_translations_are_listed_and_fall_back`. Jalankan setiap perintah **AC-1** sampai **AC-10** dan catat perintah serta keluarannya di `verify.md` bagian `## Milestone 1: Katalog dan sumber localization`, satisfies **AC-3**, **AC-4**, **AC-5**, **AC-6**, **AC-7**, **AC-8**

**Milestone 2: Dokumen rujukan mencatat keadaan nyata** (langkah 3, sesudah milestone 1)

3. Perbarui `docs/shared-resources/README.md` dan `docs/architecture/README.md` sesuai bagian **Perubahan dokumen rujukan**, dan catat pemeriksaan **AC-11** di `verify.md` bagian `## Milestone 2: Dokumen rujukan mencatat keadaan nyata`, satisfies **AC-11**

## Consequences

**Positive**:

* Ketiga aplikasi memakai kunci, teks, dan aturan yang sama, dan spec OS tidak perlu memutuskan apa pun tentang isi katalog.
* Terjemahan yang hilang dan placeholder yang tidak cocok tertangkap oleh `cargo test --workspace --locked`, gerbang yang sudah dijalankan lead, tanpa membuka jendela.
* Aplikasi tidak pernah menampilkan kunci mentah, karena setiap resource lengkap sesudah fallback dan bahasa sumber selalu ditulis.
* Tidak ada alat, paket, atau dependensi baru. Katalog menumpang pola yang sama dengan token.
* Menambah teks berikutnya adalah menambah satu entri. Tidak ada rancangan baru.
* Core tetap tanpa dependensi dan tanpa pengetahuan bahasa.

**Negative / tradeoffs**:

* Layar sapaan menjadi setengah terjemahan untuk pengguna bahasa Indonesia: tombol `Sapa`, tetapi label kolom `Name` dan hasil `Hello, Ayu!` tetap bahasa Inggris. Itu harga dari "satu contoh".
* `localization check` tetap keluar dengan kode 0 saat ada terjemahan yang hilang. Yang menolaknya adalah uji gerbang, jadi orang yang hanya menjalankan `check` harus membaca ringkasannya.
* Setiap OS menambah satu modul emitter dan satu atau lebih berkas hasil generator yang dilacak git.
* Kurung kurawal literal, baris baru, dan bentuk jamak belum bisa ditulis. Teks pertama yang membutuhkannya menunggu pembaruan spec ini.
* Nama locale hanya bahasa dan wilayah. Bahasa yang butuh aksara (misalnya `zh-Hans`) harus ditulis dengan wilayah (`zh-CN`) sampai L-4 diperluas.
* Bahasa wilayah seperti `pt-BR` tidak mewarisi teks `pt`. Setiap locale yang terdaftar wajib punya teks sendiri untuk setiap kunci.
* Uji gerbang membuat teks baru tidak bisa masuk `main` sebelum semua terjemahannya ada.
* Teks sumber yang berubah tidak menandai terjemahannya sebagai usang. Hanya tinjauan yang menangkapnya.
* JSON tidak punya komentar. Catatan untuk penerjemah hanya bisa ditulis di `description`, dengan karakter yang dibatasi pola.
* Build spec ini menunggu milestone 1 spec `_root/0003` dan tidak bisa paralel dengan build scope 009.
* Teks `Sapa` belum disetujui user.

**Neutral**:

* Kasus uji ada di dua tempat (tabel di spec ini dan berkas uji). Mengubah satu tanpa yang lain membuat keduanya tidak cocok.
* Aturan ditulis di spec ini, dan `shared/localization/README.md` serta dokumen rujukan hanya menautkannya.
* Pemilihan bahasa di luar hasil minimum L-9 boleh berbeda antar OS dan dicatat di spec OS.
* Semua bahasa tinggal di satu berkas. Bila jumlah bahasa atau teks tumbuh sampai merepotkan penerjemah, pemecahan per bahasa adalah perubahan `schemaVersion`.
* Menambah pewarisan teks dari bahasa induk kelak tidak memutus katalog yang sudah lengkap, karena katalog yang lengkap memberi hasil yang sama dengan atau tanpa pewarisan.
* Angka di **AC-2** dan **AC-9** (`1 outputs`) berbatas "pada akhir milestone 1" dan naik saat spec OS menambah emitter, tanpa pembaruan spec ini. Spec `macos/0006` menaikkannya ke `3 outputs`.

## Follow-up

* [x] Jawaban atas Follow-up "Scope 010" di spec macos/0001: hanya `Greet` yang pindah ke katalog, sebagai `greeting.submit`. `Name` tetap literal, karena berkas fitur scope 010 meminta satu teks. `Desktop Foundation` adalah nama produk, jadi tidak pernah masuk katalog. Hasil `hello` tetap memakai `Text(verbatim:)` (OS-7). Baris Follow-up itu ditandai oleh pemilik spec macos/0001. Dijawab spec [`macos/0006`](../../macos/0006-satu-contoh-lokalisasi/index.md) (2026-11-15): tombol membaca `greeting.submit`, `Name` tetap literal, dan hasil tetap `Text(verbatim:)`.
* [ ] Jawaban atas Follow-up "Scope 010" di spec `_root/0004`: katalog tidak memuat nama produk (L-1). Judul jendela mengambil nama produk dari nilai turunan `build.product_name` lewat spec OS scope 009, bukan dari katalog.
* [x] macOS (scope 010, `docs/specs/macos/`, sesudah spec ini diterima): pilih format resource Apple dan lokasinya di `apps/macos/`, tambah emitter macOS (OS-2), ubah `GreetingView` agar membaca `greeting.submit`, dan buktikan OS-1 sampai OS-9. Perubahan itu menambah path di `apps/macos/`, jadi **AC-12** spec macos/0001 (tepat 16 path) perlu diperbarui lebih dulu. SwiftUI membaca literal string sebagai kunci lokalisasi dan menafsirkan Markdown, jadi spec macOS perlu menetapkan cara menampilkan teks katalog sebagai teks biasa (OS-6). Dijawab spec [`macos/0006`](../../macos/0006-satu-contoh-lokalisasi/index.md) (2026-11-15): berkas `.strings` per bahasa di `apps/macos/Presentation/Resources/`, emitter `localization/macos.rs`, tombol menerima `String`, dan OS-1 sampai OS-9 dicakup AC-1 sampai AC-11 spec itu. Perubahan AC-12 spec macos/0001 masuk pembaruan gabungan lead.
* [ ] Windows (scope 010, menunggu host Windows): adapter lokalisasi Rust untuk presentasi WinUI, dengan OS-1 sampai OS-9. Adapter menerapkan langkah pemilihan bahasa L-9 sendiri. Spec ini tidak menulis bukti untuk Windows.
* [ ] Linux (scope 010, menunggu host Linux): resource yang sesuai GTK, dengan OS-1 sampai OS-9. Bila memakai gettext, kunci menjadi `msgid`, jadi katalog bahasa sumber wajib ditulis dan dimuat sebagai pilihan terakhir (OS-3). Spec ini tidak menulis bukti untuk Linux.
* [ ] Bukti `desktop-codegen` di host Windows dan Linux mengikuti AC-12 spec `_root/0003`. Di sana `cargo run -p desktop-codegen --locked -- localization check` ikut dijalankan (`owner: user`).
* [ ] Nilai dari user: bahasa yang didukung selain bahasa sumber, dan teks terjemahan yang disetujui. Usulan sekarang `id` dengan `Sapa`. Menggantinya adalah perubahan `catalog.json`, tabel **Nilai hasil**, **AC-1**, **AC-2**, kasus uji yang menyebut teks itu, dan OS-4, lewat pembaruan spec ini, tanpa mengubah aturan.
* [ ] Pemilik spec `_root/0003`: sumber `localization` menambah tiga path di `tooling/codegen/` dan satu baris `.gitattributes`. Angka path di AC-1 spec itu dan baris `sources:` di usage ikut berubah. Bila spec `_root/0003` berubah (nama tipe, pola deskripsi, bentuk baris sukses), spec ini diperbarui mengikutinya.
* [x] Scope 012: skrip `scripts/validate.sh` dan CI menjalankan `localization check` dan `cargo test --workspace --locked`. Dijawab [spec `_root/0008`](../0008-build-dan-penerimaan-per-target/index.md) (2026-11-15).
* [ ] Pola deskripsi `^[A-Za-z0-9 ,.()-]{1,120}$` ditulis dua kali sebagai `pub(crate) const DESCRIPTION_PATTERN`, di `tooling/codegen/src/tokens/mod.rs` dan `tooling/codegen/src/localization/mod.rs`. Tambahkan satu uji unit di crate itu yang membandingkan kedua konstanta, atau biarkan `localization` memakai konstanta `tokens`, lewat build berikutnya yang menyentuh salah satu modul. Uji integrasi tidak bisa melihat konstanta `pub(crate)`.
* [ ] Scope 002: jadikan L-1 pemeriksaan arsitektur yang tetap (teks antarmuka yang punya kunci tidak boleh ditulis literal di kode UI), dan jadikan `desktop-core` tanpa pustaka lokalisasi bagian dari daftar larangan Core. Separuh kedua sudah terjawab: aturan A-3 spec [`_root/0007`](../0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md) mengunci daftar dependensi luar langsung `desktop-core` ke "tidak ada". L-1 belum menjadi aturan `A`, karena saat spec itu ditulis belum ada aplikasi yang memakai katalog (Follow-up spec `_root/0007`); spec OS yang memasang katalog menambahkannya lewat pembaruan spec itu.
* [x] `/sync` sesudah build: kalimat "sumber bersama masih rencana" di `AGENTS.md` akar tidak lagi benar untuk `shared/localization/`. Diperbaiki `/sync` wave 2 (`1acc686`, 2026-11-15): kalimat pembuka `AGENTS.md` kini menyebut sumber bersama di `design-system/` dan `shared/` sudah ada.
* [ ] Lead: tambahkan tautan balik ke spec ini di berkas scope 010. Berkas itu di luar ownership sesi penulis spec.
* [ ] Bila sebuah teks butuh bentuk jamak, kurung kurawal literal, baris baru, nama locale dengan aksara, atau pewarisan teks dari bahasa induk, perbarui aturan terkait lebih dulu dan naikkan `schemaVersion` bila katalog lama tidak lagi sah.
