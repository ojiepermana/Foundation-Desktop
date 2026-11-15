# 0002. Kontrak sapaan bersama

**Scope**: [003 - Kontrak sapaan bersama](../../../scope/003%20-%20Kontrak%20sapaan%20bersama.md)
**Date**: 2026-11-15
**Status**: Proposed

## Summary

Spec ini menetapkan satu fungsi publik di Rust Core, `hello`, yang menerima nama dan mengembalikan sapaan `Hello, {name}!`. Fungsi itu menerima teks apa pun dan memakainya apa adanya: tidak ada pemangkasan, penolakan, atau kesalahan, supaya ketiga aplikasi native tidak punya ruang untuk berperilaku berbeda. Spec ini juga menulis aturan untuk semua API publik Core berikutnya, dan uji yang membuktikan semuanya tanpa membuka jendela. Untuk membangunnya kamu cukup menambah satu modul kecil, satu berkas uji, dan beberapa baris di crate Core yang kerangkanya dibuat spec 0001.

## Requirements

**User stories**:

* Sebagai developer aplikasi macOS, Windows, atau Linux, saya ingin satu fungsi sapaan dengan perilaku yang tertulis untuk setiap masukan, supaya UI saya cukup mengirim nama dan menampilkan hasilnya.
* Sebagai developer Core, saya ingin aturan yang jelas tentang apa yang publik dan bagaimana mengubahnya, supaya perubahan saya tidak memutus tiga aplikasi tanpa disadari.
* Sebagai pemeriksa, saya ingin membuktikan perilaku sapaan dengan `cargo test` saja, tanpa Xcode, Windows App SDK, atau GTK.

**Acceptance criteria**:

Angka kriteria merujuk ke tabel di [Penerimaan dan bukti fondasi](../../../quality/README.md). Semua perintah dijalankan dari akar repo.

* **AC-1** (Done when scope 003, kriteria 3): `desktop_core::hello("Ayu".to_string())` mengembalikan tepat `Hello, Ayu!`. Buktinya uji `ayu_returns_hello_ayu` di `crates/core/tests/hello_contract.rs`, yang hanya memakai path publik crate. `cargo test -p desktop-core --locked --test hello_contract` lulus.
* **AC-2** (kriteria 2): Core dibangun, diperiksa, dan diuji sendiri, tanpa pustaka UI. Keempat perintah ini lulus pada mesin yang hanya punya toolchain Rust dari `rust-toolchain.toml` (spec `_root/0001` memasang komponen `rustfmt` dan `clippy` lewat berkas itu): `cargo build -p desktop-core --locked`, `cargo test -p desktop-core --locked`, `cargo fmt -p desktop-core --check`, dan `cargo clippy -p desktop-core --all-targets --locked -- -D warnings`. Perintah `cargo tree -p desktop-core -e normal,build,dev --prefix none --locked` mencetak tepat satu baris, dan baris itu diawali `desktop-core v`. Artinya Core tidak punya dependensi apa pun, termasuk untuk uji.
* **AC-3** (kriteria 3): teks sapaan dibentuk di satu tempat saja. `git grep --untracked -lF 'Hello, {' -- '*.rs'` mencetak tepat satu baris: `crates/core/src/greeting.rs` (`--untracked` membuat berkas yang belum masuk commit ikut dicari). Uji membandingkan hasil dengan nilai literal atau dengan pemeriksaan awalan dan akhiran. Uji tidak menghitung ulang harapan dengan pola format yang sama. Pencarian ini menangkap pola `format!`. Bentuk lain, misalnya menyambung string dengan `+`, dijaga lewat tinjauan kode sampai scope 002 menjadikannya pemeriksaan tetap.
* **AC-4** (kriteria 22): permukaan publik Core persis seperti bagian **API surface**. `grep -E '^(pub|mod) ' crates/core/src/lib.rs` mencetak tepat dua baris: `mod greeting;` dan `pub use greeting::hello;`. Jadi modul tidak publik dan hanya `hello` yang diekspor. Uji `signature_is_pinned` mengikat `hello` ke tipe `fn(String) -> String` saat kompilasi, sehingga perubahan tanda tangan membuat uji gagal dibangun.
* **AC-5** (kriteria 3): untuk setiap vektor V-1 sampai V-11 di bagian **Vektor kontrak**, `hello` mengembalikan tepat hasil yang tertulis. Tidak ada masukan yang ditolak, dipangkas, dinormalisasi, atau dipotong. Tipe hasilnya `String`, bukan `Result`, jadi tidak ada kesalahan yang bisa dikembalikan, dan tidak ada vektor yang membuat `hello` panik. Buktinya uji `vectors_match_contract` dan `long_name_is_not_cut` di `hello_contract.rs`, serta uji unit di `greeting.rs`.
* **AC-6** (kriteria 2): `hello` tidak punya efek samping. Buktinya pemeriksaan kode, bukan uji: bagian `crates/core/src/greeting.rs` di atas modul uji tidak memuat `use`, `static`, `extern`, atau `unsafe`, jadi fungsi itu hanya menyentuh argumennya. Perintah `sed '/#\[cfg(test)\]/,$d' crates/core/src/greeting.rs | grep -nE '^[[:space:]]*(pub )?(use|static|extern|unsafe)[[:space:]]'` tidak mencetak apa pun. `crates/core/Cargo.toml` memuat `unsafe_code = "forbid"` di `[lints.rust]`, jadi `unsafe` di crate ini gagal dibangun. Uji unit `same_input_gives_same_output` mencatat sifat itu sebagai contoh yang berjalan.
* **AC-7** (kriteria 22): aturan kontrak publik bersama tersedia dan diperiksa. Bagian **Aturan kontrak publik bersama** spec ini (R-1 sampai R-9) adalah sumbernya. Komentar dokumen crate di `crates/core/src/lib.rs` menyebut path spec ini dan merangkum aturannya. `crates/core/Cargo.toml` memuat `missing_docs = "warn"` di `[lints.rust]`, setiap item publik punya rustdoc dengan satu contoh yang berjalan, `cargo test -p desktop-core --locked --doc` menjalankan paling sedikit satu contoh dan lulus, dan `RUSTDOCFLAGS="-D warnings" cargo doc -p desktop-core --no-deps --locked` lulus.

## Decision

**Chosen option**: Option 1: Fungsi total, nama dipakai apa adanya

`hello(name: String) -> String` menerima setiap `String` dan selalu mengembalikan `Hello, ` lalu `name` persis seperti yang diterima lalu `!`, tanpa validasi dan tanpa kesalahan. Fungsi itu tinggal di modul privat `greeting` dan hanya terlihat lewat akar crate sebagai `desktop_core::hello`.

Nilai dari spec lain yang dipakai di sini, tanpa diputuskan ulang:

| Nilai | Isi | Pemilik |
| --- | --- | --- |
| Paket Cargo dan nama lib | `desktop-core`, dipakai sebagai `desktop_core` | Spec `_root/0001` |
| Path crate | `crates/core/` | Spec `_root/0001` |
| Edisi, toolchain, versi awal | Edisi `2024`, toolchain `1.99.1`, `version = "0.1.0"` yang diwarisi dari `[workspace.package]` | Spec `_root/0001` |
| Kerangka crate | `crates/core/Cargo.toml` dengan `[dependencies]` kosong, dan `crates/core/src/lib.rs` yang hanya berisi komentar dokumen crate | Spec `_root/0001`, Build plan langkah 1 sampai 4 |
| Bentuk fungsi dan format hasil | `pub fn hello(name: String) -> String`, hasil `Hello, {name}!` | [Kontrak Rust](../../../contracts/README.md) |

## Rationale

Alasan dan pilihan lain: lihat [rationale.md](rationale.md).

## Feature design

**Data model sketch**:

Tidak ada model data. Kontrak ini hanya memakai dua nilai `String` yang hidup selama satu panggilan. Tidak ada yang disimpan, dan tidak ada tipe baru.

**State transitions**:

Tidak ada. `hello` tidak menyimpan keadaan.

**API surface**:

| Path publik | Tanda tangan | Masukan | Hasil | Akses | Kesalahan |
| --- | --- | --- | --- | --- | --- |
| `desktop_core::hello` | `pub fn hello(name: String) -> String` | `name`: setiap `String` (wajib, boleh kosong) | `String` baru berisi `Hello, ` + `name` + `!` | Publik, tanpa syarat | Tidak ada. Tidak pernah gagal, tidak pernah panik. |

Itu seluruh permukaan publik `desktop-core`. Modul `greeting` privat, jadi `desktop_core::greeting::hello` tidak bisa dipakai dari luar.

Perilaku untuk masukan di tepi:

| Masukan | Perilaku |
| --- | --- |
| Kosong | Diterima. Hasilnya `Hello, !`. |
| Spasi, tab, baris baru | Dipertahankan persis, di tepi maupun di tengah. Tidak ada pemangkasan. |
| Unicode | Setiap `String` adalah UTF-8 yang sah, jadi semua diterima. Tidak ada normalisasi (bentuk terurai tetap terurai), tidak ada perubahan huruf besar kecil, dan tidak ada penanganan arah teks. |
| Teks yang mirip pola format (`%s`, `{0}`) | Diperlakukan sebagai teks biasa. Nama tidak pernah ditafsirkan. |
| Karakter NUL di tengah | Dipertahankan. Hasil tidak terpotong. |
| Sangat panjang | Tidak ada batas panjang. Panjang hasil selalu panjang `name` ditambah 8 byte. Satu satunya batas adalah memori proses. |

Sifat pemanggilan: sinkron, tanpa I/O, tanpa keadaan global, dan boleh dipanggil dari thread mana pun, juga bersamaan. Sifat ini mengikuti badan fungsi (lihat **AC-6**) dan tidak punya uji tersendiri. Waktu dan memorinya sebanding dengan panjang nama, jadi untuk nama yang diketik pengguna fungsi ini aman dipanggil dari thread UI. Tidak ada runtime asinkron.

Isi `crates/core/src/greeting.rs` (uji unit menyusul di bawahnya, lihat Build plan langkah 3):

```rust
//! The greeting shown by every app.
//!
//! Contract: `docs/specs/_root/0002-kontrak-sapaan-bersama/index.md`.

/// Returns the greeting for `name`: the text `Hello, `, then `name` exactly
/// as given, then `!`.
///
/// Every `String` is accepted. Nothing is trimmed, normalised, or cut, and
/// the function never fails or panics. An empty `name` gives `Hello, !`.
///
/// # Examples
///
/// ```
/// assert_eq!(desktop_core::hello("Ayu".to_string()), "Hello, Ayu!");
/// ```
#[must_use]
#[allow(
    clippy::needless_pass_by_value,
    reason = "the shared contract fixes an owned String (spec _root/0002, R-3)"
)]
pub fn hello(name: String) -> String {
    format!("Hello, {name}!")
}
```

Tambahan di `crates/core/src/lib.rs`, di bawah komentar dokumen crate yang sudah dibuat spec 0001 (baris yang sudah ada tidak diubah):

```rust
//!
//! # Public contract
//!
//! The rules for this crate's public API live in
//! `docs/specs/_root/0002-kontrak-sapaan-bersama/index.md` (R-1 to R-9).
//! In short:
//!
//! * Only items re-exported from this crate root are public. Modules stay private.
//! * A public item is added or changed only after a `_root` spec records it.
//! * Public functions take and return owned values. No lifetimes, no generic parameters.
//! * Apps and bridges pass input and show output unchanged. They add no behaviour.

mod greeting;

pub use greeting::hello;
```

Tambahan di `crates/core/Cargo.toml` (tabel lain tidak diubah, dan `Cargo.lock` tidak berubah):

```toml
[lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"
```

Kedua blok kode Rust di atas sudah dicoba pada workspace percobaan dengan toolchain `1.99.1`: `cargo fmt`, `cargo clippy ... -D warnings`, `cargo test`, dan `cargo doc` lulus. Atribut `#[allow(...)]` pada `hello` tidak berpengaruh pada lint bawaan. Pada percobaan yang sama, tanpa atribut itu `clippy::pedantic` menolak `name: String`. Atribut itu menjaga agar fungsi tetap lulus bila grup `pedantic` kelak dinyalakan (kebijakan lint seluruh workspace milik scope 002).

**Vektor kontrak**:

Tabel ini normatif. Kolom masukan dan hasil ditulis sebagai literal string Rust, supaya spasi dan karakter khusus tidak ambigu. Spec per OS boleh merujuk vektor lewat ID.

| ID | `name` | Hasil `hello(name)` | Yang dibuktikan |
| --- | --- | --- | --- |
| V-1 | `"Ayu"` | `"Hello, Ayu!"` | Contoh di baris Done when. |
| V-2 | `""` | `"Hello, !"` | Nama kosong diterima. |
| V-3 | `"   "` (tiga spasi) | `"Hello,    !"` (empat spasi sesudah koma) | Nama yang hanya spasi tidak dipangkas. |
| V-4 | `"  Ayu  "` | `"Hello,   Ayu  !"` | Spasi di tepi dipertahankan. |
| V-5 | `"Ayu\nBudi\t"` | `"Hello, Ayu\nBudi\t!"` | Baris baru dan tab dipertahankan. |
| V-6 | `"\u{141}ukasz \u{674E} \u{1F469}\u{1F3FD}\u{200D}\u{1F4BB}"` | `"Hello, \u{141}ukasz \u{674E} \u{1F469}\u{1F3FD}\u{200D}\u{1F4BB}!"` | Huruf Latin beraksen, aksara Han, dan emoji gabungan tetap utuh. |
| V-7 | `"Cafe\u{301}"` | `"Hello, Cafe\u{301}!"`, dan hasilnya tidak sama dengan `"Hello, Caf\u{e9}!"` | Tidak ada normalisasi Unicode. |
| V-8 | `"\u{62F}\u{646}\u{64A}\u{627}"` | `"Hello, \u{62F}\u{646}\u{64A}\u{627}!"` | Teks kanan ke kiri tidak diubah urutannya. |
| V-9 | `"%s {0} \\n"` | `"Hello, %s {0} \\n!"` | Nama tidak ditafsirkan sebagai pola format atau urutan escape. |
| V-10 | `"A\0B"` | `"Hello, A\0B!"` | NUL di tengah tidak memotong hasil. |
| V-11 | `"a".repeat(1_048_576)` | Panjang 1.048.584 byte, diawali `Hello, `, diakhiri `!`, dan bagian tengahnya sama dengan `name` | Tidak ada batas panjang dan tidak ada pemotongan. |

**Value sourcing**:

| Aksi | Nilai yang dihasilkan | Sumber |
| --- | --- | --- |
| `hello` | Awalan `Hello, ` (dengan satu spasi sesudah koma) | Tetapan di `greeting.rs`, ditetapkan oleh [Kontrak Rust](../../../contracts/README.md) |
| `hello` | Nama di tengah | Parameter `name`, tanpa perubahan |
| `hello` | Akhiran `!` | Tetapan di `greeting.rs`, ditetapkan oleh Kontrak Rust |
| Uji kontrak | Nilai harapan tiap vektor | Tabel **Vektor kontrak** di spec ini |
| Catatan versi | Versi kontrak | `version` paket `desktop-core`, ditetapkan spec `_root/0001` |

Bahasa hasil selalu Inggris dan tidak bergantung pada locale mesin. Lokalisasi teks antarmuka adalah milik scope 010 dan tidak menyentuh hasil `hello`.

**Aturan kontrak publik bersama**:

Aturan ini berlaku untuk `hello` dan untuk setiap API publik Core sesudahnya. Kata "jembatan" berarti crate penghubung macOS (`crates/macos-ffi/`).

* **R-1 Satu permukaan.** API publik Core hanya item yang diekspor ulang di akar crate `desktop_core`. Semua modul privat. `lib.rs` hanya berisi dokumen crate, deklarasi `mod`, dan `pub use`.
* **R-2 Spec dulu.** Siapa pun boleh mengusulkan API publik baru atau perubahan pada yang ada, tetapi hanya lewat spec di `docs/specs/_root/` yang diterima sebelum kodenya ditulis. Spec itu mencatat masukan, hasil, nilai opsional, kesalahan, kompatibilitas, dan versi. Perilaku bersama yang dibutuhkan satu UI tetap masuk ke Core lewat jalan ini.
* **R-3 Nilai yang dimiliki.** Fungsi publik menerima dan mengembalikan nilai yang dimiliki, tanpa lifetime dan tanpa parameter tipe generik pada fungsinya. Tipe pustaka standar yang dimiliki seperti `String`, `Option`, `Vec`, dan `Result` boleh dipakai. Alasannya satu bentuk untuk ketiga pemanggil: nilai yang melintasi jembatan macOS selalu disalin, jadi tanda tangan yang sama dipakai juga oleh pemanggil Rust langsung. Itu sebabnya `hello` menerima `String`, bukan `&str`.
* **R-4 Kesalahan bertipe.** Fungsi yang bisa gagal mengembalikan `Result<T, E>` dengan `E` berupa enum milik Core. Panik dan string bukan cara melaporkan kesalahan. `hello` tidak bisa gagal, jadi belum ada tipe kesalahan, dan tidak ada yang dibuat sebelum sebuah fungsi membutuhkannya.
* **R-5 Kompatibilitas dan versi.** Versi kontrak adalah `version` paket `desktop-core` dan mengikuti SemVer. Selama versi masih `0.x`: perubahan yang memutus menaikkan angka minor, penambahan menaikkan angka patch. Perubahan yang memutus adalah mengubah tanda tangan, mengubah hasil untuk masukan mana pun (termasuk tanda baca dan spasi), menghapus atau mengganti nama ekspor, dan menolak masukan yang sebelumnya diterima. Setiap perubahan dicatat di tabel **Riwayat kontrak**. Versi `0.1.0` adalah versi kontrak pertama: kerangka crate tanpa API dari spec 0001 belum dihitung sebagai kontrak, jadi menambahkan `hello` tidak menaikkan versi. Karena `version` diwarisi dari `[workspace.package]`, kenaikannya dilakukan pemilik manifest root dalam perubahan yang sama, dan versi semua anggota workspace ikut naik.
* **R-6 Aman dan tanpa efek samping tersembunyi.** Core hanya memakai safe Rust (`unsafe_code = "forbid"`), tanpa keadaan global yang bisa berubah. API publik tidak melakukan I/O, tidak mencatat log, dan tidak menyimpan masukan pemanggil, kecuali spec pemiliknya menyebut hal itu.
* **R-7 Terdokumentasi.** Setiap item publik punya rustdoc yang menjelaskan perilakunya dan memuat satu contoh yang berjalan sebagai doc test. `missing_docs` menjaga hal ini.
* **R-8 Pemanggil tidak mengubah.** Aplikasi dan jembatan mengirim masukan apa adanya dan menampilkan hasil apa adanya. Untuk sapaan: UI mengirim isi kolom nama saat tombol ditekan tanpa memangkas, menolak, atau menggantinya, juga saat kolom kosong, lalu menampilkan `String` hasil sebagai teks biasa. Tidak ada salinan logika sapaan di luar Core. Jembatan hanya meneruskan panggilan dengan nama dan tanda tangan yang sama.
* **R-9 Diuji tanpa UI.** Setiap fungsi publik punya uji unit di modulnya dan uji kontrak di `crates/core/tests/` yang hanya memakai path publik. Keduanya berjalan dengan `cargo test -p desktop-core --locked`.

**Riwayat kontrak**:

| Versi `desktop-core` | Perubahan | Spec |
| --- | --- | --- |
| `0.1.0` | `hello(name: String) -> String` ditambahkan. | 0002 |

**Key invariants**:

* Untuk setiap `name`, `hello(name)` sama dengan `Hello, ` + `name` + `!`, byte demi byte.
* Panjang hasil dalam byte sama dengan panjang `name` ditambah 8.
* `hello` tidak mengembalikan kesalahan dan tidak panik untuk masukan apa pun. Ini mengikuti badan fungsi yang satu baris. Uji membuktikannya untuk vektor di atas.
* Masukan yang sama selalu memberi hasil yang sama, di mesin, locale, dan thread mana pun.
* Pola format sapaan hanya ada di `crates/core/src/greeting.rs`.
* `desktop-core` tidak punya dependensi selama kontraknya hanya `hello`. Dependensi pertama, termasuk yang dibutuhkan tipe publik baru agar bisa melintasi jembatan macOS, diputuskan di spec yang menambah API itu (R-2).

**Security model**:

Tidak ada autentikasi atau peran. Siapa pun yang menautkan crate boleh memanggil `hello`. Nama adalah teks dari pengguna yang tidak dipercaya, dan bisa berupa data pribadi:

* Core tidak menafsirkan, mencatat log, atau menyimpan nama. Nama hanya hidup selama panggilan.
* Hasil bisa memuat karakter apa pun yang diketik pengguna. Pemanggil menampilkannya sebagai teks biasa, bukan sebagai markup, pola format, atau kunci lokalisasi. Cara melakukannya di tiap toolkit ditulis di spec OS (lihat Follow-up).
* Tidak ada data yang diatur regulasi, jadi tidak ada lingkup kepatuhan.

**Configuration required**:

Tidak ada. Tidak ada variabel lingkungan, kredensial, atau berkas konfigurasi baru.

**Critical test scenarios**:

Uji kontrak di `crates/core/tests/hello_contract.rs`. Berkas ini diawali komentar dokumen `//!`, karena `missing_docs` juga memeriksa crate uji. Berkas ini hanya memuat `use desktop_core::hello;` dari crate Core.

* Jalur utama: `ayu_returns_hello_ayu` memanggil `hello("Ayu".to_string())` dan membandingkannya dengan `"Hello, Ayu!"`, memeriksa **AC-1**.
* Tanda tangan: `signature_is_pinned` menulis `let pinned: fn(String) -> String = hello;` lalu memanggil `pinned`, memeriksa **AC-4**.
* Masukan tepi: `vectors_match_contract` menjalankan tabel V-1 sampai V-10 dan pemeriksaan tidak sama pada V-7, dengan ID vektor di pesan gagal, memeriksa **AC-5**.
* Nama sangat panjang: `long_name_is_not_cut` menjalankan V-11 dengan pemeriksaan panjang, awalan, akhiran, dan bagian tengah, memeriksa **AC-5**.

Uji unit di `mod tests` dalam `crates/core/src/greeting.rs`:

* `greets_by_name` (V-1), `accepts_empty_name` (V-2), dan `keeps_whitespace_as_given` (V-3 dan V-4), memeriksa **AC-5**.
* `same_input_gives_same_output`: dua panggilan dengan `"Ayu"` memberi hasil yang sama, mendukung **AC-6**.
* `adds_exactly_eight_bytes`: untuk `""`, `"Ayu"`, nama V-8, dan nama V-10, panjang hasil sama dengan panjang nama ditambah 8, memeriksa **AC-5**.

Contoh di rustdoc `hello` berjalan sebagai doc test, memeriksa **AC-7**.

Tidak ada skenario izin, karena tidak ada pembatasan akses.

## Build plan

Urutan mengikuti pendekatan Tracer Bullet dari [Panduan scope](../../../scope/000%20-%20Panduan%20scope.md): lebih dulu satu jalur tipis dari path publik sampai uji yang lulus, lalu ditebalkan. Prasyarat: kerangka crate Core dari spec `_root/0001` (Build plan langkah 1 sampai 4) sudah ada di `main`.

1. Buat `crates/core/src/greeting.rs` dengan isi di bagian **API surface**, lalu tambahkan `mod greeting;` dan `pub use greeting::hello;` di `crates/core/src/lib.rs`, satisfies **AC-1**, **AC-4**
2. Buat `crates/core/tests/hello_contract.rs` dengan komentar `//!`, uji `ayu_returns_hello_ayu`, dan uji `signature_is_pinned`. Jalankan `cargo test -p desktop-core --locked`, satisfies **AC-1**, **AC-2**, **AC-4**
3. Tambahkan `mod tests` di `greeting.rs` dengan lima uji unit dari **Critical test scenarios**, satisfies **AC-5**, **AC-6**
4. Lengkapi `hello_contract.rs` dengan tabel vektor V-1 sampai V-10 (`vectors_match_contract`) dan uji V-11 (`long_name_is_not_cut`), satisfies **AC-3**, **AC-5**
5. Tambahkan `[lints.rust]` di `crates/core/Cargo.toml` dan bagian `# Public contract` di dokumen crate `lib.rs`, satisfies **AC-6**, **AC-7**
6. Jalankan dan catat hasil semua perintah di **AC-2**, **AC-3**, dan **AC-7**, lalu periksa `lib.rs` dan `greeting.rs` terhadap **AC-4** dan **AC-6**, satisfies **AC-2**, **AC-3**, **AC-4**, **AC-6**, **AC-7**

Langkah 1 dan 2 membentuk milestone pertama: baris Done when scope 003 sudah terbukti di sana. Langkah 3 sampai 6 membentuk milestone kedua.

## Consequences

**Positive**:

* Perilaku sapaan tertulis untuk setiap masukan, jadi spec macOS, Windows, dan Linux tidak perlu memutuskan apa pun tentang nama.
* Core lulus uji dengan `cargo test` saja dan tanpa dependensi, sehingga bukti kriteria 2 dan 3 murah diulang di host mana pun.
* Tidak ada tipe kesalahan, jadi jembatan macOS untuk alur pertama cukup meneruskan dua string.
* Aturan R-1 sampai R-9 memberi tempat yang jelas untuk API publik berikutnya.

**Negative / tradeoffs**:

* `Hello, !` dan sapaan yang hanya berisi spasi adalah hasil yang sah. UI tidak boleh menyembunyikannya dengan aturan sendiri.
* Menambah validasi kelak adalah perubahan yang memutus kontrak: tanda tangan berubah, versi naik, dan tiga UI serta jembatan ikut berubah.
* Tanpa batas panjang, pemanggil yang mengirim nama sangat besar membayar memori dan waktunya sendiri. Core tidak melindungi dari itu.
* R-2 membuat penambahan API publik lebih lambat, karena butuh spec lebih dulu.
* Vektor kontrak ada di dua tempat (spec dan uji). Mengubah satu tanpa yang lain membuat keduanya tidak cocok, dan hanya tinjauan yang menangkapnya.

**Neutral**:

* `hello` menerima `String` yang dimiliki walau hanya membacanya. Itu disengaja (R-3), dan atribut `allow` di fungsi mencatat alasannya.
* Versi kontrak menumpang pada versi workspace. Kalau Core kelak butuh versi sendiri, manifestnya berubah lewat pemilik spec 0001.
* Lint Core ditulis di `crates/core/Cargo.toml`. Kalau scope 002 kelak membuat `[workspace.lints]`, dua lint itu perlu pindah ke sana, karena Cargo tidak menerima `lints.workspace = true` bersama `[lints.rust]` di paket yang sama.
* `tests/contracts/` dan `tests/fixtures/` di akar repo tetap rencana. Uji kontrak Core tinggal di `crates/core/tests/`.

## Follow-up

* [ ] macOS (scope 004, `docs/specs/macos/`): jembatan mengekspos `hello` dengan nama dan tanda tangan yang sama dan hanya meneruskan panggilan (R-8). Ekspor ulang dengan `pub use` tidak cukup untuk UniFFI, jadi jembatan menulis fungsi pembungkus sendiri yang badannya hanya `desktop_core::hello(name)`. Uji Swift membuktikan paling sedikit V-1, V-2, V-6, dan V-10 lewat binding. SwiftUI menampilkan hasil sebagai teks biasa, misalnya `Text(verbatim:)`, supaya hasil tidak dibaca sebagai kunci lokalisasi atau Markdown. `git grep -lF 'Hello, ' -- '*.swift'` hanya boleh mencetak berkas uji.
* [ ] Windows (scope 006) dan Linux (scope 007), menunggu host masing masing: aplikasi memanggil `desktop_core::hello` langsung dan menampilkan hasil sebagai teks biasa (di GTK, label tanpa markup Pango). `cargo test -p desktop-core --locked` dijalankan di host itu sebagai bukti target. Spec ini tidak menulis bukti untuk kedua OS itu.
* [ ] Scope 002: jadikan pemeriksaan **AC-3** dan aturan R-1 serta R-8 pemeriksaan arsitektur yang tetap, termasuk untuk kode Swift dan aplikasi Rust. Kebijakan lint seluruh workspace dan bahasa komentar kode juga diputuskan di sana, termasuk memindahkan `unsafe_code` dan `missing_docs` Core bila `[workspace.lints]` dibuat. Sampai saat itu rustdoc Core memakai bahasa Inggris seperti contoh di spec ini.
* [ ] Lead: tambahkan tautan balik ke spec ini di berkas scope 003, dan pertimbangkan tautan dari `docs/contracts/README.md` ke bagian **Aturan kontrak publik bersama**. Kedua berkas itu di luar ownership sesi penulis spec.
* [ ] Bila pemanggil non Rust perlu membaca vektor kontrak dari berkas, pindahkan tabel vektor ke fixture bersama di `tests/fixtures/` lewat pembaruan spec ini.
* [ ] Bila produk meminta aturan nama yang sah (misalnya menolak nama kosong) atau sapaan yang dilokalkan, itu perubahan kontrak: perbarui spec ini lebih dulu, sesuai R-2, R-4, dan R-5.
