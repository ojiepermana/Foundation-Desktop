# 0007. Aset merek bersama di macOS

**Scope**: [011 - Aset merek bersama](../../../scope/011%20-%20Aset%20merek%20bersama.md)
**Date**: 2026-11-15
**Status**: Proposed

## Summary

Spec ini memasukkan ikon aplikasi dari sumber merek bersama ke bundel aplikasi Mac. Sumber `assets` di `desktop-codegen` mendapat satu emitter (penulis file untuk satu target) yang menulis dokumen ikon Icon Composer, yaitu folder `apps/macos/Resources/AppIcon.icon/` berisi `icon.json` dan salinan SVG sumber. Keduanya teks, jadi alat tidak butuh dependensi baru dan `assets check` membandingkannya byte demi byte. Xcode 27.1 lalu mengubah dokumen itu menjadi `AppIcon.icns` dan `Assets.car` di dalam bundel saat build, jadi repo tidak menyimpan berkas biner apa pun. Hampir semuanya terbukti tanpa jendela. Yang harus kamu lihat sendiri hanya ikon di Dock, Finder, dan jendela About. Aturan sumber, penamaan, dan tujuan ekspor milik [spec `_root/0005`](../../_root/0005-aset-merek-bersama/index.md) dan tidak diputuskan ulang di sini.

## Requirements

**User stories**:

* Sebagai pengguna Mac, saya ingin aplikasi punya ikonnya sendiri di Dock, Finder, dan jendela About, bukan ikon kosong bawaan sistem.
* Sebagai pemilik produk, saya ingin mengganti ikon di satu berkas sumber, lalu satu perintah membawanya ke aplikasi Mac.
* Sebagai pemeriksa, saya ingin bukti tanpa jendela bahwa ikon di bundel berasal dari sumber bersama, dan daftar pendek hal yang harus dilihat mata.

**Acceptance criteria**:

Angka kriteria merujuk tabel di [Penerimaan dan bukti fondasi](../../../quality/README.md). Aturan **E-1** sampai **E-8**, **A-1** sampai **A-3**, dan **S-1** sampai **S-7** milik spec `_root/0005`. Aturan keluaran **O-1** sampai **O-7** milik spec `_root/0003`. Semua perintah dijalankan dari akar repo pada Mac Apple Silicon. `SPEC=docs/specs/macos/0007-aset-merek-bersama/index.md`, `C=$(git log --format=%H --grep='spec macos/0007 step 1' | tail -1)`, dan `B=$C^`, yaitu commit sebelum langkah 1. Perintah yang memakai `$B` dan `$C` membandingkan dua commit itu, jadi hasilnya tidak berubah oleh merge `main` sesudahnya. Bukti setiap AC dicatat bersama host dan versi dari **AC-10**.

* **AC-1** (Done when "aset masuk ke bundle macOS" bagian sumber, **E-1**, **E-4**): `cargo run -p desktop-codegen --locked -- assets check` keluar dengan kode 0. Selama ikon masih penanda, baris terakhirnya tepat `assets: ok, 1 assets, 1 source files, 1 placeholder values, 3 outputs up to date`. Pada working tree yang bersih, `cargo run -p desktop-codegen --locked -- assets generate` mencetak baris terakhir yang berakhir `3 outputs, 0 written`, lalu `git diff --exit-code -- apps/macos/Resources shared/assets` keluar dengan kode 0. Sesudah `rm -r apps/macos/Resources/AppIcon.icon`, perintah `generate` yang sama mencetak `wrote apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg` dan `wrote apps/macos/Resources/AppIcon.icon/icon.json`, dan `git status --short` tidak mencetak apa pun. Kedua keluaran sama persis dengan bagian "Isi file": ``awk '/^```json$/{n++;f=(n==1);next} /^```$/{f=0} f' "$SPEC" | diff - apps/macos/Resources/AppIcon.icon/icon.json`` dan ``awk '/^```xml$/{f=1;next} /^```$/{f=0} f' "$SPEC" | diff - apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg`` keluar dengan kode 0 tanpa keluaran.
* **AC-2** (**E-2**, **E-5**, **E-6**, kriteria 32): `git ls-tree -r --name-only "$C" -- apps/macos/Resources` mencetak tepat dua path, `apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg` dan `apps/macos/Resources/AppIcon.icon/icon.json`, dan `git ls-files apps/macos/Resources` mencetak dua path yang sama sampai ada spec lain yang menambah berkas di folder itu. Jumlah path bertambah tepat dua di langkah 1: selisih `git ls-tree -r --name-only "$C" -- apps/macos bindings/swift crates/macos-ffi | wc -l` dan `git ls-tree -r --name-only "$B" -- apps/macos bindings/swift crates/macos-ffi | wc -l` adalah `2`. Tidak ada keluaran biner di repo: `git ls-files | grep -v '^graphify-out/' | grep -ciE '\.(png|icns|ico|car|jpe?g|gif|webp|pdf|tiff?)$'` mencetak `0`, dan `git ls-files | grep -c 'brand-export\.txt$'` mencetak `0`. Pola itu sengaja tidak memuat `svg`, karena salinan SVG adalah keluaran teks menurut **E-4**; jumlah berkas `.svg` di repo menjadi dua, yaitu sumber dan salinannya.
* **AC-3** (**E-3**, **E-4**, **O-1** sampai **O-6**): emitter dikunci uji. `cargo test -p desktop-codegen --locked --test assets_macos_contract` mencetak `test result: ok. 6 passed`. `cargo test -p desktop-codegen --locked --test assets_contract` tetap mencetak `test result: ok. 7 passed`. `cargo test -p desktop-codegen --locked --test assets_rules` tetap lulus tanpa berkas itu diubah, dengan dua keluaran baru ikut diperiksa uji `every_output_follows_the_output_rules`. Sesudah `touch apps/macos/Resources/AppIcon.icon/Assets/old.svg`, `assets check` keluar dengan kode 1 dan stderr memuat baris yang diawali `apps/macos/Resources/AppIcon.icon/Assets/old.svg: I-1:`; hapus berkas itu sesudahnya. `git ls-files --eol apps/macos/Resources | grep -c 'attr/text eol=lf'` mencetak `2`.
* **AC-4** (Done when "aset masuk ke bundle macOS", **E-5**): Xcode memasukkan ikon ke bundel. `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build` mencetak `** BUILD SUCCEEDED **`, begitu juga dengan `-configuration Release`. Untuk kedua konfigurasi, dengan `<K>` diganti `Debug` atau `Release` dan `A=apps/macos/DerivedData/Build/Products/<K>/DesktopMac.app`: `test -s "$A/Contents/Resources/AppIcon.icns" && test -s "$A/Contents/Resources/Assets.car"; echo $?` mencetak `0`; `/usr/libexec/PlistBuddy -c 'Print :CFBundleIconFile' -c 'Print :CFBundleIconName' "$A/Contents/Info.plist"` mencetak dua baris `AppIcon`; `iconutil -c iconset -o "$TMPDIR/AppIcon-<K>.iconset" "$A/Contents/Resources/AppIcon.icns"` keluar dengan kode 0 dan folder hasilnya memuat `icon_16x16.png`; dan `lipo -archs "$A/Contents/MacOS/DesktopMac"` tetap mencetak `arm64`. Sesudah kedua build, `git status --short` tidak mencetak apa pun.
* **AC-5** (**E-3**, kasus gagal): keluaran yang disunting tangan tertangkap dua kali. Sesudah `printf '<svg' > apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg`, perintah `assets check` keluar dengan kode 1, stderr memuat baris yang diawali `apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg: differs from the generator output`, dan build Debug di **AC-4** mencetak `** BUILD FAILED **`. Sesudah `cargo run -p desktop-codegen --locked -- assets generate`, `assets check` kembali keluar dengan kode 0 dan `git status --short` tidak mencetak apa pun.
* **AC-6** (kriteria 32, **E-7**, berkas bersama): `project.pbxproj` hanya mendapat sembilan baris dari bagian "Perubahan `project.pbxproj`". `git diff --numstat "$B" "$C" -- apps/macos/DesktopMac.xcodeproj/project.pbxproj` mencetak baris yang diawali `9` lalu tab lalu `0`. `grep -c 'ASSETCATALOG_COMPILER_APPICON_NAME = AppIcon;' apps/macos/DesktopMac.xcodeproj/project.pbxproj` mencetak `2`, `grep -c 'D00000000000000000000020 /\* Resources \*/' apps/macos/DesktopMac.xcodeproj/project.pbxproj` mencetak `3`, dan `grep -c 'path = Resources;' apps/macos/DesktopMac.xcodeproj/project.pbxproj` mencetak `1`. Langkah 1 tidak menyentuh nama produk atau setelan `Info.plist` lain (**E-7**): `git diff "$B" "$C" -- apps/macos/DesktopMac.xcodeproj/project.pbxproj | grep -c '^+.*INFOPLIST_KEY_'` mencetak `0`. `git diff --quiet "$B" "$C" -- apps/macos/Package.swift tooling/codegen/src/lib.rs; echo $?` mencetak `0`.
* **AC-7** (aturan spec `_root/0001` dan `_root/0003`): workspace tetap hijau dan alat tetap tanpa dependensi baru. `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, dan `cargo clippy --workspace --all-targets --locked -- -D warnings` keluar dengan kode 0. `cargo tree -p desktop-codegen --depth 1 --prefix none --locked` tetap mencetak tepat tiga baris. `git diff --quiet "$B" "$C" -- Cargo.toml Cargo.lock tooling/codegen/Cargo.toml; echo $?` mencetak `0`. Sesudah `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin`, `swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"` keluar dengan kode 0 dan suite yang sudah ada tetap lulus.
* **AC-8** (kriteria 32): setiap commit fitur ini memakai subject yang memuat `spec macos/0007 step <n>`, dan `git log --name-only --format= --grep='spec macos/0007 step' | sed '/^$/d' | sort -u`, dijalankan sesudah commit terakhir tiap milestone, hanya mencetak path dari tabel "File yang boleh diubah tiap milestone".
* **AC-9** (kriteria 21): pemilik setiap berkas ikon di aplikasi tertulis di berkas itu sendiri dan di spec ini. `head -1 apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg | grep -c 'Generated by desktop-codegen from shared/assets/source/app-icon.svg. Do not edit.'` mencetak `1`. `sed -n 2p apps/macos/Resources/AppIcon.icon/icon.json | grep -c 'Generated by desktop-codegen from tooling/codegen/src/assets/macos.rs. Do not edit.'` mencetak `1`. Tabel "Lokasi dan pemilik" spec ini menyebut pemilik setiap path.
* **AC-10** (**E-8**, bukti Kompatibilitas di [rujukan macOS](../../../platforms/macos/README.md)): kombinasi yang dipakai tercatat bersama keluaran perintahnya: `sw_vers -productVersion`, `uname -m`, `xcodebuild -version`, `xcrun --sdk macosx --show-sdk-version`, `rustc --version`, dan `xcrun actool --version`. Pada Mac koordinasi nilainya `27.0.1`, `arm64`, `Xcode 27.1` build `27A9275`, SDK `27.0`, `rustc 1.99.1`, dan `actool` dengan `short-bundle-version` `27.1` serta `bundle-version` `25098`. Bila host lain dipakai, nilainya dicatat apa adanya dan AC lain dinilai pada host itu.
* **AC-11** (Done when "aset masuk ke bundle macOS", bagian yang dilihat mata; langkah manual, `owner: user`): pada sesi GUI, langkah 1 sampai 6 di bagian "Rencana verifikasi manual" memberi hasil yang tertulis. Ikon penanda, yaitu kotak abu abu dengan tanda silang, tampil di Dock saat aplikasi berjalan, di Finder, di jendela Get Info, dan di jendela About, di tampilan terang maupun gelap. Hasil mencatat host, arsitektur, versi macOS, dan tampilan yang aktif.
* **AC-12** (nilai dari user; `BLOCKED`, `owner: user`, sesudah langkah manual `F011-1` spec `_root/0005`): sesudah ikon nyata masuk `shared/assets/source/app-icon.svg` dan `placeholder` dihapus dari `brand.json`, perintah **AC-1** mencetak baris terakhir yang memuat `0 placeholder values` dan berakhir `3 outputs up to date`, perintah **AC-4** tetap lulus, dan langkah manual **AC-11** menunjukkan ikon nyata.

Bagian Done when tentang resource Windows dan direktori ikon Linux bukan AC di sini. Keduanya menunggu host masing masing (lihat Follow-up).

## Decision

**Chosen option**: Opsi 1: dokumen Icon Composer `AppIcon.icon` yang ditulis emitter `desktop-codegen`, didaftarkan lewat grup folder tersinkron `Resources`, dan dikompilasi `actool` milik Xcode saat build.

Ikon macOS adalah keluaran teks dari sumber `assets`. Emitter baru `tooling/codegen/src/assets/macos.rs` menulis `icon.json` dan salinan SVG bertanda ke `apps/macos/Resources/AppIcon.icon/`. Proyek Xcode mengenal folder `Resources` sebagai grup tersinkron dan memakai `AppIcon` sebagai ikon aplikasi, jadi build Xcode yang sudah ada menghasilkan `AppIcon.icns` dan `Assets.car` di bundel tanpa alat atau unduhan tambahan.

Keputusan turunannya:

| Hal | Keputusan |
| --- | --- |
| Format ikon di repo | Dokumen Icon Composer (`.icon`): folder berisi `icon.json` dan `Assets/<berkas>.svg`. Keduanya teks. |
| Penulis | Emitter `macos` di sumber `assets`, sesuai aturan **E-4** dan pola emitter spec `_root/0003`. |
| Lokasi | `apps/macos/Resources/AppIcon.icon/`, sesuai baris macOS di tabel tujuan ekspor spec `_root/0005`. |
| Berkas biner | Tidak ada di repo. `actool` membuatnya di `DerivedData` saat build, jadi **E-5** dipenuhi Xcode dan **E-6** (`brand-export.txt`) tidak dipakai di macOS. |
| Daftar keluaran menurut **E-8** | Di repo: `icon.json` (JSON) dan `Assets/app-icon.svg` (SVG). Di bundel: `Contents/Resources/AppIcon.icns` (ICNS) dan `Contents/Resources/Assets.car` (katalog aset terkompilasi), plus kunci `CFBundleIconFile` dan `CFBundleIconName` di `Info.plist`. Penyimpangan dari **E-8**: spec ini tidak menulis ukuran piksel dan skala, karena `actool` memilihnya dari kanvas vektor; buktinya **AC-4**. Perintah ekspor ulang adalah `assets generate`, alatnya `desktop-codegen` dan `actool` dengan versi dari **AC-10**. |
| Berkas basi | Aturan baru **I-1**: `plan` menolak berkas di `AppIcon.icon` yang tidak lagi dihasilkan. |
| Aset yang diekspor | Hanya `app-icon`. Aset lain di `brand.json` tidak punya tujuan macOS sampai ada spec yang memakainya. |
| Varian | `any` menjadi satu lapisan. `light-dark` menjadi satu lapisan dengan `image-name-specializations` untuk tampilan `dark`. |
| Penanda **O-1** | Di `icon.json` lewat kunci `$comment`. Di salinan SVG lewat komentar XML di baris pertama, atau baris kedua bila sumber diawali deklarasi `<?xml ... ?>`. Teks komentar tidak memuat `--`. |
| Akhir baris | Dua baris `.gitattributes`, satu per jenis berkas, hanya untuk folder `AppIcon.icon`. |
| Proyek Xcode | Grup tersinkron `D00000000000000000000020 /* Resources */` dan `ASSETCATALOG_COMPILER_APPICON_NAME = AppIcon` di dua konfigurasi target. Sembilan baris ditambah, tidak ada yang dihapus. |
| Nama produk | Tidak disentuh. Nama tampil bundel milik spec `macos/0004`. |
| Ikon sistem bawaan | Tidak disentuh (**E-7**). |

Nilai dari spec lain yang dipakai di sini, tanpa diputuskan ulang:

| Nilai | Isi | Pemilik |
| --- | --- | --- |
| Sumber, manifest, aturan A dan S, tujuan ekspor, E-1 sampai E-8 | `shared/assets/`, `brand.json`, `app-icon` dengan `variants` `any` dan `placeholder` `true` | Spec `_root/0005` |
| Bentuk alat, pola `Source`, emitter per OS, aturan O-1 sampai O-7, `.gitattributes` per jenis berkas | `desktop-codegen`, dependensi hanya `serde` dan `serde_json` | Spec [`_root/0003`](../../_root/0003-tema-bersama-dan-kontrol-native/index.md) |
| Nama tampil bundel | `build.product_name` lewat `Shared.xcconfig` | Spec [`macos/0004`](../0004-konfigurasi-publik/index.md) |
| Proyek Xcode, target, versi minimum `14.0`, arsitektur `arm64` | `apps/macos/DesktopMac.xcodeproj` | Spec [`macos/0001`](../0001-sapaan-di-jendela-macos/index.md) |
| ID objek `D00000000000000000000014` | Dipakai spec `macos/0004`, jadi spec ini memakai `D00000000000000000000020` | Spec `macos/0004` |

## Rationale

Alasan, opsi yang ditimbang, daftar keputusan beserta pilihan keduanya, dan bukti percobaan ada di [rationale.md](rationale.md).

## Feature design

### Lokasi dan pemilik

| Path | Keadaan sesudah spec ini | Pemilik | Yang mengisinya |
| --- | --- | --- | --- |
| `apps/macos/Resources/AppIcon.icon/icon.json` | dibuat | Aplikasi macOS | `assets generate`. Jangan disunting. |
| `apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg` | dibuat | Aplikasi macOS. Gambar di dalamnya milik user | `assets generate` dari `shared/assets/source/app-icon.svg`. Jangan disunting. |
| `tooling/codegen/src/assets/macos.rs` | dibuat | Tooling bersama | Spec ini |
| `tooling/codegen/src/assets/mod.rs` | diubah | Tooling bersama | Spec ini |
| `tooling/codegen/tests/assets_macos_contract.rs` | dibuat | Tooling bersama | Spec ini |
| `.gitattributes` | dua baris ditambah | Tooling bersama | Spec ini |
| `apps/macos/DesktopMac.xcodeproj/project.pbxproj` | sembilan baris ditambah | Aplikasi macOS | Bagian "Perubahan `project.pbxproj`" |

`apps/macos/Resources/` adalah grup tersinkron, jadi setiap berkas yang kelak ditaruh di sana ikut masuk bundel tanpa mengubah `project.pbxproj`. Sampai ada spec lain yang memakainya, folder itu hanya memuat `AppIcon.icon/`. Spec `macos/0006` menaruh resource bahasa di `apps/macos/Presentation/Resources/`, bukan di sini.

### Isi file

Kedua berkas berikut adalah hasil `assets generate`. Kamu tidak mengetiknya. Cocokkan hasil emitter dengan blok ini.

`apps/macos/Resources/AppIcon.icon/icon.json`, ditulis dengan `desktop_codegen::json::to_canonical_json`:

```json
{
  "$comment": "Generated by desktop-codegen from tooling/codegen/src/assets/macos.rs. Do not edit. To regenerate, run: cargo run -p desktop-codegen --locked -- assets generate",
  "groups": [
    {
      "layers": [
        {
          "image-name": "app-icon.svg",
          "name": "app-icon"
        }
      ]
    }
  ],
  "supported-platforms": {
    "squares": [
      "macOS"
    ]
  }
}
```

`apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg`, yaitu salinan `shared/assets/source/app-icon.svg` dengan satu baris penanda di depan:

```xml
<!-- Generated by desktop-codegen from shared/assets/source/app-icon.svg. Do not edit. To regenerate, run the assets generate command of desktop-codegen. -->
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024">
  <title>Placeholder app icon, not a brand asset</title>
  <rect x="64" y="64" width="896" height="896" rx="160" fill="#d9d9d9" stroke="#8c8c8c" stroke-width="16"/>
  <path d="M192 192 L832 832 M832 192 L192 832" fill="none" stroke="#8c8c8c" stroke-width="16"/>
</svg>
```

Dua baris ditambahkan di akhir `.gitattributes`, sesudah baris yang sudah ada. Baris lain tidak diubah.

```gitattributes
/apps/macos/Resources/AppIcon.icon/*.json text eol=lf
/apps/macos/Resources/AppIcon.icon/Assets/*.svg text eol=lf
```

### Emitter macOS

Bagian ini memakai bentuk alat dari spec `_root/0003` dan sumber `assets` dari spec `_root/0005`, seperti yang ada di `main` pada 2026-11-15. Kode di sini netral host: tanpa atribut `cfg`, tanpa dependensi baru, dan berjalan di host mana pun. Setiap item publik punya rustdoc berbahasa Inggris.

```rust
// tooling/codegen/src/assets/macos.rs
pub const ICON_DIR: &str = "apps/macos/Resources/AppIcon.icon";
pub const ICON_JSON_PATH: &str = "apps/macos/Resources/AppIcon.icon/icon.json";
pub const ICON_ASSETS_DIR: &str = "apps/macos/Resources/AppIcon.icon/Assets";
pub const ICON_JSON_COMMENT: &str = "Generated by desktop-codegen from tooling/codegen/src/assets/macos.rs. Do not edit. To regenerate, run: cargo run -p desktop-codegen --locked -- assets generate";
pub fn svg_marker(source_path: &str) -> String;
pub fn svg_copy(source_path: &str, contents: &str) -> String;
pub fn icon_json(asset: &Asset) -> String;
pub fn outputs(asset: &Asset, contents: &[String]) -> Vec<Output>;
```

| Fungsi | Perilaku |
| --- | --- |
| `svg_marker` | `source_path` adalah path sumber dari akar repo, misalnya `shared/assets/source/app-icon.svg`. Mengembalikan `<!-- Generated by desktop-codegen from <source_path>. Do not edit. To regenerate, run the assets generate command of desktop-codegen. -->`. Teks ini sengaja tidak memuat perintah `cargo run ... --locked -- ...`, karena XML melarang `--` di dalam komentar dan `actool` menolak berkas seperti itu. |
| `svg_copy` | Fungsi murni, dengan langkah berikut. (1) `body` = `contents` tanpa semua `\n` di akhir. (2) Bila `body` diawali `<?xml` yang diikuti spasi, tab, atau `\n`, dan memuat `?>`: `decl` = teks sampai dan termasuk `?>` pertama, `rest` = sisa sesudahnya; bila `rest` diawali `\n`, buang satu `\n` itu saja (spasi atau tab sebelum `\n` tidak dibuang, jadi `rest` tetap apa adanya). Hasil = `decl` + `\n` + penanda + `\n` + `rest` + `\n`. (3) Selain itu, termasuk `<?xml` tanpa `?>` (tidak terjadi, karena **S-2** menolaknya lebih dulu): hasil = penanda + `\n` + `body` + `\n`. Contoh: `<?xml version="1.0"?>\n<svg/>\n\n` menjadi `<?xml version="1.0"?>\n<penanda>\n<svg/>\n`; `<svg/>` menjadi `<penanda>\n<svg/>\n`. |
| `icon_json` | Fungsi murni. Membangun objek dengan kunci `$comment` berisi `ICON_JSON_COMMENT`, `groups` berisi satu grup dengan satu lapisan, dan `supported-platforms` berisi `{"squares": ["macOS"]}`. Lapisan untuk `Variants::Any` adalah `{"image-name": "<id>.svg", "name": "<id>"}`. Lapisan untuk `Variants::LightDark` adalah `{"image-name": "<id>-light.svg", "image-name-specializations": [{"appearance": "dark", "value": "<id>-dark.svg"}], "name": "<id>"}`. Ditulis dengan `to_canonical_json`. |
| `outputs` | Fungsi murni. `contents` berisi isi setiap path di `asset.source_paths()`, dalam urutan yang sama; panjang yang berbeda adalah kesalahan pemrogram, jadi fungsi ini memanggil `assert_eq!(contents.len(), asset.source_paths().len(), "one content per source path")`. Mengembalikan satu `Output` per sumber, dengan path `ICON_ASSETS_DIR` ditambah `/` dan nama berkas sumber, dan isi `svg_copy(path sumber, isi)`, dalam urutan `source_paths`, lalu satu `Output` untuk `ICON_JSON_PATH` berisi `icon_json(asset)`. |

Perubahan di `tooling/codegen/src/assets/mod.rs`:

* Satu baris `pub mod macos;` di samping `mod schema;` dan `mod svg;`.
* Di `plan`, sesudah semua aturan lulus dan sebelum `Plan` dikembalikan: ambil aset dengan id `REQUIRED_ASSET` dengan `.expect("A-2 guarantees app-icon")`, baca setiap path di `source_paths()` nya sebagai teks UTF-8, lalu tambahkan hasil `macos::outputs` sesudah keluaran skema. Kegagalan baca memberi `Problem` di path sumber itu dengan pesan `S-1: cannot read: <kesalahan OS>`, dan `plan` gagal. Lalu jalankan aturan **I-1** di bawah; pelanggarannya membuat `plan` gagal. Ringkasan `summary` tidak berubah.
* Komentar dokumen modul menyebut bahwa `plan` juga menulis dokumen ikon macOS dari spec `macos/0007`. `macos.rs` dibuka komentar dokumen `//!` yang menyebut spec ini, karena paket memakai `missing_docs = "warn"` dan gerbang memakai `-D warnings`.

Aturan **I-1** (berkas basi di dokumen ikon): bila `<root>/apps/macos/Resources/AppIcon.icon` ada, isinya hanya berkas `icon.json` dan folder `Assets/`, dan `Assets/` hanya memuat berkas yang path nya ada di keluaran `macos::outputs`. Entri bernama `.DS_Store`, `Thumbs.db`, dan `desktop.ini` diabaikan. Setiap entri lain, termasuk subfolder, memberi satu `Problem` di path entri itu dari akar repo dengan pesan `I-1: not produced by the generator, remove it`. Berkas yang hilang tidak dilaporkan aturan ini, karena `check` melaporkannya sebagai `missing` dan `generate` menulisnya. Bila folder `AppIcon.icon` tidak ada, aturan ini tidak memeriksa apa pun, jadi akar sementara di uji yang hanya memuat `shared/assets/` tetap lulus. Kegagalan membaca folder memberi `Problem` di `ICON_DIR` dengan pesan `I-1: cannot read the folder: <kesalahan OS>`.

`tooling/codegen/src/lib.rs` tidak diubah, karena emitter tinggal di dalam modul `assets` dan daftar `SOURCES` tetap.

Urutan `Plan.outputs` sesudah spec ini: `shared/assets/brand.schema.json`, `apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg`, lalu `apps/macos/Resources/AppIcon.icon/icon.json`. Untuk aset `light-dark`, urutannya skema, salinan `-light`, salinan `-dark`, lalu `icon.json`.

### Uji kontrak

`tooling/codegen/tests/assets_macos_contract.rs` hanya memakai path publik `desktop_codegen` dan pustaka standar, dan dibuka dengan komentar dokumen `//!`.

| Uji | Yang dibuktikan |
| --- | --- |
| `committed_plan_lists_the_icon_outputs` | `(assets::SOURCE.plan)(&repo_root())` lulus. Path keluarannya tepat tiga, dalam urutan di atas. Isi `icon.json` sama dengan `icon_json` dari aset `app-icon` yang dibaca `parse`, dan isi salinan SVG sama dengan `svg_copy("shared/assets/source/app-icon.svg", <isi sumber>)`. Isi `icon.json` diawali `{\n  "$comment": "Generated by desktop-codegen from tooling/codegen/src/assets/macos.rs. Do not edit.`. |
| `declaration_stays_on_the_first_line` | `svg_copy("shared/assets/source/app-icon.svg", ...)` atas teks `<?xml version="1.0" encoding="UTF-8"?>\n<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 8 8"/>\n` memberi tepat tiga baris: deklarasi itu, penanda, lalu baris `svg`. Atas teks `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 8 8"/>\n` memberi tepat dua baris: penanda, lalu baris `svg`. |
| `light_dark_pair_uses_the_dark_appearance` | `icon_json` untuk `Asset { id: "app-icon".to_string(), variants: Variants::LightDark, placeholder: false }` sama persis dengan blok JSON kedua di bawah. `outputs` untuk aset itu dengan dua isi memberi path `.../Assets/app-icon-light.svg`, `.../Assets/app-icon-dark.svg`, dan `.../icon.json`, dalam urutan itu. |
| `marker_is_a_valid_xml_comment` | Untuk `source_path` `shared/assets/source/app-icon.svg`, `shared/assets/source/logo-dark.svg`, dan `shared/assets/source/a1-b2.svg`, teks di antara `<!--` dan `-->` pada `svg_marker` tidak memuat `--`, dan penanda memuat `Generated by desktop-codegen` serta `Do not edit`. |
| `trailing_newlines_end_in_one` | `svg_copy` atas isi yang diakhiri `\n\n\n`, dan atas isi tanpa `\n` di akhir, sama sama memberi teks yang diakhiri tepat satu `\n` dan tidak memuat `\r`. |
| `stale_icon_files_are_reported` | Di akar sementara di bawah `env!("CARGO_TARGET_TMPDIR")` bernama `assets_macos_contract_stale`, berisi salinan `shared/assets/` dari repo dan hasil `generate` di akar itu: `plan` lulus. Sesudah berkas `apps/macos/Resources/AppIcon.icon/Assets/old.svg` ditambahkan, `plan` gagal dengan tepat satu masalah, path itu, pesan diawali `I-1:`. Sesudah berkas itu dihapus dan `.DS_Store` ditambahkan di `Assets/`, `plan` lulus. Folder lama dihapus di awal uji lalu dibuat ulang. |

Blok JSON kedua, keluaran `icon_json` untuk aset `light-dark`:

```json
{
  "$comment": "Generated by desktop-codegen from tooling/codegen/src/assets/macos.rs. Do not edit. To regenerate, run: cargo run -p desktop-codegen --locked -- assets generate",
  "groups": [
    {
      "layers": [
        {
          "image-name": "app-icon-light.svg",
          "image-name-specializations": [
            {
              "appearance": "dark",
              "value": "app-icon-dark.svg"
            }
          ],
          "name": "app-icon"
        }
      ]
    }
  ],
  "supported-platforms": {
    "squares": [
      "macOS"
    ]
  }
}
```

Uji yang sudah ada tetap lulus tanpa diubah: `committed_outputs_match_the_source` di `assets_contract.rs` sekarang juga membandingkan dua keluaran macOS dengan berkas di repo, dan `generation_is_deterministic` memeriksa penanda di ketiga keluaran.

### Uji aturan keluaran

`tooling/codegen/tests/assets_rules.rs` dari tahap `Test it` scope 011 bagian `_root` ada di `main` sejak 2026-11-15. Uji `every_output_follows_the_output_rules` di sana memeriksa setiap keluaran sumber `assets`, jadi dua keluaran baru wajib lulus aturan yang sama dengan skema:

| Aturan | Perlakuan untuk keluaran macOS |
| --- | --- |
| **O-1** penanda di tiga baris pertama | Dipenuhi. `icon.json` memuatnya di baris 2 lewat `$comment`, salinan SVG di baris 1 atau 2 lewat komentar XML. |
| **O-3** UTF-8, tanpa BOM, `\n`, satu newline di akhir | Dipenuhi oleh `svg_copy` dan `to_canonical_json`. |
| **O-4** JSON kanonik | Dipenuhi `icon.json`. Salinan SVG bukan JSON. |
| **O-5** hanya bilangan bulat | `icon.json` tidak memuat angka. |
| **O-6** lokasi dan `text eol=lf` per jenis | Path diawali `apps/`. Dua baris `.gitattributes` di atas diawali `/apps/` dan diakhiri `*.json` serta `*.svg`, jadi cocok dengan cara `config_rules.rs` mencari baris itu. |
| Nilai `$comment` yang persis | Uji yang mengunci `$comment` keluaran di `shared/assets/` tidak berlaku untuk `icon.json`. Nilai `$comment` nya dikunci `assets_macos_contract.rs`. |

Uji lokasi di berkas itu sudah menerima path yang diawali `apps/`, dan uji `broken_source_writes_nothing` menuntut bahwa sumber yang rusak tidak menulis apa pun di bawah `apps/`, yang dipenuhi karena `generate` tidak menulis bila `plan` gagal. Jadi spec ini tidak mengubah `assets_rules.rs`. Tidak ada berkas biner yang ditulis alat, jadi tidak ada pengecualian untuk penanda atau `eol=lf`.

### Perubahan `project.pbxproj`

Developer menerapkan tepat perubahan ini pada `apps/macos/DesktopMac.xcodeproj/project.pbxproj`. Indentasi memakai tab. Baris lain tidak diubah. Perubahan ini tidak bergantung pada perubahan spec `macos/0004`, jadi keduanya bisa diterapkan dalam urutan mana pun, selama satu penulis per wave.

Grup tersinkron baru, sesudah grup `App`:

```diff
 		D00000000000000000000002 /* App */ = {
 			isa = PBXFileSystemSynchronizedRootGroup;
 			path = App;
 			sourceTree = "<group>";
 		};
+		D00000000000000000000020 /* Resources */ = {
+			isa = PBXFileSystemSynchronizedRootGroup;
+			path = Resources;
+			sourceTree = "<group>";
+		};
 /* End PBXFileSystemSynchronizedRootGroup section */
```

Anak grup utama `D00000000000000000000004`, tepat sesudah baris `App`:

```diff
 			children = (
 				D00000000000000000000002 /* App */,
+				D00000000000000000000020 /* Resources */,
```

Grup tersinkron target `D00000000000000000000006 /* DesktopMac */`:

```diff
 			fileSystemSynchronizedGroups = (
 				D00000000000000000000002 /* App */,
+				D00000000000000000000020 /* Resources */,
 			);
```

Di kedua konfigurasi target, yaitu objek `D0000000000000000000000E /* Debug */` dan `D00000000000000000000011 /* Release */`, satu baris pertama di `buildSettings`:

```diff
 			buildSettings = {
+				ASSETCATALOG_COMPILER_APPICON_NAME = AppIcon;
 				CODE_SIGN_IDENTITY = "-";
```

Hasilnya `git diff --numstat` untuk berkas itu melaporkan 9 baris ditambah dan 0 dihapus. Konfigurasi tingkat proyek (`D0000000000000000000000C` dan `D0000000000000000000000D`) tidak diubah. Fase `Resources` (`D00000000000000000000009`) tetap kosong, karena grup tersinkron mendaftarkan `AppIcon.icon` ke fase itu saat build.

### Nilai dan sumbernya

| Nilai | Sumber |
| --- | --- |
| Isi lapisan ikon | `shared/assets/source/app-icon.svg`, lewat `svg_copy`. |
| Nama lapisan dan nama berkas di `icon.json` | Id aset dan `variants` dari `brand.json`, lewat `icon_json`. |
| Nama ikon `AppIcon` | Nama folder `AppIcon.icon` dan setelan `ASSETCATALOG_COMPILER_APPICON_NAME`, keputusan spec ini. |
| `CFBundleIconFile` dan `CFBundleIconName` di `Info.plist` | Ditulis `actool` dari nama ikon saat build. |
| `AppIcon.icns` dan `Assets.car` | Dibuat `actool` dari `AppIcon.icon` saat build, di `DerivedData`. |
| Ukuran raster di bundel | Ditetapkan `actool` dari kanvas vektor `0 0 1024 1024`. Spec ini tidak menulis daftar ukuran. |
| Bentuk sudut dan efek sistem | Ditambahkan macOS dan `actool` dari dokumen `.icon`, bukan dari sumber (aturan "Resolusi" spec `_root/0005`). |
| Teks penanda | `ICON_JSON_COMMENT` dan `svg_marker` di kode alat. |
| ID objek `D00000000000000000000020` | ID berikutnya yang tidak dipakai spec `macos/0001`, `macos/0004`, `macos/0005`, atau `macos/0006`. |
| Versi alat yang dicatat | `xcrun actool --version` pada host build (**AC-10**). |

### Invarian

* Satu satunya jalur dari sumber merek ke bundel Mac adalah `assets generate` lalu build Xcode.
* `apps/macos/Resources/AppIcon.icon/` hanya memuat berkas yang ada di `Plan.outputs`. Aturan **I-1** menjaganya.
* Repo tidak memuat berkas ikon biner untuk macOS.
* `desktop-codegen` tetap hanya bergantung pada `serde` dan `serde_json`.
* Proyek Xcode tidak memuat path ke `shared/assets/`. Xcode hanya membaca salinan di folder aplikasi.

### Keamanan

Spec ini tidak menambah rahasia, kredensial, variabel lingkungan, atau unduhan. `actool` dan `iconutil` sudah ada bersama Xcode 27.1 dan macOS. Salinan SVG sudah lolos aturan **S-1** sampai **S-7** spec `_root/0005`, jadi tidak membawa skrip, gaya, atau rujukan luar ke bundel.

### Kasus gagal yang sudah diputuskan

| Kejadian | Perilaku yang diharapkan |
| --- | --- |
| Salinan SVG atau `icon.json` disunting tangan | `assets check` gagal dengan `differs from the generator output` (**AC-5**). Jalankan `assets generate`. |
| Sumber lolos aturan S tetapi bukan XML yang utuh, misalnya komentar yang memuat `--` | `assets check` lulus, tetapi build Xcode gagal dengan pesan `actool` untuk `AppIcon.icon`. Minta pemilik merek memperbaiki sumber. Bentuk merek tidak diperbaiki sendiri. |
| `icon.json` menunjuk berkas yang tidak ada | Build Xcode gagal dengan pesan `references an image named ... that does not exist`. Itu hanya terjadi bila keluaran disunting tangan, dan `assets check` menangkapnya lebih dulu. |
| Aset `app-icon` berubah dari `any` ke `light-dark` | `Assets/app-icon.svg` tidak lagi dihasilkan, jadi **I-1** membuat `check` dan `generate` gagal dengan pesan `I-1: not produced by the generator, remove it` untuk berkas itu. Hapus berkas itu, jalankan `generate`, dan commit semuanya bersama. **AC-1** dan **AC-2** lalu memakai dua path baru. |
| Xcode atau Icon Composer di GUI menulis ke `AppIcon.icon` | `assets check` menangkap selisihnya. Kembalikan dengan `assets generate`. Ikon tidak disunting di Icon Composer, karena sumbernya `shared/assets/`. |
| Build di Xcode lebih lama dari 26, yang belum mengenal `.icon` | Build tidak didukung. Spec `macos/0001` mengunci Xcode 27.1. |
| macOS 14 sampai 25 menjalankan aplikasi | `actool` juga menulis `AppIcon.icns` untuk sistem lama. Belum diuji di host seperti itu (lihat Follow-up). |
| Hook Graphify gagal untuk berkas baru di `apps/macos/Resources/` | Ikuti kasus yang sama di spec `_root/0001`. Pada percobaan untuk spec `_root/0005`, JSON dan SVG diterima tanpa entri `.graphifyignore`. |

### Skenario uji kritis

* Jalur utama: `assets generate` menulis dua keluaran, `assets check` lulus, build Debug dan Release memuat `AppIcon.icns`, `Assets.car`, dan dua kunci ikon di `Info.plist`, memverifikasi **AC-1**, **AC-4**.
* Cakupan: dua path baru, tanpa berkas biner, sembilan baris `project.pbxproj`, memverifikasi **AC-2**, **AC-6**, **AC-8**.
* Emitter: enam uji kontrak, termasuk pasangan terang dan gelap, penanda yang sah sebagai komentar XML, dan berkas basi yang ditolak **I-1**, memverifikasi **AC-3**.
* Kasus gagal: salinan yang dirusak ditolak `check` dan menggagalkan build, lalu dipulihkan `generate`, memverifikasi **AC-5**.
* Gerbang: workspace dan uji Swift tetap hijau, memverifikasi **AC-7**.
* Kepemilikan: penanda di kedua keluaran, memverifikasi **AC-9**.
* Mata: ikon di Dock, Finder, Get Info, dan About, memverifikasi **AC-11**.

### Rencana verifikasi manual

Langkah ini hanya dijalankan user pada sesi GUI. `/develop` menyalinnya ke `verify.md` di bawah `### UI / manual`.

Prasyarat: Mac Apple Silicon dengan macOS 14 atau lebih baru (yang sudah diuji: 27.0.1), Xcode 27.1, sesi desktop yang sedang login, dan repo pada commit yang lulus **AC-4**.

1. Dari akar repo jalankan `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build`. Harapan: `** BUILD SUCCEEDED **`. (**AC-11**)
2. Jalankan `open apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app`. Harapan: ikon aplikasi di Dock adalah kotak abu abu dengan tanda silang, bukan ikon aplikasi kosong bawaan sistem. Sistem boleh menambah bentuk sudut dan efeknya sendiri. (**AC-11**)
3. Di menu aplikasi, pilih item About, yaitu item pertama menu itu (misalnya `About DesktopMac`). Harapan: jendela About menampilkan ikon yang sama. Tutup jendela itu. (**AC-11**)
4. Jalankan `open -R apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app`, pilih `DesktopMac.app` di Finder, lalu tekan Command I. Harapan: Finder dan jendela Get Info menampilkan ikon yang sama. Tutup jendela Get Info. (**AC-11**)
5. Buka System Settings, Appearance, lalu pilih Dark. Harapan: ikon di Dock, Finder, dan About tetap terlihat sebagai kotak dengan tanda silang. Sistem boleh menggelapkan latar ikon. Kembalikan Appearance ke pilihan semula. (**AC-11**)
6. Tekan Command Q, lalu jalankan `git status --short`. Harapan: tidak ada keluaran. Bila ada, catat setiap path yang tercetak dan jangan di commit. (**AC-11**)

Catat host, arsitektur, versi macOS, tampilan yang aktif, dan hasil tiap langkah. Bila Dock masih menampilkan ikon lama dari build sebelumnya, catat hal itu dan ulangi langkah 2 sesudah `killall Dock`.

## Build plan

Pendekatan build proyek adalah Tracer Bullet. Benang tipis fitur ini adalah satu ikon penanda yang berjalan dari `shared/assets/source/` sampai bundel Mac dalam satu commit: emitter, keluaran, dan proyek Xcode masuk bersama. Satu langkah menjadi satu commit, dengan subject yang memuat `spec macos/0007 step <n>`.

File yang boleh diubah tiap milestone:

| Milestone | Path |
| --- | --- |
| 1 | `tooling/codegen/src/assets/macos.rs`, `tooling/codegen/src/assets/mod.rs`, `tooling/codegen/tests/assets_macos_contract.rs`, `.gitattributes` (dua baris), `apps/macos/Resources/AppIcon.icon/icon.json`, `apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg`, `apps/macos/DesktopMac.xcodeproj/project.pbxproj` (sembilan baris), `verify.md` spec ini, baris `**Status**` di `index.md` spec ini |
| 2 | Hanya `verify.md` spec ini |
| 3 | `shared/assets/source/app-icon.svg`, `shared/assets/brand.json`, `apps/macos/Resources/AppIcon.icon/`, `verify.md` spec `_root/0005` dan spec ini |

Empat berkas `graphify-out/` yang ditulis hook commit selalu boleh ikut. Berkas bersama satu penulis per wave yang disentuh: `.gitattributes` dan `project.pbxproj` di langkah 1. `tooling/codegen/src/lib.rs` dan `apps/macos/Package.swift` tidak disentuh.

**Milestone 1: Ikon penanda dari sumber bersama sampai bundel Mac** (langkah 1 dan 2; needs pembaruan spec `macos/0001` lewat `architect (update)`; after build `_root/0006` dan `Test it` scope 011 bagian `_root`; tidak bersamaan dengan build lain yang menyentuh `.gitattributes`, `tooling/codegen/src/assets/`, atau `project.pbxproj`)

1. Dalam satu commit: buat `tooling/codegen/src/assets/macos.rs` menurut "Emitter macOS"; ubah `tooling/codegen/src/assets/mod.rs` seperti tertulis di sana; tulis `tooling/codegen/tests/assets_macos_contract.rs` dengan enam uji di "Uji kontrak"; tambahkan dua baris `.gitattributes`; jalankan `cargo run -p desktop-codegen --locked -- assets generate` untuk menulis dua keluaran; lalu terapkan "Perubahan `project.pbxproj`". Sebelum commit, perintah **AC-1**, **AC-3**, **AC-4**, dan empat perintah gerbang di **AC-7** harus lulus, satisfies **AC-1**, **AC-3**, **AC-4**, **AC-7**, **AC-9**
2. Dengan `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, jalankan ulang setiap perintah **AC-1** sampai **AC-10** pada commit langkah 1, catat perintah dan keluarannya di `verify.md` bagian `## Milestone 1: Ikon penanda dari sumber bersama sampai bundel Mac` bersama host dan versi, hapus `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/`, lalu commit `verify.md`. Path langkah 1 cocok dengan pola **Kapan gerbang wajib** spec `macos/0008`, jadi jalankan juga gerbang Mac `G-1` sampai `G-5` spec itu dan catat hasilnya di bagian yang sama. Jalankan ulang perintah **AC-8** sesudah commit itu. Perintah **AC-2**, **AC-5**, **AC-6**, dan **AC-8** baru bisa dijalankan di langkah ini, karena butuh commit langkah 1, satisfies **AC-1** sampai **AC-10**

**Milestone 2: Ikon terlihat di sesi GUI** (langkah 3, after milestone 1; langkah manual, `owner: user`)

3. Salin "Rencana verifikasi manual" ke `verify.md` bagian `## Milestone 2: Ikon terlihat di sesi GUI` di bawah `### UI / manual`. User menjalankan keenam langkah dan mencatat hasilnya. Sesi koordinasi tidak menjalankan langkah ini, satisfies **AC-11**

**Milestone 3: Ikon nyata dari user** (langkah 4, `BLOCKED`, `owner: user`, sesudah langkah manual `F011-1` spec `_root/0005`)

4. Sesudah user menyerahkan ikon dalam SVG, jalankan langkah 1 sampai 5 di bagian "Mengganti atau menambah aset" `shared/assets/README.md`. Commit itu memuat sumber, `brand.json`, dan dua keluaran di `apps/macos/Resources/AppIcon.icon/` bersama. Lalu jalankan ulang **AC-1**, **AC-4**, dan langkah manual **AC-11**, dan catat hasilnya di `verify.md` bagian `## Milestone 3: Ikon nyata dari user`, satisfies **AC-12**

## Consequences

**Positive**:

* Ikon Mac berasal dari satu sumber lewat satu perintah, dan setiap selisih tertangkap `assets check` serta `cargo test --workspace`.
* Repo tidak memuat berkas biner untuk macOS, jadi tidak ada catatan ekspor yang bisa basi dan tidak ada langkah ekspor yang hanya bisa dijalankan orang.
* Tidak ada dependensi, alat, atau unduhan baru. `actool` sudah ada bersama Xcode.
* Mengganti ikon dengan yang nyata tidak mengubah kode, proyek Xcode, atau nama berkas.
* Pasangan terang dan gelap sudah dipetakan dan diuji, walau belum dipakai.
* Grup tersinkron `Resources` membuat resource macOS berikutnya tidak perlu mengubah `project.pbxproj`.

**Negative / tradeoffs**:

* Format `.icon` butuh Xcode 26 atau lebih baru untuk build. Mesin dengan Xcode lebih lama tidak bisa membangun aplikasi ini.
* Hasil raster di bundel ditentukan `actool`, bukan repo. Versi Xcode yang berbeda bisa memberi raster yang sedikit berbeda, dan itu tidak terlihat di `git diff`.
* `icon.json` mengikuti format Icon Composer yang tidak didokumentasikan sebagai skema publik. Kunci yang dipakai sengaja sedikit, tetapi Xcode berikutnya bisa mengubah formatnya.
* Kesalahan XML di sumber yang lolos aturan teks spec `_root/0005` baru tertangkap saat build Xcode, bukan saat `assets check`.
* Aturan **I-1** membuat `plan` membaca satu folder aplikasi, hal yang belum dilakukan sumber lain. Itu satu satunya cara menangkap berkas basi, karena `check` hanya mengenal keluaran di `Plan`.
* Placeholder punya kotak bersudutnya sendiri, jadi di macOS 26 ke atas terlihat kotak di dalam bentuk ikon sistem. Itu hilang saat ikon nyata yang mengisi kanvas penuh masuk.
* Belum ada bukti di macOS 14 sampai 25 dan di Mac Intel.

**Neutral**:

* Baris sukses `assets check` berubah dari `1 outputs` menjadi `3 outputs`. Spec `_root/0005` menulis angka lama di **AC-5** (lihat Follow-up).
* Nama `AppIcon` adalah nama bawaan Xcode untuk ikon aplikasi.
* `/develop` menulis `verify.md` saat build. Spec ini tidak membuatnya.

## Follow-up

* [x] Jawaban atas Follow-up "Spec macOS untuk scope 011" di spec `_root/0005`: `app-icon` diekspor ke `apps/macos/Resources/AppIcon.icon/` sebagai keluaran teks (**E-4**), ikon terdaftar di proyek Xcode lewat grup tersinkron dan `ASSETCATALOG_COMPILER_APPICON_NAME`, tidak ada keluaran biner sehingga `brand-export.txt` tidak dipakai, dan setiap baris tabel "Kontrak untuk spec OS" dijawab di "Emitter macOS", "Isi file", "Uji aturan keluaran", dan "Rencana verifikasi manual".
* [x] Jawaban atas Follow-up "Scope 011" di spec `macos/0001` dan baris Scope 011 di spec `macos/0004`: ikon aplikasi dijawab di sini, nama tampil bundel sudah dijawab spec `macos/0004`, dan aset merek lain menunggu kebutuhan produk.
* [ ] Lead, lewat `architect (update)` spec `macos/0001`, digabung dengan permintaan spec `macos/0004`, `macos/0005`, dan `macos/0006`. Perubahan persis yang dibutuhkan spec ini:
  1. **AC-12**: jumlah path bertambah tepat dua dari angka yang berlaku sebelum spec ini dibangun. Dua path barunya `apps/macos/Resources/AppIcon.icon/Assets/app-icon.svg` dan `apps/macos/Resources/AppIcon.icon/icon.json`. Kalimat "jadi tidak ada folder `Platform`, `DesignSystem`, atau `Resources`" diganti, untuk bagian spec ini, menjadi "jadi tidak ada folder `Platform`, dan `apps/macos/Resources/` hanya memuat `AppIcon.icon/` dari spec `macos/0007`". Bagian `DesignSystem` mengikuti permintaan spec `macos/0005`.
  2. **Tata letak**: kalimat pembuka yang menyebut jumlah path, dan tabelnya mendapat dua baris itu dengan sumber isi "spec `macos/0007`, keluaran `desktop-codegen`. Jangan disunting." Kalimat "Folder `Platform`, `DesignSystem`, dan `Resources` ... belum dibuat" tidak lagi menyebut `Resources`.
  3. Tabel peran bagian `apps/macos/`: tambah baris `Resources/`, "resource bundel aplikasi, didaftarkan sebagai grup tersinkron; isinya sekarang hanya keluaran generator `AppIcon.icon`".
  4. **Isi file** `apps/macos/DesktopMac.xcodeproj/project.pbxproj`: beri keterangan bahwa blok itu keadaan akhir milestone 2 spec `macos/0001`, dan sembilan baris dari bagian "Perubahan `project.pbxproj`" spec `macos/0007` ditambahkan sesudahnya. Kalimat "yang menyinkronkan folder `App/` otomatis" menjadi "yang menyinkronkan folder `App/` dan `Resources/` otomatis".
  5. **Rencana verifikasi manual** langkah 9: tambahkan "Bila Xcode menulis ke `apps/macos/Resources/AppIcon.icon/`, `assets check` menangkapnya; kembalikan dengan `assets generate`."
  6. **Follow-up** baris Scope 011: tandai `[x] ikon aplikasi diterapkan di spec macos/0007; nama tampil bundel di spec macos/0004`.
  7. **Decision**, baris "Struktur Swift" yang berbunyi "Target aplikasi `DesktopMac` hanya berisi folder `App/`": menjadi "Target aplikasi `DesktopMac` berisi folder `App/` dan grup tersinkron `Resources/` (spec `macos/0007`)". Paragraf pembuka **Decision** yang berbunyi "proyek Xcode hanya berisi titik masuk aplikasi" menjadi "proyek Xcode berisi titik masuk aplikasi dan resource bundel".
  8. Tidak perlu diubah oleh spec ini: AC-1 sampai AC-3, AC-4 (perintah dan keluarannya tetap, bundel hanya bertambah ikon), AC-6 sampai AC-11, AC-13, `apps/macos/Package.swift`, dan `DesktopMacApp.swift`.
  9. Kalimat AC-12 juga diubah spec `macos/0005` (menghapus `DesignSystem`) dan spec `macos/0006`. Lead menggabungkannya. Usulan kalimat gabungan bila ketiganya masuk: "jadi tidak ada folder `Platform`, dan `apps/macos/Resources/` hanya memuat `AppIcon.icon/` dari spec `macos/0007`".
* [ ] Lead, lewat `architect (update)` spec `_root/0005`: **AC-5** menyebut `1 outputs, 0 written` dan `1 outputs, 1 written` tanpa batas waktu. Sesudah langkah 1 spec ini angkanya `3`. Tambahkan "pada akhir milestone 1 spec ini" di sana, atau ganti angkanya. **AC-3** sudah dibatasi "pada akhir milestone 1" dan tetap benar. **AC-2** spec itu mencetak `1` untuk jumlah berkas gambar "pada akhir milestone 1", dan kalimat "jadi tidak ada berkas gambar lain di repo" sesudahnya perlu batas waktu yang sama, karena sesudah spec ini ada dua berkas `.svg`. Kolom "Dirinci di" baris macOS di tabel tujuan ekspor `shared/assets/README.md` boleh menaut ke spec ini; itu mengubah isi yang dikunci **AC-1** spec `_root/0005`, jadi masuk pembaruan yang sama atau dibiarkan.
* [x] Jawaban atas Follow-up "Spec macOS berikutnya yang menambah folder Swift di `apps/macos/`" di spec [`macos/0008`](../0008-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md): `apps/macos/Resources/` tidak memuat berkas Swift, jadi aturan `C-1` dan `C-2` tidak perlu membacanya. Aturan `C-4` tetap berlaku untuk `project.pbxproj` dan dipenuhi, karena sembilan baris spec ini tidak memuat setelan yang dibuat generator. Bila kelak ada Swift di `Resources/`, spec yang menambahkannya memutuskan ulang.
* [ ] Lead: hitungan path relatif di spec lain bergantung pada urutan build. **AC-11** spec `macos/0004` dan **AC-9** spec `macos/0005` membandingkan dengan commit sebelum langkah 1 masing masing, jadi tetap benar, tetapi angka total di **AC-12** spec `macos/0001` hanya bisa ditetapkan sesudah urutan build keempat spec macOS diketahui.
* [ ] Lead: `test-preferences.json` perlu mencatat `tooling/codegen/tests/assets_macos_contract.rs`. Sesudah merge, `docs/platforms/macos/README.md` dan tabel "Yang sudah ada" di `docs/architecture/README.md` perlu menyebut `apps/macos/Resources/`. Pemiliknya lead lewat `/sync`.
* [ ] Scope 012: runner macOS di GitHub Actions butuh Xcode 26 atau lebih baru supaya `.icon` bisa dikompilasi, dan skripnya menjalankan `assets check` sebelum `xcodebuild`.
* [ ] Windows dan Linux (menunggu host): spec Windows dan Linux untuk scope 011 menjawab tabel "Kontrak untuk spec OS" spec `_root/0005` untuk resource WinUI dan direktori ikon `hicolor`. Tidak ada spec, kode, atau bukti untuk keduanya di sini.
* [ ] Uji di macOS 14 sampai 26 dan di Mac Intel belum ada. Bila host seperti itu tersedia, jalankan **AC-4** dan langkah manual di sana.
* [ ] Bila kelak aset merek lain dibutuhkan di dalam jendela (misalnya logo), tambahkan emitter katalog aset lewat pembaruan spec ini. Grup tersinkron `Resources` sudah bisa menampungnya.
