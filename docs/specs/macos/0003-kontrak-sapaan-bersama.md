# 0003. Kontrak sapaan bersama di macOS

**Scope**: [003 - Kontrak sapaan bersama](../../scope/003%20-%20Kontrak%20sapaan%20bersama.md)
**Date**: 2026-11-15
**Status**: In Progress

## Summary

Spec ini menjelaskan bagaimana kontrak `hello` dari [spec `_root/0002`](../_root/0002-kontrak-sapaan-bersama/index.md) dipakai di Mac: lewat jalur apa UI memanggilnya, aturan kontrak mana yang mengikat kode macOS, dan bukti apa yang menunjukkannya. Spec ini tidak memutuskan ulang kontrak dan tidak menambah kode atau uji. Semua bukti sudah ada di spec `_root/0002` atau dihasilkan oleh [spec macOS 0001](0001-sapaan-di-jendela-macos/index.md), jadi di sini cukup dirujuk. Karena itu spec ini tidak punya langkah build.

## Context

Berkas fitur scope 003 berkata bagian spec tiap OS menjelaskan bagaimana UI memanggil fungsi sapaan. Untuk Mac, jalur itu dirancang di spec macOS 0001 (scope 004). Kontraknya sendiri, termasuk vektor V-1 sampai V-11 dan aturan R-1 sampai R-9, sudah tetap di spec `_root/0002` dan sudah terbukti di Mac koordinasi.

Yang belum tertulis di satu tempat adalah jawaban untuk pemeriksa: untuk kriteria 2, 3, dan 22, mana bukti yang diambil di Mac dan pada host apa, bagaimana tiap aturan R diterapkan oleh kode macOS, vektor mana yang dibuktikan lewat binding dan mana yang hanya di Core, dan apa yang sengaja tidak berlaku di Mac. Tanpa itu, kotak scope 003 untuk macOS tidak punya kontrak yang bisa diperiksa, dan developer macOS berikutnya harus mencari sendiri aturan mana yang mengikat kodenya.

Hanya ada satu host: Mac Apple Silicon (`arm64`) dengan macOS 27.0.1 dan Rust 1.99.1.

## Requirements

**User stories**:

* Sebagai pemeriksa, saya ingin satu daftar yang menunjuk bukti kontrak sapaan di Mac, dari uji Core sampai nilai di jendela.
* Sebagai developer aplikasi macOS, saya ingin tahu aturan kontrak mana yang mengikat jembatan dan UI saya, dan apa yang terjadi saat kontrak berubah.

**Acceptance criteria**:

Angka kriteria merujuk tabel di [Penerimaan dan bukti fondasi](../../quality/README.md). Setiap AC berlaku untuk host macOS pada `arm64`. "Spec bersama" berarti spec `_root/0002`, dan "spec 004" berarti spec macOS 0001. Semua perintah dijalankan dari akar repo.

Kotak `[x]` di `verify.md` dicentang oleh `/check verify` spec pemilik berkas itu. Spec ini tidak punya `verify.md` sendiri. Hasil perintah yang dijalankan langsung untuk spec ini dicatat `/check verify` di `docs/reviews/`, bersama host, arsitektur, versi, perintah, dan hasilnya.

* **AC-1** (Done when scope 003, kriteria 3): uji Core tanpa UI membuktikan `hello("Ayu".to_string())` menghasilkan `Hello, Ayu!` di Mac. Di `docs/specs/_root/0002-kontrak-sapaan-bersama/verify.md` bagian Milestone 1, langkah `cargo test -p desktop-core --locked --test hello_contract` bertanda `[x]`. Saat spec ini diverifikasi, perintah itu dijalankan ulang di Mac dan keluar dengan kode 0.
* **AC-2** (kriteria 2): Core dibangun dan diuji tanpa pustaka UI di Mac. Di `verify.md` spec bersama, semua langkah untuk AC-2 di Milestone 1 dan Milestone 2 bertanda `[x]`. Sesudah anggota macOS bergabung, `cargo tree -p desktop-core --target all --all-features --prefix none --locked` tetap mencetak tepat satu baris. Itu perintah terakhir **AC-9** spec 004.
* **AC-3** (kriteria 3): UI macOS tidak punya salinan logika sapaan. Langkah `git grep --untracked -lF 'Hello, {' -- '*.rs'` di `verify.md` spec bersama bertanda `[x]`. **AC-7** spec 004 (jembatan hanya meneruskan) dan **AC-8** spec 004 (tidak ada `Hello, ` di Swift selain uji) tercatat `[x]` di `verify.md` spec 004.
* **AC-4** (kalimat berkas fitur "bagaimana UI memanggil fungsi itu", kriteria 3): jalur panggilan di bagian **Jalur panggilan di Mac** terbukti. Tanpa jendela: **AC-2** dan **AC-3** spec 004 tercatat `[x]` di `verify.md` spec 004. Di jendela nyata (`owner: user`): langkah manual `F004-1` membuktikan tombol `Greet` (**AC-5** spec 004) dan `F004-2` membuktikan tombol Return (**AC-13** spec 004). Keduanya berstatus `PASS` di tabel berkas manual koordinasi dan tercatat di `verify.md` spec 004 bagian `## Milestone 4: Sesi GUI di Mac`.
* **AC-5** (kriteria 22): aturan kontrak publik bersama diterapkan oleh kode macOS seperti tabel **Aturan kontrak di Mac**. Bukti yang dikutip tabel itu tercatat `[x]` di `verify.md` spec 004: **AC-2**, **AC-3**, **AC-6**, **AC-7**, **AC-8**, dan **AC-9** spec 004. Baris yang bertanda "tinjauan" tidak punya bukti berupa perintah.
* **AC-6** (kriteria 3): cakupan vektor di Mac sesuai tabel **Vektor di Mac**. V-1 sampai V-11 terbukti di Core lewat langkah `Tabel VECTORS ... dibandingkan` di `verify.md` spec bersama bagian Milestone 2, yang bertanda `[x]`. V-1, V-2, V-6, V-7, V-10, dan V-11 terbukti lewat binding oleh **AC-2** spec 004. Kolom ViewModel terbukti oleh **AC-3** spec 004.
* **AC-7** (catatan per target, mendukung kriteria 2 dan 3): bukti yang dirujuk menyebut host, arsitektur, dan versi. ``grep -c 'macOS 27.0.1, `arm64`, `rustc 1.99.1' docs/specs/_root/0002-kontrak-sapaan-bersama/verify.md`` mencetak `1`, dan `grep -c '^- \[ \]' docs/specs/_root/0002-kontrak-sapaan-bersama/verify.md` mencetak `0`. `verify.md` spec 004 memuat keluaran perintah **AC-10** spec 004 (versi macOS, arsitektur, Xcode, Swift, Rust, dan UniFFI).

## Options considered

### Opsi 1: Rujuk bukti yang ada, tanpa langkah build

Spec ini memetakan tiap kriteria, aturan, dan vektor ke bukti di spec bersama dan spec 004.

**Pros**:
* Tidak ada kode, uji, atau catatan yang ditulis dua kali.
* Aturan kontrak untuk kode macOS bisa dibaca di satu tabel.

**Cons**:
* Sebagian AC baru bisa diperiksa sesudah milestone spec 004 selesai.
* Pemeriksa harus membuka `verify.md` spec lain.

### Opsi 2: Uji kontrak macOS sendiri untuk semua vektor

Spec ini menambah uji Swift untuk V-3, V-4, V-5, V-8, dan V-9 lewat binding, supaya sebelas vektor terbukti di kedua sisi.

**Pros**:
* Cakupan lewat binding menjadi penuh.

**Cons**:
* Mengubah berkas uji yang isinya ditetapkan spec 004, jadi butuh pembaruan spec itu.
* Lima vektor itu berbeda dari yang sudah diuji hanya pada isi byte, dan jembatan menyalin byte apa adanya. Vektor yang menguji batas (kosong, Unicode, NUL, sangat panjang) sudah lewat binding.

### Opsi 3: Gabungkan ke spec macOS 0001

Tidak ada spec terpisah. Pemetaan ini menjadi satu bagian di spec 004.

**Pros**:
* Satu berkas untuk semua hal macOS tentang sapaan.

**Cons**:
* Panduan scope meminta spec per OS untuk tiap fitur bersama, dan kotak scope 003 untuk macOS butuh spec yang bisa ditautkan.
* Spec 004 sudah diterima dan sedang dibangun.

## Decision

**Chosen option**: Opsi 1: rujuk bukti yang ada, tanpa langkah build.

Bukti scope 003 di Mac adalah catatan `verify.md` spec `_root/0002` dan spec macOS 0001. Spec ini hanya menulis pemetaannya.

## Rationale

Kontrak dan ujinya sudah lengkap di Core, dan host buktinya adalah Mac koordinasi itu sendiri. Jalur dari UI ke Core sudah dirancang dan diuji oleh spec 004. Menambah uji (Opsi 2) berarti mengubah berkas yang isinya dimiliki spec lain demi vektor yang tidak menguji hal baru di batas FFI. Menggabungkan (Opsi 3) melanggar cara menurunkan scope yang sudah ditetapkan. Jadi nilai spec ini ada pada pemetaan yang bisa diperiksa, bukan pada pekerjaan baru.

Cross check oleh model lain (Claude Opus, hanya membaca, 2026-11-15) mencocokkan tabel vektor dengan berkas uji di spec 004 dan membenarkannya. Temuannya sudah diterapkan: satu kalimat yang ternyata keputusan baru (jembatan hanya mengekspor API yang dipakai aplikasi) dihapus, bukti tiap baris tabel aturan disamakan dengan AC yang benar benar membuktikannya, baris tanpa bukti perintah ditandai "tinjauan", bagian "Saat kontrak berubah" diringkas menjadi rujukan, dan langkah manual `F004-2` ikut dirujuk.

## Feature design

**Data model sketch**: tidak ada.

**State transitions**: tidak ada.

**API surface**: tidak ada API baru. Permukaan yang dipakai Mac adalah `desktop_core::hello` (spec bersama) dan pembungkusnya `desktop_macos_ffi::hello` (spec 004).

**Jalur panggilan di Mac**:

| Urutan | Lapisan | Yang dilakukan | File |
| --- | --- | --- | --- |
| 1 | View SwiftUI | Tombol `Greet` atau Return memanggil `greet()` | `apps/macos/Presentation/GreetingView.swift` |
| 2 | ViewModel | Mengirim `name` apa adanya dan menyimpan hasil apa adanya | `apps/macos/Presentation/GreetingViewModel.swift` |
| 3 | Adapter | `CoreGreeter.hello(name:)` memanggil binding | `apps/macos/Interop/Greeter.swift` |
| 4 | Binding Swift | Menyalin nama ke Rust dan hasil ke Swift | `bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift` (hasil generator) |
| 5 | Jembatan | Meneruskan ke Core, tanpa perilaku sendiri | `crates/macos-ffi/src/lib.rs` |
| 6 | Core | Membentuk `Hello, {name}!` | `crates/core/src/greeting.rs` |

Hasil kembali lewat jalur yang sama dan ditampilkan dengan `Text(verbatim:)`. Rinciannya ada di spec 004, bagian API surface dan Kepemilikan di batas FFI.

**Aturan kontrak di Mac**:

| Aturan spec bersama | Penerapan di Mac | Bukti |
| --- | --- | --- |
| R-1 Satu permukaan | Jembatan hanya memakai item dari akar crate `desktop_core`. Modul Core privat, jadi jalur lain tidak terkompilasi. | **AC-7** spec 004 dan AC-4 spec bersama |
| R-2 Spec dulu | API publik Core baru sampai ke Swift sesudah spec `_root` yang menambahnya diterima dan spec 004 diperbarui | Tinjauan. Lihat **Saat kontrak berubah**. |
| R-3 Nilai yang dimiliki | Setiap nilai disalin saat melintas, jadi tanda tangan Swift sama dengan Rust: `hello(name: String) -> String` | **AC-6** dan **AC-7** spec 004 |
| R-4 Kesalahan bertipe | `hello` tidak bisa gagal, jadi fungsi Swift tidak bertanda `throws`. Belum ada tipe kesalahan yang dipetakan. | **AC-7** spec 004 |
| R-5 Kompatibilitas dan versi | Versi `desktop-macos-ffi` diwarisi dari `[workspace.package]`, jadi naik bersama Core. Binding yang tidak dibuat ulang sesudah kontrak berubah menghentikan uji dan aplikasi lewat pemeriksaan checksum. | **AC-9** spec 004 (`desktop-macos-ffi v0.1.0`) dan uji `workspace_layout`. Pemeriksaan checksum adalah rancangan spec 004 dan dicoba di percobaannya, tanpa AC. |
| R-6 Aman, tanpa efek samping | Jembatan memakai `unsafe_code = "forbid"`, tidak mencatat log, dan tidak menyimpan nama | **AC-6** spec 004 untuk `unsafe`. Tanpa log dan tanpa penyimpanan: tinjauan (invarian spec 004). |
| R-7 Terdokumentasi | Aturan ini mengikat item publik Core. Pembungkus di jembatan punya rustdoc tanpa contoh doc test, dijaga `missing_docs` dan `clippy` dengan `-D warnings`, dan rustdoc itu ikut tersalin ke binding Swift. | **AC-9** spec 004 |
| R-8 Pemanggil tidak mengubah | Jembatan hanya meneruskan. UI mengirim isi kolom apa adanya, juga saat kosong, dan menampilkan hasil sebagai teks biasa. | **AC-3**, **AC-7**, **AC-8** spec 004 |
| R-9 Diuji tanpa UI | Uji Core berjalan di Mac dengan `cargo test`. Sisi Swift diuji lewat binding dengan `swift test`, tanpa jendela. | `verify.md` spec bersama (Milestone 1 dan 2) dan **AC-2** spec 004 |

**Vektor di Mac**:

| Vektor | Di Core, pada host Mac | Lewat binding Swift | Lewat ViewModel |
| --- | --- | --- | --- |
| V-1 | Ya | Ya | Ya, dengan Core yang asli |
| V-2 | Ya | Ya | Ya, dengan Core yang asli |
| V-3, V-5, V-8, V-9 | Ya | Tidak diulang | Tidak diulang |
| V-4 | Ya | Tidak diulang | Nama `"  Ayu  "` sampai ke greeter tiruan tanpa perubahan |
| V-6, V-7, V-10, V-11 | Ya | Ya | Tidak diulang |

Vektor yang tidak diulang lewat binding hanya berbeda pada isi byte. Jembatan menyalin byte apa adanya, dan itu sudah diuji oleh V-6, V-7, dan V-10. V-9 (teks yang mirip pola format) tetap berisiko rendah: binding dan ViewModel tidak pernah memformat string, dan tampilan memakai `Text(verbatim:)`.

**Saat kontrak berubah**:

Bagian ini hanya menunjuk aturan yang sudah ada, sebagai penerapan R-2 dan R-5 di Mac.

* Jembatan memanggil `desktop_core::hello(name)`. Jadi perubahan tanda tangan di Core mematahkan build `desktop-macos-ffi`, dan invarian `cargo build --workspace --locked` dari spec `_root/0001` memaksa pembungkus ikut berubah dalam perubahan yang sama.
* Sesudah pembungkus berubah, binding dibuat ulang dengan perintah di bagian "Membuat ulang binding" spec 004. Binding yang tertinggal ketahuan lewat baris "API jembatan berubah" di tabel kasus gagal spec 004.
* Fungsi yang bisa gagal atau tipe publik baru butuh pembaruan spec 004 lebih dulu, karena pemetaan kesalahan dan kepemilikan di batas FFI diputuskan di sana.

Yang sengaja tidak berlaku di Mac:

* Mac Intel (`x86_64`). Spec 004 tidak mendukungnya di fase ini.
* Contoh doc test dari R-7. Itu kewajiban item publik Core, bukan pembungkus di jembatan.
* Pemanggilan Core langsung lewat Cargo. Itu jalur Windows dan Linux (scope 006 dan 007). Di Mac, kode Swift tidak pernah memanggil Core tanpa jembatan.
* Batas tanpa panjang untuk nama. Lewat jembatan, satu nama dibatasi sekitar 2 GiB (spec 004). Kontrak Core sendiri tetap tanpa batas.
* Lokalisasi hasil `hello`. Hasil selalu bahasa Inggris. Teks antarmuka macOS milik scope 010.
* Berkas fixture bersama untuk vektor. Uji Swift menulis nilai harapannya sendiri, sesuai Follow-up spec bersama.

**Value sourcing**:

| Aksi | Nilai yang dihasilkan atau ditampilkan | Sumber |
| --- | --- | --- |
| Pemeriksaan AC-1, AC-2, AC-6, AC-7 | Host, arsitektur, versi Rust, dan hasil uji Core | `verify.md` spec bersama, paragraf pembuka dan langkah Milestone 1 dan 2 |
| Pemeriksaan AC-3 sampai AC-7 | Hasil perintah untuk jembatan, binding, dan uji Swift | `verify.md` spec 004, yang ditulis `/develop` per milestone |
| Pemeriksaan AC-4 di jendela | Nilai yang terlihat di label | Langkah manual `F004-1` dan `F004-2` di berkas manual koordinasi |
| Tabel vektor | Daftar vektor yang diuji lewat binding | Berkas uji `BindingContractTests.swift` di spec 004 |

**Key invariants**:

* Spec ini tidak punya keputusan sendiri tentang kontrak. Bila isinya berbeda dari spec bersama atau spec 004, kedua spec itu yang berlaku.
* Di Mac, satu satunya jalur dari UI ke `hello` adalah enam lapisan di **Jalur panggilan di Mac**.
* Berkas binding di git selalu sama dengan keluaran generator untuk commit itu (invarian spec 004).

**Security model**: tidak ada perubahan. Nama tetap teks yang tidak dipercaya dan tidak dicatat atau disimpan (spec bersama dan spec 004).

**Configuration required**: tidak ada.

**Critical test scenarios**:

* Uji Core di Mac: `cargo test -p desktop-core --locked`, memeriksa **AC-1**, **AC-2**, **AC-6**.
* Panggilan lewat binding tanpa jendela: perintah `swift test` dari **AC-2** spec 004, memeriksa **AC-4**, **AC-5**, **AC-6**.
* Satu implementasi: pencarian `git grep` di spec bersama dan **AC-8** spec 004, memeriksa **AC-3**.
* Nilai di jendela nyata: langkah manual `F004-1` dan `F004-2`, memeriksa **AC-4**.
* Kelengkapan catatan: perintah `grep` di **AC-7**, memeriksa **AC-7**.

Tidak ada uji baru. Spec ini memakai uji dan perintah yang sudah ada.

## Build plan

Spec ini tidak punya langkah build. Tidak ada kode, uji, atau dokumen yang perlu dibuat. Setiap AC dipenuhi oleh build spec lain:

| AC | Dipenuhi oleh | Bisa diperiksa sesudah |
| --- | --- | --- |
| **AC-1** | Spec bersama milestone 1 (sudah selesai) | Sekarang |
| **AC-2** | Spec bersama milestone 1 dan 2 (sudah selesai) dan spec 004 langkah 1 | Scope 004 milestone 1 |
| **AC-3** | Spec bersama milestone 2 (sudah selesai) dan spec 004 langkah 1 sampai 4 | Scope 004 milestone 2 |
| **AC-4** | Spec 004 langkah 3 (tanpa jendela) dan langkah 6 (jendela, `owner: user`) | Scope 004 milestone 1, lalu `F004-1` dan `F004-2` |
| **AC-5** | Spec 004 langkah 1 sampai 4 | Scope 004 milestone 2 |
| **AC-6** | Spec bersama milestone 2 (sudah selesai) dan spec 004 langkah 3 | Scope 004 milestone 1 |
| **AC-7** | Spec bersama (sudah selesai) dan spec 004 langkah 5 | Scope 004 milestone 3 |

Kotak `[x]` di `verify.md` spec 004 baru ada sesudah `/check verify` scope 004 berjalan. Jadi kotak berikutnya untuk spec ini adalah `/check verify`, yang needs kotak Verify it scope 004. Bagian jendela dari **AC-4** menunggu langkah manual `F004-1` dan `F004-2`.

## Consequences

**Positive**:

* Kotak scope 003 untuk macOS punya AC yang bisa diperiksa tanpa pekerjaan baru.
* Developer macOS punya satu tabel aturan dan satu tempat yang menunjuk apa yang terjadi saat kontrak berubah.

**Negative / tradeoffs**:

* Lima vektor (V-3, V-4, V-5, V-8, V-9) tidak diuji lewat binding. Kesalahan yang hanya muncul untuk isi byte tertentu di batas FFI tidak akan tertangkap di sana.
* Bukti tersebar di tiga berkas. Bila teks di `verify.md` yang dirujuk berubah, perintah `grep` di **AC-7** perlu disesuaikan.
* Verifikasi penuh menunggu kotak Verify it scope 004, dan bagian jendela menunggu user.
* Hanya macOS 27.0.1 pada `arm64` yang punya bukti.

**Neutral**:

* Status spec ini mengikuti kotak scope 003 untuk macOS. Tanpa langkah build, tidak ada yang menaikkannya ke `In Progress`. Lead menetapkannya sesudah `/check verify`.

## Follow-up

* [ ] Scope 006 dan 007 (menunggu host Windows dan Linux): spec penerapan scope 003 untuk Windows dan Linux menulis jalur panggilan langsung lewat Cargo dan bukti host masing masing.
* [ ] Scope 002: jadikan "binding sama dengan keluaran generator" pemeriksaan tetap pada setiap perubahan `crates/core/` atau `crates/macos-ffi/`.
* [ ] Bila vektor kontrak dipindah ke berkas fixture bersama (Follow-up spec bersama), uji Swift di spec 004 membacanya dari sana dan tabel **Vektor di Mac** diperbarui.
* [ ] Bila sebuah kesalahan nyata ditemukan pada vektor yang tidak diuji lewat binding, tambahkan vektor itu ke `BindingContractTests` lewat pembaruan spec 004.
