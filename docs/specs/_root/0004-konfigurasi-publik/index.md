# 0004. Konfigurasi publik

**Scope**: [009 - Konfigurasi publik](../../../scope/009%20-%20Konfigurasi%20publik.md)
**Date**: 2026-11-15
**Status**: In Progress

## Summary

Spec ini menetapkan satu file, `shared/config/public.json`, sebagai sumber nilai publik yang wajib sama di aplikasi macOS, Windows, dan Linux. Fondasi saat ini hanya butuh dua nilai di file itu, yaitu nama produk dan dasar pengenal aplikasi. Versi aplikasi tetap bersumber dari Cargo. Skemanya tertutup (kunci yang tidak dikenal ditolak) dan bernomor versi, sehingga pengaturan lokal dan rahasia penandatanganan tidak bisa menyelinap masuk. Semua nilai dipakai saat build, dan tiap OS memindahkannya ke bentuk native lewat generator atau lewat salinan yang dibandingkan pemeriksaan otomatis.

## Requirements

**User stories**:

- Sebagai pemilik produk, kamu ingin mengganti nama produk atau pengenal aplikasi di satu tempat, supaya ketiga aplikasi ikut berubah tanpa kamu menyunting tiap proyek native.
- Sebagai pengembang aplikasi satu OS, kamu ingin tahu nilai mana yang datang dari sumber bersama dan nilai mana yang boleh kamu atur sendiri di proyek native, supaya kamu tidak menyalin nilai bersama dengan tangan.
- Sebagai lead, kamu ingin satu pemeriksaan yang gagal saat seseorang menaruh nilai lokal, rahasia, atau kunci yang belum diputuskan di folder bersama.

**Acceptance criteria** (kontrak; nomor kriteria merujuk [Penerimaan dan bukti fondasi](../../../quality/README.md); semua perintah dijalankan dari root repo di host macOS, pada file yang sudah masuk commit):

- **AC-1** (kriteria 18; Done when "skema"): `git ls-files shared/config` mencetak tepat tiga baris: `shared/config/README.md`, `shared/config/public.json`, dan `shared/config/public.schema.json`. Isi `README.md` dan `public.schema.json` sama persis dengan bagian "Isi file": `awk '/^````markdown$/{f=1;next} /^````$/{f=0} f' docs/specs/_root/0004-konfigurasi-publik/index.md | diff - shared/config/README.md` dan `awk '/^```json$/{n++;f=(n==3);next} /^```$/{f=0} f' docs/specs/_root/0004-konfigurasi-publik/index.md | diff - shared/config/public.schema.json` tidak mencetak apa pun. `grep -cxF '/shared/config/**/*.json text eol=lf' .gitattributes` mencetak `1`.
- **AC-2** (Done when "nilai publik"): `jq -c . shared/config/public.json` mencetak `{"$schema":"./public.schema.json","schemaVersion":1,"build":{"product_name":"NAMA_PRODUK","app_identifier":"DASAR_PENGENAL"}}`, dengan `NAMA_PRODUK` dan `DASAR_PENGENAL` dari tabel "Nilai dari user". Selama user belum memberi nilai akhir, keduanya adalah nilai sementara `Desktop Foundation` dan `com.example.desktop-foundation`, dan `awk '/^```json$/{n++;f=(n==2);next} /^```$/{f=0} f' docs/specs/_root/0004-konfigurasi-publik/index.md | diff - shared/config/public.json` tidak mencetak apa pun.
- **AC-3** (Done when "versi"): nomor versi skema sama di setiap tempat ia tercatat. `jq -r '.schemaVersion' shared/config/public.json` dan `jq -r '.properties.schemaVersion.const' shared/config/public.schema.json` sama sama mencetak `1`, `grep '^| [0-9]' shared/config/README.md | tail -1` mencetak baris yang diawali `| 1 |`, dan pemeriksaan hanya mengenal versi `1` (uji T-25 sampai T-27 di **AC-5**). Versi aplikasi tidak ada di `public.json`: `grep -c '"version"' shared/config/public.json` mencetak `0`, dan `grep -c '^version = ' Cargo.toml` mencetak `1` (baris di `[workspace.package]`).
- **AC-4** (kriteria 18; Done when "cara validasi"): `cargo run -p desktop-codegen --locked -- config check` keluar dengan kode 0 dan mencetak baris yang diawali `config: ok, schema version 1, 2 build values`. Sesudah `touch shared/config/local.json`, perintah yang sama keluar dengan kode 1, dan stderr memuat satu baris yang diawali `shared/config/local.json: R-1` lalu baris `config: failed, 1 problems`; sesudah `rm shared/config/local.json`, perintah itu kembali keluar dengan kode 0. `cargo run -p desktop-codegen --locked -- config generate` keluar dengan kode 0, mencetak baris yang diakhiri `0 written`, dan sesudahnya `git status --short` tidak mencetak nama file apa pun, jadi skema yang terlacak sama dengan keluaran generator.
- **AC-5** (Done when "cara validasi"): `cargo test -p desktop-codegen --locked --test config_contract` keluar dengan kode 0. File uji itu memuat satu uji untuk setiap baris tabel "Kasus yang ditolak" dan "Kasus yang diterima", masing masing memberi masukan lewat root sementara tanpa menyunting file di `shared/config/`, serta uji `committed_outputs_match_the_source` yang menjalankan `check(&repo_root(), &config::SOURCE)`. Tiap uji tolak menyatakan `plan` gagal dan salah satu pesannya diawali ID aturan di kolom "Aturan". Nama tiap uji diawali nomor kasusnya, sehingga `cargo test -p desktop-codegen --locked --test config_contract -- --list` mencetak 32 uji bernama `t01_` sampai `t32_` dan uji `committed_outputs_match_the_source`. Uji yang sama lulus lewat `cargo test --workspace --locked`.
- **AC-6** (Done when "pisahkan nilai publik dari pengaturan lokal dan rahasia"): `shared/config/README.md` memuat heading `## Yang tidak boleh ada di folder ini` dengan tabel empat kelas dari bagian "Empat kelas nilai". `git ls-files shared/config | wc -l | tr -d ' '` mencetak `3`. `git grep -n -i 'DEVELOPMENT_TEAM\|CODE_SIGN\|password\|secret\|token' -- shared/config/public.json shared/config/public.schema.json` tidak mencetak apa pun.
- **AC-7** (Done when "konsumsi"): `shared/config/README.md` memuat heading `## Cara aplikasi memakai nilai` dengan tabel pemakaian dan aturan `K-1` sampai `K-7`; `grep -c '^- \*\*K-[1-7]\*\*' shared/config/README.md` mencetak `7`. Yang berlaku terus: `git grep -n 'shared/config\|public\.json' -- crates/core/src crates/macos-ffi/src bindings` tidak mencetak apa pun, karena Core dan jembatan tidak membaca konfigurasi (`K-4`).
- **AC-8** (kriteria 18; Done when "batas pengaturan khusus tiap OS"): `shared/config/README.md` memuat heading `## Batas pengaturan khusus OS` dengan tiga pertanyaan penentu dan tabel contoh per OS. `docs/shared-resources/README.md` dan `docs/architecture/README.md` memuat teks dari bagian "Perubahan dokumen": `grep -c '0004-konfigurasi-publik' docs/shared-resources/README.md` mencetak `1`, `grep -c 'akan memiliki skema untuk nilai publik' docs/shared-resources/README.md` mencetak `0`, `grep -c '^Path ini menunjukkan.*.shared/config/.*sudah ada\.' docs/architecture/README.md` mencetak `1`, dan `grep -c '^| .shared/config/. |' docs/architecture/README.md` mencetak `1`.
- **AC-9** (kriteria 32): `jq -c 'keys' shared/config/public.json` mencetak `["$schema","build","schemaVersion"]`, jadi tidak ada grup `runtime`, definisi environment, atau flag. Build spec ini hanya menyentuh tiga file di `shared/config/`, `tooling/codegen/src/config/mod.rs`, `tooling/codegen/src/lib.rs` (baris `pub mod config;` dan baris `SOURCES`), `tooling/codegen/tests/config_contract.rs`, `.gitattributes` (satu baris), `docs/shared-resources/README.md`, `docs/architecture/README.md`, `verify.md` spec ini, baris `**Status**` di `index.md` spec ini, dan empat file `graphify-out/` dari hook. Daftar itu diperiksa dengan `git diff --name-only main...HEAD` di branch developer sebelum merge. `Cargo.toml`, `Cargo.lock`, `tooling/codegen/Cargo.toml`, dan semua file di `apps/`, `crates/`, serta `bindings/` tidak berubah, jadi tidak ada dependensi baru. Berkas scope disunting lead di luar branch developer.
- **AC-10** (gerbang merge lead): `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, dan `cargo test --workspace --locked` keluar dengan kode 0.

## Decision

**Chosen option**: Opsi 1: satu file nilai JSON dengan skema tertutup di `shared/config/`.

Nilai publik bersama tinggal di `shared/config/public.json`, diatur skema tertutup bernomor versi, dipakai saat build saja, dan tidak pernah disalin ke proyek native tanpa generator atau pemeriksaan yang membandingkannya.

Keputusan turunannya:

| Hal | Keputusan |
| --- | --- |
| Nilai di versi 1 | `build.product_name` dan `build.app_identifier`. Hanya dua, karena fondasi belum butuh yang lain. |
| Versi aplikasi | Sumbernya `[workspace.package].version` di `Cargo.toml` root. Tidak disalin ke `public.json`. |
| Format | JSON, UTF-8, indentasi dua spasi, diakhiri satu baris baru. |
| Skema | Tabel "Skema versi 1", ditegakkan tipe Rust tertutup dan pemeriksaan tulis tangan di `desktop-codegen`. `shared/config/public.schema.json` adalah keluaran generator untuk editor, bukan sumber aturan. |
| Versi skema | Bilangan bulat mulai dari `1`, naik satu pada setiap perubahan skema. |
| Susunan kunci | Grup menurut kapan nilai dipakai. Versi 1 hanya punya `build`. |
| Kapan dipakai | Saat build. Aplikasi tidak membuka `shared/config/` saat berjalan. |
| Salinan di proyek native | Dibuat generator, atau dibandingkan pemeriksaan otomatis. Spec OS memilih. |
| Pengenal tiap OS | Diturunkan dari `build.app_identifier` dengan aturan tetap yang spec OS tulis (`K-6`). |
| Nilai saat berjalan, environment, flag | Tidak ada di versi 1. Ditambah lewat pembaruan spec ini saat ada kebutuhan nyata. |
| Timpaan lokal | Tidak ada untuk nilai publik. |
| Isi folder | Tertutup: tiga file di "Isi file". |
| Alat validasi | Sumber baru bernama `config` di paket `desktop-codegen` dari [spec `_root/0003`](../0003-tema-bersama-dan-kontrol-native/index.md) (scope 008). Spec ini tidak membuat alat sendiri dan tidak menambah dependensi. |

## Rationale

Alasan, opsi yang ditimbang, dan daftar keputusan beserta pilihan keduanya ada di [rationale.md](rationale.md).

## Rancangan konfigurasi

### Empat kelas nilai

Setiap nilai yang mengatur aplikasi masuk tepat satu kelas. Kelas menentukan tempatnya.

| Kelas | Arti | Tempat | Dilacak git | Contoh |
| --- | --- | --- | --- | --- |
| Nilai publik bersama | Boleh dilihat siapa pun dan wajib sama maknanya di lebih dari satu aplikasi. | `shared/config/public.json`. Versi aplikasi di `Cargo.toml` root. | Ya | Nama produk, dasar pengenal aplikasi, versi aplikasi. |
| Pengaturan khusus OS | Hanya dimengerti satu OS, atau tiap OS punya nilainya sendiri. | Proyek native di `apps/<os>/`, milik spec OS itu. | Ya | Lihat "Batas pengaturan khusus OS". |
| Pengaturan lokal | Hanya berlaku di satu mesin atau untuk satu orang. | File yang diabaikan git atau variabel lingkungan shell. | Tidak | `xcuserdata/`, `DerivedData/`, `CARGO_PROFILE_DEV_DEBUG`, tim pengembang pribadi di Xcode. |
| Rahasia | Memberi akses atau membuktikan identitas. | Keychain di mesin pengembang atau penyimpanan rahasia CI. Tidak pernah di repo. | Tidak | Sertifikat dan kunci privat penandatanganan, kredensial notarisasi, token CI. |

Tiga pertanyaan penentu, dijawab berurutan:

1. Apakah nilai itu rahasia, atau hanya benar di satu mesin atau untuk satu orang? Bila ya, nilai itu rahasia atau pengaturan lokal dan tidak masuk repo.
2. Apakah lebih dari satu aplikasi butuh nilai itu dengan makna yang sama? Bila ya, nilai itu nilai publik bersama.
3. Selain itu, nilai itu pengaturan khusus OS dan tinggal di proyek native aplikasi itu.

### Batas pengaturan khusus OS

Pengaturan khusus OS dimiliki spec OS itu dan tidak pernah masuk `shared/config/`. Daftar berikut contoh, bukan daftar lengkap.

| OS | Contoh pengaturan khusus OS | Pemilik |
| --- | --- | --- |
| macOS | Versi minimum macOS (`MACOSX_DEPLOYMENT_TARGET`), nomor build (`CURRENT_PROJECT_VERSION`), nama target, gaya penandatanganan (`CODE_SIGN_STYLE`, `CODE_SIGN_IDENTITY`), sandbox, hardened runtime, entitlement, kategori aplikasi, aturan turunan pengenal bundel. | Spec di `docs/specs/macos/` |
| Windows | Manifest aplikasi, identitas paket, versi Windows App SDK dan versi minimum Windows, aturan turunan pengenal aplikasi. | Spec di `docs/specs/windows/`, menunggu host |
| Linux | Isi berkas `.desktop` selain nama, kategori menu, versi minimum GTK dan libadwaita, lokasi ikon, aturan turunan pengenal aplikasi. | Spec di `docs/specs/linux/`, menunggu host |

Aturan di perbatasan:

- Nilai OS yang diturunkan dari nilai bersama tetap mengikuti sumbernya. Pengenal bundel macOS diturunkan dari `build.app_identifier` (aturan `K-6`), dan nama tampil bundel diturunkan dari `build.product_name`.
- Bila sebuah OS butuh nilai yang tidak bisa diturunkan dari nilai bersama, nilai itu pengaturan khusus OS dengan nama lain di proyek native. `public.json` tidak diberi kunci per OS.
- Bila pengembang butuh nilai lain di mesinnya (misalnya pengenal bundel unik untuk tim pribadi di Xcode), itu pengaturan lokal. Spec OS menetapkan file native yang diabaikan git untuk itu. `public.json` tidak diubah dan tidak punya file timpaan.
- Debug dan Release adalah konfigurasi build tiap toolchain, jadi termasuk pengaturan khusus OS. Keduanya bukan environment.

### Skema versi 1

| Kunci | Tipe | Wajib | Arti | Aturan |
| --- | --- | --- | --- | --- |
| `$schema` | string | ya | Path relatif ke skema hasil generator, supaya editor bisa memeriksa file ini. | Tepat `./public.schema.json`. |
| `schemaVersion` | bilangan bulat | ya | Versi skema yang dipakai file ini. | Bilangan bulat JSON tanpa pecahan dan tanpa tanda. Nilainya diperiksa aturan `R-3`. |
| `build` | objek | ya | Nilai yang dipakai tiap aplikasi saat build. | Hanya memuat dua kunci di bawah. |
| `build.product_name` | string | ya | Nama produk yang tampil kepada pengguna. Sama untuk semua bahasa. | Paling sedikit satu karakter. Tidak memuat karakter kontrol (U+0000 sampai U+001F dan U+007F sampai U+009F) dan tidak memuat pemisah baris U+2028 atau U+2029. Karakter pertama dan terakhir bukan whitespace Unicode (properti `White_Space`) dan bukan U+FEFF. |
| `build.app_identifier` | string | ya | Dasar pengenal aplikasi bergaya reverse DNS (nama domain dibalik, misalnya `com.example.produk`). | Paling banyak 100 karakter. Dua segmen atau lebih, dipisah satu titik. Tiap segmen diawali huruf ASCII, hanya memuat huruf ASCII, angka ASCII, dan tanda `-`, dan tidak diakhiri `-`. |

Objek teratas dan objek `build` tertutup: kunci lain dan kunci ganda ditolak. Dua kunci pertama, `$schema` dan `schemaVersion`, adalah pembuka wajib setiap file sumber menurut spec `_root/0003`. Aturan nilai ditulis dengan kata kata dan ditegakkan kode pemeriksaan, supaya hasilnya tidak bergantung pada mesin regex tertentu. Uji di "Kasus yang ditolak" dan "Kasus yang diterima" adalah bentuk aturan yang bisa dijalankan.

Aturan `build.app_identifier` memenuhi aturan pengenal bundel Apple (huruf, angka, tanda `-`, dan titik). Kecocokannya dengan format Windows dan Linux belum diperiksa di host itu, dan sebagian format di sana lebih sempit (lihat `K-6` dan Follow-up). Batas 100 karakter adalah batas kewajaran yang ditetapkan spec ini, bukan batas dari satu OS.

Yang sengaja tidak ada di versi 1:

| Jenis | Keadaan | Cara menambahnya kelak |
| --- | --- | --- |
| Nilai saat aplikasi berjalan | Tidak ada. Fondasi tidak membaca apa pun saat berjalan. | Pembaruan spec ini menambah grup `runtime`, menetapkan cara nilai itu sampai ke aplikasi, dan menaikkan versi skema. |
| Definisi environment | Tidak ada. Fondasi tidak punya backend, jadi hanya ada satu rasa build. | Pembaruan spec ini menambah definisi environment saat nilai pertama yang berbeda per environment muncul. |
| Feature flag dan konfigurasi jarak jauh | Di luar fase fondasi. | PRD dan spec tersendiri. |

### Isi file

Developer menulis `public.json` dan `README.md` persis seperti ini, masing masing diakhiri satu baris baru. `public.schema.json` tidak ditulis tangan: generator yang menulisnya, dan hasilnya harus sama persis dengan blok di bawah. `NAMA_PRODUK` dan `DASAR_PENGENAL` diganti nilai dari tabel "Nilai dari user".

`shared/config/public.json`:

```json
{
  "$schema": "./public.schema.json",
  "schemaVersion": 1,
  "build": {
    "product_name": "NAMA_PRODUK",
    "app_identifier": "DASAR_PENGENAL"
  }
}
```

Dengan nilai sementara, file itu berisi:

```json
{
  "$schema": "./public.schema.json",
  "schemaVersion": 1,
  "build": {
    "product_name": "Desktop Foundation",
    "app_identifier": "com.example.desktop-foundation"
  }
}
```

`shared/config/public.schema.json`, keluaran `cargo run -p desktop-codegen --locked -- config generate` (JSON Schema draft-07, kunci terurut, dua spasi, satu baris baru di akhir; teks berbahasa Inggris mengikuti bahasa komentar kode sampai scope 002 memutuskan kebijakannya):

```json
{
  "$comment": "Generated by desktop-codegen from tooling/codegen/src/config. Do not edit. To regenerate, run: cargo run -p desktop-codegen --locked -- config generate",
  "$schema": "http://json-schema.org/draft-07/schema#",
  "additionalProperties": false,
  "description": "Public values that must be the same in the macOS, Windows, and Linux apps. An editor aid; the check in desktop-codegen is the authority. Rules: docs/specs/_root/0004-konfigurasi-publik/index.md.",
  "properties": {
    "$schema": {
      "const": "./public.schema.json",
      "description": "Relative path to this generated schema."
    },
    "build": {
      "additionalProperties": false,
      "description": "Values each app takes in at build time.",
      "properties": {
        "app_identifier": {
          "description": "Reverse DNS base of the application identifier. Each OS derives its own identifier from it.",
          "maxLength": 100,
          "pattern": "^[A-Za-z]([A-Za-z0-9-]*[A-Za-z0-9])?(\\.[A-Za-z]([A-Za-z0-9-]*[A-Za-z0-9])?)+$",
          "type": "string"
        },
        "product_name": {
          "description": "Product name shown to people, the same in every language. No control characters, no line separators, no whitespace at either end.",
          "minLength": 1,
          "type": "string"
        }
      },
      "required": [
        "product_name",
        "app_identifier"
      ],
      "type": "object"
    },
    "schemaVersion": {
      "const": 1,
      "description": "Version of this schema. Goes up by one on every schema change."
    }
  },
  "required": [
    "$schema",
    "schemaVersion",
    "build"
  ],
  "title": "Public configuration of the desktop foundation, schema version 1",
  "type": "object"
}
```

Skema itu bantuan untuk editor. Ia memuat kunci tertutup, kunci wajib, tipe, nilai tetap `$schema` dan `schemaVersion`, serta panjang dan pola `build.app_identifier`. Aturan `build.product_name` tentang whitespace dan karakter kontrol hanya ditegakkan kode pemeriksaan, dan tertulis di skema sebagai deskripsi saja. Generator membangun skema dari konstanta yang sama dengan pemeriksaan (versi skema, nama kunci, batas 100 karakter), sehingga keduanya tidak bisa berbeda.

`shared/config/README.md`:

````markdown
# Konfigurasi publik

`public.json` adalah satu satunya sumber nilai publik yang wajib sama di aplikasi macOS, Windows, dan Linux. `public.schema.json` dibuat generator untuk editor; jangan disunting dengan tangan. Aturan lengkap dan alasannya ada di [spec 0004](../../docs/specs/_root/0004-konfigurasi-publik/index.md).

## Skema versi 1

| Kunci | Tipe | Arti | Aturan |
| --- | --- | --- | --- |
| `$schema` | string | Path relatif ke skema hasil generator. | Tepat `./public.schema.json`. |
| `schemaVersion` | bilangan bulat | Versi skema yang dipakai file ini. | Sama dengan versi di baris terakhir "Riwayat skema". |
| `build.product_name` | string | Nama produk yang tampil kepada pengguna. Sama untuk semua bahasa. | Paling sedikit satu karakter. Tanpa karakter kontrol dan tanpa pemisah baris. Tidak diawali atau diakhiri whitespace. |
| `build.app_identifier` | string | Dasar pengenal aplikasi bergaya reverse DNS. | Paling banyak 100 karakter. Dua segmen atau lebih, dipisah satu titik. Tiap segmen diawali huruf ASCII, hanya memuat huruf ASCII, angka ASCII, dan tanda `-`, dan tidak diakhiri `-`. |

Semua kunci wajib ada. Kunci lain ditolak. Versi aplikasi tidak ada di sini; sumbernya `[workspace.package].version` di `Cargo.toml` root.

Belum ada nilai yang dibaca saat aplikasi berjalan dan belum ada definisi environment. Keduanya ditambah lewat pembaruan spec 0004 saat ada kebutuhan nyata.

## Cara aplikasi memakai nilai

| Nilai | Sumber | Dipakai saat | Tujuan di tiap OS |
| --- | --- | --- | --- |
| Nama produk | `build.product_name` | Build | Nama tampil aplikasi. Setting native ditetapkan spec OS. |
| Dasar pengenal aplikasi | `build.app_identifier` | Build | Pengenal aplikasi tiap OS, menurut `K-6`. |
| Versi aplikasi | `[workspace.package].version` di `Cargo.toml` root | Build | Versi yang tampil dan tercatat di aplikasi. |

- **K-1**: Nilai publik bersama hanya ditulis tangan di sumbernya, yaitu `public.json` atau `Cargo.toml` root untuk versi.
- **K-2**: Setiap salinan nilai itu di luar sumbernya wajib dibuat generator, atau dibandingkan dengan sumbernya oleh pemeriksaan otomatis. Spec OS memilih salah satunya dan menulis perintah buktinya.
- **K-3**: Semua nilai dipakai saat build. Aplikasi tidak membuka `shared/config/` saat berjalan, dan folder ini tidak ikut dikemas.
- **K-4**: Rust Core dan jembatan macOS tidak membaca konfigurasi. Bila Core kelak butuh sebuah nilai, aplikasi mengirimnya sebagai argumen API publik.
- **K-5**: Aplikasi Rust mengambil versi dari Cargo saat kompilasi. Aplikasi yang proyeknya bukan Cargo mengikuti `K-2`.
- **K-6**: Pengenal aplikasi tiap OS diturunkan dari `build.app_identifier` dengan aturan tetap yang spec OS tulis. Aturan bawaan: sama dengan nilai itu, atau nilai itu ditambah titik dan satu segmen. Bila format native menolak nilai itu, spec OS menulis pemetaan tetap dari nilai itu. Tidak ada pengenal kedua yang ditulis tangan.
- **K-7**: Tidak ada file timpaan dan tidak ada variabel lingkungan yang mengubah nilai publik. Satu nilai berlaku untuk semua orang dan semua build.

## Batas pengaturan khusus OS

Tiga pertanyaan, dijawab berurutan:

1. Apakah nilai itu rahasia, atau hanya benar di satu mesin atau untuk satu orang? Bila ya, nilai itu tidak masuk repo.
2. Apakah lebih dari satu aplikasi butuh nilai itu dengan makna yang sama? Bila ya, nilai itu masuk `public.json` lewat pembaruan spec 0004.
3. Selain itu, nilai itu pengaturan khusus OS dan tinggal di proyek native aplikasi itu.

| OS | Contoh pengaturan khusus OS |
| --- | --- |
| macOS | Versi minimum macOS, nomor build, nama target, gaya penandatanganan, sandbox, entitlement, kategori aplikasi, aturan turunan pengenal bundel. |
| Windows | Manifest aplikasi, identitas paket, versi Windows App SDK dan versi minimum Windows, aturan turunan pengenal aplikasi. |
| Linux | Isi berkas `.desktop` selain nama, kategori menu, versi minimum GTK dan libadwaita, lokasi ikon, aturan turunan pengenal aplikasi. |

Debug dan Release adalah konfigurasi build tiap toolchain, bukan environment.

## Yang tidak boleh ada di folder ini

| Kelas | Arti | Tempat | Dilacak git |
| --- | --- | --- | --- |
| Nilai publik bersama | Boleh dilihat siapa pun dan wajib sama maknanya di lebih dari satu aplikasi. | `shared/config/public.json`. Versi aplikasi di `Cargo.toml` root. | Ya |
| Pengaturan khusus OS | Hanya dimengerti satu OS, atau tiap OS punya nilainya sendiri. | Proyek native di `apps/<os>/`. | Ya |
| Pengaturan lokal | Hanya berlaku di satu mesin atau untuk satu orang. | File yang diabaikan git atau variabel lingkungan shell. | Tidak |
| Rahasia | Memberi akses atau membuktikan identitas. | Keychain di mesin pengembang atau penyimpanan rahasia CI. Tidak pernah di repo. | Tidak |

Hanya kelas pertama yang boleh ada di folder ini. Folder ini berisi tepat tiga file: `public.json`, `public.schema.json`, dan `README.md`. File atau subfolder lain membuat validasi gagal, kecuali `.DS_Store` yang diabaikan.

## Cara mengubah

- Memeriksa: `cargo run -p desktop-codegen --locked -- config check` dari root repo.
- Mengganti nilai: sunting `public.json`, jalankan pemeriksaan di atas, lalu jalankan generator atau pemeriksaan tiap OS supaya salinannya ikut. Versi skema tidak naik.
- Mengubah skema (menambah, menghapus, atau mengganti nama kunci, atau mengubah aturan): perbarui spec 0004 lebih dulu, lalu dalam satu commit ubah aturan dan versi di `tooling/codegen/src/config/mod.rs`, naikkan `schemaVersion` di `public.json` sebesar satu, tambah satu baris di "Riwayat skema", perbarui tabel "Skema versi" di atas, jalankan `cargo run -p desktop-codegen --locked -- config generate`, dan sesuaikan setiap konsumen.

## Riwayat skema

| Versi | Tanggal | Perubahan | Spec |
| --- | --- | --- | --- |
| 1 | 2026-11-15 | Skema pertama: `build.product_name` dan `build.app_identifier`. | 0004 |
````

### Aturan versi skema

- Versi skema adalah bilangan bulat. Versi pertama `1`.
- Setiap perubahan skema menaikkan versi sebesar satu: menambah, menghapus, atau mengganti nama kunci atau grup, dan mengubah tipe atau aturan sebuah kunci.
- Mengganti nilai tidak menaikkan versi. Riwayat nilai ditelusuri dengan `git log -- shared/config/public.json`.
- Nomor versi ditulis tangan di tiga tempat: `schemaVersion` di `public.json`, baris terakhir "Riwayat skema" di `README.md`, dan satu konstanta di kode pemeriksaan (versi yang dikenal). `const` di `public.schema.json` dibuat generator dari konstanta itu. Pemeriksaan hanya mengenal satu versi, yaitu versi terkini.
- Kenaikan versi terjadi dalam satu commit yang memuat ketiga tempat itu, skema hasil generator, tabel skema di `README.md`, aturan dan uji pemeriksaan, dan setiap konsumen yang terdampak. Tidak ada masa transisi, karena semua konsumen ada di repo ini dan dibangun dari commit yang sama.
- Perubahan skema selalu didahului pembaruan spec ini.

### Validasi

Yang diperiksa:

| ID | Aturan |
| --- | --- |
| `R-1` | `shared/config/` memuat file `public.json` dan file `README.md`, keduanya wajib ada. Selain itu hanya `public.schema.json` yang boleh ada. Entri bernama `.DS_Store` diabaikan. File lain dan subfolder apa pun ditolak. Ada tidaknya `public.schema.json` dan isinya diperiksa `check` sebagai keluaran generator, bukan oleh aturan ini. |
| `R-2` | `public.json` adalah teks UTF-8 berisi satu objek JSON yang memenuhi tabel "Skema versi 1": kunci tertutup tanpa kunci ganda, semua kunci wajib ada, tipe benar, `$schema` tepat `./public.schema.json`, dan aturan `build.product_name` serta `build.app_identifier` terpenuhi. |
| `R-3` | Nomor versi skema sama di tiga tempat: konstanta versi yang dikenal di kode pemeriksaan (sekarang `1`), `schemaVersion` di `public.json`, dan versi di `README.md`. Versi di `README.md` adalah bilangan di sel pertama baris terakhir yang diawali `| ` lalu angka. Bila baris seperti itu tidak ada, aturan ini gagal. |

Syarat untuk pemeriksaan:

- Fungsi `plan` sumber `config` menerima root repo sebagai masukan dan hanya membaca `<root>/shared/config/`, sehingga uji bisa memberinya root sementara lewat `(config::SOURCE.plan)(&root)`.
- `plan` menjalankan ketiga aturan dan mengembalikan satu `Problem` untuk setiap pelanggaran, bukan hanya yang pertama. Bila `public.json` tidak ada atau bukan JSON yang bisa dibaca, aturan yang butuh isinya dilewati.
- Tiap `Problem` memakai path file dari root repo (misalnya `shared/config/public.json`), dan pesannya diawali ID aturan lalu menyebut nama kunci atau nama entri yang salah, misalnya `R-1: unexpected entry` untuk path `shared/config/local.json`. Susunan baris keluaran dan kode keluar mengikuti alat: tiap `Problem` dicetak ke stderr sebagai `<path>: <pesan>`, disusul baris `config: failed, <n> problems` (kata `problems` selalu jamak), dan kode keluar 1 untuk sumber yang tidak sah atau keluaran yang menyimpang.
- Pemeriksaan berjalan di ketiga host tanpa SDK UI dan tanpa jaringan.

Kasus yang ditolak. Tiap baris berangkat dari root sementara yang `shared/config/` nya berisi tiga file "Isi file" dengan nilai sementara, lalu diubah seperti kolom "Masukan". Salah satu pesan wajib memuat ID di kolom "Aturan"; pesan lain boleh ada.

| No. | Masukan | Aturan |
| --- | --- | --- |
| T-1 | File tambahan `local.json` di folder | `R-1` |
| T-2 | File tambahan `.env` di folder | `R-1` |
| T-3 | Subfolder `signing/` di folder | `R-1` |
| T-4 | `public.json` tidak ada | `R-1` |
| T-5 | `README.md` tidak ada | `R-1` |
| T-6 | `public.json` berisi teks `{` | `R-2` |
| T-7 | `public.json` berisi `[]` | `R-2` |
| T-8 | Objek teratas mendapat kunci `runtime` berisi `{}` | `R-2` |
| T-9 | `build` mendapat kunci `api_url` berisi `"https://example.com"` | `R-2` |
| T-10 | `build` tanpa `product_name` | `R-2` |
| T-11 | `$schema` adalah `"./other.schema.json"` | `R-2` |
| T-12 | `build.product_name` adalah `""` | `R-2` |
| T-13 | `build.product_name` adalah `" Desktop Foundation"` (spasi di awal) | `R-2` |
| T-14 | `build.product_name` adalah `"Desktop\nFoundation"` (baris baru di tengah) | `R-2` |
| T-15 | `build.product_name` adalah `"Desktop\u2028Foundation"` | `R-2` |
| T-16 | `build.product_name` adalah `"\ufeffDesktop Foundation"` | `R-2` |
| T-17 | `build.product_name` adalah bilangan `1` | `R-2` |
| T-18 | `build` memuat kunci `product_name` dua kali | `R-2` |
| T-19 | `build.app_identifier` adalah `desktop` (satu segmen) | `R-2` |
| T-20 | `build.app_identifier` adalah `com.1example.app` (segmen diawali angka) | `R-2` |
| T-21 | `build.app_identifier` adalah `com.example.app_x` (garis bawah) | `R-2` |
| T-22 | `build.app_identifier` adalah `com.example.app-` (segmen diakhiri `-`) | `R-2` |
| T-23 | `build.app_identifier` sepanjang 101 karakter: `com.` diikuti 97 huruf `a` | `R-2` |
| T-24 | `schemaVersion` adalah string `"1"` | `R-2` |
| T-25 | `schemaVersion` di `public.json` adalah `2`, `README.md` tetap | `R-3` |
| T-26 | Baris terakhir "Riwayat skema" di `README.md` diawali `| 2 |`, `public.json` tetap | `R-3` |
| T-27 | `schemaVersion` di `public.json` adalah `2` dan baris terakhir "Riwayat skema" diawali `| 2 |` | `R-3` |

Kasus yang diterima (`plan` lulus). Uji ini tidak menghitung keluaran `plan` dan tidak mengunci daftar path keluarannya, karena spec OS menambah keluaran sendiri:

| No. | Masukan |
| --- | --- |
| T-28 | Root sementara yang sah dengan nilai sementara dari "Isi file" |
| T-29 | `build.product_name` adalah `Ünï 名前`, `build.app_identifier` adalah `Com.Example.App` |
| T-30 | Folder sah ditambah file `.DS_Store` |
| T-31 | `build.app_identifier` sepanjang 100 karakter: `com.` diikuti 96 huruf `a` |
| T-32 | Folder sah tanpa `public.schema.json`. `plan` lulus; ketiadaan keluaran ditangkap `check`, bukan `plan`. |

#### Bentuk alat

Pemeriksaan dan generator memakai paket `desktop-codegen` di `tooling/codegen/`, yang dibuat milestone 1 [spec `_root/0003`](../0003-tema-bersama-dan-kontrol-native/index.md) (scope 008) dan sudah ada di `main`. Spec ini menambah satu sumber bernama `config` ke alat itu dan tidak membuat alat, paket, skrip, atau dependensi sendiri. Nilai berikut sudah dicocokkan dengan `tooling/codegen/src/lib.rs`, `main.rs`, dan `json.rs` di `main` pada 2026-11-15; bila alat itu berubah, bagian ini ikut diperbarui.

| Hal | Nilai |
| --- | --- |
| Modul baru | `tooling/codegen/src/config/mod.rs`, mengekspor `pub const SOURCE: Source = Source { name: "config", plan };` dengan `fn plan(root: &Path) -> Result<Plan, Vec<Problem>>`. Tipe `Source`, `Plan`, `Output`, dan `Problem` datang dari akar crate. |
| Pendaftaran | Di `tooling/codegen/src/lib.rs`: satu baris baru `pub mod config;`, dan `config::SOURCE` ditambahkan sesudah `tokens::SOURCE` di `pub const SOURCES: &[Source]`. Subperintah dan baris usage `sources: tokens, config` mengikuti `SOURCES`. |
| Bentuk yang diterima | Tipe Rust `serde` dengan `#[serde(deny_unknown_fields)]` untuk objek teratas dan `build`, ditambah pemeriksaan tulis tangan untuk `R-1`, `R-3`, nilai `$schema`, dan aturan dua nilai `build`. Kesalahan baca dari `serde` dilaporkan sebagai `R-2`. |
| Konstanta | Versi skema yang dikenal `1`, batas panjang `build.app_identifier` `100`, dan tiga path file di `shared/config/`. Pemeriksaan dan skema hasil generator memakai konstanta yang sama. |
| Aturan keluaran | Skema mematuhi O-1 sampai O-7 spec `_root/0003`. Penandanya kunci `$comment` dengan teks di blok "Isi file". |
| Keluaran `plan` | Untuk spec ini satu `Output` dengan path `shared/config/public.schema.json` dan isi dari `desktop_codegen::json::to_canonical_json`, serta `summary` berisi `schema version 1, 2 build values`. Spec OS menambah keluarannya sesudah itu; yang pertama `apps/macos/Config/Shared.xcconfig` dari [spec `macos/0004`](../../macos/0004-konfigurasi-publik/index.md). |
| Perintah periksa | `cargo run -p desktop-codegen --locked -- config check`. Kode 0 lulus, 1 sumber tidak sah atau keluaran menyimpang, 2 salah pakai. Tidak menulis apa pun. Baris sukses: `config: ok, schema version 1, 2 build values, <n> outputs up to date`. |
| Perintah generator | `cargo run -p desktop-codegen --locked -- config generate`. Menulis hanya keluaran yang hilang atau berbeda, membuat folder induk bila belum ada, dan mencetak `wrote <path>` per file. Baris sukses: `config: ok, schema version 1, 2 build values, <n> outputs, <w> written`. Sumber yang tidak sah tidak menulis apa pun. |
| Uji | `tooling/codegen/tests/config_contract.rs`: uji `committed_outputs_match_the_source` yang memanggil `check(&repo_root(), &config::SOURCE)`, dan satu uji per baris T-1 sampai T-32. Uji hanya memakai API publik paket dan pustaka standar. |
| Root sementara di uji | Folder di bawah `env!("CARGO_TARGET_TMPDIR")` bernama `config_contract_` ditambah nama uji, seperti di `tokens_contract.rs`. Folder lama dihapus di awal uji lalu dibuat ulang. Tanpa crate tambahan. |
| Akhir baris | Satu baris `/shared/config/**/*.json text eol=lf` ditambahkan ke `.gitattributes` root, sesudah baris yang sudah ada. Barisnya per jenis file, menurut aturan O-6 spec `_root/0003`. `README.md` tidak mendapat baris; `R-3` membacanya dengan mengabaikan `\r` di akhir baris. |
| Gerbang | `cargo test --workspace --locked` yang sudah ada ikut menjalankan `config_contract`, jadi skema yang menyimpang atau sumber yang tidak sah menggagalkan gerbang. |

Batas yang berlaku datang dari spec [_root/0001](../0001-batas-workspace-dan-pemilik/index.md): tanpa `.cargo/config.toml`, paket berawalan `desktop-`, aturan netral host, dan `desktop-core` tetap tanpa dependensi. `desktop-codegen` tidak bergantung pada anggota workspace lain dan tidak ada anggota yang bergantung padanya.

Bila sebuah spec OS memilih generator untuk memenuhi `K-2`, keluaran native OS itu ditambahkan ke `plan` sumber `config` lewat spec OS itu, sehingga `check` yang sama membandingkannya.

### Cara aplikasi memakai nilai

| Nilai | Sumber | Dipakai saat | macOS | Windows | Linux |
| --- | --- | --- | --- | --- | --- |
| Nama produk | `build.product_name` | Build | Nama tampil bundel dan judul jendela, lewat berkas hasil generator `apps/macos/Config/Shared.xcconfig` ([spec `macos/0004`](../../macos/0004-konfigurasi-publik/index.md)). | Menunggu host | Menunggu host |
| Dasar pengenal aplikasi | `build.app_identifier` | Build | Pengenal bundel menurut `K-6`: nilai itu ditambah `.macos` (spec `macos/0004`). | Menunggu host | Menunggu host |
| Versi aplikasi | `[workspace.package].version` di `Cargo.toml` root | Build | `MARKETING_VERSION` di berkas hasil generator yang sama, mengikuti `K-2` (spec `macos/0004`). | `CARGO_PKG_VERSION` saat kompilasi | `CARGO_PKG_VERSION` saat kompilasi |

Aturan konsumsi:

- **K-1**: Nilai publik bersama hanya ditulis tangan di sumbernya, yaitu `public.json` atau `Cargo.toml` root untuk versi.
- **K-2**: Setiap salinan nilai itu di luar sumbernya wajib dibuat generator, atau dibandingkan dengan sumbernya oleh pemeriksaan otomatis. Spec OS memilih salah satunya dan menulis perintah buktinya.
- **K-3**: Semua nilai dipakai saat build. Aplikasi tidak membuka `shared/config/` saat berjalan, dan folder ini tidak ikut dikemas.
- **K-4**: Rust Core dan jembatan macOS tidak membaca konfigurasi. Bila Core kelak butuh sebuah nilai, aplikasi mengirimnya sebagai argumen API publik.
- **K-5**: Aplikasi Rust mengambil versi dari Cargo saat kompilasi. Aplikasi yang proyeknya bukan Cargo mengikuti `K-2`.
- **K-6**: Pengenal aplikasi tiap OS diturunkan dari `build.app_identifier` dengan aturan tetap yang spec OS tulis. Aturan bawaan: sama dengan nilai itu, atau nilai itu ditambah titik dan satu segmen. Bila format native menolak nilai itu, spec OS menulis pemetaan tetap dari nilai itu. Tidak ada pengenal kedua yang ditulis tangan.
- **K-7**: Tidak ada file timpaan dan tidak ada variabel lingkungan yang mengubah nilai publik. Satu nilai berlaku untuk semua orang dan semua build.

Rincian `K-6`:

- Segmen tambahan mengikuti aturan segmen `build.app_identifier`: diawali huruf ASCII, hanya huruf ASCII, angka ASCII, dan tanda `-`, tidak diakhiri `-`.
- Pemetaan tetap adalah fungsi dari `build.app_identifier` saja, misalnya membuang tanda `-` atau mengganti huruf besar kecil. Spec OS menulis aturannya dan batas panjang hasilnya di OS itu.
- Hasil turunan tiap OS adalah salinan, jadi tunduk pada `K-2`.

Salinan tulis tangan di `project.pbxproj` aplikasi macOS melanggar `K-2` sampai milestone 1 spec `macos/0004` dibangun. Sesudah itu salinan macOS dibuat generator sebagai keluaran kedua sumber `config`, dan `check` yang sama menjaganya. Selama masa antara, nilainya sudah sama: nama `Desktop Foundation`, pengenal `com.example.desktop-foundation` ditambah `.macos`, dan versi `0.1.0`.

Nama produk bukan teks yang diterjemahkan dan bukan berkas merek. Katalog bahasa (scope 010) tidak menyalinnya, dan aset merek (scope 011) hanya memegang ikon dan berkas gambar.

### Nilai dan sumbernya

| Aksi | Nilai | Sumber |
| --- | --- | --- |
| Menulis `public.json` | `$schema` | `./public.schema.json`, path relatif ke skema hasil generator di folder yang sama. |
| Menulis `public.json` | `schemaVersion` | `1`, ditetapkan spec ini. |
| Menulis `public.json` | `build.product_name` | `NAMA_PRODUK` dari "Nilai dari user". Nilai sementara `Desktop Foundation`, yaitu nama tampil bundel di spec `macos/0001`. |
| Menulis `public.json` | `build.app_identifier` | `DASAR_PENGENAL` dari "Nilai dari user". Nilai sementara `com.example.desktop-foundation`, yaitu pengenal bundel sementara di spec `macos/0001` tanpa `.macos`. |
| Menulis `README.md` | Tanggal di "Riwayat skema" | Tanggal spec yang menetapkan versi itu. Untuk versi 1: `2026-11-15`. |
| Menulis `README.md` | Nomor spec di "Riwayat skema" | Nomor spec ini, `0004`. |
| Build aplikasi | Versi aplikasi | `[workspace.package].version` di `Cargo.toml` root, sekarang `0.1.0` (spec `_root/0001`). |
| Build aplikasi | Pengenal aplikasi tiap OS | `build.app_identifier` menurut `K-6`. Aturan turunan ditetapkan spec OS. |
| Validasi | Versi skema yang dikenal | Konstanta di kode pemeriksaan, sekarang `1`, ditetapkan spec ini. |
| Validasi | Versi di `README.md` | Sel pertama baris terakhir yang diawali `| ` lalu angka. |
| Validasi | Root yang diperiksa | Argumen `root` fungsi `plan`. Perintah repo memberinya root repo dari alat; uji memberinya root sementara. |
| Generator | Isi `public.schema.json` | Dibangun dari konstanta pemeriksaan dan teks tetap di blok "Isi file". |
| Generator | Baris ringkasan `check` | `summary` dari `plan`: `schema version 1, 2 build values`. |
| Validasi gagal | ID aturan di pesan | Tabel aturan `R-1` sampai `R-3`. |
| Validasi gagal | Path file atau nama kunci di pesan | Masukan yang diperiksa. |

Nilai dari user:

| Nama | Arti | Nilai sementara | Siapa yang tahu |
| --- | --- | --- | --- |
| `NAMA_PRODUK` | Nama produk yang tampil kepada pengguna di ketiga OS. | `Desktop Foundation` | User |
| `DASAR_PENGENAL` | Dasar pengenal aplikasi bergaya reverse DNS, dari domain yang dikuasai pemilik produk. | `com.example.desktop-foundation` | User |

Nilai sementara cukup untuk build lokal. `DASAR_PENGENAL` wajib diganti sebelum penandatanganan atau distribusi.

### Invarian

- `shared/config/` berisi tepat tiga file dan tidak pernah memuat pengaturan khusus OS, pengaturan lokal, atau rahasia.
- `public.json` selalu memenuhi skema versi terkini, dan versi skema sama di setiap tempat ia tercatat.
- `public.schema.json` selalu sama dengan keluaran generator dan tidak pernah disunting tangan.
- Versi aplikasi hanya ditulis tangan di `[workspace.package].version`.
- Tidak ada aplikasi yang membuka `shared/config/` saat berjalan.
- `desktop-core` dan `desktop-macos-ffi` tidak membaca dan tidak memuat nilai konfigurasi.
- Tidak ada nilai publik yang berbeda antara dua build dari commit yang sama.

### Keamanan

Isi `shared/config/` bersifat publik: siapa pun yang bisa membaca repo boleh melihatnya, dan nilai itu memang tampil kepada pengguna aplikasi. Perubahan lewat commit dan review seperti file lain. Skema tertutup dan daftar file tertutup mencegah kunci dan file yang belum diputuskan, tetapi tidak bisa mengenali rahasia yang ditempel ke dalam nilai yang sah; itu tetap dijaga review. Sertifikat, kunci privat, kredensial notarisasi, dan token CI tidak pernah masuk repo. Spec ini tidak menambah variabel lingkungan dan tidak menambah kredensial.

### Kasus gagal yang sudah diputuskan

| Kejadian | Perilaku yang diharapkan |
| --- | --- |
| User memberi nama produk atau dasar pengenal akhir | Sunting `public.json` saja, jalankan validasi, lalu generator atau pemeriksaan tiap OS. Versi skema tidak naik. Di macOS, `config generate` menulis ulang `apps/macos/Config/Shared.xcconfig` (spec `macos/0004`); sebelum milestone 1 spec itu dibangun, salinan di `project.pbxproj` masih diganti dengan tangan. |
| Sebuah aplikasi butuh nilai bersama baru | Perbarui spec ini lebih dulu, lalu ikuti "Aturan versi skema". |
| Nilai baru hanya dibutuhkan satu OS | Pengaturan khusus OS. Tinggal di proyek native, lewat spec OS itu. |
| Pengembang butuh pengenal lain untuk penandatanganan di mesinnya | Pengaturan lokal di file native yang diabaikan git, ditetapkan spec OS. `public.json` tidak diubah. |
| Sebuah nilai harus berbeda per environment atau dibaca saat berjalan | Pembaruan spec ini menambah grup yang sesuai dan menaikkan versi skema. Sebelum itu nilai seperti itu tidak boleh ada. |
| Versi workspace naik | Ubah `[workspace.package].version`. Salinan di proyek bukan Cargo ikut lewat `K-2`. |
| Format native sebuah OS menolak `build.app_identifier` | Spec OS itu menulis pemetaan tetap menurut `K-6`. Bila pemetaan tidak mungkin, spec ini mempersempit aturan nilai dan menaikkan versi skema. |
| Format native sebuah OS menolak `build.product_name` | Aturan keluaran OS itu menolaknya saat `config check`, karena nama produk harus sama di semua OS dan tidak bisa dipetakan. Di macOS itu aturan `M-1` spec `macos/0004`. Ganti namanya, atau perbarui spec OS itu. |
| Konflik merge di `public.json` | Ambil satu sisi, lalu jalankan validasi. |
| File baru di `shared/config/` membuat hook gagal dengan `Graphify extraction failed` | Ikuti baris yang sama di "Kasus gagal yang sudah diputuskan" spec `_root/0001`. File JSON di root sudah lolos hook, jadi ini tidak diharapkan terjadi. |
| Bentuk alat di spec `_root/0003` berubah sebelum build spec ini | Bagian "Bentuk alat" diperbarui lebih dulu. Developer tidak membuat alat kedua. |
| `check` gagal karena `public.schema.json` berbeda dari keluaran generator | Jalankan `generate` dan commit hasilnya. Skema tidak disunting tangan. |

### Skenario uji kritis

- Jalur utama: repo dengan tiga file dari "Isi file" lulus `config check`, dan `generate` tidak mengubah apa pun, memverifikasi **AC-1**, **AC-2**, **AC-4**.
- Versi: nomor versi sama di tiga tempat, dan mengubah satu atau dua di antaranya membuat validasi gagal dengan `R-3`, memverifikasi **AC-3**, **AC-5**.
- Kunci tak dikenal: `runtime` atau `build.api_url` ditolak dengan `R-2`, memverifikasi **AC-5**, **AC-9**.
- File tersesat: `local.json`, `.env`, atau subfolder di `shared/config/` ditolak dengan `R-1`, memverifikasi **AC-4**, **AC-5**, **AC-6**.
- Batas Core: `git grep` di `crates/core/src`, `crates/macos-ffi/src`, dan `bindings` tidak menemukan rujukan ke konfigurasi, memverifikasi **AC-7**.
- Dokumen: tiga heading di `shared/config/README.md` dan teks baru di dua dokumen rujukan ada, memverifikasi **AC-6**, **AC-7**, **AC-8**.
- Gerbang: `cargo fmt`, `cargo clippy`, dan `cargo test --workspace`, memverifikasi **AC-10**.

## Perubahan dokumen

Langkah 3 Build plan mengubah dua dokumen rujukan. Bagian lain dokumen itu tidak diubah.

1. Di `docs/shared-resources/README.md`, ganti seluruh paragraf di bawah heading `## Konfigurasi` dengan:

   ```markdown
   `shared/config/public.json` adalah sumber nilai publik yang wajib sama di ketiga aplikasi. Skemanya tertutup dan bernomor versi, dan `shared/config/README.md` menjelaskan tiap kunci serta cara mengubahnya. Fondasi saat ini hanya butuh dua nilai, yaitu nama produk dan dasar pengenal aplikasi. Versi aplikasi tetap bersumber dari `[workspace.package]` di `Cargo.toml` root. Semua nilai dipakai saat build. Belum ada nilai yang dibaca saat aplikasi berjalan dan belum ada definisi environment; keduanya ditambah lewat kenaikan versi skema saat ada kebutuhan nyata. Pengaturan khusus OS, termasuk izin, tinggal di proyek native tiap aplikasi. Pengaturan lokal tinggal di file yang diabaikan git. Kunci privat dan kredensial penandatanganan tetap di luar repo. Layanan konfigurasi jarak jauh dan mesin feature flag ditunda. Rincian dan alasannya ada di spec 0004.
   ```

   Jadikan kata `spec 0004` di kalimat terakhir tautan ke `docs/specs/_root/0004-konfigurasi-publik/index.md`; dari dokumen itu path relatifnya diawali `../specs/_root/`.

2. Di `docs/architecture/README.md` bagian "Tata letak yang dituju", di paragraf yang diawali "Path ini menunjukkan", ada kalimat berisi daftar lokasi yang berakhir "sudah ada.". Sisipkan `` `shared/config/`, `` tepat sebelum kata `dan` terakhir di kalimat itu. Lokasi lain yang sudah tercantum di kalimat itu, termasuk yang ditambah spec lain, tidak diubah. Dengan isi `main` pada 2026-11-15 kalimatnya menjadi:

   ```markdown
   Root workspace, `crates/core/`, `crates/macos-ffi/`, `bindings/swift/`, `shared/config/`, dan `apps/macos/` sudah ada.
   ```

3. Di `docs/architecture/README.md` tabel `### Yang sudah ada`, tambahkan satu baris sesudah baris terakhir:

   ```markdown
   | `shared/config/` | Sumber nilai publik `public.json`, skema hasil generator, dan aturan pakainya dari spec 0004. |
   ```

   Jadikan kata `spec 0004` tautan ke `docs/specs/_root/0004-konfigurasi-publik/index.md` dengan path relatif yang diawali `../specs/_root/`.

## Build plan

Pendekatan build proyek adalah Tracer Bullet. Untuk fitur ini benang tipisnya adalah satu alur utuh: sumber nilai, pemeriksaan yang menjaganya, dan satu keluaran generator. Dokumen rujukan menyusul sesudah alur itu nyata. Konsumen pertama datang dari spec macOS scope 009. Satu langkah menjadi satu commit.

**Milestone 1: Sumber nilai, pemeriksaan, dan skema hasil generator** (langkah 1 dan 2, sesudah milestone 1 spec `_root/0003` ada di `main`)

1. Dalam satu commit: buat `shared/config/public.json` dan `shared/config/README.md` persis seperti "Isi file" dengan nilai dari tabel "Nilai dari user"; tambahkan sumber `config` ke `desktop-codegen` menurut "Bentuk alat" dengan aturan `R-1`, `R-2`, dan `R-3`; tambahkan baris `.gitattributes`; jalankan `cargo run -p desktop-codegen --locked -- config generate` untuk menulis `shared/config/public.schema.json`; dan tulis `tooling/codegen/tests/config_contract.rs` dengan uji T-1 sampai T-32 serta `committed_outputs_match_the_source`. Sebelum commit, perintah AC-4, AC-5, dan AC-10 harus lulus, satisfies **AC-1**, **AC-2**, **AC-3**, **AC-4**, **AC-5**, **AC-6**, **AC-7**, **AC-9**, **AC-10**
2. Dengan `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, jalankan ulang perintah AC-1 sampai AC-7, AC-9, dan AC-10 pada commit langkah 1, catat perintah dan keluarannya di `verify.md` bagian `## Milestone 1: Sumber nilai, pemeriksaan, dan skema hasil generator`, lalu hapus `target/`, satisfies **AC-1**, **AC-2**, **AC-3**, **AC-4**, **AC-5**, **AC-6**, **AC-7**, **AC-9**, **AC-10**

**Milestone 2: Dokumen rujukan mencatat sumber yang nyata** (langkah 3, sesudah milestone 1)

3. Perbarui `docs/shared-resources/README.md` dan `docs/architecture/README.md` sesuai bagian "Perubahan dokumen", lalu catat perintah AC-8 dan AC-9 beserta keluarannya di `verify.md` bagian `## Milestone 2: Dokumen rujukan mencatat sumber yang nyata`, satisfies **AC-8**, **AC-9**

## Consequences

**Positive**:

- Nilai yang hanya user tahu punya satu tempat. Mengganti nama produk atau pengenal adalah satu suntingan.
- Skema tertutup dan daftar file tertutup membuat nilai lokal, rahasia, atau kunci yang belum diputuskan gagal di pemeriksaan, bukan di review saja.
- Scope 006 dan 007 mulai dengan aturan yang sudah ada, jadi aplikasi Windows dan Linux tidak menyalin nilai dengan tangan.
- Versi aplikasi tetap punya satu sumber yang sudah dipakai Cargo.
- Pemeriksaan memakai alat yang sama dengan token, tanpa dependensi baru dan tanpa perubahan `Cargo.lock`.
- Skema untuk editor dibuat dari konstanta yang sama dengan pemeriksaan, jadi keduanya tidak bisa berbeda.

**Negative / tradeoffs**:

- Satu file sumber, satu skema hasil generator, dan satu modul pemeriksaan untuk dua nilai. Itu harga satu sumber.
- Tiap OS tetap butuh langkah sendiri untuk memindahkan nilai ke bentuk native. Sampai spec macOS scope 009 dibangun, salinan di `project.pbxproj` belum dibandingkan.
- Setiap kunci baru menuntut pembaruan spec dan kenaikan versi skema. Itu disengaja, tetapi melambatkan penambahan nilai.
- Aturan skema tertulis di tiga tempat (spec ini, `README.md`, dan kode pemeriksaan). Ketiganya dijaga tetap sama oleh AC-1 dan uji T-1 sampai T-32.
- Skema hasil generator tidak menegakkan aturan whitespace dan karakter kontrol `build.product_name`, jadi editor bisa menerima nama yang ditolak `check`.
- Build spec ini tidak bisa berjalan paralel dengan milestone 1 spec `_root/0003`, karena `tooling/codegen/src/lib.rs` dan `.gitattributes` dipakai bersama.
- Aturan `build.app_identifier` dan aturan `K-6` belum diuji di host Windows dan Linux. Sebagian format di sana lebih sempit, jadi spec OS itu mungkin butuh pemetaan.
- Tidak ada timpaan lokal. Pengembang yang butuh pengenal lain di mesinnya bergantung pada mekanisme native yang ditetapkan spec OS.

**Neutral**:

- `docs/shared-resources/README.md` dan `docs/architecture/README.md` biasanya terlarang bagi sub sesi. Langkah 3 butuh izin eksplisit di brief developer.
- `.gitattributes` adalah file root yang dibuat spec `_root/0003`. Spec ini hanya menambah satu baris.
- Nama produk berpindah dari urusan scope 011 (nama tampil bundel di spec `macos/0001`) dan scope 010 (literal judul jendela `Desktop Foundation` di spec yang sama) ke spec ini. Kedua scope itu perlu tahu.
- `/develop` menulis `verify.md` saat build. Spec ini tidak membuatnya.

## Follow-up

- [x] Jawaban atas Follow-up Scope 009 di spec `macos/0001` ("konfigurasi publik untuk aplikasi macOS. Belum ada yang dibaca aplikasi ini."): sumber bersama dan aturannya ditetapkan di sini. Cara aplikasi macOS memakainya milik spec macOS scope 009 di bawah.
- [x] Jawaban atas Follow-up Scope 012 di spec `macos/0001` tentang `MARKETING_VERSION`: sumber versi tetap `[workspace.package].version`, dan salinannya wajib mengikuti `K-2`. Spec macOS scope 009 memilih generator atau pemeriksaan; scope 012 menjalankannya di skrip dan CI.
- [x] Spec macOS scope 009: dijawab di [spec `macos/0004`](../../macos/0004-konfigurasi-publik/index.md) (2026-11-15). `K-2` dipenuhi dengan generator, akhiran `.macos` dipertahankan, judul jendela ikut nama produk, `PRODUCT_NAME` tidak ikut, dan belum ada berkas untuk pengaturan lokal.
- [ ] Scope 006 (menunggu host Windows): cara `apps/windows/` memakai dua nilai itu dan versi dari Cargo, dan aturan turunan pengenal. Format Windows belum diperiksa: untuk aplikasi berpaket, nama identitas paket dan ID aplikasi di manifest diduga lebih sempit dari `build.app_identifier` (tanpa tanda `-`, lebih pendek), jadi spec itu kemungkinan butuh pemetaan tetap menurut `K-6`. Validasi belum dijalankan di host Windows.
- [ ] Scope 007 (menunggu host Linux): cara `apps/linux/` memakai dua nilai itu dan versi dari Cargo, dan aturan turunan pengenal. ID aplikasi GTK mengizinkan tanda `-` tetapi tidak menyarankannya, jadi spec itu memutuskan apakah butuh pemetaan. Validasi belum dijalankan di host Linux.
- [ ] Scope 010: katalog bahasa tidak memuat nama produk. Teks UI yang menyebut nama produk, termasuk judul jendela, mengambilnya dari nilai native turunan `build.product_name`.
- [ ] Scope 011: aset merek memegang ikon dan berkas gambar. Nama tampil bundel sudah dijawab di sini; baris Follow-up Scope 011 di spec `macos/0001` yang menyebut "nama tampil bundel" perlu disesuaikan pemilik spec itu.
- [ ] Scope 012: skrip `scripts/validate.sh` dan CI memanggil `cargo run -p desktop-codegen --locked -- config check`; penyimpanan rahasia CI untuk kredensial penandatanganan ditetapkan di sana atau di spec distribusi.
- [ ] `/sync` atau lead, sesudah milestone 1: `AGENTS.md` root masih berkata sumber bersama adalah rencana. Cantumkan `shared/config/`, perintah `cargo run -p desktop-codegen --locked -- config check`, dan aturan empat kelas di sana, dengan spec ini sebagai pemiliknya.
- [ ] Scope 002 (spec pemeriksaan arsitektur, bila ditulis): jadikan `K-4` pemeriksaan tetap (Core dan jembatan tidak merujuk `shared/config/`).
- [x] Bagian "Bentuk alat", AC-4, AC-5, dan AC-9 dicocokkan dengan paket `desktop-codegen` yang sudah ada di `main` (2026-11-15). Bila API `Source`, perintah, atau aturan keluaran spec `_root/0003` berubah lagi, bagian itu diperbarui di sini.
- [ ] Nilai dari user: `NAMA_PRODUK` dan `DASAR_PENGENAL` akhir. `N4` di file langkah manual koordinasi (pengenal bundel akhir macOS) sekarang berarti `DASAR_PENGENAL` yang diturunkan dengan aturan spec macOS.
- [ ] Lead: tambahkan tautan balik ke spec ini di berkas scope 009.
