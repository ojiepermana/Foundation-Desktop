# 0004. Konfigurasi publik di macOS

**Scope**: [009 - Konfigurasi publik](../../../scope/009%20-%20Konfigurasi%20publik.md)
**Date**: 2026-11-15
**Status**: Proposed

## Summary

Spec ini menjelaskan cara aplikasi macOS memakai nilai publik bersama dari [spec `_root/0004`](../../_root/0004-konfigurasi-publik/index.md), tanpa memutuskan ulang apa pun di sana. Alat `desktop-codegen` menulis satu berkas xcconfig (berkas teks setelan build Xcode) berisi nama tampil, pengenal bundel, dan versi, lalu proyek Xcode membacanya sebagai dasar setelan target. Tiga nilai itu tidak lagi ditulis tangan di proyek, dan judul jendela diambil dari bundel aplikasi. Pengenal bundel tetap dasar pengenal ditambah `.macos`.

## Requirements

**User stories**:

- Sebagai pemilik produk, kamu ingin mengganti nama produk atau dasar pengenal di `shared/config/public.json` lalu menjalankan satu perintah, supaya aplikasi macOS ikut berubah tanpa kamu membuka proyek Xcode.
- Sebagai developer aplikasi macOS, kamu ingin tahu setelan mana yang datang dari sumber bersama dan setelan mana yang tetap kamu atur di proyek, supaya kamu tidak menyalin nilai bersama dengan tangan.
- Sebagai pemeriksa, kamu ingin membuktikan nilai di bundel aplikasi sama dengan sumbernya tanpa membuka jendela.

**Acceptance criteria**:

Angka kriteria merujuk tabel di [Penerimaan dan bukti fondasi](../../../quality/README.md). Aturan `K-1` sampai `K-7` adalah milik spec `_root/0004`, dan "spec 004" berarti [spec macOS 0001](../0001-sapaan-di-jendela-macos/index.md). Semua perintah dijalankan dari akar repo, pada berkas yang sudah masuk commit.

Setiap catatan bukti menyebut host, arsitektur, dan versi. AC bertanda **Mac** dijalankan di Mac Apple Silicon dan mencatat keluaran `sw_vers -productVersion`, `uname -m`, `xcodebuild -version`, dan `rustc --version` (yang sudah diuji: macOS `27.0.1`, `arm64`, Xcode `27.1`, Rust `1.99.1`). AC bertanda **host mana pun** hanya butuh Rust dan alat teks; catatannya menyebut OS, `uname -m`, dan `rustc --version`. Di sesi koordinasi semuanya dijalankan di Mac yang sama.

Tiga nilai acuan dipakai di beberapa AC dan dihitung dari sumbernya:

```bash
NAME=$(jq -r '.build.product_name' shared/config/public.json)
BASE=$(jq -r '.build.app_identifier' shared/config/public.json)
VERSION=$(sed -n '/^\[workspace\.package\]$/,/^\[/s/^version = "\(.*\)"$/\1/p' Cargo.toml)
```

Dengan nilai sementara, ketiganya adalah `Desktop Foundation`, `com.example.desktop-foundation`, dan `0.1.0`.

- **AC-1** (kriteria 18; Done when "konsumsi", aturan `K-2`; **host mana pun**): salinan macOS dibuat generator dan dijaga `check`. `git ls-files apps/macos/Config` mencetak tepat satu baris, `apps/macos/Config/Shared.xcconfig`. Selama nilai sementara dipakai, `awk '/^```xcconfig$/{f=1;next} /^```$/{f=0} f' docs/specs/macos/0004-konfigurasi-publik/index.md | diff - apps/macos/Config/Shared.xcconfig` tidak mencetak apa pun. `cargo run -p desktop-codegen --locked -- config check` keluar dengan kode 0 dan mencetak baris `config: ok, schema version 1, 2 build values, 2 outputs up to date`. `cargo run -p desktop-codegen --locked -- config generate` lalu `git status --short` tidak mencetak nama berkas apa pun. `grep -c '^/apps/macos/Config/\*\.xcconfig text eol=lf$' .gitattributes` mencetak `1`.
- **AC-2** (Done when "konsumsi", aturan `K-1`; **host mana pun**): tidak ada salinan tulis tangan di `apps/macos/`. `grep -c 'INFOPLIST_KEY_CFBundleDisplayName\|MARKETING_VERSION\|PRODUCT_BUNDLE_IDENTIFIER' apps/macos/DesktopMac.xcodeproj/project.pbxproj` mencetak `0`. `grep -c 'baseConfigurationReference = D00000000000000000000014' apps/macos/DesktopMac.xcodeproj/project.pbxproj` mencetak `2`. `grep -cxF "INFOPLIST_KEY_CFBundleDisplayName = $NAME" apps/macos/Config/Shared.xcconfig` dan `grep -cxF "PRODUCT_BUNDLE_IDENTIFIER = $BASE.macos" apps/macos/Config/Shared.xcconfig` masing masing mencetak `1`. Selama nilai sementara dipakai, `git grep -lF "$NAME" -- apps/macos` dan `git grep -lF "$BASE" -- apps/macos` masing masing mencetak tepat satu baris, `apps/macos/Config/Shared.xcconfig`. Pemeriksaan `git grep` itu bergantung pada nilainya: nama akhir yang juga kata biasa di kode bisa cocok dengan berkas lain, dan saat itu dua hitungan pertama serta AC-6 yang menjadi bukti.
- **AC-3** (Done when "konsumsi", aturan `K-2` dan `K-6`; **Mac**): Xcode memakai nilai dari sumber untuk kedua konfigurasi. Dengan `<K>` diganti `Debug` lalu `Release`, perintah berikut mencetak `5`:

  ```bash
  xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration <K> -showBuildSettings 2>/dev/null \
    | grep -cxF -e "    INFOPLIST_KEY_CFBundleDisplayName = $NAME" -e "    MARKETING_VERSION = $VERSION" \
      -e "    PRODUCT_BUNDLE_IDENTIFIER = $BASE.macos" -e "    PRODUCT_NAME = DesktopMac" -e "    CURRENT_PROJECT_VERSION = 1"
  ```

- **AC-4** (Done when "konsumsi"; kriteria 18; **Mac**, tanpa jendela): bundel hasil build membawa nilai dari sumber. Sesudah kedua perintah `xcodebuild ... build` di **AC-4** spec 004 berhasil, untuk `<K>` `Debug` dan `Release`, dengan `P=apps/macos/DerivedData/Build/Products/<K>/DesktopMac.app/Contents/Info.plist`: `plutil -extract CFBundleDisplayName raw -o - "$P"` mencetak isi `$NAME`, `plutil -extract CFBundleIdentifier raw -o - "$P"` mencetak isi `$BASE` ditambah `.macos`, `plutil -extract CFBundleShortVersionString raw -o - "$P"` mencetak isi `$VERSION`, `plutil -extract CFBundleVersion raw -o - "$P"` mencetak `1`, dan `plutil -extract CFBundleName raw -o - "$P"` mencetak `DesktopMac`.
- **AC-5** (Done when "cara validasi" bagian macOS; **host mana pun**): aturan keluaran macOS diuji. `cargo test -p desktop-codegen --locked --test config_macos_contract` keluar dengan kode 0. Berkas uji itu memuat satu uji untuk setiap baris tabel "Kasus keluaran macOS", dan nama tiap uji diawali nomor kasusnya, sehingga `cargo test -p desktop-codegen --locked --test config_macos_contract -- --list` mencetak sebelas uji bernama `x01_` sampai `x11_`. Tiap uji tolak menyatakan fungsi gagal, salah satu pesannya diawali ID aturan di kolom "Hasil", dan path `Problem` itu sama dengan kolom "Path di `Problem`" tabel aturan. Uji yang sama lulus lewat `cargo test --workspace --locked`.
- **AC-6** (Done when "konsumsi", aturan `K-3`; **host mana pun**): judul jendela tidak lagi literal. `awk '/^```swift$/{f=1;next} /^```$/{f=0} f' docs/specs/macos/0004-konfigurasi-publik/index.md | diff - apps/macos/App/DesktopMacApp.swift` tidak mencetak apa pun. `grep -c 'WindowGroup("' apps/macos/App/DesktopMacApp.swift` mencetak `0`, dan `grep -c 'forInfoDictionaryKey: "CFBundleDisplayName"' apps/macos/App/DesktopMacApp.swift` mencetak `1`.
- **AC-7** (Done when "konsumsi"; langkah manual, `owner: user`; **Mac** dengan sesi GUI): pada sesi GUI, langkah 1 sampai 4 di bagian "Rencana verifikasi manual" memberi hasil yang tertulis. Judul jendela sama dengan `build.product_name`, dan menu aplikasi di menu bar bernama `DesktopMac`.
- **AC-8** (aturan `K-3` dan `K-4`; **host mana pun**): aplikasi dan build Xcode tidak membaca `shared/config/`. `git grep -n 'shared/config\|public\.json' -- '*.swift' apps/macos/DesktopMac.xcodeproj crates/macos-ffi` tidak mencetak apa pun. `grep -c 'shellScript = ' apps/macos/DesktopMac.xcodeproj/project.pbxproj` mencetak `1`, yaitu fase `Build Rust bridge` yang sudah ada.
- **AC-9** (kriteria 18; Done when "batas pengaturan khusus tiap OS" bagian macOS; **host mana pun**): setelan yang dimiliki macOS tetap di proyek, dan berkas hasil generator hanya memuat tiga setelan turunan. `grep -vc '^//' apps/macos/Config/Shared.xcconfig` mencetak `3`. Untuk tiap teks berikut, `grep -cF '<teks>' apps/macos/DesktopMac.xcodeproj/project.pbxproj` mencetak `2`: `MACOSX_DEPLOYMENT_TARGET = 14.0;`, `CURRENT_PROJECT_VERSION = 1;`, `PRODUCT_NAME = "$(TARGET_NAME)";`, `CODE_SIGN_IDENTITY = "-";`, `CODE_SIGN_STYLE = Automatic;`, dan `DEVELOPMENT_TEAM = "";`. Tabel "Batas setelan di macOS" spec ini adalah catatan batas itu.
- **AC-10** (kriteria 5 dan 14 lewat spec 004; **Mac**): alur sapaan tetap utuh. Perintah **AC-2**, **AC-3**, dan **AC-4** spec 004 tetap lulus dengan keluaran yang tertulis di sana. `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, dan `cargo test --workspace --locked` keluar dengan kode 0, termasuk uji `hand_written_swift_uses_no_pointers` dan `committed_outputs_match_the_source`.
- **AC-11** (kriteria 32; **host mana pun**): build spec ini hanya menyentuh `apps/macos/Config/Shared.xcconfig`, `apps/macos/App/DesktopMacApp.swift`, `apps/macos/DesktopMac.xcodeproj/project.pbxproj`, `tooling/codegen/src/config/macos.rs`, `tooling/codegen/src/config/mod.rs`, `tooling/codegen/tests/config_macos_contract.rs`, `.gitattributes` (satu baris), `tooling/codegen/tests/config_contract.rs` (hanya bila sebuah uji di sana mengunci jumlah atau daftar keluaran `plan`, lihat "Kasus gagal yang sudah diputuskan"), `verify.md` spec ini, baris `**Status**` di `index.md` spec ini, dan empat berkas `graphify-out/` dari hook. Daftar itu diperiksa dengan `git diff --name-only main...HEAD` di branch developer sebelum merge. `git ls-files apps/macos bindings/swift crates/macos-ffi | wc -l | tr -d ' '` mencetak `17`. `Cargo.toml`, `Cargo.lock`, `.gitignore`, `shared/config/`, dan `tooling/codegen/Cargo.toml` tidak berubah. Tidak ada berkas timpaan lokal dan tidak ada fase skrip baru.

## Decision

**Chosen option**: Opsi 1: berkas xcconfig hasil generator sebagai dasar setelan target.

Sumber `config` di `desktop-codegen` menulis `apps/macos/Config/Shared.xcconfig`, dua konfigurasi target Xcode membacanya lewat `baseConfigurationReference`, dan judul jendela dibaca dari bundel saat aplikasi berjalan.

Keputusan turunannya:

| Hal | Keputusan |
| --- | --- |
| Cara memenuhi `K-2` | Generator. `check` membandingkan berkas terlacak dengan keluaran generator. |
| Nama tampil bundel | `INFOPLIST_KEY_CFBundleDisplayName` sama dengan `build.product_name`. |
| Pengenal bundel (`K-6`) | `PRODUCT_BUNDLE_IDENTIFIER` sama dengan `build.app_identifier` ditambah `.macos`. Akhiran dipertahankan. |
| Versi | `MARKETING_VERSION` sama dengan versi workspace, yang dibaca generator dari `env!("CARGO_PKG_VERSION")`. |
| Judul jendela | Dibaca dari `CFBundleDisplayName` bundel. Cadangannya nama proses. |
| `PRODUCT_NAME` | Tetap `$(TARGET_NAME)`, yaitu `DesktopMac`. Nama berkas bundel dan nama di menu bar tidak berubah. |
| Nomor build | `CURRENT_PROJECT_VERSION = 1` tetap di proyek. |
| Nama yang tidak bisa dibawa xcconfig | Ditolak generator dengan aturan `M-1` (memuat `//` atau `$`, atau diakhiri `;` atau `\`). |
| Pengaturan lokal | Tidak ada berkas timpaan. Ditambah lewat pembaruan spec ini bila ada kebutuhan nyata. |

## Rationale

Alasan, opsi yang ditimbang, dan daftar keputusan beserta pilihan keduanya ada di [rationale.md](rationale.md).

## Rancangan

### Alur nilai

| Nilai | Sumber (spec `_root/0004`) | Setelan di `Shared.xcconfig` | Hasil di bundel | Dipakai saat |
| --- | --- | --- | --- | --- |
| Nama produk | `build.product_name` di `shared/config/public.json` | `INFOPLIST_KEY_CFBundleDisplayName` | `CFBundleDisplayName`, lalu judul jendela | Build; judul dibaca dari bundel saat berjalan |
| Pengenal bundel | `build.app_identifier` ditambah `.macos` | `PRODUCT_BUNDLE_IDENTIFIER` | `CFBundleIdentifier` | Build |
| Versi | `[workspace.package].version` di `Cargo.toml` root | `MARKETING_VERSION` | `CFBundleShortVersionString` | Build |

Urutannya: kamu menyunting sumber, menjalankan `cargo run -p desktop-codegen --locked -- config generate`, lalu commit sumber dan `Shared.xcconfig` bersama. Xcode membaca `Shared.xcconfig` saat build. Aplikasi yang berjalan hanya membaca Info.plist miliknya sendiri, jadi `K-3` tetap berlaku.

### Batas setelan di macOS

| Setelan | Kelas (spec `_root/0004`) | Tempat | Siapa yang mengubah |
| --- | --- | --- | --- |
| `INFOPLIST_KEY_CFBundleDisplayName`, `PRODUCT_BUNDLE_IDENTIFIER`, `MARKETING_VERSION` | Turunan nilai publik bersama | `apps/macos/Config/Shared.xcconfig` | Generator saja |
| Akhiran `.macos` pada pengenal bundel | Pengaturan khusus macOS (aturan turunan `K-6`) | Konstanta di `tooling/codegen/src/config/macos.rs` | Pembaruan spec ini |
| `PRODUCT_NAME`, nama target, `CURRENT_PROJECT_VERSION`, `MACOSX_DEPLOYMENT_TARGET`, `CODE_SIGN_STYLE`, `CODE_SIGN_IDENTITY`, `DEVELOPMENT_TEAM`, `GENERATE_INFOPLIST_FILE`, dan setelan build lain | Pengaturan khusus macOS | `project.pbxproj` | Spec di `docs/specs/macos/` |
| `xcuserdata/`, `DerivedData/`, tim pengembang pribadi | Pengaturan lokal | Diabaikan git atau belum didukung | Tidak masuk repo |
| Sertifikat dan kredensial penandatanganan | Rahasia | Di luar repo | Dokumentasi distribusi |

Setelan di `project.pbxproj` menimpa setelan dari xcconfig. Karena itu tiga setelan turunan tidak boleh muncul lagi di `project.pbxproj` (AC-2).

### Tata letak

| Path | Keadaan | Sumber isi |
| --- | --- | --- |
| `apps/macos/Config/Shared.xcconfig` | Baru | Generator. Jangan disunting. |
| `apps/macos/App/DesktopMacApp.swift` | Diubah | Bagian "Isi file" |
| `apps/macos/DesktopMac.xcodeproj/project.pbxproj` | Diubah di sepuluh baris | Bagian "Perubahan `project.pbxproj`" |
| `tooling/codegen/src/config/macos.rs` | Baru | Bagian "Keluaran macOS di `desktop-codegen`" |
| `tooling/codegen/src/config/mod.rs` | Diubah | Mendaftarkan modul `macos` dan menambah keluarannya ke `plan` |
| `tooling/codegen/tests/config_macos_contract.rs` | Baru | Tabel "Kasus keluaran macOS" |
| `.gitattributes` | Satu baris ditambah | `/apps/macos/Config/*.xcconfig text eol=lf` |

`apps/macos/Config/` bukan bagian paket Swift dan bukan folder yang disinkronkan Xcode. Isinya hanya berkas setelan.

### Isi file

`apps/macos/Config/Shared.xcconfig` tidak ditulis tangan. Dengan nilai sementara, keluaran generator harus sama persis dengan blok ini, diakhiri satu baris baru:

```xcconfig
// Generated by desktop-codegen from shared/config/public.json and the workspace version. Do not edit.
// Run: cargo run -p desktop-codegen --locked -- config generate
INFOPLIST_KEY_CFBundleDisplayName = Desktop Foundation
MARKETING_VERSION = 0.1.0
PRODUCT_BUNDLE_IDENTIFIER = com.example.desktop-foundation.macos
```

Aturan bentuknya: dua baris komentar tetap, lalu tiga baris `KUNCI = nilai` dengan kunci terurut menurut abjad, satu spasi di tiap sisi tanda `=`, nilai tanpa tanda kutip, akhir baris `\n`, dan satu baris baru di akhir berkas.

`apps/macos/App/DesktopMacApp.swift` sesudah perubahan:

```swift
import Presentation
import SwiftUI

@main
struct DesktopMacApp: App {
    /// The product name Xcode writes into the bundle at build time. Its
    /// source is `build.product_name` in the shared public configuration.
    private let productName =
        Bundle.main.object(forInfoDictionaryKey: "CFBundleDisplayName") as? String
        ?? ProcessInfo.processInfo.processName

    var body: some Scene {
        WindowGroup(productName) {
            GreetingView(model: GreetingViewModel())
        }
        .windowResizability(.contentSize)
    }
}
```

`WindowGroup` dengan argumen `String` menampilkan teks itu apa adanya, tidak sebagai kunci lokalisasi.

### Perubahan `project.pbxproj`

Developer menerapkan tepat perubahan ini pada `apps/macos/DesktopMac.xcodeproj/project.pbxproj`. Indentasi memakai tab. Baris lain tidak diubah.

```diff
 /* Begin PBXFileReference section */
 		D00000000000000000000001 /* DesktopMac.app */ = {isa = PBXFileReference; explicitFileType = wrapper.application; includeInIndex = 0; path = DesktopMac.app; sourceTree = BUILT_PRODUCTS_DIR; };
+		D00000000000000000000014 /* Shared.xcconfig */ = {isa = PBXFileReference; lastKnownFileType = text.xcconfig; name = Shared.xcconfig; path = Config/Shared.xcconfig; sourceTree = "<group>"; };
 /* End PBXFileReference section */
```

```diff
 			children = (
 				D00000000000000000000002 /* App */,
+				D00000000000000000000014 /* Shared.xcconfig */,
 				D00000000000000000000005 /* Products */,
 			);
```

Di kedua konfigurasi target, yaitu objek `D0000000000000000000000E /* Debug */` dan `D00000000000000000000011 /* Release */`, tambahkan satu baris dan hapus tiga baris:

```diff
 			isa = XCBuildConfiguration;
+			baseConfigurationReference = D00000000000000000000014 /* Shared.xcconfig */;
 			buildSettings = {
 				CODE_SIGN_IDENTITY = "-";
 				CODE_SIGN_STYLE = Automatic;
 				COMBINE_HIDPI_IMAGES = YES;
 				CURRENT_PROJECT_VERSION = 1;
 				DEVELOPMENT_TEAM = "";
 				ENABLE_PREVIEWS = YES;
 				GENERATE_INFOPLIST_FILE = YES;
-				INFOPLIST_KEY_CFBundleDisplayName = "Desktop Foundation";
 				LD_RUNPATH_SEARCH_PATHS = "$(inherited) @executable_path/../Frameworks";
-				MARKETING_VERSION = 0.1.0;
-				PRODUCT_BUNDLE_IDENTIFIER = "com.example.desktop-foundation.macos";
 				PRODUCT_NAME = "$(TARGET_NAME)";
 				SWIFT_EMIT_LOC_STRINGS = NO;
 			};
```

Hasilnya `git diff --stat` untuk berkas itu melaporkan 4 baris ditambah dan 6 baris dihapus. ID objek `D00000000000000000000014` adalah ID berikutnya yang belum dipakai di berkas itu. Konfigurasi tingkat proyek (`D0000000000000000000000C` dan `D0000000000000000000000D`) tidak diubah.

### Keluaran macOS di `desktop-codegen`

Bagian ini memakai bentuk alat dari spec `_root/0003` dan sumber `config` dari spec `_root/0004`. Kode di sini netral host: tanpa atribut `cfg`, tanpa dependensi baru, dan berjalan di host mana pun.

| Hal | Nilai |
| --- | --- |
| Modul baru | `tooling/codegen/src/config/macos.rs`, didaftarkan dengan `pub mod macos;` di `tooling/codegen/src/config/mod.rs`. |
| Fungsi publik | `pub fn shared_xcconfig(product_name: &str, app_identifier: &str, version: &str) -> Result<Output, Vec<Problem>>`. Fungsi murni: tidak membaca berkas dan tidak membaca lingkungan. |
| Path keluaran | `apps/macos/Config/Shared.xcconfig`. |
| Isi keluaran | Dua baris komentar tetap dari "Isi file", lalu `INFOPLIST_KEY_CFBundleDisplayName = <product_name>`, `MARKETING_VERSION = <version>`, dan `PRODUCT_BUNDLE_IDENTIFIER = <app_identifier>.macos`. |
| Akhiran pengenal | Konstanta `.macos` di modul itu. |
| Pemanggil | `plan` sumber `config`, sesudah `R-1` sampai `R-3` lulus, memanggil `shared_xcconfig` dengan dua nilai dari `public.json` dan `env!("CARGO_PKG_VERSION")`. Keluarannya ditambahkan sesudah `shared/config/public.schema.json`, jadi `plan` mengembalikan dua `Output`. Bila fungsi itu mengembalikan `Problem`, `plan` gagal dengan `Problem` itu. |
| Ringkasan `check` | Tidak berubah: `schema version 1, 2 build values`. Jumlah keluaran menjadi 2. |
| Uji | `tooling/codegen/tests/config_macos_contract.rs`, satu uji per baris X-1 sampai X-11, hanya memakai API publik paket dan pustaka standar. Uji `committed_outputs_match_the_source` yang sudah ada di `config_contract.rs` ikut membandingkan `Shared.xcconfig`. |
| Akhir baris | Satu baris `/apps/macos/Config/*.xcconfig text eol=lf` ditambahkan ke `.gitattributes` root, sesudah baris yang sudah ada. |

`desktop-codegen` mewarisi `version` dari `[workspace.package]` (spec `_root/0001`, dijaga uji `every_member_inherits_shared_fields_and_uses_desktop_prefix`), jadi `env!("CARGO_PKG_VERSION")` di paket itu selalu sama dengan versi workspace. Saat versi workspace naik, Cargo membangun ulang paket itu, `check` melihat `Shared.xcconfig` yang usang, dan gerbang gagal sampai `generate` dijalankan.

Aturan keluaran macOS:

| ID | Aturan | Path di `Problem` |
| --- | --- | --- |
| `M-1` | `product_name` tidak memuat `//`, tidak memuat `$`, tidak diakhiri `;`, dan tidak diakhiri `\`. Di xcconfig, `//` memulai komentar, `$(...)` dibaca sebagai rujukan setelan, satu `;` di akhir nilai dibuang, dan `\` di akhir baris menyambung baris berikutnya. Nama seperti itu tidak sampai apa adanya. | `shared/config/public.json` |
| `M-2` | `version` berbentuk tepat tiga bilangan desimal dipisah titik, misalnya `0.1.0`. Akhiran pra rilis atau metadata build ditolak. Ini pilihan spec ini: versi Cargo selalu punya tiga bagian itu, dan `CFBundleShortVersionString` hanya memuat bilangan dan titik. | `Cargo.toml` |

Pesan tiap `Problem` diawali ID aturannya, sama seperti aturan `R` di spec `_root/0004`. Fungsi melaporkan setiap aturan yang gagal, bukan hanya yang pertama. Karakter lain yang sah menurut skema versi 1 dibawa apa adanya; spasi di awal dan akhir sudah ditolak `R-2`. `app_identifier` hanya memuat huruf ASCII, angka, tanda `-`, dan titik, jadi selalu aman di xcconfig.

Kasus keluaran macOS (semua memanggil `shared_xcconfig` langsung). X-8 tidak bisa datang dari Cargo, yang selalu menuntut tiga bagian, tetapi fungsinya murni dan tetap diuji:

| No. | `product_name` | `app_identifier` | `version` | Hasil |
| --- | --- | --- | --- | --- |
| X-1 | `Desktop Foundation` | `com.example.desktop-foundation` | `0.1.0` | Lulus. Path `apps/macos/Config/Shared.xcconfig`, isi sama persis dengan blok di "Isi file". |
| X-2 | `Ünï 名前` | `Com.Example.App` | `12.34.56` | Lulus. Tiga baris setelan memuat `Ünï 名前`, `12.34.56`, dan `Com.Example.App.macos`. |
| X-3 | `A/B; "Q" = #1 \ ok` | `com.example.app` | `1.0.0` | Lulus. Baris nama tampil adalah `INFOPLIST_KEY_CFBundleDisplayName = A/B; "Q" = #1 \ ok`. |
| X-4 | `A  B` (dua spasi) | `com.example.app` | `1.0.0` | Lulus. Dua spasi di tengah tetap ada. |
| X-5 | `A // B` | `com.example.app` | `1.0.0` | Ditolak dengan `M-1`. |
| X-6 | `Cost $5` | `com.example.app` | `1.0.0` | Ditolak dengan `M-1`. |
| X-7 | `Desktop Foundation` | `com.example.app` | `0.1.0-beta.1` | Ditolak dengan `M-2`. |
| X-8 | `Desktop Foundation` | `com.example.app` | `1.0` | Ditolak dengan `M-2`. |
| X-9 | `Desktop Foundation` | `com.example.app` | `1.0.0+build.5` | Ditolak dengan `M-2`. |
| X-10 | `Name;` | `com.example.app` | `1.0.0` | Ditolak dengan `M-1`. |
| X-11 | `Name\` | `com.example.app` | `1.0.0` | Ditolak dengan `M-1`. |

### Nilai dan sumbernya

| Aksi | Nilai | Sumber |
| --- | --- | --- |
| `generate` | `INFOPLIST_KEY_CFBundleDisplayName` | `build.product_name` di `shared/config/public.json`, sesudah lulus `R-2` dan `M-1`. |
| `generate` | `PRODUCT_BUNDLE_IDENTIFIER` | `build.app_identifier` ditambah konstanta `.macos` (`K-6`). |
| `generate` | `MARKETING_VERSION` | `env!("CARGO_PKG_VERSION")` di `desktop-codegen`, sama dengan `[workspace.package].version`, sesudah lulus `M-2`. |
| `generate` | Dua baris komentar | Teks tetap dari "Isi file". |
| Build Xcode | Tiga setelan di atas | `apps/macos/Config/Shared.xcconfig` lewat `baseConfigurationReference`. |
| Build Xcode | `CFBundleName`, nama berkas bundel | `PRODUCT_NAME`, yaitu nama target `DesktopMac`, di `project.pbxproj`. |
| Build Xcode | `CFBundleVersion` | `CURRENT_PROJECT_VERSION = 1` di `project.pbxproj` (spec 004). |
| Jendela terbuka | Judul jendela | `CFBundleDisplayName` dari Info.plist bundel. Bila kunci itu tidak ada, `ProcessInfo.processInfo.processName`. |
| `check` gagal | ID aturan dan path di pesan | Tabel aturan `M-1` dan `M-2`, atau aturan `R` spec `_root/0004`. |
| AC-3 dan AC-4 | `NAME`, `BASE`, `VERSION` | Tiga perintah di awal bagian "Acceptance criteria". |

Nilai dari user tidak bertambah. `NAMA_PRODUK` dan `DASAR_PENGENAL` dari spec `_root/0004` (`N5` dan `N6` di file langkah manual) cukup; pengenal bundel akhir macOS (`N4`) adalah `DASAR_PENGENAL` ditambah `.macos`.

### Invarian

- `apps/macos/Config/Shared.xcconfig` selalu sama dengan keluaran generator untuk commit itu dan tidak pernah disunting tangan.
- `project.pbxproj` tidak pernah memuat `INFOPLIST_KEY_CFBundleDisplayName`, `MARKETING_VERSION`, atau `PRODUCT_BUNDLE_IDENTIFIER`.
- Nama produk dan dasar pengenal tidak pernah muncul sebagai literal di berkas Swift atau `project.pbxproj`.
- Aplikasi dan fase build Xcode tidak membaca `shared/config/`.
- `tooling/codegen/src/config/macos.rs` tidak memakai atribut `cfg` dan tidak menambah dependensi.
- Hanya `apps/macos/Interop/Greeter.swift` yang mengimpor `DesktopMacosFFI`, dan jembatan tetap tidak tahu soal konfigurasi (`K-4`).

### Keamanan

Tiga nilai itu publik dan memang tampil kepada pengguna. Spec ini tidak menambah rahasia, variabel lingkungan, kredensial, atau fase skrip. Penandatanganan tetap ad hoc seperti di spec 004, dan `DEVELOPMENT_TEAM` tetap kosong. Fungsi `shared_xcconfig` menolak nama yang akan dibaca Xcode sebagai rujukan setelan (`$`), sehingga isi `public.json` tidak bisa menyisipkan nilai setelan lain ke dalam build.

### Kasus gagal yang sudah diputuskan

| Kejadian | Perilaku yang diharapkan |
| --- | --- |
| User memberi nama produk atau dasar pengenal akhir | Sunting `public.json`, jalankan `config generate`, commit sumber dan `Shared.xcconfig` bersama. `project.pbxproj` tidak disentuh. |
| Versi workspace naik | Ubah `Cargo.toml`, jalankan `cargo build` sekali tanpa `--locked` supaya `Cargo.lock` ikut (aturan spec `_root/0001`), jalankan `config generate`, lalu commit `Cargo.toml`, `Cargo.lock`, dan `Shared.xcconfig` bersama. Tanpa `generate`, `cargo test --workspace --locked` gagal di `committed_outputs_match_the_source`. |
| Nama produk memuat `//` atau `$`, atau diakhiri `;` atau `\` | `config check` gagal dengan `M-1`. Ganti namanya, atau perbarui spec ini supaya nama tampil dibawa lewat `Info.plist` hasil generator. |
| Versi workspace memakai akhiran pra rilis | `config check` gagal dengan `M-2`. Putuskan aturan pemetaannya lewat pembaruan spec ini. |
| `Shared.xcconfig` disunting tangan | `check` gagal dengan keluaran yang menyimpang. Jalankan `generate`. |
| `Shared.xcconfig` tidak ada | `xcodebuild` gagal membaca dasar setelan, dan `check` melaporkan keluaran yang hilang. Jalankan `generate`. |
| `generate` gagal karena folder `apps/macos/Config/` belum ada | Langkah 1 membuat folder itu lebih dulu. Apakah alat membuat folder induk sendiri adalah urusan spec `_root/0003`. |
| Uji di `tooling/codegen/tests/config_contract.rs` mengunci jumlah atau daftar keluaran `plan` | `plan` sekarang mengembalikan dua keluaran. Langkah 1 menyesuaikan uji itu supaya tidak menghitung keluaran, tanpa mengubah kasus T-1 sampai T-32. |
| Xcode menulis salah satu dari tiga setelan ke `project.pbxproj` (misalnya lewat editor setelan) | Nilai di proyek menimpa xcconfig. AC-2 menangkapnya. Hapus baris itu dari proyek. |
| Pengembang butuh pengenal bundel lain untuk tim pribadi | Belum didukung. Pembaruan spec ini menambah berkas native yang diabaikan git, dengan perubahan `.gitignore` lewat spec `_root/0001`. |
| Aplikasi dijalankan tanpa `CFBundleDisplayName` di bundelnya | Judul jendela memakai nama proses. Ini tidak terjadi pada build dari proyek ini, karena AC-4 membuktikan kunci itu ada. |
| Hook Graphify gagal pada `Shared.xcconfig` dengan `Graphify extraction failed` | Ikuti aturan `.graphifyignore` di spec `_root/0001` dan catat entrinya di spec ini lewat pembaruan. |
| Bentuk alat di spec `_root/0003` atau sumber `config` di spec `_root/0004` berubah sebelum build | Bagian "Keluaran macOS di `desktop-codegen`" diperbarui lebih dulu. |

### Skenario uji kritis

- Jalur utama: `config check` lulus dengan dua keluaran, lalu `xcodebuild -showBuildSettings` mencetak tiga nilai dari sumber untuk Debug dan Release, memverifikasi **AC-1**, **AC-3**.
- Bundel: Info.plist hasil build memuat nama tampil, pengenal, dan versi dari sumber, memverifikasi **AC-4**.
- Satu sumber: `git grep` hanya menemukan nama produk dan dasar pengenal di `Shared.xcconfig`, memverifikasi **AC-2**, **AC-8**.
- Aturan keluaran: nama dengan `//` atau `$`, nama yang diakhiri `;` atau `\`, dan versi bukan tiga bilangan ditolak, memverifikasi **AC-5**.
- Keluaran usang: mengubah `Shared.xcconfig` dengan tangan membuat `check` gagal, memverifikasi **AC-1**.
- Judul jendela: tidak ada literal di `DesktopMacApp.swift`, dan jendela nyata menampilkan nama produk, memverifikasi **AC-6**, **AC-7**.
- Regresi: `swift test`, kedua build `xcodebuild`, dan gerbang Cargo tetap lulus, memverifikasi **AC-10**.

### Rencana verifikasi manual

Langkah ini hanya dijalankan user pada sesi GUI. `/develop` menyalinnya ke `verify.md` di bawah `### UI / manual` pada milestone 2.

Prasyarat: Mac Apple Silicon dengan macOS 14 atau lebih baru (yang sudah diuji: 27.0.1), Xcode 27.1, sesi desktop yang sedang login, dan repo pada commit yang lulus **AC-4**.

1. Dari akar repo jalankan `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build`. Harapan: `** BUILD SUCCEEDED **`. (**AC-7**)
2. Jalankan `jq -r '.build.product_name' shared/config/public.json` dan catat keluarannya. Lalu jalankan `open apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app`. Harapan: satu jendela terbuka, dan judulnya sama persis dengan keluaran `jq` tadi (dengan nilai sementara: `Desktop Foundation`). (**AC-7**)
3. Lihat menu bar selagi aplikasi di depan. Harapan: menu aplikasi di sebelah menu Apple bernama `DesktopMac`. Catat juga nama yang muncul saat kursor berada di atas ikon aplikasi di Dock; nama itu dicatat saja dan tidak menentukan lulus atau gagal. (**AC-7**)
4. Tekan Command Q. Harapan: aplikasi tertutup tanpa dialog. (**AC-7**)

Catat host, arsitektur, versi macOS, versi Xcode, dan hasil tiap langkah. Langkah yang belum dijalankan tetap tercatat sebagai belum diverifikasi.

## Build plan

Pendekatan build proyek adalah Tracer Bullet. Untuk fitur ini benang tipisnya berjalan dari `public.json` lewat generator dan xcconfig sampai Info.plist bundel, dan terbukti tanpa jendela. Judul jendela menyusul, lalu sesi GUI. Satu langkah menjadi satu commit, dengan subject yang memuat `spec macos/0004 step <n>`.

Prasyarat: needs scope 009 milestone 1 (spec `_root/0004`: sumber `config`, `shared/config/public.json`, dan uji `config_contract`) di `main`, yang sendiri needs scope 008 milestone 1 (spec `_root/0003`: alat `desktop-codegen`) di `main`. Needs pembaruan spec `macos/0001` dari daftar di Follow-up diterima lebih dulu, karena spec itu mengunci isi dua berkas yang diubah di sini dan jumlah path di AC-12.

**Milestone 1: Setelan Xcode dari sumber bersama** (langkah 1 sampai 3)

1. Dalam satu commit: buat `tooling/codegen/src/config/macos.rs` dengan `shared_xcconfig` dan aturan `M-1` serta `M-2`; daftarkan modul itu dan tambahkan keluarannya ke `plan` di `tooling/codegen/src/config/mod.rs`; tulis `tooling/codegen/tests/config_macos_contract.rs` dengan uji X-1 sampai X-11; tambahkan baris `.gitattributes`; buat folder `apps/macos/Config/`, lalu jalankan `cargo run -p desktop-codegen --locked -- config generate` untuk menulis `apps/macos/Config/Shared.xcconfig`; dan terapkan bagian "Perubahan `project.pbxproj`". Sebelum commit, perintah AC-3, AC-5, dan gerbang Cargo di AC-10 harus lulus, satisfies **AC-1**, **AC-3**, **AC-5**, **AC-8**, **AC-9**, **AC-11**
2. Ganti isi `apps/macos/App/DesktopMacApp.swift` dengan blok di "Isi file", lalu jalankan kedua build `xcodebuild` dan `swift test` seperti di AC-10. Baru sesudah langkah ini literal nama produk hilang dari `apps/macos/`, satisfies **AC-2**, **AC-4**, **AC-6**, **AC-10**
3. Dengan `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, pada working tree yang bersih, jalankan ulang setiap perintah AC-1 sampai AC-6 dan AC-8 sampai AC-11, catat perintah dan keluarannya di `verify.md` bagian `## Milestone 1: Setelan Xcode dari sumber bersama` bersama host, arsitektur, dan versi, lalu hapus `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/`, satisfies **AC-1**, **AC-2**, **AC-3**, **AC-4**, **AC-5**, **AC-6**, **AC-8**, **AC-9**, **AC-10**, **AC-11**

**Milestone 2: Sesi GUI di Mac** (langkah 4, after milestone 1; `BLOCKED`, `owner: user`)

4. User menjalankan empat langkah di "Rencana verifikasi manual" pada sesi GUI dan mencatat hasilnya di `verify.md` bagian `## Milestone 2: Sesi GUI di Mac`, di bawah `### UI / manual`. Sesi koordinasi tidak menjalankan langkah ini, satisfies **AC-7**

## Consequences

**Positive**:

- Nama produk, pengenal bundel, dan versi tidak lagi ditulis tangan di `apps/macos/`. Tujuh baris salinan hilang.
- Versi workspace yang naik tanpa `generate` menggagalkan `cargo test --workspace --locked`, jadi `MARKETING_VERSION` tidak bisa tertinggal.
- Pemeriksaan berjalan di host mana pun tanpa Xcode, karena `check` hanya membandingkan teks.
- Mengganti nilai tidak menyentuh `project.pbxproj`.

**Negative / tradeoffs**:

- Satu berkas hasil generator dan satu folder baru di `apps/macos/`, dan satu modul macOS di alat bersama. Developer macOS harus tahu bahwa setelan itu tinggal di luar proyek.
- Nama produk yang memuat `//` atau `$`, atau diakhiri `;` atau `\`, ditolak, walau sah menurut skema versi 1. Itu batas xcconfig, bukan batas produk.
- Menu bar tetap menampilkan `DesktopMac`, bukan nama produk. Mengubahnya berarti mengganti `PRODUCT_NAME` dan nama berkas bundel.
- Setelan yang ditambahkan lewat editor Xcode ke `project.pbxproj` diam diam menimpa xcconfig. Hanya AC-2 yang menangkapnya, dan belum ada uji tetap untuk itu.
- Judul jendela baru terbukti penuh di sesi GUI. Tanpa jendela, yang terbukti hanya isi bundel dan kode yang membacanya.
- Spec `macos/0001` harus diperbarui lebih dulu, karena ia mengunci isi `project.pbxproj` dan `DesktopMacApp.swift`.

**Neutral**:

- `tooling/codegen/src/config/mod.rs` dimiliki build spec `_root/0004`. Langkah 1 mengubahnya, jadi build ini berjalan sesudah milestone 1 scope 009 di `main`, bukan paralel.
- `.gitattributes` adalah berkas root yang dibuat spec `_root/0003`. Spec ini hanya menambah satu baris.
- `/develop` menulis `verify.md` saat build. Spec ini tidak membuatnya.
- Uji di `crates/core/tests/` tidak perlu diubah. `macos_bridge_ownership.rs` membaca berkas Swift di `apps/macos/` dan tetap lulus, karena `DesktopMacApp.swift` yang baru tidak memakai pointer. `workspace_layout.rs` tidak membaca berkas yang diubah di sini.

## Follow-up

Pembaruan yang diminta pada spec `macos/0001` sebelum build (lewat `architect (update)`, bukan disunting dari sini):

- [ ] AC-12: "tepat 16 path" menjadi 17, dan tabel "Tata letak" mendapat baris `apps/macos/Config/Shared.xcconfig` dengan sumber "Generator `desktop-codegen` (spec `macos/0004`). Jangan disunting.".
- [ ] "Isi file" untuk `apps/macos/App/DesktopMacApp.swift`: diganti blok di spec ini, atau menunjuk ke spec ini.
- [ ] "Isi file" untuk `project.pbxproj`: sepuluh baris berubah seperti di "Perubahan `project.pbxproj`" spec ini (satu `PBXFileReference`, satu anak grup, dan di tiap konfigurasi target satu `baseConfigurationReference` ditambah serta tiga setelan dihapus).
- [ ] Catatan di bawah `project.pbxproj` yang berbunyi `MARKETING_VERSION = 0.1.0` disalin dan dinaikkan dengan tangan: diganti dengan rujukan ke `Shared.xcconfig`.
- [ ] Tabel "Value sourcing": baris judul jendela, nama tampil bundel, pengenal bundel, dan versi bundel menunjuk ke spec ini.
- [ ] "Configuration required": menyebut `apps/macos/Config/Shared.xcconfig` sebagai berkas setelan hasil generator.
- [ ] AC-5 dan langkah manual 2: judul jendela adalah nilai `build.product_name`, dengan nilai sementara `Desktop Foundation`.
- [ ] Tabel keputusan, baris "Teks antarmuka": `Desktop Foundation` dikeluarkan dari daftar literal. Judul jendela datang dari bundel menurut spec ini; `Name` dan `Greet` mengikuti spec `_root/0006`.
- [ ] Follow-up "Nilai dari user: pengenal bundel akhir": menunjuk ke `DASAR_PENGENAL` (`N6`) ditambah `.macos`.
- [ ] Follow-up Scope 009: ditandai `[x]`, diterapkan di spec `_root/0004` dan spec ini.
- [ ] Follow-up Scope 010: `Desktop Foundation` dikeluarkan dari daftar teks yang pindah ke katalog bahasa. Nasib `Name` dan `Greet` diputuskan spec `_root/0006`.
- [ ] Follow-up Scope 011: "nama tampil bundel" dikeluarkan. Yang tersisa ikon aplikasi dan aset merek lain.
- [ ] Follow-up Scope 012: bagian "jaga `MARKETING_VERSION` tetap sama dengan versi workspace" ditandai terjawab di sini. Yang tersisa untuk scope 012 adalah membungkus perintah dalam skrip dan CI.

Pembaruan yang diminta pada spec `_root/0004`:

- [ ] Baris `.gitattributes` di AC-1 dan "Bentuk alat" ditulis `/shared/config/** text eol=lf`. Architect scope 008 mengubah aturannya menjadi per jenis berkas, jadi barisnya menjadi `/shared/config/**/*.json text eol=lf`.
- [ ] Baris "Keluaran `plan`" di "Bentuk alat" menyebut satu `Output`. Sesudah spec ini dibangun ada dua, dan AC-4 di sana tetap benar karena hanya memeriksa awal baris.
- [ ] Tabel "Kasus gagal yang sudah diputuskan" belum punya baris untuk format native yang menolak `build.product_name`. Di macOS itu aturan `M-1` spec ini.
- [ ] Tabel "Cara aplikasi memakai nilai", kolom macOS: "Cara baca ditetapkan spec macOS scope 009" menjadi rujukan ke spec ini.
- [ ] Paragraf di bawah aturan `K` yang menyebut salinan di `project.pbxproj` sebagai pelanggaran `K-2` yang diketahui: tidak berlaku lagi sesudah milestone 1 spec ini.
- [ ] Baris "User memberi nama produk atau dasar pengenal akhir" di "Kasus gagal yang sudah diputuskan": kalimat tentang mengganti salinan lewat spec `macos/0001` tidak berlaku lagi sesudah milestone 1 spec ini.
- [ ] Uji T-28 sampai T-32 di `config_contract.rs` tidak boleh menghitung keluaran `plan` atau mengunci daftar path keluarannya, karena spec OS menambah keluaran.
- [ ] `wc -l` di AC-6 mencetak spasi di depan angka pada macOS. Tambahkan `| tr -d ' '`.

Lainnya:

- [x] Jawaban atas Follow-up "Spec macOS scope 009" di spec `_root/0004`: `K-2` dipenuhi dengan generator, akhiran `.macos` dipertahankan, judul jendela ikut nama produk, `PRODUCT_NAME` tidak ikut, dan belum ada berkas untuk pengaturan lokal.
- [ ] Scope 010: judul jendela bukan entri katalog, sejalan dengan aturan `L-1` spec `_root/0006`. Bila kelak judul butuh teks lain di samping nama produk, spec scope 010 memutuskan bentuknya.
- [ ] Scope 012: `scripts/build-macos.sh` dan CI menjalankan `cargo run -p desktop-codegen --locked -- config check` sebelum `xcodebuild`, dan mencatat hasil AC-3 serta AC-4 per arsitektur.
- [ ] Scope 002 (spec pemeriksaan arsitektur, bila ditulis): jadikan AC-2 pemeriksaan tetap, supaya tiga setelan turunan tidak kembali ke `project.pbxproj`.
- [ ] `/sync` atau lead, sesudah milestone 1: `docs/platforms/macos/README.md` dan `AGENTS.md` root menyebut `apps/macos/Config/Shared.xcconfig` sebagai berkas hasil generator dan perintah `config generate`.
- [ ] Bila nama di menu bar harus sama dengan nama produk, perbarui spec ini dan spec `macos/0001` bersama: `PRODUCT_NAME` ikut nama produk, dan path bundel di AC-4, AC-10, serta rencana manual spec `macos/0001` berubah. Percobaan menunjukkan `INFOPLIST_KEY_CFBundleName` tidak dipakai Xcode, jadi tidak ada jalan tengah lewat setelan.
- [ ] Bila pengembang butuh pengenal bundel sendiri untuk tim pribadi, perbarui spec ini dengan berkas `Local.xcconfig` yang diabaikan git dan perubahan `.gitignore` lewat spec `_root/0001`.
- [ ] AC-7 dan milestone 2 menunggu sesi GUI (`owner: user`).
- [ ] Lead: tambahkan tautan balik ke spec ini di berkas scope 009.
