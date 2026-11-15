# 0004. Konfigurasi publik di macOS: alasan

Catatan keputusan untuk [index.md](index.md). `/develop` tidak perlu membaca file ini.

## Context

Spec [_root/0004](../../_root/0004-konfigurasi-publik/index.md) menetapkan satu sumber untuk nilai publik bersama: `shared/config/public.json` untuk nama produk dan dasar pengenal aplikasi, dan `[workspace.package].version` di `Cargo.toml` root untuk versi. Aturan `K-2` di sana mewajibkan setiap salinan nilai itu dibuat generator atau dibandingkan pemeriksaan otomatis, dan menyerahkan pilihannya ke spec tiap OS. Aturan `K-6` mewajibkan pengenal aplikasi tiap OS diturunkan dari dasar pengenal dengan aturan tetap.

Aplikasi macOS adalah satu satunya konsumen yang ada, dan pada 2026-11-15 ia melanggar `K-2`:

| Nilai | Tempat di `apps/macos/` | Jumlah salinan tulis tangan |
| --- | --- | --- |
| Nama tampil `Desktop Foundation` | `INFOPLIST_KEY_CFBundleDisplayName` di `project.pbxproj` (Debug dan Release) dan literal `WindowGroup("Desktop Foundation")` di `App/DesktopMacApp.swift` | 3 |
| Pengenal bundel `com.example.desktop-foundation.macos` | `PRODUCT_BUNDLE_IDENTIFIER` di `project.pbxproj` (Debug dan Release) | 2 |
| Versi `0.1.0` | `MARKETING_VERSION` di `project.pbxproj` (Debug dan Release) | 2 |

Spec [macos/0001](../0001-sapaan-di-jendela-macos/index.md) mencatat ketiganya sebagai nilai sementara atau salinan tangan, dan menitipkan penyelesaiannya ke scope 009 sampai 012.

Gaya yang membentuk keputusan:

- **Xcode butuh nilai sebelum build mulai.** Pengenal bundel dan versi adalah build setting. Fase skrip berjalan sesudah setting itu dibaca, jadi skrip tidak bisa mengisinya.
- **Info.plist dibuat Xcode.** Proyek memakai `GENERATE_INFOPLIST_FILE = YES`, jadi nama tampil juga masuk lewat build setting (`INFOPLIST_KEY_CFBundleDisplayName`).
- **Alat sudah ada.** Sumber `config` di paket `desktop-codegen` (spec `_root/0003` dan `_root/0004`) memvalidasi `public.json`, menulis keluaran yang dilacak git, dan membandingkannya lewat `check`. Uji `committed_outputs_match_the_source` menjalankan `check` di `cargo test --workspace --locked`.
- **Nama berkas bundel dipakai banyak bukti.** `PRODUCT_NAME` sekarang `$(TARGET_NAME)`, yaitu `DesktopMac`. Path `DesktopMac.app/Contents/MacOS/DesktopMac` muncul di AC-4, AC-10, dan rencana verifikasi manual spec `macos/0001`.
- **Bukti sebisa mungkin tanpa jendela.** Sesi koordinasi boleh menjalankan `xcodebuild` dan membaca bundel, tetapi tidak boleh membuka jendela. Yang butuh mata manusia menjadi langkah manual milik user.
- **Spec `macos/0001` mengunci isi berkas.** Spec itu menulis `project.pbxproj` dan `DesktopMacApp.swift` byte demi byte dan menghitung tepat 16 path di tiga folder macOS (AC-12). Setiap perubahan di sini menuntut pembaruan spec itu lebih dulu.
- **Kriteria 32.** Tidak ada mekanisme untuk kebutuhan yang belum ada, misalnya timpaan lokal atau banyak environment.

Bila keputusan ini ditunda, mengganti nama produk atau pengenal tetap berarti menyunting tujuh baris dengan tangan, dan versi workspace bisa naik tanpa `MARKETING_VERSION` ikut naik.

## Options considered

### Opsi 1: Berkas xcconfig hasil generator sebagai dasar setelan target

Sumber `config` di `desktop-codegen` menulis satu berkas `apps/macos/Config/Shared.xcconfig` (berkas teks setelan build Xcode) yang berisi tiga setelan. Dua konfigurasi target menunjuknya lewat `baseConfigurationReference`, dan tiga setelan itu dihapus dari `project.pbxproj`. Judul jendela dibaca dari bundel saat aplikasi berjalan.

**Pros**:

- Tidak ada lagi salinan tulis tangan. Mengganti nilai adalah satu suntingan dan satu perintah `generate`.
- `check` yang sudah ada menjaga berkas itu di setiap host, tanpa Xcode.
- xcconfig adalah cara bawaan Xcode untuk setelan dari luar proyek.

**Cons**:

- Satu berkas dan satu folder baru di `apps/macos/`, dan `project.pbxproj` berubah di sepuluh baris.
- Sintaks xcconfig tidak bisa membawa `//`, rujukan `$(...)`, atau nilai yang diakhiri `;` atau `\` apa adanya, jadi nama produk seperti itu ditolak.
- Kode keluaran macOS tinggal di alat bersama, di luar `apps/macos/`.

### Opsi 2: Pemeriksaan teks atas `project.pbxproj`

Nilai tetap ditulis tangan di `project.pbxproj`. Satu pemeriksaan membaca berkas itu sebagai teks dan membandingkan tujuh baris dengan sumbernya.

**Pros**:

- `project.pbxproj` tidak berubah bentuk dan tidak ada berkas baru.
- Paling sedikit perubahan pada spec `macos/0001`.

**Cons**:

- Mengganti nilai tetap berarti menyunting sumber dan tujuh baris, lalu menunggu pemeriksaan gagal bila ada yang terlewat.
- Pemeriksaan harus mengerti format `project.pbxproj`, termasuk tanda kutip dan dua konfigurasi.
- Literal judul jendela tetap salinan ketiga.

### Opsi 3: Fase skrip yang membaca `public.json` saat build

Fase skrip Xcode membaca `shared/config/public.json` dengan `plutil` dan menulis nilai ke Info.plist hasil build.

**Pros**:

- Tidak ada berkas hasil generator yang dilacak.

**Cons**:

- Pengenal bundel dan versi tidak bisa diisi dari fase skrip, karena keduanya build setting.
- Build Xcode jadi membaca `shared/config/` langsung, tanpa lewat validasi.
- Hasilnya hanya terlihat sesudah build penuh, jadi tidak bisa dijaga di host tanpa Xcode.

### Opsi 4: `Info.plist` dan xcconfig, dua keluaran generator

Seperti Opsi 1, tetapi nama tampil ditulis ke berkas `Info.plist` hasil generator (XML, dengan escape yang jelas), dan hanya pengenal serta versi yang lewat xcconfig.

**Pros**:

- Setiap nama produk yang sah menurut skema bisa dibawa, termasuk yang ditolak xcconfig.

**Cons**:

- Dua keluaran dan satu setelan `INFOPLIST_FILE` baru, untuk kasus nama yang hampir tidak pernah ada.
- Info.plist menjadi gabungan berkas dan setelan, yang lebih sulit dibaca.

## Rationale

Opsi 1 dipilih karena ia satu satunya yang benar benar menghapus salinan tulis tangan, dan ia memakai alat yang sudah diputuskan spec `_root/0003` dan `_root/0004`. Pengenal bundel dan versi harus berupa build setting, jadi Opsi 3 gugur. Opsi 2 memenuhi `K-2` secara harfiah, tetapi cerita pengguna spec `_root/0004` adalah mengganti nilai di satu tempat, dan Opsi 2 tidak memberi itu.

Opsi 4 lebih lengkap, tetapi biayanya dibayar setiap hari untuk kasus yang hampir tidak mungkin: nama produk yang memuat `//` atau `$`, atau yang diakhiri `;` atau `\`. Lebih sederhana menolak nama seperti itu dengan pesan yang jelas, dan menambah keluaran kedua bila nama nyata pernah membutuhkannya.

Judul jendela dibaca dari bundel, bukan dari berkas Swift hasil generator, karena bundel sudah memuat nilainya dan satu keluaran lebih sedikit lebih baik. `PRODUCT_NAME` tidak disentuh karena mengubahnya mengganti nama berkas bundel dan mematahkan path di banyak bukti spec `macos/0001`, sedangkan judul jendela, tempat nama produk paling terlihat, sudah datang dari nama tampil.

Akhiran `.macos` dipertahankan. Itu aturan bawaan `K-6` (dasar ditambah satu segmen), nilainya sama dengan pengenal yang sudah dipakai, dan ia memberi tiap OS pengenal sendiri bila kelak dibutuhkan.

## Keputusan yang diambil

Mode keputusan brief adalah `auto`: tiap keputusan memakai opsi yang direkomendasikan, dan pilihan keduanya dicatat di sini. Keputusan spec `_root/0004` tidak diputuskan ulang.

| No. | Pertanyaan | Diambil | Pilihan kedua | Tanda |
| --- | --- | --- | --- | --- |
| 1 | Generator atau pemeriksaan untuk salinan macOS (`K-2`)? | Generator: `apps/macos/Config/Shared.xcconfig` sebagai keluaran kedua sumber `config`. | Pemeriksaan teks atas `project.pbxproj`. | biasa |
| 2 | Bagaimana Xcode memakai berkas itu? | `baseConfigurationReference` di dua konfigurasi target; tiga setelan dihapus dari `project.pbxproj`. | Fase skrip yang membaca `public.json`. | biasa |
| 3 | Nasib akhiran `.macos` (`K-6`)? | Dipertahankan: pengenal bundel adalah dasar pengenal ditambah `.macos`. | Tanpa akhiran. | biasa |
| 4 | Dari mana generator tahu versi workspace? | `env!("CARGO_PKG_VERSION")` milik `desktop-codegen`, yang mewarisi versi workspace. | Membaca `Cargo.toml` root sebagai teks. | biasa |
| 5 | Bentuk versi yang diterima untuk `MARKETING_VERSION`? | Tepat tiga bilangan dipisah titik, pilihan spec ini yang lebih ketat dari Apple. Bentuk lain ditolak (`M-2`). | Membuang akhiran pra rilis secara diam diam. | biasa |
| 6 | Dari mana judul jendela? | `CFBundleDisplayName` dibaca dari bundel saat berjalan; cadangannya nama proses. | Literal tetap sampai scope 010. | biasa |
| 7 | Apakah `PRODUCT_NAME` ikut nama produk? | Tidak. Tetap `$(TARGET_NAME)`, pengaturan khusus macOS. Nama di menu bar tetap `DesktopMac`. | `PRODUCT_NAME` diisi nama produk, sehingga bundel menjadi `Desktop Foundation.app`. | berat |
| 8 | Nama produk yang tidak bisa dibawa xcconfig? | Ditolak generator (`M-1`): nama yang memuat `//` atau `$`, atau diakhiri `;` atau `\`. | `Info.plist` hasil generator sebagai keluaran kedua. | biasa |
| 9 | Letak dan nama berkas? | `apps/macos/Config/Shared.xcconfig`, folder baru. | Di dalam `App/`, folder yang disinkronkan Xcode. | biasa |
| 10 | Letak kode keluaran macOS? | `tooling/codegen/src/config/macos.rs`, dengan uji di `tooling/codegen/tests/config_macos_contract.rs`. | Di dalam `tooling/codegen/src/config/mod.rs`. | biasa |
| 11 | Berkas native untuk pengaturan lokal? | Tidak dibuat. Belum ada kebutuhan. | `Local.xcconfig` yang diabaikan git dan disertakan dengan `#include?`. | biasa |
| 12 | Nomor build `CURRENT_PROJECT_VERSION`? | Tetap `1` di `project.pbxproj`, pengaturan khusus macOS. | Diturunkan dari versi workspace. | biasa |

Keputusan 7 bertanda `berat` karena hasilnya terlihat pengguna: menu bar menampilkan `DesktopMac`, bukan nama produk. Pilihan yang diambil paling murah dibalik (satu setelan), sedangkan pilihan kedua mengganti nama berkas bundel dan menuntut perubahan AC-4, AC-10, dan rencana manual spec `macos/0001`. Percobaan `INFOPLIST_KEY_CFBundleName` di "Bukti percobaan" menunjukkan apakah ada jalan tengah.

Akibat bila pilihan kedua yang diambil:

- Keputusan 1 dan 2: `Shared.xcconfig`, `config/macos.rs`, dan perubahan `project.pbxproj` hilang; diganti pemeriksaan yang membaca `project.pbxproj`, dan AC-1 sampai AC-3 berubah.
- Keputusan 3: baris `PRODUCT_BUNDLE_IDENTIFIER` kehilangan `.macos`, dan `N4` di file langkah manual sama dengan `N6`.
- Keputusan 6: `DesktopMacApp.swift` tidak berubah, AC-6 dan langkah manual hilang, dan literal itu menunggu scope 010.
- Keputusan 7: bundel menjadi `<nama produk>.app`; path di AC-4, AC-10, dan rencana manual spec `macos/0001` serta AC-4 spec ini berubah.
- Keputusan 8: keluaran kedua `apps/macos/Config/Info.plist`, setelan `INFOPLIST_FILE`, dan aturan `M-1` hilang.

## Bukti percobaan

Dicoba pada 2026-11-15 di salinan repo di folder sementara (macOS `27.0.1`, `arm64`, Xcode `27.1`, Rust `1.99.1`). Repo sendiri tidak diubah, dan salinan itu dihapus sesudahnya. `desktop-codegen` belum ada, jadi `Shared.xcconfig` ditulis tangan dengan isi yang sama dengan blok "Isi file".

| Yang dicoba | Hasil |
| --- | --- |
| `project.pbxproj` dengan perubahan di "Perubahan `project.pbxproj`", lalu perintah AC-3 untuk Debug dan Release | Mencetak `5` untuk keduanya |
| `git diff --stat` untuk `project.pbxproj` | 4 baris ditambah, 6 baris dihapus |
| `xcodebuild ... -configuration Debug ... build` dengan `DesktopMacApp.swift` dari "Isi file" | `** BUILD SUCCEEDED **`, tanpa peringatan dari berkas Swift itu |
| `plutil -extract` pada Info.plist bundel Debug | `CFBundleDisplayName` `Desktop Foundation`, `CFBundleIdentifier` `com.example.desktop-foundation.macos`, `CFBundleShortVersionString` `0.1.0`, `CFBundleVersion` `1`, `CFBundleName` `DesktopMac` |
| Baris `INFOPLIST_KEY_CFBundleName = Desktop Foundation` ditambahkan ke xcconfig, lalu build ulang | `CFBundleName` tetap `DesktopMac`. Xcode tidak memakai kunci itu, jadi nama di menu bar hanya bisa diubah lewat `PRODUCT_NAME` |
| Nama `A // B` di xcconfig | Setelan menjadi `A`. Sisanya dibaca sebagai komentar |
| Nama `A $(HOME) B` di xcconfig | Setelan menjadi `A /Users/... B`. Rujukan setelan diperluas |
| Nama `Name;` dan `Name ;` di xcconfig | Setelan menjadi `Name`. Satu `;` di akhir dibuang; `Name;;` menjadi `Name;` |
| Nama `Name\` di xcconfig | Setelan menjadi `Name MARKETING_VERSION = 0.1.0`. Garis miring terbalik di akhir menyambung baris berikutnya |
| Nama `Cost $5 Shop`, `"Quoted" Name`, `It's; here`, `A  B`, `A/B`, `A /* c */ B`, `A = B`, `Semi;colon`, `Back\slash`, `Hash # tag`, dan `Ünï 名前` | Semua dibawa apa adanya, termasuk tanda kutip dan dua spasi |
| Perintah AC-2, AC-6, AC-8, AC-9, dan hitungan path AC-11 pada salinan itu | `0`, `2`, satu path `Shared.xcconfig` untuk tiap `git grep`, `0`, `1`, tanpa keluaran, `3`, enam kali `2`, dan `17` |
| Blok `xcconfig`, blok `swift`, dan baris `+` serta `-` di tiga blok `diff` spec, dibandingkan dengan berkas di salinan itu | Sama |

Yang belum dicoba: fungsi `shared_xcconfig` dan uji X-1 sampai X-11, karena alatnya belum ada; build Release sampai bundel; jendela nyata (langkah manual); dan `WindowGroup` tanpa judul, yang mungkin sudah menampilkan nama aplikasi tetapi hanya bisa dilihat di jendela nyata. Judul eksplisit dipilih karena hasilnya tidak bergantung pada perilaku bawaan itu.

## Cross check

Dibaca oleh model lain (read only) pada 2026-11-15 sebelum spec ini diajukan. Rancangan dinilai utuh: sumber versi lewat `env!("CARGO_PKG_VERSION")`, kode judul jendela, tiga blok `diff`, dan perintah `sed`, `awk`, serta `grep` di AC dinilai benar. Temuan berikut diterapkan ke `index.md`. Mode keputusan `auto`, jadi tiap celah memakai perbaikan yang direkomendasikan.

| Temuan | Perbaikan |
| --- | --- |
| Langkah 1 mengklaim AC-2, padahal literal di `DesktopMacApp.swift` baru hilang di langkah 2 | AC-2 pindah ke langkah 2 |
| Versi workspace yang naik juga menuntut `Cargo.lock` diperbarui sebelum perintah `--locked` | Baris "Versi workspace naik" menyebut `cargo build` tanpa `--locked` dan tiga berkas yang di commit bersama |
| Alasan `M-2` keliru: Apple menerima satu sampai tiga bilangan | `M-2` ditulis sebagai pilihan spec ini |
| Nama yang diakhiri `;` atau `\` belum dicoba | Dicoba: keduanya diubah Xcode. `M-1` diperluas, uji X-10 dan X-11 ditambahkan |
| `plan` sekarang mengembalikan dua keluaran dan uji `config_contract.rs` bisa mengunci jumlahnya | Baris kasus gagal, izin bersyarat di AC-11, dan permintaan pembaruan spec `_root/0004` |
| Tidak jelas apakah `generate` membuat folder induk | Langkah 1 membuat folder `apps/macos/Config/` lebih dulu |
| Daftar pembaruan spec `macos/0001` belum memuat baris "Teks antarmuka" | Ditambahkan |
| Tiga kalimat spec `_root/0004` menjadi usang sesudah build | Ditambahkan ke daftar pembaruan |
| `git grep -lF "$NAME"` bergantung pada nilai nama | AC-2 memakai hitungan baris persis di `Shared.xcconfig`; `git grep` berlaku selama nilai sementara |
| AC-5 menyebut X-1 padahal `--list` mencetak nama uji; uji tolak belum wajib memeriksa awal pesan dan path | Diperbaiki |
| AC-1 hanya memeriksa sebagian baris sukses | Baris lengkap |
| `wc -l` mencetak spasi di depan angka pada macOS | `tr -d ' '` |
| `WindowGroup` tanpa judul belum dicoba | Dicatat sebagai hal yang belum dicoba |
