# 0003. Tema bersama dan kontrol native

**Scope**: [008 - Tema bersama dan kontrol native](../../../scope/008%20-%20Tema%20bersama%20dan%20kontrol%20native.md)
**Date**: 2026-11-15
**Status**: Proposed

## Summary

Spec ini menetapkan satu sumber untuk token desain (nilai warna, tipografi, dan jarak yang diberi nama) di `design-system/tokens/`, dan satu alat Rust kecil bernama `desktop-codegen` yang memeriksa sumber itu lalu menulis ulang setiap file turunannya. Isinya sengaja sedikit: tiga warna semantik dengan nilai terang dan gelap, satu peran tipografi, dan dua jarak, yaitu yang benar benar dipakai jendela sapaan. Bentuk token yang diterima ditulis sekali sebagai tipe Rust, dan file skema di `design-system/schemas/` dihasilkan dari tipe itu, jadi editor dan alat tidak bisa berbeda pendapat. Warna yang tertulis masih nilai contoh bertanda sementara, karena warna merek hanya kamu yang tahu. Cara tiap OS memakai token ditulis di spec OS itu, dengan kewajiban yang didaftar di sini.

## Requirements

**User stories**:

* Sebagai developer aplikasi macOS, Windows, atau Linux, saya ingin satu tempat untuk mengubah warna, tipografi, dan jarak, supaya ketiga jendela sapaan tetap sama maknanya tanpa menyalin angka dengan tangan.
* Sebagai pemeriksa, saya ingin satu perintah yang berkata apakah sumber token sah dan apakah semua file turunan masih sama dengan hasil generator, tanpa membuka jendela.
* Sebagai pemilik merek, saya ingin mengganti nilai contoh dengan warna asli di satu file, lalu diberi tahu bila teks menjadi sulit dibaca.
* Sebagai penulis spec scope 009, 010, dan 011, saya ingin bentuk alat yang sudah pasti, supaya sumber bersama berikutnya cukup menambah satu modul.

**Acceptance criteria**:

Angka kriteria merujuk tabel di [Penerimaan dan bukti fondasi](../../../quality/README.md). Semua perintah dijalankan dari akar repo pada Mac koordinasi. Angka yang bertanda "pada akhir milestone 1" bertambah saat token atau keluaran baru masuk lewat pembaruan spec.

* **AC-1** (kriteria 15, satu sumber acuan): nilai token hanya ditulis tangan di `design-system/tokens/`. Pada akhir milestone 1, `git ls-files design-system` mencetak tepat enam path: tiga file token (`colors.json`, `spacing.json`, `typography.json` di `design-system/tokens/`) dan tiga file skema (`colors.schema.json`, `spacing.schema.json`, `typography.schema.json` di `design-system/schemas/`). Isi tiga file token sama persis dengan bagian **Isi file**. `git ls-files tooling` mencetak tepat tujuh path dari tabel **Tata letak**. Tidak ada `radius.json`, `motion.json`, atau folder `tooling/validation/`.
* **AC-2** (kriteria 15, arti token): `cargo run -p desktop-codegen --locked -- tokens check` keluar dengan kode 0. Pada akhir milestone 1 baris terakhirnya tepat `tokens: ok, 6 semantic tokens, 3 placeholder values, 3 outputs up to date`. Uji `resolves_light_and_dark` lulus: setiap token semantik terurai menjadi nilai di tabel **Nilai hasil**, terurut menurut nama.
* **AC-3** (kriteria 17, bagian bersama): setiap warna semantik punya nilai terang dan gelap. File warna tanpa `dark` pada satu token ditolak dengan pesan yang memuat ``missing field `dark` ``. Warna yang menyebut `on` punya kontras 4,5 banding 1 atau lebih terhadap warna itu, di tampilan terang dan di tampilan gelap. Nilai `#999999` untuk `neutral.950` ditolak dengan pesan yang memuat `semantic.text.primary: contrast on background is`.
* **AC-4** (tabel "Pemeriksaan yang direncanakan" baris Sumber bersama, validasi skema token): uji `rejects_broken_sources` lulus untuk 18 kasus di tabel **Kasus sumber rusak**. Sumber yang tidak sah membuat `tokens check` dan `tokens generate` keluar dengan kode 1, mencetak satu baris per masalah ke stderr berbentuk `<path dari akar>: <pesan>`, lalu `tokens: failed, <n> problems`, dan tidak menulis file apa pun. `cargo run -p desktop-codegen --locked -- tokens` dan `cargo run -p desktop-codegen --locked -- nope check` keluar dengan kode 2 dan mencetak dua baris usage ke stderr.
* **AC-5** (kriteria 16, bagian bersama): keluaran yang menyimpang tertangkap tanpa langkah tambahan. Uji `committed_outputs_match_the_source` memanggil `check` pada repo dan lulus, jadi `cargo test --workspace --locked` gagal begitu satu keluaran hilang atau disunting tangan. Uji `check_reports_stale_and_missing_outputs_and_generate_repairs_them` lulus. `cargo test -p desktop-codegen --locked --test tokens_contract` mencetak `test result: ok. 5 passed`.
* **AC-6** (Done when "token yang bisa dihasilkan ulang", kriteria 16): pada working tree yang bersih, `cargo run -p desktop-codegen --locked -- tokens generate` mencetak baris terakhir `tokens: ok, 6 semantic tokens, 3 placeholder values, 3 outputs, 0 written`, lalu `git diff --exit-code -- design-system` keluar dengan kode 0. Sesudah `rm design-system/schemas/*.schema.json`, perintah yang sama mencetak tiga baris `wrote design-system/schemas/...` dan baris terakhir yang berakhir `3 outputs, 3 written`, dan `git status --short` tidak mencetak apa pun. `git ls-files --eol design-system` mencetak enam baris yang semuanya memuat `i/lf`, `w/lf`, dan `attr/text eol=lf`. Uji `generation_is_deterministic` lulus.
* **AC-7** (kriteria 16, dan bagian bersama dari Done when "kontrol mempertahankan fokus, penskalaan, dan aksesibilitas native"): skema dihasilkan dari bentuk yang sama dengan validator, dan bentuk itu tertutup. Isi tiga file skema sama persis dengan bagian **Isi file**. `grep -c '"additionalProperties": false' design-system/schemas/colors.schema.json` mencetak `3`, untuk `spacing.schema.json` `3`, dan untuk `typography.schema.json` `2`. Kunci yang tidak dikenal, misalnya `focus`, ditolak dengan pesan yang memuat ``unknown field `focus` ``. Jadi token tidak bisa membawa tinggi kontrol, cincin fokus, radius, atau ukuran huruf tetap. `grep -c '"\$schema": "../schemas/' design-system/tokens/*.json` mencetak `1` untuk tiap file.
* **AC-8** (nilai dari user): nilai warna merek belum diketahui dan ditandai. `grep -c '"placeholder": true' design-system/tokens/colors.json` mencetak `3`, dan baris sukses `tokens check` menyebut `3 placeholder values`. Tanda itu tidak menggagalkan pemeriksaan. Uji `resolves_light_and_dark` memeriksa angka 3 itu.
* **AC-9** (kriteria 16 lewat aturan [spec `_root/0001`](../0001-batas-workspace-dan-pemilik/index.md)): workspace tetap hijau dan alat berdiri sendiri. `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, dan `cargo clippy --workspace --all-targets --locked -- -D warnings` keluar dengan kode 0. `cargo tree -p desktop-codegen --depth 1 --prefix none --locked` mencetak tepat tiga baris, yang diawali `desktop-codegen v0.1.0`, `serde v1.`, dan `serde_json v1.`. `cargo tree -p desktop-codegen --target all --all-features --prefix none --locked | grep -cE '^(desktop-core|desktop-macos-ffi|desktop-windows|desktop-linux|uniffi|gtk|gtk4|libadwaita|windows-reactor) '` mencetak `0`. `cargo tree --workspace --locked -i desktop-codegen --prefix none` mencetak tepat satu baris, jadi tidak ada anggota yang bergantung pada alat ini. `cargo tree -p desktop-core --target all --all-features --prefix none --locked` tetap mencetak tepat satu baris. Pada commit langkah 1, `git diff HEAD~1 -- Cargo.toml` hanya mengubah baris `members`, dan `grep -c '^\[\[package\]\]' Cargo.lock` naik tepat satu (dari 77 menjadi 78 bila tidak ada anggota lain yang masuk lebih dulu).
* **AC-10** (kriteria 32): fitur ini hanya membuat yang dibutuhkan jendela sapaan. Setiap commit fitur ini memakai subject yang memuat `spec _root/0003 step <n>`. `git log --name-only --format= --grep='spec _root/0003 step [12]' | sort -u` hanya mencetak path dari tabel **File yang boleh diubah tiap milestone** baris milestone 1. `git ls-files apps/macos bindings/swift crates/macos-ffi` tetap mencetak tepat 16 path (AC-12 spec `macos/0001`). Tidak ada path di bawah `scripts/`, `.github/`, `apps/`, `crates/`, atau `bindings/` di daftar itu.
* **AC-11** (kriteria 15, dokumen): `docs/design-system/README.md` dan `docs/architecture/README.md` memuat teks dari bagian **Perubahan dokumen**. `grep -c 'desktop-codegen' docs/design-system/README.md` mencetak 2 atau lebih, `grep -c 'tooling/codegen' docs/architecture/README.md` mencetak 2 atau lebih, dan tautan ke `docs/specs/_root/0003-tema-bersama-dan-kontrol-native/index.md` di kedua dokumen mengarah ke file yang ada. `cargo test -p desktop-core --locked` tetap lulus, jadi tabel aturan arah dependensi tidak ikut berubah.
* **AC-12** (kriteria 16 pada host lain; `BLOCKED`, `owner: user`): pada host Windows `x86_64-pc-windows-msvc` dan host Linux `x86_64-unknown-linux-gnu` dengan toolchain `1.99.1`, `cargo build -p desktop-codegen --locked`, `cargo test -p desktop-codegen --locked`, dan `cargo run -p desktop-codegen --locked -- tokens check` keluar dengan kode 0. Artinya file yang ditulis di Mac sama byte demi byte dengan yang dihasilkan di host itu. Hasil dicatat per host dengan OS, arsitektur, versi, perintah, dan keluaran.

Bagian Done when yang hanya bisa dibuktikan di satu OS (jendela memakai token, tampilan terang dan gelap di layar, fokus keyboard, penskalaan teks, dan pembaca layar) bukan AC di sini. Itu kewajiban spec OS, didaftar di bagian **Kewajiban spec OS**.

## Decision

**Chosen option**: Opsi 1: alat Rust di workspace, bentuk token sebagai tipe, skema dihasilkan.

Token ditulis sebagai tiga file JSON kecil di `design-system/tokens/`. Paket `desktop-codegen` di `tooling/codegen/` membaca file itu ke tipe Rust yang menolak apa pun di luar bentuknya, memeriksa aturan lintas nilai, lalu menulis file turunan yang dilacak git. Perintah `check` dan satu uji di `cargo test --workspace` membandingkan file turunan dengan hasil generator byte demi byte.

Keputusan turunannya:

| Hal | Keputusan |
| --- | --- |
| Lokasi dan paket alat | `tooling/codegen/`, paket `desktop-codegen`, lib `desktop_codegen` dan bin `desktop-codegen`. Generator dan validasi satu paket. `tooling/validation/` tidak dibuat. |
| Dependensi alat | `serde` `1` dengan fitur `derive` dan `serde_json` `1`. Keduanya sudah ada di `Cargo.lock` lewat UniFFI. Tanpa crate mesin JSON Schema. |
| Cara jalan | `cargo run -p desktop-codegen --locked -- <sumber> <check\|generate>`. Sumber pertama bernama `tokens`. |
| Kode keluar | `0` lulus, `1` sumber tidak sah atau keluaran menyimpang, `2` salah pakai. |
| Format sumber | JSON. Tiga file: `colors.json`, `typography.json`, `spacing.json`. Tiap file dibuka `"$schema"` dan `"schemaVersion": 1`. |
| Skema | JSON Schema draft-07 di `design-system/schemas/`, **keluaran generator**, dibangun dari konstanta yang sama dengan validator. Tidak disunting tangan. |
| Versi | `schemaVersion` bilangan bulat untuk bentuk. Versi isi adalah riwayat git. Keluaran tidak membawa cap versi atau waktu. |
| Token dasar dan alias | Token dasar menyimpan nilai. Token semantik hanya menunjuk token dasar di file yang sama, satu lompatan. Token dasar yang tidak ditunjuk ditolak. |
| Terang dan gelap | Himpunan tertutup `light` dan `dark`, keduanya wajib di tiap warna semantik. Hanya warna yang berbeda per tampilan. |
| Warna | `#RRGGBB`, heksadesimal huruf besar, sRGB, tanpa alfa. |
| Tipografi | Peran dari himpunan tertutup (`body`) dengan bobot (`regular` atau `semibold`). Tanpa ukuran dan tanpa nama huruf. Tiap OS memetakan peran ke gaya teks native yang ikut skala teks OS. |
| Jarak | Bilangan bulat 0 sampai 128 dalam satuan logis (point di macOS, effective pixel di Windows, logical pixel di Linux). Lebar minimum jendela bukan token. |
| Aksen | Warna semantik `accent` diterapkan lewat mekanisme aksen native tiap OS. |
| Kontras | Warna yang menyebut `on` wajib 4,5 banding 1 atau lebih di kedua tampilan. |
| Nilai sementara | `"placeholder": true` pada token warna dasar. `check` melaporkan jumlahnya dan tidak gagal. |
| Keluaran | Dilacak git, path tetap di kode alat, mematuhi aturan O-1 sampai O-7. |
| Akhir baris | `.gitattributes` di akar dengan `/design-system/**/*.json text eol=lf`. Polanya per jenis file, supaya file biner yang kelak masuk folder itu tidak ikut diubah. |
| Akar repo | Alat memakai `CARGO_MANIFEST_DIR/../..` saat dikompilasi, jadi perintah berjalan dari folder mana pun di repo. |
| Keluaran per OS | Tiap spec OS menambah satu modul emitter (penulis file untuk satu target) di paket yang sama. Spec ini hanya menulis skema. |

Nilai dari spec lain yang dipakai di sini, tanpa diputuskan ulang:

| Nilai | Isi | Pemilik |
| --- | --- | --- |
| Aturan anggota workspace | `members` satu per satu, nama `desktop-` ditambah peran, empat field diwarisi, tanpa `.cargo/config.toml`, `--locked` di setiap perintah bukti | Spec `_root/0001` |
| Lokasi `design-system/tokens/`, `design-system/schemas/`, `tooling/codegen/` | Dicadangkan untuk scope 008 dan untuk scope pertama yang butuh alat | Spec `_root/0001`, tabel Lokasi dan pemilik |
| Toolchain | Rust `1.99.1`, edisi `2024` | Spec `_root/0001` |
| Jarak `12` dan `20` | Jarak antar kontrol dan jarak tepi di `GreetingView` | Spec `macos/0001`, bagian Isi file |
| Pola generator terlacak | Jalankan generator lalu `git diff --exit-code` | Spec `macos/0001`, AC-1 |

## Rationale

Alasan, opsi yang ditimbang, daftar keputusan beserta pilihan keduanya, dan bukti percobaan ada di [rationale.md](rationale.md).

## Feature design

**Data model sketch**:

Tidak ada basis data. Modelnya tiga file JSON. Setiap objek tertutup: kunci di luar tabel ini ditolak.

`design-system/tokens/colors.json`:

| Kunci | Tipe | Wajib | Isi |
| --- | --- | --- | --- |
| `$schema` | teks | ya | Tepat `../schemas/colors.schema.json` |
| `schemaVersion` | bilangan bulat | ya | Tepat `1` |
| `base.<nama>.value` | teks | ya | `#RRGGBB` huruf besar |
| `base.<nama>.placeholder` | boolean | tidak, bawaan `false` | `true` selama nilainya masih contoh |
| `semantic.<nama>.description` | teks | ya | Peran warna di layar, 1 sampai 120 karakter dari `A-Z a-z 0-9`, spasi, koma, titik, kurung, dan tanda hubung |
| `semantic.<nama>.light`, `.dark` | teks | ya | Nama token di `base` |
| `semantic.<nama>.on` | teks | tidak | Nama warna semantik lain yang menjadi latar warna ini |

`design-system/tokens/typography.json`:

| Kunci | Tipe | Wajib | Isi |
| --- | --- | --- | --- |
| `$schema`, `schemaVersion` | seperti di atas | ya | `../schemas/typography.schema.json`, `1` |
| `roles.<peran>.description` | teks | ya | Kegunaan peran, aturan karakter yang sama |
| `roles.<peran>.weight` | teks | ya | `regular` atau `semibold` |

`<peran>` hanya boleh `body`. Peran baru ditambah lewat pembaruan spec ini, karena tiap emitter OS wajib memetakannya.

`design-system/tokens/spacing.json`:

| Kunci | Tipe | Wajib | Isi |
| --- | --- | --- | --- |
| `$schema`, `schemaVersion` | seperti di atas | ya | `../schemas/spacing.schema.json`, `1` |
| `base.<nama>.value` | bilangan bulat | ya | 0 sampai 128, satuan logis |
| `semantic.<nama>.description` | teks | ya | Tempat jarak dipakai, aturan karakter yang sama |
| `semantic.<nama>.ref` | teks | ya | Nama token di `base` |

Nama token (`<nama>`) cocok dengan `^[a-z][a-z0-9]*(\.[a-z0-9]+)*$`. Hubungannya: satu token semantik menunjuk satu token dasar (dua untuk warna, satu per tampilan), dan satu token dasar boleh ditunjuk banyak token semantik.

**Nilai hasil** (token semantik sesudah diurai, pada akhir milestone 1):

| Token | Terang | Gelap | Catatan |
| --- | --- | --- | --- |
| `accent` | `#3366CC` | `#3366CC` | sementara |
| `background` | `#FAFAFA` | `#111111` | sementara |
| `text.primary` | `#111111` | `#FAFAFA` | sementara, di atas `background`, kontras sekitar 18 banding 1 |
| `body` (tipografi) | bobot `regular` | sama | |
| `gap.control` | `12` | sama | |
| `inset.window` | `20` | sama | |

**State transitions**:

Tidak ada mesin keadaan. Satu token warna dasar punya satu tanda: `placeholder` bernilai `true` sampai kamu mengganti nilainya dan menghapus tanda itu.

**API surface**:

Baris perintah (bin `desktop-codegen`, dijalankan lewat `cargo run -p desktop-codegen --locked --`):

| Perintah | Masukan | Keluaran saat sukses (stdout) | Kode | Saat gagal (stderr) |
| --- | --- | --- | --- | --- |
| `tokens check` | Tiga file token dan file keluaran di disk | `tokens: ok, <ringkasan>, <n> outputs up to date` | `0` | Satu baris per masalah, lalu `tokens: failed, <n> problems`, kode `1` |
| `tokens generate` | Tiga file token | Satu baris `wrote <path>` per file yang ditulis, lalu `tokens: ok, <ringkasan>, <n> outputs, <k> written` | `0` | Sama, kode `1`, tidak ada yang ditulis |
| Argumen lain, kurang, atau lebih | | | `2` | `usage: desktop-codegen <source> <check\|generate>` lalu `sources: tokens` |

`<ringkasan>` untuk token adalah `<s> semantic tokens, <p> placeholder values`, dengan `<s>` jumlah warna semantik, peran, dan jarak semantik. `generate` hanya menulis file yang hilang atau isinya berbeda. Pustaka mengembalikan baris sukses atau daftar masalah; `main` yang mencetak baris `wrote`, baris sukses, tiap masalah, dan baris `<sumber>: failed, <n> problems`. Kata `problems` selalu jamak, juga untuk satu masalah.

Permukaan Rust. Tulis tanda tangan ini persis. Setiap item publik punya rustdoc berbahasa Inggris (`missing_docs`). `Output`, `Problem`, `Plan`, `Color`, `Role`, `Space`, dan `TokenSet` menurunkan `Debug`, `Clone`, `PartialEq`, dan `Eq`. `Rgb`, `Weight`, dan `RoleName` juga menurunkan `Copy`.

```rust
// tooling/codegen/src/lib.rs
pub mod json;
pub mod tokens;

pub struct Output { pub path: String, pub contents: String }   // path dari akar repo, pemisah `/`
pub struct Problem { pub path: String, pub message: String }   // Display: "{path}: {message}"
impl Problem { pub fn new(path: &str, message: impl Into<String>) -> Self }
pub struct Plan { pub outputs: Vec<Output>, pub summary: String }
pub struct Source { pub name: &'static str, pub plan: fn(&Path) -> Result<Plan, Vec<Problem>> }
pub const SOURCES: &[Source] = &[tokens::SOURCE];
pub fn repo_root() -> PathBuf;                       // Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
pub fn source(name: &str) -> Option<&'static Source>;
pub fn check(root: &Path, source: &Source) -> Result<String, Vec<Problem>>;
pub fn generate(root: &Path, source: &Source) -> Result<(Vec<String>, String), Vec<Problem>>;

// tooling/codegen/src/json.rs
pub struct UniqueMap<T>(pub BTreeMap<String, T>);   // Deserialize; kunci ganda adalah kesalahan
pub fn to_canonical_json(value: &serde_json::Value) -> String;

// tooling/codegen/src/tokens/mod.rs
pub const SOURCE: Source;                            // name: "tokens"
pub const SCHEMA_VERSION: u64 = 1;
pub const COLORS_PATH: &str = "design-system/tokens/colors.json";
pub const TYPOGRAPHY_PATH: &str = "design-system/tokens/typography.json";
pub const SPACING_PATH: &str = "design-system/tokens/spacing.json";
pub const MAX_SPACE: u64 = 128;
pub const MIN_CONTRAST: f64 = 4.5;
pub struct Rgb(pub u8, pub u8, pub u8);
pub struct Color { pub name: String, pub description: String, pub light: Rgb, pub dark: Rgb, pub placeholder: bool }
pub enum Weight { Regular, Semibold }
pub enum RoleName { Body }
impl RoleName { pub const ALL: [RoleName; 1]; pub fn as_str(self) -> &'static str }
pub struct Role { pub name: RoleName, pub description: String, pub weight: Weight }
pub struct Space { pub name: String, pub description: String, pub value: u32 }
pub struct TokenSet { pub colors: Vec<Color>, pub roles: Vec<Role>, pub spacing: Vec<Space>, pub placeholders: usize }
pub fn parse(colors: &str, typography: &str, spacing: &str) -> Result<TokenSet, Vec<Problem>>;
```

Perilaku tiap fungsi:

| Fungsi | Perilaku |
| --- | --- |
| `check` | Memanggil `(source.plan)(root)`. Untuk tiap keluaran, dalam urutan `Plan.outputs`, membaca file di `root` sebagai teks UTF-8 lalu membandingkan seluruh isinya. Nilai `Ok` adalah baris sukses lengkap yang dicetak `main`, misalnya `tokens: ok, 6 semantic tokens, 3 placeholder values, 3 outputs up to date`. File yang tidak ada atau tidak terbaca sebagai teks memberi masalah `<path>: missing, run: cargo run -p desktop-codegen --locked -- <sumber> generate`. Isi berbeda memberi `<path>: differs from the generator output, run: cargo run -p desktop-codegen --locked -- <sumber> generate`. Tidak pernah menulis. |
| `generate` | Memanggil `plan` lebih dulu. Bila `plan` gagal, tidak ada yang ditulis. Bila lulus, menulis file yang hilang atau berbeda (membuat foldernya bila perlu) dan mengembalikan daftar path yang ditulis beserta baris sukses lengkap. Kegagalan menulis satu file memberi `<path>: cannot write: <kesalahan OS>`; file lain tetap dicoba, lalu semua masalah dikembalikan. |
| `tokens::SOURCE.plan` | Membaca tiga file dari `root`. File yang tidak terbaca memberi `<path>: cannot read: <kesalahan OS>`; bila ada satu saja yang tidak terbaca, `plan` mengembalikan masalah baca itu tanpa memanggil `parse`. Bila semua terbaca, `parse`, lalu menyusun keluaran: tiga skema dengan urutan `colors`, `spacing`, `typography`. Emitter OS ditambahkan sesudah skema. |
| `parse` | Fungsi murni atas teks tiga file. Mengumpulkan semua masalah, bukan hanya yang pertama. Tiap file dibaca ke tipenya lebih dulu; bila itu gagal, file itu memberi tepat satu masalah (pesan `serde_json`) dan aturan T-2 sampai T-9 dilewati untuk file itu, sedangkan dua file lain tetap diperiksa. Semua aturan berlaku per file, tidak ada aturan lintas file. Masalah terurut per file: warna, tipografi, lalu jarak. Urutan di dalam satu file tidak dikunci. `colors`, `roles`, dan `spacing` di hasilnya terurut menurut nama dalam urutan byte. `Color.placeholder` bernilai `true` bila salah satu nilai dasarnya bertanda. `placeholders` adalah jumlah token warna dasar yang bertanda. |
| `to_canonical_json` | Menulis JSON dengan indentasi dua spasi, teks dan angka ditulis persis seperti `serde_json::to_string` menulisnya (termasuk escape), satu elemen per baris, kunci objek terurut dalam urutan byte, pemisah `": "`, larik dan objek kosong sebagai `[]` dan `{}`, dan satu newline di akhir. Untuk nilai yang kuncinya sudah terurut hasilnya sama dengan `serde_json::to_string_pretty` ditambah newline. Fungsi ini mengurutkan sendiri dan tidak bergantung pada urutan iterasi `serde_json::Map`. |

**Value sourcing**:

| Aksi | Nilai yang dihasilkan atau ditampilkan | Sumber |
| --- | --- | --- |
| `tokens check`, baris sukses | Jumlah token semantik | Jumlah entri `semantic` di `colors.json` dan `spacing.json` ditambah entri `roles` |
| `tokens check`, baris sukses | Jumlah nilai sementara | Jumlah entri `base` di `colors.json` dengan `"placeholder": true` |
| `tokens check`, baris sukses | Jumlah keluaran | Panjang `Plan.outputs` |
| `parse` | Nilai terang dan gelap tiap warna semantik | `base.<light>.value` dan `base.<dark>.value` di `colors.json` |
| `parse` | Nilai jarak semantik | `base.<ref>.value` di `spacing.json` |
| `parse` | Ambang kontras | Konstanta `MIN_CONTRAST = 4.5` (ambang teks biasa di WCAG tingkat AA) |
| `parse` | Rasio kontras | Dihitung dari dua nilai `Rgb` dengan rumus di **Aturan sumber** T-8. Hanya masuk pesan kesalahan, tidak pernah masuk file. |
| Keluaran skema | Pola nama, pola warna, pola deskripsi, versi, daftar peran, batas jarak | Konstanta di `tokens/mod.rs` yang juga dipakai validator |
| Keluaran skema | Teks penanda | Konstanta di `tokens/schema.rs`, tertulis di **Aturan keluaran** O-1 |
| Semua perintah | Akar repo | `CARGO_MANIFEST_DIR/../..` saat kompilasi |
| File token | Tiga nilai warna dasar | Nilai contoh di spec ini, bertanda sementara. Nilai asli dari user (lihat Follow-up). |
| File token | Jarak `12` dan `20` | `GreetingView.swift` di spec `macos/0001` |
| File token | Bobot `regular` untuk `body` | Tampilan label hasil saat ini di spec `macos/0001` (huruf sistem bawaan) |

**Aturan sumber** (yang diperiksa `parse`; pesan ditulis persis, `<...>` diganti nilainya):

| ID | Aturan | Pesan |
| --- | --- | --- |
| T-1 | Bentuk tertutup: kunci tidak dikenal, kunci wajib hilang, tipe salah, atau kunci ditulis dua kali ditolak | Pesan `serde_json` apa adanya, misalnya ``unknown field `focus`, expected one of ... at line 3 column 29``, ``missing field `dark` ``, ``duplicate field `light` ``. Untuk nama token yang ditulis dua kali: ``duplicate key `<nama>` `` |
| T-2 | `$schema` dan `schemaVersion` tepat | `$schema: expected ../schemas/<file>.schema.json, found <nilai>` dan `schemaVersion: expected 1, found <angka>` |
| T-3 | Nama token cocok dengan pola, dan dua nama dalam satu kelompok tidak boleh sama sesudah titiknya dibuang | `<kelompok>.<nama>: name must match ^[a-z][a-z0-9]*(\.[a-z0-9]+)*$` dan `<kelompok>: names <a> and <b> collide once the dots are removed` |
| T-4 | Deskripsi cocok dengan `^[A-Za-z0-9 ,.()-]{1,120}$` | `<tempat>.description: must match ^[A-Za-z0-9 ,.()-]{1,120}$` |
| T-5 | Nilai warna dasar `#RRGGBB` huruf besar; nilai jarak dasar 0 sampai 128 | `base.<nama>.value: expected #RRGGBB with uppercase hex digits, found <nilai>` dan `base.<nama>.value: expected a whole number from 0 to 128, found <angka>` |
| T-6 | Token semantik menunjuk token dasar yang ada di file yang sama. Menunjuk token semantik lain tidak boleh. | `semantic.<nama>.<light\|dark\|ref>: unknown base token <rujukan>` |
| T-7 | Setiap token dasar ditunjuk paling sedikit satu token semantik | `base.<nama>: not used by any semantic token` |
| T-8 | `on` menunjuk warna semantik lain, dan kontrasnya 4,5 atau lebih di kedua tampilan | `semantic.<nama>.on: unknown semantic color <nilai>`, `semantic.<nama>.on: a color cannot sit on itself`, dan `semantic.<nama>: contrast on <on> is <rasio dua desimal> in <light\|dark>, needs 4.50 or more` |
| T-9 | `semantic` dan `roles` tidak kosong; peran dari himpunan tertutup | `semantic: needs at least one token`, `roles: needs at least one role`, dan `roles.<nama>: unknown role, expected one of: body` |

`<kelompok>` adalah `base` atau `semantic`. Untuk tabrakan nama, nama diproses dalam urutan byte, dan tiap nama yang bertabrakan dilaporkan sekali terhadap nama yang lebih dulu, jadi `<a>` selalu lebih kecil dari `<b>` dalam urutan byte. Setiap pesan diawali path file, misalnya `design-system/tokens/colors.json: base.neutral.50.value: ...`. Kelompok nama diperiksa terpisah: `base` dan `semantic` di tiap file.

Rumus kontras T-8: untuk tiap kanal `v` (0 sampai 255), `c = v / 255`, lalu `c / 12.92` bila `c <= 0.04045`, selain itu `((c + 0.055) / 1.055)` pangkat `2.4`. Luminans `L = 0.2126 R + 0.7152 G + 0.0722 B`. Untuk dua warna dengan luminans `L1` dan `L2`, rasio adalah `(max(L1, L2) + 0.05) / (min(L1, L2) + 0.05)`. Rasio dihitung sekali untuk pasangan nilai tampilan terang dan sekali untuk pasangan nilai tampilan gelap. Yang dibandingkan dengan 4,5 adalah nilai mentah. Di pesan, rasio dipotong ke bawah menjadi dua desimal (bukan dibulatkan), supaya nilai 4,496 tercetak `4.49`.

**Aturan keluaran** (berlaku untuk setiap file yang ditulis alat, termasuk emitter OS dan sumber scope 009 sampai 011):

| ID | Aturan |
| --- | --- |
| O-1 | Penanda. Tiga baris pertama file memuat teks `Generated by desktop-codegen` dan `Do not edit`, dalam bentuk komentar target itu. Di JSON penanda adalah kunci `$comment`. Untuk skema token teksnya: `Generated by desktop-codegen from tooling/codegen/src/tokens. Do not edit. To regenerate, run: cargo run -p desktop-codegen --locked -- tokens generate`. |
| O-2 | Isi hanya bergantung pada sumber dan kode alat. Tanpa jam, tanggal, variabel lingkungan, nama host, nama pengguna, atau path absolut. |
| O-3 | UTF-8 tanpa BOM, akhir baris `\n`, tepat satu newline di akhir. |
| O-4 | Urutan tetap. Token terurut menurut nama dalam urutan byte, kunci objek terurut. Keluaran tidak pernah disusun dengan mengiterasi `HashMap` atau `serde_json::Map` secara langsung, karena fitur `preserve_order` milik crate lain di workspace bisa mengubah urutannya. |
| O-5 | Angka. Hanya bilangan bulat dan teks yang disalin dari sumber, misalnya kanal warna sebagai bilangan 0 sampai 255 atau teks heksadesimal. Tanpa pecahan hasil hitung, karena pustaka matematika tiap OS bisa berbeda di digit terakhir. |
| O-6 | Lokasi. Path keluaran tetap di kode alat, bukan argumen perintah. Keluaran bersama tinggal di folder sumbernya (`design-system/`), keluaran satu OS tinggal di folder aplikasi OS itu. Semua dilacak git, dan jenis file sumber serta keluarannya punya baris `text eol=lf` di `.gitattributes`, ditulis per jenis file (misalnya `/design-system/**/*.json`), bukan untuk seluruh folder. |
| O-7 | Satu pintu. Semua keluaran satu sumber masuk `Plan.outputs`, jadi satu `generate` menulis semuanya dan satu `check` membandingkan semuanya. |

**Kewajiban spec OS** (spec macOS, Windows, dan Linux untuk scope 008 mengutip ID ini dan menulis buktinya sendiri):

| ID | Kewajiban |
| --- | --- |
| K-1 | Emitter. Tambah satu modul `tooling/codegen/src/tokens/<target>.rs` yang mengubah `&TokenSet` menjadi satu `Output` atau lebih, dan panggil dari `tokens::SOURCE.plan` sesudah skema. Keluaran mematuhi O-1 sampai O-7 dan tinggal di folder sistem desain aplikasi itu. Emitter adalah teks murni, jadi terbangun dan berjalan di host mana pun. |
| K-2 | Pemetaan lengkap. Emitter memetakan setiap warna semantik, setiap `RoleName`, setiap `Weight`, dan setiap jarak. Pakai `match` tanpa cabang bawaan atas `RoleName` dan `Weight`, supaya peran atau bobot baru gagal dikompilasi sampai dipetakan. Nama hasil pemetaan yang sama dengan kata kunci bahasa target ditulis dengan cara aman bahasa itu (backtick di Swift, awalan `r#` di Rust). |
| K-3 | Pemakaian. Kode tampilan jendela sapaan mengambil warna, peran tipografi, dan jarak hanya dari definisi hasil generator. Spec OS menulis pemeriksaan teks yang membuktikan tidak ada warna, bobot, atau jarak literal di kode tampilan. Ukuran jendela, misalnya lebar minimum `320` di macOS, adalah konstanta milik OS itu dan bukan token. |
| K-4 | Tidak ada token menganggur. Setiap token semantik dipakai jendela sapaan di OS itu, atau spec OS menulis alasannya. Token yang tidak dipakai OS mana pun dihapus dari sumber lewat pembaruan spec ini. |
| K-5 | Terang dan gelap. Aplikasi mengikuti pengaturan tampilan OS. Tidak ada tombol tema di dalam aplikasi. Kedua tampilan diperiksa di jendela nyata oleh user, dengan nilai yang diharapkan dibaca dari file hasil generator. |
| K-6 | Kontrol native. Kolom teks, tombol, dan label tetap kontrol asli OS. `accent` diterapkan lewat mekanisme aksen atau tint native. Peran tipografi dipetakan ke gaya teks native yang ikut skala teks OS, bukan ukuran tetap. Token tidak dipakai untuk mengganti tinggi kontrol, cincin fokus, bentuk, atau area sentuh. Bila OS punya mode kontras tinggi yang mengganti palet (misalnya tema kontras di Windows), aplikasi mengikuti palet sistem dan tidak menerapkan token warna selama mode itu aktif. |
| K-7 | Pemeriksaan manual (`owner: user`). Di jendela nyata: urutan dan cincin fokus keyboard, skala teks OS diperbesar, pembaca layar membacakan kolom, tombol, dan hasil, dan mode kontras tinggi OS bila ada. Hasil dicatat per host. |
| K-8 | Bukti keluaran. AC spec OS memuat `cargo run -p desktop-codegen --locked -- tokens check` dan `git diff --exit-code -- <folder keluaran>` sesudah `generate`. |
| K-9 | Nilai sementara. Keluaran menandai token yang nilainya masih sementara dengan komentar, memakai `Color.placeholder`. |

**Key invariants**:

* Nilai token hanya ditulis tangan di `design-system/tokens/`. File lain yang memuat nilai token adalah keluaran generator dan sama byte demi byte dengan hasil `generate`.
* Alat hanya menerima bentuk yang bisa ia baca ke tipe Rust. Tidak ada jalur di mana sumber lolos pemeriksaan tetapi generator tidak mengerti isinya.
* Skema dan validator memakai konstanta yang sama. Mengubah bentuk berarti mengubah konstanta atau tipe, lalu `generate`.
* `generate` pada sumber yang tidak sah tidak menulis apa pun.
* `desktop-codegen` tidak bergantung pada anggota workspace mana pun, dan tidak ada anggota yang bergantung padanya. Aplikasi memakai file hasil generator, bukan crate ini.
* Perubahan yang membuat file token lama tidak lagi sah menaikkan `SCHEMA_VERSION` dan `schemaVersion` di ketiga file dalam commit yang sama.

**Security model**:

Token adalah nilai publik. Spec ini tidak menambah rahasia, kredensial, variabel lingkungan, atau akses jaringan. Alat membaca tiga path tetap dan menulis path tetap di bawah akar repo, tanpa argumen path. Nama dan deskripsi token dibatasi ke karakter aman (T-3 dan T-4), jadi teks dari sumber tidak bisa keluar dari komentar atau nama pengenal di file hasil generator. `unsafe_code = "forbid"` berlaku untuk paket ini.

**Configuration required**:

Tidak ada.

**Tata letak** (tujuh path di `tooling/`, enam di `design-system/`, satu di akar):

| Path | Isi | Sumber isi |
| --- | --- | --- |
| `tooling/codegen/Cargo.toml` | Manifest paket | Bagian **Isi file** |
| `tooling/codegen/src/lib.rs` | `Output`, `Problem`, `Plan`, `Source`, `SOURCES`, `repo_root`, `source`, `check`, `generate` | Bagian **API surface** |
| `tooling/codegen/src/main.rs` | Baris perintah | Bagian **API surface** |
| `tooling/codegen/src/json.rs` | `UniqueMap` dan `to_canonical_json` | Bagian **API surface** |
| `tooling/codegen/src/tokens/mod.rs` | Tipe sumber, `parse`, aturan T-1 sampai T-9, `SOURCE` | Bagian **API surface** dan **Aturan sumber** |
| `tooling/codegen/src/tokens/schema.rs` | Penulis tiga skema (modul privat) | Bagian **Isi file** |
| `tooling/codegen/tests/tokens_contract.rs` | Lima uji kontrak | Bagian **Uji kontrak** |
| `design-system/tokens/colors.json`, `typography.json`, `spacing.json` | Sumber token | Bagian **Isi file** |
| `design-system/schemas/colors.schema.json`, `typography.schema.json`, `spacing.schema.json` | Keluaran generator. Jangan disunting. | Generator; isi yang diharapkan di **Isi file** |
| `.gitattributes` | Aturan akhir baris | Bagian **Isi file** |

Selain itu fitur ini mengubah `Cargo.toml` akar dan `Cargo.lock`.

**File yang boleh diubah tiap milestone**:

| Milestone | File |
| --- | --- |
| 1 | `Cargo.toml` akar (hanya baris `members`), `Cargo.lock`, `.gitattributes`, tujuh path `tooling/codegen/`, enam path `design-system/`, `verify.md` spec ini, baris `**Status**` di `index.md` spec ini, dan empat file `graphify-out/` dari hook |
| 2 | `docs/design-system/README.md`, `docs/architecture/README.md`, `verify.md` spec ini, dan empat file `graphify-out/` dari hook |
| 3 | `verify.md` spec ini dan empat file `graphify-out/` dari hook |

**Isi file**:

Tulis file berikut persis seperti ini. Komentar kode dan deskripsi token berbahasa Inggris, mengikuti spec `_root/0002`, sampai scope 002 memutuskan kebijakan bahasa komentar.

`Cargo.toml` di akar sesudah perubahan (hanya baris `members` yang berubah):

```toml
[workspace]
resolver = "3"
members = ["crates/core", "crates/macos-ffi", "tooling/codegen"]

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.99"
publish = false

[workspace.dependencies]
desktop-core = { path = "crates/core" }
```

`Cargo.lock` diperbarui dengan menjalankan `cargo build -p desktop-codegen` sekali tanpa `--locked`, lalu ikut di commit yang sama. Jangan disunting dengan tangan dan jangan memakai `cargo generate-lockfile`, karena perintah itu menaikkan versi paket lain. Hasilnya tepat satu blok baru, `desktop-codegen` dengan dependensi `serde` dan `serde_json`.

`tooling/codegen/Cargo.toml`:

```toml
[package]
name = "desktop-codegen"
description = "Validates the shared sources of the desktop foundation and regenerates the files derived from them."
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish.workspace = true

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"

[lints.rust]
unsafe_code = "forbid"
missing_docs = "warn"
```

`serde` dan `serde_json` dipakai satu anggota saja, jadi keduanya ditulis di sini dan bukan di `[workspace.dependencies]` (aturan spec `_root/0001`). Versi yang terkunci adalah `serde 1.0.229` dan `serde_json 1.0.154`.

`.gitattributes` di akar, tepat satu baris:

```gitattributes
/design-system/**/*.json text eol=lf
```

`design-system/tokens/colors.json`:

```json
{
  "$schema": "../schemas/colors.schema.json",
  "schemaVersion": 1,
  "base": {
    "brand.primary": { "value": "#3366CC", "placeholder": true },
    "neutral.50": { "value": "#FAFAFA", "placeholder": true },
    "neutral.950": { "value": "#111111", "placeholder": true }
  },
  "semantic": {
    "accent": {
      "description": "Fill of the primary action control.",
      "light": "brand.primary",
      "dark": "brand.primary"
    },
    "background": {
      "description": "Window background behind all content.",
      "light": "neutral.50",
      "dark": "neutral.950"
    },
    "text.primary": {
      "description": "Main text.",
      "light": "neutral.950",
      "dark": "neutral.50",
      "on": "background"
    }
  }
}
```

`design-system/tokens/typography.json`:

```json
{
  "$schema": "../schemas/typography.schema.json",
  "schemaVersion": 1,
  "roles": {
    "body": {
      "description": "Default reading text.",
      "weight": "regular"
    }
  }
}
```

`design-system/tokens/spacing.json`:

```json
{
  "$schema": "../schemas/spacing.schema.json",
  "schemaVersion": 1,
  "base": {
    "space.large": { "value": 20 },
    "space.medium": { "value": 12 }
  },
  "semantic": {
    "gap.control": {
      "description": "Gap between stacked controls.",
      "ref": "space.medium"
    },
    "inset.window": {
      "description": "Padding between the window edge and its content.",
      "ref": "space.large"
    }
  }
}
```

Tiga file skema berikut adalah hasil `tokens generate`. Kamu tidak mengetiknya. Cocokkan hasil generatormu dengan blok ini. `tokens/schema.rs` membangunnya dari konstanta `SCHEMA_VERSION`, `MAX_SPACE`, `RoleName::ALL`, dan tiga pola, lalu menulisnya dengan `to_canonical_json`. Draft yang dipakai adalah draft-07, karena itu yang paling luas didukung editor.

`design-system/schemas/colors.schema.json`:

```json
{
  "$comment": "Generated by desktop-codegen from tooling/codegen/src/tokens. Do not edit. To regenerate, run: cargo run -p desktop-codegen --locked -- tokens generate",
  "$schema": "http://json-schema.org/draft-07/schema#",
  "additionalProperties": false,
  "properties": {
    "$schema": {
      "const": "../schemas/colors.schema.json"
    },
    "base": {
      "additionalProperties": {
        "additionalProperties": false,
        "properties": {
          "placeholder": {
            "type": "boolean"
          },
          "value": {
            "pattern": "^#[0-9A-F]{6}$",
            "type": "string"
          }
        },
        "required": [
          "value"
        ],
        "type": "object"
      },
      "propertyNames": {
        "pattern": "^[a-z][a-z0-9]*(\\.[a-z0-9]+)*$"
      },
      "type": "object"
    },
    "schemaVersion": {
      "const": 1
    },
    "semantic": {
      "additionalProperties": {
        "additionalProperties": false,
        "properties": {
          "dark": {
            "pattern": "^[a-z][a-z0-9]*(\\.[a-z0-9]+)*$",
            "type": "string"
          },
          "description": {
            "pattern": "^[A-Za-z0-9 ,.()-]{1,120}$",
            "type": "string"
          },
          "light": {
            "pattern": "^[a-z][a-z0-9]*(\\.[a-z0-9]+)*$",
            "type": "string"
          },
          "on": {
            "pattern": "^[a-z][a-z0-9]*(\\.[a-z0-9]+)*$",
            "type": "string"
          }
        },
        "required": [
          "description",
          "light",
          "dark"
        ],
        "type": "object"
      },
      "minProperties": 1,
      "propertyNames": {
        "pattern": "^[a-z][a-z0-9]*(\\.[a-z0-9]+)*$"
      },
      "type": "object"
    }
  },
  "required": [
    "$schema",
    "schemaVersion",
    "base",
    "semantic"
  ],
  "title": "Color tokens",
  "type": "object"
}
```

`design-system/schemas/typography.schema.json`:

```json
{
  "$comment": "Generated by desktop-codegen from tooling/codegen/src/tokens. Do not edit. To regenerate, run: cargo run -p desktop-codegen --locked -- tokens generate",
  "$schema": "http://json-schema.org/draft-07/schema#",
  "additionalProperties": false,
  "properties": {
    "$schema": {
      "const": "../schemas/typography.schema.json"
    },
    "roles": {
      "additionalProperties": {
        "additionalProperties": false,
        "properties": {
          "description": {
            "pattern": "^[A-Za-z0-9 ,.()-]{1,120}$",
            "type": "string"
          },
          "weight": {
            "enum": [
              "regular",
              "semibold"
            ]
          }
        },
        "required": [
          "description",
          "weight"
        ],
        "type": "object"
      },
      "minProperties": 1,
      "propertyNames": {
        "enum": [
          "body"
        ]
      },
      "type": "object"
    },
    "schemaVersion": {
      "const": 1
    }
  },
  "required": [
    "$schema",
    "schemaVersion",
    "roles"
  ],
  "title": "Typography tokens",
  "type": "object"
}
```

`design-system/schemas/spacing.schema.json`:

```json
{
  "$comment": "Generated by desktop-codegen from tooling/codegen/src/tokens. Do not edit. To regenerate, run: cargo run -p desktop-codegen --locked -- tokens generate",
  "$schema": "http://json-schema.org/draft-07/schema#",
  "additionalProperties": false,
  "properties": {
    "$schema": {
      "const": "../schemas/spacing.schema.json"
    },
    "base": {
      "additionalProperties": {
        "additionalProperties": false,
        "properties": {
          "value": {
            "maximum": 128,
            "minimum": 0,
            "type": "integer"
          }
        },
        "required": [
          "value"
        ],
        "type": "object"
      },
      "propertyNames": {
        "pattern": "^[a-z][a-z0-9]*(\\.[a-z0-9]+)*$"
      },
      "type": "object"
    },
    "schemaVersion": {
      "const": 1
    },
    "semantic": {
      "additionalProperties": {
        "additionalProperties": false,
        "properties": {
          "description": {
            "pattern": "^[A-Za-z0-9 ,.()-]{1,120}$",
            "type": "string"
          },
          "ref": {
            "pattern": "^[a-z][a-z0-9]*(\\.[a-z0-9]+)*$",
            "type": "string"
          }
        },
        "required": [
          "description",
          "ref"
        ],
        "type": "object"
      },
      "minProperties": 1,
      "propertyNames": {
        "pattern": "^[a-z][a-z0-9]*(\\.[a-z0-9]+)*$"
      },
      "type": "object"
    }
  },
  "required": [
    "$schema",
    "schemaVersion",
    "base",
    "semantic"
  ],
  "title": "Spacing tokens",
  "type": "object"
}
```

Skema hanya mencakup bentuk (T-1 sampai T-5 dan T-9). Aturan lintas nilai (T-6, T-7, T-8, dan tabrakan nama di T-3) hanya diperiksa `tokens check`.

**Uji kontrak** (`tooling/codegen/tests/tokens_contract.rs`, hanya memakai path publik `desktop_codegen`, dibuka dengan komentar dokumen `//!`):

| Uji | Yang dibuktikan | Langkah |
| --- | --- | --- |
| `committed_outputs_match_the_source` | `check(&repo_root(), tokens)` lulus dan barisnya diawali `tokens: ok, ` | 1 |
| `generation_is_deterministic` | Dua panggilan `plan` memberi `Plan` yang sama. Tiap keluaran berakhir `\n`, tidak memuat `\r`, tidak memuat path absolut akar repo (hasil `repo_root().canonicalize()`), dan memuat `Generated by desktop-codegen` serta `Do not edit` | 1 |
| `resolves_light_and_dark` | `parse` atas tiga file asli memberi nilai di tabel **Nilai hasil**, urutan nama `accent`, `background`, `text.primary`, dan `placeholders == 3` | 1 |
| `rejects_broken_sources` | 18 kasus di tabel berikut | 2 |
| `check_reports_stale_and_missing_outputs_and_generate_repairs_them` | Di salinan sumber di bawah `env!("CARGO_TARGET_TMPDIR")`: `check` melaporkan tiga keluaran `missing` dan tidak menulis; `generate` menulis tiga; `check` lulus; `generate` kedua menulis nol; keluaran yang diganti `{}` dilaporkan `differs from the generator output` dan hanya file itu yang ditulis ulang; sumber yang dirusak membuat `generate` gagal tanpa menulis | 2 |

**Kasus sumber rusak**. Tiap kasus mengganti kemunculan pertama teks di kolom Cari dengan teks di kolom Ganti pada file asli, memanggil `parse`, dan mencari pesan yang memuat teks di kolom terakhir. `\n` di tabel berarti satu newline, dan enam spasi sesudahnya adalah indentasi di **Isi file**. Kasus 5, 6, 12, 17, dan 18 memakai teks dari `serde` dan `serde_json`; bila teks itu berubah saat versinya naik, perbarui tabel ini. Semua kasus harus lulus lewat `cargo test -p desktop-codegen --locked` dan lewat `cargo test --workspace --locked`, karena fitur crate lain di workspace ikut disatukan pada perintah kedua.

| No. | File | Cari | Ganti | Pesan memuat |
| --- | --- | --- | --- | --- |
| 1 | warna | `"#FAFAFA"` | `"#fafafa"` | `colors.json: base.neutral.50.value: expected #RRGGBB with uppercase hex digits, found #fafafa` |
| 2 | warna | `"#FAFAFA"` | `"#FFF"` | `colors.json: base.neutral.50.value: expected #RRGGBB` |
| 3 | warna | `"dark": "neutral.950"` | `"dark": "neutral.900"` | `colors.json: semantic.background.dark: unknown base token neutral.900` |
| 4 | warna | `"dark": "neutral.950"` | `"dark": "accent"` | `colors.json: semantic.background.dark: unknown base token accent` |
| 5 | warna | `      "dark": "neutral.950"\n` | `      "light": "neutral.50"\n` | ``colors.json: duplicate field `light` `` |
| 6 | warna | `"dark": "neutral.50",\n      "on"` | `"on"` | ``colors.json: missing field `dark` `` |
| 7 | warna | `"light": "brand.primary",\n      "dark": "brand.primary"` | `"light": "neutral.50",\n      "dark": "neutral.50"` | `colors.json: base.brand.primary: not used by any semantic token` |
| 8 | warna | `"#111111"` | `"#999999"` | `colors.json: semantic.text.primary: contrast on background is ` |
| 9 | warna | `"on": "background"` | `"on": "surface"` | `colors.json: semantic.text.primary.on: unknown semantic color surface` |
| 10 | warna | `"on": "background"` | `"on": "text.primary"` | `colors.json: semantic.text.primary.on: a color cannot sit on itself` |
| 11 | warna | `"schemaVersion": 1` | `"schemaVersion": 2` | `colors.json: schemaVersion: expected 1, found 2` |
| 12 | warna | `"schemaVersion": 1,` | `"schemaVersion": 1, "focus": {},` | ``colors.json: unknown field `focus` `` |
| 13 | warna | `"neutral.50": {` | `"brand.primary": {` | ``colors.json: duplicate key `brand.primary` `` |
| 14 | warna | `"text.primary": {` | `"backgro.und": {` | `colors.json: semantic: names backgro.und and background collide once the dots are removed` |
| 15 | warna | `"Main text."` | `"Main */ text"` | `colors.json: semantic.text.primary.description: must match` |
| 16 | tipografi | `"body"` | `"title"` | `typography.json: roles.title: unknown role, expected one of: body` |
| 17 | tipografi | `"regular"` | `"heavy"` | ``typography.json: unknown variant `heavy`, expected `regular` or `semibold` `` |
| 18 | jarak | `"value": 12 }` | `"value": 12.5 }` | ``spacing.json: invalid type: floating point `12.5`, expected u64`` |

**Kasus gagal yang sudah diputuskan**:

| Kejadian | Perilaku yang diharapkan |
| --- | --- |
| Keluaran disunting tangan atau hilang | `check` dan `cargo test --workspace --locked` gagal dan menyebut perintah `generate`. Obatnya: jalankan `generate`, lalu commit. |
| Sumber diubah tanpa `generate` | Sama. Sumber dan keluarannya masuk commit yang sama. |
| Sumber tidak sah | Kode 1, semua masalah dicetak, tidak ada yang ditulis. |
| Warna merek asli membuat teks sulit dibaca | `check` gagal dengan pesan kontras T-8. Pilih nilai lain atau ubah pasangan `on` lewat pembaruan spec ini. |
| Emitter atau keluaran dihapus atau diganti nama | `check` hanya mengenal keluaran yang ada di `Plan`, jadi file lama tidak terdeteksi. Hapus file lama di commit yang sama dengan perubahan emitter. |
| Konflik merge di file keluaran | Ambil satu sisi, selesaikan konflik di file sumber, jalankan `generate`, lalu commit hasilnya. |
| Checkout Windows mengubah akhir baris | Dicegah `.gitattributes` (O-6), yang hanya menyentuh file `.json` di `design-system/`. Bila tetap terjadi di host itu, itu temuan untuk AC-12, bukan alasan melonggarkan `check`. |
| Sumber baru membuat hook Graphify gagal | Ikuti aturan spec `_root/0001`: tambah entri `.graphifyignore` dan catat di spec pemilik file. Pada percobaan untuk spec ini hook menerima semua file baru tanpa entri tambahan. |
| `desktop-codegen` gagal dibangun di host lain | Pelanggaran aturan netral host spec `_root/0001`. Alat diperbaiki; perintah `--workspace` tidak diganti. |

**Critical test scenarios**:

* Jalur utama: sumber asli lulus `check`, `generate` menulis nol file, memverifikasi **AC-2**, **AC-6**.
* Arti token: token semantik terurai ke nilai terang dan gelap yang benar, memverifikasi **AC-2**, **AC-3**, **AC-8**.
* Sumber rusak: 18 kasus ditolak dengan pesan yang menyebut file dan tempatnya, memverifikasi **AC-3**, **AC-4**, **AC-7**.
* Keluaran menyimpang: file hilang atau disunting tertangkap oleh `check` dan oleh `cargo test`, lalu diperbaiki `generate`, memverifikasi **AC-5**.
* Bisa diulang: dua kali `generate` memberi byte yang sama, tanpa `\r`, tanpa path absolut, memverifikasi **AC-6**.
* Batas: alat tidak membawa anggota workspace atau pustaka UI, dan tidak ada yang bergantung padanya, memverifikasi **AC-9**.
* Cakupan: hanya file di tabel milestone yang berubah, dan 16 path macOS tetap, memverifikasi **AC-1**, **AC-10**.
* Akses: tidak ada peran atau izin di fitur ini. Yang ditolak adalah sumber yang tidak sah, memverifikasi **AC-4**.

## Perubahan dokumen

Langkah 3 Build plan mengubah dua dokumen berikut. Bagian lain kedua dokumen tidak diubah. Kedua file biasanya terlarang bagi sub sesi, jadi brief developer untuk milestone 2 perlu izin eksplisit.

`docs/design-system/README.md`. Dokumen itu hanya punya satu judul. Sesudah paragraf yang diawali "Bukti minimum kelak" dan sebelum baris tautan `[Scope dan urutan kerja]`, tambahkan:

```markdown
## Yang sudah ada

| Lokasi | Isi |
| --- | --- |
| `design-system/tokens/colors.json` | Tiga nilai dasar dan warna semantik `background`, `text.primary`, dan `accent`, masing masing dengan nilai terang dan gelap. Nilai dasarnya masih bertanda `placeholder` sampai warna merek ditetapkan. |
| `design-system/tokens/typography.json` | Peran `body` dengan bobot hurufnya. Tiap aplikasi memetakan peran ke gaya teks native. |
| `design-system/tokens/spacing.json` | Skala jarak dan jarak semantik `gap.control` dan `inset.window`, dalam satuan logis. |
| `design-system/schemas/` | Skema JSON untuk ketiga file token, hasil `desktop-codegen`. Jangan disunting. |
| `tooling/codegen/` | Paket `desktop-codegen`, yang memeriksa sumber token dan menulis ulang file turunannya. |

`radius.json` dan `motion.json` belum dibuat, karena jendela sapaan belum memakainya.

Cara mengubah token: sunting file di `design-system/tokens/`, jalankan `cargo run -p desktop-codegen --locked -- tokens generate`, lalu `cargo run -p desktop-codegen --locked -- tokens check`, dan commit sumber bersama file turunannya. Rincian dan alasannya ada di spec 0003.
```

Jadikan kata `spec 0003` di kalimat terakhir tautan ke `docs/specs/_root/0003-tema-bersama-dan-kontrol-native/index.md`; dari dokumen itu path relatifnya diawali `../specs/_root/`.

`docs/architecture/README.md`:

1. Di bagian "Tata letak yang dituju", ganti kalimat "Root workspace, `crates/core/`, `crates/macos-ffi/`, `bindings/swift/`, dan `apps/macos/` sudah ada." dengan:

   ```markdown
   Root workspace, `crates/core/`, `crates/macos-ffi/`, `bindings/swift/`, `apps/macos/`, `design-system/`, dan `tooling/codegen/` sudah ada.
   ```

2. Di tabel "Yang sudah ada", sesudah baris `apps/macos/`, tambahkan dua baris:

   ```markdown
   | `design-system/tokens/`, `design-system/schemas/` | Sumber token warna, tipografi, dan jarak, beserta skema hasil generator, dari spec 0003. |
   | `tooling/codegen/` | Paket `desktop-codegen`, alat yang memeriksa sumber bersama dan menulis ulang file turunannya. Tidak ada anggota yang bergantung padanya. |
   ```

   Jadikan kata `spec 0003` tautan ke `docs/specs/_root/0003-tema-bersama-dan-kontrol-native/index.md` dengan path relatif `../specs/_root/`.

Tabel "Aturan arah dependensi" di dokumen arsitektur tidak diubah di langkah ini. Tabel itu dikunci baris demi baris dengan spec `_root/0001`, jadi baris untuk `desktop-codegen` masuk lewat pembaruan spec itu (lihat Follow-up).

## Build plan

Pendekatan build proyek adalah Tracer Bullet. Untuk fitur ini benang tipisnya adalah satu jalur dari file token, lewat pemeriksaan, sampai file turunan yang dibandingkan oleh uji. Sesudah itu jalur gagal ditebalkan, lalu dokumen mengikuti. Satu langkah menjadi satu commit dengan subject `type(scope): spec _root/0003 step <n>, ...`, kecuali langkah 2 yang menjadi dua commit.

**Milestone 1: Alat dan sumber token** (langkah 1 dan 2)

1. Buat paket `desktop-codegen` (tujuh path di tabel **Tata letak**, dengan uji `committed_outputs_match_the_source`, `generation_is_deterministic`, dan `resolves_light_and_dark`), tiga file token, dan `.gitattributes`. Tambah `tooling/codegen` ke `members`, jalankan `cargo build -p desktop-codegen` sekali tanpa `--locked`, lalu `cargo run -p desktop-codegen --locked -- tokens generate` untuk menulis tiga skema. Cocokkan skema dengan bagian **Isi file**. Semua masuk satu commit. Langkah ini memenuhi bagian isi skema dan `grep` dari AC-7; penolakan kunci tidak dikenal dibuktikan di langkah 2, satisfies **AC-1**, **AC-2**, **AC-6**, **AC-7**, **AC-8**, **AC-9**, **AC-10**
2. Tambah uji `rejects_broken_sources` (18 kasus) dan `check_reports_stale_and_missing_outputs_and_generate_repairs_them`, lalu commit (commit pertama langkah 2). Pada working tree yang bersih sesudah commit itu, dengan `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, jalankan setiap perintah AC-1 sampai AC-10, catat perintah dan keluarannya di `verify.md` bagian `## Milestone 1: Alat dan sumber token`, hapus `target/`, lalu commit `verify.md` (commit kedua langkah 2, subject yang sama). Perintah `git log` di AC-10 dijalankan sebelum commit kedua dan sekali lagi sesudahnya; yang kedua hanya menambah `verify.md`, satisfies **AC-3**, **AC-4**, **AC-5**, **AC-7**

**Milestone 2: Dokumen mengikuti keadaan nyata** (langkah 3, sesudah milestone 1)

3. Perbarui `docs/design-system/README.md` dan `docs/architecture/README.md` sesuai bagian **Perubahan dokumen**, dan catat pemeriksaan AC-11 di `verify.md` bagian `## Milestone 2: Dokumen mengikuti keadaan nyata`, satisfies **AC-11**

**Milestone 3: Bukti di host Windows dan Linux** (langkah 4, `BLOCKED`, `owner: user`)

4. Pada host Windows `x86_64-pc-windows-msvc` dan host Linux `x86_64-unknown-linux-gnu`, jalankan tiga perintah AC-12 dan catat OS, arsitektur, versi, perintah, dan keluaran di `verify.md` bagian `## Milestone 3: Bukti di host Windows dan Linux`. Sesi koordinasi di Mac tidak menjalankan langkah ini, satisfies **AC-12**

## Consequences

**Positive**:

* Satu tempat untuk mengubah token, dan satu perintah untuk tahu apakah semuanya masih selaras.
* Keluaran yang menyimpang tertangkap oleh `cargo test --workspace --locked`, gerbang yang sudah dijalankan lead, tanpa skrip atau CI baru.
* Alat tidak menambah satu pun paket pihak ketiga ke `Cargo.lock`. Lockfile hanya bertambah paket alat itu sendiri.
* Skema tidak bisa berbeda dari validator, dan editor yang mengerti JSON Schema memeriksa token saat kamu mengetik.
* Scope 009, 010, dan 011 cukup menambah satu modul dan satu berkas uji. Subperintah dan uji gerbang mengikuti polanya.
* Mengganti warna merek aman: pemeriksaan kontras berkata bila teks utama menjadi sulit dibaca.

**Negative / tradeoffs**:

* Format token adalah milik repo ini, bukan format standar alat desain. Ekspor dari alat desain butuh konversi, atau pembaruan spec ini.
* Aturan bentuk ditulis sebagai kode Rust. Mengubah bentuk berarti mengubah kode alat, bukan menyunting satu file skema.
* Pemeriksaan pola nama dan deskripsi ditulis tangan di Rust, sedangkan skema memuat pola yang setara sebagai teks. Kesetaraannya dijaga uji contoh, bukan dibuktikan mesin.
* `cargo build --workspace` di setiap host sekarang ikut membangun alat ini. Biayanya kecil (sekitar 5 detik dari kosong di Mac koordinasi), tetapi belum diuji di Windows dan Linux.
* Tiap OS wajib menulis emitter sendiri di paket bersama. Perubahan satu emitter menyentuh crate yang dipakai semua tim.
* Token tipografi tidak membawa ukuran. Ukuran huruf tiap OS bisa berbeda beberapa pixel, karena masing masing memakai gaya teks native.
* Warna `accent` menggantikan warna aksen pilihan pengguna di OS untuk tindakan utama. Kontras `accent` terhadap latar dan terhadap teks tombol tidak diperiksa otomatis, hanya lewat pemeriksaan manual K-7.
* `check` tidak mencari file hasil generator yang sudah tidak punya emitter. File seperti itu harus dihapus dengan tangan saat emitternya dihapus.
* Tiga nilai warna masih contoh. Tampilan belum mencerminkan merek sampai kamu memberi nilainya.

**Neutral**:

* `.gitattributes` adalah file akar baru. Scope lain yang menambah folder sumber bersama menambah barisnya sendiri, per jenis file.
* Milestone 2 butuh izin eksplisit di brief developer untuk dua dokumen yang biasanya terlarang.
* `tooling/validation/` tetap rencana. Scope 002 memutuskan apakah pemeriksaan arsitektur tinggal di sana atau di paket yang sama.
* `/develop` menulis `verify.md` saat build. Spec ini tidak membuatnya.

## Follow-up

* [ ] Nilai dari user: tiga warna dasar (`brand.primary`, `neutral.50`, `neutral.950`) masih contoh. Yang dibutuhkan: warna merek utama, warna latar terang dan gelap, warna teks terang dan gelap, dan apakah tampilan gelap memakai rona merek yang berbeda. Sesudah nilai masuk, hapus `"placeholder": true`, jalankan `generate` dan `check`, dan perbarui tabel **Nilai hasil**, AC-2, AC-6, AC-8, dan uji `resolves_light_and_dark` lewat pembaruan spec ini.
* [x] Jawaban atas Follow-up Scope 008 di spec `macos/0001` (bagian bersama): sumber token, skema, dan alat ditetapkan di sini. Jarak `12` dan `20` menjadi `gap.control` dan `inset.window`. Lebar `320` tetap konstanta macOS (K-3). Modul `DesignSystem`, emitter Swift, dan pemeriksaan terang dan gelap di Mac menunggu spec macOS scope 008.
* [x] Jawaban atas Follow-up `tooling/codegen/` di spec `_root/0001` dan `macos/0001`: folder itu dibuat spec ini sebagai paket `desktop-codegen`. Generator binding tetap di crate jembatan.
* [x] Lead dan pemilik spec `_root/0001`: tambahkan baris `desktop-codegen` ke tabel "Aturan arah dependensi" di spec itu dan di `docs/architecture/README.md` dalam satu commit (uji `architecture_rule_table_matches_the_spec_row_by_row` mengunci keduanya). Usulan baris: `| desktop-codegen | Pustaka teks umum seperti serde dan serde_json. | Anggota workspace mana pun; pustaka UI; SDK khusus OS. Tidak ada anggota yang bergantung padanya. |`. Perbarui juga tabel "Lokasi dan pemilik": `tooling/codegen/` berpaket `desktop-codegen`, dan `design-system/` sudah dibuat. Catat `.gitattributes` sebagai file akar yang diatur spec `_root`. Baris tabel aturan dan baris `tooling/codegen/` diterapkan di spec `_root/0001` (2026-11-15). Baris `design-system/` di tabel lokasi tidak diubah karena tabel itu mencatat keadaan sesudah spec `_root/0001`, dan `.gitattributes` dicatat lead di daftar file bersama.
* [ ] Lead: penerapan macOS scope 008 menambah file di bawah `apps/macos/` (folder `DesignSystem`). AC-12 spec `macos/0001` mengunci tepat 16 path dan melarang folder itu, dan `test-preferences.json` menyebut aturan yang sama. Spec `macos/0001` perlu diperbarui lebih dulu atau bersamaan dengan spec macOS scope 008.
* [ ] Lead: tambahkan tautan balik ke spec ini di berkas scope 008, dan pertimbangkan memasukkan `.gitattributes` ke daftar file bersama satu pemilik per wave.
* [ ] Lead: sesi architect scope 010 dan 011 belum menerima bentuk alat ini. Sesi scope 009 (`arc-f001-spec`) sudah. Nilainya ada di bagian **Decision**, **API surface**, dan **Aturan keluaran**.
* [ ] Spec macOS scope 008: emitter Swift (K-1, K-2), modul `DesignSystem`, pemakaian di `GreetingView` (K-3, K-4), penerapan `accent` lewat tint, pemetaan `body` ke gaya teks SwiftUI, dan langkah manual K-5 dan K-7.
* [ ] Spec Windows scope 008 (menunggu host Windows): emitter untuk definisi Rust atau resource WinUI, tema kontras tinggi (himpunan tampilan di spec ini hanya `light` dan `dark`; menambah tampilan ketiga adalah pembaruan spec ini dengan `schemaVersion` baru), dan AC-12.
* [ ] Spec Linux scope 008 (menunggu host Linux): emitter untuk CSS GTK atau definisi Rust, pemetaan `accent` ke warna bernama libadwaita, dan AC-12.
* [ ] Scope 002: jadikan `cargo run -p desktop-codegen --locked -- tokens check` bagian pemeriksaan tetap, tulis aturan "sunting sumber, jalankan `generate`, commit bersama" di `AGENTS.md`, dan putuskan apakah pemeriksaan arsitektur masuk paket ini atau `tooling/validation/`.
* [ ] Scope 009, 010, dan 011: tambah sumber masing masing sebagai modul di `desktop-codegen` dengan pola `Source`, berkas uji `<sumber>_contract.rs`, aturan keluaran O-1 sampai O-7, dan baris `.gitattributes` untuk jenis file di foldernya. Build mereka berjalan sesudah milestone 1 spec ini masuk `main`.
* [ ] Scope 011: bila merek membawa huruf sendiri, nama huruf dan berkasnya adalah aset merek. Token tipografi di sini tetap peran dan bobot.
* [ ] Scope 012: `scripts/generate-design-tokens.sh` dan `scripts/validate.sh` membungkus perintah `generate` dan `check` yang sama dengan `--locked`, dan CI menjalankannya di ketiga runner.
* [ ] Agent Skill dan server MCP untuk `serde` dan JSON Schema belum dicari. Tidak ada yang dibutuhkan untuk membangun spec ini; cari bila kelak berguna.
* [ ] `radius.json` dan `motion.json` dari dokumen sistem desain belum dibuat. Buat lewat pembaruan spec ini saat ada layar yang memakainya.
* [ ] AC-12 dan milestone 3 menunggu host Windows dan Linux (`owner: user`).
