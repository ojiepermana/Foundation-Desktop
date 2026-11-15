# 0006. Satu contoh lokalisasi di macOS: catatan keputusan

Spec yang dibangun ada di [index.md](index.md). Berkas ini menyimpan alasannya: masalah, pilihan yang ditimbang, daftar keputusan kecil beserta pilihan keduanya, dan bukti percobaan.

## Context

Spec [`_root/0006`](../../_root/0006-satu-contoh-lokalisasi/index.md) sudah menetapkan katalog bahasa bersama, aturannya (L-1 sampai L-12), dan sembilan kewajiban yang tiap OS buktikan sendiri (OS-1 sampai OS-9). Katalog itu sudah ada di `main` dengan satu kunci, `greeting.submit`, dalam bahasa `en` (`Greet`) dan `id` (`Sapa`). Aplikasi macOS dari spec [`macos/0001`](../0001-sapaan-di-jendela-macos/index.md) masih menulis `Greet` sebagai literal di `GreetingView.swift`, dan belum punya resource bahasa, `defaultLocalization`, atau berkas `.lproj`.

Yang harus diputuskan khusus macOS ada empat: format resource Apple dan lokasinya, cara SwiftUI menampilkan teks katalog sebagai teks biasa, cara aplikasi memilih bahasa, dan bukti mana yang bisa berjalan tanpa jendela. Ada beberapa gaya yang saling tarik:

* **Sesi koordinasi tidak boleh membuka jendela.** Bukti teks per bahasa harus bisa dijalankan `swift test` dan `xcodebuild`. Pergantian bahasa di jendela nyata milik user.
* **Aturan keluaran yang sudah dikunci uji.** `every_output_follows_the_output_rules` di `tooling/codegen/tests/localization_rules.rs` menuntut penanda O-1 di tiga baris pertama, folder `shared/localization/` atau `apps/`, akhir baris `\n`, dan satu baris `text eol=lf` per jenis berkas di `.gitattributes`. Format yang dipilih harus memenuhi itu tanpa mengubah uji.
* **SwiftUI menafsirkan literal string.** `Button("...")` dan `Text("...")` dengan literal memperlakukan teks sebagai `LocalizedStringKey`: SwiftUI mencarinya lagi di bundel utama dan menafsirkan Markdown. L-6 dan OS-6 melarang dua hal itu.
* **Berkas bersama satu penulis per wave.** `project.pbxproj`, `Package.swift`, `GreetingView.swift`, `.gitattributes`, dan `tooling/codegen/src/lib.rs` juga disentuh build spec `macos/0004` (konfigurasi), `macos/0005` (tema), dan spec macOS scope 011. Setiap berkas bersama yang disentuh spec ini menambah urutan yang harus diatur lead.
* **Kriteria 32.** Hanya yang dibutuhkan satu contoh teks.

Bila tidak diputuskan, `/develop` harus memilih format, lokasi, dan cara membaca sendiri, dan pilihan yang salah (misalnya literal `LocalizedStringKey`) lolos semua uji yang ada sekarang tetapi melanggar L-6 diam diam.

## Options considered

### Opsi 1: Berkas `.strings` per bahasa sebagai resource modul `Presentation`

Emitter menulis `apps/macos/Presentation/Resources/<locale>.lproj/Localizable.strings`. Paket Swift lokal menyatakan folder itu sebagai resource target `Presentation` dan `defaultLocalization: "en"`. Kode Swift memilih bahasa dengan `Bundle.preferredLocalizations`, membaca teks dari folder `.lproj` itu, dan memberikannya ke tombol sebagai `String`.

**Pros**:

* `swift test` membaca resource yang sama dengan yang dibawa aplikasi, lewat `Bundle.module`, jadi teks per bahasa terbukti tanpa jendela.
* Penanda O-1 ditulis dengan komentar `/* ... */` yang memang bagian format `.strings`.
* `project.pbxproj` tidak berubah. Xcode menyalin bundel resource paket ke aplikasi sendiri (terbukti di prototipe).
* Satu berkas per bahasa, baris demi baris, mudah dibaca di diff.

**Cons**:

* `.strings` adalah format lama. Xcode modern mendorong String Catalog, dan editor terjemahan Xcode tidak dipakai.
* Bundel utama aplikasi tidak memuat `.lproj`, jadi macOS menganggap aplikasi berbahasa Inggris untuk menu bawaannya.
* Kunci harus ditulis ulang di Swift (`TextKey`).

### Opsi 2: Satu String Catalog `.xcstrings` sebagai resource modul `Presentation`

Emitter menulis satu berkas `Localizable.xcstrings` (JSON milik Apple) yang memuat semua bahasa, dengan kunci `$comment` sebagai penanda di baris kedua. SwiftPM dan Xcode mengompilasinya menjadi `.lproj` saat build.

**Pros**:

* Format yang dianjurkan Apple sekarang, satu berkas untuk semua bahasa, sama bentuknya dengan katalog bersama.
* `xcstringstool compile` menerima kunci `$comment` tambahan (terbukti di prototipe), jadi O-1 bisa dipenuhi.

**Cons**:

* Penanda bergantung pada kelonggaran alat Apple terhadap kunci yang tidak dikenal. Kelonggaran itu tidak tertulis, jadi bisa hilang di versi Xcode berikutnya.
* Xcode memperlakukan `.xcstrings` sebagai berkas yang ia sunting sendiri (status terjemahan, kunci hasil ekstraksi). Membuka proyek di Xcode berisiko mengubah berkas hasil generator dan membuat `git status` kotor. Risiko ini belum dicoba di jendela nyata.
* Isinya JSON, tetapi ekstensinya bukan `.json`, jadi pemeriksaan JSON kanonis O-4 di uji aturan tidak berjalan atasnya.

### Opsi 3: Resource `.lproj` di target aplikasi Xcode

Emitter menulis `apps/macos/Resources/<locale>.lproj/Localizable.strings` (folder `Resources` di tingkat atas dari rujukan macOS), dan proyek Xcode mendapat grup, referensi berkas, dan fase salin resource untuk folder itu. Teks dibaca dari `Bundle.main`.

**Pros**:

* Bundel utama aplikasi sendiri terlokalisasi, jadi menu bawaan macOS dan pilihan bahasa per aplikasi di System Settings ikut bahasa Indonesia.
* Mengikuti folder `Resources` di rujukan macOS apa adanya.

**Cons**:

* `swift test` tidak bisa melihat `Bundle.main` aplikasi, jadi teks per bahasa hanya terbukti lewat `xcodebuild` dan pemeriksaan berkas, atau di jendela.
* Mengubah `project.pbxproj`, berkas bersama yang juga diubah spec `macos/0004`, dengan ID objek yang ditulis tangan. Spec `macos/0007` (scope 011) berencana menambah grup tersinkron untuk `apps/macos/Resources/`. Bila grup itu sudah ada, folder `.lproj` di sana ikut masuk bundel tanpa perubahan `project.pbxproj` lagi, tetapi spec ini lalu bergantung pada urutan build spec itu, dan kelemahan pertama tetap ada.

### Opsi 4: Teks katalog sebagai kode Swift hasil generator

Emitter menulis satu berkas Swift berisi tabel teks per bahasa (misalnya `switch` atas kunci dan bahasa). Tidak ada resource Apple.

**Pros**:

* Kunci yang hilang gagal dikompilasi, dan tidak ada bundel resource yang bisa hilang saat berjalan.
* Paling mudah diuji, karena semuanya kode.

**Cons**:

* Bukan "resource lokalisasi Apple" seperti yang ditulis [Resource bersama](../../../shared-resources/README.md) dan tabel target spec `_root/0006`.
* Negosiasi bahasa Apple dan mekanisme bundel tidak dipakai, jadi L-9 per pengguna harus ditulis tangan.
* Menambah keluaran `.swift` di `apps/macos/` dan baris `.gitattributes` untuk Swift, wilayah yang juga dipakai emitter tema spec `macos/0005`.

## Rationale

Opsi 1 dipilih karena hanya opsi ini yang memenuhi gaya pertama dan keempat bersama. Bukti teks per bahasa harus berjalan tanpa jendela, dan Opsi 1 membuat `swift test` membaca resource yang persis sama dengan yang Xcode salin ke aplikasi, lewat `Bundle.module`. Opsi 3 kehilangan bukti itu. Pada saat yang sama, Opsi 1 tidak menyentuh `project.pbxproj` atau `lib.rs`, jadi spec ini hanya bersaing dengan build lain di `Package.swift`, `GreetingView.swift`, dan `.gitattributes`. Opsi 3 menambah satu lagi berkas bersama yang paling rapuh, karena ditulis tangan dengan ID objek.

`.strings` dipilih di atas `.xcstrings` karena penanda O-1 menjadi bagian sah dari format, bukan kunci tambahan yang kebetulan diterima alat Apple. Alasan kedua, berkas hasil generator tidak boleh disunting oleh siapa pun selain generator, dan Xcode tidak memperlakukan `.strings` sebagai katalog yang ia kelola. Harga yang diterima: format yang lebih tua dan tanpa editor terjemahan Xcode. Itu cocok untuk alur ini, karena tempat menyunting teks adalah `catalog.json`, bukan Xcode.

Untuk teks biasa (OS-6), tombol menerima `String`, sehingga SwiftUI memakai init `StringProtocol` yang tidak mencari kunci dan tidak menafsirkan Markdown. Untuk pemilihan bahasa, negosiasi Apple sudah memberi dua hasil minimum L-9 (`id-ID` memakai `id`, bahasa yang tidak didukung memakai `en`), jadi tidak ada algoritme bahasa yang ditulis tangan. Kode yang memilih bahasa dan kode yang diuji adalah fungsi yang sama, `LocalizedText.language(for:)`, dengan daftar pilihan dari `Locale.preferredLanguages` saat aplikasi berjalan dan daftar tetap di uji.

### Keputusan yang lebih kecil

Setiap keputusan bertanda `biasa`, karena mode keputusan `auto` dan pilihan keduanya bisa diterapkan lewat pembaruan spec tanpa mengubah katalog atau aturan bersama.

1. **Format resource**: `.strings` per bahasa. Pilihan kedua: `.xcstrings` (Opsi 2). Bila dipilih, emitter menulis satu berkas JSON dengan `$comment`, AC-1, AC-2, AC-8, dan baris `.gitattributes` berubah.
2. **Lokasi**: resource target `Presentation` di `apps/macos/Presentation/Resources/`. Pilihan kedua: target aplikasi di `apps/macos/Resources/` (Opsi 3). Bila dipilih, `project.pbxproj` berubah, AC-4 pindah dari `swift test` ke pemeriksaan berkas, dan AC-10 berubah.
3. **Cara membaca**: `Bundle.localizedString(forKey:value:table:)` pada folder `.lproj` yang dibuka langsung, lalu `String(format:arguments:)` tanpa argumen. Pilihan kedua: `String(localized:bundle:)`. Itu ditolak karena teks dengan `%` akan diperlakukan sebagai pola format bila kelak memakai interpolasi, dan karena uji tidak bisa memaksa bahasa tertentu tanpa mengganti bahasa proses.
4. **Pemilihan bahasa**: negosiasi Apple, `Bundle.preferredLocalizations(from:forPreferences:)`, dengan bahasa sumber ditambahkan di akhir daftar pilihan. Pilihan kedua: langkah L-9 ditulis sendiri di Swift (cocok persis, potong wilayah, lalu `en`). Itu tidak perlu, karena prototipe menunjukkan Apple memberi hasil yang sama untuk semua contoh OS-4, dan L-9 mengizinkan negosiasi OS. Tambahan bahasa sumber di akhir dibutuhkan karena, tanpa itu, hasil Apple untuk pilihan yang tidak cocok jatuh ke daftar bahasa host, bukan ke `defaultLocalization`.
5. **Kunci di Swift**: enum `TextKey` tulisan tangan, dijaga uji `everyKeyHasTextInEveryLanguage`. Pilihan kedua: berkas Swift kunci hasil generator. Itu menambah keluaran `.swift` dan baris `.gitattributes` untuk Swift di `apps/macos/`, wilayah emitter tema spec `macos/0005`, untuk satu kunci saja. Bila jumlah kunci tumbuh, pilihan kedua layak ditimbang ulang.
6. **Peloloskan dan placeholder**: `\` menjadi `\\`, `"` menjadi `\"`, `%` menjadi `%%`, `{nama}` menjadi `%<n>$@`. Pilihan kedua: `%` tidak diloloskan dan `String(format:)` tidak dipakai. Itu gagal begitu teks pertama berplaceholder masuk, karena `%` di teks yang sama lalu terbaca sebagai pola format.
7. **Nama folder bahasa**: nama locale katalog apa adanya (`pt-BR.lproj`). Pilihan kedua: garis bawah ala Apple lama (`pt_BR.lproj`). Apple menerima keduanya, dan identitas lebih mudah diperiksa sebagai satu ke satu (OS-8).
8. **Pengenal teks**: kunci dengan titik apa adanya. Pilihan kedua: titik diganti `_` (L-11 butir 2). `.strings` menerima titik, jadi penggantian tidak perlu.
9. **Proyek Xcode**: tidak diubah, `knownRegions` tetap `en` dan `Base`. Pilihan kedua: menambah `id` ke `knownRegions`. Itu hanya metadata proyek, tidak mengubah perilaku saat berjalan, dan menyentuh berkas bersama.
10. **Penanda terjemahan yang hilang**: komentar `Missing translation` di atas pasangan yang berasal `Fallback`. Pilihan kedua: tanpa komentar. Komentar itu membantu tinjauan diff dan tidak mengubah perilaku.
11. **API placeholder di Swift**: belum dibuat. Pilihan kedua: `string(_:_ values: String...)` sekarang. Kriteria 32 menolak API tanpa pemakai. Emitter sudah menulis bentuk native, jadi menambah API nanti tidak mengubah resource.

## Bukti percobaan

Semua dicoba pada 2026-11-15 di Mac koordinasi (macOS 27.0.1, `arm64`, Xcode 27.1, Swift 6.4), di folder scratchpad sesi, bukan di repo.

| Yang dicoba | Hasil |
| --- | --- |
| Paket Swift kecil dengan `defaultLocalization: "en"` dan `resources: [.process("Resources")]`, berisi `en.lproj` dan `id.lproj` dengan komentar penanda dua baris di awal | `swift test` lulus. `Bundle.module.localizations` adalah `en`, `id`, dan `developmentLocalization` adalah `en`. Komentar tidak mengganggu pembacaan. |
| `Bundle.preferredLocalizations(from: ["en", "id"], forPreferences: ...)` | `en` memberi `en`, `id` memberi `id`, `id-ID` memberi `id`, `fr` memberi `en`, `ms` memberi `en`, `fr, id` memberi `id`, `en-ID` memberi `en`, `zh-Hans` memberi `en`, daftar kosong memberi `en` |
| `Bundle.preferredLocalizations(from: ["ar", "en", "id"], forPreferences: ...)` dari skrip Swift di host berbahasa Inggris | `fr` memberi `en` dan daftar kosong memberi `en`, walaupun `ar` ada di urutan pertama `from:` dan tidak ada `defaultLocalization`. Hasil itu datang dari daftar bahasa host. Dengan `fr` lalu `en` di daftar pilihan, hasilnya `en` secara pasti, dan `id-ID` lalu `en` tetap memberi `id`. Karena itu `language(for:)` menambahkan bahasa sumber di akhir. |
| `String(format: "%1$@ x", arguments: [])` | `(null) x`, bukan crash. Karena itu `TextKey` belum boleh memuat kunci berplaceholder, dan `everyKeyHasTextInEveryLanguage` menolak `(null)`. |
| `localizedString(forKey: "no.such", ...)` | Mengembalikan `no.such`, kunci mentah. Karena itu OS-5 dijaga uji, bukan oleh perilaku bundel. |
| Teks `100%% *selesai* <b> & \"q\"` lewat `String(format:arguments: [])` | `100% *selesai* <b> & "q"`. `%2$@ and %1$@` dengan `Ayu`, `App` memberi `App and Ayu`. |
| `xcrun xcstringstool compile` atas `.xcstrings` biasa, lalu atas `.xcstrings` dengan kunci `$comment` di awal | Keduanya kode 0 dan menulis `en.lproj` dan `id.lproj` |
| Clone repo sekali pakai pada `d7885e6` dengan perubahan persis seperti **Isi berkas** (tanpa uji `percentSignsStayPlain`), lalu `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin` dan `swift test` | Kode 0. `PresentationTests` mencetak `Test run with 9 tests in 2 suites passed`, `InteropTests` tetap `Test run with 7 tests in 1 suite passed`. Dengan uji keenam, angkanya menjadi `10 tests in 2 suites`. |
| `xcodebuild ... -configuration Debug ... build` pada clone yang sama, tanpa perubahan `project.pbxproj` | `** BUILD SUCCEEDED **`. Aplikasi memuat `Contents/Resources/DesktopMacModules_Presentation.bundle/Contents/Resources/en.lproj/Localizable.strings` dan `id.lproj/...`, tersalin byte demi byte. `plutil -convert json -o -` mencetak `{"greeting.submit":"Greet"}` dan `{"greeting.submit":"Sapa"}`. Satu satunya peringatan adalah pilihan destination, yang sudah ada sebelum perubahan ini. |
| Kode Swift dan berkas `.strings` final yang diambil langsung dari blok **Isi berkas** `index.md` (sesudah perbaikan cross check), lalu `swift test` | Kode 0, `Test run with 10 tests in 2 suites passed` dan `Test run with 7 tests in 1 suite passed`. Pencarian **AC-3**, **AC-6**, dan **AC-7** memberi hasil yang tertulis. |
| `plutil -extract greeting.submit raw` | Gagal, karena `plutil` membaca titik sebagai pemisah path. Karena itu AC-8 memakai `plutil -convert json`. |
| `git status --short` sesudah `swift test` dan `xcodebuild` | Hanya perubahan sumber yang disengaja. Build tidak menulis berkas lain yang dilacak. |

Yang belum dicoba: build Release, emitter Rust (ditulis saat build), teks non ASCII lewat `.process` SwiftPM dan salinan Xcode (teks sekarang ASCII saja; uji `m04_` hanya membuktikan byte keluaran emitter), opsi `-AppleLanguages` di langkah manual, dan semua langkah jendela.

## Cross check

Draf spec dibaca oleh subagen hanya baca dengan model Claude Sonnet 5.5 pada 2026-11-15, dengan tugas utama mencari nilai dan keputusan yang belum tertulis. Penulis spec adalah Claude Opus 5.5, jadi pemeriksa adalah model yang berbeda tetapi dari family yang sama, karena Fable tidak boleh dipakai sampai batas pemakaiannya pulih. Mode keputusan `auto`, jadi perbaikan yang direkomendasikan langsung diterapkan.

Diterapkan:

* `language(for:)` menambahkan bahasa sumber di akhir daftar pilihan. Pemeriksa menduga hasil Apple jatuh ke elemen pertama `from:`. Percobaan menunjukkan hasilnya jatuh ke daftar bahasa host, dan perbaikannya sama.
* `showsTheTextForEachLanguage` juga memeriksa bahasa tanpa folder `.lproj` (`xx`), jadi cabang resource bahasa sumber teruji (OS-3).
* Cara menjalankan bukti gagal **AC-5** ditulis lengkap: worktree sekali pakai, baris yang ditambah, dan keluaran yang diharapkan.
* Separuh OS-6 tentang nilai placeholder dinyatakan ditunda, `TextKey` dibatasi ke kunci tanpa placeholder, dan uji menolak `(null)`.
* `m01_` menyimpan isi berkas sebagai literal. Berkas uji menulis sendiri `edited`, K-1, dan K-2 dengan teks yang sama dengan `localization_contract.rs`, dan suntingan `m06_` ditulis persis.
* Bentuk perubahan `plan` di `mod.rs` ditulis, bukan "dua baris".
* Penanda di `macos.rs` adalah salinan konstanta privat `schema.rs`, dan `m01_` serta `m07_` menjaga keduanya tetap sama.
* **AC-1** mendapat perintah untuk "satu baris baru" di `.gitattributes`. **AC-10** mendapat angka yang diharapkan dan `tr -d ' '`.
* Build Release disebut dingin, dan `export` variabel lingkungan berlaku juga untuk fase skrip `xcodebuild`.
* Uji `bundleListsTheCatalogLanguages` yang mengunci bahasa nyata diterima dengan sengaja dan alasannya ditulis di **AC-4**.
* Yang belum dicoba (teks non ASCII lewat Xcode, `-AppleLanguages`) dicatat di atas.

Tidak diterapkan: tidak ada. Pemeriksa tidak menemukan rancangan yang jauh lebih sederhana dan tidak menemukan masalah konkurensi Swift 6.
