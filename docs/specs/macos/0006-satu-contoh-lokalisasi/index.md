# 0006. Satu contoh lokalisasi di macOS

**Scope**: [010 - Satu contoh lokalisasi](../../../scope/010%20-%20Satu%20contoh%20lokalisasi.md)
**Date**: 2026-11-15
**Status**: In Progress

## Summary

Spec ini membawa katalog bahasa bersama dari spec [`_root/0006`](../../_root/0006-satu-contoh-lokalisasi/index.md) ke aplikasi macOS. Alat `desktop-codegen` menulis satu berkas `Localizable.strings` (format teks resource bahasa Apple) per bahasa di dalam modul Swift `Presentation`, dan tombol sapaan membaca labelnya lewat kunci `greeting.submit`, jadi tombol menampilkan `Greet` dalam bahasa Inggris dan `Sapa` dalam bahasa Indonesia. Bahasa dipilih dari pengaturan bahasa macOS dengan negosiasi bawaan Apple, dan teks selalu tampil sebagai teks biasa. Semua bukti berjalan tanpa jendela lewat `cargo test`, `swift test`, dan `xcodebuild`, kecuali pergantian bahasa di jendela nyata, yang dijalankan user.

## Requirements

**User stories**:

* Sebagai pengguna Mac dengan bahasa sistem Indonesia, saya ingin tombol sapaan memakai bahasa saya, supaya aplikasi terasa milik saya.
* Sebagai developer aplikasi macOS, saya ingin teks tombol datang dari katalog bersama lewat kunci yang sama dengan aplikasi Windows dan Linux, supaya saya tidak menyalin terjemahan dengan tangan.
* Sebagai pemeriksa, saya ingin bukti tanpa jendela bahwa setiap bahasa katalog memberi teks yang benar dan bahwa kunci yang hilang membuat uji gagal.

**Acceptance criteria**:

Angka kriteria merujuk tabel di [Penerimaan dan bukti fondasi](../../../quality/README.md). Aturan L-1 sampai L-12 dan kewajiban OS-1 sampai OS-9 milik spec `_root/0006`. Semua perintah dijalankan dari akar repo pada Mac Apple Silicon. Angka dan daftar yang bertanda "pada akhir milestone 1" hanya berlaku pada titik itu; sesudahnya angka itu bisa bertambah lewat spec lain, misalnya spec `macos/0004`, `macos/0005`, atau spec macOS scope 011.

* **AC-1** (OS-2, kriteria 19 baris "Sumber bersama", konsistensi keluaran generator): `cargo run -p desktop-codegen --locked -- localization check` keluar dengan kode 0. Pada akhir milestone 1 baris terakhirnya tepat `localization: ok, 1 keys, 2 locales, 0 missing translations, 3 outputs up to date`. Pada working tree yang bersih, `cargo run -p desktop-codegen --locked -- localization generate` berakhir `3 outputs, 0 written`, lalu `git diff --exit-code -- shared/localization apps/macos` keluar dengan kode 0. Sesudah `rm -r apps/macos/Presentation/Resources`, perintah yang sama mencetak `wrote apps/macos/Presentation/Resources/en.lproj/Localizable.strings` lalu `wrote apps/macos/Presentation/Resources/id.lproj/Localizable.strings`, baris terakhirnya berakhir `3 outputs, 2 written`, dan `git status --short` tidak mencetak apa pun. `.gitattributes` mendapat tepat satu baris baru: `git diff -U0 <induk commit langkah 1> HEAD -- .gitattributes | grep -c '^+[^+]'` mencetak `1`, dan baris itu tepat `+/apps/macos/**/*.strings text eol=lf`. `git ls-files --eol 'apps/macos/*.strings'` mencetak dua baris yang keduanya memuat `i/lf`, `w/lf`, dan `attr/text eol=lf`.
* **AC-2** (OS-2, OS-8, aturan L-11): isi kedua berkas `Localizable.strings` sama byte demi byte dengan bagian **Isi berkas**. `cargo test -p desktop-codegen --locked --test localization_macos_contract` mencetak `test result: ok. 7 passed`, yaitu uji `m01_` sampai `m07_` di tabel **Uji emitter**. Uji `every_output_follows_the_output_rules` di `tooling/codegen/tests/localization_rules.rs` lulus tanpa diubah, dan begitu juga seluruh `cargo test -p desktop-codegen --locked`.
* **AC-3** (OS-1, kriteria 20, Done when "teks UI dari kunci yang sama"): label tombol sapaan dibaca lewat kunci `greeting.submit`. `grep -c 'Button(LocalizedText.string(.greetingSubmit))' apps/macos/Presentation/GreetingView.swift` mencetak `1`. `git grep -nF '"Greet"' -- 'apps/macos/App/*.swift' 'apps/macos/Presentation/*.swift' 'apps/macos/Interop/*.swift'` tidak mencetak apa pun (berkas `.strings` hasil generator memang memuat `"Greet"`, jadi pencarian ini hanya atas kode Swift). `git grep -lF '"greeting.submit"' -- '*.swift'` mencetak tepat satu baris, `apps/macos/Presentation/LocalizedText.swift`.
* **AC-4** (OS-3, OS-4, OS-8, Done when "aturan fallback"): sesudah `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin`, `swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"` keluar dengan kode 0, dan keluarannya memuat `✔ Suite LocalizedTextTests passed` serta `Test run with 10 tests in 2 suites passed` (empat uji `GreetingViewModelTests` ditambah enam uji `LocalizedTextTests`, pada akhir milestone 1). Uji `bundleListsTheCatalogLanguages`, `picksTheCatalogLanguage`, dan `showsTheTextForEachLanguage` membuktikan nilai di tabel **Nilai hasil**: bundel resource mendaftarkan `en` dan `id` dengan `en` sebagai bahasa pengembangan, pilihan `en` memberi `Greet`, `id` dan `id-ID` memberi `Sapa`, `fr` serta daftar kosong memberi `Greet`, dan bahasa tanpa folder `.lproj` (`xx`) memberi `Greet` lewat resource bahasa sumber. Hasil ini tidak bergantung pada bahasa sistem host uji, karena `language(for:)` selalu menambahkan bahasa sumber sebagai pilihan terakhir. Uji `bundleListsTheCatalogLanguages` sengaja mengunci daftar bahasa nyata, karena yang dibuktikan adalah bahwa bundel macOS mendaftarkan persis bahasa katalog (OS-3). Mengganti bahasa katalog mengubah uji itu lewat pembaruan spec ini.
* **AC-5** (OS-5): kunci yang dipakai kode UI wajib ada di resource. Kode UI hanya bisa menyebut kunci lewat enum `TextKey`, jadi kunci di luar enum gagal dikompilasi. Uji `everyKeyHasTextInEveryLanguage` lulus pada repo. Bukti gagalnya dijalankan di worktree sekali pakai, bukan di working tree: sesudah `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin` di akar repo, jalankan `git worktree add --detach <scratch>/m06-probe HEAD`, tambahkan baris `    case probeMissing = "probe.missing"` tepat sesudah baris `case greetingSubmit = "greeting.submit"` di `<scratch>/m06-probe/apps/macos/Presentation/LocalizedText.swift`, lalu `swift test --package-path <scratch>/m06-probe/apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"`. Harapan: kode keluar bukan 0, dan keluarannya memuat `everyKeyHasTextInEveryLanguage` pada baris yang diawali `✘`. Bersihkan dengan `git worktree remove --force <scratch>/m06-probe`. `<scratch>` adalah folder sementara di luar repo.
* **AC-6** (OS-6, aturan L-6 dan L-7): teks katalog tampil sebagai teks biasa. Tombol menerima `String`, bukan `LocalizedStringKey`, jadi SwiftUI tidak mencari kunci lagi dan tidak menafsirkan Markdown. `git grep -nE 'LocalizedStringKey|String\(localized|NSLocalizedString|Text\("' -- apps/macos/Presentation apps/macos/App` tidak mencetak apa pun. Uji `percentSignsStayPlain` lulus: `LocalizedText.render("100%% *done* <b> & _x_")` sama dengan `100% *done* <b> & _x_`. Uji emitter `m04_` membuktikan peloloskan `%`, `"`, dan `\` di berkas resource. Separuh OS-6 yang lain, nilai placeholder diisikan sebagai teks biasa, sengaja ditunda: katalog belum punya teks berplaceholder, dan API Swift pengisinya dibuat bersama teks pertama itu (Follow-up). Sampai saat itu `TextKey` hanya boleh memuat kunci tanpa placeholder, dan `everyKeyHasTextInEveryLanguage` juga memeriksa bahwa teks hasil `render` tidak memuat `(null)`, yaitu hasil `String(format:)` atas `%1$@` tanpa argumen.
* **AC-7** (OS-7, aturan R-8 spec `_root/0002`): hasil `hello` tetap tampil apa adanya dan tetap bahasa Inggris. Uji `greetingStaysEnglish` lulus: dengan teks tombol `Sapa`, nama `Ayu` memberi `Hello, Ayu!`. `grep -c 'Text(verbatim: model.greeting' apps/macos/Presentation/GreetingView.swift` tetap mencetak `1`. Setiap baris dari `git grep --untracked -lF 'Hello, ' -- '*.swift'` berada di bawah `apps/macos/Tests/`, dan pada akhir milestone 1 jumlahnya tepat tiga.
* **AC-8** (OS-3 di aplikasi, Done when "ketiga aplikasi menampilkan teks UI" bagian build, kriteria 20): `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build` mencetak `** BUILD SUCCEEDED **`, dan perintah yang sama dengan `-configuration Release` juga berhasil. Untuk kedua konfigurasi, dengan `<K>` diganti `Debug` atau `Release` dan `<R>` diganti `apps/macos/DerivedData/Build/Products/<K>/DesktopMac.app/Contents/Resources/DesktopMacModules_Presentation.bundle/Contents/Resources`: `plutil -convert json -o - <R>/en.lproj/Localizable.strings` mencetak tepat `{"greeting.submit":"Greet"}`, dan untuk `id.lproj` tepat `{"greeting.submit":"Sapa"}`. `project.pbxproj` tidak berubah (lihat **AC-10**). Build Release menjalankan build Rust `--release` sendiri lewat fase skrip proyek, jadi build itu dingin dan lebih lama sesudah `target/` dihapus.
* **AC-9** (kriteria 19 dan 20 lewat aturan spec `_root/0001`): workspace dan uji yang sudah ada tetap hijau. `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, dan `cargo run -p desktop-codegen --locked -- architecture check` (aturan A-1 sampai A-13 spec [`_root/0007`](../../_root/0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md), terutama A-8 teks sapaan, A-10 impor jembatan, A-11 paket Swift jarak jauh, dan A-12 tautan Markdown) keluar dengan kode 0. `swift test` yang sama dengan **AC-4** juga memuat `Test run with 7 tests in 1 suite passed` untuk `BindingContractTests`. `cargo tree -p desktop-codegen --depth 1 --prefix none --locked` tetap mencetak tepat tiga baris, dan `cargo tree -p desktop-core --target all --all-features --prefix none --locked` tetap mencetak tepat satu baris.
* **AC-10** (kriteria 32): spec ini hanya menambah yang dibutuhkan contoh. Setiap commit fitur ini memakai subject yang memuat `spec macos/0006 step <n>`, dan `git log --name-only --format= --grep='spec macos/0006 step' | sort -u` hanya mencetak path dari tabel **File yang boleh diubah tiap milestone**. Daftar itu tidak memuat `apps/macos/DesktopMac.xcodeproj/`, `apps/macos/App/`, `apps/macos/Interop/`, `tooling/codegen/src/lib.rs`, `crates/`, `bindings/`, `scripts/`, `.github/`, `Cargo.toml`, atau `Cargo.lock`. `git ls-files apps/macos bindings/swift crates/macos-ffi | wc -l | tr -d ' '` bertambah tepat empat dibanding commit induk langkah 1, yaitu empat path baru di tabel **Tata letak**. Di `main` saat spec ini ditulis angkanya `16`, jadi hasilnya `20` bila tidak ada spec lain yang menambah path lebih dulu.
* **AC-11** (OS-9, Done when "ketiga aplikasi menampilkan teks UI" bagian jendela; langkah manual, `owner: user`): pada sesi GUI, langkah 1 sampai 7 di bagian **Rencana verifikasi manual** memberi hasil yang tertulis. Dengan bahasa Indonesia, tombol menampilkan dan dibacakan VoiceOver sebagai `Sapa`, dan hasil tetap `Hello, Ayu!`.

Bagian Done when yang menyebut Windows dan Linux bukan AC di sini (lihat Follow-up).

## Decision

**Chosen option**: Opsi 1: berkas `.strings` per bahasa sebagai resource modul `Presentation`, dibaca lewat negosiasi bahasa `Bundle`.

Emitter macOS di `desktop-codegen` menulis `apps/macos/Presentation/Resources/<locale>.lproj/Localizable.strings` untuk setiap bahasa katalog. Paket Swift lokal membawa folder itu sebagai resource modul `Presentation`, dan Xcode menyalinnya ke bundel resource di dalam aplikasi tanpa perubahan pada proyek Xcode. Kode Swift tulisan tangan memilih bahasa dengan `Bundle.preferredLocalizations(from:forPreferences:)` atas `Locale.preferredLanguages`, membaca teks dengan `Bundle.localizedString(forKey:value:table:)`, lalu memberikannya ke tombol sebagai `String`.

Keputusan turunannya:

| Hal | Keputusan |
| --- | --- |
| Format resource | Format teks `.strings` Apple (plist gaya lama, UTF-8), satu berkas `Localizable.strings` per bahasa. Bukan `.xcstrings`. |
| Lokasi | `apps/macos/Presentation/Resources/<locale>.lproj/Localizable.strings`, resource target paket `Presentation` (`resources: [.process("Resources")]`). Bukan folder `apps/macos/Resources/` di tingkat atas. |
| Bahasa bawaan | `defaultLocalization: "en"` di `apps/macos/Package.swift`, sama dengan `sourceLocale`. |
| Nama locale | Nama folder `.lproj` adalah nama locale katalog apa adanya (`en`, `id`, kelak `pt-BR`). Pemetaan identitas, jadi satu ke satu (L-4, OS-8). |
| Pengenal teks | Kunci katalog apa adanya, dengan titik (`"greeting.submit"`). Format `.strings` menerima titik di kunci, jadi aturan ganti `.` dengan `_` di L-11 butir 2 tidak dipakai. |
| Pemilihan bahasa | Negosiasi Apple, `Bundle.preferredLocalizations(from: bundle.localizations, forPreferences: Locale.preferredLanguages + [sourceLocale])`, lalu folder `.lproj` hasilnya dibuka langsung. Bahasa sumber selalu ditambahkan sebagai pilihan terakhir, karena tanpa itu hasil Apple untuk pilihan yang tidak cocok bergantung pada daftar bahasa host (lihat bukti percobaan). Aplikasi tidak menyimpan pilihan bahasa dan tidak punya pengalih bahasa. |
| Kunci di Swift | Enum tulisan tangan `TextKey: String, CaseIterable` di `LocalizedText.swift`. Satu kasus untuk satu kunci yang dipakai kode UI. |
| Teks biasa | Tombol memakai `Button(_ title: some StringProtocol, action:)`. Teks melewati `String(format:arguments:)` tanpa argumen, yang hanya mengubah `%%` menjadi `%`. |
| Placeholder | Emitter mengubah `{nama}` menjadi `%<n>$@`, dengan `<n>` urutan nama di `Entry.placeholders` dimulai dari 1. API Swift untuk mengisi nilai placeholder belum dibuat, karena katalog belum punya teks berplaceholder. Sampai API itu ada, `TextKey` hanya memuat kunci tanpa placeholder. |
| Proyek Xcode | `project.pbxproj` tidak berubah. `knownRegions` tetap `en` dan `Base`. |
| Teks lain | `Name` tetap literal di `TextField("Name", ...)`. Judul jendela milik spec `macos/0004`. Hasil `hello` tetap `Text(verbatim:)`. |

Nilai dari spec lain yang dipakai di sini, tanpa diputuskan ulang:

| Nilai | Isi | Pemilik |
| --- | --- | --- |
| Katalog, kunci, bahasa, dan teks | `shared/localization/catalog.json`: `greeting.submit`, `en` `Greet`, `id` `Sapa`, `sourceLocale` `en` | Spec [`_root/0006`](../../_root/0006-satu-contoh-lokalisasi/index.md) |
| Aturan | L-1 sampai L-12, terutama fallback L-9, teks biasa L-6, placeholder L-7, dan keluaran per OS L-11 | Spec `_root/0006` |
| Kewajiban OS | OS-1 sampai OS-9 | Spec `_root/0006` |
| Tipe `Catalog`, `Entry`, `Text`, `Origin` dan fungsi `parse` | Permukaan Rust modul `localization` | Spec `_root/0006`, bagian API surface |
| Alat, `Output`, `Plan`, perintah, kode keluar | `desktop-codegen` | Spec [`_root/0003`](../../_root/0003-tema-bersama-dan-kontrol-native/index.md) |
| Aturan keluaran | O-1 sampai O-7 | Spec `_root/0003` |
| Paket Swift, modul, perintah `swift test` dan `xcodebuild` | `DesktopMacModules`, modul `Interop` dan `Presentation`, bagian **Perintah build dan uji** | Spec [`macos/0001`](../0001-sapaan-di-jendela-macos/index.md) |
| Judul jendela | Nama produk dari bundel | Spec [`macos/0004`](../0004-konfigurasi-publik/index.md) |

## Rationale

Alasan, pilihan lain, daftar keputusan beserta pilihan keduanya, dan bukti percobaan: lihat [rationale.md](rationale.md).

## Feature design

**Data model sketch**:

Tidak ada basis data dan tidak ada data pengguna. Modelnya adalah `Catalog` dari spec `_root/0006` sesudah fallback L-9, dan berkas turunannya:

| Berkas | Isi | Jumlah |
| --- | --- | --- |
| `apps/macos/Presentation/Resources/<locale>.lproj/Localizable.strings` | Satu pasangan `"<kunci>" = "<teks>";` untuk setiap `Entry`, dengan teks untuk locale itu | Satu berkas per isi `Catalog.locales` |
| `TextKey` di `apps/macos/Presentation/LocalizedText.swift` | Satu kasus per kunci yang dipakai kode UI, `rawValue` sama dengan kunci katalog | Pada akhir milestone 1, satu kasus |

Hubungannya: satu kunci katalog muncul sekali di setiap berkas `.strings`. Satu kasus `TextKey` menunjuk satu kunci katalog. Kunci katalog tanpa kasus `TextKey` boleh ada (belum dipakai macOS). Kasus `TextKey` tanpa kunci katalog dilarang dan ditangkap uji (**AC-5**).

**Nilai hasil** (pada akhir milestone 1):

| Pilihan bahasa pengguna | Bahasa yang dipilih | Teks `greeting.submit` |
| --- | --- | --- |
| `en` | `en` | `Greet` |
| `id` | `id` | `Sapa` |
| `id-ID` | `id` | `Sapa` |
| `fr` | `en` | `Greet` |
| tidak ada | `en` | `Greet` |

**State transitions**:

Tidak ada. Bahasa dibaca setiap kali `LocalizedText.string(_:)` dipanggil, dari pengaturan macOS. Pengguna yang mengganti bahasa sistem melihat bahasa baru sesudah membuka ulang aplikasi.

**API surface**:

Rust, emitter (`tooling/codegen/src/localization/macos.rs`, modul publik `localization::macos`, rustdoc berbahasa Inggris):

```rust
pub const RESOURCES_DIR: &str = "apps/macos/Presentation/Resources";
pub fn outputs(catalog: &Catalog) -> Vec<Output>;
```

| Item | Perilaku |
| --- | --- |
| `outputs` | Fungsi murni. Satu `Output` per isi `Catalog.locales`, dalam urutan itu, dengan path `<RESOURCES_DIR>/<locale>.lproj/Localizable.strings` dan isi menurut **Bentuk berkas `.strings`**. Tidak pernah gagal, karena `Catalog` sudah sah. |
| `localization::SOURCE.plan` | Sesudah skema, menambahkan hasil `macos::outputs(&catalog)` ke `Plan.outputs`. Tidak ada perubahan lain. `tooling/codegen/src/localization/mod.rs` mendapat baris `pub mod macos;`, dan `plan` disusun ulang menjadi `let mut outputs = vec![Output { path: SCHEMA_PATH.to_string(), contents: schema::contents() }]; outputs.extend(macos::outputs(&catalog));` lalu `Ok(Plan { outputs, summary: summary(&catalog) })`. |

**Bentuk berkas `.strings`** (aturan emitter, semua dalam UTF-8 tanpa BOM, akhir baris `\n`):

1. Baris pertama tepat `/* Generated by desktop-codegen from tooling/codegen/src/localization. Do not edit. To regenerate, run: cargo run -p desktop-codegen --locked -- localization generate */`. Teksnya sama dengan penanda skema katalog. Konstanta penanda di `localization/schema.rs` privat, dan `schema.rs` tidak termasuk tabel **Tata letak**, jadi `macos.rs` menulis konstantanya sendiri dengan teks yang sama. Uji `m01_` dan `m07_` membandingkan baris itu dengan teks literal, jadi kedua salinan tidak bisa berbeda diam diam.
2. Untuk setiap `Entry` dalam urutan `Catalog.entries`: satu baris kosong, lalu `/* <description> */`, lalu bila teks untuk locale itu berasal `Origin::Fallback`, baris `/* Missing translation for <locale>, shows the <sourceLocale> text. */`, lalu `"<key>" = "<teks>";`.
3. Tidak ada baris lain. Berkas berakhir dengan tepat satu `\n` sesudah baris pasangan terakhir. Katalog tanpa entri memberi berkas yang hanya berisi baris penanda.
4. `<teks>` dibuat dari teks katalog, karakter demi karakter: `\` menjadi `\\`, `"` menjadi `\"`, `%` menjadi `%%`, dan placeholder `{nama}` menjadi `%<n>$@`. Karakter lain, termasuk huruf non ASCII, emoji, `*`, `_`, `<`, dan `&`, ditulis apa adanya. Karakter kendali tidak mungkin ada (L-6).
5. `<description>` ditulis apa adanya. Polanya (`^[A-Za-z0-9 ,.()-]{1,120}$`) tidak memuat `*` atau `/`, jadi tidak bisa menutup komentar.

Swift, modul `Presentation` (`apps/macos/Presentation/LocalizedText.swift`, tulisan tangan):

| Item | Tanda tangan | Perilaku |
| --- | --- | --- |
| `TextKey` | `public enum TextKey: String, CaseIterable, Sendable` dengan `case greetingSubmit = "greeting.submit"` | Satu kasus per kunci katalog yang dipakai kode UI |
| `LocalizedText.sourceLanguage` | `public static let sourceLanguage = "en"` | Sama dengan `sourceLocale` katalog dan `defaultLocalization` paket |
| `LocalizedText.bundle` | `public static var bundle: Bundle { .module }` | Bundel resource modul `Presentation` |
| `LocalizedText.language(for:)` | `public static func language(for preferences: [String]) -> String` | Hasil pertama `Bundle.preferredLocalizations(from: bundle.localizations, forPreferences: preferences + [sourceLanguage])`, atau `sourceLanguage` bila kosong |
| `LocalizedText.string(_:language:)` | `public static func string(_ key: TextKey, language: String) -> String` | Membuka `<language>.lproj` di `bundle` (atau `<sourceLanguage>.lproj` bila tidak ada), membaca `localizedString(forKey: key.rawValue, value: nil, table: nil)`, lalu `render` |
| `LocalizedText.string(_:)` | `public static func string(_ key: TextKey) -> String` | `string(key, language: language(for: Locale.preferredLanguages))` |
| `LocalizedText.render(_:)` | `public static func render(_ template: String) -> String` | `String(format: template, arguments: [])` |

Isi lengkap berkas Swift ada di bagian **Isi berkas**.

**Value sourcing**:

| Aksi | Nilai yang dihasilkan atau ditampilkan | Sumber |
| --- | --- | --- |
| Tombol sapaan tampil | Label tombol | `LocalizedText.string(.greetingSubmit)`, dari resource bahasa terpilih |
| Memilih bahasa | Bahasa terpilih | `Locale.preferredLanguages` (pengaturan macOS), dinegosiasikan dengan `Bundle.preferredLocalizations` atas `bundle.localizations` |
| Memilih bahasa | Daftar bahasa yang didukung | Folder `.lproj` di bundel resource, yang ditulis emitter dari `Catalog.locales` |
| Memilih bahasa | Bahasa bila tidak ada yang cocok | `LocalizedText.sourceLanguage`, sama dengan `sourceLocale` katalog dan `defaultLocalization` paket |
| Emitter | Path tiap berkas | `RESOURCES_DIR` dan isi `Catalog.locales` |
| Emitter | Kunci dan teks tiap baris | `Entry.key` dan `Entry.texts` sesudah fallback L-9 |
| Emitter | Nomor placeholder | Urutan nama di `Entry.placeholders` (L-7) |
| Emitter | Komentar per entri | `Entry.description`, dan `Origin` untuk baris terjemahan yang hilang |
| Emitter | Penanda | Konstanta penanda, teks di **Bentuk berkas `.strings`** butir 1 |
| Label hasil | Teks hasil | `GreetingViewModel.greeting` dari Core (spec `macos/0001`), tidak diterjemahkan |

**Isi berkas**:

Komentar kode berbahasa Inggris, mengikuti spec `_root/0002`.

`apps/macos/Presentation/Resources/en.lproj/Localizable.strings` (hasil generator):

```text
/* Generated by desktop-codegen from tooling/codegen/src/localization. Do not edit. To regenerate, run: cargo run -p desktop-codegen --locked -- localization generate */

/* Label of the button that sends the name and shows the greeting. */
"greeting.submit" = "Greet";
```

`apps/macos/Presentation/Resources/id.lproj/Localizable.strings` (hasil generator):

```text
/* Generated by desktop-codegen from tooling/codegen/src/localization. Do not edit. To regenerate, run: cargo run -p desktop-codegen --locked -- localization generate */

/* Label of the button that sends the name and shows the greeting. */
"greeting.submit" = "Sapa";
```

`apps/macos/Presentation/LocalizedText.swift`:

```swift
import Foundation

/// Keys of the shared catalog (`shared/localization/catalog.json`) that this app shows.
/// Spec `docs/specs/macos/0006-satu-contoh-lokalisasi/index.md`.
public enum TextKey: String, CaseIterable, Sendable {
    /// Label of the greeting button.
    case greetingSubmit = "greeting.submit"
}

/// Reads interface text from the generated `Localizable.strings` resources, as plain text.
public enum LocalizedText {
    /// The catalog's source language, used when no preferred language matches.
    public static let sourceLanguage = "en"

    /// The resource bundle that holds one `.lproj` folder per catalog language.
    public static var bundle: Bundle { .module }

    /// The catalog language Apple's negotiation picks for `preferences`, else the source language.
    /// The source language is always the last choice, so the result never depends on the host's languages.
    public static func language(for preferences: [String]) -> String {
        Bundle.preferredLocalizations(from: bundle.localizations, forPreferences: preferences + [sourceLanguage])
            .first ?? sourceLanguage
    }

    /// The text for `key` in `language`, as plain text.
    public static func string(_ key: TextKey, language: String) -> String {
        let folder = bundle.path(forResource: language, ofType: "lproj")
            ?? bundle.path(forResource: sourceLanguage, ofType: "lproj")
        let table = folder.flatMap(Bundle.init(path:)) ?? bundle
        return render(table.localizedString(forKey: key.rawValue, value: nil, table: nil))
    }

    /// The text for `key` in the language the user prefers in macOS settings.
    public static func string(_ key: TextKey) -> String {
        string(key, language: language(for: Locale.preferredLanguages))
    }

    /// Turns a resource text into the text people see. Only `%%` changes, into `%`.
    public static func render(_ template: String) -> String {
        String(format: template, arguments: [])
    }
}
```

`apps/macos/Presentation/GreetingView.swift`: hanya satu baris berubah. `Button("Greet") {` menjadi `Button(LocalizedText.string(.greetingSubmit)) {`. Baris lain, termasuk perubahan dari spec lain yang sudah masuk lebih dulu (misalnya spec `macos/0005`), dipertahankan.

`apps/macos/Package.swift`: dua tambahan, baris lain dipertahankan, termasuk perubahan dari spec lain yang sudah masuk lebih dulu.

1. Sesudah baris `name: "DesktopMacModules",` tambahkan baris `defaultLocalization: "en",` dengan indentasi yang sama.
2. Di deklarasi target `Presentation`, tambahkan argumen `resources: [.process("Resources")]` sebagai argumen terakhir, sesudah `path: "Presentation"`. Argumen lain di deklarasi itu tetap. Bila spec `macos/0005` sudah masuk, hasilnya `.target(name: "Presentation", dependencies: ["Interop", "DesignSystem"], path: "Presentation", resources: [.process("Resources")]),`. Bila belum, `.target(name: "Presentation", dependencies: ["Interop"], path: "Presentation", resources: [.process("Resources")]),`.

`apps/macos/Tests/PresentationTests/LocalizedTextTests.swift`:

```swift
import Foundation
import Testing
import Presentation

/// Spec macos/0006: text per language, language choice, missing keys, plain text, and `hello`.
@MainActor
@Suite struct LocalizedTextTests {
    @Test func bundleListsTheCatalogLanguages() {
        #expect(LocalizedText.bundle.localizations.sorted() == ["en", "id"])
        #expect(LocalizedText.bundle.developmentLocalization == "en")
    }

    @Test func picksTheCatalogLanguage() {
        #expect(LocalizedText.language(for: ["en"]) == "en")
        #expect(LocalizedText.language(for: ["id"]) == "id")
        #expect(LocalizedText.language(for: ["id-ID"]) == "id")
        #expect(LocalizedText.language(for: ["fr"]) == "en")
        #expect(LocalizedText.language(for: []) == "en")
    }

    @Test func showsTheTextForEachLanguage() {
        #expect(LocalizedText.string(.greetingSubmit, language: "en") == "Greet")
        #expect(LocalizedText.string(.greetingSubmit, language: "id") == "Sapa")
        #expect(LocalizedText.string(.greetingSubmit, language: LocalizedText.language(for: ["id-ID"])) == "Sapa")
        #expect(LocalizedText.string(.greetingSubmit, language: LocalizedText.language(for: ["fr"])) == "Greet")
        #expect(LocalizedText.string(.greetingSubmit, language: "xx") == "Greet")
    }

    @Test func everyKeyHasTextInEveryLanguage() {
        for language in LocalizedText.bundle.localizations {
            for key in TextKey.allCases {
                let text = LocalizedText.string(key, language: language)
                #expect(text != key.rawValue, "\(key.rawValue) has no text for \(language)")
                #expect(!text.isEmpty, "\(key.rawValue) is empty for \(language)")
                #expect(!text.contains("(null)"), "\(key.rawValue) needs placeholder values")
            }
        }
    }

    @Test func percentSignsStayPlain() {
        #expect(LocalizedText.render("100%% *done* <b> & _x_") == "100% *done* <b> & _x_")
    }

    @Test func greetingStaysEnglish() {
        #expect(LocalizedText.string(.greetingSubmit, language: "id") == "Sapa")
        let model = GreetingViewModel()
        model.name = "Ayu"
        model.greet()
        #expect(model.greeting == "Hello, Ayu!")
    }
}
```

`.gitattributes` di akar mendapat satu baris baru di akhir. Baris yang sudah ada tidak diubah:

```gitattributes
/apps/macos/**/*.strings text eol=lf
```

**Uji emitter** (`tooling/codegen/tests/localization_macos_contract.rs`, hanya memakai path publik `desktop_codegen`, dibuka dengan komentar dokumen `//!`):

* Berkas uji integrasi tidak bisa memakai fungsi bantu berkas uji lain, jadi berkas ini menulis sendiri fungsi `edited(find, replace)` (ganti kemunculan pertama di teks katalog nyata, dan gagal bila `find` tidak ada) dan dua katalog uji, dengan teks yang sama persis dengan `localization_contract.rs`. K-1: `"Greet"` menjadi `"{name}, welcome to {app}."`, lalu `"Sapa"` menjadi `"Selamat datang di {app}, {name}."`. K-2: `["en", "id"]` menjadi `["en", "id", "ms"]`, lalu `"Greet",\n        "id": "Sapa"` menjadi `"Greet"` (`\n` adalah satu baris baru, diikuti delapan spasi). Setiap katalog uji diurai dengan `localization::parse` lalu diberikan ke `localization::macos::outputs`.
* `m01_` menyimpan isi kedua berkas `.strings` dari **Isi berkas** sebagai literal teks di dalam uji, dan membandingkannya dengan isi `Output` hasil `plan`, bukan dengan berkas di disk. Berkas di disk dibandingkan oleh `committed_outputs_match_the_source` (spec `_root/0006`).

| Uji | Yang dibuktikan |
| --- | --- |
| `m01_real_catalog_writes_one_file_per_locale_after_the_schema` | `(SOURCE.plan)(&repo_root())` memberi path `shared/localization/catalog.schema.json`, lalu `en.lproj`, lalu `id.lproj` di bawah `RESOURCES_DIR`, dan isi kedua berkas `.strings` sama dengan **Isi berkas** |
| `m02_missing_translations_fall_back_and_are_marked` | Katalog K-2 spec `_root/0006`: tiga berkas (`en`, `id`, `ms`). Berkas `id` dan `ms` memuat `"greeting.submit" = "Greet";` dengan baris `/* Missing translation for id, shows the en text. */` dan `... for ms, ...`. Berkas `en` tidak memuat baris itu. |
| `m03_placeholders_become_positional` | Katalog K-1 spec `_root/0006`: berkas `en` memuat `"greeting.submit" = "%1$@, welcome to %2$@.";` dan berkas `id` memuat `"greeting.submit" = "Selamat datang di %2$@, %1$@.";` |
| `m04_special_characters_are_escaped` | `"Greet"` diganti `"100% \"done\" \\ *x* _y_ <b> & Łukasz 李"` (escape JSON): berkas `en` memuat `"greeting.submit" = "100%% \"done\" \\ *x* _y_ <b> & Łukasz 李";` |
| `m05_repeated_placeholder_keeps_its_number` | `"Greet"` menjadi `"{name} and {name}"` dan `"Sapa"` menjadi `"{name} dan {name}"`: berkas `en` memuat `"%1$@ and %1$@"` |
| `m06_locale_folders_use_catalog_names` | `edited("[\"en\", \"id\"]", "[\"en\", \"id\", \"pt-BR\"]")`, lalu pada hasilnya `"id": "Sapa"` menjadi `"id": "Sapa", "pt-BR": "Saudar"` (kemunculan pertama): path ketiga adalah `apps/macos/Presentation/Resources/pt-BR.lproj/Localizable.strings`, dan berkas itu memuat `"greeting.submit" = "Saudar";` |
| `m07_every_file_starts_with_the_marker_and_ends_with_one_newline` | Untuk setiap keluaran `outputs` dari katalog nyata, K-1, dan K-2: baris pertama tepat penanda di **Bentuk berkas `.strings`** butir 1, berkas berakhir `";\n"` dan tidak berakhir `"\n\n"`, dan tidak memuat `\r` |

**Key invariants**:

* Teks antarmuka macOS yang dilokalkan hanya datang dari resource hasil generator. Berkas `.strings` tidak disunting dengan tangan dan sama byte demi byte dengan hasil `localization generate`.
* Setiap kunci katalog ada di setiap berkas `.strings`, dan setiap bahasa katalog punya berkas (L-11 butir 1).
* Setiap kasus `TextKey` punya teks yang bukan kunci mentah dan bukan teks kosong di setiap bahasa bundel.
* Teks yang tampil sama dengan teks katalog: tidak ada pencarian kunci kedua oleh SwiftUI, tidak ada Markdown, dan `%` tampil sebagai `%`.
* Hasil `hello` tidak pernah lewat `LocalizedText` dan tetap `Text(verbatim:)`.
* Hanya `apps/macos/Interop/` yang mengimpor `DesktopMacosFFI` (spec `macos/0001`). `LocalizedText.swift` hanya mengimpor `Foundation`.

**Security model**:

Tidak ada autentikasi, izin, jaringan, atau data pengguna yang disimpan. Teks katalog adalah isi repo (spec `_root/0006`). Teks tidak pernah ditafsirkan sebagai Markdown, markup, atau pola format: emitter meloloskan `%` sehingga `String(format:)` tanpa argumen tidak bisa membaca argumen yang tidak ada, dan tombol menerima `String` biasa. Deskripsi hanya masuk komentar, dan polanya tidak bisa menutup komentar. Nama yang diketik pengguna tidak pernah masuk `LocalizedText`.

**Configuration required**:

Tidak ada. Tidak ada variabel lingkungan, pengaturan aplikasi, atau perubahan proyek Xcode. Bahasa mengikuti pengaturan macOS.

**Tata letak** (path yang ditambah atau diubah):

| Path | Jenis | Sumber isi |
| --- | --- | --- |
| `apps/macos/Presentation/Resources/en.lproj/Localizable.strings` | Baru, hasil generator | Emitter. Jangan disunting. |
| `apps/macos/Presentation/Resources/id.lproj/Localizable.strings` | Baru, hasil generator | Emitter. Jangan disunting. |
| `apps/macos/Presentation/LocalizedText.swift` | Baru | Bagian **Isi berkas** |
| `apps/macos/Tests/PresentationTests/LocalizedTextTests.swift` | Baru | Bagian **Isi berkas** |
| `apps/macos/Presentation/GreetingView.swift` | Diubah, satu baris | Bagian **Isi berkas** |
| `apps/macos/Package.swift` | Diubah, dua tambahan | Bagian **Isi berkas** |
| `tooling/codegen/src/localization/macos.rs` | Baru | Bagian **API surface** dan **Bentuk berkas `.strings`** |
| `tooling/codegen/src/localization/mod.rs` | Diubah: `pub mod macos;` dan isi `plan` | Bagian **API surface** |
| `tooling/codegen/tests/localization_macos_contract.rs` | Baru | Bagian **Uji emitter** |
| `.gitattributes` | Diubah, satu baris | Bagian **Isi berkas** |

Empat path baru di `apps/macos/` dihitung oleh **AC-10**. Folder `Resources` ini ada di bawah `Presentation/`, bukan folder `apps/macos/Resources/` di tingkat atas, yang dipakai spec `macos/0007` untuk ikon aplikasi.

**Kasus gagal yang sudah diputuskan**:

| Kejadian | Perilaku |
| --- | --- |
| Katalog diubah tanpa `generate` | `localization check` dan `cargo test --workspace --locked` gagal dan menyebut perintah `generate` (spec `_root/0003`). Katalog dan berkas `.strings` masuk commit yang sama. |
| Kunci baru di katalog, belum dipakai macOS | Berkas `.strings` memuatnya, `TextKey` tidak. Tidak ada yang gagal. |
| Kasus `TextKey` baru tanpa kunci katalog | `everyKeyHasTextInEveryLanguage` gagal (**AC-5**). Tambahkan kunci ke katalog lebih dulu. |
| Terjemahan hilang saat developer membangun dari working tree | Berkas bahasa itu memuat teks bahasa sumber dengan komentar `Missing translation` (L-9). `cargo test --workspace --locked` tetap gagal lewat uji katalog spec `_root/0006`. |
| Bahasa pengguna tidak didukung | Negosiasi Apple memberi `en`, jadi teks bahasa sumber tampil (**AC-4**). |
| Bundel resource tidak ditemukan saat berjalan | Akses `Bundle.module` menghentikan aplikasi dengan pesan dari SwiftPM. **AC-8** membuktikan bundel ada di aplikasi Debug dan Release. |
| Teks pertama yang butuh placeholder | Emitter sudah menulis `%<n>$@` (**AC-2**). Fungsi Swift yang mengisi nilai ditambahkan lewat pembaruan spec ini sebelum teks itu dipakai. |
| Konflik merge di berkas `.strings` | Ambil satu sisi, selesaikan konflik di `catalog.json`, jalankan `generate`, commit hasilnya. |
| Hook Graphify menolak berkas `.strings` atau berkas baru lain | Ikuti aturan spec `_root/0001`: entri sempit di `.graphifyignore`, catat di `verify.md` dan laporan. Jangan memakai `--no-verify`. |

**Critical test scenarios**:

* Jalur utama: katalog nyata lulus `check`, emitter menulis dua berkas yang sama dengan **Isi berkas**, dan aplikasi membawa keduanya, memverifikasi **AC-1**, **AC-2**, **AC-8**.
* Teks per bahasa: `en` dan `fr` memberi `Greet`, `id` dan `id-ID` memberi `Sapa`, memverifikasi **AC-4**.
* Tombol membaca kunci: tidak ada literal `"Greet"` di kode UI, memverifikasi **AC-3**.
* Kunci hilang: kasus `TextKey` tanpa teks membuat `swift test` gagal, memverifikasi **AC-5**.
* Teks biasa: `%`, `*`, `_`, `<`, dan `&` tampil apa adanya, dan tombol menerima `String`, memverifikasi **AC-6**.
* Hasil tetap: `Hello, Ayu!` dengan bahasa Indonesia, memverifikasi **AC-7**.
* Cakupan: hanya path di tabel milestone yang berubah, proyek Xcode tidak disentuh, memverifikasi **AC-9**, **AC-10**.
* Jendela nyata: bahasa sistem Indonesia menampilkan `Sapa`, memverifikasi **AC-11**.
* Akses: tidak ada peran atau izin di fitur ini. Yang ditolak adalah kunci yang tidak punya teks, memverifikasi **AC-5**.

**Rencana verifikasi manual**:

Langkah ini hanya dijalankan user pada sesi GUI. `/develop` menyalinnya ke `verify.md` di bawah `### UI / manual` pada milestone 2. Opsi `-AppleLanguages` mengganti bahasa untuk satu kali buka aplikasi saja, tanpa mengubah pengaturan sistem.

Prasyarat: Mac Apple Silicon dengan macOS 14 atau lebih baru (yang sudah diuji: 27.0.1), Xcode 27.1, sesi desktop yang sedang login, repo pada commit yang lulus **AC-8**, dan aplikasi `DesktopMac` tidak sedang berjalan.

1. Dari akar repo jalankan `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build`. Harapan: `** BUILD SUCCEEDED **`. (**AC-11**)
2. Jalankan `open -n apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app --args -AppleLanguages '(id)'`. Harapan: tombol bertuliskan `Sapa`, kolom tetap `Name`. Ketik `Ayu`, klik `Sapa`. Harapan: label `Hello, Ayu!`. Tekan Command Q. (**AC-11**)
3. Jalankan `open -n apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app --args -AppleLanguages '(id-ID)'`. Harapan: tombol `Sapa`. Tekan Command Q. (**AC-11**)
4. Jalankan `open -n apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app --args -AppleLanguages '(fr)'`. Harapan: tombol `Greet`. Tekan Command Q. (**AC-11**)
5. Buka System Settings, General, Language & Region. Tambahkan Bahasa Indonesia bila belum ada dan seret ke urutan pertama di Preferred Languages. Jalankan `open apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app` tanpa argumen. Harapan: tombol `Sapa`. Catat apakah menu aplikasi bawaan macOS ikut berbahasa Indonesia atau tetap bahasa Inggris. Keduanya tidak menggagalkan langkah ini. (**AC-11**)
6. Dengan bahasa Indonesia masih di urutan pertama, nyalakan VoiceOver dengan Command F5, lalu pindah ke tombol. Harapan: VoiceOver membacakan `Sapa`. Ketik `Ayu`, tekan Return, lalu pindah ke label. Harapan: VoiceOver membacakan `Hello, Ayu!`. Matikan VoiceOver dan tekan Command Q. (**AC-11**)
7. Kembalikan urutan bahasa seperti semula, buka aplikasi lagi. Harapan: tombol `Greet`. Tekan Command Q, lalu jalankan `git status --short`. Harapan: tidak ada keluaran. (**AC-11**)

Catat host, arsitektur, versi macOS, dan hasil tiap langkah. Langkah yang belum dijalankan tetap tercatat sebagai belum diverifikasi.

## Build plan

Pendekatan build proyek adalah Tracer Bullet. Benang tipisnya berjalan dari katalog, lewat emitter, ke resource yang dibaca `swift test`, lalu ke tombol di aplikasi yang dibangun `xcodebuild`. Sesudah itu bukti dikumpulkan, lalu langkah jendela dijalankan user. Satu langkah menjadi satu commit dengan subject yang memuat `spec macos/0006 step <n>` (dipakai **AC-10**). Hasil dicatat di `docs/specs/macos/0006-satu-contoh-lokalisasi/verify.md` di bagian `## Milestone <n>: <judul>` dengan judul persis seperti di bawah. Developer mengubah baris `**Status**` spec ini menjadi `In Progress` pada commit pertama. Jalankan `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0` sekali di shell yang sama sebelum semua perintah, supaya `cargo` dan fase skrip `xcodebuild` yang menjalankan `cargo` sama sama memakainya.

Prasyarat:

* needs pembaruan spec `macos/0001` yang menerima butir di Follow-up spec ini (AC-2, AC-3, AC-8, AC-12, tabel **Tata letak**, **Isi file** `GreetingView.swift` dan `Package.swift`, dan baris "Teks antarmuka"), lewat `architect (update)` gabungan lead.
* needs pembaruan AC-6 spec `macos/0005` bila spec itu dibangun sebelum atau bersamaan dengan spec ini (lihat Follow-up).
* Scope 010 bagian `_root` (spec `_root/0006`) sudah ada di `main`.

Berkas bersama dengan satu penulis per wave, per langkah: langkah 1 menyentuh `.gitattributes` dan `tooling/codegen/src/localization/mod.rs`. Langkah 2 menyentuh `apps/macos/Package.swift` dan `apps/macos/Presentation/GreetingView.swift`. Tidak ada langkah yang menyentuh `tooling/codegen/src/lib.rs` atau `apps/macos/DesktopMac.xcodeproj/project.pbxproj`. Build spec `macos/0005` juga mengubah `Package.swift` (baris target `Presentation`), `GreetingView.swift`, dan `.gitattributes`, dan build spec `macos/0004` serta `macos/0007` juga mengubah `.gitattributes`, jadi lead mengurutkan keempatnya. Baris `.gitattributes` keempat spec tidak saling tumpang tindih.

**Milestone 1: Resource dari katalog sampai tombol** (langkah 1 sampai 3)

1. Buat `tooling/codegen/src/localization/macos.rs`, ubah `localization/mod.rs` seperti di **API surface**, buat `tooling/codegen/tests/localization_macos_contract.rs` dengan uji `m01_` sampai `m07_`, dan tambah baris `.gitattributes`. Jalankan `cargo run -p desktop-codegen --locked -- localization generate`, cocokkan kedua berkas `.strings` dengan **Isi berkas**, lalu jalankan `cargo test -p desktop-codegen --locked`. Semua masuk satu commit, satisfies **AC-1**, **AC-2**
2. Ubah `apps/macos/Package.swift` dan `GreetingView.swift`, buat `LocalizedText.swift` dan `LocalizedTextTests.swift`, lalu jalankan `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin`, `swift test` seperti di **AC-4**, dan `xcodebuild` Debug serta Release seperti di **AC-8**, satisfies **AC-3**, **AC-4**, **AC-5**, **AC-6**, **AC-7**, **AC-8**
3. Pada working tree yang bersih, jalankan setiap perintah **AC-1** sampai **AC-10**, termasuk percobaan **AC-5** di salinan sekali pakai, catat perintah dan keluarannya di `verify.md` bagian `## Milestone 1: Resource dari katalog sampai tombol` bersama host dan arsitektur, lalu hapus `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/`, satisfies **AC-9**, **AC-10**

**Milestone 2: Bahasa di jendela nyata** (langkah 4, after milestone 1; `BLOCKED`, `owner: user`)

4. User menjalankan tujuh langkah di **Rencana verifikasi manual** pada sesi GUI dan mencatat hasilnya di `verify.md` bagian `## Milestone 2: Bahasa di jendela nyata`, di bawah `### UI / manual`. Sesi koordinasi tidak menjalankan langkah ini, satisfies **AC-11**

**File yang boleh diubah tiap milestone**:

| Milestone | File |
| --- | --- |
| 1 | Sepuluh path di tabel **Tata letak**, `verify.md` spec ini, baris `**Status**` di `index.md` spec ini, `.graphifyignore` bila hook menolak berkas baru, dan empat berkas `graphify-out/` dari hook |
| 2 | Hanya `verify.md` spec ini dan empat berkas `graphify-out/` dari hook |

`.gitattributes`, `apps/macos/Package.swift`, dan `tooling/codegen/src/localization/mod.rs` adalah berkas bersama, jadi langkah 1 dan 2 butuh izin eksplisit di brief developer.

## Consequences

**Positive**:

* Tombol sapaan macOS memakai kunci, teks, dan fallback yang sama dengan katalog bersama, tanpa terjemahan yang disalin tangan.
* Semua bukti kecuali langkah jendela berjalan tanpa layar: `cargo test` untuk emitter, `swift test` untuk teks per bahasa, dan `xcodebuild` untuk isi aplikasi.
* Proyek Xcode dan `lib.rs` tidak berubah, jadi spec ini tidak bersaing dengan build lain di dua berkas bersama itu.
* Kunci yang salah ketik di Swift gagal dikompilasi, dan kunci tanpa teks gagal di uji, jadi aplikasi tidak menampilkan kunci mentah.
* Format `.strings` adalah teks baris demi baris, jadi diff berkas hasil generator mudah dibaca saat tinjauan.

**Negative / tradeoffs**:

* `TextKey` menulis ulang kunci katalog di Swift. Satu uji menjaga keduanya tetap cocok, tetapi menambah teks berarti menyunting dua tempat.
* `.strings` adalah format lama Apple. Xcode modern mendorong String Catalog (`.xcstrings`), dan alat penerjemah Xcode tidak dipakai di sini.
* Bahasa aplikasi di mata macOS tetap bahasa Inggris, karena bundel utama aplikasi tidak memuat folder `.lproj`. Menu bawaan macOS dan dialog sistem bisa tetap bahasa Inggris saat tombol sudah `Sapa`. Langkah manual 5 mencatatnya.
* Aplikasi tidak muncul dengan pilihan bahasa per aplikasi di System Settings, karena alasan yang sama.
* Layar menjadi setengah terjemahan: `Name` dan `Hello, Ayu!` tetap bahasa Inggris (konsekuensi spec `_root/0006`).
* Fungsi Swift untuk placeholder belum ada. Teks berplaceholder pertama menunggu pembaruan spec ini.
* Angka di beberapa AC spec `macos/0001` dan `macos/0005` berubah karena spec ini, dan perubahan itu menunggu pembaruan gabungan lead sebelum build.

**Neutral**:

* `Bundle.module` dipakai di aplikasi dan di `swift test`. Xcode menyalin bundel `DesktopMacModules_Presentation.bundle` ke `Contents/Resources` aplikasi.
* Aturan ganti `.` dengan `_` di L-11 butir 2 tidak dipakai di macOS, karena `.strings` menerima titik.
* Ringkasan `localization check` naik dari `1 outputs` ke `3 outputs`. AC-2 spec `_root/0006` sudah berbatas "pada akhir milestone 1", AC-9 spec itu belum (lihat Follow-up).

## Follow-up

Butir untuk pembaruan gabungan spec `macos/0001` (lead mengumpulkan butir dari spec `macos/0004`, `macos/0005`, `macos/0006`, dan spec macOS scope 011). Teks yang diusulkan:

* [x] AC-12 spec `macos/0001`: ganti "mencetak tepat 16 path dari tabel **Tata letak**, jadi tidak ada folder `Platform`, `DesignSystem`, atau `Resources`" dengan "pada akhir milestone 3 spec ini mencetak tepat 16 path dari tabel **Tata letak**. Spec berikutnya menambah path lewat tabel **Tata letak** miliknya sendiri, dan tidak ada folder `apps/macos/Platform/`". Folder `apps/macos/Resources/` di tingkat atas tidak lagi dilarang, karena spec macOS scope 011 (`macos/0007`) membuatnya untuk ikon aplikasi. Spec `macos/0006` menambah tepat empat path (dua berkas `.strings` di `apps/macos/Presentation/Resources/`, `apps/macos/Presentation/LocalizedText.swift`, dan `apps/macos/Tests/PresentationTests/LocalizedTextTests.swift`) dan tidak memakai `apps/macos/Resources/`. Angka gabungan: 16 + 1 (`macos/0004`) + 3 (`macos/0005`) + 4 (`macos/0006`) + jumlah path `macos/0007`, bila semuanya dibangun; lead menetapkan angka akhirnya. Diterapkan di pembaruan gabungan spec `macos/0001` (`12c98a0`, 2026-11-15).
* [x] AC-8 spec `macos/0001`: ganti "`git grep --untracked -lF 'Hello, ' -- '*.swift'` mencetak tepat dua baris, keduanya di bawah `apps/macos/Tests/`" dengan "setiap baris dari `git grep --untracked -lF 'Hello, ' -- '*.swift'` berada di bawah `apps/macos/Tests/`; pada akhir milestone 2 spec ini jumlahnya tepat dua". Spec `macos/0006` menambah satu berkas (`LocalizedTextTests.swift`). Diterapkan di pembaruan gabungan spec `macos/0001` (`12c98a0`, 2026-11-15).
* [x] AC-3 spec `macos/0001`: ganti "Keluaran perintah `swift test` yang sama memuat `Test run with 4 tests in 1 suite passed`, yaitu empat uji suite `GreetingViewModelTests`" dengan "keluarannya memuat `✔ Suite GreetingViewModelTests passed`, dan pada akhir milestone 1 spec ini juga `Test run with 4 tests in 1 suite passed`". Sesudah spec `macos/0006`, target `PresentationTests` mencetak `Test run with 10 tests in 2 suites passed`. Diterapkan di pembaruan gabungan spec `macos/0001` (`12c98a0`, 2026-11-15).
* [x] AC-2 spec `macos/0001`: tambahkan "pada akhir milestone 1 spec ini" pada `Test run with 7 tests in 1 suite passed`. Spec `macos/0006` tidak mengubah angka itu, tetapi spec lain yang menambah target uji bisa mengubahnya. Diterapkan di pembaruan gabungan spec `macos/0001` (`12c98a0`, 2026-11-15).
* [x] Tabel **Tata letak** spec `macos/0001`: tambahkan empat baris dari tabel **Tata letak** spec ini, dengan sumber "Spec `macos/0006`". Kalimat "Folder `Platform`, `DesignSystem`, dan `Resources` dari rujukan macOS belum dibuat" mengikuti spec yang membuat folder itu: `DesignSystem` dari spec `macos/0005`, `Resources` di tingkat atas dari spec `macos/0007`, dan `Platform` tetap belum dibuat. Diterapkan di pembaruan gabungan spec `macos/0001` (`12c98a0`, 2026-11-15).
* [x] **Isi file** spec `macos/0001`: blok `GreetingView.swift` dan `apps/macos/Package.swift` tidak lagi menjadi isi akhir. Tulis catatan bahwa spec `macos/0004`, `macos/0005`, dan `macos/0006` mengubah berkas itu, dengan perubahan masing masing di spec pemiliknya. Diterapkan di pembaruan gabungan spec `macos/0001` (`12c98a0`, 2026-11-15).
* [x] Tabel keputusan spec `macos/0001`, baris "Teks antarmuka": ganti dengan "`Name` tetap literal bahasa Inggris. Label tombol dibaca dari katalog lewat kunci `greeting.submit` (spec `macos/0006`). Judul jendela dari bundel (spec `macos/0004`). Hasil sapaan memakai `Text(verbatim:)`". Diterapkan di pembaruan gabungan spec `macos/0001` (`12c98a0`, 2026-11-15).
* [x] Follow-up "Scope 010" spec `macos/0001`: tandai `[x]` dengan "Diterapkan di spec `macos/0006` (2026-11-15): hanya `Greet` yang pindah, sebagai `greeting.submit`. `Name` tetap literal. `Desktop Foundation` milik spec `macos/0004`". Diterapkan di pembaruan gabungan spec `macos/0001` (`12c98a0`, 2026-11-15).
* [x] Rencana verifikasi manual spec `macos/0001` langkah 2, 3, 6, dan 7: tambahkan "dengan bahasa sistem Inggris" pada prasyarat, karena tombol `Greet` sekarang bergantung pada bahasa. Diterapkan di pembaruan gabungan spec `macos/0001` (`12c98a0`, 2026-11-15).

Butir untuk spec lain:

* [x] Spec `macos/0005` (scope 008 macOS): diterapkan di pembaruan gabungan (`12c98a0`, 2026-11-15) dengan pola yang menerima kedua bentuk tombol. AC-6 mengunci `grep -c 'Button("Greet")' apps/macos/Presentation/GreetingView.swift` = `1`. Sesudah spec ini, ganti dengan `grep -c 'Button(LocalizedText.string(.greetingSubmit))' apps/macos/Presentation/GreetingView.swift` = `1`. Isi berkas utuh `GreetingView.swift` di spec itu juga memakai baris tombol yang baru bila spec ini masuk lebih dulu. Pemeriksaan lain spec itu atas `GreetingView.swift` (`grep -cE '[0-9]'` = `1`, `grep -c 'Tokens\.'` = `6`, dan pola larangan AC-3) tetap lulus, karena baris baru tidak memuat angka, `Tokens.`, atau pola itu. `LocalizedText.swift` juga tidak memuat pola larangan itu.
* [x] Spec `_root/0006`: diterapkan di pembaruan spec `_root/0006` (`85859e7`, 2026-11-15); AC-9 kini dibatasi akhir milestone. AC-9 menulis `1 outputs, 0 written` dan `1 outputs, 1 written` tanpa batas waktu. Tambahkan "pada akhir milestone 1", seperti AC-2 spec itu, karena sesudah spec ini angkanya `3 outputs`. Tandai Follow-up "macOS (scope 010 ...)" dan baris pertama Follow-up (hanya `Greet` pindah) sebagai dijawab spec `macos/0006`.
* [ ] Spec `macos/0004`: tidak ada perubahan. Judul jendela tetap dari bundel, bukan katalog, sejalan dengan L-1.
* [ ] Lead: tambahkan tautan balik ke spec ini di berkas scope 010.
* [ ] Lead: `docs/platforms/macos/README.md` menyebut folder `Resources` untuk sumber daya native. Sesudah build, catat bahwa resource bahasa tinggal di `apps/macos/Presentation/Resources/` sebagai resource paket Swift.

Lainnya:

* [ ] Placeholder di Swift: saat katalog mendapat teks berplaceholder pertama, tambahkan `LocalizedText.string(_ key: TextKey, _ values: String...)` yang mengisi `%<n>$@` dengan `String(format:arguments:)`, beserta uji, lewat pembaruan spec ini.
* [ ] Bahasa bundel utama: bila langkah manual 5 menunjukkan menu macOS tetap bahasa Inggris dan itu tidak diterima, tambahkan folder `.lproj` kosong per bahasa ke target aplikasi atau `CFBundleLocalizations` ke Info.plist lewat pembaruan spec ini. Itu mengubah `project.pbxproj`.
* [ ] Scope 002: jadikan pemeriksaan tetap bahwa kode UI di `apps/macos/Presentation/` dan `apps/macos/App/` tidak memakai `LocalizedStringKey`, `String(localized:)`, `NSLocalizedString`, atau literal teks tombol yang punya kunci katalog (L-1).
* [x] Scope 012: skrip build macOS menjalankan `localization check` sebelum `xcodebuild`. Dijawab [spec `_root/0008`](../../_root/0008-build-dan-penerimaan-per-target/index.md): langkah 4 `build-macos.sh` (2026-11-15).
* [ ] Windows dan Linux (scope 010, menunggu host): tidak ada spec, kode, atau bukti di sini. OS-1 sampai OS-9 untuk kedua host ditulis di spec `windows/` dan `linux/` saat host tersedia.
