# 0001. Sapaan di jendela macOS: catatan keputusan

Berkas ini menyimpan alasan di balik [spec macOS 0001](index.md). Kamu tidak perlu membacanya untuk membangun. Bacalah saat ingin tahu mengapa sebuah pilihan diambil, atau saat spec ini akan diubah.

## Context

Scope 004 adalah alur pertama yang bisa dibuktikan dari Mac: pengguna mengetik nama di jendela SwiftUI, menekan tombol, dan melihat sapaan yang dibuat Rust Core. Sebelum spec ini, belum ada satu baris kode pun untuk sisi macOS. Kalau bentuknya tidak ditetapkan, developer akan memilih sendiri versi alat binding, cara menaut pustaka Rust, dan struktur proyek Xcode saat menulis kode, dan pilihan itu sulit ditinjau sesudah jadi.

Beberapa hal sudah tetap sebelum spec ini ditulis:

* [Rujukan macOS](../../../platforms/macos/README.md) menetapkan jalurnya: View SwiftUI, ViewModel, adapter di `Interop`, binding Swift hasil UniFFI di `bindings/swift/`, jembatan tipis di `crates/macos-ffi/`, lalu Core. Rujukan itu juga meminta bukti kompatibilitas, build, integrasi, dan penggunaan nyata.
* [Spec `_root/0001`](../../_root/0001-batas-workspace-dan-pemilik/index.md) menetapkan nama paket `desktop-macos-ffi`, lokasi, arah dependensi, dan aturan manifest akar. `rust-toolchain.toml` dan `.gitignore` hanya boleh berubah lewat spec itu. Core tidak boleh memakai `uniffi`.
* [Spec `_root/0002`](../../_root/0002-kontrak-sapaan-bersama/index.md) menetapkan `hello(name: String) -> String`, vektor V-1 sampai V-11, dan aturan R-8: jembatan dan UI tidak mengubah masukan atau hasil.
* Kriteria 4, 5, 14, dan 23 di [Penerimaan dan bukti fondasi](../../../quality/README.md) adalah yang harus dibuktikan. Kriteria 32 melarang layanan untuk fitur mendatang.

Hal yang membentuk keputusan:

* Hanya ada satu host: Mac Apple Silicon dengan macOS 27.0.1, Xcode 27.1, Swift 6.4, dan Rust 1.99.1. Tidak ada Mac Intel dan tidak ada macOS yang lebih lama untuk diuji.
* Sesi koordinasi boleh menjalankan build dan uji tanpa jendela, tetapi tidak boleh membuka aplikasi. Setiap bukti yang butuh jendela menunggu user. Jadi makin banyak yang terbukti tanpa jendela, makin cepat fitur ini bisa diperiksa.
* Proyek Xcode harus bisa dibuat tanpa membuka Xcode, karena developer di koordinasi ini adalah sesi tanpa GUI.
* Skrip build bersama dan CI adalah milik scope 012, pemeriksaan arsitektur milik scope 002, tema milik 008, lokalisasi milik 010, dan aset merek milik 011. Spec ini tidak boleh mengambil pekerjaan itu.
* Hook Graphify ikut berjalan pada setiap commit dan bisa menolak jenis berkas baru.

Yang belum tetap, dan menjadi isi spec ini: versi dan mode UniFFI, jenis crate jembatan, di mana generator tinggal, apakah binding dilacak git, bagaimana Swift menaut pustaka Rust, struktur `apps/macos/`, versi minimum macOS, arsitektur, cara menguji tanpa jendela, dan siapa yang memiliki memori di batas Rust dan Swift.

## Options considered

Keputusan utama adalah bagaimana sisi Swift memakai jembatan Rust dan bagaimana ia diuji tanpa jendela. Keputusan lain yang lebih kecil ada di tabel sesudah bagian ini.

### Opsi 1: Binding terlacak sebagai paket Swift lokal, modul Swift yang diuji `swift test`, target Xcode tipis

Berkas hasil generator dilacak git di `bindings/swift/` bersama satu `Package.swift`. `apps/macos/` punya paket Swift lokal dengan modul `Interop` dan `Presentation`. Proyek Xcode hanya berisi titik masuk aplikasi dan menaut pustaka statis dari `target/`.

**Pros**:
* `swift test` menjalankan uji lewat binding yang asli tanpa membuka aplikasi.
* Batas modul dipaksa oleh `Package.swift`, jadi `Presentation` tidak diam diam memanggil binding.
* Tidak ada berkas biner di luar `target/`, jadi `.gitignore` tidak perlu berubah.
* Selisih binding terlihat di tinjauan kode, dan CI kelak cukup membandingkan keluaran generator dengan isi git.

**Cons**:
* `swift test` butuh flag `-Xlinker -L` dan pustaka Rust yang sudah dibangun.
* `bindings/swift/` memuat satu berkas tulisan tangan di samping keluaran generator.
* Ada dua sistem build Swift yang harus dipahami: SwiftPM untuk modul dan Xcode untuk aplikasi.

### Opsi 2: Semua kode Swift di target aplikasi Xcode, binding lewat bridging header

Ini cara yang ditulis dokumentasi UniFFI untuk proyek Xcode. Berkas Swift hasil generator dikompilasi langsung di target aplikasi, dan header C masuk lewat bridging header.

**Pros**:
* Satu proyek, tanpa `Package.swift`. Sumber daya dan lokalisasi Apple paling mudah ditambahkan di target aplikasi.
* Tidak ada berkas tulisan tangan di `bindings/swift/`.

**Cons**:
* Uji unit target aplikasi dijalankan di dalam aplikasi yang sedang berjalan, jadi jendela terbuka. Supaya tanpa jendela, berkas sumber harus menjadi anggota dua target sekaligus.
* Tidak ada batas modul. View bisa memanggil binding langsung dan hanya tinjauan yang menangkapnya.
* `project.pbxproj` yang ditulis tangan menjadi jauh lebih besar (dua target, bridging header, dan keanggotaan ganda).

### Opsi 3: XCFramework sebagai target biner

Pustaka statis, header, dan modulemap dibungkus menjadi XCFramework, lalu dipakai sebagai `binaryTarget` paket Swift. Ini bentuk yang umum untuk membagikan pustaka UniFFI.

**Pros**:
* Tidak butuh flag penaut. `swift test` dan Xcode menemukan pustaka sendiri.
* Bentuk yang sama dipakai kalau pustaka kelak dibagikan ke proyek lain atau ke iOS.

**Cons**:
* `binaryTarget` harus berada di dalam folder paket, jadi berkas biner berada di luar `target/` dan butuh entri `.gitignore` baru. Itu perubahan spec `_root/0001`.
* Menambah satu langkah build (`xcodebuild -create-xcframework`) yang harus diulang setiap kali Rust berubah, sebelum scope 012 punya skrip.
* Tidak ada konsumen kedua yang membutuhkan bentuk itu sekarang.

### Opsi 4: Binding tidak dilacak, dibuat saat build

Fase skrip Xcode menjalankan generator pada setiap build, dan `bindings/swift/` diabaikan git.

**Pros**:
* Binding tidak pernah usang dan tidak pernah muncul di selisih commit.

**Cons**:
* Butuh entri `.gitignore` untuk `bindings/swift/`, yang berarti perubahan spec `_root/0001`.
* Setiap build membangun generator (sekitar dua menit pada build pertama).
* Perubahan binding tidak terlihat di tinjauan, dan `swift test` tidak bisa berjalan sebelum Xcode pernah membangun.
* Dokumen mutu meminta keluaran generator bisa dibandingkan dengan sumbernya. Tanpa berkas di git, tidak ada yang dibandingkan.

### Keputusan yang lebih kecil

Tanda `biasa` atau `berat` mengikuti definisi mode keputusan koordinasi. Semua diambil dengan mode `auto`.

| Pertanyaan | Diambil | Mengapa | Pilihan kedua | Tanda |
| --- | --- | --- | --- | --- |
| Versi UniFFI? | Persis `=0.32.2`, versi terbaru di crates.io pada 2026-11-15 | Sudah dicoba dengan Rust 1.99.1 dan Swift 6.4, dan keluarannya lolos mode bahasa Swift 6 tanpa peringatan. Pin persis menjaga versi itu juga saat `Cargo.lock` dibuat pertama kali. | Batas bawah `0.32.2` yang dijaga `Cargo.lock`. Lebih longgar, tetapi lockfile baru bisa mengambil rilis patch yang belum dicoba dan mengubah checksum binding. | biasa |
| Mode UniFFI? | Makro (`#[uniffi::export]`) | Satu fungsi tidak butuh berkas UDL kedua yang harus dijaga sama dengan kode. | UDL dengan `build.rs`. Berguna bila antarmuka ditulis lebih dulu dari kode, tetapi menambah berkas dan dependensi build. | biasa |
| Di mana generator tinggal? | Bin di crate jembatan, di balik fitur `bindgen` | Versi generator dan pustaka selalu sama, tanpa anggota workspace baru, dan build biasa tidak membangunnya. | Crate sendiri di `tooling/codegen/`. Lebih rapi bila ada generator kedua, tetapi menambah anggota dan dua tempat versi. | biasa |
| Titik masuk generator yang mana? | `uniffi_bindgen_swift` | Bisa menulis sumber Swift dan header ke folder berbeda dengan nama `module.modulemap`, dan tidak memanggil pemformat. | `uniffi_bindgen_main generate`. Satu perintah, tetapi satu folder keluaran dan nama modulemap yang tidak dikenali SwiftPM. | biasa |
| Jenis crate? | `staticlib` dan `lib` | Pustaka statis masuk ke dalam biner aplikasi, tanpa menyalin, menandatangani, atau mengatur rpath pustaka dinamis. | `cdylib`. Dibutuhkan bila pustaka dimuat saat berjalan, yang tidak terjadi di sini. | biasa |
| Binding dilacak git? | Ya, berkas teks saja | Selisih terlihat di tinjauan, bisa dibandingkan dengan keluaran generator, dan `.gitignore` tidak berubah. | Tidak dilacak (Opsi 4). | biasa |
| Bagaimana Swift menaut pustaka? | Target `systemLibrary`, `linkedLibrary`, dan path pencarian dari pemanggil | Tidak ada biner di luar `target/` dan tidak ada flag tidak aman di `Package.swift`. | XCFramework (Opsi 3). | biasa |
| Proyek Xcode dibuat bagaimana? | `project.pbxproj` ditulis tangan, format versi 77, dilacak git | Tidak butuh alat tambahan dan tidak butuh GUI. Folder tersinkron membuat berkasnya kecil. | XcodeGen atau Tuist. Berkas proyek tidak perlu dilacak, tetapi itu alat baru yang harus dipasang di setiap mesin dan CI. | biasa |
| Siapa yang membangun Rust saat Xcode membangun? | Fase skrip di target aplikasi | Tombol Run di Xcode selalu memakai Rust terbaru, dan penautan tidak pernah memakai pustaka lama. | Perintah terpisah sebelum `xcodebuild`. Proyek lebih sederhana, tetapi Run gagal menaut bila langkah itu terlupa. | biasa |
| Target Rust ditulis eksplisit? | Ya, `--target aarch64-apple-darwin` | Arsitektur tercatat di perintah dan di path keluaran, dan menambah Intel kelak tinggal menambah satu target. | Target bawaan host. Perintah lebih pendek, tetapi arsitektur hanya tersirat. | biasa |
| Versi minimum macOS? | `14.0` | Versi pertama yang punya `@Observable`. Menaikkan batas kelak hanya satu baris, sedangkan menurunkan butuh uji ulang. | `15.0` atau `26.0`. Lebih dekat ke host yang diuji, tetapi menutup Mac yang masih didukung tanpa alasan teknis. | biasa |
| Intel `x86_64`? | Tidak didukung di fase ini | Tidak ada Mac Intel untuk menguji, target Rust itu belum terpasang, dan menambahnya butuh perubahan `rust-toolchain.toml` lewat spec `_root/0001`. | Biner universal. Butuh dua target Rust, langkah `lipo`, dan bukti yang tidak bisa dikumpulkan sekarang. | biasa |
| Tanda tangan dan sandbox? | Ad hoc, tanpa sandbox, tanpa entitlement | Cukup untuk dijalankan lokal tanpa akun Apple. Rujukan macOS menaruh sandbox dan penandatanganan di dokumentasi distribusi. | Sandbox dan hardened runtime sejak sekarang. Lebih dekat ke bentuk rilis, tetapi menambah berkas entitlement sebelum ada yang perlu dilindungi. | berat |
| Thread untuk memanggil `hello`? | Sinkron di thread utama | Kontrak menyebut `hello` aman dipanggil dari thread UI untuk nama yang diketik, dan tanpa `async` tidak ada keadaan memuat. | `Task` di luar thread utama. Melindungi dari nama raksasa, tetapi menambah keadaan dan urutan hasil yang harus diuji. | biasa |
| Bentuk state? | `@Observable` dan `@MainActor` | Bawaan SwiftUI sejak macOS 14 dan lolos pemeriksaan konkurensi Swift 6. | `ObservableObject`. Jalan di macOS yang lebih lama, tetapi lebih banyak kode. | biasa |
| Kerangka uji Swift? | Swift Testing | Sudah ada di toolchain Swift 6.4 dan berjalan lewat `swift test` tanpa dependensi. | XCTest. Sama mampunya, hanya lebih banyak kode pembungkus. | biasa |
| Uji UI otomatis? | Tidak ada. Alur jendela dibuktikan dengan langkah manual | Uji UI mengambil alih layar dan tetap harus dijalankan user, jadi targetnya belum memberi bukti tambahan. | Target XCUITest yang dijalankan user. Berguna bila alurnya bertambah. | biasa |
| Teks antarmuka? | Literal bahasa Inggris, hasil lewat `Text(verbatim:)` | Lokalisasi milik scope 010. Literal SwiftUI sudah berbentuk kunci, jadi kelak cukup ditambah katalog. | Katalog bahasa sekarang. Itu pekerjaan scope 010. | biasa |
| Lint jembatan? | `unsafe_code = "forbid"` dan `missing_docs = "warn"` | Percobaan menunjukkan lint itu lolos walau makro UniFFI menghasilkan `unsafe`, jadi kode tulisan tangan terjaga oleh compiler. | Tanpa lint sampai scope 002. Lebih sedikit baris, tetapi klaim "tanpa `unsafe` tulisan tangan" hanya dijaga tinjauan. | biasa |
| Nama? | Proyek dan target `DesktopMac`, paket `DesktopMacModules`, modul `Interop`, `Presentation`, `DesktopMacosFFI` | Mengikuti nama bagian di rujukan macOS dan nama crate. Modul C `desktop_macos_ffiFFI` adalah nama bawaan generator. | Nama produk yang lebih ramah. Itu keputusan merek milik scope 011. | biasa |

Pengenal bundel bukan keputusan. Nilai akhirnya hanya user yang tahu, jadi spec memakai `com.example.desktop-foundation.macos` sebagai nilai sementara dan mencatatnya di Follow-up.

## Rationale

Opsi 1 dipilih karena tiga hal dari Context.

Pertama, bukti harus sebanyak mungkin berjalan tanpa jendela. Hanya Opsi 1 dan Opsi 3 yang memberi `swift test` atas binding yang asli. Opsi 2 menjalankan uji di dalam aplikasi atau memaksa keanggotaan ganda, dan Opsi 4 baru bisa diuji sesudah Xcode pernah membangun.

Kedua, `.gitignore` dan `rust-toolchain.toml` bukan milik spec ini. Opsi 3 dan Opsi 4 sama sama menaruh berkas yang tidak dilacak di luar `target/`, jadi keduanya menunggu pembaruan spec `_root/0001` sebelum bisa dibangun. Opsi 1 menaruh semua biner di `target/`, yang sudah diabaikan, dan melacak berkas teks saja.

Ketiga, proyek Xcode harus ditulis tanpa GUI. Makin sedikit yang hidup di `project.pbxproj`, makin kecil kemungkinan salah. Di Opsi 1 berkas itu hanya memuat satu target, satu fase skrip, dan satu rujukan paket. Semua logika yang perlu diuji tinggal di `Package.swift` yang mudah dibaca.

Harga Opsi 1 adalah flag `-Xlinker -L` pada `swift test` dan satu `Package.swift` tulisan tangan di folder binding. Keduanya kecil, tertulis di spec, dan bisa dibungkus skrip saat scope 012 tiba. XCFramework tetap jalan keluar yang jelas bila pustaka ini kelak punya konsumen kedua.

Binding dilacak git karena dokumen mutu meminta keluaran generator bisa dibandingkan dengan sumbernya, dan karena baris Done when berkata "binding dapat dibuat ulang". Dengan berkas di git, kalimat itu menjadi satu perintah yang bisa lulus atau gagal.

Versi minimum 14.0 dan arsitektur arm64 dipilih dengan jujur terhadap bukti: hanya satu host yang ada. Batas bawah yang rendah murah dinaikkan, dan Intel sengaja ditunda karena membutuhkan perubahan file milik spec lain dan mesin yang tidak ada.

Generator sengaja tinggal di crate jembatan. Kesalahan yang paling sering pada UniFFI adalah binding yang dibuat generator versi lain dari pustaka yang ditautkan. Di sini itu tidak bisa terjadi, dan kalaupun binding usang, pemeriksaan checksum menghentikan uji dengan pesan yang jelas.

## Bukti percobaan

Semua isi file di [index.md](index.md) disalin dari workspace percobaan di luar repo, pada Mac koordinasi (arm64, macOS 27.0.1, Xcode 27.1 build 27A9275, Swift 6.4, Rust 1.99.1) tanggal 2026-11-15. Core percobaan memakai isi dari spec `_root/0001` dan `_root/0002`. Aplikasi tidak pernah dibuka, sesuai aturan langkah manual.

| Yang dicoba | Hasil |
| --- | --- |
| `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings` | Lulus. Build biasa tidak membangun `uniffi_bindgen`. |
| `cargo clippy -p desktop-macos-ffi --all-targets --all-features --locked -- -D warnings` dengan `unsafe_code = "forbid"` | Lulus. Lint tidak menyala untuk hasil makro UniFFI. |
| `cargo tree -p desktop-macos-ffi --depth 1 --prefix none --locked` | Tiga baris: `desktop-macos-ffi`, `desktop-core`, `uniffi v0.32.2`. |
| `cargo tree -p desktop-core --target all --all-features --prefix none --locked` | Satu baris. |
| `grep -c '^\[\[package\]\]' Cargo.lock` | `77`. |
| Perintah di **Membuat ulang binding**, dua kali berturut turut, dari pustaka debug dan dari pustaka release | Keluaran sama byte demi byte. `git diff --exit-code -- bindings/swift` keluar 0. |
| `swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"` | 7 uji `BindingContractTests` dan 4 uji `GreetingViewModelTests` lulus, tanpa peringatan compiler. |
| `swift build --build-tests` tanpa `-Xlinker -L` | Gagal dengan `ld: library 'desktop_macos_ffi' not found`. |
| Mengubah nama parameter `hello` di jembatan tanpa membuat ulang binding, lalu `swift test` | Berhenti dengan `Fatal error: UniFFI API checksum mismatch`. |
| `xcodebuild ... -configuration Debug ... build` | `** BUILD SUCCEEDED **`. Biner `arm64`, tanda tangan ad hoc, `LSMinimumSystemVersion` `14.0`, simbol `uniffi_desktop_macos_ffi_fn_func_hello` ada di `DesktopMac.debug.dylib`. |
| `xcodebuild ... -configuration Release ... build` | `** BUILD SUCCEEDED **`. Aplikasi 1,7 MB, `minos 14.0`, `sdk 27.0`, simbol jembatan ada di biner utama. |
| `git status --short` sesudah semua perintah di atas, dengan `.gitignore` dari spec `_root/0001` | Kosong. |
| `git grep --untracked -lF 'Hello, ' -- '*.swift'` dan `git grep --untracked -l 'import DesktopMacosFFI' -- apps/macos` | Dua berkas uji, dan hanya `apps/macos/Interop/Greeter.swift`. |
| Sesudah `xcodebuild` membangun pustaka dengan `MACOSX_DEPLOYMENT_TARGET=14.0`, `cargo build` yang sama dari terminal | Selesai dalam 0,12 detik tanpa membangun ulang. Berpindah antara Xcode dan terminal tidak memicu build ulang Rust. |
| `cargo tree -p desktop-macos-ffi --target all --all-features --prefix none --locked` disaring dengan pola crate aplikasi dan pustaka UI | `0` baris. Yang muncul untuk target Windows hanya `windows-sys` dan `windows-link`, yaitu dependensi tak langsung milik generator. |
| `xcrun --sdk macosx --show-sdk-version` dan `grep -c 'bindings_contract_version = 30'` pada binding | `27.0` dan `1`. |

Tiga temuan percobaan yang membentuk isi spec:

* Bin generator awalnya bernama `uniffi-bindgen` dan memakai `uniffi_bindgen_main`. Perintah itu mencoba memformat keluaran dengan alat yang mungkin tidak ada di mesin lain, dan menulis semua berkas ke satu folder. `uniffi_bindgen_swift` tidak punya kedua masalah itu, tetapi butuh `--module-name desktop_macos_ffiFFI` supaya nama modul di modulemap cocok dengan yang diimpor berkas Swift.
* Berkas tambahan di `src/bin/` otomatis menjadi bin tanpa syarat fitur dan mematahkan build biasa. Karena itu manifest jembatan memuat `autobins = false`.
* Pengaturan build Xcode yang kosong tidak diekspor ke fase skrip, jadi skrip memilih `--release` dari `CONFIGURATION`, bukan dari pengaturan buatan sendiri.

Yang tidak dicoba: membuka aplikasi atau proyek di GUI, macOS selain 27.0.1, Mac Intel, dan host Windows atau Linux.

## Cross check

Spec ini dibaca ulang oleh model lain (Claude Opus, hanya membaca) pada 2026-11-15. Pemeriksa mencocokkan setiap `grep` dan hitungan dengan isi file di spec. Yang ia benarkan: hitungan uji 7 dan 4, hitungan 16 path, serta pemeriksaan **AC-6**, **AC-7**, dan **AC-8**. Semua temuan di bawah sudah diterapkan, dan perubahan pada isi file dicoba ulang di workspace percobaan (`cargo`, generator, `swift test`, dan `xcodebuild` Debug lulus lagi).

| Temuan | Yang diubah |
| --- | --- |
| Versi UniFFI belum terkunci saat `Cargo.lock` dibuat pertama kali | Dependensi menjadi `=0.32.2`. |
| Bukti anggota baru yang diminta spec `_root/0001` belum ada | **AC-9** mendapat perintah `cargo tree ... --target all --all-features` dengan pola larangan. |
| **AC-11** dan **AC-12** tidak punya perintah, dan klaim "tidak ada skrip" salah karena `scripts/update_graphify.py` sudah ada | Keduanya memakai `git log --grep='spec macos/0001 step'`, dan subject commit ditetapkan di Build plan. |
| **AC-8** memakai `git grep` biasa, yang melewatkan berkas yang belum dilacak | Dua pencarian pertama memakai `--untracked`. |
| Uji V-11 tidak memeriksa bagian tengah nama | `v11LongNameIsNotCut` mendapat satu `#expect` untuk bagian tengah. |
| **AC-1** baru bermakna sesudah binding masuk commit | **AC-1** dan langkah 2 menyebut urutannya. |
| Langkah manual 6 memberi harapan yang salah, dan langkah 9 tidak punya aturan lulus | Langkah 6 menghapus kolom lebih dulu. Langkah 9 dan **AC-13** menyebut bahwa berkas dari Xcode dicatat dan tidak menggagalkan. |
| Pemulihan hook Graphify tidak menyebut `git add` | Ditambahkan, bersama catatan bahwa `.graphifyignore` sudah ada di `main`. |
| Dua nilai tabel kombinasi tidak punya perintah | **AC-10** mendapat `xcrun --sdk macosx --show-sdk-version` dan `grep` versi kontrak. |
| Heading `verify.md` tidak disebut | Build plan menyebut heading tiap milestone. |
| Tabel kepemilikan tidak menyebut `RustCallStatus` dan salinan sementara | Baris ketiga dan satu kalimat ditambahkan. |
| Larangan berkas lain di `src/bin/` hanya berupa kalimat | Diganti `autobins = false`. |
| `cdylib` akan dipilih penaut lebih dulu | Menjadi invarian dan catatan manifest. |
| `uniffi` tidak berada di bawah tabel `target` tanpa penjelasan | Catatan manifest menjelaskan alasannya. |
| `LIBRARY_SEARCH_PATHS` patah bila path repo memuat spasi | Nilainya diapit tanda kutip di dalam. |
| Generator gagal sesudah folder keluaran dihapus, `CARGO_TARGET_DIR` milik user, dan Xcode di GUI menolak paket lokal | Tiga baris baru di tabel kasus gagal, dan satu Follow-up untuk jalan keluar paket lokal. |

Satu temuan tidak diterapkan karena percobaan menunjukkan sebaliknya: pemeriksa menduga berpindah antara Xcode dan terminal membangun ulang seluruh Rust, sebab Xcode mengekspor `MACOSX_DEPLOYMENT_TARGET`. Pada Rust 1.99.1 itu tidak terjadi (lihat **Bukti percobaan**).
