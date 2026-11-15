# 0001. Sapaan di jendela macOS

**Scope**: [004 - Sapaan di jendela macOS](../../../scope/004%20-%20Sapaan%20di%20jendela%20macOS.md)
**Date**: 2026-11-15
**Status**: Proposed

## Summary

Spec ini menghubungkan jendela SwiftUI ke fungsi `hello` di Rust Core. Jembatan kecil `desktop-macos-ffi` mengekspos `hello` lewat UniFFI 0.32.2 (alat yang membuat kode Swift dari API Rust), binding Swift hasil generator disimpan di `bindings/swift/` dan dilacak git, dan aplikasi di `apps/macos/` memakainya lewat paket Swift lokal. Hampir semua bukti berjalan tanpa jendela (`cargo test`, `swift test`, dan `xcodebuild build`). Hanya langkah mengetik nama dan menekan tombol yang butuh sesi GUI. Setiap isi file dan perintah di spec ini sudah dicoba di Mac koordinasi, jadi kamu cukup menyalinnya.

## Requirements

**User stories**:

* Sebagai pengguna Mac, saya ingin mengetik nama, menekan tombol, dan melihat sapaan, supaya saya tahu jendela native benar benar memakai Rust Core.
* Sebagai developer aplikasi macOS, saya ingin satu perintah yang membuat ulang binding Swift dan satu perintah uji tanpa jendela, supaya perubahan kontrak di Rust langsung terlihat di Swift.
* Sebagai pemeriksa, saya ingin tahu siapa yang memiliki memori di batas Rust dan Swift, dan melihat uji yang membuktikannya, tanpa membuka aplikasi.

**Acceptance criteria**:

Angka kriteria merujuk tabel di [Penerimaan dan bukti fondasi](../../../quality/README.md). Vektor V-1 sampai V-11 dan aturan R-1 sampai R-9 adalah milik [spec `_root/0002`](../../_root/0002-kontrak-sapaan-bersama/index.md). Semua perintah dijalankan dari akar repo pada Mac Apple Silicon.

* **AC-1** (Done when "binding dapat dibuat ulang", kriteria 4): pada working tree yang bersih, dengan isi `bindings/swift/` sudah masuk commit, keempat perintah pertama di bagian **Membuat ulang binding** keluar dengan kode 0. Sesudahnya `git diff --exit-code -- bindings/swift` keluar dengan kode 0 dan `git status --short -- bindings/swift` tidak mencetak apa pun. `git ls-files bindings/swift` mencetak tepat empat path: `bindings/swift/Package.swift`, `bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift`, `bindings/swift/Sources/desktop_macos_ffiFFI/desktop_macos_ffiFFI.h`, dan `bindings/swift/Sources/desktop_macos_ffiFFI/module.modulemap`.
* **AC-2** (kriteria 5 dan 14): Swift memanggil `hello` di Core lewat binding, tanpa jendela. Sesudah `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin`, perintah `swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"` keluar dengan kode 0. Keluaran memuat `Test run with 7 tests in 1 suite passed`, yaitu tujuh uji suite `BindingContractTests`. Uji itu membuktikan V-1, V-2, V-6, V-7, V-10, dan V-11 byte demi byte lewat `CoreGreeter`.
* **AC-3** (kriteria 14, aturan R-8): ViewModel mengirim isi kolom apa adanya dan menyimpan hasil apa adanya. Keluaran perintah `swift test` yang sama memuat `Test run with 4 tests in 1 suite passed`, yaitu empat uji suite `GreetingViewModelTests`: `greeting` bernilai `nil` sebelum tombol ditekan, nama `Ayu` memberi `Hello, Ayu!` lewat Core yang asli, nama kosong memberi `Hello, !`, dan dengan greeter tiruan nama `"  Ayu  "` sampai tanpa perubahan serta teks yang dikembalikan greeter tiruan tampil tanpa perubahan.
* **AC-4** (Done when "aplikasi terbuka di macOS" bagian build, kriteria 5): `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build` keluar dengan kode 0 dan mencetak `** BUILD SUCCEEDED **`. Perintah yang sama dengan `-configuration Release` juga berhasil. Untuk kedua konfigurasi, dengan `<K>` diganti `Debug` atau `Release`: `lipo -archs apps/macos/DerivedData/Build/Products/<K>/DesktopMac.app/Contents/MacOS/DesktopMac` mencetak `arm64`, dan `nm apps/macos/DerivedData/Build/Products/<K>/DesktopMac.app/Contents/MacOS/* 2>/dev/null | grep -c uniffi_desktop_macos_ffi_fn_func_hello` mencetak angka 1 atau lebih. Artinya aplikasi membawa jembatan Rust di dalamnya.
* **AC-5** (Done when "menekan tombol sesudah mengisi nama menampilkan nilai dari Core", kriteria 5 dan 14; langkah manual, `owner: user`): pada sesi GUI, langkah 1 sampai 5 di bagian **Rencana verifikasi manual** memberi hasil yang tertulis. Jendela berjudul `Desktop Foundation` terbuka, nama `Ayu` lalu tombol `Greet` menampilkan `Hello, Ayu!`, dan kolom kosong menampilkan `Hello, !`.
* **AC-6** (kriteria 23): kepemilikan di batas FFI (batas antara kode Rust dan Swift) tertulis dan diuji. Bagian **Kepemilikan di batas FFI** spec ini adalah sumbernya. Komentar dokumen crate di `crates/macos-ffi/src/lib.rs` menyebut path spec ini dan merangkum aturannya. `crates/macos-ffi/Cargo.toml` memuat `unsafe_code = "forbid"`, dan `grep -rn unsafe crates/macos-ffi/src` tidak mencetak apa pun. Uji `v10NulInTheMiddleIsKept`, `v11LongNameIsNotCut`, dan `repeatedCallsFromManyThreads` di `BindingContractTests` lulus, begitu juga uji Rust `forwards_to_core` lewat `cargo test -p desktop-macos-ffi --locked`.
* **AC-7** (kriteria 5, aturan R-8 untuk jembatan): jembatan mengekspos `hello` dengan nama dan tanda tangan yang sama dan hanya meneruskan panggilan. `grep -nE '^[[:space:]]*pub fn ' crates/macos-ffi/src/lib.rs` mencetak tepat satu baris, yang berisi `pub fn hello(name: String) -> String {`. `grep -c 'desktop_core::hello(name)' crates/macos-ffi/src/lib.rs` mencetak `1`. `grep -c '^public func hello(name: String) -> String' bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift` mencetak `1`.
* **AC-8** (kriteria 14): hasil yang tampil hanya berasal dari Core, lewat satu jalur. `git grep --untracked -lF 'Hello, ' -- '*.swift'` mencetak tepat dua baris, keduanya di bawah `apps/macos/Tests/`. `git grep --untracked -l 'import DesktopMacosFFI' -- apps/macos` mencetak tepat satu baris: `apps/macos/Interop/Greeter.swift`. `grep -c 'Text(verbatim: model.greeting' apps/macos/Presentation/GreetingView.swift` mencetak `1`, jadi hasil ditampilkan sebagai teks biasa. `git grep --untracked -lF 'Hello, {' -- '*.rs'` tetap mencetak tepat satu baris: `crates/core/src/greeting.rs`.
* **AC-9** (kriteria 5 lewat aturan [spec `_root/0001`](../../_root/0001-batas-workspace-dan-pemilik/index.md)): workspace tetap hijau dan arah dependensi sesuai tabel spec 0001. `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, dan `cargo clippy -p desktop-macos-ffi --all-targets --all-features --locked -- -D warnings` keluar dengan kode 0. `cargo tree -p desktop-macos-ffi --depth 1 --prefix none --locked` mencetak tepat tiga baris, yang diawali `desktop-macos-ffi v0.1.0`, `desktop-core v0.1.0`, dan `uniffi v0.32.2`. `cargo tree -p desktop-macos-ffi --target all --all-features --prefix none --locked | grep -cE '^(desktop-windows|desktop-linux|gtk|gtk4|libadwaita|windows-reactor) '` mencetak `0`, jadi jembatan tidak membawa crate aplikasi atau pustaka UI. `cargo tree -p desktop-core --target all --all-features --prefix none --locked` tetap mencetak tepat satu baris.
* **AC-10** (bukti Kompatibilitas di [rujukan macOS](../../../platforms/macos/README.md), mendukung kriteria 4 dan 5): kombinasi yang dipakai tercatat bersama keluaran perintahnya dan sama dengan tabel **Kombinasi yang dipakai**. Perintahnya: `sw_vers -productVersion`, `uname -m`, `xcodebuild -version`, `swift --version`, `rustc --version`, `xcrun --sdk macosx --show-sdk-version` (mencetak `27.0`), dan `grep -A1 '^name = "uniffi"$' Cargo.lock` (baris kedua `version = "0.32.2"`). `grep -c 'bindings_contract_version = 30' bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift` mencetak `1`. `vtool -show-build apps/macos/DerivedData/Build/Products/Release/DesktopMac.app/Contents/MacOS/DesktopMac` memuat `minos 14.0`.
* **AC-11** (kriteria 1 lewat spec `_root/0001`): repo tetap bersih dan file bersama tidak disentuh. Sesudah semua perintah AC-1 sampai AC-4 dijalankan pada working tree yang bersih, `git status --short` tidak mencetak apa pun. Setiap commit fitur ini memakai subject yang memuat `spec macos/0001 step <n>`, dan `git log --name-only --format= --grep='spec macos/0001 step' | sort -u` hanya mencetak path dari tabel **File yang boleh diubah tiap milestone** dan kalimat di bawahnya. Jadi `.gitignore` dan `rust-toolchain.toml` tidak muncul di daftar itu. Pada commit langkah 1, `Cargo.toml` akar sama persis dengan bagian **Isi file**: hanya baris `members` yang berubah dan tabel `[workspace.dependencies]` yang ditambahkan.
* **AC-12** (kriteria 32): fitur ini hanya membuat yang dibutuhkan alur sapaan. `git ls-files apps/macos bindings/swift crates/macos-ffi` mencetak tepat 16 path dari tabel **Tata letak**, jadi tidak ada folder `Platform`, `DesignSystem`, atau `Resources`. Daftar path dari perintah `git log` di **AC-11** tidak memuat path di bawah `scripts/` atau `.github/`. `grep -c ui-testing apps/macos/DesktopMac.xcodeproj/project.pbxproj` mencetak `0`, jadi tidak ada target uji UI.
* **AC-13** (bukti Penggunaan nyata di rujukan macOS, mendukung kriteria 5 dan 14; langkah manual, `owner: user`): pada sesi GUI, langkah 6 sampai 9 di bagian **Rencana verifikasi manual** memberi hasil yang tertulis. Tombol Return menjalankan `Greet` tanpa mouse, VoiceOver membacakan kolom, tombol, dan hasil, dan Run dari Xcode membuka jendela yang sama. Berkas yang dibuat atau diubah Xcode pada langkah 9 dicatat dan tidak menggagalkan AC ini.

## Decision

**Chosen option**: Opsi 1: binding terlacak sebagai paket Swift lokal, modul Swift yang diuji `swift test`, dan target aplikasi Xcode yang tipis.

Jembatan `desktop-macos-ffi` adalah pustaka statis yang mengekspos `hello` lewat makro UniFFI. Generator di crate yang sama menulis binding Swift ke `bindings/swift/`, yang dilacak git dan dipakai `apps/macos/` sebagai paket Swift lokal. Logika Swift tinggal di dua modul paket (`Interop` dan `Presentation`), dan proyek Xcode hanya berisi titik masuk aplikasi.

Keputusan turunannya:

| Hal | Keputusan |
| --- | --- |
| UniFFI | Versi persis `=0.32.2`, mode makro (`#[uniffi::export]` dan `uniffi::setup_scaffolding!()`), tanpa berkas UDL. `default-features = false`. |
| Jenis crate jembatan | `crate-type = ["staticlib", "lib"]`. Pustaka statis ditautkan ke dalam aplikasi. `lib` dipakai uji Rust. |
| Generator binding | Bin `uniffi-bindgen-swift` di crate jembatan, di balik fitur `bindgen`. Build biasa tidak ikut membangunnya. |
| Binding di git | Tiga berkas hasil generator dan satu `Package.swift` tulisan tangan dilacak git. Pustaka hasil build tinggal di `target/`. |
| Bukti binding mutakhir | Jalankan generator, lalu `git diff --exit-code -- bindings/swift`. |
| Struktur Swift | Paket lokal `apps/macos/Package.swift` dengan modul `Interop` dan `Presentation`. Target aplikasi `DesktopMac` hanya berisi folder `App/`. |
| Cara menaut | Target `systemLibrary` untuk header, `linkedLibrary("desktop_macos_ffi")`, dan path pencarian pustaka dari pemanggil (`-Xlinker -L` atau `LIBRARY_SEARCH_PATHS`). |
| Proyek Xcode | `apps/macos/DesktopMac.xcodeproj`, ditulis tangan dan dilacak git. Fase skrip `Build Rust bridge` menjalankan `cargo build` sebelum menaut. |
| Versi minimum macOS | `14.0`. |
| Arsitektur | `arm64` saja, target Rust `aarch64-apple-darwin`. Intel `x86_64` tidak didukung di fase ini. |
| Tanda tangan kode | Ad hoc (tanda tangan lokal tanpa akun Apple), tanpa sandbox dan tanpa entitlement. |
| State dan thread | `GreetingViewModel` bertanda `@MainActor` dan `@Observable`. `hello` dipanggil sinkron di thread utama. |
| Uji Swift | Swift Testing lewat `swift test`. Tidak ada target uji UI. Alur jendela dibuktikan dengan langkah manual. |
| Teks antarmuka | Literal bahasa Inggris (`Desktop Foundation`, `Name`, `Greet`) sampai scope 010. Hasil sapaan memakai `Text(verbatim:)`. |

Nilai dari spec lain yang dipakai di sini, tanpa diputuskan ulang:

| Nilai | Isi | Pemilik |
| --- | --- | --- |
| Nama paket, path, dan pemilik | `desktop-macos-ffi` di `crates/macos-ffi/`, `bindings/swift/`, `apps/macos/`, semuanya milik Aplikasi macOS | Spec `_root/0001` |
| Aturan manifest akar | Menambah baris `members` dan `[workspace.dependencies]` dengan `desktop-core = { path = "crates/core" }` | Spec `_root/0001` |
| Edisi, toolchain, versi | Edisi `2024`, Rust `1.99.1`, `version = "0.1.0"` dari `[workspace.package]` | Spec `_root/0001` |
| Arah dependensi | `desktop-macos-ffi` boleh bergantung pada `desktop-core` dan alat binding. `apps/macos/` hanya memakai `bindings/swift/` dan pustaka hasil build jembatan | Spec `_root/0001` |
| Kontrak `hello` | `pub fn hello(name: String) -> String`, hasil `Hello, {name}!`, tidak pernah gagal | Spec `_root/0002` |
| Vektor dan aturan | V-1 sampai V-11, R-1 sampai R-9 | Spec `_root/0002` |

## Rationale

Alasan, opsi yang ditimbang, daftar keputusan beserta pilihan keduanya, dan bukti percobaan ada di [rationale.md](rationale.md).

## Feature design

**Data model sketch**:

Tidak ada model data dan tidak ada yang disimpan. Nilai yang hidup di aplikasi hanya dua properti `GreetingViewModel`:

| Properti | Tipe | Isi |
| --- | --- | --- |
| `name` | `String`, awalnya `""` | Isi kolom nama, terikat langsung ke `TextField`. Tidak pernah diubah oleh kode. |
| `greeting` | `String?`, awalnya `nil` | Hasil terakhir dari greeter. Hanya bisa diubah oleh `greet()`. |

**State transitions**:

`greeting` berpindah dari `nil` ke hasil pertama saat `greet()` pertama dipanggil. Setiap panggilan berikutnya mengganti nilainya dengan hasil baru. Tidak ada keadaan memuat atau gagal, karena `hello` sinkron dan tidak bisa gagal.

**API surface**:

| Permukaan | Tanda tangan | Masukan | Hasil | Akses | Kesalahan |
| --- | --- | --- | --- | --- | --- |
| Rust, `desktop_macos_ffi::hello` | `pub fn hello(name: String) -> String` dengan `#[uniffi::export]` | `name`: setiap `String` | Hasil `desktop_core::hello(name)` tanpa perubahan | Publik | Tidak ada |
| Swift, modul `DesktopMacosFFI` (hasil generator) | `public func hello(name: String) -> String` | `name`: setiap `String` Swift di bawah 2 GiB | `String` Swift baru | Publik di modul, hanya diimpor `Interop` | Tidak melempar. Lihat **Kepemilikan di batas FFI** untuk panik dan binding usang. |
| Swift, modul `Interop` | `public protocol Greeter: Sendable { func hello(name: String) -> String }` dan `public struct CoreGreeter: Greeter` | Sama | Sama | Publik | Tidak ada |
| Swift, modul `Presentation` | `GreetingViewModel` (`name`, `greeting`, `greet()`, `init(greeter:)`) dan `GreetingView(model:)` | Lihat tabel data | `greeting` diisi hasil greeter | Publik, `@MainActor` | Tidak ada |
| Bin `uniffi-bindgen-swift` | `uniffi::uniffi_bindgen_swift()` | Path pustaka statis dan folder keluaran | Berkas `.swift`, `.h`, dan `module.modulemap` | Hanya dengan `--features bindgen` | Keluar dengan kode bukan 0 dan pesan dari UniFFI |

Alur panggilan: `GreetingView` → `GreetingViewModel.greet()` → `Greeter.hello(name:)` → `DesktopMacosFFI.hello(name:)` → `desktop_macos_ffi::hello` → `desktop_core::hello`. Hasil kembali lewat jalur yang sama dan disimpan di `greeting`.

**Kepemilikan di batas FFI**:

Setiap nilai yang melintas disalin. Tidak ada pointer, objek, handle, atau callback yang dibagi antara Rust dan Swift, dan tidak ada yang tersisa sesudah satu panggilan selesai. Semua kode di batas ini dihasilkan UniFFI. Tidak ada kode batas yang ditulis tangan.

| Nilai | Yang mengalokasi | Pemilik sesudah melintas | Yang membebaskan |
| --- | --- | --- | --- |
| `name`, dari Swift ke Rust | Kode Swift hasil generator menyalin byte UTF-8 nama ke buffer baru yang dialokasi Rust (`ffi_desktop_macos_ffi_rustbuffer_from_bytes`) | Rust. Kode scaffolding (kode penghubung yang dibuat makro UniFFI) mengubah buffer itu menjadi `String` | Rust, saat `String` itu selesai dipakai `desktop_core::hello`. `String` Swift yang asli tetap milik Swift. |
| Hasil, dari Rust ke Swift | Rust. `String` hasil diserahkan sebagai buffer milik Rust | Swift menerima buffer, menyalin isinya ke `String` Swift baru | Kode Swift hasil generator memanggil `ffi_desktop_macos_ffi_rustbuffer_free` tepat sesudah menyalin (`defer` di `FfiConverterString.lift`) |
| Status panggilan (`RustCallStatus`) | Swift, di stack, untuk setiap panggilan | Tetap milik Swift. Rust hanya menulis kode hasil ke dalamnya. Saat panik, Rust juga mengisi buffer pesan yang dialokasi Rust | Stack Swift. Buffer pesan panik dibaca lalu dibebaskan kode Swift hasil generator |

Saat menurunkan `name`, kode Swift hasil generator juga membuat salinan UTF-8 sementara milik Swift, yang hilang begitu panggilan `rustbuffer_from_bytes` selesai.

Aturan lain di batas ini:

* **Panjang, bukan terminator.** Buffer membawa panjangnya sendiri, jadi karakter NUL di tengah nama tidak memotong apa pun (V-10). Panjang nama yang dikirim Swift disimpan sebagai `Int32`, jadi satu nama dibatasi 2.147.483.647 byte lewat jembatan. Nama yang lebih panjang menghentikan aplikasi. Kontrak Core sendiri tidak punya batas.
* **Kesalahan.** `hello` tidak bisa gagal, jadi fungsi Swift tidak bertanda `throws` dan tidak ada tipe kesalahan yang dipetakan. Fungsi Core yang kelak bisa gagal (aturan R-4) muncul di Swift sebagai fungsi `throws` dengan kesalahan bertipe. Bentuknya diputuskan spec yang menambah fungsi itu.
* **Panik.** Panik di Rust ditangkap scaffolding UniFFI dan tidak pernah merambat ke Swift sebagai perilaku tak terdefinisi. Untuk fungsi yang tidak `throws`, kode Swift hasil generator menghentikan aplikasi dengan pesan paniknya. `hello` tidak panik untuk masukan apa pun, jadi jalur ini tidak tercapai.
* **Binding usang.** Pada panggilan pertama, kode Swift membandingkan versi kontrak UniFFI (`30`) dan checksum tiap fungsi dengan pustaka Rust. Kalau berbeda, aplikasi dan uji berhenti dengan pesan `UniFFI API checksum mismatch` atau `UniFFI contract version mismatch`. Obatnya: jalankan perintah di **Membuat ulang binding**.
* **Thread.** `hello` boleh dipanggil dari thread mana pun, juga bersamaan. Aplikasi memanggilnya dari thread utama, karena untuk nama yang diketik pengguna panggilannya sangat singkat (spec `_root/0002`, bagian API surface).
* **Sumber daya.** Jembatan adalah pustaka statis yang ditautkan ke dalam aplikasi. Tidak ada pustaka dinamis yang dimuat saat berjalan, tidak ada berkas yang dibuka, dan tidak ada yang perlu dilepas saat aplikasi ditutup.
* **`unsafe`.** Kode Rust tulisan tangan di jembatan tidak boleh memuat `unsafe` (`unsafe_code = "forbid"`). Lint itu tidak berlaku untuk hasil makro UniFFI. Kode Swift tulisan tangan tidak memakai pointer.

**Kombinasi yang dipakai**:

| Bagian | Nilai |
| --- | --- |
| Versi minimum macOS | `14.0` (`MACOSX_DEPLOYMENT_TARGET` di proyek Xcode dan `.macOS(.v14)` di kedua `Package.swift`) |
| Host yang diuji | macOS `27.0.1`, Apple Silicon. Versi 14 sampai 26 belum diuji. |
| Arsitektur | `arm64`, target Rust `aarch64-apple-darwin`. Intel `x86_64` tidak didukung. |
| Xcode | `27.1` (build `27A9275`), SDK macOS `27.0` |
| Swift | `6.4`, mode bahasa Swift 6, `swift-tools-version: 6.0` |
| Rust | `1.99.1` dari `rust-toolchain.toml` |
| UniFFI | `0.32.2`, versi kontrak binding `30` |

**Tata letak**:

Enam belas path yang dibuat fitur ini. Kolom sumber menyebut dari mana isinya berasal.

| Path | Sumber isi |
| --- | --- |
| `crates/macos-ffi/Cargo.toml` | Bagian **Isi file** |
| `crates/macos-ffi/src/lib.rs` | Bagian **Isi file** |
| `crates/macos-ffi/src/bin/uniffi-bindgen-swift.rs` | Bagian **Isi file** |
| `bindings/swift/Package.swift` | Bagian **Isi file** (tulisan tangan) |
| `bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift` | Generator. Jangan disunting. |
| `bindings/swift/Sources/desktop_macos_ffiFFI/desktop_macos_ffiFFI.h` | Generator. Jangan disunting. |
| `bindings/swift/Sources/desktop_macos_ffiFFI/module.modulemap` | Generator. Jangan disunting. |
| `apps/macos/Package.swift` | Bagian **Isi file** |
| `apps/macos/Interop/Greeter.swift` | Bagian **Isi file** |
| `apps/macos/Presentation/GreetingViewModel.swift` | Bagian **Isi file** |
| `apps/macos/Presentation/GreetingView.swift` | Bagian **Isi file** |
| `apps/macos/Tests/InteropTests/BindingContractTests.swift` | Bagian **Isi file** |
| `apps/macos/Tests/PresentationTests/GreetingViewModelTests.swift` | Bagian **Isi file** |
| `apps/macos/App/DesktopMacApp.swift` | Bagian **Isi file** |
| `apps/macos/DesktopMac.xcodeproj/project.pbxproj` | Bagian **Isi file** |
| `apps/macos/DesktopMac.xcodeproj/project.xcworkspace/contents.xcworkspacedata` | Bagian **Isi file** |

Selain itu fitur ini mengubah `Cargo.toml` akar dan `Cargo.lock`. Folder `Platform`, `DesignSystem`, dan `Resources` dari rujukan macOS belum dibuat, karena belum ada tanggung jawab nyata untuknya (spec `_root/0001`, baris Folder rencana).

Peran tiap bagian `apps/macos/`:

| Bagian | Jenis | Isi | Boleh mengimpor |
| --- | --- | --- | --- |
| `App/` | Sumber target aplikasi `DesktopMac` di proyek Xcode | Titik masuk `@main` dan jendela | `Presentation`, `SwiftUI` |
| `Presentation/` | Modul paket Swift | `GreetingView` dan `GreetingViewModel` | `Interop`, `SwiftUI`, `Observation` |
| `Interop/` | Modul paket Swift | Protokol `Greeter` dan `CoreGreeter` | `DesktopMacosFFI` |
| `Tests/` | Target uji paket Swift | Uji tanpa jendela | Modul yang diuji, `Testing` |

**Isi file**:

Tulis file berikut persis seperti ini. Komentar kode berbahasa Inggris, mengikuti spec `_root/0002`, sampai scope 002 memutuskan kebijakan bahasa komentar.

`Cargo.toml` di akar sesudah perubahan (baris `members` berubah, tabel terakhir ditambahkan, baris lain tetap):

```toml
[workspace]
resolver = "3"
members = ["crates/core", "crates/macos-ffi"]

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.99"
publish = false

[workspace.dependencies]
desktop-core = { path = "crates/core" }
```

`Cargo.lock` diperbarui dengan menjalankan `cargo build` sekali tanpa `--locked`, lalu ikut di commit yang sama (aturan spec `_root/0001`). Jangan disunting dengan tangan.

`crates/macos-ffi/Cargo.toml`:

```toml
[package]
name = "desktop-macos-ffi"
description = "Thin UniFFI bridge that exposes the public contract of desktop-core to Swift."
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true
autobins = false

[lib]
crate-type = ["staticlib", "lib"]

[[bin]]
name = "uniffi-bindgen-swift"
path = "src/bin/uniffi-bindgen-swift.rs"
required-features = ["bindgen"]

[features]
bindgen = ["uniffi/cli"]

[dependencies]
desktop-core.workspace = true
uniffi = { version = "=0.32.2", default-features = false }

[lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"
```

Catatan untuk manifest ini:

* `uniffi` hanya dipakai satu anggota, jadi ia ditulis di sini dan bukan di `[workspace.dependencies]` (aturan spec `_root/0001`).
* Versinya dikunci persis dengan `=0.32.2`. UniFFI mengunci crate bagiannya sendiri dengan cara yang sama, jadi satu baris ini menentukan semuanya, juga saat `Cargo.lock` dibuat pertama kali di langkah 1.
* `uniffi` tidak ditaruh di bawah tabel `target` untuk macOS. Pustaka itu Rust murni dan bukan pustaka satu OS, jadi aturan netral host spec `_root/0001` tidak memintanya. Akibatnya untuk host lain ada di Follow-up.
* Bin punya `required-features`, jadi `cargo build --workspace`, `cargo test --workspace`, dan `cargo clippy --workspace --all-targets` tidak membangun generator. Hanya `--features bindgen` atau `--all-features` yang membangunnya.
* `autobins = false` mematikan penemuan bin otomatis, jadi berkas lain di `src/bin/` tidak diam diam menjadi bin tanpa syarat fitur.
* `crate-type` tidak boleh memuat `cdylib`. Penaut memilih pustaka dinamis lebih dulu bila keduanya ada di folder yang sama, dan aplikasi lalu butuh pustaka itu saat berjalan.

`crates/macos-ffi/src/lib.rs`:

```rust
//! Thin bridge from Swift to the Rust Core.
//!
//! Design: `docs/specs/macos/0001-sapaan-di-jendela-macos/index.md`.
//!
//! Every exported function has the same name and signature as the function in
//! `desktop_core` and only forwards the call. No behaviour lives here.
//!
//! Ownership at the boundary: every value that crosses is copied. Swift copies
//! the argument into a buffer owned by Rust, and Rust takes it over. Rust
//! returns a buffer owned by Rust, and the generated Swift code copies it into
//! a Swift `String` and frees the buffer. Nothing is shared after a call ends.

uniffi::setup_scaffolding!();

/// Returns the greeting for `name`. Forwards to [`desktop_core::hello`].
#[uniffi::export]
#[must_use]
pub fn hello(name: String) -> String {
    desktop_core::hello(name)
}

#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn forwards_to_core() {
        let pinned: fn(String) -> String = hello;
        assert_eq!(pinned("Ayu".to_string()), "Hello, Ayu!");
        assert_eq!(pinned(String::new()), "Hello, !");
    }
}
```

Rustdoc fungsi yang diekspor ikut disalin ke binding Swift. Jadi rustdoc `hello` di jembatan tidak boleh memuat teks `Hello, ` (lihat **AC-8**), dan mengubah rustdoc itu berarti binding harus dibuat ulang.

`crates/macos-ffi/src/bin/uniffi-bindgen-swift.rs`:

```rust
//! Generates the Swift binding for this crate.

fn main() {
    uniffi::uniffi_bindgen_swift();
}
```

`bindings/swift/Package.swift` (satu satunya berkas tulisan tangan di folder ini):

```swift
// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "DesktopMacosFFI",
    platforms: [.macOS(.v14)],
    products: [
        .library(name: "DesktopMacosFFI", targets: ["DesktopMacosFFI"]),
    ],
    targets: [
        .systemLibrary(name: "desktop_macos_ffiFFI"),
        .target(
            name: "DesktopMacosFFI",
            dependencies: ["desktop_macos_ffiFFI"],
            linkerSettings: [.linkedLibrary("desktop_macos_ffi")]
        ),
    ]
)
```

`apps/macos/Package.swift`. SwiftPM memberi paket lokal identitas dari nama foldernya, jadi paket binding dirujuk sebagai `package: "swift"`:

```swift
// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "DesktopMacModules",
    platforms: [.macOS(.v14)],
    products: [
        .library(name: "Presentation", targets: ["Presentation"]),
    ],
    dependencies: [
        .package(path: "../../bindings/swift"),
    ],
    targets: [
        .target(
            name: "Interop",
            dependencies: [.product(name: "DesktopMacosFFI", package: "swift")],
            path: "Interop"
        ),
        .target(name: "Presentation", dependencies: ["Interop"], path: "Presentation"),
        .testTarget(name: "InteropTests", dependencies: ["Interop"], path: "Tests/InteropTests"),
        .testTarget(name: "PresentationTests", dependencies: ["Presentation", "Interop"], path: "Tests/PresentationTests"),
    ]
)
```

`apps/macos/Interop/Greeter.swift`:

```swift
import DesktopMacosFFI

/// Returns a greeting for a name. The app talks to the Rust Core through this.
public protocol Greeter: Sendable {
    func hello(name: String) -> String
}

/// The real greeter. It forwards to `hello` in the Rust Core through the UniFFI binding.
public struct CoreGreeter: Greeter {
    public init() {}

    public func hello(name: String) -> String {
        DesktopMacosFFI.hello(name: name)
    }
}
```

`apps/macos/Presentation/GreetingViewModel.swift`:

```swift
import Interop
import Observation

/// State for the greeting window. It sends the name as typed and keeps the
/// result exactly as the greeter returned it.
@MainActor
@Observable
public final class GreetingViewModel {
    /// The text in the name field, unchanged.
    public var name: String = ""

    /// The last greeting returned by the greeter. `nil` until the first press.
    public private(set) var greeting: String?

    private let greeter: any Greeter

    public init(greeter: any Greeter = CoreGreeter()) {
        self.greeter = greeter
    }

    /// Sends `name` to the greeter and stores what comes back.
    public func greet() {
        greeting = greeter.hello(name: name)
    }
}
```

`apps/macos/Presentation/GreetingView.swift`:

```swift
import SwiftUI

/// The greeting window content: a name field, a button, and the result label.
public struct GreetingView: View {
    @State private var model: GreetingViewModel

    public init(model: GreetingViewModel) {
        _model = State(initialValue: model)
    }

    public var body: some View {
        VStack(alignment: .leading, spacing: 12) {
            TextField("Name", text: $model.name)
            Button("Greet") {
                model.greet()
            }
            .keyboardShortcut(.defaultAction)
            Text(verbatim: model.greeting ?? "")
        }
        .padding(20)
        .frame(minWidth: 320)
    }
}
```

`apps/macos/Tests/InteropTests/BindingContractTests.swift`:

```swift
import Testing
import Interop

@Suite struct BindingContractTests {
    let greeter = CoreGreeter()

    @Test func v1AyuReturnsHelloAyu() {
        #expect(greeter.hello(name: "Ayu") == "Hello, Ayu!")
    }

    @Test func v2EmptyNameIsAccepted() {
        #expect(greeter.hello(name: "") == "Hello, !")
    }

    @Test func v6UnicodeStaysIntact() {
        let name = "\u{141}ukasz \u{674E} \u{1F469}\u{1F3FD}\u{200D}\u{1F4BB}"
        #expect(greeter.hello(name: name) == "Hello, \(name)!")
        #expect(Array(greeter.hello(name: name).utf8) == Array("Hello, ".utf8) + Array(name.utf8) + Array("!".utf8))
    }

    @Test func v7NoNormalisation() {
        let out = greeter.hello(name: "Cafe\u{301}")
        #expect(Array(out.utf8) == Array("Hello, Cafe\u{301}!".utf8))
        #expect(Array(out.utf8) != Array("Hello, Caf\u{e9}!".utf8))
    }

    @Test func v10NulInTheMiddleIsKept() {
        let out = greeter.hello(name: "A\0B")
        #expect(Array(out.utf8) == Array("Hello, A\0B!".utf8))
        #expect(out.utf8.count == 11)
    }

    @Test func v11LongNameIsNotCut() {
        let name = String(repeating: "a", count: 1_048_576)
        let out = greeter.hello(name: name)
        #expect(out.utf8.count == 1_048_584)
        #expect(out.hasPrefix("Hello, "))
        #expect(out.hasSuffix("!"))
        #expect(out.utf8.dropFirst(7).dropLast(1).elementsEqual(name.utf8))
    }

    @Test func repeatedCallsFromManyThreads() async {
        await withTaskGroup(of: Bool.self) { group in
            for i in 0..<64 {
                group.addTask {
                    (0..<200).allSatisfy { j in
                        CoreGreeter().hello(name: "n\(i)-\(j)") == "Hello, n\(i)-\(j)!"
                    }
                }
            }
            for await ok in group { #expect(ok) }
        }
    }
}
```

`apps/macos/Tests/PresentationTests/GreetingViewModelTests.swift`:

```swift
import Testing
import Interop
import Presentation

private final class RecordingGreeter: Greeter, @unchecked Sendable {
    var received: [String] = []
    func hello(name: String) -> String {
        received.append(name)
        return "<<from greeter>>"
    }
}

@MainActor
@Suite struct GreetingViewModelTests {
    @Test func startsWithoutGreeting() {
        let model = GreetingViewModel()
        #expect(model.greeting == nil)
        #expect(model.name == "")
    }

    @Test func showsWhatCoreReturns() {
        let model = GreetingViewModel()
        model.name = "Ayu"
        model.greet()
        #expect(model.greeting == "Hello, Ayu!")
    }

    @Test func sendsEmptyNameAsIs() {
        let model = GreetingViewModel()
        model.greet()
        #expect(model.greeting == "Hello, !")
    }

    @Test func sendsNameUnchangedAndShowsResultUnchanged() {
        let greeter = RecordingGreeter()
        let model = GreetingViewModel(greeter: greeter)
        model.name = "  Ayu  "
        model.greet()
        #expect(greeter.received == ["  Ayu  "])
        #expect(model.greeting == "<<from greeter>>")
    }
}
```

`apps/macos/App/DesktopMacApp.swift`:

```swift
import Presentation
import SwiftUI

@main
struct DesktopMacApp: App {
    var body: some Scene {
        WindowGroup("Desktop Foundation") {
            GreetingView(model: GreetingViewModel())
        }
        .windowResizability(.contentSize)
    }
}
```

`apps/macos/DesktopMac.xcodeproj/project.xcworkspace/contents.xcworkspacedata`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<Workspace
   version = "1.0">
   <FileRef
      location = "self:">
   </FileRef>
</Workspace>
```

`apps/macos/DesktopMac.xcodeproj/project.pbxproj`. Indentasinya memakai tab. Format proyek adalah versi 77, yang menyinkronkan folder `App/` otomatis, jadi berkas Swift baru di folder itu tidak perlu didaftarkan di sini:

```
// !$*UTF8*$!
{
	archiveVersion = 1;
	classes = {
	};
	objectVersion = 77;
	objects = {

/* Begin PBXBuildFile section */
		D0000000000000000000000A /* Presentation in Frameworks */ = {isa = PBXBuildFile; productRef = D00000000000000000000013 /* Presentation */; };
/* End PBXBuildFile section */

/* Begin PBXFileReference section */
		D00000000000000000000001 /* DesktopMac.app */ = {isa = PBXFileReference; explicitFileType = wrapper.application; includeInIndex = 0; path = DesktopMac.app; sourceTree = BUILT_PRODUCTS_DIR; };
/* End PBXFileReference section */

/* Begin PBXFileSystemSynchronizedRootGroup section */
		D00000000000000000000002 /* App */ = {
			isa = PBXFileSystemSynchronizedRootGroup;
			path = App;
			sourceTree = "<group>";
		};
/* End PBXFileSystemSynchronizedRootGroup section */

/* Begin PBXFrameworksBuildPhase section */
		D00000000000000000000003 /* Frameworks */ = {
			isa = PBXFrameworksBuildPhase;
			buildActionMask = 2147483647;
			files = (
				D0000000000000000000000A /* Presentation in Frameworks */,
			);
			runOnlyForDeploymentPostprocessing = 0;
		};
/* End PBXFrameworksBuildPhase section */

/* Begin PBXGroup section */
		D00000000000000000000004 = {
			isa = PBXGroup;
			children = (
				D00000000000000000000002 /* App */,
				D00000000000000000000005 /* Products */,
			);
			sourceTree = "<group>";
		};
		D00000000000000000000005 /* Products */ = {
			isa = PBXGroup;
			children = (
				D00000000000000000000001 /* DesktopMac.app */,
			);
			name = Products;
			sourceTree = "<group>";
		};
/* End PBXGroup section */

/* Begin PBXNativeTarget section */
		D00000000000000000000006 /* DesktopMac */ = {
			isa = PBXNativeTarget;
			buildConfigurationList = D00000000000000000000010 /* Build configuration list for PBXNativeTarget "DesktopMac" */;
			buildPhases = (
				D00000000000000000000007 /* Build Rust bridge */,
				D00000000000000000000008 /* Sources */,
				D00000000000000000000003 /* Frameworks */,
				D00000000000000000000009 /* Resources */,
			);
			buildRules = (
			);
			dependencies = (
			);
			fileSystemSynchronizedGroups = (
				D00000000000000000000002 /* App */,
			);
			name = DesktopMac;
			packageProductDependencies = (
				D00000000000000000000013 /* Presentation */,
			);
			productName = DesktopMac;
			productReference = D00000000000000000000001 /* DesktopMac.app */;
			productType = "com.apple.product-type.application";
		};
/* End PBXNativeTarget section */

/* Begin PBXProject section */
		D0000000000000000000000B /* Project object */ = {
			isa = PBXProject;
			attributes = {
				BuildIndependentTargetsInParallel = 1;
				LastSwiftUpdateCheck = 2710;
				LastUpgradeCheck = 2710;
				TargetAttributes = {
					D00000000000000000000006 = {
						CreatedOnToolsVersion = 27.1;
					};
				};
			};
			buildConfigurationList = D0000000000000000000000F /* Build configuration list for PBXProject "DesktopMac" */;
			developmentRegion = en;
			hasScannedForEncodings = 0;
			knownRegions = (
				en,
				Base,
			);
			mainGroup = D00000000000000000000004;
			minimizedProjectReferenceProxies = 1;
			packageReferences = (
				D00000000000000000000012 /* XCLocalSwiftPackageReference "." */,
			);
			preferredProjectObjectVersion = 77;
			productRefGroup = D00000000000000000000005 /* Products */;
			projectDirPath = "";
			projectRoot = "";
			targets = (
				D00000000000000000000006 /* DesktopMac */,
			);
		};
/* End PBXProject section */

/* Begin PBXResourcesBuildPhase section */
		D00000000000000000000009 /* Resources */ = {
			isa = PBXResourcesBuildPhase;
			buildActionMask = 2147483647;
			files = (
			);
			runOnlyForDeploymentPostprocessing = 0;
		};
/* End PBXResourcesBuildPhase section */

/* Begin PBXShellScriptBuildPhase section */
		D00000000000000000000007 /* Build Rust bridge */ = {
			isa = PBXShellScriptBuildPhase;
			alwaysOutOfDate = 1;
			buildActionMask = 2147483647;
			files = (
			);
			inputFileListPaths = (
			);
			inputPaths = (
			);
			name = "Build Rust bridge";
			outputFileListPaths = (
			);
			outputPaths = (
			);
			runOnlyForDeploymentPostprocessing = 0;
			shellPath = /bin/sh;
			shellScript = "set -eu\nexport PATH=\"$HOME/.cargo/bin:$PATH\"\ncd \"$SRCROOT/../..\"\nif [ \"$CONFIGURATION\" = \"Release\" ]; then\n  cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin --release\nelse\n  cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin\nfi\n";
		};
/* End PBXShellScriptBuildPhase section */

/* Begin PBXSourcesBuildPhase section */
		D00000000000000000000008 /* Sources */ = {
			isa = PBXSourcesBuildPhase;
			buildActionMask = 2147483647;
			files = (
			);
			runOnlyForDeploymentPostprocessing = 0;
		};
/* End PBXSourcesBuildPhase section */

/* Begin XCBuildConfiguration section */
		D0000000000000000000000C /* Debug */ = {
			isa = XCBuildConfiguration;
			buildSettings = {
				ALWAYS_SEARCH_USER_PATHS = NO;
				ARCHS = arm64;
				CLANG_ENABLE_MODULES = YES;
				COPY_PHASE_STRIP = NO;
				DEBUG_INFORMATION_FORMAT = dwarf;
				ENABLE_STRICT_OBJC_MSGSEND = YES;
				ENABLE_TESTABILITY = YES;
				ENABLE_USER_SCRIPT_SANDBOXING = NO;
				GCC_OPTIMIZATION_LEVEL = 0;
				LIBRARY_SEARCH_PATHS = "\"$(SRCROOT)/../../target/aarch64-apple-darwin/$(RUST_PROFILE_DIR)\"";
				MACOSX_DEPLOYMENT_TARGET = 14.0;
				ONLY_ACTIVE_ARCH = YES;
				RUST_PROFILE_DIR = debug;
				SDKROOT = macosx;
				SWIFT_ACTIVE_COMPILATION_CONDITIONS = DEBUG;
				SWIFT_OPTIMIZATION_LEVEL = "-Onone";
				SWIFT_VERSION = 6.0;
			};
			name = Debug;
		};
		D0000000000000000000000D /* Release */ = {
			isa = XCBuildConfiguration;
			buildSettings = {
				ALWAYS_SEARCH_USER_PATHS = NO;
				ARCHS = arm64;
				CLANG_ENABLE_MODULES = YES;
				COPY_PHASE_STRIP = NO;
				DEBUG_INFORMATION_FORMAT = "dwarf-with-dsym";
				ENABLE_STRICT_OBJC_MSGSEND = YES;
				ENABLE_USER_SCRIPT_SANDBOXING = NO;
				LIBRARY_SEARCH_PATHS = "\"$(SRCROOT)/../../target/aarch64-apple-darwin/$(RUST_PROFILE_DIR)\"";
				MACOSX_DEPLOYMENT_TARGET = 14.0;
				RUST_PROFILE_DIR = release;
				SDKROOT = macosx;
				SWIFT_COMPILATION_MODE = wholemodule;
				SWIFT_VERSION = 6.0;
			};
			name = Release;
		};
		D0000000000000000000000E /* Debug */ = {
			isa = XCBuildConfiguration;
			buildSettings = {
				CODE_SIGN_IDENTITY = "-";
				CODE_SIGN_STYLE = Automatic;
				COMBINE_HIDPI_IMAGES = YES;
				CURRENT_PROJECT_VERSION = 1;
				DEVELOPMENT_TEAM = "";
				ENABLE_PREVIEWS = YES;
				GENERATE_INFOPLIST_FILE = YES;
				INFOPLIST_KEY_CFBundleDisplayName = "Desktop Foundation";
				LD_RUNPATH_SEARCH_PATHS = "$(inherited) @executable_path/../Frameworks";
				MARKETING_VERSION = 0.1.0;
				PRODUCT_BUNDLE_IDENTIFIER = "com.example.desktop-foundation.macos";
				PRODUCT_NAME = "$(TARGET_NAME)";
				SWIFT_EMIT_LOC_STRINGS = NO;
			};
			name = Debug;
		};
		D00000000000000000000011 /* Release */ = {
			isa = XCBuildConfiguration;
			buildSettings = {
				CODE_SIGN_IDENTITY = "-";
				CODE_SIGN_STYLE = Automatic;
				COMBINE_HIDPI_IMAGES = YES;
				CURRENT_PROJECT_VERSION = 1;
				DEVELOPMENT_TEAM = "";
				ENABLE_PREVIEWS = YES;
				GENERATE_INFOPLIST_FILE = YES;
				INFOPLIST_KEY_CFBundleDisplayName = "Desktop Foundation";
				LD_RUNPATH_SEARCH_PATHS = "$(inherited) @executable_path/../Frameworks";
				MARKETING_VERSION = 0.1.0;
				PRODUCT_BUNDLE_IDENTIFIER = "com.example.desktop-foundation.macos";
				PRODUCT_NAME = "$(TARGET_NAME)";
				SWIFT_EMIT_LOC_STRINGS = NO;
			};
			name = Release;
		};
/* End XCBuildConfiguration section */

/* Begin XCConfigurationList section */
		D0000000000000000000000F /* Build configuration list for PBXProject "DesktopMac" */ = {
			isa = XCConfigurationList;
			buildConfigurations = (
				D0000000000000000000000C /* Debug */,
				D0000000000000000000000D /* Release */,
			);
			defaultConfigurationIsVisible = 0;
			defaultConfigurationName = Release;
		};
		D00000000000000000000010 /* Build configuration list for PBXNativeTarget "DesktopMac" */ = {
			isa = XCConfigurationList;
			buildConfigurations = (
				D0000000000000000000000E /* Debug */,
				D00000000000000000000011 /* Release */,
			);
			defaultConfigurationIsVisible = 0;
			defaultConfigurationName = Release;
		};
/* End XCConfigurationList section */

/* Begin XCLocalSwiftPackageReference section */
		D00000000000000000000012 /* XCLocalSwiftPackageReference "." */ = {
			isa = XCLocalSwiftPackageReference;
			relativePath = .;
		};
/* End XCLocalSwiftPackageReference section */

/* Begin XCSwiftPackageProductDependency section */
		D00000000000000000000013 /* Presentation */ = {
			isa = XCSwiftPackageProductDependency;
			productName = Presentation;
		};
/* End XCSwiftPackageProductDependency section */
	};
	rootObject = D0000000000000000000000B /* Project object */;
}
```

Hal yang perlu kamu tahu tentang proyek ini:

* Fase `Build Rust bridge` berjalan pada setiap build, sebelum kompilasi dan penautan. Ia menambahkan `$HOME/.cargo/bin` ke `PATH` (Xcode tidak mewarisi `PATH` shell), pindah ke akar repo, lalu menjalankan `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin`, dengan `--release` untuk konfigurasi Release.
* `ENABLE_USER_SCRIPT_SANDBOXING = NO` diperlukan, karena `cargo` menulis ke `target/` di luar folder build Xcode.
* `LIBRARY_SEARCH_PATHS` menunjuk `target/aarch64-apple-darwin/debug` atau `release` lewat `RUST_PROFILE_DIR`. Nilainya diapit tanda kutip di dalam, supaya path repo yang memuat spasi tetap utuh. Flag `-ldesktop_macos_ffi` datang dari `bindings/swift/Package.swift`.
* Tidak ada berkas skema yang dilacak. `xcodebuild` dan Xcode membuat skema `DesktopMac` sendiri.
* `MARKETING_VERSION = 0.1.0` disalin dari `version` workspace. Kalau versi workspace naik, nilai ini ikut dinaikkan dengan tangan.

**Membuat ulang binding**:

Jalankan dari akar repo setiap kali API yang diekspor jembatan, rustdoc fungsi yang diekspor, atau versi UniFFI berubah. Perintah kedua menghapus folder keluaran lebih dulu, supaya berkas yang tidak lagi dihasilkan ikut hilang. `Package.swift` berada di luar folder itu.

```bash
cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin
rm -rf bindings/swift/Sources
cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- \
  --swift-sources target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a \
  bindings/swift/Sources/DesktopMacosFFI
cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- \
  --headers --modulemap --module-name desktop_macos_ffiFFI --modulemap-filename module.modulemap \
  target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a \
  bindings/swift/Sources/desktop_macos_ffiFFI
git diff --exit-code -- bindings/swift
```

Generator membaca API dari pustaka statis hasil perintah pertama. Keluarannya sama untuk pustaka debug dan release, dan sama setiap kali dijalankan pada sumber yang sama. Perintah terakhir keluar dengan kode 0 bila binding di git sudah mutakhir. Generator ini tidak memanggil pemformat Swift, jadi hasilnya tidak bergantung pada alat yang terpasang di mesin.

**Perintah build dan uji**:

Semua dari akar repo. Urutan ini adalah bukti Build dan Integrasi dari rujukan macOS.

```bash
# 1. Pustaka Rust (jembatan dan Core), arm64
cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin
# 2. Binding Swift: lihat "Membuat ulang binding"
# 3. Uji tanpa jendela
cargo test --workspace --locked
swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"
# 4. Aplikasi lewat Xcode (fase skrip menjalankan cargo sendiri)
xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac \
  -configuration Debug -derivedDataPath apps/macos/DerivedData build
```

Keluaran build tinggal di `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/`. Ketiganya sudah diabaikan oleh `.gitignore` dari spec `_root/0001` (`/target`, `.build/`, `DerivedData/`), jadi `.gitignore` tidak perlu diubah.

**Value sourcing**:

| Aksi | Nilai yang dihasilkan atau ditampilkan | Sumber |
| --- | --- | --- |
| Tekan `Greet` atau Return | Nama yang dikirim ke Core | `GreetingViewModel.name`, yang terikat ke `TextField`, tanpa perubahan |
| Tekan `Greet` atau Return | Teks label hasil | Nilai kembali `Greeter.hello(name:)`, disimpan di `greeting`, ditampilkan dengan `Text(verbatim:)` |
| Jendela baru terbuka | Teks label hasil | String kosong, karena `greeting` masih `nil` |
| `CoreGreeter.hello` | Sapaan | `DesktopMacosFFI.hello(name:)`, yang memanggil `desktop_core::hello` lewat jembatan |
| Jendela | Judul `Desktop Foundation`, label kolom `Name`, teks tombol `Greet` | Literal di `DesktopMacApp.swift` dan `GreetingView.swift`. Diputuskan spec ini sebagai teks sementara sampai scope 010. |
| Bundel aplikasi | Nama tampil `Desktop Foundation` | `INFOPLIST_KEY_CFBundleDisplayName` di `project.pbxproj`. Teks sementara sampai scope 011. |
| Bundel aplikasi | Nama di menu bar `DesktopMac` | `PRODUCT_NAME`, yang sama dengan nama target, di `project.pbxproj`. Teks sementara sampai scope 011. |
| Bundel aplikasi | Pengenal bundel `com.example.desktop-foundation.macos` | Nilai sementara di `project.pbxproj`. Nilai akhirnya hanya user yang tahu (lihat Follow-up). |
| Bundel aplikasi | Versi `0.1.0` dan build `1` | `MARKETING_VERSION` disalin dari `version` workspace (spec `_root/0001`). `CURRENT_PROJECT_VERSION` ditetapkan spec ini. |
| Bundel aplikasi | Versi minimum macOS `14.0` | `MACOSX_DEPLOYMENT_TARGET` di `project.pbxproj`, ditetapkan spec ini |
| Penautan | Lokasi `libdesktop_macos_ffi.a` | `target/aarch64-apple-darwin/<profil>/`, hasil `cargo build --target aarch64-apple-darwin` |
| Generator | Checksum fungsi dan versi kontrak di binding | Dihitung UniFFI dari pustaka statis |
| Uji Swift | Nilai harapan tiap vektor | Tabel Vektor kontrak di spec `_root/0002` |

**Key invariants**:

* Label hasil hanya pernah menampilkan string kosong (sebelum tombol ditekan) atau nilai yang dikembalikan `Greeter.hello(name:)`, tanpa perubahan.
* Aplikasi mengirim isi kolom nama apa adanya, juga saat kosong. Tombol tidak pernah dinonaktifkan karena isi kolom.
* Tidak ada berkas Swift di luar `apps/macos/Tests/` yang memuat teks `Hello, `.
* Hanya `apps/macos/Interop/Greeter.swift` yang mengimpor `DesktopMacosFFI`.
* `crate-type` jembatan adalah `staticlib` dan `lib`, tanpa `cdylib`.
* Jembatan tidak punya perilaku sendiri. Setiap fungsi yang diekspor bernama dan bertanda tangan sama dengan fungsi `desktop_core`, dan badannya hanya meneruskan panggilan.
* Berkas di `bindings/swift/Sources/` selalu sama dengan keluaran generator untuk commit itu.
* Versi UniFFI yang membuat binding selalu sama dengan versi yang ditautkan ke aplikasi, karena generator dibangun dari crate jembatan itu sendiri.
* `desktop-core` tetap tanpa dependensi, dan `uniffi` tidak pernah masuk ke Core.

**Security model**:

Tidak ada autentikasi, peran, jaringan, atau penyimpanan. Nama adalah teks dari pengguna yang tidak dipercaya dan bisa berupa data pribadi:

* Aplikasi dan jembatan tidak mencatat log dan tidak menyimpan nama atau sapaan. Keduanya hanya hidup di memori `GreetingViewModel` sampai jendela ditutup.
* Hasil ditampilkan dengan `Text(verbatim:)`, jadi isinya tidak dibaca sebagai kunci lokalisasi atau Markdown.
* Aplikasi ditandatangani ad hoc untuk dijalankan lokal. Tidak ada sandbox, hardened runtime, entitlement, atau kredensial penandatanganan. Semua itu milik dokumentasi distribusi (lihat Follow-up).
* Fase skrip Xcode menjalankan `cargo` dengan sandbox skrip dimatikan. Skripnya hanya memanggil `cargo build` di repo ini.
* UniFFI dan dependensinya dikunci oleh `Cargo.lock`, dan setiap perintah bukti memakai `--locked`.
* Tidak ada data yang diatur regulasi, jadi tidak ada lingkup kepatuhan.

**Configuration required**:

Tidak ada variabel lingkungan, kredensial, atau berkas konfigurasi baru. Prasyarat mesin: Xcode `27.1` terpilih lewat `xcode-select`, dan `rustup` dengan `cargo` di `$HOME/.cargo/bin`.

**Kasus gagal yang sudah diputuskan**:

| Kejadian | Perilaku yang diharapkan |
| --- | --- |
| `swift test` dijalankan sebelum pustaka Rust dibangun, atau tanpa `-Xlinker -L` | Penautan gagal dengan `library 'desktop_macos_ffi' not found`. Jalankan `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin` lalu ulangi dengan flag itu. |
| API jembatan berubah tetapi binding belum dibuat ulang | Uji dan aplikasi berhenti dengan `UniFFI API checksum mismatch`, dan `git diff --exit-code -- bindings/swift` gagal sesudah generator dijalankan. Buat ulang binding dan sertakan di commit yang sama. |
| Xcode tidak menemukan `cargo` | Fase `Build Rust bridge` gagal dengan `cargo: command not found`. Pasang `rustup` sehingga `cargo` ada di `$HOME/.cargo/bin`. |
| `--locked` gagal sesudah manifest berubah | Perbarui lockfile dengan `cargo build`, lalu sertakan di commit yang sama (spec `_root/0001`). |
| Kolom nama kosong saat tombol ditekan | Label menampilkan `Hello, !`. Itu hasil yang sah (spec `_root/0002`). |
| Tombol ditekan berkali kali | Setiap tekanan mengganti `greeting` dengan hasil baru. Tidak ada antrean. |
| Generator gagal sesudah `rm -rf bindings/swift/Sources` | Pulihkan dengan `git checkout -- bindings/swift/Sources`, perbaiki penyebabnya, lalu ulangi. |
| `CARGO_TARGET_DIR` atau `target-dir` di konfigurasi Cargo milik user mengarah ke folder lain | Tidak didukung. Semua path di spec ini memakai `target/` di akar repo (spec `_root/0001`). Kosongkan pengaturan itu untuk repo ini. |
| Xcode di GUI menolak paket lokal yang foldernya sama dengan folder proyek | Jangan memindahkan berkas sendiri. Catat pesannya di laporan. Jalan keluarnya adalah memindahkan paket ke subfolder lewat pembaruan spec ini (lihat Follow-up). `xcodebuild` di baris perintah sudah terbukti menerimanya. |
| Build di Mac Intel | Tidak didukung. `cargo build --target aarch64-apple-darwin` gagal karena target itu tidak terpasang di sana. |
| Xcode membuat berkas baru yang tidak diabaikan git saat proyek dibuka di GUI | Jangan di commit. Catat path berkasnya di laporan. Keputusan melacak atau mengabaikannya milik lead (lihat Follow-up). |
| Hook Graphify menolak commit dengan `Graphify extraction failed` | Lihat bagian **Bila hook Graphify menolak commit** di Build plan. |

**Critical test scenarios**:

Uji Swift tanpa jendela di `apps/macos/Tests/`, dijalankan `swift test`:

* Jalur utama lewat binding: `v1AyuReturnsHelloAyu` memanggil `CoreGreeter().hello(name: "Ayu")` dan mengharapkan `Hello, Ayu!`, memeriksa **AC-2**.
* Masukan tepi lewat binding: `v2EmptyNameIsAccepted`, `v6UnicodeStaysIntact`, dan `v7NoNormalisation` membandingkan byte UTF-8, memeriksa **AC-2**.
* Kepemilikan dan panjang: `v10NulInTheMiddleIsKept` (NUL tidak memotong, panjang 11 byte) dan `v11LongNameIsNotCut` (1.048.576 byte masuk, 1.048.584 byte keluar), memeriksa **AC-2** dan **AC-6**.
* Panggilan bersamaan: `repeatedCallsFromManyThreads` menjalankan 12.800 panggilan dari 64 tugas dan setiap hasil cocok dengan namanya, memeriksa **AC-6**.
* ViewModel: `startsWithoutGreeting`, `showsWhatCoreReturns`, `sendsEmptyNameAsIs`, dan `sendsNameUnchangedAndShowsResultUnchanged`, memeriksa **AC-3**.

Uji Rust di `crates/macos-ffi/src/lib.rs`:

* `forwards_to_core` mengikat `hello` ke tipe `fn(String) -> String` dan memeriksa V-1 serta V-2, memeriksa **AC-6** dan **AC-7**.

Pemeriksaan dengan perintah:

* Binding bisa dibuat ulang tanpa selisih, memeriksa **AC-1**.
* Build Debug dan Release, arsitektur, dan simbol jembatan di aplikasi, memeriksa **AC-4**.
* Pencarian `git grep` dan `grep`, memeriksa **AC-7** dan **AC-8**.
* Perintah `cargo` untuk workspace dan pohon dependensi, memeriksa **AC-9**.
* Catatan versi dan `vtool`, memeriksa **AC-10**.
* `git status --short` dan `git ls-files`, memeriksa **AC-11** dan **AC-12**.

Tidak ada skenario izin, karena tidak ada pembatasan akses. Alur di jendela nyata diperiksa dengan langkah manual, memeriksa **AC-5** dan **AC-13**.

**Rencana verifikasi manual**:

Langkah ini hanya dijalankan user pada sesi GUI. `/develop` menyalinnya ke `verify.md` di bawah `### UI / manual` pada milestone 4.

Prasyarat: Mac Apple Silicon dengan macOS 14 atau lebih baru (yang sudah diuji: 27.0.1), Xcode 27.1, sesi desktop yang sedang login, dan repo pada commit yang lulus **AC-4**.

1. Dari akar repo jalankan `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build`. Harapan: `** BUILD SUCCEEDED **`. (**AC-5**)
2. Jalankan `open apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app`. Harapan: satu jendela berjudul `Desktop Foundation` dengan kolom `Name`, tombol `Greet`, dan label hasil yang kosong. (**AC-5**)
3. Ketik `Ayu` di kolom, lalu klik `Greet`. Harapan: label menampilkan `Hello, Ayu!`. (**AC-5**)
4. Hapus isi kolom, lalu klik `Greet`. Harapan: label menampilkan `Hello, !`. (**AC-5**)
5. Tempel teks `Łukasz 李 👩🏽‍💻` ke kolom, lalu klik `Greet`. Harapan: label menampilkan `Hello, Łukasz 李 👩🏽‍💻!` dengan emoji yang utuh. (**AC-5**)
6. Hapus isi kolom, ketik `Budi`, lalu tekan Return tanpa memakai mouse. Harapan: label menampilkan `Hello, Budi!`. (**AC-13**)
7. Nyalakan VoiceOver dengan Command F5 (pada sebagian laptop Fn Command F5), lalu pindah ke kolom, tombol, dan label. Harapan: VoiceOver menyebut `Name` untuk kolom, `Greet` untuk tombol, dan membacakan `Hello, Budi!` untuk label. Matikan VoiceOver. (**AC-13**)
8. Tekan Command Q. Harapan: aplikasi tertutup tanpa dialog. (**AC-13**)
9. Jalankan `open apps/macos/DesktopMac.xcodeproj`, tunggu paket selesai dimuat, lalu tekan Command R. Harapan: build berhasil dan jendela yang sama terbuka. Tutup Xcode, lalu jalankan `git status --short`. Harapan: tidak ada keluaran. Bila ada, catat setiap path yang tercetak dan jangan di commit. Keluaran di sini tidak menggagalkan langkah ini. Yang menentukan adalah build berhasil dan jendela terbuka. (**AC-13**)

Catat host, arsitektur, versi macOS, dan hasil tiap langkah. Langkah yang belum dijalankan tetap tercatat sebagai belum diverifikasi.

## Build plan

Pendekatan build proyek adalah Tracer Bullet. Untuk fitur ini benang tipisnya berjalan dari Core sampai ViewModel lewat binding dan terbukti tanpa jendela. Sesudah itu jendela ditambahkan, lalu bukti dikumpulkan. Satu langkah menjadi satu commit, dengan subject yang memuat `spec macos/0001 step <n>` (dipakai **AC-11**).

Prasyarat: needs scope 001 milestone 1 (workspace dan kerangka Core dari spec `_root/0001` sudah ada di `main`) dan needs scope 003 milestone 1 (`desktop_core::hello` dari spec `_root/0002` sudah ada di `main`).

**Milestone 1: Jembatan, binding, dan uji tanpa jendela** (langkah 1 sampai 3)

1. Buat tiga berkas `crates/macos-ffi/`, ubah `Cargo.toml` akar, jalankan `cargo build` sekali tanpa `--locked` supaya `Cargo.lock` ikut diperbarui, lalu jalankan perintah **AC-9** dan `cargo test -p desktop-macos-ffi --locked`. Langkah ini memenuhi bagian Rust dari **AC-6** dan **AC-7**, satisfies **AC-6**, **AC-7**, **AC-9**
2. Buat `bindings/swift/Package.swift`, jalankan keempat perintah pertama di **Membuat ulang binding**, lalu commit `Package.swift` bersama tiga berkas hasil generator. Sesudah commit, jalankan lagi semua perintah di bagian itu dan pemeriksaan **AC-1**, karena pemeriksaan itu baru bermakna saat berkasnya sudah dilacak, satisfies **AC-1**, **AC-7**
3. Buat `apps/macos/Package.swift`, `Interop/Greeter.swift`, `Presentation/GreetingViewModel.swift`, dan kedua berkas uji, lalu jalankan `swift test` seperti di **AC-2**, satisfies **AC-2**, **AC-3**, **AC-6**

**Milestone 2: Jendela SwiftUI dan proyek Xcode** (langkah 4, after milestone 1)

4. Buat `Presentation/GreetingView.swift`, `App/DesktopMacApp.swift`, `project.pbxproj`, dan `contents.xcworkspacedata`, lalu jalankan `xcodebuild` untuk Debug dan Release seperti di **AC-4**, ulangi `swift test`, dan jalankan pencarian **AC-8**, satisfies **AC-4**, **AC-8**

**Milestone 3: Bukti build, kombinasi versi, dan kebersihan repo** (langkah 5, after milestone 2)

5. Pada working tree yang bersih, jalankan ulang setiap perintah **AC-1** sampai **AC-4** dan **AC-6** sampai **AC-12**, catat perintah dan keluarannya di `verify.md` bagian `## Milestone 3: Bukti build, kombinasi versi, dan kebersihan repo` bersama host dan arsitektur, lalu hapus `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/`, satisfies **AC-1**, **AC-10**, **AC-11**, **AC-12**

**Milestone 4: Sesi GUI di Mac** (langkah 6, after milestone 2; `BLOCKED`, `owner: user`)

6. User menjalankan sembilan langkah di **Rencana verifikasi manual** pada sesi GUI dan mencatat hasilnya di `verify.md` bagian `## Milestone 4: Sesi GUI di Mac`, di bawah `### UI / manual`. Sesi koordinasi tidak menjalankan langkah ini, satisfies **AC-5**, **AC-13**

**File yang boleh diubah tiap milestone**:

| Milestone | File |
| --- | --- |
| 1 | `Cargo.toml` akar, `Cargo.lock`, `crates/macos-ffi/`, `bindings/swift/`, `apps/macos/Package.swift`, `apps/macos/Interop/`, `apps/macos/Presentation/GreetingViewModel.swift`, `apps/macos/Tests/` |
| 2 | `apps/macos/Presentation/GreetingView.swift`, `apps/macos/App/`, `apps/macos/DesktopMac.xcodeproj/` |
| 3 dan 4 | Hanya `verify.md` spec ini |

`/develop` menulis bukti tiap milestone di `verify.md` di bawah heading `## Milestone <n>: <judul>`, dengan judul persis seperti di atas. Setiap milestone juga boleh mengubah `verify.md` spec ini, baris `**Status**` di `index.md` ini, `.graphifyignore` di akar sesuai bagian di bawah, dan empat berkas `graphify-out/` dari hook. `Cargo.toml` akar dan `Cargo.lock` adalah file bersama dengan satu pemilik per wave, jadi langkah 1 butuh izin eksplisit di brief developer.

**Bila hook Graphify menolak commit**:

Hook `pre-commit` menolak commit bila sebuah sumber punya extractor tetapi tidak menghasilkan node, dengan pesan `Graphify extraction failed`. Berkas baru fitur ini bisa terkena, misalnya `Package.swift`, `project.pbxproj`, atau berkas hasil generator. Daftar pengecualiannya tidak ditebak di spec ini. `.graphifyignore` di akar sudah ada di `main` dan memuat `/Cargo.toml`. Saat commit pertama untuk satu jenis berkas ditolak dengan pesan itu, tambahkan satu entri yang sempit ke berkas tersebut, berupa path lengkap yang diawali `/` untuk berkas itu saja, jalankan `git add .graphifyignore` (hook membaca indeks git, bukan working tree), lalu ulangi commit. Jangan mengubah `scripts/update_graphify.py` atau `.githooks/`, dan jangan memakai `--no-verify`. Sebutkan setiap entri yang ditambahkan di laporan.

## Consequences

**Positive**:

* Kriteria 4, 5, 14, dan 23 hampir seluruhnya terbukti tanpa jendela dengan dua perintah, `cargo test` dan `swift test`.
* Binding yang usang ketahuan dua kali: uji berhenti dengan pesan checksum, dan `git diff` menunjukkan selisihnya.
* Versi generator dan versi pustaka UniFFI tidak bisa berbeda, karena keduanya berasal dari satu baris dependensi.
* `.gitignore` dan `rust-toolchain.toml` tidak berubah. Build biasa di workspace tidak membangun generator.
* Batas modul Swift mengikuti rujukan macOS, jadi scope 008 sampai 011 punya tempat yang jelas untuk menambah `DesignSystem` dan `Resources`.

**Negative / tradeoffs**:

* `cargo build --workspace` sekarang mengunduh dan membangun pustaka UniFFI di setiap host, termasuk Windows dan Linux kelak. `Cargo.lock` bertambah dari 1 menjadi sekitar 77 paket.
* `project.pbxproj` ditulis tangan. Xcode bisa menulis ulang berkas itu saat proyek dibuka atau pengaturannya diubah, dan selisihnya harus ditinjau.
* `swift test` butuh flag `-Xlinker -L` dan pustaka Rust yang sudah dibangun. Tanpa itu penautan gagal. Belum ada skrip yang membungkusnya sampai scope 012.
* Mengubah API jembatan atau rustdoc fungsi yang diekspor berarti binding harus dibuat ulang di commit yang sama.
* `bindings/swift/` memuat satu berkas tulisan tangan (`Package.swift`) di samping keluaran generator.
* Hanya macOS 27.0.1 pada arm64 yang diuji. Versi minimum 14.0 adalah pengaturan build, belum bukti berjalan. Mac Intel tidak didukung.
* Lewat jembatan, satu nama dibatasi sekitar 2 GiB, dan panik di Rust menghentikan aplikasi. Keduanya tidak tercapai oleh nama yang diketik pengguna.
* Aplikasi belum memakai sandbox dan hanya ditandatangani ad hoc, jadi belum bisa dibagikan ke Mac lain.
* Teks antarmuka masih literal bahasa Inggris, dan pengenal bundel masih nilai sementara.

**Neutral**:

* Spec `_root/0001` AC-4 menulis bahwa `Cargo.lock` memuat satu paket. Itu benar pada akhir milestone 1 scope 001 dan tidak lagi benar sesudah langkah 1 spec ini.
* Paket binding dirujuk dengan identitas `swift`, yang berasal dari nama foldernya.
* Fase skrip Xcode berjalan pada setiap build. Bila tidak ada perubahan Rust, `cargo` selesai dalam sepersekian detik.
* `/develop` menulis `verify.md` saat build. Spec ini tidak membuatnya.

## Follow-up

* [x] Jawaban atas Follow-up macOS di spec `_root/0002`: jembatan memakai fungsi pembungkus dengan nama dan tanda tangan yang sama (**AC-7**), uji Swift membuktikan V-1, V-2, V-6, V-10 dan juga V-7 serta V-11 (**AC-2**), hasil memakai `Text(verbatim:)` dan `git grep` hanya menemukan berkas uji (**AC-8**), dan UI mengirim isi kolom apa adanya (**AC-3**).
* [x] Jawaban atas Follow-up Scope 004 di spec `_root/0001`: `crates/macos-ffi/` bergabung dan `[workspace.dependencies]` ditambahkan, UniFFI `0.32.2`, jenis crate `staticlib` dan `lib`, `bindings/swift/` dilacak git, target Intel tidak dipakai sehingga `rust-toolchain.toml` tidak berubah, dan tidak ada entri `.gitignore` baru.
* [ ] Lead dan pemilik spec `_root/0001`: bila Intel `x86_64` kelak didukung, `rust-toolchain.toml` butuh `targets` berisi `x86_64-apple-darwin` dan proyek butuh langkah pustaka universal. Itu pembaruan spec `_root/0001` lebih dulu, lalu pembaruan spec ini.
* [ ] Lead dan pemilik spec `_root/0001`: AC-4 spec itu menyebut `Cargo.lock` memuat satu paket tanpa batas waktu. Sesudah langkah 1 spec ini angkanya berubah. Pertimbangkan menambah kata "pada akhir milestone 1" di sana.
* [ ] Lead: bila langkah manual 9 menemukan berkas yang dibuat Xcode dan tidak diabaikan git, putuskan apakah berkas itu dilacak (pembaruan spec ini) atau diabaikan (pembaruan `.gitignore` lewat spec `_root/0001`).
* [ ] Lead: tambahkan tautan balik ke spec ini di berkas scope 004. Sesudah build, `docs/architecture/README.md` bagian "Yang sudah ada" dan `docs/platforms/macos/README.md` perlu menyebut `crates/macos-ffi/`, `bindings/swift/`, dan `apps/macos/` sebagai hal yang sudah ada. Berkas berkas itu di luar ownership sesi penulis spec.
* [ ] Lead: `test-preferences.json` di akar hanya menyebut `cargo test`. Sesudah milestone 1, uji Swift di `apps/macos/Tests/` berjalan lewat perintah `swift test` di **AC-2**. Pertimbangkan mencatatnya di berkas itu supaya `/test` menemukannya.
* [ ] Nilai dari user: pengenal bundel akhir untuk aplikasi macOS. Nilai sementara `com.example.desktop-foundation.macos` cukup untuk build lokal dan wajib diganti sebelum penandatanganan atau distribusi.
* [ ] Scope 002: jadikan pemeriksaan tetap untuk **AC-7** dan **AC-8** (hanya `Interop` yang mengimpor `DesktopMacosFFI`, tidak ada `Hello, ` di Swift selain uji, binding sama dengan keluaran generator), dan putuskan apakah `[lints.rust]` jembatan pindah ke `[workspace.lints]`.
* [ ] Scope 008: token tema untuk Swift dan modul `DesignSystem`, serta pemeriksaan tampilan terang dan gelap. Jarak dan ukuran di `GreetingView` masih angka literal.
* [ ] Scope 009: konfigurasi publik untuk aplikasi macOS. Belum ada yang dibaca aplikasi ini.
* [ ] Scope 010: pindahkan `Desktop Foundation`, `Name`, dan `Greet` ke katalog bahasa. Hasil `hello` tetap ditampilkan dengan `Text(verbatim:)`.
* [ ] Scope 011: ikon aplikasi, nama tampil bundel, dan aset merek lain.
* [ ] Scope 012: bungkus perintah di **Membuat ulang binding** dan **Perintah build dan uji** dalam `scripts/generate-bindings.sh` dan `scripts/build-macos.sh`, jalankan di runner macOS GitHub Actions, catat hasil per arsitektur, dan jaga `MARKETING_VERSION` tetap sama dengan versi workspace.
* [ ] Distribusi (di luar fase fondasi): sandbox, hardened runtime, entitlement, penandatanganan dengan identitas tim, dan notarisasi.
* [ ] Scope 006 dan 007 (menunggu host Windows dan Linux): `cargo build --workspace --locked` di host itu sekarang ikut membangun `desktop-macos-ffi` dan pustaka UniFFI. Itu belum diuji. Bila gagal di sana, perbaiki lewat aturan netral host di spec `_root/0001` dengan pembaruan spec ini.
* [ ] Uji di macOS 14 sampai 26 belum ada. Bila host seperti itu tersedia, jalankan **AC-4** dan langkah manual di sana, atau naikkan versi minimum lewat pembaruan spec ini.
* [ ] Bila langkah manual 9 menunjukkan Xcode di GUI menolak paket lokal di folder yang sama dengan proyek, pindahkan `Package.swift`, `Interop/`, `Presentation/`, dan `Tests/` ke `apps/macos/Modules/` lewat pembaruan spec ini, dan ubah `relativePath` di proyek menjadi `Modules`.
* [ ] `tooling/codegen/` sengaja belum dibuat. Generator binding tinggal di crate jembatan. Scope 002 atau 008 yang pertama membutuhkan folder itu yang membuatnya (spec `_root/0001`).
* [ ] Bila alur di jendela perlu diuji otomatis, tambahkan target uji UI lewat pembaruan spec ini. Uji seperti itu mengambil alih layar, jadi tetap dijalankan user.
* [ ] Menaikkan versi UniFFI: ubah satu baris di `crates/macos-ffi/Cargo.toml`, jalankan `cargo build` supaya `Cargo.lock` ikut berubah, buat ulang binding, jalankan semua AC, dan perbarui tabel **Kombinasi yang dipakai** di spec ini.
