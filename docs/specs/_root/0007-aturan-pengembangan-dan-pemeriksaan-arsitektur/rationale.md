# 0007. Aturan pengembangan dan pemeriksaan arsitektur: alasan

Catatan keputusan untuk [index.md](index.md). `/develop` tidak perlu membaca file ini.

## Context

Pada 2026-11-15 repo sudah punya Cargo Workspace dengan tiga anggota (`desktop-core`, `desktop-macos-ffi`, `desktop-codegen`), binding Swift yang dilacak git, aplikasi macOS, dan dua sumber bersama yang diperiksa alat (`tokens`, lalu `config` yang sedang dibangun). Aturan pengembangan sudah ada, tetapi tersebar: tabel arah dependensi di spec `_root/0001`, R-1 sampai R-9 di spec `_root/0002`, aturan keluaran generator di spec `_root/0003`, `K-4` di spec `_root/0004`, dan AC-6 sampai AC-8 di spec `macos/0001`. `/audit` merangkum semuanya sebagai penunjuk di `AGENTS.md` dan `crates/core/AGENTS.md`.

Sebagian aturan sudah dikunci uji di `crates/core/tests/`, sebagian lagi hanya berupa perintah `cargo tree` dan `git grep` di spec yang dijalankan orang sekali saat verifikasi. Ada tiga celah nyata:

- **Bukti dependensi Core berumur pendek.** Bukti "`cargo tree` mencetak tepat satu baris" berhenti berlaku begitu Core punya dependensi pertama. Tidak ada daftar larangan, dan tidak ada pemeriksaan untuk anggota lain selain perintah yang dijalankan saat anggota bergabung.
- **Uji Core tidak berdiri sendiri.** `macos_bridge_ownership.rs` memakai `include_str!("../../macos-ffi/src/lib.rs")`, dan `workspace_layout.rs` membaca dokumen di luar crate. Jadi `cargo test -p desktop-core` gagal dikompilasi tanpa `crates/macos-ffi/`, berseberangan dengan kriteria 2 dan maksud spec `_root/0001`.
- **Pemeriksaan sekali jalan.** Teks sapaan di luar Core, impor `DesktopMacosFFI` di luar `Interop`, dan konfigurasi di jembatan hanya diperiksa saat spec itu diverifikasi. Perubahan berikutnya tidak menjalankannya.

Gaya yang membentuk keputusan:

- **Gerbang yang sudah ada.** Lead menjalankan `cargo build`, `cargo test`, `cargo fmt`, dan `cargo clippy` dengan `--workspace --locked` sebelum setiap merge. Hook `pre-commit` hanya memperbarui Graphify, `.githooks/` terlarang bagi sub sesi, dan CI milik scope 012.
- **Tidak ada `.cargo/config.toml`.** Spec `_root/0001` melarangnya dan uji menguncinya, jadi alias seperti `cargo xtask` tidak tersedia tanpa pembaruan spec itu.
- **Tiga host, satu yang ada.** Pemeriksaan harus berjalan sama di Windows dan Linux, tanpa SDK UI dan tanpa jaringan.
- **Unduhan dibatasi.** Alat di luar dependensi Cargo yang sudah dikunci adalah langkah manual user.
- **Kriteria 32.** Pemeriksaan hanya untuk yang sudah ada di repo, bukan kerangka untuk aplikasi Windows dan Linux yang belum ada.
- **Kriteria yang harus tercakup:** 25 (pemeriksaan kepatuhan arsitektur dikonfigurasi), 26 (`AGENTS.md` memuat aturan yang bisa diperiksa), dan 32, ditambah baris **Done when** scope 002.

Bila keputusan ini ditunda, setiap spec baru akan menulis perintah `grep` sendiri untuk aturan yang sama, uji Core tetap bergantung pada jembatan macOS, dan dependensi pertama Core akan menghapus satu satunya bukti arah dependensi.

## Options considered

### Opsi 1: Sumber `architecture` di `desktop-codegen`

Pemeriksaan menjadi satu modul di alat yang sudah ada, memakai pola `Source` yang sama dengan `tokens` dan `config`. Sumber ini tidak punya keluaran, jadi `check` hanya menjalankan aturan. Satu uji di paket itu memanggil `check` untuk setiap sumber, sehingga `cargo test --workspace --locked` menjadi pintu masuk. Uji yang membaca file di luar Core pindah ke paket alat.

**Pros**:

- Tidak ada paket, dependensi, skrip, atau hook baru. Lockfile tidak berubah.
- Satu perintah untuk semua pemeriksaan sumber bersama dan arsitektur, dengan bentuk keluaran dan kode keluar yang sudah diuji.
- Sumber bersama berikutnya otomatis ikut gerbang.
- Uji yang membaca file lintas crate tinggal di paket yang memang membaca file repo, jadi Core kembali berdiri sendiri.

**Cons**:

- Nama paket `desktop-codegen` lebih sempit dari isinya.
- `generate` untuk sumber ini tidak berarti apa apa.
- Satu file (`tooling/codegen/src/lib.rs`) disentuh oleh beberapa scope.

### Opsi 2: Paket pemeriksa sendiri di `tooling/validation/`

Paket baru `desktop-validation` dengan bin dan uji sendiri, seperti rencana awal di peta arsitektur.

**Pros**:

- Nama dan folder cocok dengan isinya.
- Pemeriksaan arsitektur tidak bergantung pada perubahan API alat generator.

**Cons**:

- Anggota keempat, baris `members`, blok lockfile, dan baris aturan arah dependensi baru, semuanya untuk sekitar dua ratus baris pemeriksaan.
- Dua alat dengan dua bentuk keluaran, atau API `Source` yang disalin.
- Spec `_root/0003` sudah memutuskan generator dan validasi satu paket dan tidak membuat `tooling/validation/`.

### Opsi 3: Perluas uji di `crates/core/tests/` dan perintah di spec

Tambah uji teks baru di samping `workspace_layout.rs`, dan tulis perintah `cargo tree` serta `git grep` untuk anggota lain di `AGENTS.md`.

**Pros**:

- Tidak menyentuh alat. Pola uji sudah dikenal.

**Cons**:

- Memperparah masalah yang ingin diperbaiki: uji Core makin bergantung pada file di luar crate.
- Perintah `cargo tree` di `AGENTS.md` tetap dijalankan orang, bukan gerbang.

### Opsi 4: Alat pihak ketiga seperti `cargo-deny`

Daftar larangan dependensi ditulis di `deny.toml`, dan aturan teks lain tetap uji.

**Pros**:

- Alat yang matang untuk larangan dependensi, lisensi, dan advisori.

**Cons**:

- Harus dipasang terpisah di tiap host dan di CI, di luar `Cargo.lock`. Di sesi ini itu langkah manual user.
- Tidak memeriksa Swift, binding, atau dokumen, jadi tetap butuh pemeriksaan kedua.
- Advisori butuh jaringan, dan lisensi menunggu nilai `N1`.

## Rationale

Opsi 1 dipilih karena gerbang yang sudah dijalankan sebelum setiap merge adalah satu satunya tempat yang pasti berjalan. Hook tidak boleh diubah dan CI belum ada, jadi pemeriksaan yang tidak masuk `cargo test --workspace --locked` hanya akan dijalankan saat seseorang ingat. Alat `desktop-codegen` sudah membaca file repo, sudah punya bentuk keluaran yang diuji, dan sudah berjalan di gerbang lewat uji `committed_outputs_match_the_source`. Menambah satu sumber memakai semua itu tanpa paket baru, sesuai kriteria 32 dan keputusan spec `_root/0003` bahwa validasi tinggal di paket yang sama.

Larangan dependensi dibaca dari teks `Cargo.lock`, bukan dari `cargo metadata` atau `cargo tree`. Lockfile versi 4 memuat setiap paket yang mungkin dibangun, untuk semua target dan fitur, termasuk dependensi build dan uji, jadi penutupannya sama luas atau lebih luas dari `cargo tree --target all --all-features`. Membacanya tidak menjalankan proses lain, tidak berebut kunci Cargo saat dipanggil dari dalam `cargo test`, dan tidak butuh jaringan. Kekurangannya, lockfile tidak membedakan dependensi biasa dari dependensi uji, jadi aturan ini lebih ketat dari yang mutlak perlu. Untuk Core itu memang yang diminta spec `_root/0002` (tanpa dependensi uji).

Daftar persis dependensi luar langsung (`A-3`) ditambahkan di atas larangan kelas (`A-4`), karena larangan kelas hanya sebaik daftar nama polanya. Dengan daftar persis, setiap paket pihak ketiga baru terlihat sebagai satu baris yang diubah di review. Itu juga yang membuat bukti dependensi Core tetap berlaku sesudah Core punya dependensi pertama.

Pemindahan dua uji lintas crate menyelesaikan masalah Core yang tidak berdiri sendiri dengan perubahan terkecil: isinya tidak diubah, hanya lokasinya dan dua path `include_str!`. Uji itu tetap berjalan di gerbang yang sama.

## Keputusan yang diambil

Mode keputusan koordinasi adalah `auto`: opsi rekomendasi diambil dan pilihan keduanya dicatat. `berat` berarti mahal dibalik sesudah dibangun.

| No. | Pertanyaan | Diambil | Pilihan kedua | Tanda | Yang berubah bila pilihan kedua dipakai |
| --- | --- | --- | --- | --- | --- |
| 1 | Di mana pemeriksaan arsitektur tinggal? | Sumber `architecture` di `desktop-codegen` | Paket baru di `tooling/validation/` | biasa | Bentuk alat, AC-1, AC-10, semua langkah; tambah baris `members` dan aturan arah dependensi |
| 2 | Bagaimana uji Core berdiri sendiri? | Pindahkan `workspace_layout.rs` dan `macos_bridge_ownership.rs` ke `tooling/codegen/tests/` | Tetap di Core, ganti `include_str!` ke jembatan dengan baca saat jalan yang melewati file yang tidak ada | biasa | AC-7, langkah 3, butir `AGENTS.md` |
| 3 | Dari mana graf dependensi dibaca? | Teks `Cargo.lock` | `cargo metadata --format-version 1` sebagai proses anak | biasa | Bagian "Cara tiap aturan membaca repo"; uji jadi lambat dan bergantung pada `cargo` di PATH |
| 4 | Bentuk larangan dependensi | Daftar persis dependensi luar langsung, ditambah kelas terlarang di seluruh penutupan | Hanya kelas terlarang | biasa | `A-3` hilang, kasus `N-4`, `N-5` |
| 5 | Apakah `[workspace.lints]` dibuat? | Tidak. Tiap anggota tetap membawa `[lints.rust]`, dijaga `A-5` | `[workspace.lints]` dengan `unsafe_code` dan `missing_docs`, tiap anggota `lints.workspace = true` | biasa | `A-5`, AC-8, tiga manifest, uji `manifest_forbids_unsafe_code_and_warns_on_missing_docs` dan `bridge_manifest_forbids_unsafe_code` |
| 6 | Grup clippy | Bawaan, dengan `-D warnings` di perintah gerbang | Tambah `clippy::pedantic` sebagai `warn` | biasa | Kode yang sudah ada perlu dirapikan; AC-8 |
| 7 | `rustfmt.toml` atau `clippy.toml` | Tidak ada, dan dilarang `A-5` | Satu `rustfmt.toml` dengan `edition` dan `max_width` | biasa | `A-5`, kasus `N-8`, AC-8 |
| 8 | Bahasa komentar kode dan rustdoc (`N2`) | Inggris untuk kode, Indonesia untuk dokumen | Indonesia untuk keduanya | berat | `H-6`, butir "Bahasa" di `AGENTS.md`, semua rustdoc dan deskripsi paket yang sudah ada, teks `$comment` keluaran generator |
| 9 | ADR atau spec | Spec di `docs/specs/` menggantikan ADR; `docs/decisions/` tidak dibuat | Buat `docs/decisions/` untuk keputusan tanpa scope | biasa | AC-9, teks pengganti dokumen arsitektur |
| 10 | Kapan pemeriksaan berjalan | Di `cargo test --workspace --locked`, gerbang yang sudah ada | Perintah terpisah di `AGENTS.md` | biasa | AC-3, uji `every_source_passes_check_on_the_repo`, tabel "Kapan pemeriksaan berjalan" |
| 11 | Cakupan pemeriksaan dokumen | Tautan di semua Markdown kecuali `docs/scope/coordination/`; rujukan di setiap `AGENTS.md` | Hanya tautan di `AGENTS.md` | biasa | `A-12`, kasus `N-20`, `P-4`, AC-5 |
| 12 | Binding Swift sama dengan keluaran generator | Tetap perintah di Mac dari spec `macos/0001`; `A-9` menangkap fungsi yang hilang di host mana pun | Uji yang menjalankan generator UniFFI di `cargo test` | biasa | Tabel "Peta aturan ke pemeriksaan", Follow-up spec macOS scope 002 |
| 13 | Apakah `desktop-macos-ffi` dilarang membawa SDK OS? | Tidak, mengikuti kolom "Dilarang" di spec `_root/0001` | Dilarang, dengan pengecualian untuk `windows-sys` | biasa | Baris `desktop-macos-ffi` di `MEMBERS`, kelas |
| 14 | Paket Swift jarak jauh | Dilarang tanpa spec (`A-11`) | Diserahkan ke tinjauan | biasa | `A-11`, kasus `N-19` |
| 15 | Awalan ID aturan tinjauan manusia | `H-` | `M-` | biasa | Teks saja. `M-` sudah dipakai spec `macos/0004` untuk aturan keluaran macOS |
| 16 | Teks pengganti tiga dokumen yang tidak cocok dengan kode | Teks literal di bagian **Perubahan dokumen**, satu langkah dengan `A-12` dan `A-13` | Diserahkan ke `/sync` | biasa | Langkah 5, AC-9 |
| 17 | Rujukan dan cross check | Tanpa bagian References; cross check oleh subagent baru | Rujukan sumber saja | biasa | Tidak ada |
| 18 | Apakah pola kelas memuat nama crate Windows dan Linux sekarang? | Ya, sebagai data larangan untuk Core dan alat | Hanya pola yang sudah muncul di `Cargo.lock` (`uniffi`, `windows`) | biasa | Tabel kelas; spec Windows dan Linux menambah polanya sendiri |

Alasan singkat per keputusan:

1. Lihat Rationale di atas.
2. Pemindahan tidak mengubah isi uji, jadi tidak ada aturan yang hilang. Pilihan kedua tetap membuat uji Core membaca dokumen arsitektur dan file Swift.
3. Proses anak `cargo` dari dalam `cargo test` bisa menunggu kunci build yang sedang dipegang, dan `cargo metadata` bisa mencoba memperbarui lockfile. Teks lockfile cukup untuk nama, versi, dan sisi.
4. Lihat Rationale.
5. Cargo menolak `lints.workspace = true` bersama `[lints.rust]` di manifest yang sama. Jadi bila kelak satu anggota butuh pengecualian, misalnya aplikasi Windows yang memanggil API lewat FFI, anggota itu harus melepas seluruh lint workspace. Dengan tabel per anggota, pengecualian itu satu baris yang terlihat. Sekarang ketiga anggota memakai baris yang sama, jadi pilihan kedua hanya menghemat enam baris dan memaksa dua uji yang sudah ada ditulis ulang.
6. Kode yang ada lulus grup bawaan dengan `-D warnings`. `pedantic` menambah peringatan yang sering bertentangan dengan R-3 (misalnya `needless_pass_by_value` yang sudah dibuka dengan `allow`).
7. Gaya bawaan sudah dipakai semua file, dan file konfigurasi tanpa isi yang berbeda hanya menambah tempat yang harus dijaga.
8. Semua rustdoc, deskripsi paket, nama uji, pesan alat, dan teks `$comment` keluaran generator sudah berbahasa Inggris, dan pesan `serde` yang dikutip spec `_root/0003` juga Inggris. Dokumen repo sudah berbahasa Indonesia. Menetapkan keadaan sekarang tidak mengubah file apa pun. Ditandai `berat` karena ini nilai `N2` dari user, dan membaliknya kelak berarti menulis ulang rustdoc dan keluaran generator.
9. Semua keputusan nyata sudah ada di `docs/specs/`, masing masing dengan `rationale.md` yang memuat konteks, opsi, dan alasan, yaitu isi sebuah ADR. Folder kedua berarti dua tempat mencari keputusan.
10. Gerbang itu sudah dijalankan lead sebelum setiap merge dan oleh developer sebelum melapor. Perintah terpisah hanya berjalan bila diingat.
11. Kriteria 26 meminta aturan yang bisa diperiksa di `AGENTS.md`, dan `AGENTS.md` menunjuk spec. Tautan rusak di spec sama merugikannya. `docs/scope/coordination/` dikecualikan karena isinya catatan proses yang ditulis lead dan memuat contoh path relatif ke worktree lain.
12. Generator UniFFI membaca pustaka statis hasil build untuk target macOS, jadi uji seperti itu hanya bermakna di Mac dan memperlambat gerbang di setiap host. `A-9` menangkap kasus paling umum (fungsi baru di jembatan tanpa binding dibuat ulang) di host mana pun.
13. Penutupan `desktop-macos-ffi` sekarang memuat `windows-sys` dan `windows-link`, lewat alat baris perintah UniFFI. Tabel prosa hanya melarang crate aplikasi dan pustaka UI. Melarangnya berarti mengubah aturan spec `_root/0001`.
14. Paket jarak jauh mengunduh kode di luar `Cargo.lock` dan di luar review. Sekarang tidak ada satu pun, jadi aturannya tidak mengubah apa pun.
15. Lihat kolom pilihan kedua.
16. `/sync` menyunting baris yang ia miliki dan tidak menulis ulang prosa. Menulis teksnya di sini juga membuat `A-12` dan `A-13` lulus pada commit yang sama.
17. Sumber keputusan ini adalah dokumen dan kode repo yang sudah ditautkan di teks.
18. Aturan "Core tidak membawa pustaka UI atau SDK OS, langsung maupun transitif" berlaku sekarang, dan yang paling mungkin masuk adalah pustaka yang dipakai host lain. Daftar nama itu data larangan, bukan kerangka layanan, jadi tidak bertentangan dengan kriteria 32. Tanpa nama itu, Core bisa membawa `gtk4` lewat dependensi transitif dan `A-4` tetap lulus. `A-3` tetap mencegah dependensi langsung baru tanpa review, jadi pilihan kedua masih aman, hanya lebih lemah. Spec Windows dan Linux menambah nama crate yang benar benar mereka pakai.

## Jawaban atas masukan `/audit`

| No. | Masukan | Jawaban |
| --- | --- | --- |
| 1 | Bahasa komentar kode dan rustdoc | Keputusan 8: Inggris untuk kode, Indonesia untuk dokumen (`H-6`). |
| 2 | `[workspace.lints]`, grup clippy, `rustfmt.toml`, `clippy.toml` | Keputusan 5, 6, dan 7. Tidak ada yang dipindah, jadi uji `manifest_forbids_unsafe_code_and_warns_on_missing_docs` tidak berubah. |
| 3 | Daftar larangan dependensi termasuk transitif, sisi yang diizinkan per anggota | `A-2` sampai `A-4`. Larangan "Core tanpa dependensi" yang sudah dikunci `contract_surface.rs` tidak diulang; sesudah dependensi pertama Core, uji itu dihapus dan `A-3` mengambil alih (bagian "Cara menambah atau mengubah aturan"). |
| 4 | Rumah pemeriksaan dan `include_str!` lintas crate | Keputusan 1 dan 2. Tanpa `xtask`, jadi spec `_root/0001` tidak perlu diubah untuk alias. |
| 5 | Kapan pemeriksaan jalan | Keputusan 10. `"gate": null` di `test-preferences.json` benar dan tidak diubah, karena field itu hanya untuk proyek tanpa penguji. Hook tidak diubah. CI milik scope 012. |
| 6 | Pemeriksaan tetap dari spec lain | AC-3 spec `_root/0002` menjadi `A-8`. R-1 tetap `contract_surface.rs`. R-8 menjadi `A-9` dan `H-1`. AC-7 spec `macos/0001` menjadi `A-9`, AC-8 menjadi `A-8` dan `A-10`. Binding sama dengan keluaran generator: keputusan 12. `tokens check` dan `config check` lewat `every_source_passes_check_on_the_repo`. |
| 7 | Keluaran generator yang menyimpang | Aturannya tetap O-1 sampai O-7 spec `_root/0003`. Spec ini menambah satu uji yang menjalankan `check` untuk semua sumber, sehingga sumber yang terdaftar tidak bisa lolos dari gerbang (AC-3). |
| 8 | Aturan yang diperiksa manusia | `H-1` sampai `H-10`, dengan kapan ditinjau dan buktinya, juga di `AGENTS.md`. |
| 9 | ADR atau spec | Keputusan 9. |
| 10 | Bukti otomatis untuk `AGENTS.md` sendiri | `A-12` dan `A-13`. |
| 11 | Tiga dokumen yang tidak cocok dengan kode | Keputusan 16. Teks pengganti ada di bagian **Perubahan dokumen**, termasuk `tooling/validation/` yang tidak akan dibuat. |
| 12 | Hook `pre-merge-commit` dan gabungan `manifest.json` | Di luar spec ini selama `.githooks/` terlarang. Dicatat di Follow-up untuk user. Spec ini tidak menulis aturan pengembangan tentang hook. |

Dua masukan sesudah brief: usul `DESKTOP_CODEGEN_ROOT` dari tester scope 008 tidak dibutuhkan pemeriksaan ini dan tetap milik spec `_root/0003` (Follow-up). Catatan hook untuk sub sesi adalah aturan proses koordinasi, bukan aturan pengembangan, jadi tidak masuk spec ini.

## Bukti percobaan

Dijalankan pada 2026-11-15 di Mac koordinasi, pada `main` `abd3a43` (dan diulang untuk tautan pada `cd0c908`), dengan skrip Python di scratchpad sesi yang meniru cara baca di bagian "Cara tiap aturan membaca repo". Skrip itu bukan alat yang dibangun; ia membuktikan bahwa aturan lulus pada repo sekarang dan bahwa angkanya masuk akal.

| Pemeriksaan | Hasil |
| --- | --- |
| Penutupan transitif `desktop-core` dari `Cargo.lock` | 0 paket |
| Penutupan transitif `desktop-codegen` | 11 paket: `itoa`, `memchr`, `proc-macro2`, `quote`, `serde`, `serde_core`, `serde_derive`, `serde_json`, `syn`, `unicode-ident`, `zmij`. Tidak ada yang cocok dengan kelas mana pun. |
| Penutupan transitif `desktop-macos-ffi` | 75 paket. Yang cocok kelas: delapan paket `uniffi*` (kelas `binding`, diizinkan) dan `windows-sys`, `windows-link` (kelas `os-sdk`, diizinkan). Tidak ada kelas `ui`. |
| Dependensi langsung | `desktop-core`: tidak ada. `desktop-macos-ffi`: `desktop-core`, `uniffi`. `desktop-codegen`: `serde`, `serde_json`. Sama dengan tabel `MEMBERS`. |
| Entri dependensi dengan versi | `syn 2.0.119` dan `syn 3.0.10`. Satu satunya nama dengan dua versi; cara resolusi entri di spec menanganinya. |
| Cabang OS dan path ke luar di `crates/core/` | Tidak ada `target_os`, `target_family`, `cfg(windows)`, atau `cfg(unix)`. `../../` dan `CARGO_MANIFEST_DIR` hanya ada di dua uji yang dipindah (dan di `crates/core/AGENTS.md`, yang bukan `.rs`). |
| `shared/config` dan `public.json` di Core, jembatan, binding | Tidak ada |
| Teks sapaan di bagian bukan uji | Hanya `crates/core/src/greeting.rs`, di rustdoc dan di `format!`. Uji Swift dan uji Rust memuatnya, dan semuanya di folder `tests` atau `Tests`. |
| Jembatan dan binding | Satu `pub fn` di jembatan, `hello`, dengan tanda tangan dan badan sesuai `A-9`. Binding memuat `public func hello(` dan `public func uniffiEnsureDesktopMacosFfiInitialized(`. |
| Impor `DesktopMacosFFI` di `apps/macos/` | Hanya `apps/macos/Interop/Greeter.swift` |
| Paket Swift jarak jauh | Tidak ada. `apps/macos/Package.swift` hanya memakai `.package(path: "../../bindings/swift")`. |
| Tautan Markdown (tanpa `.claude/`, `graphify-out/`, `docs/scope/coordination/`) | 44 file, 197 tautan, 0 rusak. Dengan folder itu ikut, 3 rusak: dua contoh `NNNN-slug` di skill koordinasi dan satu tautan di file aturan umum koordinasi. Karena itu kedua folder dikecualikan. |
| Calon path di `AGENTS.md` | 6 di akar dan 9 di `crates/core/`, semuanya ada. Rujukan spec dan nama paket sesudah `-p` semuanya ada. |

## Cross check

Dibaca subagent baru berkonteks bersih pada model `sonnet` (model lain dari penulis, `claude-opus-5-5`), read only, pada 2026-11-15 sebelum spec ini diajukan. Subagent itu menjalankan ulang aturan `A-8`, `A-12`, dan `A-13` pada repo dengan skrip sementara, dan memeriksa path `include_str!` uji yang dipindah, hitungan aturan, perintah `grep`, kasus `N` dan `P`, serta kalimat lama yang diganti di bagian **Perubahan dokumen**. Tidak ada blocker. Temuan berikut diterapkan ke `index.md`.

| Temuan | Perbaikan |
| --- | --- |
| `<teks>` di pesan `A-6` dan `A-7` tidak didefinisikan, dan tidak jelas satu masalah per file atau per baris | Paragraf "Satu masalah per baris": satu masalah per baris yang melanggar, `<teks>` adalah pola yang ditemukan |
| Urutan "menurut ID aturan" sebagai teks menaruh `A-10` sebelum `A-2` | Urutan menurut angka ID, lalu path, lalu nomor baris |
| `A-5` mengulang uji lint Core yang sudah ada, berlawanan dengan invarian "satu aturan, satu pemeriksaan" | Invarian dan peta aturan menyebut tumpang tindih itu dan alasannya |
| Baris `desktop-codegen` di `MEMBERS` melarang kelas `binding`, padahal tabel prosa tidak | Kelas `binding` dihapus dari baris itu; kalimat di bawah tabel menjelaskan pemetaannya |
| "Entri terakhir `SOURCES`" bertabrakan dengan sumber `config` dari spec `macos/0004` | Urutan di `SOURCES` bebas |
| Milestone 2 sampai 4 tidak boleh menyentuh baris `**Status**`, jadi AC-10 akan gagal saat status dimajukan | Baris `**Status**` diizinkan di semua milestone |
| Pencocokan tanda tangan `A-9` bisa lulus untuk awalan seperti `-> String2` | Tanda tangan harus diikuti tepat ` {` |
| Fungsi jembatan yang tidak ada di Core bisa memberi tiga pesan sekaligus | Pemeriksaan pertama yang gagal adalah satu satunya masalah per fungsi |
| `A-12` tidak menyebut tautan berjudul, `<...>`, path dari `/`, dan file yang belum dilacak | Keempatnya disebut |
| Cara menulis teks sapaan di kode alat hanya disebut untuk modul `architecture` | Berlaku untuk semua kode bukan uji di `tooling/` |
| AC-1 menuntut `git status --short` kosong tanpa menyebut working tree bersih | AC-1 dimulai dari working tree yang bersih |

Satu temuan tidak diterapkan: pola kelas `ui` dan `os-sdk` memuat nama crate yang baru dipakai host lain, yang dinilai mendekati kriteria 32. Alasannya di keputusan 18.

Subagent juga memastikan hal berikut benar pada repo: kedua path `include_str!` baru di `macos_bridge_ownership.rs` dan path lama di `workspace_layout.rs` menunjuk file yang ada dari `tooling/codegen/tests/`; kedua file memuat 10 dan 4 uji; `A-12` memberi 0 tautan rusak, termasuk pada spec ini; calon path `A-13` pada teks `AGENTS.md` baru semuanya ada sesudah langkahnya; AC-5 dan AC-6 memberi hitungan yang tertulis; setiap kalimat lama di **Perubahan dokumen** ada persis di dokumen sekarang.
