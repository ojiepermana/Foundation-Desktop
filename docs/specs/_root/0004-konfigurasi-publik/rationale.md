# 0004. Konfigurasi publik: alasan

Catatan keputusan untuk [index.md](index.md). `/develop` tidak perlu membaca file ini.

## Context

Tiga aplikasi desktop native di repo ini harus menampilkan produk yang sama. Itu berarti ada nilai yang wajib sama di ketiganya, walau tiap OS menyimpannya dengan cara sendiri. Pada 2026-11-15 hanya aplikasi macOS yang ada, dan nilai seperti itu sudah tersebar:

| Nilai | Tempat sekarang | Cara masuk |
| --- | --- | --- |
| Nama tampil `Desktop Foundation` | `apps/macos/DesktopMac.xcodeproj/project.pbxproj`, dua kali (Debug dan Release), dan literal judul jendela di `apps/macos/App/DesktopMacApp.swift` | Ditulis tangan |
| Pengenal bundel `com.example.desktop-foundation.macos` | `project.pbxproj`, dua kali | Ditulis tangan, nilai sementara |
| Versi `0.1.0` | `[workspace.package]` di `Cargo.toml` root, lalu `MARKETING_VERSION` di `project.pbxproj`, dua kali | Disalin tangan dari Cargo |

Aplikasi Windows dan Linux kelak butuh nama dan pengenal yang sama maknanya. Tanpa satu sumber, tiap aplikasi menyalin nilai itu sendiri, dan nilai akhir yang hanya user tahu (nama produk dan pengenal, lihat `N4` di file langkah manual koordinasi) harus diganti di banyak tempat.

[Resource bersama](../../../shared-resources/README.md) sudah menetapkan arahnya: `shared/config/` memegang skema untuk nilai publik yang perlu konsisten, skema itu membedakan nilai saat build, nilai saat aplikasi berjalan, definisi environment, dan pengaturan khusus OS, perubahan skema bisa divalidasi dan ditelusuri versinya, dan izin, kunci privat, serta kredensial penandatanganan berada di luar resource publik. Layanan konfigurasi jarak jauh dan mesin feature flag ditunda.

Gaya yang membentuk keputusan:

- **Nilainya sangat sedikit.** Fondasi ini satu jendela sapaan tanpa backend, tanpa akun, dan tanpa jaringan. Tidak ada alamat layanan, tidak ada environment, dan tidak ada yang dibaca aplikasi saat berjalan. [Kriteria 32](../../../quality/README.md) dan "Batas fase fondasi" di [peta arsitektur](../../../architecture/README.md) melarang layanan untuk fitur mendatang.
- **Tiga konsumen dengan cara baca berbeda.** Xcode butuh nilai sebagai build setting sebelum bundel dibuat. Aplikasi Rust Windows dan Linux mendapat versi dari Cargo saat kompilasi. Tidak ada satu mekanisme baca yang cocok untuk semuanya.
- **Versi sudah punya sumber.** `[workspace.package].version` diwarisi setiap paket (spec [_root/0001](../0001-batas-workspace-dan-pemilik/index.md)). Cargo tidak bisa membaca versi dari file lain.
- **Core tidak tahu soal presentasi.** `desktop-core` tanpa dependensi dan hanya memegang perilaku. Nama produk dan pengenal bukan perilaku.
- **Hanya Mac yang tersedia.** Aturan untuk Windows dan Linux tidak bisa dibuktikan sekarang, jadi aturan bersama harus cukup sempit untuk benar di ketiganya dan cukup longgar supaya spec OS bisa memilih cara baca native.
- **Hook Graphify membaca setiap sumber baru.** File JSON di root (`test-preferences.json`) sudah lolos hook. Manifest TOML virtual pernah gagal diekstrak dan harus masuk `.graphifyignore` (spec `_root/0001`).
- **Alat generator dan validasi sumber bersama ditetapkan scope 008** (spec `_root/0003`). Alat itu paket `desktop-codegen` di `tooling/codegen/`, hanya bergantung pada `serde` dan `serde_json`, memvalidasi sumber JSON dengan tipe Rust tertutup, dan menulis file skema sebagai keluaran generator. Mesin JSON Schema ditolak di sana karena menambah 57 paket ke lockfile bersama. Spec ini memakai bentuk itu dan tidak membuat alat kedua.
- **Kriteria yang harus tercakup:** 18 (aturan konfigurasi bersama terdokumentasi) dan baris **Done when** scope 009: skema, versi, cara validasi, cara konsumsi, dan batas pengaturan khusus tiap OS.

Bila keputusan ini ditunda, scope 006 dan 007 menyalin nama dan pengenal ke proyek masing masing, salinan versi di macOS tetap dijaga tangan, dan tidak ada aturan yang mencegah nilai lokal atau rahasia masuk ke folder bersama.

## Options considered

### Opsi 1: Satu file nilai JSON dengan skema tertutup di `shared/config/`

`shared/config/public.json` memuat nilai publik, dikelompokkan menurut kapan nilai itu dipakai. Skemanya tertutup (kunci yang tidak dikenal ditolak) dan bernomor versi bilangan bulat, ditegakkan alat `desktop-codegen`. Versi aplikasi tetap di Cargo. Tiap OS memakai nilai lewat keluaran generator atau salinan yang dibandingkan pemeriksaan otomatis.

**Pros**:

- Satu tempat untuk nilai yang hanya user tahu.
- JSON terbaca tanpa pustaka tambahan oleh alat Apple (`plutil`, `Foundation`), oleh `jq`, dan oleh `serde_json` di Rust.
- Skema tertutup membuat nilai lokal atau rahasia yang tersesat langsung ditolak.
- File JSON sudah terbukti lolos hook Graphify.

**Cons**:

- JSON tidak punya komentar, jadi penjelasan tiap kunci tinggal di skema dan `README.md`.
- Satu file sumber, satu skema hasil generator, dan satu modul pemeriksaan untuk dua nilai saja.
- Tiap OS tetap butuh langkah sendiri untuk memindahkan nilai ke bentuk native.

### Opsi 2: File nilai TOML dengan aturan di kode validasi

`shared/config/public.toml` memuat nilai, dan aturannya ditulis sebagai tipe Rust di alat validasi.

**Pros**:

- TOML sudah dipakai repo ini (`Cargo.toml`, `rust-toolchain.toml`) dan punya komentar.
- Tipe Rust dengan penolakan kunci tak dikenal memberi skema tertutup tanpa file skema.

**Cons**:

- Alat Apple dan Python 3.9 di Mac ini tidak punya pembaca TOML bawaan, jadi sisi macOS butuh alat Rust untuk setiap pembacaan.
- Skema tinggal di kode alat, bukan di `shared/config/` seperti yang dokumen resource bersama minta.
- File TOML tanpa struktur yang dikenal Graphify berisiko gagal di hook, seperti manifest virtual.

### Opsi 3: Tanpa file bersama, nilai tetap di proyek native

Nilai tetap di `project.pbxproj` dan kelak di proyek Windows dan Linux. Satu tabel di dokumen mencatat nilai yang wajib sama, dan satu pemeriksaan membandingkan salinannya.

**Pros**:

- Paling sedikit bagian baru. Jujur untuk jumlah nilai sekecil ini.
- Tiap OS memakai format native tanpa langkah perantara.

**Cons**:

- Tidak ada sumber, hanya salinan yang saling dibandingkan. Mengganti nilai berarti menyunting tiap proyek.
- Pemeriksaan harus mengerti format tiap proyek native dari host mana pun.
- Bertentangan dengan dokumen resource bersama yang meminta satu sumber berversi.

### Opsi 4: Tabel `[workspace.metadata]` di `Cargo.toml` root

Nilai ditulis di `[workspace.metadata.desktop]`, di file yang sama dengan versi. `cargo metadata` mengeluarkannya sebagai JSON di setiap host.

**Pros**:

- Versi dan nilai publik lain berada di satu file.
- Tidak butuh pembaca baru, karena Cargo sudah mengurai file itu.

**Cons**:

- Setiap perubahan nilai menyentuh manifest root, yang perubahannya dibatasi spec `_root/0001`.
- Mengabaikan lokasi `shared/config/` yang sudah ditetapkan peta arsitektur.
- Build Xcode harus memanggil Cargo hanya untuk membaca nama produk.
- Skema tertutup tidak bisa dipasang pada tabel metadata tanpa alat tambahan.

## Rationale

Opsi 1 dipilih karena memenuhi arah yang sudah tertulis (satu sumber berversi di `shared/config/`) dengan bagian sesedikit mungkin. Nilai akhir hanya user yang tahu, jadi satu file yang jelas tempatnya lebih berharga daripada penghematan satu file di Opsi 3. Tiga konsumen membaca dengan cara berbeda, dan JSON adalah satu satunya format di antara pilihan ini yang terbaca alat bawaan di sisi Apple maupun di Rust. Hook Graphify sudah terbukti menerima JSON, sedangkan TOML tanpa struktur punya riwayat gagal.

Versi tidak dipindah ke file itu. Cargo tidak bisa membaca versi dari luar manifest, jadi menyalinnya ke `public.json` hanya menambah salinan. Sumber versi tetap `[workspace.package].version`, dan aturan konsumsi yang sama (salinan wajib dibuat generator atau dibandingkan) berlaku untuk `MARKETING_VERSION` di macOS.

Skema hanya memuat grup `build` karena tidak ada nilai yang dibaca saat berjalan dan tidak ada environment di fondasi ini. Menambah grup kosong demi kelengkapan melanggar kriteria 32. Aturan cara menambahnya ditulis, isinya tidak.

Opsi 4 menarik karena menyatukan versi dan nilai lain, tetapi ia menjadikan manifest root tempat data produk dan memaksa Xcode bergantung pada Cargo untuk hal yang bukan build Rust. Opsi 2 kalah karena sisi macOS tidak bisa membacanya tanpa alat tambahan.

## Keputusan yang diambil

Mode keputusan brief adalah `auto`: tiap keputusan memakai opsi yang direkomendasikan, dan pilihan keduanya dicatat di sini.

| No. | Pertanyaan | Diambil | Pilihan kedua | Tanda |
| --- | --- | --- | --- | --- |
| 1 | Nilai publik mana yang dibutuhkan fondasi sekarang? | Dua nilai di file: nama produk dan dasar pengenal aplikasi. Versi dirujuk dari Cargo. | Hanya dasar pengenal; nama produk menunggu scope 011. | biasa |
| 2 | Di mana sumber versi aplikasi? | `[workspace.package].version` di `Cargo.toml` root, tidak disalin ke `public.json`. | Salinan di `public.json` yang dibandingkan dengan Cargo. | biasa |
| 3 | Format file nilai? | JSON. | TOML. | biasa |
| 4 | Bentuk versi skema? | Bilangan bulat mulai dari `1`, naik satu pada setiap perubahan skema. | SemVer `MAJOR.MINOR`. | biasa |
| 5 | Susunan kunci? | Grup menurut kapan nilai dipakai. Versi 1 hanya punya grup `build`. | Kunci datar dengan jenis dicatat di skema. | biasa |
| 6 | Kapan aplikasi memakai nilai? | Saat build saja. Aplikasi tidak membuka `shared/config/` saat berjalan. | File ikut dikemas dan dibaca saat aplikasi mulai. | biasa |
| 7 | Bagaimana salinan di proyek native dijaga? | Dibuat generator, atau dibandingkan pemeriksaan otomatis. Spec OS memilih. | Wajib generator untuk semua OS. | biasa |
| 8 | Hubungan pengenal tiap OS dengan dasar pengenal? | Sama dengan dasar, atau dasar ditambah satu segmen yang spec OS tetapkan. | Tiap OS bebas, dengan tabel pemetaan di spec OS. | biasa |
| 9 | Apakah nilai publik boleh ditimpa secara lokal? | Tidak. Satu nilai untuk semua orang dan semua build. | File `local.json` yang diabaikan git. | biasa |
| 10 | Apakah versi 1 mendefinisikan environment? | Tidak. Satu rasa build. Debug dan Release adalah konfigurasi build tiap OS. | Satu environment bernama `default` sebagai tempat. | biasa |
| 11 | Apa yang boleh ada di `shared/config/`? | Daftar file tertutup. File lain membuat validasi gagal. | Folder bebas, hanya `public.json` yang diperiksa. | biasa |
| 12 | Siapa pemilik nama produk? | Konfigurasi publik (spec ini). Bukan entri katalog bahasa (scope 010) dan bukan aset merek (scope 011). | Nama produk menjadi entri katalog bahasa. | berat |
| 13 | Di mana aturan untuk orang yang menyunting nilai? | `shared/config/README.md`, dengan ringkasan di `docs/shared-resources/README.md`. | Hanya di spec ini. | biasa |
| 14 | Di mana skema hasil generator diletakkan? | `shared/config/public.schema.json`, di samping sumbernya. | Subfolder `shared/config/schemas/`. | biasa |
| 15 | Seberapa banyak aturan yang masuk ke skema hasil generator? | Kunci tertutup, tipe, nilai tetap, serta panjang dan pola `build.app_identifier`. Aturan whitespace `build.product_name` hanya di kode pemeriksaan. | Pola regex untuk `build.product_name` juga. | biasa |
| 16 | Bagaimana aturan nilai ditulis? | Dengan kata kata, ditegakkan kode dan uji T-1 sampai T-32. | Sebagai ekspresi reguler yang dievaluasi mesin. | biasa |
| 17 | Bagaimana pemeriksaan melaporkan pelanggaran? | Semua pelanggaran dilaporkan, tiap pesan diawali ID aturan. | Berhenti di pelanggaran pertama. | biasa |
| 18 | Apa yang terjadi bila format native sebuah OS menolak dasar pengenal? | Spec OS itu menulis pemetaan tetap dari dasar pengenal (`K-6`). | Mempersempit aturan dasar pengenal sekarang (tanpa tanda `-`, lebih pendek). | berat |

Keputusan 12 bertanda `berat` karena spec [macos/0001](../../macos/0001-sapaan-di-jendela-macos/index.md) menulis nama tampil bundel sebagai urusan scope 011 dan literal judul jendela sebagai urusan scope 010, dan kedua scope itu belum punya spec. Keputusan ini murah dibalik (satu kunci dan satu kenaikan versi skema), tetapi ia menggeser batas antara tiga scope.

Keputusan 18 bertanda `berat` karena kedua pilihannya hampir setara dan belum bisa diuji tanpa host Windows dan Linux. Nilai sementara `com.example.desktop-foundation` memuat tanda `-`, sah untuk Apple, tetapi sebagian format Windows untuk aplikasi berpaket diduga menolaknya. Pilihan yang diambil paling murah dibalik: mempersempit aturan kelak adalah satu kenaikan versi skema, sedangkan mempersempit sekarang mengubah nilai sementara yang sudah dipakai spec `macos/0001`.

Keputusan yang diikuti dari spec `_root/0003` (milik architect scope 008, bukan keputusan spec ini): alat `desktop-codegen` di `tooling/codegen/`, sumber JSON, tipe `serde` tertutup, tanpa mesin JSON Schema, skema sebagai keluaran generator draft-07, pembuka `$schema` dan `schemaVersion` di tiap file sumber, perintah `check` dan `generate`, uji `committed_outputs_match_the_source`, dan baris `text eol=lf` di `.gitattributes`. Spec ini hanya memilih nama sumber `config`, sesuai saran architect itu.

Akibat bila pilihan kedua yang diambil:

- Keputusan 1: `build.product_name` hilang dari skema, dari AC-2, dan dari aturan `R-2`; uji T-10 dan T-12 sampai T-18 ikut hilang.
- Keputusan 2: skema mendapat kunci `version`, dan validasi mendapat aturan yang membandingkannya dengan `Cargo.toml`.
- Keputusan 3: nama file menjadi `public.toml`, perintah `jq` di AC diganti, dan `desktop-codegen` butuh pembaca TOML, yang berarti perubahan spec `_root/0003`.
- Keputusan 6: muncul aturan pengemasan per OS dan pembaca saat berjalan di tiap aplikasi.
- Keputusan 8: aturan `K-6` dihapus dan tiap spec OS menulis pemetaannya sendiri.
- Keputusan 9: `.gitignore` berubah lewat spec `_root/0001`, dan validasi harus menggabungkan dua file.
- Keputusan 12: `build.product_name` pindah ke katalog scope 010.
- Keputusan 18: aturan `build.app_identifier` kehilangan tanda `-` dan batasnya turun, nilai sementara menjadi `com.example.desktopfoundation`, dan spec `macos/0001` ikut diperbarui.

## Bukti yang diperiksa

Diperiksa di Mac koordinasi pada 2026-11-15, tanpa mengubah repo:

| Yang diperiksa | Hasil |
| --- | --- |
| `grep -rn 'Desktop Foundation\|com\.example' apps/macos` | Lima baris: satu literal `WindowGroup("Desktop Foundation")`, dua `INFOPLIST_KEY_CFBundleDisplayName`, dua `PRODUCT_BUNDLE_IDENTIFIER = "com.example.desktop-foundation.macos"` |
| `[workspace.package]` di `Cargo.toml` root | `version = "0.1.0"` |
| `graphify-out/manifest.json` | Memuat `test-preferences.json`, jadi sumber JSON lolos hook |
| `which jq` | `/usr/bin/jq` |
| `python3 --version` | `Python 3.9.6`, tanpa modul `tomllib` |
| `ls shared tooling` | Keduanya belum ada |
| Purwarupa architect scope 008 (pesan antarsesi, 2026-11-15) | `desktop-codegen` dengan `serde` dan `serde_json` hanya menambah satu paket ke `Cargo.lock`; `jsonschema` menambah 57 paket |
| Blok "Isi file" diekstrak ke folder sementara dengan perintah `awk` di AC-1 dan AC-2 | Perintah `jq` dan `grep` di AC-1 sampai AC-3, AC-6, AC-7, dan AC-9 mencetak nilai yang tertulis di spec; skema sama dengan bentuk kanonik (kunci terurut, dua spasi) |

## Cross check

Dibaca oleh model lain (read only) pada 2026-11-15 sebelum spec ini diajukan. Jumlah nilai (dua) dan cakupan Done when dinilai benar; temuan berikut diterapkan ke `index.md`. Mode keputusan `auto`, jadi tiap celah memakai perbaikan yang direkomendasikan.

| Temuan | Perbaikan |
| --- | --- |
| Aturan dasar pengenal disebut irisan aturan tiga OS, padahal format Windows untuk aplikasi berpaket lebih sempit | Klaim irisan dihapus. `K-6` mengizinkan pemetaan tetap per OS, dan dugaan tentang Windows dicatat sebagai hal terbuka di Follow-up (keputusan 18) |
| Pola regex nama produk berbeda hasil antar mesin dan tidak menolak U+2028, U+2029, serta kontrol C1 | Aturan ditulis dengan kata kata, daftar karakter diperluas, dan uji T-14 sampai T-16 ditambahkan |
| `R-2` diam diam butuh mesin JSON Schema | Pemeriksaan memakai tipe Rust tertutup di `desktop-codegen`; skema menjadi keluaran generator |
| `R-1` tidak mewajibkan file ada | `public.json` dan `README.md` wajib ada; uji T-4 dan T-5 |
| Versi yang dikenal dibaca dari file yang diperiksa | Versi menjadi konstanta di kode; uji T-27 |
| Aturan segmen tambahan dan batas panjang turunan `K-6` tidak tertulis | Bagian "Rincian `K-6`" |
| Kalimat pengganti di dokumen arsitektur tidak tertulis dan tidak diperiksa | Cara sisip dan kalimat hasil tertulis; AC-8 mendapat `grep` |
| Daftar file AC-9 tidak menyebut berkas scope | AC-9 menyebut berkas scope disunting lead di luar branch developer |
| "Sama persis dengan Isi file" tanpa perintah | AC-1 dan AC-2 mendapat perintah `awk` dan `diff` |
| Catatan alasan menyebut `R-5` yang tidak ada | Diperbaiki |
| Teks `README.md` bertentangan dengan pengecualian `.DS_Store` | Kalimat disesuaikan |
| Tidak jelas apakah pemeriksaan melaporkan semua pelanggaran | Semua dilaporkan (keputusan 17) |
| Baris Neutral tidak menyebut literal judul jendela | Ditambahkan |
