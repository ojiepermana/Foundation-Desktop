# 0005. Tema bersama dan kontrol native di macOS: catatan keputusan

Berkas ini menyimpan alasan di balik [spec macOS 0005](index.md). Kamu tidak perlu membacanya untuk membangun. Bacalah saat ingin tahu mengapa sebuah pilihan diambil, atau saat spec ini akan diubah.

## Context

[Spec `_root/0003`](../../_root/0003-tema-bersama-dan-kontrol-native/index.md) sudah menetapkan arti token, sumbernya, alat `desktop-codegen`, aturan keluaran O-1 sampai O-7, dan kewajiban K-1 sampai K-9 untuk tiap OS. Yang belum ada adalah sisi macOS: bagaimana nilai token menjadi kode Swift, di mana kode itu tinggal, bagaimana `GreetingView` memakainya, dan bagaimana semua itu dibuktikan. Jarak di `GreetingView` masih angka literal, dan jendela belum memakai satu warna pun dari sumber bersama.

Beberapa hal sudah tetap:

* Spec `_root/0003` menetapkan enam token semantik, bahwa tipografi adalah peran dan bobot yang dipetakan ke gaya teks native, bahwa `accent` diterapkan lewat mekanisme aksen native, dan bahwa lebar minimum jendela bukan token.
* [Spec `macos/0001`](../0001-sapaan-di-jendela-macos/index.md) menetapkan paket Swift lokal dengan modul `Interop` dan `Presentation`, proyek Xcode tulisan tangan, versi minimum macOS 14, arsitektur `arm64`, dan perintah `swift test` serta `xcodebuild`. AC-12 spec itu mengunci 16 path dan melarang folder `DesignSystem`.
* [Rujukan macOS](../../../platforms/macos/README.md) menyebut bagian `DesignSystem` di `apps/macos/` dan meminta token diperiksa di tampilan terang dan gelap, bersama fokus keyboard dan aksesibilitas kontrol.
* Kriteria 16 dan 17 di [Penerimaan dan bukti fondasi](../../../quality/README.md) adalah yang harus dibuktikan di Mac. Kriteria 32 melarang pustaka komponen.

Hal yang membentuk keputusan:

* Sesi koordinasi boleh menjalankan `swift test` dan `xcodebuild`, tetapi tidak boleh membuka jendela. Setiap bukti yang butuh mata menunggu user.
* `swift test` di baris perintah adalah jalur uji tanpa jendela yang sudah terbukti. Apa pun yang hanya bisa dibangun Xcode keluar dari jalur itu.
* Spec macOS scope 009 (`macos/0004`, sudah di `main`) mengubah `project.pbxproj` dan `DesktopMacApp.swift`. Dua spec yang mengubah file tulisan tangan yang sama harus digabung lead dengan tangan.
* `tokens check` berjalan di setiap host dan membandingkan semua keluaran, termasuk keluaran untuk Mac, jadi akhir baris file Swift harus sama di Windows dan Linux.
* Warna merek belum ada. Nilai sekarang adalah contoh bertanda sementara.

Yang belum tetap, dan menjadi isi spec ini: bentuk dan lokasi keluaran Swift, cara warna mengikuti tampilan sistem, cara peran tipografi menjadi huruf, cara `accent` dan `background` diterapkan tanpa mengganti kontrol asli, susunan modul, dan pembagian antara bukti tanpa jendela dan langkah manual.

## Options considered

Keputusan utama adalah bagaimana warna token sampai ke layar dan mengikuti tampilan terang dan gelap. Keputusan lain yang lebih kecil ada di tabel sesudah bagian ini.

### Opsi 1: Warna dinamis di kode, modul `DesignSystem` di paket Swift yang sudah ada

Emitter menulis satu file Swift berisi konstanta. `ThemeColor` membungkus `NSColor` dinamis yang memilih nilai terang atau gelap saat digambar, dan `Color(nsColor:)` membawanya ke SwiftUI.

**Pros**:

* Terbangun dan teruji lewat `swift test` di baris perintah, jadi pemetaan terang dan gelap terbukti tanpa jendela untuk setiap token.
* Satu file keluaran, tanpa berkas sumber daya dan tanpa perubahan proyek Xcode.
* Salah ketik nama token gagal saat kompilasi, karena token adalah konstanta, bukan teks.
* Tampilan kontras tinggi ikut terpetakan tanpa kode tambahan.

**Cons**:

* Bergantung pada AppKit, jadi modul ini hanya untuk macOS.
* Warna tidak terlihat di pemilih warna Xcode atau Interface Builder.
* Ada dua tipe tulisan tangan yang harus dijaga di samping file hasil generator.

### Opsi 2: Asset catalog hasil generator

Emitter menulis satu `colorset` per warna di sebuah `Assets.xcassets`, dengan varian terang dan gelap. Kode memakai `Color("accent", bundle:)`.

**Pros**:

* Cara baku Apple, dengan varian kontras tinggi bawaan dan pratinjau di Xcode.
* Tidak ada kode AppKit.

**Cons**:

* `swift build` dan `swift test` di baris perintah tidak mengompilasi asset catalog, jadi uji tanpa jendela untuk warna hilang dan hanya `xcodebuild` yang bisa memakainya.
* Butuh folder sumber daya dan perubahan `project.pbxproj` atau `resources:` di `Package.swift`, yang bertabrakan dengan spec macOS scope 009.
* Satu folder dan satu file JSON per warna, dan nama dipanggil sebagai teks yang baru ketahuan salah saat berjalan.
* Hanya menjawab warna. Tipografi dan jarak tetap butuh file Swift.

### Opsi 3: Memilih nilai di tiap tampilan lewat `colorScheme`

Token tetap konstanta, tetapi tiap tampilan membaca `@Environment(\.colorScheme)` dan memilih nilai terang atau gelap sendiri.

**Pros**:

* SwiftUI murni, tanpa AppKit.
* Mudah dimengerti saat dibaca.

**Cons**:

* Setiap tampilan harus ingat membaca lingkungan dan memilih nilai. Yang lupa akan salah di satu tampilan tanpa ada yang gagal.
* Pemilihan nilai tidak bisa diuji tanpa menjalankan tampilan.
* Tampilan kontras tinggi tidak tercakup tanpa kode tambahan di tiap tempat.

### Opsi 4: Hanya warna sistem

Jendela memakai warna semantik macOS (`windowBackgroundColor`, `labelColor`) dan token hanya dipakai untuk jarak.

**Pros**:

* Paling native, dan selalu ikut setiap pengaturan aksesibilitas.
* Hampir tanpa kode.

**Cons**:

* Warna dari sumber bersama tidak sampai ke layar, jadi kriteria 16 dan Done when scope 008 tidak terpenuhi untuk warna.
* Tiga OS tidak lagi punya arti warna yang sama.

### Keputusan yang lebih kecil

| No. | Pertanyaan | Diambil | Pilihan kedua | Tanda | Yang berubah bila pilihan kedua |
| --- | --- | --- | --- | --- | --- |
| 1 | Cara warna mengikuti tampilan | `NSColor` dinamis di kode | Asset catalog hasil generator | berat | Emitter, **Tata letak**, AC-4, AC-9, dan `project.pbxproj` |
| 2 | Tempat modul | Target `DesignSystem` di `apps/macos/Package.swift` | Paket Swift tersendiri di luar `apps/macos/` | biasa | `Package.swift`, AC-9, path keluaran |
| 3 | Bentuk keluaran | Satu file, `enum Tokens` dengan `Colors`, `Typography`, `Spacing` | Satu file per jenis token | biasa | `swift.rs`, AC-1, `.gitattributes` |
| 4 | Nama yang sama dengan kata kunci Swift | Daftar kata kunci, backtick hanya bila perlu | Backtick pada setiap nama | biasa | `identifier` di `swift.rs`, blok `Tokens.swift` |
| 5 | Penerapan `accent` | `.tint` dan `.buttonStyle(.borderedProminent)` pada tombol `Greet` | `.tint` pada seluruh tampilan, atau `AccentColor` di asset catalog | biasa | `GreetingView.swift`, AC-6, langkah manual 2 |
| 6 | Penerapan `background` | `.background` pada isi jendela | Mengatur `NSWindow.backgroundColor` dari kode `App` | biasa | `GreetingView.swift`, `DesktopMacApp.swift`, AC-9 |
| 7 | Huruf untuk peran | `Font.system(style).weight(weight)` | Ukuran tetap dalam point | biasa | `TextRole`, AC-6, K-6 |
| 8 | Mode Increase contrast | Token tetap dipakai | Kembali ke warna sistem saat mode itu aktif | biasa | `ThemeColor.nsColor`, uji `colorFollowsTheAppearance`, langkah manual 10 |
| 9 | Uji tampilan | Uji nilai warna lewat tampilan AppKit, tanpa gambar | Uji cuplikan gambar jendela | biasa | `ThemeTests`, dependensi baru |
| 10 | Uji emitter | Berkas `tokens_swift_contract.rs` dengan token set buatan tangan | Hanya uji `committed_outputs_match_the_source` | biasa | AC-2 |
| 11 | Lebar minimum `320` | Tetap literal di `GreetingView.swift` | Konstanta bernama di `DesignSystem` | biasa | `GreetingView.swift`, AC-3 |
| 12 | Akhir baris keluaran | Baris `.gitattributes` untuk `Tokens.swift` saja | Pola `*.swift` untuk seluruh folder | biasa | `.gitattributes`, AC-9 |

Alasan singkat:

* **2.** Paket yang sama berarti `swift test` yang sama dan tidak ada perubahan proyek Xcode. Paket kedua hanya menambah satu `Package.swift` dan satu rujukan paket.
* **3.** Enam konstanta tidak butuh tiga file, dan satu file berarti satu baris `.gitattributes`.
* **4.** File ini dibaca orang saat memeriksa warna di langkah manual. Backtick di setiap nama membuatnya lebih sulit dibaca, dan daftar kata kunci hanya beberapa baris.
* **5.** Deskripsi token `accent` adalah "isi kontrol tindakan utama". Memasangnya pada tombol saja membuat cincin fokus dan warna pilihan teks tetap mengikuti aksen sistem (K-6). Gaya `borderedProminent` adalah gaya sistem untuk tombol yang diisi warna, jadi kontrolnya tetap asli. `AccentColor` di asset catalog membawa kembali masalah Opsi 2.
* **6.** `DesktopMacApp.swift` tidak tersentuh, jadi tidak bertabrakan dengan spec macOS scope 009, dan tampilan tetap bisa dipakai di jendela mana pun.
* **7.** Ukuran tetap melanggar K-6. Gaya teks sistem adalah satu satunya jalur yang ikut pengaturan ukuran teks, sejauh macOS menerapkannya.
* **8.** Increase contrast di macOS mempertegas garis dan kontras kontrol, tetapi tidak mengganti palet seperti tema kontras Windows. Kontras teks utama terhadap latar sudah sekitar 18 banding 1. Kalimat K-6 tentang palet sistem berlaku untuk mode yang mengganti palet.
* **9.** Uji cuplikan butuh jendela atau penggambar di luar layar dan mudah goyah antar versi macOS. Nilai warna per tampilan adalah yang benar benar diputuskan spec ini.
* **10.** Sumber asli hanya memakai bobot `regular` dan tidak punya nama kata kunci. Tanpa token set buatan tangan, cabang itu tidak pernah berjalan.
* **11.** Spec `_root/0003` menyebut angka itu konstanta milik OS. Memindahkannya ke `DesignSystem` membuatnya tampak seperti token.
* **12.** Hanya file hasil generator yang dibandingkan byte demi byte di host lain. File Swift tulisan tangan tidak perlu aturan itu.

## Rationale

Opsi 1 dipilih karena dua gaya dari Context. Pertama, sesi koordinasi tidak boleh membuka jendela, jadi nilai keputusan diukur dari berapa banyak yang terbukti lewat `swift test`. Dengan warna dinamis di kode, pemetaan terang dan gelap setiap token, termasuk tampilan kontras tinggi, terbukti di uji yang berjalan sepersekian detik. Opsi 2 memindahkan bukti itu ke sesi GUI, dan Opsi 3 menyebarkannya ke setiap tampilan tanpa bisa diuji. Kedua, spec macOS scope 009 akan mengubah `project.pbxproj` dan `DesktopMacApp.swift`; Opsi 1 tidak menyentuh keduanya, jadi lead tidak perlu menggabung dua perubahan pada file proyek tulisan tangan. Opsi 4 ditolak karena tidak memenuhi apa yang diminta scope ini.

Harga Opsi 1 adalah ketergantungan pada AppKit dan dua tipe tulisan tangan. Keduanya kecil: aplikasi ini memang hanya untuk macOS, dan dua tipe itu berjumlah sekitar 55 baris dengan lima uji.

Yang tetap tidak bisa dibuktikan tanpa jendela adalah apa yang benar benar digambar: warna latar dan tombol di layar, cincin fokus, ukuran teks, dan VoiceOver. Itu dijadikan sepuluh langkah manual dengan hasil yang bisa diputuskan ya atau tidak, dan nilai yang diharapkan dibaca dari file hasil generator.

## Bukti percobaan

Semua dicoba pada 2026-11-15 di Mac koordinasi (macOS 27.0.1, `arm64`, Xcode 27.1 build `27A9275`, Swift 6.4, `rustc 1.99.1`), di salinan repo di folder sementara yang sudah memuat purwarupa `desktop-codegen` dari spec `_root/0003`. Setiap blok di **Isi file** disalin dari purwarupa yang lulus, bukan diketik ulang.

| Yang dicoba | Hasil |
| --- | --- |
| `cargo run -q -p desktop-codegen --locked -- tokens generate` sesudah emitter ditambahkan | `wrote apps/macos/DesignSystem/Tokens.swift`, lalu `tokens: ok, 6 semantic tokens, 3 placeholder values, 4 outputs, 1 written` |
| `tokens check` sesudahnya, lalu `tokens generate` lagi | `4 outputs up to date`, lalu `4 outputs, 0 written` |
| `cargo test -p desktop-codegen --locked` | `tokens_contract` 5 passed (dengan perubahan `OUTPUTS`), `tokens_swift_contract` 2 passed |
| `cargo test --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo fmt --all --check` | Semua kode 0, termasuk `macos_bridge_ownership` dengan file Swift baru |
| `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin`, lalu `swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"` | Tiga baris ringkasan: `4 tests in 1 suite passed`, `7 tests in 1 suite passed`, dan `5 tests in 1 suite passed` (suite `ThemeTests`) |
| `swift build` dengan flag yang sama, dihitung baris `warning` | `0`, jadi mode bahasa Swift 6 menerima `ThemeColor` dan `TextRole` sebagai `Sendable` |
| `xcodebuild` Debug dan Release dengan perintah AC-5 | `** BUILD SUCCEEDED **` untuk keduanya, `lipo -archs` mencetak `arm64` |
| Pemeriksaan teks AC-3 dan AC-6 pada `GreetingView.swift` purwarupa | Enam token masing masing sekali; tiga `grep` larangan tidak mencetak apa pun |
| `grep -c '_ =>' tooling/codegen/src/tokens/swift.rs` | `0` |
| `git ls-files --eol` untuk `Tokens.swift` dengan baris `.gitattributes` baru | `i/lf w/lf attr/text eol=lf`; `Theme.swift` tanpa atribut |
| Jumlah file di `apps/macos`, `bindings/swift`, `crates/macos-ffi` | 19 |

Pengulangan pada kode nyata, 2026-11-15, sesudah sesi berpindah ke `claude-opus-5-5`: salinan baru `main` di `705b135` (memuat paket `desktop-codegen` nyata dari milestone 1 spec `_root/0003`, sumber `config` dari spec `_root/0004`, dan spec `macos/0004` yang belum dibangun). Perubahan `mod.rs` dan `tokens_contract.rs` di **Isi file** ditulis ulang untuk kode nyata, lalu diterapkan persis seperti tertulis.

| Yang dicoba | Hasil |
| --- | --- |
| `tokens generate`, lalu `tokens check`, lalu `config check` | `wrote apps/macos/DesignSystem/Tokens.swift` dan `4 outputs, 1 written`; `4 outputs up to date`; `config: ok, ..., 1 outputs up to date` |
| `Tokens.swift` hasil kode nyata dibanding blok di **Isi file** | Sama byte demi byte |
| `cargo test --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo fmt --all --check` | Semua kode 0; `config_contract` 33 passed, `tokens_contract` 5 passed, `tokens_swift_contract` 2 passed, `macos_bridge_ownership` 4 passed, `workspace_layout` 10 passed |
| `swift test`, `swift build` (baris `warning`), `xcodebuild` Debug dan Release | `4 tests`, `7 tests`, dan `5 tests in 1 suite passed`; `0` baris `warning`; `** BUILD SUCCEEDED **` dua kali, `lipo -archs` mencetak `arm64` |

Catatan dari percobaan:

* Swift Testing mencetak satu baris ringkasan per target uji, jadi baris `7 tests` dan `4 tests` dari spec `macos/0001` tetap muncul apa adanya.
* `NSAppearance.performAsCurrentDrawingAppearance` cukup untuk membaca nilai warna dinamis di `swift test`, tanpa jendela.
* `Font` bisa dibandingkan dengan `==`, jadi `TextRole.font` teruji tanpa menggambar teks.
* Yang tidak dicoba, karena butuh jendela: apakah `.tint` benar benar mewarnai tombol `borderedProminent` di macOS 27, dan bagaimana ukuran teks sistem memengaruhi jendela ini. Keduanya menjadi langkah manual 2 dan 8, dengan baris kasus gagal bila hasilnya berbeda.

## Cross check

Draf dibaca oleh subagent baru berkonteks bersih (Claude Opus, hanya baca) pada 2026-11-15. Sebagian besar spec ditulis Claude Fable, lalu diselesaikan Claude Opus sesudah sesi dipindah karena batas pemakaian, jadi pemeriksa sefamily dengan penulis bagian akhir. Mode keputusan `auto`, jadi perbaikan yang direkomendasikan langsung diterapkan, dan blok kode yang berubah diuji ulang pada salinan `main` nyata.

Diterapkan:

* Rencana verifikasi manual: Accent color sistem diganti Orange sebelum langkah 2, supaya `.tint` yang tidak bekerja bisa terlihat; pengukuran Digital Color Meter wajib dan diulang di Dark; jendela diaktifkan sebelum warna tombol dinilai; bilah judul dikecualikan; langkah 5, 7, dan 9 memakai nama baru supaya hasilnya teramati; langkah 6 menyebut tampilannya.
* Baris kasus gagal baru untuk jendela yang tidak aktif.
* AC-5 menghapus `apps/macos/.build` sebelum menghitung `warning`, supaya build tidak terambil dari cache.
* AC-3 memakai pola larangan yang lebih ketat (`Color.`, `.tint(.`, `.background(.`, `.bold(`, `padding(.x, n)`, `minLength:`) dan perintah untuk "satu satunya angka". Pola itu diperiksa terhadap `GreetingView.swift` dan kedua versi `DesktopMacApp.swift`.
* AC-1 mendapat perintah `awk | diff` untuk membandingkan `Tokens.swift` dengan bloknya; AC-9 mendapat perintah hitungan relatif dan waktu pemeriksaan ulang `git log`; AC-11 mencatat versi SDK.
* Langkah 1 Build plan menjalankan gerbang `cargo` workspace sebelum commit.
* `precedencegroup` masuk daftar kata kunci (53); catatan bahwa `self`, `init`, `Type`, dan `Protocol` tetap butuh backtick di tempat pakai.
* Pernyataan ketiga `textRoleUsesTheSystemTextStyle` yang tautologis diganti `Tokens.Typography.body == TextRole(style: .body, weight: .regular)`.
* Dua komentar dokumen yang akan usang (`plan` di `tokens/mod.rs` dan contoh baris `check` di `lib.rs`) ikut diubah; `lib.rs` masuk daftar file milestone 1.
* `.graphifyignore` diizinkan di setiap milestone untuk kasus hook Graphify, dan dokumen platform serta arsitektur ditulis tegas sebagai milik lead lewat `/sync`.
* Follow-up baru: AC-11 spec `macos/0004` mengunci 17 path, yang bertabrakan dengan spec ini apa pun urutan build-nya.
* Rujukan skenario tipografi diperbaiki ke AC-4.

Tidak diterapkan:

* Uji `everyColorTokenResolvesInBothAppearances` tetap, walau membandingkan dengan `channels()` yang sama. Nilainya dikunci `tokens check`, dan uji itu tetap membuktikan setiap token warna terurai di kedua tampilan.

Pemeriksa menyatakan kriteria 16, 17, 32 dan bagian macOS dari Done when tercakup, K-1 sampai K-9 terjawab, setiap hitungan dan `grep` cocok dengan blok kode, dan perubahan `mod.rs` serta `tokens_contract.rs` cocok dengan kode nyata di `main`.
