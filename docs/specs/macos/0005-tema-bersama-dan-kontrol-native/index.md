# 0005. Tema bersama dan kontrol native di macOS

**Scope**: [008 - Tema bersama dan kontrol native](../../../scope/008%20-%20Tema%20bersama%20dan%20kontrol%20native.md)
**Date**: 2026-11-15
**Status**: Proposed

## Summary

Spec ini membuat jendela sapaan di Mac memakai token dari sumber bersama. Alat `desktop-codegen` mendapat satu emitter (penulis file untuk satu target) yang menulis `apps/macos/DesignSystem/Tokens.swift`, dan modul Swift baru `DesignSystem` mengubah nilai itu menjadi warna yang ikut tampilan terang dan gelap, huruf dari gaya teks sistem, dan jarak. `GreetingView` lalu mengambil warna, huruf, dan jarak hanya dari token, sementara kolom teks, tombol, dan label tetap kontrol asli macOS. Hampir semuanya terbukti tanpa jendela lewat `check`, `cargo test`, `swift test`, `xcodebuild`, dan pemeriksaan teks. Yang harus kamu lihat sendiri di jendela nyata hanya tampilan terang dan gelap, fokus keyboard, ukuran teks, dan VoiceOver. Keputusan tentang arti token ada di [spec `_root/0003`](../../_root/0003-tema-bersama-dan-kontrol-native/index.md) dan tidak diputuskan ulang di sini.

## Requirements

**User stories**:

* Sebagai pengguna Mac, saya ingin jendela sapaan ikut tampilan terang atau gelap yang saya pilih di sistem, dengan kontrol yang terasa seperti kontrol Mac lain.
* Sebagai developer aplikasi macOS, saya ingin mengubah warna atau jarak di satu file token lalu melihatnya di Swift tanpa menyalin angka.
* Sebagai pemeriksa, saya ingin bukti tanpa jendela bahwa jendela sapaan hanya memakai token, dan daftar pendek hal yang harus dilihat mata.

**Acceptance criteria**:

Angka kriteria merujuk tabel di [Penerimaan dan bukti fondasi](../../../quality/README.md). K-1 sampai K-9 dan O-1 sampai O-7 adalah milik spec `_root/0003`. Semua perintah dijalankan dari akar repo pada Mac Apple Silicon. Bukti setiap AC dicatat bersama host, arsitektur, dan versi dari **AC-11**; langkah manual juga mencatat versi macOS dan tampilan yang sedang aktif.

* **AC-1** (kriteria 16, Done when "token yang bisa dihasilkan ulang", K-1, K-8): `cargo run -p desktop-codegen --locked -- tokens check` keluar dengan kode 0 dan baris terakhirnya `tokens: ok, 6 semantic tokens, 3 placeholder values, 4 outputs up to date`. Pada working tree yang bersih, `cargo run -p desktop-codegen --locked -- tokens generate` mencetak baris terakhir yang berakhir `4 outputs, 0 written`, lalu `git diff --exit-code -- apps/macos/DesignSystem design-system` keluar dengan kode 0. Sesudah `rm apps/macos/DesignSystem/Tokens.swift`, perintah `generate` mencetak `wrote apps/macos/DesignSystem/Tokens.swift` dan `git status --short` tidak mencetak apa pun. Isi `Tokens.swift` sama persis dengan blok di bagian **Isi file**: `awk '/^`apps\/macos\/DesignSystem\/Tokens.swift` adalah hasil/{s=1} s&&/^```swift$/{f=1;next} f&&/^```$/{exit} f' docs/specs/macos/0005-tema-bersama-dan-kontrol-native/index.md | diff - apps/macos/DesignSystem/Tokens.swift` tidak mencetak apa pun. `git ls-files --eol apps/macos/DesignSystem/Tokens.swift` mencetak satu baris yang memuat `i/lf`, `w/lf`, dan `attr/text eol=lf`. Dicatat dengan host dari AC-11.
* **AC-2** (kriteria 16, K-2, K-9): emitter memetakan setiap jenis token. `cargo test -p desktop-codegen --locked` lulus; berkas uji `tokens_contract` mencetak `5 passed`, `tokens_rules` mencetak `17 passed`, dan `tokens_swift_contract` mencetak `2 passed`. `grep -c '_ =>' tooling/codegen/src/tokens/swift.rs` mencetak `0`, jadi peran atau bobot baru gagal dikompilasi sampai dipetakan. `grep -c 'Placeholder value' apps/macos/DesignSystem/Tokens.swift` mencetak `3`. Dicatat dengan host dari AC-11.
* **AC-3** (kriteria 16 di aplikasi, K-3, K-4): jendela sapaan hanya memakai token, dan setiap token dipakai. `grep -c 'Tokens\.' apps/macos/Presentation/GreetingView.swift` mencetak `6`. Untuk tiap nama `Tokens.Colors.accent`, `Tokens.Colors.background`, `Tokens.Colors.textPrimary`, `Tokens.Typography.body`, `Tokens.Spacing.gapControl`, dan `Tokens.Spacing.insetWindow`, `grep -c` pada file itu mencetak `1`. `grep -c 'public static let' apps/macos/DesignSystem/Tokens.swift` mencetak `6`. `grep -rnE 'spacing: [0-9]|padding\([0-9]|padding\(\.[a-z]+, *[0-9]|minLength: *[0-9]|Color\(|Color\.|NSColor|Font\.|\.font\(\.|fontWeight|\.bold\(|\.foregroundStyle\(\.|\.foregroundColor|\.tint\(\.|\.background\(\.|size:' apps/macos/Presentation apps/macos/App` tidak mencetak apa pun, untuk `DesktopMacApp.swift` versi spec `macos/0001` maupun versi spec `macos/0004`. `grep -rl 'import DesignSystem' apps/macos/App apps/macos/Presentation apps/macos/Interop apps/macos/DesignSystem apps/macos/Tests` mencetak tepat dua baris: `apps/macos/Presentation/GreetingView.swift` dan `apps/macos/Tests/DesignSystemTests/ThemeTests.swift`. Satu satunya angka yang tersisa di `GreetingView.swift` adalah `minWidth: 320`, konstanta milik macOS (K-3): `grep -cE '[0-9]' apps/macos/Presentation/GreetingView.swift` mencetak `1`. Dicatat dengan host dari AC-11.
* **AC-4** (kriteria 17 tanpa jendela, K-5): warna ikut tampilan sistem, dan itu terbukti tanpa membuka jendela. Sesudah `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin`, perintah `swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"` keluar dengan kode 0. Keluarannya memuat `Test run with 5 tests in 1 suite passed` (suite `ThemeTests`) dan tetap memuat `✔ Suite BindingContractTests passed` serta `✔ Suite GreetingViewModelTests passed` dari spec `macos/0001`. Pada akhir milestone 2 spec ini keluarannya juga memuat `Test run with 7 tests in 1 suite passed` dan `Test run with 4 tests in 1 suite passed`; spec `macos/0006` mengubah angka target `PresentationTests`. `grep -rnE 'preferredColorScheme|colorScheme|NSApp\.appearance|\.appearance = ' apps/macos/App apps/macos/Presentation apps/macos/DesignSystem apps/macos/Interop` tidak mencetak apa pun, jadi tidak ada tombol tema dan tidak ada tampilan yang dipaksa. Dicatat dengan host dari AC-11.
* **AC-5** (kriteria 16 dan 17, build): `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build` mencetak `** BUILD SUCCEEDED **`, begitu juga dengan `-configuration Release`. Untuk kedua konfigurasi, `lipo -archs apps/macos/DerivedData/Build/Products/<K>/DesktopMac.app/Contents/MacOS/DesktopMac` mencetak `arm64`. Sesudah `rm -rf apps/macos/.build`, supaya semua modul dikompilasi ulang, `swift build --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug" 2>&1 | grep -ci warning` mencetak `0`. Dicatat dengan host dari AC-11.
* **AC-6** (Done when "kontrol mempertahankan fokus, penskalaan, dan aksesibilitas native", bagian tanpa jendela, K-6): kontrol tetap kontrol asli. `grep -c 'TextField("Name", text: \$model.name)' apps/macos/Presentation/GreetingView.swift`, `grep -cE 'Button\("Greet"\)|Button\(LocalizedText\.string\(\.greetingSubmit\)\)' ...` (bentuk tombol sebelum atau sesudah spec `macos/0006`), `grep -c 'Text(verbatim: model.greeting' ...`, `grep -c '.keyboardShortcut(.defaultAction)' ...`, dan `grep -c '.buttonStyle(.borderedProminent)' ...` masing masing mencetak `1`. `grep -rnE 'focusable|focusEffectDisabled|focusRing|controlSize|frame\((min|max)?[Hh]eight|accessibilityHidden|\.disabled\(|ButtonStyle \{|TextFieldStyle \{' apps/macos/Presentation apps/macos/App` tidak mencetak apa pun. `grep -c 'Font.system(style)' apps/macos/DesignSystem/Theme.swift` mencetak `1` dan `grep -c 'size' apps/macos/DesignSystem/Tokens.swift` mencetak `0`, jadi ukuran huruf selalu datang dari gaya teks sistem. Dicatat dengan host dari AC-11.
* **AC-7** (kriteria 17, Done when "tampilan terang dan gelap memakai token", K-5; langkah manual, `owner: user`): pada sesi GUI, langkah 1 sampai 5 di bagian **Rencana verifikasi manual** memberi hasil yang tertulis. Di tampilan terang latar jendela `#FAFAFA` dan teks hasil `#111111`; di tampilan gelap sebaliknya; tombol `Greet` berisi `#3366CC` di keduanya; pergantian tampilan terjadi tanpa menutup aplikasi. Hasil mencatat host, arsitektur, versi macOS, dan tampilan yang aktif di tiap langkah.
* **AC-8** (Done when "kontrol mempertahankan fokus, penskalaan, dan aksesibilitas native", K-7; langkah manual, `owner: user`): pada sesi GUI, langkah 6 sampai 10 di bagian **Rencana verifikasi manual** memberi hasil yang tertulis: cincin fokus sistem terlihat, Return dan Spasi menjalankan `Greet`, teks tidak terpotong saat ukuran teks sistem diperbesar, VoiceOver membacakan kolom, tombol, dan hasil, dan jendela tetap terbaca dengan Increase contrast. Hasil mencatat host, arsitektur, versi macOS, dan tampilan yang aktif.
* **AC-9** (kriteria 32): fitur ini hanya menambah yang dibutuhkan. Fitur ini menambah tepat tiga path: `git log --diff-filter=A --name-only --format= --grep='spec macos/0005 step' -- apps/macos bindings/swift crates/macos-ffi | sort -u` mencetak tepat tiga path: `apps/macos/DesignSystem/Theme.swift`, `apps/macos/DesignSystem/Tokens.swift`, dan `apps/macos/Tests/DesignSystemTests/ThemeTests.swift`. Pemeriksaan ini hanya membaca commit fitur ini, jadi tetap benar sesudah spec macOS lain menambah path. `git ls-files apps/macos | grep -c '^apps/macos/Platform/'` mencetak `0`, dan fitur ini tidak menambah apa pun di bawah `apps/macos/Resources/` (milik spec `macos/0007`). Pemeriksaan `git log` AC ini dijalankan lagi sesudah commit terakhir fitur (langkah 4) atau saat merge. `grep -c '^/apps/macos/DesignSystem/Tokens.swift text eol=lf$' .gitattributes` mencetak `1`. Setiap commit fitur ini memakai subject yang memuat `spec macos/0005 step <n>`, dan `git log --name-only --format= --grep='spec macos/0005 step' | sort -u` hanya mencetak path dari tabel **File yang boleh diubah tiap milestone**. Daftar itu tidak memuat path di bawah `apps/macos/DesktopMac.xcodeproj/`, `apps/macos/App/`, `bindings/`, `crates/`, `design-system/`, `scripts/`, atau `.github/`. Dicatat dengan host dari AC-11.
* **AC-10** (kriteria 16 lewat aturan spec `_root/0001` dan `_root/0003`): workspace tetap hijau. `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, dan `cargo clippy --workspace --all-targets --locked -- -D warnings` keluar dengan kode 0. `cargo test -p desktop-codegen --locked --test macos_bridge_ownership` lulus, jadi file Swift baru tidak memakai pointer. `cargo tree -p desktop-codegen --depth 1 --prefix none --locked` tetap mencetak tepat tiga baris. Dicatat dengan host dari AC-11.
* **AC-11** (bukti Kompatibilitas di [rujukan macOS](../../../platforms/macos/README.md)): kombinasi yang dipakai tercatat bersama keluaran perintahnya: `sw_vers -productVersion`, `uname -m`, `xcodebuild -version`, `swift --version`, `xcrun --sdk macosx --show-sdk-version`, dan `rustc --version`. Pada Mac koordinasi nilainya `27.0.1`, `arm64`, `Xcode 27.1` build `27A9275`, Swift `6.4`, SDK `27.0`, dan `rustc 1.99.1`, sama dengan tabel kombinasi spec `macos/0001`. Bila host lain dipakai, nilainya dicatat apa adanya dan AC lain dinilai pada host itu.

## Decision

**Chosen option**: Opsi 1: warna dinamis di kode, modul `DesignSystem` di paket Swift yang sudah ada, token diterapkan di `GreetingView`.

Emitter menulis satu file Swift berisi konstanta. Dua tipe kecil tulisan tangan di modul yang sama mengubahnya menjadi nilai SwiftUI: `ThemeColor` menjadi warna yang memilih nilai terang atau gelap menurut tampilan yang sedang menggambar, dan `TextRole` menjadi huruf dari gaya teks sistem. `GreetingView` memakai keduanya dan tidak menyimpan angka sendiri selain lebar minimum jendela.

Keputusan turunannya:

| Hal | Keputusan |
| --- | --- |
| Emitter | Modul publik `tooling/codegen/src/tokens/swift.rs`, fungsi `output(&TokenSet) -> Output`, dipanggil dari `tokens::SOURCE.plan` sesudah keluaran skema. |
| Keluaran | Satu file: `apps/macos/DesignSystem/Tokens.swift`, dilacak git, tidak disunting tangan. |
| Bentuk keluaran | `public enum Tokens` dengan tiga ruang nama: `Colors`, `Typography`, `Spacing`. Satu `public static let` per token semantik. |
| Nama Swift | Nama token tanpa titik, huruf pertama tiap bagian sesudah titik dijadikan huruf besar: `text.primary` menjadi `textPrimary`. Nama yang sama dengan kata kunci Swift diberi backtick. |
| Warna | `ThemeColor(light: 0xRRGGBB, dark: 0xRRGGBB)`. Angka heksadesimal disalin dari sumber (O-5). |
| Warna di layar | `NSColor` dinamis (`NSColor(name:dynamicProvider:)`) dibungkus `Color(nsColor:)`. Tanpa asset catalog. |
| Tipografi | `body` menjadi `Font.TextStyle.body`; `regular` menjadi `.regular`, `semibold` menjadi `.semibold`. Huruf dibuat dengan `Font.system(style).weight(weight)`. |
| Jarak | `CGFloat` dalam point. |
| `accent` | `.tint(...)` pada tombol `Greet`, dengan `.buttonStyle(.borderedProminent)`. Cincin fokus dan warna pilihan teks tetap mengikuti aksen sistem. |
| `background` | `.background(...)` pada isi jendela. Bilah judul tetap milik sistem. |
| `text.primary` dan `body` | `.foregroundStyle(...)` dan `.font(...)` pada label hasil. Teks di dalam kolom dan tombol tetap diatur kontrolnya. |
| Kontras tinggi | Increase contrast di macOS tidak mengganti palet, jadi token tetap dipakai. Tampilan kontras tinggi dipetakan ke nilai terang atau gelap yang sama. |
| Modul | Target `DesignSystem` di `apps/macos/Package.swift`, folder `apps/macos/DesignSystem/`. Hanya `Presentation` yang mengimpornya. |
| Uji | Swift Testing, target `DesignSystemTests`, tanpa jendela. Uji emitter di `tooling/codegen/tests/tokens_swift_contract.rs`. |
| Proyek Xcode | Tidak berubah. `DesignSystem` masuk lewat `Presentation`. `DesktopMacApp.swift` juga tidak berubah. |
| Akhir baris | Satu baris baru di `.gitattributes`: `/apps/macos/DesignSystem/Tokens.swift text eol=lf`. |

Nilai dari spec lain yang dipakai di sini, tanpa diputuskan ulang:

| Nilai | Isi | Pemilik |
| --- | --- | --- |
| Token dan artinya | `accent`, `background`, `text.primary`, `body`, `gap.control`, `inset.window`, dan nilai sementaranya | Spec `_root/0003` |
| Alat | Paket `desktop-codegen`, perintah `tokens check` dan `tokens generate`, tipe `TokenSet`, `Output`, aturan O-1 sampai O-7 | Spec `_root/0003` |
| Kewajiban spec OS | K-1 sampai K-9 | Spec `_root/0003` |
| Paket Swift, modul `Interop` dan `Presentation`, proyek Xcode, versi minimum macOS `14.0`, arsitektur `arm64` | Tata letak dan isi `apps/macos/` | Spec `macos/0001` |
| Perintah `swift test` dan `xcodebuild` | Bagian "Perintah build dan uji" | Spec `macos/0001` |
| `DesktopMacApp.swift`, `project.pbxproj`, `Config/Shared.xcconfig`, sumber `config` di alat | Judul jendela dan setelan bundel dari konfigurasi publik. Tidak diubah spec ini. | Spec `macos/0004` |

## Rationale

Alasan, opsi yang ditimbang, daftar keputusan beserta pilihan keduanya, dan bukti percobaan ada di [rationale.md](rationale.md).

## Feature design

**Data model sketch**:

Tidak ada data yang disimpan. Nilai yang hidup di aplikasi adalah konstanta di `Tokens.swift`:

| Token bersama | Konstanta Swift | Tipe | Nilai sekarang |
| --- | --- | --- | --- |
| `accent` | `Tokens.Colors.accent` | `ThemeColor` | terang `0x3366CC`, gelap `0x3366CC`, sementara |
| `background` | `Tokens.Colors.background` | `ThemeColor` | terang `0xFAFAFA`, gelap `0x111111`, sementara |
| `text.primary` | `Tokens.Colors.textPrimary` | `ThemeColor` | terang `0x111111`, gelap `0xFAFAFA`, sementara |
| `body` | `Tokens.Typography.body` | `TextRole` | gaya `.body`, bobot `.regular` |
| `gap.control` | `Tokens.Spacing.gapControl` | `CGFloat` | `12` |
| `inset.window` | `Tokens.Spacing.insetWindow` | `CGFloat` | `20` |

Pemetaan peran dan bobot (K-2):

| Di sumber | Di Swift |
| --- | --- |
| Peran `body` | `Font.TextStyle.body` |
| Bobot `regular` | `Font.Weight.regular` |
| Bobot `semibold` | `Font.Weight.semibold` |

**State transitions**:

Tidak ada. Warna berganti saat sistem berganti tampilan, tanpa keadaan di aplikasi: `NSColor` dinamis memilih nilainya setiap kali digambar.

**API surface**:

| Permukaan | Tanda tangan | Masukan | Hasil | Akses | Kesalahan |
| --- | --- | --- | --- | --- | --- |
| Rust, `desktop_codegen::tokens::swift::output` | `pub fn output(set: &TokenSet) -> Output` | Token yang sudah diurai | Satu `Output` dengan `path` `apps/macos/DesignSystem/Tokens.swift` | Publik | Tidak ada. Setiap `TokenSet` yang lolos `parse` bisa ditulis. |
| Rust, `desktop_codegen::tokens::swift::PATH` | `pub const PATH: &str` | | `apps/macos/DesignSystem/Tokens.swift` | Publik | |
| Swift, `Tokens` (hasil generator) | `public enum Tokens` dengan `Colors`, `Typography`, `Spacing` | | Konstanta di tabel data | Publik di modul `DesignSystem` | |
| Swift, `ThemeColor` | `init(light: UInt32, dark: UInt32)`, `func channels(dark: Bool) -> [UInt8]`, `var nsColor: NSColor`, `var color: Color` | Dua nilai `0xRRGGBB` | Warna yang ikut tampilan | Publik, `Sendable`, `Equatable` | Tidak ada |
| Swift, `TextRole` | `init(style: Font.TextStyle, weight: Font.Weight)`, `var font: Font` | Gaya teks dan bobot | `Font.system(style).weight(weight)` | Publik, `Sendable`, `Equatable` | Tidak ada |
| Swift, `GreetingView` | Tidak berubah: `GreetingView(model:)` | | Tampilan yang sama, kini memakai token | Publik | Tidak ada |

Alur nilai: `design-system/tokens/*.json` → `desktop-codegen` (`parse`, lalu `swift::output`) → `Tokens.swift` → `ThemeColor.color` dan `TextRole.font` → pengubah tampilan di `GreetingView`.

**Value sourcing**:

| Aksi | Nilai yang dihasilkan atau ditampilkan | Sumber |
| --- | --- | --- |
| `swift::output` | Nama konstanta | Nama token di `TokenSet`, lewat aturan nama di tabel keputusan |
| `swift::output` | `0xRRGGBB` terang dan gelap | `Color.light` dan `Color.dark` di `TokenSet` |
| `swift::output` | Komentar dokumen tiap konstanta | `description` token; ditambah ` Placeholder value.` bila `Color.placeholder` benar |
| `swift::output` | Gaya teks dan bobot | `Role.name` dan `Role.weight`, lewat tabel pemetaan |
| `swift::output` | Nilai jarak | `Space.value` |
| `swift::output` | Path keluaran dan teks penanda | Konstanta `PATH` dan teks tetap di `swift.rs` |
| Jendela | Warna latar | `Tokens.Colors.background.color` |
| Jendela | Warna dan huruf label hasil | `Tokens.Colors.textPrimary.color` dan `Tokens.Typography.body.font` |
| Jendela | Warna isi tombol `Greet` | `Tokens.Colors.accent.color` lewat `.tint` |
| Jendela | Jarak antar kontrol dan jarak tepi | `Tokens.Spacing.gapControl` dan `Tokens.Spacing.insetWindow` |
| Jendela | Lebar minimum `320` | Literal di `GreetingView.swift`, konstanta milik macOS (K-3, spec `macos/0001`) |
| Jendela | Tampilan terang atau gelap | Pengaturan sistem, dibaca AppKit saat menggambar |
| Jendela | Warna teks di tombol, cincin fokus, latar kolom teks | Sistem. Bukan token. |
| Langkah manual | Nilai warna yang diharapkan | `apps/macos/DesignSystem/Tokens.swift` pada commit yang diuji. Bila berbeda dari angka di spec ini, file itu yang dipakai. |

**Key invariants**:

* `Tokens.swift` selalu sama byte demi byte dengan hasil `tokens generate` untuk commit itu.
* Kode tampilan di `apps/macos/Presentation/` dan `apps/macos/App/` tidak memuat warna, bobot huruf, atau jarak literal. Pengecualiannya hanya `minWidth: 320`.
* Hanya `Presentation` yang mengimpor `DesignSystem`. `DesignSystem` tidak mengimpor `Interop`, `Presentation`, atau `DesktopMacosFFI`.
* Aplikasi tidak pernah memaksa tampilan terang atau gelap.
* Ukuran huruf selalu berasal dari `Font.TextStyle`, tidak pernah dari angka.
* `ThemeColor` dan `TextRole` tidak menyimpan keadaan dan aman dipakai dari thread mana pun (`Sendable`).

**Security model**:

Tidak ada autentikasi, peran, jaringan, penyimpanan, atau data pribadi baru. Token adalah nilai publik. Teks dari sumber token yang masuk ke `Tokens.swift` hanya nama dan deskripsi, dan keduanya sudah dibatasi ke karakter aman oleh aturan T-3 dan T-4 spec `_root/0003`, jadi tidak bisa keluar dari komentar atau nama konstanta. Modul `DesignSystem` tidak memakai pointer dan tidak membaca file saat berjalan.

**Configuration required**:

Tidak ada.

**Tata letak**:

| Path | Jenis | Sumber isi |
| --- | --- | --- |
| `tooling/codegen/src/tokens/swift.rs` | Baru | Bagian **Isi file** |
| `tooling/codegen/tests/tokens_swift_contract.rs` | Baru | Bagian **Isi file** |
| `tooling/codegen/src/tokens/mod.rs` | Diubah: satu baris, isi `Plan`, dan komentar dokumen `plan` | Bagian **Isi file** |
| `tooling/codegen/src/lib.rs` | Diubah: satu baris komentar dokumen | Bagian **Isi file** |
| `tooling/codegen/tests/tokens_rules.rs` | Diubah: uji `outputs_live_under_design_system_with_lf_attributes` | Bagian **Isi file** |
| `tooling/codegen/tests/tokens_contract.rs` | Diubah, satu konstanta dan enam baris | Bagian **Isi file** |
| `.gitattributes` | Diubah, satu baris | Bagian **Isi file** |
| `apps/macos/DesignSystem/Tokens.swift` | Baru, hasil generator. Jangan disunting. | Generator; isi yang diharapkan di **Isi file** |
| `apps/macos/DesignSystem/Theme.swift` | Baru | Bagian **Isi file** |
| `apps/macos/Tests/DesignSystemTests/ThemeTests.swift` | Baru | Bagian **Isi file** |
| `apps/macos/Package.swift` | Diubah | Bagian **Isi file** |
| `apps/macos/Presentation/GreetingView.swift` | Diubah | Bagian **Isi file** |

Peran bagian baru di `apps/macos/`:

| Bagian | Jenis | Isi | Boleh mengimpor |
| --- | --- | --- | --- |
| `DesignSystem/` | Modul paket Swift | `Tokens` (hasil generator), `ThemeColor`, `TextRole` | `SwiftUI`, `AppKit` |
| `Presentation/` | Modul paket Swift | Seperti spec `macos/0001` | `Interop`, `DesignSystem`, `SwiftUI`, `Observation` |

**File yang boleh diubah tiap milestone**:

| Milestone | File |
| --- | --- |
| 1 | `tooling/codegen/src/tokens/swift.rs`, `tooling/codegen/src/tokens/mod.rs`, `tooling/codegen/src/lib.rs` (satu baris komentar dokumen), `tooling/codegen/tests/tokens_swift_contract.rs`, `tooling/codegen/tests/tokens_contract.rs`, `tooling/codegen/tests/tokens_rules.rs` (satu uji), `.gitattributes`, `apps/macos/DesignSystem/`, `apps/macos/Tests/DesignSystemTests/`, `apps/macos/Package.swift` |
| 2 | `apps/macos/Presentation/GreetingView.swift` |
| 3 | Hanya `verify.md` spec ini |

Setiap milestone juga boleh mengubah `verify.md` spec ini, baris `**Status**` di `index.md` ini, `.graphifyignore` di akar (hanya bila hook Graphify menolak file baru, lihat **Kasus gagal**), dan empat berkas `graphify-out/` dari hook. `docs/platforms/macos/README.md` dan `docs/architecture/README.md` tidak diubah build ini; pemiliknya lead, lewat `/sync` sesudah merge (Follow-up). `.gitattributes` adalah file bersama dengan satu pemilik per wave.

**Isi file**:

Tulis file berikut persis seperti ini. Komentar kode berbahasa Inggris, mengikuti spec `_root/0002`.

`tooling/codegen/src/tokens/swift.rs`:

```rust
//! Writes the Swift definitions of the design tokens for the macOS app.
//!
//! Design: `docs/specs/macos/0005-tema-bersama-dan-kontrol-native/index.md`.
//!
//! The file holds one constant per semantic token. The types it uses,
//! `ThemeColor` and `TextRole`, are hand written in the same Swift module.

use std::fmt::Write as _;

use super::{Rgb, RoleName, TokenSet, Weight};
use crate::Output;

/// Path of the generated Swift file from the repo root.
pub const PATH: &str = "apps/macos/DesignSystem/Tokens.swift";

/// Swift words that cannot be used as a constant name without backticks.
const KEYWORDS: [&str; 53] = [
    "as",
    "associatedtype",
    "await",
    "break",
    "case",
    "catch",
    "class",
    "continue",
    "default",
    "defer",
    "deinit",
    "do",
    "else",
    "enum",
    "extension",
    "fallthrough",
    "false",
    "fileprivate",
    "for",
    "func",
    "guard",
    "if",
    "import",
    "in",
    "init",
    "inout",
    "internal",
    "is",
    "let",
    "nil",
    "open",
    "operator",
    "precedencegroup",
    "private",
    "protocol",
    "public",
    "repeat",
    "rethrows",
    "return",
    "self",
    "static",
    "struct",
    "subscript",
    "super",
    "switch",
    "throw",
    "throws",
    "true",
    "try",
    "typealias",
    "var",
    "where",
    "while",
];

/// Turns the resolved tokens into the Swift file for the macOS app.
#[must_use]
pub fn output(set: &TokenSet) -> Output {
    let mut text = String::from(
        "// Generated by desktop-codegen from design-system/tokens. Do not edit.\n\
         // To regenerate, run: cargo run -p desktop-codegen --locked -- tokens generate\n\
         \n\
         import SwiftUI\n\
         \n\
         /// Design tokens for the macOS app, one constant per semantic token.\n\
         public enum Tokens {\n",
    );
    text.push_str("    /// Semantic colors, each with a light and a dark value.\n");
    text.push_str("    public enum Colors {\n");
    for (index, color) in set.colors.iter().enumerate() {
        let note = if color.placeholder {
            " Placeholder value."
        } else {
            ""
        };
        separate(&mut text, index);
        let _ = writeln!(text, "        /// {}{note}", color.description);
        let _ = writeln!(
            text,
            "        public static let {} = ThemeColor(light: {}, dark: {})",
            identifier(&color.name),
            hex(color.light),
            hex(color.dark)
        );
    }
    text.push_str("    }\n\n");
    text.push_str("    /// Typography roles, each mapped to a native text style.\n");
    text.push_str("    public enum Typography {\n");
    for (index, role) in set.roles.iter().enumerate() {
        let style = match role.name {
            RoleName::Body => ".body",
        };
        let weight = match role.weight {
            Weight::Regular => ".regular",
            Weight::Semibold => ".semibold",
        };
        separate(&mut text, index);
        let _ = writeln!(text, "        /// {}", role.description);
        let _ = writeln!(
            text,
            "        public static let {} = TextRole(style: {style}, weight: {weight})",
            identifier(role.name.as_str())
        );
    }
    text.push_str("    }\n\n");
    text.push_str("    /// Spacing in points.\n");
    text.push_str("    public enum Spacing {\n");
    for (index, space) in set.spacing.iter().enumerate() {
        separate(&mut text, index);
        let _ = writeln!(text, "        /// {}", space.description);
        let _ = writeln!(
            text,
            "        public static let {}: CGFloat = {}",
            identifier(&space.name),
            space.value
        );
    }
    text.push_str("    }\n}\n");
    Output {
        path: PATH.to_string(),
        contents: text,
    }
}

/// Puts one blank line between two constants.
fn separate(text: &mut String, index: usize) {
    if index > 0 {
        text.push('\n');
    }
}

/// Writes a color as `0xRRGGBB`.
fn hex(color: Rgb) -> String {
    format!("0x{:02X}{:02X}{:02X}", color.0, color.1, color.2)
}

/// Turns a token name such as `text.primary` into `textPrimary`. A name that
/// is a Swift keyword is wrapped in backticks.
fn identifier(name: &str) -> String {
    let mut out = String::new();
    for (index, part) in name.split('.').enumerate() {
        if index == 0 {
            out.push_str(part);
        } else {
            let (first, rest) = part.split_at(part.len().min(1));
            out.push_str(&first.to_ascii_uppercase());
            out.push_str(rest);
        }
    }
    if KEYWORDS.contains(&out.as_str()) {
        format!("`{out}`")
    } else {
        out
    }
}
```

Di `tooling/codegen/src/tokens/mod.rs`, tiga perubahan dan tidak lebih:

1. Sesudah baris `mod schema;`, tambahkan baris `pub mod swift;`.
2. Di akhir fungsi `plan`, ganti

   ```rust
       Ok(Plan {
           outputs: schema::outputs(),
           summary,
       })
   ```

   dengan

   ```rust
       let mut outputs = schema::outputs();
       outputs.push(swift::output(&tokens));
       Ok(Plan { outputs, summary })
   ```

   `tokens` adalah `TokenSet` hasil `parse` yang sudah ada di fungsi itu. Urutan keluaran menjadi skema `colors`, `spacing`, `typography`, lalu `Tokens.swift`.
3. Komentar dokumen `/// Reads the three token files under `root` and plans the schema outputs.` di atas `plan` menjadi dua baris:

   ```rust
   /// Reads the three token files under `root` and plans the schema outputs
   /// and the Swift tokens of spec `macos/0005`.
   ```

Di `tooling/codegen/src/lib.rs`, satu perubahan: di komentar dokumen `check`, contoh baris sukses `... 3 placeholder values, 3 outputs up to date` menjadi `... 3 placeholder values, 4 outputs up to date`.

Di `tooling/codegen/tests/tokens_contract.rs`, uji `check_reports_stale_and_missing_outputs_and_generate_repairs_them` mengunci daftar dan jumlah keluaran. Tujuh perubahan dan tidak lebih:

1. Sesudah konstanta `SCHEMAS`, tambahkan:

   ```rust
   /// Every output of the source, in plan order: the schemas, then the Swift
   /// tokens of spec `macos/0005`.
   const OUTPUTS: [&str; 4] = [
       SCHEMAS[0],
       SCHEMAS[1],
       SCHEMAS[2],
       "apps/macos/DesignSystem/Tokens.swift",
   ];
   ```

2. `assert_eq!(missing, SCHEMAS);` menjadi `assert_eq!(missing, OUTPUTS);`.
3. `assert_eq!(problems.len(), 3);` sesudahnya menjadi `assert_eq!(problems.len(), 4);`.
4. `assert_eq!(written, SCHEMAS);` menjadi `assert_eq!(written, OUTPUTS);`.
5. `"3 outputs, 3 written"` menjadi `"4 outputs, 4 written"`.
6. `"3 outputs up to date"` menjadi `"4 outputs up to date"`.
7. `"3 outputs, 0 written"` menjadi `"4 outputs, 0 written"`.

Bagian sesudahnya, yang menyunting `SCHEMAS[0]` dan memeriksa bahwa hanya file itu yang ditulis ulang, tidak berubah. Jalankan `cargo fmt -p desktop-codegen` sesudahnya.

Di `tooling/codegen/tests/tokens_rules.rs`, uji `outputs_live_under_design_system_with_lf_attributes` mengunci bahwa setiap keluaran `tokens` tinggal di `design-system/`. Aturan O-6 spec `_root/0003` mengizinkan keluaran satu OS di folder aplikasinya, jadi uji itu dilonggarkan dengan sempit. Dua perubahan dan tidak lebih:

1. Komentar dokumen di atas uji itu menjadi:

   ```rust
   /// Covers AC-6 (O-6): outputs use fixed relative paths with `/`. Shared
   /// ones live under `design-system/` with a per type `text eol=lf` line in
   /// `.gitattributes`; the macOS tokens of spec `macos/0005` live under
   /// `apps/macos/DesignSystem/` with a `text eol=lf` line for that path.
   ```

2. Di dalam perulangan, ganti

   ```rust
           assert!(path.starts_with("design-system/"), "O-6: {path} location");
           let extension = path.rsplit('.').next().unwrap_or_default();
           assert!(
               rules.iter().any(|rule| rule.starts_with("/design-system/")
                   && rule.ends_with(&format!("*.{extension}"))),
               "O-6: no per type text eol=lf line for {path}: {rules:?}"
           );
   ```

   dengan

   ```rust
           let extension = path.rsplit('.').next().unwrap_or_default();
           let covered = if path.starts_with("design-system/") {
               rules.iter().any(|rule| {
                   rule.starts_with("/design-system/") && rule.ends_with(&format!("*.{extension}"))
               })
           } else {
               assert!(
                   path.starts_with("apps/macos/DesignSystem/"),
                   "O-6: {path} location"
               );
               rules.contains(&format!("/{path}").as_str())
           };
           assert!(covered, "O-6: no text eol=lf line for {path}: {rules:?}");
   ```

Nama uji tidak diubah. Uji itu tetap gagal bila baris `.gitattributes` untuk `Tokens.swift` hilang, atau bila sebuah keluaran ditulis di luar dua folder itu.

`tooling/codegen/tests/tokens_swift_contract.rs`:

```rust
//! Contract tests for the Swift emitter of spec `macos/0005`.
//!
//! They use only the public path of `desktop_codegen`. The token set here is
//! built by hand, so every kind of token, weight, and name is covered even
//! when the real source does not use it.

use desktop_codegen::tokens::swift::{PATH, output};
use desktop_codegen::tokens::{Color, Rgb, Role, RoleName, Space, TokenSet, Weight};

/// A token set with a placeholder color, a final color, a keyword name, a
/// name with a digit part, and the heavier weight.
fn sample() -> TokenSet {
    TokenSet {
        colors: vec![
            Color {
                name: "default".to_string(),
                description: "A keyword name.".to_string(),
                light: Rgb(0x00, 0x0A, 0xFF),
                dark: Rgb(0x12, 0x34, 0x56),
                placeholder: false,
            },
            Color {
                name: "text.on.accent2".to_string(),
                description: "A placeholder.".to_string(),
                light: Rgb(0xFF, 0xFF, 0xFF),
                dark: Rgb(0x00, 0x00, 0x00),
                placeholder: true,
            },
        ],
        roles: vec![Role {
            name: RoleName::Body,
            description: "Reading text.".to_string(),
            weight: Weight::Semibold,
        }],
        spacing: vec![Space {
            name: "gap.control".to_string(),
            description: "A gap.".to_string(),
            value: 0,
        }],
        placeholders: 1,
    }
}

/// Covers AC-2: the emitter maps every kind of token to one Swift constant.
#[test]
fn swift_emitter_maps_every_token_kind() {
    let file = output(&sample());
    assert_eq!(file.path, PATH, "AC-1: the output path is fixed");
    assert_eq!(PATH, "apps/macos/DesignSystem/Tokens.swift", "AC-1");
    let text = file.contents;
    for line in [
        "        /// A keyword name.\n        public static let `default` = ThemeColor(light: 0x000AFF, dark: 0x123456)\n",
        "        /// A placeholder. Placeholder value.\n        public static let textOnAccent2 = ThemeColor(light: 0xFFFFFF, dark: 0x000000)\n",
        "        /// Reading text.\n        public static let body = TextRole(style: .body, weight: .semibold)\n",
        "        /// A gap.\n        public static let gapControl: CGFloat = 0\n",
    ] {
        assert!(text.contains(line), "AC-2: expected `{line}` in:\n{text}");
    }
}

/// Covers AC-1: the file opens with the marker and follows the output rules.
#[test]
fn swift_output_follows_the_output_rules() {
    let text = output(&sample()).contents;
    let head = text.lines().take(3).collect::<Vec<_>>().join("\n");
    assert!(head.contains("Generated by desktop-codegen"), "O-1: {head}");
    assert!(head.contains("Do not edit"), "O-1: {head}");
    assert!(text.ends_with("}\n"), "O-3: one newline at the end");
    assert!(!text.ends_with("\n\n"), "O-3: one newline at the end");
    assert!(!text.contains('\r'), "O-3: no CR");
    assert!(!text.contains('\t'), "O-3: no tab");
    let fraction = text
        .as_bytes()
        .windows(3)
        .any(|w| w[0].is_ascii_digit() && w[1] == b'.' && w[2].is_ascii_digit());
    assert!(!fraction, "O-5: no fractions");
    assert_eq!(output(&sample()).contents, text, "O-2: two runs agree");
}
```

`.gitattributes`: tambahkan satu baris di akhir file. Baris lain tidak diubah.

```gitattributes
/apps/macos/DesignSystem/Tokens.swift text eol=lf
```

`apps/macos/DesignSystem/Tokens.swift` adalah hasil `tokens generate`. Kamu tidak mengetiknya. Cocokkan hasil generatormu dengan blok ini:

```swift
// Generated by desktop-codegen from design-system/tokens. Do not edit.
// To regenerate, run: cargo run -p desktop-codegen --locked -- tokens generate

import SwiftUI

/// Design tokens for the macOS app, one constant per semantic token.
public enum Tokens {
    /// Semantic colors, each with a light and a dark value.
    public enum Colors {
        /// Fill of the primary action control. Placeholder value.
        public static let accent = ThemeColor(light: 0x3366CC, dark: 0x3366CC)

        /// Window background behind all content. Placeholder value.
        public static let background = ThemeColor(light: 0xFAFAFA, dark: 0x111111)

        /// Main text. Placeholder value.
        public static let textPrimary = ThemeColor(light: 0x111111, dark: 0xFAFAFA)
    }

    /// Typography roles, each mapped to a native text style.
    public enum Typography {
        /// Default reading text.
        public static let body = TextRole(style: .body, weight: .regular)
    }

    /// Spacing in points.
    public enum Spacing {
        /// Gap between stacked controls.
        public static let gapControl: CGFloat = 12

        /// Padding between the window edge and its content.
        public static let insetWindow: CGFloat = 20
    }
}
```

`apps/macos/DesignSystem/Theme.swift`:

```swift
import AppKit
import SwiftUI

/// A semantic color with one sRGB value for the light appearance and one for
/// the dark appearance, each written as `0xRRGGBB`.
public struct ThemeColor: Sendable, Equatable {
    /// The value in the light appearance.
    public let light: UInt32

    /// The value in the dark appearance.
    public let dark: UInt32

    public init(light: UInt32, dark: UInt32) {
        self.light = light
        self.dark = dark
    }

    /// The red, green, and blue channels, 0 to 255, for one appearance.
    public func channels(dark isDark: Bool) -> [UInt8] {
        let value = isDark ? dark : light
        return [UInt8((value >> 16) & 0xFF), UInt8((value >> 8) & 0xFF), UInt8(value & 0xFF)]
    }

    /// An AppKit color that follows the appearance of the view that draws it.
    public var nsColor: NSColor {
        NSColor(name: nil) { appearance in
            let isDark = appearance.bestMatch(from: [.aqua, .darkAqua]) == .darkAqua
            let channels = self.channels(dark: isDark).map { CGFloat($0) / 255 }
            return NSColor(srgbRed: channels[0], green: channels[1], blue: channels[2], alpha: 1)
        }
    }

    /// The SwiftUI color. It follows the light and dark setting of the system.
    public var color: Color {
        Color(nsColor: nsColor)
    }
}

/// A typography role: a native text style and a weight. The size comes from
/// the system text style, never from a fixed number.
public struct TextRole: Sendable, Equatable {
    /// The native text style that sets the size.
    public let style: Font.TextStyle

    /// The font weight.
    public let weight: Font.Weight

    public init(style: Font.TextStyle, weight: Font.Weight) {
        self.style = style
        self.weight = weight
    }

    /// The SwiftUI font for this role.
    public var font: Font {
        Font.system(style).weight(weight)
    }
}
```

`apps/macos/Tests/DesignSystemTests/ThemeTests.swift`:

```swift
import AppKit
import DesignSystem
import SwiftUI
import Testing

/// Returns the sRGB channels, 0 to 255, that `color` has under `appearance`.
private func resolved(_ color: NSColor, under appearance: NSAppearance.Name) -> [UInt8] {
    var channels: [UInt8] = []
    NSAppearance(named: appearance)?.performAsCurrentDrawingAppearance {
        guard let srgb = color.usingColorSpace(.sRGB) else { return }
        channels = [srgb.redComponent, srgb.greenComponent, srgb.blueComponent]
            .map { UInt8(($0 * 255).rounded()) }
    }
    return channels
}

@Suite struct ThemeTests {
    @Test func channelsSplitLightAndDark() {
        let color = ThemeColor(light: 0x112233, dark: 0xAABBCC)
        #expect(color.channels(dark: false) == [0x11, 0x22, 0x33])
        #expect(color.channels(dark: true) == [0xAA, 0xBB, 0xCC])
    }

    @Test func colorFollowsTheAppearance() {
        let color = ThemeColor(light: 0x112233, dark: 0xAABBCC).nsColor
        #expect(resolved(color, under: .aqua) == [0x11, 0x22, 0x33])
        #expect(resolved(color, under: .darkAqua) == [0xAA, 0xBB, 0xCC])
        #expect(resolved(color, under: .accessibilityHighContrastAqua) == [0x11, 0x22, 0x33])
        #expect(resolved(color, under: .accessibilityHighContrastDarkAqua) == [0xAA, 0xBB, 0xCC])
    }

    @Test func everyColorTokenResolvesInBothAppearances() {
        let tokens = [Tokens.Colors.accent, Tokens.Colors.background, Tokens.Colors.textPrimary]
        for token in tokens {
            #expect(resolved(token.nsColor, under: .aqua) == token.channels(dark: false))
            #expect(resolved(token.nsColor, under: .darkAqua) == token.channels(dark: true))
        }
    }

    @Test func textRoleUsesTheSystemTextStyle() {
        let role = TextRole(style: .body, weight: .semibold)
        #expect(role.font == Font.system(.body).weight(.semibold))
        #expect(role.font != Font.system(.body).weight(.regular))
        #expect(Tokens.Typography.body == TextRole(style: .body, weight: .regular))
    }

    @Test func spacingTokensAreWholePoints() {
        for value in [Tokens.Spacing.gapControl, Tokens.Spacing.insetWindow] {
            #expect(value >= 0)
            #expect(value == value.rounded())
        }
    }
}
```

`apps/macos/Package.swift` sesudah perubahan. Tiga hal berubah dari isi spec `macos/0001`: target `DesignSystem` ditambahkan, `Presentation` bergantung juga pada `DesignSystem`, dan target uji `DesignSystemTests` ditambahkan di akhir. Bila spec lain sudah menambah baris ke file ini, pertahankan baris itu dan terapkan tiga perubahan ini saja.

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
        .target(name: "DesignSystem", path: "DesignSystem"),
        .target(name: "Presentation", dependencies: ["Interop", "DesignSystem"], path: "Presentation"),
        .testTarget(name: "InteropTests", dependencies: ["Interop"], path: "Tests/InteropTests"),
        .testTarget(name: "PresentationTests", dependencies: ["Presentation", "Interop"], path: "Tests/PresentationTests"),
        .testTarget(name: "DesignSystemTests", dependencies: ["DesignSystem"], path: "Tests/DesignSystemTests"),
    ]
)
```

`apps/macos/Presentation/GreetingView.swift` sesudah perubahan. Blok ini menganggap spec `macos/0006` belum dibangun. Bila sudah, baris tombol tetap bentuk spec itu (`Button(LocalizedText.string(.greetingSubmit)) {`) dan baris lain di blok ini diterapkan di sekitarnya; jangan kembalikan ke `Button("Greet")`.

```swift
import DesignSystem
import SwiftUI

/// The greeting window content: a name field, a button, and the result label.
public struct GreetingView: View {
    @State private var model: GreetingViewModel

    public init(model: GreetingViewModel) {
        _model = State(initialValue: model)
    }

    public var body: some View {
        VStack(alignment: .leading, spacing: Tokens.Spacing.gapControl) {
            TextField("Name", text: $model.name)
            Button("Greet") {
                model.greet()
            }
            .keyboardShortcut(.defaultAction)
            .buttonStyle(.borderedProminent)
            .tint(Tokens.Colors.accent.color)
            Text(verbatim: model.greeting ?? "")
                .font(Tokens.Typography.body.font)
                .foregroundStyle(Tokens.Colors.textPrimary.color)
        }
        .padding(Tokens.Spacing.insetWindow)
        .frame(minWidth: 320)
        .background(Tokens.Colors.background.color)
    }
}
```

Catatan untuk `GreetingView.swift`:

* `.tint` dan `.buttonStyle(.borderedProminent)` dipasang pada tombol, bukan pada seluruh tampilan, supaya hanya tindakan utama yang memakai warna merek.
* `.keyboardShortcut(.defaultAction)` tetap ada, jadi Return tetap menjalankan `Greet`.
* Baris `Text(verbatim: model.greeting ?? "")` tidak berubah, jadi AC-8 spec `macos/0001` tetap lulus.
* `.background(...)` dipasang sesudah `.padding` dan `.frame`, supaya warna mengisi seluruh isi jendela, termasuk jarak tepinya.

**Kasus gagal yang sudah diputuskan**:

| Kejadian | Perilaku yang diharapkan |
| --- | --- |
| Token diubah tetapi `Tokens.swift` belum dibuat ulang | `tokens check` dan `cargo test --workspace --locked` gagal dan menyebut perintah `generate`. Jalankan `generate`, lalu commit sumber bersama `Tokens.swift`. |
| `Tokens.swift` disunting tangan | Sama. `generate` menimpanya. |
| Token baru ditambahkan di sumber | `Tokens.swift` mendapat konstanta baru. K-4 meminta `GreetingView` memakainya atau spec ini menulis alasannya; hitungan di AC-3 diperbarui lewat pembaruan spec ini. |
| Peran atau bobot baru ditambahkan ke `RoleName` atau `Weight` | `swift.rs` gagal dikompilasi sampai cabang `match` baru ditulis. Tabel pemetaan di spec ini diperbarui lebih dulu. |
| Nama token sama dengan kata kunci Swift | Emitter memberi backtick. Di tempat pakai sesudah titik, kebanyakan kata kunci boleh ditulis biasa (`Tokens.Colors.default`), tetapi `self`, `init`, `Type`, dan `Protocol` tetap butuh backtick (`` Tokens.Colors.`self` ``). Hindari nama itu di sumber token. |
| Warna merek asli membuat teks tombol sulit dibaca | Tidak tertangkap otomatis, karena warna teks tombol diatur sistem. Terlihat di langkah manual 2 dan 4; laporkan, lalu nilai `accent` diubah lewat spec `_root/0003`. |
| `swift test` dijalankan tanpa pustaka Rust | Penautan gagal, seperti di spec `macos/0001`. Bangun `desktop-macos-ffi` lebih dulu. |
| Tombol `Greet` tidak berwarna `accent` di jendela nyata | Langkah manual 2 gagal. Jangan mengganti kontrol dengan gambar sendiri. Laporkan dengan versi macOS; jalan keluarnya adalah pembaruan spec ini. |
| Hook Graphify menolak file baru | Ikuti aturan spec `_root/0001`: tambah entri `.graphifyignore` (diizinkan di setiap milestone), catat entri dan alasannya di `verify.md`, dan laporkan ke lead supaya spec ini diperbarui. |
| Jendela tidak aktif saat dilihat | macOS menggambar tombol utama tanpa isi warna bila jendelanya bukan jendela aktif. Warna tombol hanya dinilai saat jendela `DesktopMac` aktif (klik jendela itu lebih dulu). |

**Critical test scenarios**:

* Jalur utama: `tokens check` lulus dengan empat keluaran dan `Tokens.swift` sama dengan hasil generator, memverifikasi **AC-1**.
* Pemetaan: token set buatan tangan dengan nama kata kunci, nama berangka, warna sementara dan tetap, serta bobot `semibold` ditulis ke Swift yang benar, memverifikasi **AC-2**.
* Terang dan gelap tanpa jendela: `ThemeColor` memberi nilai yang benar di tampilan `aqua`, `darkAqua`, dan kedua tampilan kontras tinggi, untuk nilai uji dan untuk setiap token warna, memverifikasi **AC-4**.
* Tipografi: `TextRole.font` sama dengan huruf dari gaya teks sistem dengan bobot itu, dan `Tokens.Typography.body` sama dengan `.body` dan `.regular`, memverifikasi **AC-4** (bagian teks di **AC-6**).
* Pemakaian: pemeriksaan teks menunjukkan enam token dipakai sekali dan tidak ada nilai literal, memverifikasi **AC-3**, **AC-6**.
* Gagal: `Tokens.swift` yang dihapus atau disunting tertangkap `check` dan diperbaiki `generate`, memverifikasi **AC-1**.
* Build: aplikasi terbangun Debug dan Release dengan modul baru, memverifikasi **AC-5**.
* Batas: daftar file, uji pointer, dan gerbang workspace, memverifikasi **AC-9**, **AC-10**.
* Akses: tidak ada peran atau izin di fitur ini.
* Jendela nyata: langkah manual, memverifikasi **AC-7**, **AC-8**.

**Rencana verifikasi manual**:

Langkah ini hanya dijalankan user pada sesi GUI. `/develop` menyalinnya ke `verify.md` di bawah `### UI / manual` pada milestone 3.

Prasyarat: Mac Apple Silicon dengan macOS 14 atau lebih baru (yang sudah dipakai membangun: 27.0.1), Xcode 27.1, sesi desktop yang sedang login, dan repo pada commit yang lulus **AC-5**. Nilai warna yang diharapkan dibaca dari `apps/macos/DesignSystem/Tokens.swift` pada commit itu; angka di bawah adalah nilai sementara saat spec ini ditulis. Bilah judul jendela selalu milik sistem dan tidak dinilai. Warna tombol hanya dinilai saat jendela `DesktopMac` aktif: klik jendela itu lebih dulu, karena macOS menggambar tombol utama tanpa isi warna di jendela yang tidak aktif.

1. Dari akar repo jalankan `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration Debug -derivedDataPath apps/macos/DerivedData build`. Harapan: `** BUILD SUCCEEDED **`. (**AC-7**)
2. Buka System Settings, Appearance. Catat pilihan Appearance dan Accent color sekarang supaya bisa dikembalikan. Pilih Appearance Light dan Accent color Orange, supaya warna aksen sistem jelas berbeda dari warna token. Jalankan `open apps/macos/DerivedData/Build/Products/Debug/DesktopMac.app`, ketik `Ayu`, lalu klik `Greet`. Harapan: area isi jendela (di bawah bilah judul) hampir putih (`#FAFAFA`); teks `Hello, Ayu!` hampir hitam (`#111111`); tombol `Greet` berisi biru (`#3366CC`), bukan oranye, dengan teks yang terbaca; kolom `Name` tampak seperti kolom teks Mac biasa. (**AC-7**)
3. Buka aplikasi Digital Color Meter dan pilih "Display in sRGB". Klik jendela `DesktopMac` supaya aktif (Digital Color Meter tetap mengukur), lalu arahkan kursor ke area isi jendela dan ke isi tombol `Greet`. Harapan: area isi sekitar `250 250 250` dan isi tombol sekitar `51 102 204`, dengan selisih kecil karena bayangan dan gradasi kontrol. (**AC-7**)
4. Tanpa menutup aplikasi, ganti Appearance ke Dark, lalu klik jendela `DesktopMac` supaya aktif. Harapan: jendela berubah tanpa dibuka ulang; area isi hampir hitam (`#111111`), teks `Hello, Ayu!` hampir putih (`#FAFAFA`), tombol `Greet` tetap biru (`#3366CC`), bukan oranye, dan kolom `Name` memakai gaya gelap sistem. Ulangi pengukuran langkah 3: area isi sekitar `17 17 17` dan isi tombol sekitar `51 102 204`. (**AC-7**)
5. Tekan Command Q, lalu jalankan lagi perintah `open` di langkah 2 saat Appearance masih Dark. Ketik `Ayu`, lalu klik `Greet`. Harapan: jendela terbuka langsung dengan area isi hampir hitam, dan teks `Hello, Ayu!` hampir putih. Biarkan Appearance Dark dan Accent color Orange sampai langkah 6 selesai. (**AC-7**)
6. Klik kolom `Name`. Harapan: cincin fokus sistem terlihat jelas di sekeliling kolom dan berwarna oranye (aksen sistem), bukan biru token. Hapus isi kolom, ketik `Budi`, lalu tekan Return tanpa mouse. Harapan: label menampilkan `Hello, Budi!`. Ganti Appearance ke Light, klik kolom `Name` lagi, dan amati cincin fokus yang sama. (**AC-8**)
7. Buka System Settings, Keyboard, nyalakan Keyboard navigation. Di aplikasi klik kolom `Name`, hapus isinya, ketik `Citra`, lalu tekan Tab. Harapan: fokus pindah ke tombol `Greet` dan cincin fokus terlihat di tombol. Tekan Spasi. Harapan: label menampilkan `Hello, Citra!`. Kembalikan pengaturan Keyboard navigation. (**AC-8**)
8. Buka System Settings, Accessibility, Display, lalu perbesar Text Size. Lihat kembali jendela aplikasi (buka ulang bila perlu, lalu ketik `Ayu` dan klik `Greet`). Harapan: tidak ada teks yang terpotong atau bertumpuk, dan label hasil seukuran dengan teks tombol dan kolom. Catat apakah teks aplikasi ikut membesar. macOS boleh tidak mengubah ukuran teks aplikasi ini; itu dicatat, bukan kegagalan. Kembalikan pengaturan Text Size. (**AC-8**)
9. Di aplikasi hapus isi kolom, ketik `Budi`, lalu tekan Return. Nyalakan VoiceOver dengan Command F5 (pada sebagian laptop Fn Command F5), lalu pindah ke kolom, tombol, dan label. Harapan: VoiceOver menyebut `Name` untuk kolom, `Greet` untuk tombol, dan membacakan `Hello, Budi!` untuk label. Matikan VoiceOver. (**AC-8**)
10. Buka System Settings, Accessibility, Display, nyalakan Increase contrast, lalu klik jendela `DesktopMac`. Harapan: area isi dan teks hasil tetap memakai warna token yang sama dan tetap mudah dibaca, dan kontrol mendapat garis tepi yang lebih tegas dari sistem. Matikan Increase contrast, kembalikan Appearance dan Accent color ke pilihan yang dicatat di langkah 2, lalu tekan Command Q. (**AC-8**)

Catat host, arsitektur, versi macOS, tampilan yang aktif, dan hasil tiap langkah. Langkah yang belum dijalankan tetap tercatat sebagai belum diverifikasi.

## Build plan

Pendekatan build proyek adalah Tracer Bullet. Benang tipisnya berjalan dari file token sampai nilai Swift yang diuji tanpa jendela. Sesudah itu jendela memakainya, lalu bukti dikumpulkan. Satu langkah menjadi satu commit, dengan subject yang memuat `spec macos/0005 step <n>`.

Prasyarat: needs scope 008 milestone 1 dari spec `_root/0003` di `main` (paket `desktop-codegen`, tiga file token, tiga skema, dan `.gitattributes`). Needs pembaruan spec `macos/0001` yang didaftar di Follow-up (AC-12 dan isi file yang berubah). Tidak boleh berjalan bersamaan dengan build lain yang mengubah `tooling/codegen/`, `.gitattributes`, atau `apps/macos/Package.swift`.

**Milestone 1: Token sampai ke Swift** (langkah 1)

1. Buat `swift.rs` dan `tokens_swift_contract.rs`, ubah `tokens/mod.rs`, `tokens_contract.rs`, dan `.gitattributes`, lalu jalankan `cargo run -p desktop-codegen --locked -- tokens generate` untuk menulis `Tokens.swift` dan cocokkan dengan **Isi file**. Buat `Theme.swift`, `ThemeTests.swift`, dan ubah `Package.swift`. Jalankan perintah `cargo` di **AC-10** (`fmt`, `clippy`, `test --workspace`, dan uji `macos_bridge_ownership`) dan `swift test` seperti di **AC-4**, semuanya sebelum commit. Sesudah commit, jalankan lagi pemeriksaan **AC-1**, karena pemeriksaan `git` baru bermakna saat filenya sudah dilacak, satisfies **AC-1**, **AC-2**, **AC-4**

**Milestone 2: Jendela sapaan memakai token** (langkah 2 dan 3, after milestone 1)

2. Ubah `GreetingView.swift` sesuai **Isi file**, lalu jalankan `swift test`, `rm -rf apps/macos/.build` dan `swift build` seperti di **AC-5**, `xcodebuild` Debug dan Release, dan pemeriksaan teks **AC-3** dan **AC-6**, satisfies **AC-3**, **AC-5**, **AC-6**
3. Pada working tree yang bersih, dengan `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, jalankan ulang setiap perintah **AC-1** sampai **AC-6** dan **AC-9** sampai **AC-11**, catat perintah dan keluarannya di `verify.md` bagian `## Milestone 2: Jendela sapaan memakai token` bersama host dan arsitektur, salin **Rencana verifikasi manual** ke bawah `### UI / manual`, lalu hapus `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/`. Pemeriksaan `git log` di **AC-9** dijalankan lagi sesudah commit langkah 3, dan sekali lagi sesudah commit langkah 4, satisfies **AC-9**, **AC-10**, **AC-11**

**Milestone 3: Sesi GUI di Mac** (langkah 4, after milestone 2; `BLOCKED`, `owner: user`)

4. User menjalankan sepuluh langkah di **Rencana verifikasi manual** pada sesi GUI dan mencatat hasilnya di `verify.md` bagian `## Milestone 3: Sesi GUI di Mac`, di bawah `### UI / manual`. Sesi koordinasi tidak menjalankan langkah ini, satisfies **AC-7**, **AC-8**

## Consequences

**Positive**:

* Warna, huruf, dan jarak jendela sapaan berasal dari satu sumber, dan penyimpangan tertangkap gerbang `cargo test --workspace --locked` yang sudah ada.
* Pemetaan terang dan gelap terbukti tanpa jendela untuk setiap token warna, termasuk tampilan kontras tinggi.
* Proyek Xcode dan `DesktopMacApp.swift` tidak berubah, jadi tidak bertabrakan dengan spec macOS scope 009.
* Tidak ada asset catalog, berkas sumber daya, atau dependensi baru.
* Kontrol tetap asli: cincin fokus, pintasan Return, dan label VoiceOver tidak disentuh.

**Negative / tradeoffs**:

* Warna dibuat lewat AppKit (`NSColor` dinamis), jadi `DesignSystem` hanya berlaku untuk macOS. Itu sesuai batas aplikasi ini, tetapi tidak bisa dipakai ulang di platform Apple lain.
* Latar jendela kini warna rata dari token, bukan latar jendela bawaan sistem. Jendela tidak lagi ikut perubahan halus yang kelak dibuat macOS pada latar bawaan.
* Tombol `Greet` memakai warna merek, bukan warna aksen pilihan pengguna. Kontras teks tombol terhadap `accent` tidak diperiksa otomatis.
* Tampilan terang dan gelap di layar, warna tombol, fokus, ukuran teks, dan VoiceOver tetap menunggu sesi GUI user.
* Menambah token berarti menyentuh sumber, `Tokens.swift`, `GreetingView`, hitungan di AC-3, dan daftar di `ThemeTests`.
* Uji `everyColorTokenResolvesInBothAppearances` menyebut tiga token warna satu per satu. Token warna baru harus ditambahkan ke daftar itu.
* Build ini mengubah dua berkas milik spec `_root/0003` (`tokens/mod.rs` dan `tokens_contract.rs`), jadi tidak bisa berjalan bersamaan dengan build lain di `tooling/codegen/`.
* Nilai warna masih sementara. Angka di langkah manual berubah saat warna merek masuk.

**Neutral**:

* Spec `macos/0001` perlu diperbarui lebih dulu (lihat Follow-up). Tanpa itu AC-12 spec tersebut gagal begitu file baru masuk.
* Uji `hand_written_swift_uses_no_pointers` di `tooling/codegen/tests/macos_bridge_ownership.rs` (dipindah dari `crates/core/tests/` lewat spec `_root/0007`) kini juga membaca `Tokens.swift` yang dihasilkan generator. Uji itu tetap lulus dan tidak perlu diubah.
* Hitungan keluaran di spec `_root/0003` (tiga) hanya berlaku pada akhir milestone 1 spec itu. Sesudah spec ini dibangun hitungannya empat.
* `/develop` menulis `verify.md` saat build. Spec ini tidak membuatnya.

## Follow-up

* [x] Jawaban atas Follow-up "Spec macOS scope 008" di spec `_root/0003`: emitter Swift (K-1, K-2, K-9) ada di **Isi file**, modul `DesignSystem` di **Tata letak**, pemakaian di `GreetingView` (K-3, K-4) di **AC-3**, `accent` lewat tint dan `body` ke gaya teks SwiftUI di **Decision**, dan langkah manual K-5 dan K-7 di **Rencana verifikasi manual**.
* [x] Lead, lewat `architect (update)` spec `macos/0001`. Diterapkan di spec `macos/0001` (2026-11-15). Perubahan persis yang dibutuhkan spec ini:
  1. **AC-12**: jumlah path bertambah tepat tiga dari angka yang berlaku sebelum spec ini dibangun (16 bila belum ada spec lain yang menambah path; spec `macos/0004` menambah `apps/macos/Config/Shared.xcconfig`, jadi angka akhirnya ditetapkan lead saat menggabungkan semua permintaan). Tiga path barunya: `apps/macos/DesignSystem/Theme.swift`, `apps/macos/DesignSystem/Tokens.swift`, dan `apps/macos/Tests/DesignSystemTests/ThemeTests.swift`. Kalimat "tidak ada folder `Platform`, `DesignSystem`, atau `Resources`" menjadi "tidak ada folder `Platform` atau `Resources`".
  2. **Tata letak**: kalimat pembuka yang menyebut jumlah path dan tabelnya mendapat tiga baris itu, dengan sumber isi "spec `macos/0005`". Kalimat "Folder `Platform`, `DesignSystem`, dan `Resources` ... belum dibuat" tidak lagi menyebut `DesignSystem`.
  3. Tabel peran bagian `apps/macos/`: tambah baris `DesignSystem/`, dan `Presentation/` boleh mengimpor `DesignSystem`.
  4. **Isi file** `apps/macos/Package.swift` dan `apps/macos/Presentation/GreetingView.swift`: beri keterangan bahwa blok itu keadaan akhir milestone 2 spec `macos/0001`, dan isi sekarang ada di spec `macos/0005`. Bila ada uji atau AC yang membandingkan kedua file itu byte demi byte dengan blok lama, sesuaikan.
  5. **Follow-up** baris Scope 008: tandai `[x] ... diterapkan di spec macos/0005`.
  6. Tidak perlu diubah oleh spec ini: AC-2 dan AC-3 (baris `7 tests` dan `4 tests` tetap muncul), AC-4, AC-8 (hitungan `Text(verbatim: model.greeting` tetap 1), `project.pbxproj`, dan `DesktopMacApp.swift`. Dua file terakhir diubah spec `macos/0004`, bukan spec ini.
* [ ] Lead: `test-preferences.json` menyebut bahwa file baru di `apps/macos` butuh pembaruan spec `macos/0001` (AC-12). Sesudah spec ini dibangun, catat juga suite `ThemeTests` di `apps/macos/Tests/DesignSystemTests/` dan uji emitter di `tooling/codegen/tests/tokens_swift_contract.rs`.
* [ ] Lead: build spec ini mengubah `tooling/codegen/src/tokens/mod.rs` (satu baris, isi `Plan`, dan komentar dokumen), `tooling/codegen/src/lib.rs` (satu baris komentar dokumen), `tooling/codegen/tests/tokens_contract.rs` (satu konstanta dan enam baris), dan satu uji di `tooling/codegen/tests/tokens_rules.rs` (milik tahap `Test it` scope 008), yang dibuat untuk spec `_root/0003`, dan menambah satu baris ke `.gitattributes`. Jadwalkan sesudah milestone 1 spec `_root/0003` dan tidak bersamaan dengan build scope 009, 010, atau 011 di `tooling/codegen/`.
* [ ] Lead: tambahkan tautan balik ke spec ini di berkas scope 008. `docs/platforms/macos/README.md` dan tabel "Yang sudah ada" di `docs/architecture/README.md` perlu menyebut modul `DesignSystem`. Itu sengaja di luar commit build ini (kedua file terlarang bagi developer); pemiliknya lead, lewat `/sync` sesudah merge.
* [ ] Nilai dari user: warna merek (`N7`). Sesudah nilai masuk lewat spec `_root/0003`, jalankan `generate`, lalu perbarui blok `Tokens.swift`, tabel **Data model sketch**, AC-2 (hitungan `Placeholder value`), AC-7, dan angka di langkah manual lewat pembaruan spec ini.
* [x] Spec macOS scope 009 (`macos/0004`, di `main`): kedua spec tidak menyentuh file yang sama di `apps/macos/`. Spec `macos/0004` mengubah `project.pbxproj` dan `DesktopMacApp.swift` dan menambah `Config/Shared.xcconfig`; spec ini mengubah `Package.swift` dan `GreetingView.swift` dan menambah `DesignSystem/` serta `Tests/DesignSystemTests/`. Pemeriksaan teks AC-3, AC-4, dan AC-6 spec ini lulus untuk `DesktopMacApp.swift` versi lama maupun versi spec `macos/0004`, dan keduanya menambah satu baris berbeda di `.gitattributes`. Isi file tidak bertabrakan, tetapi hitungan path bertabrakan: lihat butir berikut.
* [x] Lead, lewat pembaruan spec `macos/0004`: diterapkan di spec `macos/0004` (2026-11-15), AC-11 kini memeriksa path yang ditambahkan commit langkahnya. AC-11 spec itu menulis bahwa `git ls-files apps/macos bindings/swift crates/macos-ffi | wc -l` mencetak `17`. Bila spec ini dibangun lebih dulu, angkanya menjadi `20` saat spec `macos/0004` dibangun; bila spec `macos/0004` dibangun lebih dulu, pemeriksaan ulang AC-11 itu gagal sesudah spec ini dibangun. Ubah AC-11 spec `macos/0004` menjadi "bertambah tepat satu path dibanding commit sebelum langkah 1", seperti AC-9 spec ini, atau tetapkan angka akhirnya bersama pembaruan AC-12 spec `macos/0001`.
* [x] Scope 002: dijawab spec `macos/0008` (aturan `C-1` sampai `C-3`). Jadikan pemeriksaan teks **AC-3** dan **AC-6** pemeriksaan tetap (tidak ada warna, bobot, atau jarak literal di kode tampilan; hanya `Presentation` yang mengimpor `DesignSystem`).
* [ ] Scope 010: teks `Name` dan `Greet` pindah ke katalog bahasa. Pemeriksaan `grep` di AC-6 yang menyebut teks itu ikut diperbarui.
* [ ] Scope 011: bila merek membawa huruf sendiri, `TextRole.font` adalah satu satunya tempat yang berubah. Ikon aplikasi dan warna aksen di asset catalog bukan bagian spec ini.
* [x] Scope 012: `scripts/generate-design-tokens.sh` membungkus `tokens generate`, dan runner macOS menjalankan `tokens check`, `swift test`, dan `xcodebuild` dari spec ini. Dijawab [spec `_root/0008`](../../_root/0008-build-dan-penerimaan-per-target/index.md): `tokens check` lewat `validate.sh` dan langkah 2 `build-macos.sh`, `swift test` lewat `G-3`, dan `xcodebuild` lewat `G-4` (2026-11-15).
* [ ] Bila langkah manual 8 menunjukkan macOS memperbesar teks kontrol tetapi tidak memperbesar label hasil, itu pelanggaran K-6. Perbaiki lewat pembaruan spec ini.
* [ ] Uji di macOS 14 sampai 26 belum ada. `Font.system(_:)` dan `NSColor(name:dynamicProvider:)` tersedia sejak sebelum macOS 14, tetapi tampilannya di versi itu belum dilihat.
* [ ] AC-7, AC-8, dan milestone 3 menunggu sesi GUI user (`owner: user`).
