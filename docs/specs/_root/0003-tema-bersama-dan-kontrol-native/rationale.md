# 0003. Tema bersama dan kontrol native: catatan keputusan

Berkas ini menyimpan alasan di balik [spec `_root/0003`](index.md). Kamu tidak perlu membacanya untuk membangun. Bacalah saat ingin tahu mengapa sebuah pilihan diambil, atau saat spec ini akan diubah.

## Context

> ⚠️ Premise note: [Sistem desain bersama](../../../design-system/README.md) merencanakan himpunan token yang lebar: warna sukses, peringatan, kesalahan, garis, radius, dan gerak. Membangun semuanya sekarang berarti menulis token yang tidak dipakai layar mana pun, dan itu dilarang kriteria 32 serta "Batas fase fondasi". Pembingkaian yang benar: spec ini menetapkan strukturnya, lalu hanya mengisi token yang dipakai jendela sapaan. Sisanya masuk saat ada layar yang membutuhkannya.

Scope 008 meminta warna, tipografi, dan jarak yang sama maknanya di jendela sapaan tiga OS, dengan tampilan terang dan gelap dari token yang bisa dihasilkan ulang, tanpa mengorbankan fokus, penskalaan, dan aksesibilitas kontrol asli. Sebelum spec ini belum ada satu file token pun, dan jarak di `GreetingView` masih angka literal (`12`, `20`, `320`). Kalau bentuknya tidak ditetapkan, tiap OS akan menyalin angka dengan tangan dan ketiganya pelan pelan berbeda.

Beberapa hal sudah tetap:

* [Sistem desain bersama](../../../design-system/README.md) menyebut lokasi `design-system/tokens/` dan `design-system/schemas/`, pembedaan token dasar dan alias semantik, pemetaan terang dan gelap, dan syarat "satu sumber berversi menghasilkan keluaran yang sama saat dibuat ulang".
* [Spec `_root/0001`](../0001-batas-workspace-dan-pemilik/index.md) melarang `.cargo/config.toml`, meminta `members` satu per satu, nama paket berawalan `desktop-`, empat field yang diwarisi, dan aturan netral host. Spec itu mencadangkan `tooling/codegen/` dan `tooling/validation/` untuk scope pertama yang membutuhkannya.
* [Spec `macos/0001`](../../macos/0001-sapaan-di-jendela-macos/index.md) memberi preseden generator: keluaran dilacak git dan dibuktikan dengan menjalankan generator lalu `git diff --exit-code`. AC-12 spec itu mengunci 16 path di sisi macOS.
* Kriteria 15, 16, dan 17 di [Penerimaan dan bukti fondasi](../../../quality/README.md) adalah yang harus dibuktikan. Kriteria 32 melarang layanan dan token untuk fitur mendatang.

Hal yang membentuk keputusan:

* Hanya ada satu host, Mac Apple Silicon dengan Rust 1.99.1. Windows dan Linux belum tersedia, jadi apa pun yang diputuskan harus masuk akal untuk host itu tanpa bisa diuji di sana.
* Sesi koordinasi tidak boleh membuka jendela. Makin banyak yang terbukti tanpa jendela, makin sedikit yang menunggu user.
* Setiap perintah gerbang memakai `--workspace`, dan tiap worktree membangun dari `target/` kosong di disk yang dipakai banyak sesi. Dependensi baru dibayar oleh setiap sesi di setiap host.
* Warna merek dan nilai token asli hanya user yang tahu.
* Scope 009, 010, dan 011 akan mengikuti bentuk alat yang ditetapkan di sini, dan skrip build serta CI adalah milik scope 012.
* Hook Graphify ikut berjalan pada setiap commit dan bisa menolak jenis file baru.

Yang belum tetap, dan menjadi isi spec ini: format dan isi sumber token, apa arti skema dan siapa yang menjalankannya, versi, di mana kode generator tinggal, bagaimana ia dijalankan, apa yang dihasilkannya, dan bagaimana keluaran dibuktikan sama saat dibuat ulang.

## Options considered

Keputusan utama adalah bentuk alat dan cara bentuk token diperiksa. Keputusan lain yang lebih kecil ada di tabel sesudah bagian ini.

### Opsi 1: Alat Rust di workspace, bentuk token sebagai tipe, skema dihasilkan

Satu paket `desktop-codegen` di `tooling/codegen/`. Bentuk token ditulis sebagai tipe `serde` yang menolak kunci tidak dikenal, aturan lintas nilai ditulis tangan, dan file JSON Schema ditulis oleh alat yang sama dari konstanta yang sama.

**Pros**:

* Tidak menambah paket pihak ketiga ke `Cargo.lock`: `serde` dan `serde_json` sudah terkunci lewat UniFFI.
* Generator hanya menerima apa yang bisa ia baca ke tipe, jadi tidak ada sumber yang lolos periksa tetapi tidak dimengerti generator.
* Skema tidak bisa berbeda dari validator, karena keduanya satu kode.
* Toolchain sudah dipin `rust-toolchain.toml`, jadi alat yang sama berjalan di tiga host.
* Pesan kesalahan ditulis sendiri, jadi AC bisa mengunci teksnya.

**Cons**:

* Mengubah bentuk berarti mengubah kode Rust, bukan satu file skema.
* Pola nama diperiksa dengan kode tulis tangan, dan skema memuat pola setara sebagai teks. Kesetaraan itu dijaga uji contoh.
* Sekitar 1.000 baris Rust (dengan uji) yang dimiliki repo ini.

### Opsi 2: Skema JSON tulisan tangan dan crate mesin JSON Schema

File `*.schema.json` ditulis tangan sebagai sumber kebenaran, dan alat memakai crate `jsonschema` untuk menjalankannya, ditambah pemeriksaan lintas nilai.

**Pros**:

* Skema lebih dulu: bentuk diubah di satu file data yang dikenal banyak alat.
* Lebih sedikit kode pemeriksaan bentuk yang ditulis sendiri.

**Cons**:

* `jsonschema =0.61.2` tanpa fitur bawaan menambah 57 paket ke lockfile bersama (78 menjadi 135), dan `target/` untuk alat ini naik dari 38 MB menjadi 247 MB. Itu dibayar setiap worktree di setiap host, untuk memeriksa tiga file kecil.
* Generator tetap butuh akses bertipe sesudah validasi, jadi skema dan kode bisa berbeda, dan sumber yang lolos skema bisa membuat generator panik.
* Crate itu masih versi 0.x dengan rilis yang sering, jadi butuh pin persis dan kenaikan yang disengaja.

### Opsi 3: Alat siap pakai dari luar dengan format standar

Sumber memakai format token standar komunitas desain, dan generator adalah alat Node yang umum dipakai untuk itu.

**Pros**:

* Format dan alat yang dikenal desainer, dengan ekspor dari alat desain.
* Emitter untuk banyak target sudah tersedia.

**Cons**:

* Menambah Node dan pengelola paketnya sebagai toolchain kedua di tiga host, tanpa pin yang setara `rust-toolchain.toml`.
* Emitter bawaan menulis nilai tetap, sedangkan fondasi ini butuh pemetaan ke gaya teks dan mekanisme aksen native tiap OS, jadi emitter tetap ditulis sendiri.
* Aturan repo ini (token dasar yang menganggur ditolak, kontras, bentuk tertutup) tetap harus ditulis sebagai kode tambahan.

### Opsi 4: Skrip Python di `scripts/`

Generator sebagai skrip Python, karena Python 3 sudah dibutuhkan hook Graphify.

**Pros**:

* Tanpa anggota workspace baru dan tanpa waktu build.
* JSON didukung pustaka bawaan.

**Cons**:

* Versi Python tidak dipin, jadi "keluaran sama di host mana pun" bergantung pada mesin tiap orang.
* `scripts/` adalah milik scope 012, dan uji skrip tidak ikut `cargo test --workspace`, jadi butuh gerbang baru.
* Aplikasi Windows dan Linux adalah crate Rust; emitter mereka paling wajar ditulis di bahasa yang sama dengan pemakainya.

### Keputusan yang lebih kecil

| No. | Pertanyaan | Diambil | Pilihan kedua | Tanda | Yang berubah bila pilihan kedua |
| --- | --- | --- | --- | --- | --- |
| 1 | Bentuk alat | Satu paket Rust `desktop-codegen` di `tooling/codegen/` | Skrip Python di `scripts/` | berat | Seluruh Build plan; AC-5 dan AC-9; bentuk untuk scope 009 sampai 011 |
| 2 | Pemeriksaan bentuk | Tipe `serde`, skema dihasilkan | Skema tulisan tangan dan crate `jsonschema` | berat | Dependensi di **Isi file**, AC-7, AC-9, dan apakah scope 009 membawa skema tulisan tangan |
| 3 | Format sumber | JSON milik repo ini, tiga file sesuai dokumen sistem desain | Format token standar komunitas (`$value`, `$type`) | biasa | Tabel **Data model sketch**, isi tiga file token, `parse` |
| 4 | Isi token | Tiga warna semantik, satu peran, dua jarak | Himpunan lengkap dari dokumen sistem desain | biasa | AC-1, AC-2, tabel **Nilai hasil** |
| 5 | Tipografi | Peran dan bobot, dipetakan ke gaya teks native | Ukuran dan nama huruf sebagai angka dan teks | biasa | Tabel tipografi, K-6, dan penskalaan teks tiap OS |
| 6 | Warna merek di jendela | Warna semantik `accent` lewat mekanisme aksen native | Tidak ada warna merek; aksen mengikuti pengguna OS | berat | `colors.json`, tabel **Nilai hasil**, K-6 |
| 7 | Lebar minimum jendela | Konstanta milik tiap OS (K-3) | Token ukuran bersama | biasa | `spacing.json` dan K-3 |
| 8 | Tampilan | Himpunan tertutup `light` dan `dark` | Tampilan terbuka, termasuk kontras tinggi sekarang | biasa | Bentuk `semantic` di `colors.json`, `schemaVersion` |
| 9 | Kontras | Diperiksa alat lewat `on`, ambang 4,5 | Hanya pemeriksaan mata di jendela | biasa | T-8, AC-3 |
| 10 | Nilai sementara | Tanda `placeholder` per nilai, dilaporkan, tidak menggagalkan | `check` gagal selama masih ada nilai sementara | biasa | AC-8 dan gerbang lead |
| 11 | Versi | `schemaVersion` untuk bentuk, git untuk isi | Versi isi bergaya semver yang dicap ke keluaran | biasa | Dua kunci pembuka tiap file, O-2 |
| 12 | Keluaran | Dilacak git, dibandingkan oleh `check` dan satu uji | Dibuat saat build dan tidak dilacak | biasa | AC-5, AC-6, O-6 |
| 13 | Akar repo | `CARGO_MANIFEST_DIR/../..` saat kompilasi | Folder kerja saat perintah dijalankan, atau argumen `--root` | biasa | `repo_root` dan tabel perintah |
| 14 | Akhir baris | `.gitattributes` dengan `eol=lf` per jenis file | `check` menyamakan `\r\n` sebelum membandingkan | biasa | AC-6, O-6 |
| 15 | Draft skema | draft-07 | 2020-12 | biasa | Baris `$schema` di tiga keluaran |
| 16 | Keluaran spec ini | Hanya skema; emitter OS masuk lewat spec OS | Emitter Swift ikut di sini, atau tabel rujukan Markdown | biasa | Build plan, AC-1, dan pembagian dengan spec macOS |
| 17 | `tooling/validation/` | Tidak dibuat; validasi sumber tinggal bersama generatornya | Paket kedua untuk validasi | biasa | Tabel **Tata letak** dan bentuk untuk scope 009 |

Alasan singkat untuk yang tidak dibahas di bagian Rationale:

* **3.** Format standar belum punya cara tetap untuk tampilan terang dan gelap yang bisa saya pastikan tanpa menebak, dan generator harus menolak apa pun yang tidak ia mengerti. Enam token tidak membenarkan parser format yang lebih lebar. Pindah format kelak hanya mengubah `parse` dan tiga file; keluaran tidak ikut berubah.
* **5.** Ukuran tetap mematahkan skala teks OS, dan ukuran bawaan tiap OS berbeda. Peran yang dipetakan ke gaya teks native menjaga "sama maknanya" tanpa memaksa tampilan asing.
* **7.** Ukuran jendela adalah perilaku jendela, yang menurut peta arsitektur milik tiap OS.
* **8.** Kontras tinggi di Windows butuh host Windows untuk dirancang dengan benar. Menambah tampilan ketiga kelak adalah `schemaVersion` baru.
* **10.** Menggagalkan gerbang selama nilai merek belum ada akan memblokir semua scope sesudah ini pada keputusan yang hanya user bisa ambil.
* **11.** Sumber dan keluaran tinggal di satu repo dan masuk commit yang sama, jadi commit sudah menjadi versi isi. Cap versi di keluaran hanya menambah satu hal yang bisa lupa dinaikkan.
* **13.** Perintah lalu berjalan dari folder mana pun di repo dan tidak bisa menulis ke pohon lain. Uji memakai parameter `root` di pustaka.
* **14.** Perbandingan yang longgar menyembunyikan penyimpangan. Satu baris `.gitattributes` adalah cara biasa.
* **15.** Kata kunci yang dipakai (`const`, `propertyNames`, `pattern`, `enum`) semuanya ada di draft-07, dan dukungan editor untuk draft itu paling luas.
* **1 dan 17.** Daftar `SOURCES`, argumen `<sumber>`, dan aturan tabrakan nama T-3 tidak dibutuhkan jendela sapaan sendiri. Semuanya ada karena brief meminta spec ini menetapkan bentuk alat untuk scope 009 sampai 011 dan untuk emitter tiap OS. Biayanya beberapa baris, dan tanpa itu scope berikutnya akan membuat alat kedua.
* **16.** Pilihan paling hemat adalah tanpa keluaran sama sekali sampai emitter OS pertama masuk: `check` hanya memvalidasi sumber, dan penulis skema datang belakangan. Itu ditolak karena `generate`, `check`, dan aturan O-1 sampai O-7 lalu tidak terbukti pada file nyata sampai spec macOS dibangun, padahal scope 009 sampai 011 sudah bergantung padanya. Skema adalah keluaran nyata yang paling murah, dan sekaligus memberi pemeriksaan di editor. Brief menetapkan cara SwiftUI memakai token sebagai isi spec macOS, dan AC-12 spec `macos/0001` belum mengizinkan file baru di `apps/macos/`. Skema sudah cukup sebagai keluaran nyata untuk membuktikan `generate` dan `check`. Tabel rujukan Markdown ditolak karena tidak ada yang memakainya.
* **17.** Generator harus memvalidasi sebelum menulis, jadi paket kedua hanya menggandakan pembaca sumber.

## Rationale

Opsi 1 dipilih karena tiga gaya dari Context saling menguatkan. Pertama, setiap dependensi dibayar semua sesi di semua host lewat `--workspace`; Opsi 1 tidak menambah satu pun paket pihak ketiga, sedangkan Opsi 2 menambah 57 dan Opsi 3 menambah toolchain kedua. Kedua, hanya ada satu host, jadi jaminan "keluaran sama di host mana pun" harus datang dari rancangan, bukan dari uji: toolchain yang dipin, tanpa jam, tanpa pecahan hasil hitung, urutan tetap, dan akhir baris yang dikunci. Ketiga, sesi koordinasi tidak boleh membuka jendela, jadi pemeriksaan harus ikut gerbang yang sudah ada. Uji `committed_outputs_match_the_source` menumpang `cargo test --workspace --locked`, sehingga tidak perlu menunggu skrip scope 012.

Skema yang dihasilkan, bukan ditulis tangan, menjawab satu hal yang Opsi 2 biarkan terbuka: dua definisi bentuk yang bisa berbeda. Di Opsi 1 hanya ada satu definisi, yaitu tipe dan konstanta di alat, dan file skema adalah cerminannya untuk editor dan pembaca. Harganya, mengubah bentuk berarti mengubah kode. Itu harga yang wajar, karena setiap perubahan bentuk memang memaksa emitter tiap OS ikut berubah.

Pada 2026-11-15 batas waktu AC diperjelas ("pada akhir milestone 1" dan "pada akhir milestone 2"), karena build spec `_root/0004`, `_root/0005`, dan `_root/0006` sudah menambah file di `tooling/codegen/`, baris di `.gitattributes`, nama di baris `sources:`, dan lokasi di `docs/architecture/README.md` secara sah. Angka, perintah, dan keputusan tidak berubah.

Isi token sengaja kecil karena kriteria 32. Tiga warna semantik cukup untuk membuktikan seluruh rantai: nilai dasar merek, alias semantik, dan pemetaan terang dan gelap. `accent` masuk (keputusan 6, bertanda berat) karena tanpa itu tidak ada nilai merek yang sampai ke layar, dan rantai "nilai merek ke alias ke kontrol" tidak terbukti. Pilihan keduanya, membiarkan aksen mengikuti pengguna OS, lebih hormat pada pengaturan pengguna; bila kamu memilih itu, `accent` dan `brand.primary` dihapus dan jendela hanya memakai warna netral.

## Bukti percobaan

Semua dicoba pada 2026-11-15 di Mac koordinasi (macOS 27.0.1, `arm64`, `rustc 1.99.1`), di salinan repo di folder sementara, dengan `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. Purwarupa berisi tujuh path `tooling/codegen/` dan enam path `design-system/` persis seperti tabel **Tata letak**.

| Yang dicoba | Hasil |
| --- | --- |
| `cargo build -p desktop-codegen` dari `target/` kosong, hanya `serde` dan `serde_json` | 5,4 detik, `target/` 38 MB, `Cargo.lock` 77 menjadi 78 paket, hanya blok `desktop-codegen` yang bertambah |
| Hal yang sama dengan `jsonschema = { version = "=0.61.2", default-features = false }` | 11,8 detik (32 detik CPU), `target/` 247 MB, `Cargo.lock` 135 paket |
| `cargo run -q -p desktop-codegen --locked -- tokens check` sebelum `generate` | Tiga baris `missing, run: ...`, `tokens: failed, 3 problems`, kode 1 |
| `tokens generate` pertama, lalu kedua | `3 written`, lalu `0 written` |
| `tokens check` sesudahnya | `tokens: ok, 6 semantic tokens, 3 placeholder values, 3 outputs up to date`, kode 0 |
| `tokens` tanpa aksi, dan `nope check` | Dua baris usage, kode 2 |
| `#FAFAFA` diubah menjadi `#fafafa` di `colors.json` | `design-system/tokens/colors.json: base.neutral.50.value: expected #RRGGBB with uppercase hex digits, found #fafafa`, kode 1 |
| Kunci `focus` ditambahkan | ``unknown field `focus`, expected one of `$schema`, `schemaVersion`, `base`, `semantic` at line 3 column 29``, kode 1 |
| `cargo test -p desktop-codegen --locked` | `tokens_contract`: `5 passed`, termasuk 18 kasus sumber rusak |
| `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo fmt --all --check` | Semua kode 0; uji `workspace_layout` milik spec `_root/0001` tetap lulus dengan anggota baru |
| `cargo tree -p desktop-codegen --depth 1 --prefix none --locked` | Tiga baris: `desktop-codegen v0.1.0`, `serde v1.0.229`, `serde_json v1.0.154` |
| `cargo tree --workspace --locked -i desktop-codegen --prefix none` | Satu baris |
| `cargo tree -p desktop-core --target all --all-features --prefix none --locked` | Tetap satu baris |
| `RUSTDOCFLAGS="-D warnings" cargo doc -p desktop-codegen --no-deps --locked` | Kode 0 |
| `git ls-files --eol design-system` dengan `.gitattributes` satu baris (`/design-system/**/*.json text eol=lf`) | Enam baris `i/lf w/lf attr/text eol=lf` |
| Skrip hook `scripts/update_graphify.py` pada salinan sekali pakai yang memuat semua file baru dan `.gitattributes` | `Graphify: 89 sources, ...`, tanpa `failed_sources`. Tidak butuh entri `.graphifyignore`. |

Catatan dari percobaan:

* `cargo generate-lockfile` menaikkan versi paket lain di lockfile. Karena itu **Isi file** meminta `cargo build` tanpa `--locked`.
* Rasio kontras di pesan T-8 dihitung dengan `powf`, yang memakai pustaka matematika OS. Karena itu uji hanya mencocokkan awal pesannya, dan O-5 melarang pecahan hasil hitung di file keluaran.
* `serde_json::Map` berurutan abjad secara bawaan, tetapi fitur `preserve_order` milik crate lain di workspace akan mengubahnya untuk semua anggota. Karena itu `to_canonical_json` mengurutkan sendiri, dan daftar `required` di skema disusun dari larik tetap.
* Kontras `#111111` terhadap `#FAFAFA` sekitar 18 banding 1. `#999999` terhadap `#FAFAFA` sekitar 2,7 banding 1, jadi kasus 8 memang gagal.

## Cross check

Draf spec dibaca oleh model lain (Claude Opus, hanya baca) pada 2026-11-15, dengan tugas utama mencari nilai dan keputusan yang belum tertulis. Mode keputusan `auto`, jadi perbaikan yang direkomendasikan langsung diterapkan.

Diterapkan:

* Langkah 2 Build plan menjadi dua commit, supaya perintah AC-6 (butuh working tree bersih) dan AC-10 (butuh commit yang sudah ada) bisa memberi hasil yang benar. AC-7 ikut dicatat di langkah 2.
* Rumus kontras ditulis dengan `max` dan `min`, karena kata "terang" dan "gelap" sudah berarti tampilan. Ambang memakai nilai mentah, dan rasio di pesan dipotong ke bawah.
* Perilaku `parse` saat `serde` gagal di satu file, urutan masalah, kata `problems` yang selalu jamak, urutan pasangan nama di T-3, nilai kembali `check` dan `generate`, siapa yang mencetak, escape di `to_canonical_json`, dan perilaku saat file keluaran tidak terbaca atau tidak tertulis, semuanya ditulis.
* Tabel **Kasus sumber rusak** memakai pasangan Cari dan Ganti yang persis untuk setiap kasus, dan harus lulus lewat `-p` maupun `--workspace`.
* `.gitattributes` dipersempit ke `/design-system/**/*.json`, supaya file biner yang kelak masuk folder itu tidak rusak saat checkout.
* K-2 menyebut kata kunci bahasa target, dan K-6 menyebut mode kontras tinggi yang mengganti palet.
* Blok baru di `docs/design-system/README.md` dipindah ke akhir dokumen dengan judul sendiri, supaya tabel Target tidak ikut masuk di bawahnya.

Tidak diterapkan:

* Pencarian file hasil generator yang sudah tidak punya emitter (memindai folder untuk penanda). Itu kode tambahan untuk kejadian yang belum ada, dan kriteria 32 meminta yang perlu saja. Akibatnya dicatat di Consequences dan di tabel kasus gagal.
* Daftar kata terlarang bersama di T-3. Kata kunci berbeda per bahasa target, jadi aturannya tinggal di emitter (K-2).

Pemeriksa menyatakan setiap AC punya langkah Build plan, dan kriteria 15, 16, 17 serta kedua bagian Done when tercakup oleh AC atau oleh kewajiban spec OS.
