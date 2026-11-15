# 0001. Batas workspace dan pemilik: alasan

Catatan keputusan untuk [index.md](index.md). `/develop` tidak perlu membaca file ini.

## Context

Repo ini merencanakan tiga aplikasi desktop native yang memakai satu Rust Core. Pada 2026-11-15 `main` hanya berisi dokumentasi, hook Git, dan skill koordinasi. Belum ada `Cargo.toml`, jadi belum ada tempat yang pasti untuk kode pertama, dan dua spec lain yang berjalan bersamaan (scope 003 untuk kontrak `hello`, scope 004 untuk jendela macOS) butuh nama dan path crate yang sama.

[Peta arsitektur](../../../architecture/README.md) sudah menetapkan arahnya: Core bebas dari pustaka UI dan SDK khusus OS termasuk dependensi transitif, macOS memanggil Core lewat crate penghubung tipis dan binding UniFFI, Windows dan Linux mengimpor Core langsung lewat Cargo, dan crate Core, penghubung macOS, serta aplikasi Rust Windows dan Linux menjadi anggota satu Cargo Workspace. Dokumen itu juga meminta modul baru dibuat hanya saat ada tanggung jawab nyata, dan [kriteria 32](../../../quality/README.md) melarang layanan untuk fitur mendatang.

Gaya yang membentuk keputusan:

- **Tiga host, satu yang tersedia.** Hanya Mac yang ada sekarang. Host Windows dan Linux menyusul, dan saat aplikasi mereka bergabung build di Mac tidak boleh patah.
- **Gerbang lead memakai `--workspace`.** Pemeriksaan sebelum merge menjalankan `cargo clippy --workspace --all-targets --locked -- -D warnings`. Perintah itu harus tetap hijau di setiap host, berapa pun anggotanya.
- **Pustaka UI native butuh pustaka sistem.** GTK mencari pustaka lewat skrip build, Windows App SDK hanya ada di Windows. Dependensi seperti itu tidak boleh masuk graf build host lain.
- **Rust stable, dikunci sesudah kombinasinya diperiksa.** Dokumen arsitektur meminta compiler, SDK, pustaka, dan runtime dinilai sebagai satu kombinasi sebelum versi dikunci.
- **File root punya satu pemilik per wave.** `Cargo.toml` root, `Cargo.lock`, `rust-toolchain.toml`, dan `.gitignore` disentuh banyak scope, jadi aturan siapa boleh mengubah apa harus jelas.
- **Kriteria yang harus tercakup:** 1 (workspace berhasil dibangun), 2 (Core dibangun dan diuji mandiri), 27 (dokumentasi arsitektur tersedia), dan 32 (hanya layanan yang dibutuhkan fondasi), ditambah baris **Done when** scope 001.

Bila keputusan ini ditunda, scope 003 dan 004 masing masing akan menebak nama crate dan tata letak, dan aplikasi OS pertama yang bergabung akan menentukan sendiri cara ia hidup berdampingan dengan host lain.

## Options considered

### Opsi 1: Satu workspace, anggota eksplisit, aturan netral host

Satu manifest virtual di root dengan `members` yang ditulis satu per satu. Setiap anggota khusus OS menaruh dependensi OS di bawah tabel `[target.'cfg(...)'.dependencies]` dan memberi kodenya atribut `cfg`, sehingga di host lain ia terkompilasi menjadi biner kecil yang hanya melapor bahwa ia bukan untuk host itu.

**Pros**:

- `cargo build --workspace`, `clippy --workspace`, dan `test --workspace` hijau di semua host tanpa daftar paket per host.
- Satu `Cargo.lock`, jadi ketiga aplikasi memakai versi dependensi yang sama.
- Menambah anggota adalah satu baris di manifest root yang terlihat saat review.

**Cons**:

- Tiap aplikasi OS membawa `fn main()` pengganti dan atribut `cfg`.
- Build aplikasi OS di host lain hanya membuktikan kerangkanya terkompilasi.
- `Cargo.lock` memuat dependensi semua OS.

### Opsi 2: Satu workspace dengan `default-members` hanya Core

Semua crate menjadi anggota, tetapi `default-members = ["crates/core"]` membuat `cargo build` tanpa flag hanya membangun Core. Aplikasi OS tidak perlu bisa dibangun di host lain; tiap host memakai daftar `-p` sendiri.

**Pros**:

- Aplikasi OS tidak butuh kode pengganti.
- `cargo build` dan `cargo test` polos selalu aman di host mana pun.

**Cons**:

- `--workspace` patah di host asing begitu aplikasi pertama bergabung, jadi gerbang lead dan CI harus memelihara daftar paket per host.
- `cargo test` polos yang hanya menguji Core mudah disangka menguji semuanya.

### Opsi 3: Aplikasi OS di luar workspace

Workspace hanya memuat Core dan penghubung macOS. `apps/windows/` dan `apps/linux/` menjadi paket mandiri dengan lockfile sendiri, dikecualikan lewat `exclude`.

**Pros**:

- Host asing tidak pernah melihat aplikasi OS lain.
- Tiap aplikasi bebas memilih versi dependensi dan bahkan toolchain.

**Cons**:

- Bertentangan dengan peta arsitektur, yang menyebut aplikasi Rust Windows dan Linux sebagai anggota Cargo Workspace.
- Tiga lockfile bisa menyimpang, sehingga "Core yang sama" tidak lagi berarti versi dependensi yang sama.
- Perintah gerbang harus diulang per folder.

### Opsi 4: Anggota lewat glob `crates/*` dan `apps/*`

Sama dengan Opsi 1 atau 2, tetapi `members` memakai glob.

**Pros**:

- Crate baru otomatis bergabung tanpa menyentuh manifest root.

**Cons**:

- `apps/macos/` adalah proyek Xcode tanpa `Cargo.toml`, jadi glob `apps/*` gagal sampai folder itu dikecualikan.
- Anggota bergabung tanpa satu baris yang bisa direview, padahal setiap anggota wajib mematuhi aturan netral host.

## Rationale

Opsi 1 dipilih karena dua gaya dari Context saling mengunci: gerbang lead sudah memakai `--workspace`, dan hanya Mac yang tersedia sekarang. Dengan Opsi 2 atau 3, hari ketika `apps/linux/` bergabung adalah hari ketika gerbang di Mac harus diganti dengan daftar paket, tepat saat tim Linux belum tentu ada untuk memperbaikinya. Opsi 1 memindahkan biaya itu ke tempat yang benar: aplikasi yang bergabung yang menanggung aturan, dengan pola yang sudah dicoba (lihat "Bukti percobaan").

Opsi 3 ditolak karena melawan peta arsitektur dan memecah lockfile. Opsi 4 ditolak karena `apps/macos/` bukan crate, dan karena daftar eksplisit adalah satu satunya tempat review bisa bertanya "apakah anggota ini mematuhi aturan?".

Folder untuk penghubung macOS, binding, dan aplikasi tidak dibuat sekarang. Peta arsitektur meminta modul dibuat saat ada tanggung jawab nyata, dan kriteria 32 melarang kerangka untuk fitur yang belum dikerjakan. Spec ini cukup mencadangkan path dan nama.

Bukti arah dependensi sengaja dibuat sekecil mungkin: Core tanpa dependensi sama sekali, sehingga `cargo tree` mencetak satu baris. Pemeriksaan yang lebih kaya (daftar larangan, pemeriksaan transitif saat Core punya dependensi) adalah milik scope 002, yang berjalan sesudah scaffold ada.

## Keputusan yang diambil

Mode keputusan koordinasi adalah `auto`: opsi rekomendasi diambil dan pilihan keduanya dicatat. `berat` berarti mahal dibalik sesudah dibangun.

| No. | Pertanyaan | Diambil | Pilihan kedua | Tanda | Yang berubah bila pilihan kedua dipakai |
| --- | --- | --- | --- | --- | --- |
| 1 | Bagaimana aplikasi OS bergabung tanpa mematahkan host lain? | Opsi 1: anggota eksplisit, aturan netral host | Opsi 2: `default-members` hanya Core | biasa | AC-1 dan invarian `--workspace`; gerbang lead memakai daftar `-p` per host |
| 2 | Nama paket Core dan awalan anggota | `desktop-core`, awalan `desktop-` | `foundation-core`, awalan `foundation-` | berat | Setiap AC yang menyebut `desktop-core` (AC-1 sampai AC-4, AC-8, AC-9), bagian "Isi file", serta spec `_root/0002` dan spec macOS |
| 3 | Versi toolchain yang dipin | `1.99.1` persis, `rust-version = "1.99"` | `1.100.0` persis | biasa | AC-4, bagian "Isi file" |
| 4 | Edisi dan resolver | Edisi `2024`, `resolver = "3"` | Edisi `2021`, `resolver = "2"` | biasa | Bagian "Isi file" |
| 5 | Cara minimal membuktikan arah dependensi | Perintah `cargo tree` yang tertulis di spec | Crate `xtask` dengan perintah `check-deps` sekarang | biasa | AC-3, AC-7, tambah satu langkah Build plan di milestone 1 |
| 6 | Apakah folder rencana dibuat sekarang? | Tidak, hanya dicadangkan | Buat folder dengan README penanda | biasa | AC-7, langkah 1 Build plan |
| 7 | Isi `.gitignore` | `/target` ditambah entri umum Xcode, SwiftPM, dan macOS | Hanya `/target`, sisanya di scope 004 | biasa | Bagian "Isi file", langkah 1 |
| 8 | Bolehkah Core memakai `uniffi`? | Tidak. Alat binding tinggal di `desktop-macos-ffi` | `uniffi` di Core di balik feature | biasa | Tabel aturan arah dependensi; desain scope 004 |
| 9 | Kebijakan menaikkan pin | Patch cukup commit yang lulus gerbang, minor lewat pembaruan spec | `channel = "stable"` yang mengambang | biasa | AC-4, bagian "Aturan untuk manifest root" |
| 10 | Lint seluruh workspace | Tidak ada `[workspace.lints]` sekarang | `unsafe_code` dan grup clippy ditetapkan sekarang | biasa | AC-7, bagian "Isi file" |
| 11 | Kapan `[workspace.dependencies]` dibuat? | Oleh konsumen pertama Core (scope 004) | Ditulis sekarang | biasa | Bagian "Isi file" |
| 12 | Tempat uji API publik Core | `crates/core/tests/` | `tests/contracts/` di root sebagai paket uji sendiri | biasa | Tabel "Lokasi dan pemilik"; spec `_root/0002` |
| 13 | Rujukan dan cross check | Tanpa bagian References; cross check oleh model lain | Rujukan sumber saja | biasa | Tidak ada |
| 14 | Bahasa rustdoc dan deskripsi paket Core | Inggris, mengikuti spec `_root/0002` | Indonesia | biasa | Bagian "Isi file" (`crates/core/src/lib.rs`, `description` di `crates/core/Cargo.toml`) |
| 15 | Cara membuat hook Graphify menerima manifest virtual di root | `.graphifyignore` di root berisi `/Cargo.toml` | Mengubah `scripts/update_graphify.py` supaya menoleransi manifest tanpa node | biasa | Bagian "Isi file", AC-7, langkah 1 Build plan |

Alasan singkat per keputusan:

1. Lihat Rationale di atas.
2. `core` tidak bisa dipakai sebagai nama paket karena bentrok dengan pustaka bawaan Rust, jadi butuh awalan. `foundation` bertabrakan makna dengan framework `Foundation` milik Apple yang diimpor hampir setiap file Swift; modul binding bernama `Foundation...` akan membingungkan di aplikasi macOS. `desktop` adalah nama repo dan terbaca wajar untuk semua anggota (`desktop-core`, `desktop-linux`). Kedua pilihan nyaris setara dan nama ini masuk ke spec lain, jadi ditandai `berat`. Membatalkannya murah sampai scope 004 membangun binding.
3. `1.99.1` adalah stable yang aktif di Mac koordinasi, sudah mendapat satu rilis patch, dan terpasang lengkap dengan `rustfmt` dan `clippy`, jadi tidak ada unduhan. `1.100.0` terbit 2026-11-09, enam hari sebelum spec ini, dan belum dicoba dengan Xcode 27.1 atau alat binding. Kelemahannya nyata: Rust hanya memperbaiki stable terbaru. Kenaikan ke `1.100.x` adalah satu baris sesudah scope 004 membuktikan kombinasinya.
4. Edisi `2024` adalah edisi stable terbaru menurut `rustc 1.99.1`. Manifest virtual tidak punya edisi untuk menurunkan resolver, jadi `resolver = "3"` ditulis eksplisit. Resolver itu memperhitungkan `rust-version` saat memilih versi dependensi, yang berguna begitu anggota lain membawa dependensi luar.
5. Crate pemeriksa sekarang berarti kode tooling sebelum ada yang perlu diperiksa selain "Core tidak punya dependensi". Scope 002 memiliki pemeriksaan otomatis dan berjalan sesudah scaffold ada.
6. Peta arsitektur dan kriteria 32.
7. `.gitignore` adalah file satu pemilik per wave. Pola `/target` tanpa garis miring penutup dipakai supaya `git check-ignore -q target` tetap lulus sesudah folder itu dihapus. Lima baris itu adalah keluaran build dan state pengguna yang umum, tidak bergantung pada desain proyek Xcode, dan menambahkannya sekarang menghindari wave lain menunggu file ini. Apakah `bindings/swift/` dilacak tetap milik scope 004.
8. Peta arsitektur menyebut crate penghubung tipis memanggil Core, dan dokumen kontrak menyebut binding Swift berasal dari API yang diekspos penghubung macOS. `uniffi` di Core akan ikut dikompilasi aplikasi Windows dan Linux, yang tidak membutuhkannya.
9. Pin yang mengambang membuat dua host bisa membangun dengan compiler berbeda pada commit yang sama, sehingga bukti per host tidak bisa dibandingkan.
10. `unsafe_code = "forbid"` tidak bisa berlaku untuk seluruh workspace karena crate UniFFI kelak menghasilkan kode `unsafe`. Lint khusus Core dimiliki spec `_root/0002` lewat `[lints.rust]` di manifest Core; kebijakan seluruh workspace milik scope 002.
11. Entri yang belum dipakai siapa pun adalah kerangka untuk pekerjaan mendatang. Scope 004 tetap menyentuh manifest root untuk menambah anggotanya.
12. Pada manifest virtual, `tests/` di root bukan folder target Cargo. Uji di `crates/core/tests/` berjalan dengan `cargo test -p desktop-core` tanpa paket tambahan.
13. Sumber keputusan ini adalah dokumen repo yang sudah ditautkan di teks, jadi daftar rujukan terpisah tidak menambah apa pun.
14. Diminta lead saat menerima spec ini pada 2026-11-15. Spec `_root/0002` menulis rustdoc Core dalam bahasa Inggris dan menambahkan bagiannya di bawah empat baris kerangka tanpa mengubahnya, jadi kerangka berbahasa Indonesia akan membuat dokumen crate bercampur bahasa. Kebijakan bahasa komentar untuk seluruh repo tetap milik scope 002.
15. Diputuskan lead pada 2026-11-15 sesudah build milestone 1 terblokir di commit langkah 1. Graphify 0.9.48 punya extractor untuk `Cargo.toml`, tetapi manifest virtual (tanpa `[package]`) menghasilkan nol node dan masuk `failed_sources`, dan `scripts/update_graphify.py` keluar dengan `Graphify extraction failed` bila daftar itu tidak kosong. Temuan ini dilaporkan `dev-f001-m1` dan direproduksi lead di luar repo: dengan `.graphifyignore` berisi `/Cargo.toml`, manifest root dilewati, ekstraksi lulus, dan `crates/core/Cargo.toml` tetap menghasilkan satu node. Entri itu memakai mekanisme Graphify sendiri dan sesuai `AGENTS.md` butir 2 (perbaiki penyebab yang dilaporkan, lalu ulangi commit). Mengubah skrip tidak diambil karena skrip hook milik user dan sengaja ketat. Akibatnya manifest root tidak muncul di graf pengetahuan sampai baris itu dihapus.

## Bukti percobaan

Dijalankan pada 2026-11-15 di folder sementara di luar repo, macOS 27.0.1 `arm64`, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1`, dengan `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`. Folder itu dihapus sesudahnya.

Workspace dengan isi seperti bagian "Isi file". Percobaan memakai komentar dokumen dan deskripsi paket berbahasa Indonesia; keputusan 14 kemudian menggantinya dengan teks Inggris, yang tidak memengaruhi hasil perintah mana pun di bawah:

| Perintah | Hasil |
| --- | --- |
| `cargo generate-lockfile` | Kode 0, `Cargo.lock` versi 4 dengan satu paket `desktop-core 0.1.0`, tanpa akses jaringan |
| `cargo build --workspace --locked` | Kode 0 |
| `cargo build -p desktop-core --locked` | Kode 0 |
| `cargo test -p desktop-core --locked` | Dua baris `test result: ok`, masing masing 0 test |
| `cargo fmt --all --check` | Kode 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Kode 0 |
| `cargo tree --workspace --depth 0 --prefix none --locked` | Satu baris, `desktop-core v0.1.0 (...)` |
| `cargo tree -p desktop-core --target all --all-features --prefix none --locked` | Satu baris |
| `grep -c '^\[\[package\]\]' Cargo.lock` | `1` |
| `git status --short` sesudah build | Kosong |
| `git check-ignore -q target`, sebelum dan sesudah `target/` ada | Kode 0 dengan pola `/target` |
| `rustc --version` di root | `rustc 1.99.1 (58cf8f9c3 2026-10-13)` |

Aturan netral host, dicoba dengan anggota tambahan `apps/linux` (paket `desktop-linux`) yang bergantung pada `desktop-core` dan, di bawah `[target.'cfg(target_os = "linux")'.dependencies]`, pada crate lokal tiruan dengan skrip build yang selalu gagal (meniru pustaka native yang tidak ada di Mac):

| Perintah di Mac | Hasil |
| --- | --- |
| `cargo build --workspace --locked` | Kode 0; crate tiruan tidak dibangun |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | Kode 0 |
| `cargo test --workspace --locked` | Kode 0 |
| `cargo tree -p desktop-linux -e normal,build --prefix none --locked` | `desktop-linux` dan `desktop-core` saja |
| `cargo tree -p desktop-linux -e normal,build --prefix none --locked --target all` | Menambah crate tiruan, jadi `--target all` memperlihatkan dependensi semua OS |

Titik masuk pengganti yang dipakai di percobaan ini adalah `fn main()` di bawah `#[cfg(not(target_os = "linux"))]` yang menulis satu baris ke stderr dan keluar dengan kode 1. Itu contoh yang terbukti terkompilasi, bukan keputusan; bentuk akhirnya milik scope 006 dan 007.

Satu temuan dari percobaan pertama: crate tiruan yang diletakkan di dalam root workspace dan dirujuk lewat `path` otomatis menjadi anggota, sehingga `--workspace` ikut membangunnya dan gagal. Dari situ datang aturan bahwa setiap folder crate di repo wajib tercantum di `members` dan mematuhi aturan spec ini.

Keadaan toolchain di Mac koordinasi pada tanggal yang sama: `stable` menunjuk `1.99.1`; `1.100.0 (0ef1bc386 2026-11-09)` juga terpasang; `rustup check` melaporkan pembaruan stable ke `1.100.0` tersedia.

## Cross check

Dibaca oleh model lain (read only) pada 2026-11-15 sebelum spec ini diajukan. Desain Cargo dinilai benar; temuan berikut diterapkan ke `index.md`.

| Temuan | Perbaikan |
| --- | --- |
| AC-7 memeriksa daftar file dengan rentang commit yang tidak disebut, dan tidak mengizinkan `verify.md` serta baris `**Status**` | AC-7 menyebut daftar file lengkap dan dua perintah pemeriksanya |
| Isi `Cargo.lock` tidak tertulis | Isi lengkap masuk "Isi file"; AC-4 mendapat perintah `grep` |
| Langkah 1 sampai 3 tidak bisa lulus sendiri sendiri sebagai commit terpisah | Digabung menjadi langkah 1; Build plan sekarang empat langkah dalam tiga milestone |
| Milestone disebut tetapi tidak didefinisikan | Build plan memuat tiga milestone bernama |
| Teks pengganti untuk dokumen arsitektur harus dikarang developer | Teks literal dan tempatnya tertulis di "Perubahan dokumen arsitektur" |
| Pola `/target/` membuat `git check-ignore -q target` gagal sesudah folder dihapus | Pola menjadi `/target` |
| `shared/specifications/` tidak ada di tabel lokasi | Baris ditambahkan |
| Letak baris baru `.gitignore` ambigu | Isi akhir file ditulis lengkap |

AC-4 diperjelas pada 2026-11-15 karena spec [macos/0001](../../macos/0001-sapaan-di-jendela-macos/index.md): hitungan satu paket di `Cargo.lock` berlaku pada akhir milestone 1, sebab langkah 1 spec itu menambah `desktop-macos-ffi` dan UniFFI sehingga lockfile memuat sekitar 77 paket, dan keputusan spec ini tidak berubah.

AC-8 diperjelas pada 2026-11-15 karena spec [macos/0002](../../macos/0002-batas-workspace-dan-pemilik.md): pemeriksaan teks, hitungan `grep`, dan hilangnya dua kalimat lama berlaku pada akhir milestone 2, sebab langkah 1 spec itu menambah baris lokasi macOS dan menyesuaikan kalimat yang ikut usang, dan keputusan spec ini tidak berubah.

Tabel aturan arah dependensi mendapat baris `desktop-codegen`, dan tabel lokasi mendapat baris `tooling/codegen/` tersendiri, pada 2026-11-15 karena spec [_root/0003](../0003-tema-bersama-dan-kontrol-native/index.md): spec itu membuat paket alat di `tooling/codegen/`, yang tidak bergantung pada anggota workspace mana pun dan tidak boleh menjadi dependensi anggota lain.
| AC-9 tidak menyebut target dan linker | Target dan prasyarat linker disebut |
| Bukti `cargo tree` hanya melihat host, fitur bawaan, dan tanpa dependensi test | Perintah memakai `--target all --all-features` dan semua jenis sisi |
| Aturan netral host tidak mencakup skrip build | Aturan mencakup `build.rs` dan dependensi build per target |
| "Tanpa prasyarat selain toolchain" keliru karena butuh linker sistem | Invarian dan user story menyebut linker sistem |
| Bukti untuk anggota OS di host lain terlalu ketat | Hasil yang diharapkan menjadi "tanpa pustaka UI atau SDK OS" |
| Perilaku titik masuk pengganti adalah desain aplikasi Windows dan Linux | Dipindah ke Follow-up scope 006 dan 007 |
