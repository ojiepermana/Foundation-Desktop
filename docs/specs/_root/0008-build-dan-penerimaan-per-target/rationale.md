# 0008. Build dan penerimaan per target: alasan

Catatan keputusan untuk [index.md](index.md). `/develop` tidak perlu membaca file ini.

## Context

Pada 2026-11-15 (`main` di `9b84e6c`) repo punya Cargo Workspace dengan tiga anggota, aplikasi macOS, binding Swift yang dilacak git, dan alat `desktop-codegen` dengan sumber `tokens`, `config`, `localization`, dan `assets`. Sumber `architecture` (spec `_root/0007`) masuk `main` di `96b8279` saat spec ini ditulis. Gerbang merge adalah empat perintah `cargo` di `AGENTS.md` bagian Commands; gerbang Mac `G-1` sampai `G-5` ditetapkan spec `macos/0008` (scope 002 macOS, diterima lead, belum di `main`). Semua perintah itu hanya tertulis di spec dan dijalankan orang dengan menyalinnya. `scripts/` hanya berisi `update_graphify.py` milik hook, dan belum ada `.github/workflows/`.

Gaya yang membentuk keputusan:

* **Kebutuhan dari scope 012.** Baris **Done when**: tim dapat mengulang build dan uji, menelusuri hasil aktual tiap OS dan CPU, dan membuktikan tiga integrasi wajib sebelum fondasi dinyatakan selesai. Kriteria 1, 28, 29, 30, dan 31 di `docs/quality/README.md` adalah satu satunya kriteria yang menyebut scope 012.
* **Rancangan yang sudah tertulis.** `docs/quality/README.md` menamai delapan skrip di `scripts/`, meminta runner sesuai OS, versi toolchain yang dinilai cocok, izin minimum, kredensial di luar repo, dan tanpa pipeline rilis. Bagian "Bukti native per host" meminta setiap hasil mencantumkan OS, arsitektur, versi, perintah, hasil, dan batas pengujian, dan menyatakan uji jendela tetap belum terverifikasi bila runner hanya bisa membangun.
* **Sembilan baris Follow-up Scope 012** di spec lain meminta skrip dan CI membungkus perintah mereka dengan flag yang sama.
* **Satu host.** Hanya Mac Apple Silicon yang tersedia. Windows, Linux, push, dan run GitHub Actions adalah langkah manual user. Belum ada aplikasi Windows atau Linux.
* **Batas yang mengikat.** `.githooks/`, `scripts/update_graphify.py`, dan `.cargo/config.toml` terlarang (jadi tidak ada alias `cargo xtask`). Kriteria 32 melarang alat untuk fitur mendatang. Unduhan di luar Cargo dan SwiftPM adalah langkah manual.
* **Bash di macOS.** `/bin/bash` di Mac ini versi `3.2.57`. Bash yang lebih baru hanya ada bila dipasang sendiri.

Bila keputusan ini ditunda, setiap pemeriksa terus menyalin perintah dari spec dengan risiko salah flag, bukti tiap host ditulis dengan bentuk berbeda, dan tidak ada tempat yang berkata kapan fondasi selesai.

## Options considered

### Opsi 1: Skrip bash tipis, workflow yang hanya memanggil skrip, laporan yang hidup

Enam skrip `bash` dan satu pembantu di `scripts/` membungkus perintah spec lain apa adanya dan mencetak keluaran berbentuk bukti. Workflow GitHub Actions memanggil skrip yang sama. Satu berkas laporan mengumpulkan hasil per target.

**Pros**: mengikuti rancangan `scripts/` yang sudah tertulis; run lokal dan CI identik; tanpa dependensi baru; jalan di ketiga host (Git Bash di Windows).
**Cons**: butuh Git Bash di Windows; dibatasi fitur bash `3.2`; perintah tertulis dua kali (spec pemilik dan skrip).

### Opsi 2: Paket Rust kecil sebagai pengganti skrip (`cargo run -p desktop-ci`)

Satu anggota workspace baru menjalankan langkah yang sama dari Rust, sehingga tidak butuh bash dan sama persis di tiga host.

**Pros**: lintas platform tanpa shell; bisa diuji dengan `cargo test`.
**Cons**: anggota baru berarti baris `MEMBERS`, baris tabel arah dependensi, dan spec pemilik; tanpa `.cargo/config.toml` perintahnya panjang; memanggil `xcodebuild` dan `swift` dari Rust menambah kode tanpa manfaat; tidak sesuai rancangan `scripts/` di `docs/quality/README.md`; bertentangan dengan semangat kriteria 32.

### Opsi 3: Perintah langsung di workflow, tanpa skrip

Workflow memuat perintah `cargo`, `swift`, dan `xcodebuild` sendiri.

**Pros**: paling sedikit berkas.
**Cons**: kriteria 28 meminta skrip build; run lokal berbeda dari CI; bentuk bukti tidak seragam; YAML menjadi tempat kedua untuk perintah.

### Opsi 4: `make` atau `just`

Satu Makefile atau justfile dengan target per kebutuhan.

**Pros**: pola yang dikenal; dependensi antar target.
**Cons**: `just` butuh unduhan; `make` tidak ada di Windows tanpa pemasangan tambahan; bentuk bukti tetap harus ditulis sendiri.

## Rationale

Opsi 1 dipilih karena ia satu satunya yang memenuhi kriteria 28 apa adanya (skrip build yang berfungsi di host) tanpa menambah anggota, alat, atau unduhan. Rancangan `scripts/` di `docs/quality/README.md` sudah menamai skripnya, dan Follow-up spec lain menunjuk nama yang sama, jadi Opsi 1 tidak memperkenalkan konsep baru. Workflow yang hanya memanggil skrip membuat setiap kegagalan CI bisa diulang di mesin sendiri, yang paling penting saat host Windows dan Linux belum ada dan bukti CI menjadi satu satunya bukti di sana. Opsi 2 kuat untuk lintas platform, tetapi biaya anggota baru dan aturan arsitekturnya tidak sebanding untuk membungkus belasan perintah. Biaya Opsi 1 yang paling nyata, Git Bash di Windows, sudah terpenuhi di runner GitHub dan di hampir setiap mesin pengembang Windows yang memakai git.

Laporan fondasi dibuat sebagai satu berkas yang hidup, bukan laporan sekali tulis di akhir, karena Done when meminta hasil bisa ditelusuri per target sepanjang jalan, dan kriteria 31 meminta hasil aktual. Aturan **S-1** sampai **S-5** mencegah laporan memuat `PASS` tanpa catatan, mengikuti kalimat "Bukti native per host" bahwa build yang berhasil hanya bukti kompilasi.

## Keputusan yang diambil

Mode keputusan `auto`. Setiap baris: pertanyaan, pilihan, pilihan kedua, tanda, dan apa yang berubah bila pilihan kedua yang diambil.

| No. | Pertanyaan | Diambil | Pilihan kedua | Tanda | Bila pilihan kedua |
| --- | --- | --- | --- | --- | --- |
| D-1 | Bahasa skrip | `bash` setara `3.2`, dipanggil `bash scripts/<nama>.sh` | Paket Rust (Opsi 2) | biasa | Semua AC skrip dan Build plan langkah 1 sampai 3 ditulis ulang; `MEMBERS` bertambah |
| D-2 | Skrip host Windows dan Linux sekarang | Tidak dibuat; skrip bersama dipakai sampai scope 006 dan 007 | Buat `build-windows.ps1` dan `build-linux.sh` sekarang sebagai pembungkus skrip bersama | biasa | AC-16 berubah; dua berkas baru tanpa isi khusus OS |
| D-3 | Langkah yang gagal | `validate.sh`, `build-rust.sh`, `test.sh` lanjut; skrip macOS dan generator berhenti | Semua skrip berhenti di langkah gagal pertama | biasa | AC-3 hanya melihat satu langkah gagal |
| D-4 | Asal daftar sumber `desktop-codegen` | Baris `sources:` dari pesan usage alat | Daftar tetap di `validate.sh` | biasa | Setiap sumber baru butuh pembaruan skrip; AC-2 memakai angka tetap |
| D-5 | Tempat bukti run | Keluaran skrip (log), plus `$GITHUB_STEP_SUMMARY` di CI | Berkas log di `target/evidence/` dan artefak unggahan CI | biasa | Satu action unggah tambahan dan aturan retensi |
| D-6 | Runner macOS | `macos-26` dengan Xcode bawaan (26.6 pada citra 2026-11-02) | Label pratinjau `xcode-27`, cocok dengan Xcode `27.1` lokal | biasa | Blok **Workflow**; risiko label pratinjau berubah |
| D-7 | Kepercayaan action dan izin CI | Hanya `actions/checkout`, dipaku ke SHA, `contents: read`, `persist-credentials: false`, tanpa rahasia | Tag major `actions/checkout@v7` | berat | Satu baris `uses:`; AC-10 tidak lagi menuntut SHA |
| D-8 | Pemasangan toolchain di CI | `rustup toolchain install` tanpa argumen, dicek langkah 1 skrip | Action pihak ketiga untuk Rust | biasa | Satu action pihak ketiga lagi; D-7 berubah |
| D-9 | Runner ARM Windows dan Linux | Belum dipakai | Tambah `ubuntu-24.04-arm` dan `windows-11-arm` ke matriks | biasa | Matriks `rust` bertambah dua; baris ARM di laporan mendapat bukti Rust |
| D-10 | Lokasi laporan fondasi | `docs/quality/laporan-fondasi.md` (menunggu lead) | `docs/reviews/laporan-fondasi.md` | biasa | Path di AC-13, AC-14, bagian **Laporan fondasi**, dan tabel berkas; tanpa izin `docs/quality/` |
| D-11 | Arti "tiga integrasi wajib" | Integrasi sapaan di jendela native macOS, Windows, dan Linux | Tiga kelompok pemeriksaan (Core, binding, build native) | berat | AC-14 dan bagian "Tiga integrasi wajib"; aturan **S-5** |
| D-12 | Akhir baris checkout Windows | Bawaan runner (CRLF untuk berkas tanpa atribut) | Paksa LF untuk seluruh repo | biasa | Satu langkah `git config` sebelum checkout; bukti CRLF hilang |
| D-13 | Cache CI | Tanpa cache | `actions/cache` untuk `~/.cargo` dan `target/` | biasa | Action kedua; aturan kunci cache |
| D-14 | `[profile.release]` | Tidak dibutuhkan | Atur `strip` atau `lto` untuk build Release | biasa | Pembaruan spec `_root/0001` dan `Cargo.toml` |
| D-15 | Langkah manual host | Satu `F012-1` yang memenuhi `F001-1`, `F002-1`, `F008-1` | Tiga langkah lama dijalankan terpisah | biasa | Bagian **Daftar penerimaan per target** |
| D-16 | Uji Swift | Hanya lewat `G-3` di `build-macos.sh`; `test.sh` hanya Rust | Juga di `test.sh` pada Darwin `arm64` | biasa | AC-4 menjadi empat langkah di Mac; `swift test` berjalan dua kali di CI |
| D-17 | Bentuk gerbang Mac di skrip | `build-macos.sh` = `config check`, `G-1` sampai `G-5` apa adanya, lalu build Release | Perintah build dan uji sendiri dari spec `macos/0001` (draf pertama spec ini) | biasa | AC-5 dan AC-6; pemeriksaan binding ganda dengan `G-2` |
| D-18 | Pola `R` di skrip dan CI | Baris catatan `macos gate:` di `validate.sh`; di CI workflow macOS terpisah dengan filter `paths:` hasil terjemahan `R`, ditambah `scripts/**` dan berkas workflow itu | Satu workflow, job macOS selalu berjalan | biasa | Satu berkas workflow; AC-10 dan AC-11; setiap perubahan dokumen memakai menit runner macOS |
| D-19 | Akhir `generate-bindings.sh` | Baris `bindings/swift: unchanged` atau `changed`, tanpa gagal | `git diff --exit-code -- bindings/swift` dari blok **Membuat ulang binding** | biasa | AC-7; alat pembuat ulang gagal justru saat ia bekerja |
| D-20 | `test.sh` di CI | Tidak dijalankan; `validate.sh` mencetak `rust tests:` | Jalankan juga | biasa | `cargo test` dua kali per job |
| D-21 | Pembatalan run CI | Hanya untuk `pull_request` | Batalkan semua run lama pada ref yang sama | biasa | Run `main` bisa batal, sehingga AC-11 tidak punya run untuk commit itu |

Alasan singkat per baris:

* **D-1.** Lihat Rationale. Bash `3.2` karena itulah yang ada di setiap Mac tanpa pemasangan.
* **D-2.** Belum ada yang khusus Windows atau Linux untuk dibangun. Skrip kosong melanggar kriteria 32 dan harus ditulis ulang saat aplikasinya ada.
* **D-3.** Untuk bukti, melihat semua langkah gerbang yang gagal sekaligus lebih berguna. Langkah macOS saling bergantung, dan spec `macos/0008` menuntut setiap perintah gerbang lulus sebelum berikutnya, jadi melanjutkan hanya menambah kegagalan turunan.
* **D-4.** Bentuk baris `sources:` sudah dikunci AC-1 spec `_root/0007`, jadi sumber `architecture` dan sumber kelak ikut tanpa menyentuh skrip. Daftar tetap akan tertinggal.
* **D-5.** Log CI sudah tersimpan oleh GitHub; ringkasan job menaruh hasil di halaman run tanpa action tambahan.
* **D-6.** Citra `macos-26` adalah standar dan stabil; label `xcode-27` masih pratinjau. Proyek memakai `objectVersion = 77` dan `swift-tools-version: 6.0`, jadi Xcode 26 bisa membangunnya. Perbedaan kombinasi dicatat, bukan disembunyikan.
* **D-7.** Model kepercayaan CI termasuk keputusan keamanan, jadi bertanda `berat` walau mudah dibalik. Memaku SHA mencegah tag yang dipindah diam diam; satu action menjaga permukaan tetap kecil.
* **D-8.** `rustup 1.29.3` menyatakan `toolchain install` tanpa argumen memasang toolchain aktif, yang dibaca dari `rust-toolchain.toml`. Langkah 1 skrip menangkap bila yang terpasang berbeda.
* **D-9.** Runner ARM gratis hanya untuk repo publik, dan visibilitas repo adalah nilai dari user.
* **D-10.** `docs/quality/README.md` menjelaskan isi laporan tetapi tidak lokasinya. Di sebelah tabel kriteria adalah tempat yang paling mudah ditemukan, dengan biaya izin `docs/quality/` di brief developer. `docs/reviews/` berisi catatan bertanggal per pemeriksaan, bukan dokumen yang terus diperbarui.
* **D-11.** Kalimat pembuka `docs/quality/README.md` berbunyi fondasi lengkap setelah integrasi wajib pada ketiga OS punya bukti, dan setiap rujukan platform punya baris "Integrasi" sendiri. Tafsiran ini menentukan kapan fondasi selesai, jadi bertanda `berat`.
* **D-12.** Pengembang Windows memakai pengaturan bawaan git. CI yang sama dengan mereka membuktikan aturan teks spec `_root/0007` menerima `\r\n`.
* **D-13.** Belum ada waktu run yang terukur. Cache menambah action dan cara gagal baru.
* **D-14.** Fase skrip Xcode sudah memanggil `cargo build --release` untuk Release; tidak ada kebutuhan ukuran atau kecepatan yang terukur.
* **D-15.** Brief lead meminta satu daftar penerimaan per OS dan CPU tanpa teks ganda. Keluaran skrip memuat setiap baris yang diminta tiga langkah lama.
* **D-16.** Kriteria 30 meminta uji yang relevan dijalankan; di Mac uji Swift dijalankan `G-3`. Menjalankannya juga di `test.sh` hanya mengulang kerja.
* **D-17.** Lead meneruskan bahwa spec `macos/0008` (diterima, scope 002 macOS) menetapkan gerbang Mac supaya spec ini hanya membungkusnya. `G-2` membandingkan binding di `target/binding-check/` tanpa menulis berkas terlacak, jadi lebih tepat untuk CI daripada blok **Membuat ulang binding** yang menghapus `bindings/swift/Sources`. `config check` ditaruh sebelum `G-1` atas permintaan Follow-up spec `macos/0004`, dan build Release sesudah `G-5` atas AC-4 spec `macos/0001`, supaya urutan `G-1` sampai `G-5` sendiri tidak berubah.
* **D-18.** Follow-up Scope 012 spec `macos/0008` meminta filter path job macOS memakai `R`. Sebagian besar commit di repo ini hanya dokumen, dan menit runner macOS paling mahal, jadi filter itu menghemat tanpa mengurangi bukti untuk perubahan yang memengaruhi aplikasi. Filter `paths:` berlaku per workflow, bukan per job, sehingga job macOS pindah ke berkas sendiri. Glob ditulis satu lawan satu dari regex `R`. `scripts/**` dan berkas workflow itu sendiri ditambahkan karena keduanya lahir sesudah `R` dan langsung mengubah apa yang dijalankan job macOS. Checkout CI tidak punya `main` lokal, jadi baris `macos gate:` hanya berguna di mesin lokal.
* **D-19.** Kegagalan `git diff` di akhir blok **Membuat ulang binding** berarti "binding berubah, commit", yang memang tujuan menjalankan alat itu. Pemeriksaan binding sekarang milik `G-2`, jadi alat pembuat ulang cukup melaporkan apakah ada yang berubah.
* **D-20.** Usul pemeriksaan silang: `cargo test` dua kali per job hanya memakan menit.
* **D-21.** Usul pemeriksaan silang: dengan pembatalan untuk semua event, push kedua ke `main` membatalkan run commit pertama, sehingga commit itu tidak punya bukti CI.

## Bukti percobaan

Dijalankan 2026-11-15 di Mac koordinasi (macOS `27.0.1`, `arm64`, `rustc 1.99.1`, `cargo 1.99.1`), dari folder sementara di luar repo. Run pertama pada `9b84e6c`, run kedua pada `96b8279` sesudah sumber `architecture` masuk `main`.

* `/bin/bash --version` mencetak `GNU bash, version 3.2.57(1)-release (arm64-apple-darwin26)`. Prototipe `lib.sh` dan `validate.sh` (fungsi `step`, `SECONDS`, `set -u`) berjalan dengan bash itu.
* Prototipe `validate.sh` keluar dengan kode `0` dalam sekitar 31 detik dengan build hangat: `== validate.sh: PASS, 9 steps, 0 failed` (tanpa langkah toolchain, yang ditambahkan sesudahnya). Langkah per sumber mencetak `tokens: ok, 6 semantic tokens, 3 placeholder values, 3 outputs up to date`, `config: ok, schema version 1, 2 build values, 1 outputs up to date`, `localization: ok, 1 keys, 2 locales, 0 missing translations, 1 outputs up to date`, dan `assets: ok, 1 assets, 1 source files, 1 placeholder values, 1 outputs up to date`. `brand exports: 0 records`. Jumlah angka `passed` di semua baris `test result:` adalah `137`.
* Pada `96b8279`: prototipe keluar dengan kode `0` dengan 10 langkah (tanpa langkah toolchain dan langkah daftar sumber), sumber kelima mencetak `architecture: ok, 11 rules, 3 members, 0 outputs up to date`, dan jumlah `passed` adalah `159`. Dengan `printf '\n' >> design-system/schemas/colors.schema.json`, prototipe keluar dengan kode `1` dan `FAIL, 10 steps, 2 failed (steps 4 6)`: langkah `cargo test` (uji `every_source_passes_check_on_the_repo`) dan langkah `tokens check` (`differs from the generator output`). `architecture check` tetap lulus. Dengan langkah toolchain dan langkah daftar sumber di depan, nomornya menjadi `steps 5 8` seperti di AC-3.
* Perintah `sed` channel di **Kerangka `lib.sh`** mencetak `1.99.1` tanpa karakter lain. Perintah **Pemeriksaan tautan** dengan `/bin/bash` mengubah `%20` menjadi spasi dan melaporkan path yang tidak ada.
* `cargo run -p desktop-codegen --locked` tanpa argumen keluar dengan kode `2` dan mencetak `sources: tokens, config, localization, assets` di stderr pada `9b84e6c` (ditambah `architecture` pada `96b8279`).
* `git ls-remote https://github.com/actions/checkout 'refs/tags/v7*'` mencetak `3d3c42e5aac5ba805825da76410c181273ba90b1` untuk `refs/tags/v7` dan `refs/tags/v7.0.1`.
* `rustup --version` mencetak `rustup 1.29.3`, dan `rustup toolchain install --help` menyatakan perintah itu memasang toolchain aktif bila tanpa argumen.
* `apps/macos/DesktopMac.xcodeproj/project.pbxproj` memuat `objectVersion = 77`, `MACOSX_DEPLOYMENT_TARGET = 14.0`, dan fase skrip yang memanggil `cargo` dari `$HOME/.cargo/bin`.
* Kedua blok **Workflow** di `index.md`, diambil dengan fungsi `blk` AC-10, dibaca dengan `ruby -ryaml` (Ruby `2.6.10` bawaan macOS) tanpa galat: `foundation` dengan job `rust` dan `foundation-macos` dengan job `macos`, masing masing dipicu `push`, `pull_request`, dan `workflow_dispatch`. Kedua pemeriksaan `grep` di AC-10 mencetak `0` untuk tiap blok.

Fakta runner GitHub dari pemeriksaan web 2026-11-15 (subagent baca saja): `ubuntu-latest` adalah Ubuntu 24.04, `windows-latest` adalah Windows Server 2025, `macos-latest` adalah macOS 26 `arm64`; citra `macos-26` `arm64` 20261102 memasang Xcode 26.0.1 sampai 26.6 dengan 26.6 sebagai bawaan dan tanpa Xcode 27 (Xcode 27 hanya lewat label pratinjau `xcode-27`); `ubuntu-24.04-arm` dan `windows-11-arm` gratis untuk repo publik dan berbayar untuk repo privat; citra Windows 2025 memasang Git Bash `5.3.15`. Fakta ini bisa berubah; skrip mencatat versi nyata di setiap run.

## Cross check

Pemeriksaan silang baca saja oleh subagent `claude-sonnet-5-5` (penulis spec: `claude-opus-5-5`, family yang sama; Fable tidak dipakai sesuai aturan koordinasi sejak 2026-11-15 19:24). Ia membaca draf pertama, tidak menjalankan apa pun, dan menemukan bahwa kesembilan baris Follow-up Scope 012 terjawab dan kriteria 1, 28 sampai 31 serta Done when tercakup. Mode keputusan `auto`, jadi semua perbaikan yang direkomendasikan diterapkan. Draf kedua juga menyelaraskan spec dengan `macos/0008` (D-17 sampai D-19), yang tiba lewat lead pada saat yang sama, dan sesudah `macos/0008` masuk `main` (`5f7e69d`) menjawab Follow-up Scope 012 di sana dengan filter `R` (D-18). Bagian gerbang Mac dan pola `R` di `main` sama dengan teks yang dipakai draf kedua; yang bertambah hanya kalimat alasan.

| No. | Temuan | Penerapan |
| --- | --- | --- |
| 1 | Baris Swift Testing tidak diawali `Test run with` dan tidak diakhiri `passed` | AC-5 memakai pola `Test run with [0-9]+ tests? in [0-9]+ suites? passed`; uji Swift pindah ke `G-3` |
| 2 | AC kriteria 32 bertabrakan dengan berkas `graphify-out/` dari hook | AC-16 menyaring `^graphify-out/` |
| 3 | Jumlah gagal di AC-3 belum dibuktikan | Diukur pada `96b8279` (bagian Bukti percobaan); AC-3 menyebut nomor langkah dengan rumus |
| 4 | Pemeriksaan mode berkas AC-13 tidak tepat | AC-15 memakai `awk` atas `git ls-files -s` |
| 5 | Kecocokan bash `3.2` tidak benar benar diuji | Semua AC Mac memakai `/bin/bash`; aturan tanpa array |
| 6 | `brand-export.txt` CRLF di Windows | Baris `.gitattributes` tetap milik spec OS (spec `_root/0005`); kasus gagal dan Follow-up ditambah |
| 7 | Tanda `\|` di label merusak tabel ringkasan | `summary_row` menulis `\|` dan backtick |
| 8 | Pembatalan run `main` | D-21 |
| 9 | Rujukan Windows tidak punya baris "Integrasi" | Sumber diganti ke "Bukti native per host" dan bagian "Bukti build, runtime dan distribusi" |
| 10 | Cara `step` menjalankan perintah belum ditetapkan | Bagian **Kerangka `lib.sh`** |
| 11 | Baris `sources:` belum tentu baris kedua stderr; daftar kosong; `wc -l` berspasi | `grep '^sources: '`, langkah 7 `validate.sh`, `brand exports: <m> files` |
| 12 | Cara membaca channel dan `1.99.10` | Perintah `sed` dan awalan dengan spasi |
| 13 | Perintah baris fakta macOS belum lengkap | Tabel baris tambahan menyebut perintah dan path |
| 14 | Penjaga host tidak bisa diuji di satu Mac | AC-9 dengan `uname` palsu; argumen diperiksa sebelum host |
| 15 | Isi sel laporan belum ditetapkan | Tabel **Pengisian pertama** dan aturan **S-4** yang diperjelas |
| 16 | Hitungan baris laporan tanpa perintah; `grep` tanpa berkas | Fungsi `sec` di AC-13 dan AC-14; bagian **Pemeriksaan tautan** |
| 17 | Nilai `**Status**` di milestone 1 | Langkah 1 Build plan: `In Progress` |
| 18 | `git diff --exit-code` tidak melihat berkas baru | D-19: `git status --porcelain -- bindings/swift` |
| 19 | AC-1 butuh commit dulu | Kalimat pembuka AC dan Build plan: jalankan AC sesudah commit |
| 20 | Kerja ganda `cargo test` | D-20; sisanya dicatat di Consequences |
| 21 | `build-rust.sh` tidak di CI | Dicatat di AC-11 |
