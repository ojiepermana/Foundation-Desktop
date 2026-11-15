# 0007. Aturan pengembangan dan pemeriksaan arsitektur

**Scope**: [002 - Aturan pengembangan dan pemeriksaan arsitektur](../../../scope/002%20-%20Aturan%20pengembangan%20dan%20pemeriksaan%20arsitektur.md)
**Date**: 2026-11-15
**Status**: In Progress

## Summary

Spec ini mengubah aturan pengembangan yang sudah tersebar di beberapa spec menjadi satu daftar ber-ID, lalu memasang pemeriksaan otomatis untuk yang bisa diperiksa mesin. Pemeriksaan itu menjadi sumber baru bernama `architecture` di alat `desktop-codegen`, jadi `cargo test --workspace --locked` yang sudah dijalankan sebelum merge ikut menangkap dependensi terlarang dan keluaran generator yang menyimpang. Aturan yang hanya bisa dinilai orang ditulis sebagai daftar tinjauan di spec ini dan di `AGENTS.md`. Hasil build adalah alat yang menolak pelanggaran dengan satu baris per masalah, uji Core yang kembali berdiri sendiri, dan panduan root yang menunjuk aturan beserta cara ujinya.

## Requirements

**User stories**:

* Sebagai developer, saya ingin satu perintah yang berkata apakah perubahan saya melanggar arah dependensi atau membuat file turunan menyimpang, supaya saya tahu sebelum meminta merge.
* Sebagai penulis spec, saya ingin tahu persis apa yang harus saya ubah saat menambah anggota workspace atau dependensi, supaya aturan arsitektur tidak ditebak ulang di tiap spec.
* Sebagai pemeriksa, saya ingin daftar pendek aturan yang tidak bisa diperiksa mesin, supaya tinjauan saya terarah ke hal yang memang butuh mata orang.
* Sebagai developer Core, saya ingin `cargo test -p desktop-core` hanya gagal karena Core, bukan karena file Swift atau dokumen di luar crate.

**Acceptance criteria**:

Angka kriteria merujuk tabel di [Penerimaan dan bukti fondasi](../../../quality/README.md). Semua perintah dijalankan dari akar repo pada Mac koordinasi. Angka yang bertanda "pada akhir milestone 3" bertambah saat aturan atau anggota baru masuk lewat pembaruan spec.

* **AC-1** (kriteria 25, satu pintu masuk): pada working tree yang bersih, `cargo run -p desktop-codegen --locked -- architecture check` keluar dengan kode 0. Pada akhir milestone 3 baris terakhirnya tepat `architecture: ok, 13 rules, 3 members, 0 outputs up to date`. `cargo run -p desktop-codegen --locked -- architecture generate` mencetak `architecture: ok, 13 rules, 3 members, 0 outputs, 0 written`, dan `git status --short` sesudahnya tidak mencetak apa pun. `cargo run -p desktop-codegen --locked -- nope check` keluar dengan kode 2, dan baris kedua di stderr diawali `sources: ` serta memuat `architecture`.
* **AC-2** (Done when "pemeriksaan mendeteksi dependensi terlarang", kriteria 25): aturan `A-1` sampai `A-4` lulus pada repo dan menolak kasus `N-1` sampai `N-6` di tabel **Kasus yang ditolak**, masing masing dengan pesan yang memuat ID aturan di kolom terakhir. Pemeriksaan itu membaca `Cargo.lock`, jadi mencakup dependensi biasa, build, dan uji, untuk semua OS dan semua fitur, langsung maupun transitif. Buktinya uji `rejects_broken_repos` di `tooling/codegen/tests/architecture_contract.rs`.
* **AC-3** (Done when "pemeriksaan mendeteksi keluaran generator yang menyimpang", kriteria 25): uji `every_source_passes_check_on_the_repo` memanggil `check` untuk setiap entri `SOURCES`, jadi sumber yang terdaftar tidak bisa lolos dari gerbang. Pada working tree yang bersih, sesudah `printf '\n' >> design-system/schemas/colors.schema.json`, perintah `cargo test -p desktop-codegen --locked --test architecture_contract every_source_passes_check_on_the_repo` keluar dengan kode bukan 0 dan keluarannya memuat `differs from the generator output`. Sesudah `git checkout -- design-system/schemas/colors.schema.json` perintah yang sama lulus.
* **AC-4** (kriteria 25, pemeriksaan tetap dari spec lain): aturan `A-6` sampai `A-11` lulus pada repo dan menolak kasus `N-9` sampai `N-19`. Pada working tree yang bersih, sesudah `printf '\n// shared/config\n' >> crates/macos-ffi/src/lib.rs`, perintah `cargo run -p desktop-codegen --locked -- architecture check` keluar dengan kode 1, mencetak satu baris yang diawali `crates/macos-ffi/src/lib.rs: A-7: `, lalu `architecture: failed, 1 problems`. Sesudah `git checkout -- crates/macos-ffi/src/lib.rs` perintah yang sama keluar dengan kode 0.
* **AC-5** (kriteria 26, Done when "panduan root menunjukkan aturan dan cara uji"): `AGENTS.md` di akar memuat teks dari bagian **Perubahan dokumen**. `grep -c 'cargo run -p desktop-codegen --locked -- architecture check' AGENTS.md` mencetak 1 atau lebih, `grep -c 'tooling/codegen/tests/workspace_layout.rs' AGENTS.md` mencetak `1`, `grep -c 'crates/core/tests/workspace_layout.rs' AGENTS.md` mencetak `0`, dan `grep -c 'workspace_layout\|macos_bridge_ownership' crates/core/AGENTS.md` mencetak `0`. Aturan `A-12` dan `A-13` lulus pada repo dan menolak kasus `N-20` sampai `N-23`, jadi tautan, rujukan spec, nama paket, dan path file yang disebut `AGENTS.md` terbukti masih ada setiap kali gerbang berjalan.
* **AC-6** (Done when "aturan yang perlu diperiksa manusia terdokumentasi"): bagian **Aturan yang diperiksa manusia** spec ini memuat `H-1` sampai `H-10`, masing masing dengan kapan ditinjau dan apa buktinya. `AGENTS.md` di akar memuat heading `## Tinjauan manusia`, dan `grep -c '^- \*\*H-[0-9]*\*\*' AGENTS.md` mencetak `10`.
* **AC-7** (kriteria 2 lewat [spec `_root/0001`](../0001-batas-workspace-dan-pemilik/index.md), Core diuji sendiri): Pada akhir milestone 2, `git ls-files crates/core/tests` mencetak tepat tiga path: `contract_surface.rs`, `hello_contract.rs`, dan `hello_invariants.rs`. `git ls-files tooling/codegen/tests` memuat `workspace_layout.rs` dan `macos_bridge_ownership.rs`. `git grep -n '\.\./\.\./\|CARGO_MANIFEST_DIR' -- 'crates/core/*.rs'` tidak mencetak apa pun. `cargo test -p desktop-core --locked` dan `cargo test -p desktop-codegen --locked` keluar dengan kode 0. Pada commit langkah 3, `git show --stat -M --format= HEAD` menampilkan kedua file sebagai pemindahan (`=>`), dan jumlah uji di kedua file sama dengan sebelum dipindah.
* **AC-8** (kriteria 25, kebijakan lint dan format): aturan `A-5` lulus pada repo dan menolak kasus `N-7` dan `N-8`. `grep -c '^\[workspace\.lints' Cargo.toml` mencetak `0`. `git ls-files | grep -cE '(^|/)\.?(rustfmt|clippy)\.toml$'` mencetak `0`. `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, dan `cargo clippy --workspace --all-targets --locked -- -D warnings` keluar dengan kode 0.
* **AC-9** (kriteria 26, dokumen cocok dengan kode): `README.md` di akar, `docs/architecture/README.md`, dan `docs/contracts/README.md` memuat teks dari bagian **Perubahan dokumen**. `grep -c 'Semua fitur berstatus' README.md`, `grep -c 'docs/decisions' docs/architecture/README.md`, `grep -c 'tooling/validation' docs/architecture/README.md`, dan `grep -c 'Jika bagian `unsafe` diperlukan' docs/contracts/README.md` masing masing mencetak `0`. `test ! -e docs/decisions` keluar dengan kode 0. `cargo test -p desktop-codegen --locked --test workspace_layout` tetap lulus, jadi tabel aturan arah dependensi tidak ikut berubah.
* **AC-10** (kriteria 32): fitur ini hanya membuat yang dibutuhkan untuk memeriksa yang sudah ada di repo. Setiap commit fitur ini memakai subject yang memuat `spec _root/0007 step <n>`. `git log --name-only --format= --grep='spec _root/0007 step' | sort -u` hanya mencetak path dari tabel **File yang boleh diubah tiap milestone**. Daftar itu tidak memuat `Cargo.toml`, `Cargo.lock`, `.gitattributes`, atau path di bawah `scripts/`, `.github/`, `.githooks/`, `apps/`, dan `bindings/`. Pada akhir milestone 3, `cargo tree -p desktop-codegen --depth 1 --prefix none --locked` tetap mencetak tepat tiga baris, jadi fitur ini tidak memberi alat dependensi baru. `test ! -e tooling/validation` keluar dengan kode 0.
* **AC-11** (kriteria 25 pada host lain; `BLOCKED`, `owner: user`): pada host Windows `x86_64-pc-windows-msvc` dan host Linux `x86_64-unknown-linux-gnu` dengan toolchain `1.99.1`, `cargo test -p desktop-codegen --locked` dan `cargo run -p desktop-codegen --locked -- architecture check` keluar dengan kode 0. Hasil dicatat per host dengan OS, arsitektur, versi, perintah, dan keluaran.

## Decision

**Chosen option**: Opsi 1: sumber `architecture` di `desktop-codegen`, aturan ber-ID di spec ini.

Aturan pengembangan menjadi dua daftar. Daftar `A` berisi aturan yang diperiksa mesin oleh sumber baru `architecture` di paket `desktop-codegen`, memakai pola `Source` dari [spec `_root/0003`](../0003-tema-bersama-dan-kontrol-native/index.md) tanpa mengubah API alat. Daftar `H` berisi aturan yang ditinjau orang. Pintu masuknya satu, yaitu `cargo test --workspace --locked`, yang sudah menjadi gerbang merge.

Keputusan turunannya:

| Hal | Keputusan |
| --- | --- |
| Rumah pemeriksaan | Modul `tooling/codegen/src/architecture/` di paket `desktop-codegen`. Tanpa paket baru, tanpa `tooling/validation/`, tanpa `xtask`, tanpa `.cargo/config.toml`. |
| Cara jalan | `cargo run -p desktop-codegen --locked -- architecture check`. Sumber ini tidak punya keluaran, jadi `generate` tidak menulis apa pun. |
| Gerbang | `cargo test --workspace --locked` menjalankan `check` untuk setiap sumber lewat satu uji. Tanpa hook baru. `.githooks/` tidak diubah. CI milik scope 012. |
| Uji lintas crate | `workspace_layout.rs` dan `macos_bridge_ownership.rs` pindah dari `crates/core/tests/` ke `tooling/codegen/tests/`. Uji Core hanya membaca file di `crates/core/`. |
| Larangan dependensi | Dibaca dari teks `Cargo.lock`: sisi antar anggota yang diizinkan, daftar persis dependensi luar langsung tiap anggota, dan kelas paket terlarang di seluruh penutupan transitif (semua paket yang terjangkau dari satu anggota). |
| Pemilik baris aturan | Baris satu anggota di tabel `MEMBERS` dimiliki spec yang memiliki anggota itu. Menambah anggota atau dependensi langsung berarti mengubah baris itu di commit yang sama. |
| Lint | Tidak ada `[workspace.lints]`. Setiap anggota membawa `[lints.rust]` dengan `unsafe_code = "forbid"` dan `missing_docs = "warn"`, dan aturan `A-5` menjaganya. |
| Clippy dan format | Grup bawaan `clippy` dengan `-D warnings` lewat perintah gerbang. Format bawaan `rustfmt`. Tidak ada `rustfmt.toml` atau `clippy.toml`. |
| Bahasa | Komentar kode, rustdoc, nama uji, deskripsi paket, dan pesan alat berbahasa Inggris. Dokumen repo berbahasa Indonesia. |
| Catatan keputusan | Spec di `docs/specs/` menggantikan ADR. Folder `docs/decisions/` tidak dibuat. |
| Keluaran generator | Aturannya tetap `O-1` sampai `O-7` di spec `_root/0003`. Spec ini menambah satu uji yang menjalankan `check` untuk semua sumber. |
| Pemeriksaan yang butuh Mac | Perbandingan byte binding Swift dengan keluaran generator tetap perintah di [spec `macos/0001`](../../macos/0001-sapaan-di-jendela-macos/index.md). Aturan `A-9` menangkap fungsi yang hilang dari binding di host mana pun. |

Nilai dari spec lain yang dipakai di sini, tanpa diputuskan ulang:

| Nilai | Isi | Pemilik |
| --- | --- | --- |
| Pola `Source`, `Plan`, `Problem`, `check`, `generate`, kode keluar, dan bentuk baris keluaran | Bagian **API surface** | Spec `_root/0003` |
| Tabel prosa arah dependensi | Bagian "Aturan arah dependensi" | Spec `_root/0001` |
| Aturan manifest akar, satu lockfile, tanpa `.cargo/config.toml` | Bagian "Aturan untuk manifest root" dan "Invarian" | Spec `_root/0001` |
| R-1 sampai R-9 dan AC-3 | Bagian "Aturan kontrak publik bersama" | [Spec `_root/0002`](../0002-kontrak-sapaan-bersama/index.md) |
| AC-6, AC-7, AC-8 macOS dan perintah "Membuat ulang binding" | Kepemilikan FFI, jembatan yang hanya meneruskan, satu jalur ke Core | Spec `macos/0001` |
| `K-4` | Core dan jembatan tidak membaca konfigurasi | [Spec `_root/0004`](../0004-konfigurasi-publik/index.md) |

## Rationale

Alasan, opsi yang ditimbang, daftar keputusan beserta pilihan keduanya, jawaban atas tiap masukan audit, dan bukti percobaan ada di [rationale.md](rationale.md).

## Rancangan pemeriksaan

### Peta aturan ke pemeriksaan

Setiap aturan pengembangan punya tepat satu pemeriksaan. Yang sudah dikunci uji lain tidak diperiksa dua kali.

| Kelompok aturan | Sumber aturan | Pemeriksaan | Dijalankan oleh |
| --- | --- | --- | --- |
| Manifest akar, anggota, toolchain, lockfile, tabel arah dependensi sama di dua dokumen | Spec `_root/0001` | `tooling/codegen/tests/workspace_layout.rs` (dipindah, isinya tetap) | `cargo test --workspace --locked` |
| Permukaan publik Core, Core tanpa dependensi, lint Core, rustdoc `hello` | Spec `_root/0002` | `crates/core/tests/contract_surface.rs` (tetap) | `cargo test --workspace --locked` |
| Kepemilikan di batas FFI | Spec `macos/0001`, AC-6 | `tooling/codegen/tests/macos_bridge_ownership.rs` (dipindah) | `cargo test --workspace --locked` |
| Sumber bersama sah dan keluarannya sama dengan generator | Spec `_root/0003` dan spec tiap sumber | `check` tiap sumber, lewat `every_source_passes_check_on_the_repo` | `cargo test --workspace --locked` |
| Arah dependensi, lint semua anggota (tumpang tindih dengan dua uji manifest di atas untuk Core dan jembatan), batas Core, jalur sapaan, jalur macOS, dokumen | Spec ini, `A-1` sampai `A-13` | Sumber `architecture` | `cargo test --workspace --locked` dan `architecture check` |
| Binding Swift sama byte demi byte dengan keluaran generator | Spec `macos/0001`, AC-1 | Perintah "Membuat ulang binding", lalu `git diff --exit-code -- bindings/swift` | Penulis perubahan di Mac; CI scope 012 |
| Aturan yang butuh penilaian | Spec ini, `H-1` sampai `H-10` | Tinjauan orang | Penulis dan pemeriksa perubahan |

### Aturan dependensi yang dibaca mesin

Tabel ini adalah bentuk mesin dari tabel prosa di spec `_root/0001`. Isinya ditulis sebagai konstanta `MEMBERS` di alat. Pada akhir milestone 1 isinya tiga baris ini.

| Paket | Anggota yang boleh | Dependensi luar langsung | Kelas terlarang |
| --- | --- | --- | --- |
| `desktop-core` | tidak ada | tidak ada | `binding`, `ui`, `os-sdk` |
| `desktop-macos-ffi` | `desktop-core` | `uniffi` | `ui` |
| `desktop-codegen` | tidak ada | `serde`, `serde_json` | `ui`, `os-sdk` |

Kelas paket terlarang. Sebuah nama paket masuk kelas bila sama dengan satu pola, atau diawali pola itu lalu langsung diikuti `-`, `_`, atau angka. Jadi `uniffi_core`, `gtk4`, dan `windows-sys` cocok, sedangkan nama seperti `gioco` tidak.

| Kelas | Nama di alat | Pola |
| --- | --- | --- |
| Alat binding | `binding` | `uniffi` |
| Pustaka UI | `ui` | `gtk`, `gdk`, `gsk`, `libadwaita`, `glib`, `gio`, `gobject`, `pango`, `cairo`, `graphene`, `windows-reactor` |
| SDK khusus OS | `os-sdk` | `windows`, `winapi`, `objc`, `objc2`, `cocoa`, `core-foundation` |

Kelas terlarang tiap baris mengikuti kolom "Dilarang" tabel prosa apa adanya. `desktop-core` dilarang membawa alat binding, pustaka UI, dan SDK OS. `desktop-codegen` dilarang membawa pustaka UI dan SDK OS; alat binding tidak disebut di barisnya, dan larangan dependensi langsung di luar `serde` dan `serde_json` (`A-3`) sudah menutupnya. `desktop-macos-ffi` tidak dilarang membawa kelas `os-sdk`, karena tabel prosa hanya melarangnya membawa crate aplikasi dan pustaka UI. Sekarang penutupannya memang memuat `windows-sys` dan `windows-link` lewat alat baris perintah UniFFI.

### Aturan yang diperiksa mesin

Pesan ditulis persis, dengan `<...>` diganti nilainya. Setiap pesan diawali ID aturan. ID `A-` di sini milik spec ini. Spec [`_root/0005`](../0005-aset-merek-bersama/index.md) juga memakai `A-1` sampai `A-3` untuk aturan sumber `assets`, jadi dokumen yang menyebut ID ini selalu menyebut spec `_root/0007` juga. Di keluaran alat, baris penutup (`architecture: failed, ...` atau `assets: failed, ...`) menyebut sumbernya. Path di depan pesan adalah path dari akar repo dengan pemisah `/`, juga di Windows.

| ID | Aturan | Sumber aturan | Path masalah | Pesan |
| --- | --- | --- | --- | --- |
| `A-1` | Setiap anggota di `members` punya tepat satu baris di `MEMBERS`, dan setiap baris menunjuk anggota yang ada. Setiap anggota tercatat di `Cargo.lock`. | Spec ini | `Cargo.toml`, atau `Cargo.lock` untuk pesan ketiga | `A-1: member <paket> has no rule in architecture::MEMBERS`; `A-1: rule <paket> names no workspace member`; `A-1: member <paket> is missing from the lockfile, run cargo build` |
| `A-2` | Anggota hanya bergantung langsung pada anggota di kolom "Anggota yang boleh". | Spec `_root/0001`, tabel arah dependensi | `Cargo.lock` | `A-2: <paket> depends on workspace member <anggota>` |
| `A-3` | Dependensi luar langsung satu anggota sama persis dengan kolom "Dependensi luar langsung". | Spec ini; kriteria 32 | Manifest anggota itu, misalnya `crates/core/Cargo.toml` | `A-3: <paket> depends on <nama>, which is not in its allowed list`; `A-3: <paket> no longer depends on <nama>, remove it from its allowed list` |
| `A-4` | Penutupan transitif satu anggota tidak memuat paket dari kelas terlarangnya. | Spec `_root/0001`, kolom "Dilarang" | `Cargo.lock` | `A-4: <paket> reaches <nama> (class <kelas>)` |
| `A-5` | Manifest tiap anggota memuat tabel `[lints.rust]` dengan baris `unsafe_code = "forbid"` dan `missing_docs = "warn"`. Repo tidak memuat `rustfmt.toml`, `.rustfmt.toml`, `clippy.toml`, atau `.clippy.toml`. | Spec ini; spec `_root/0002` R-6 dan R-7 | Manifest anggota, atau file konfigurasi yang ditemukan | `A-5: [lints.rust] must set <baris>`; `A-5: formatter and linter config files are not used` |
| `A-6` | File `.rs` di `crates/core/src/` tidak memuat `target_os`, `target_family`, `cfg(windows)`, atau `cfg(unix)`. File `.rs` di `crates/core/` tidak memuat `../../` atau `CARGO_MANIFEST_DIR`. | Spec `_root/0001` AC-2; peta arsitektur | File yang melanggar | `A-6: Core source must not branch on the OS (<teks>)`; `A-6: Core files must not reach outside crates/core (<teks>)` |
| `A-7` | File di `crates/core/src/`, `crates/macos-ffi/src/`, dan `bindings/` tidak memuat `shared/config` atau `public.json`. | Spec `_root/0004`, `K-4` | File yang melanggar | `A-7: Core and the bridge do not read configuration (<teks>)` |
| `A-8` | Bagian bukan uji dari file `.rs` dan `.swift` di `crates/`, `apps/`, `bindings/`, dan `tooling/` tidak memuat teks sapaan (`Hello`, koma, spasi), kecuali `crates/core/src/greeting.rs`. | Spec `_root/0002` AC-3; spec `macos/0001` AC-8 | File yang melanggar | `A-8: greeting text outside crates/core/src/greeting.rs` |
| `A-9` | Setiap `pub fn` di bagian bukan uji `crates/macos-ffi/src/lib.rs` punya nama yang diekspor Core, tanda tangan yang sama dengan fungsi Core itu, dan badan yang hanya meneruskan. Binding Swift memuat fungsi publik untuk tiap fungsi itu. | Spec `_root/0002` R-8; spec `macos/0001` AC-7 | `crates/macos-ffi/src/lib.rs`, atau file binding untuk pesan keempat | `A-9: bridge exports <nama>, which desktop-core does not export`; `A-9: signature of <nama> differs from desktop-core`; `A-9: <nama> must only forward to desktop_core::<nama>`; `A-9: binding has no public func <namaSwift>, regenerate the binding` |
| `A-10` | Di `apps/macos/`, hanya file di bawah `apps/macos/Interop/` yang mengimpor `DesktopMacosFFI`. Tidak ada file di sana yang mengimpor `desktop_macos_ffiFFI`. | Spec `macos/0001` AC-8; spec `_root/0001`, baris `apps/macos/` | File yang melanggar | `A-10: only apps/macos/Interop imports DesktopMacosFFI`; `A-10: desktop_macos_ffiFFI is imported only by the generated binding` |
| `A-11` | File `Package.swift` tidak memuat `.package(url:` atau `.package(id:`. File `.pbxproj` tidak memuat `XCRemoteSwiftPackageReference`. | Spec ini; kriteria 32 | File yang melanggar | `A-11: remote Swift packages need a spec first` |
| `A-12` | Setiap tautan relatif di file Markdown mengarah ke file atau folder yang ada. | Spec ini; kriteria 26 | File Markdown itu | `A-12: broken link <tujuan> on line <n>` |
| `A-13` | Di setiap `AGENTS.md`: rujukan spec, nama paket sesudah `-p`, dan path file yang disebut masih ada. | Spec ini; kriteria 26 | File `AGENTS.md` itu | `A-13: spec <folder>/<NNNN> does not exist`; `A-13: package <nama> is not a workspace member`; `A-13: path <path> does not exist` |

### Cara tiap aturan membaca repo

Alat hanya membaca teks. Ia tidak menjalankan `cargo`, `git`, atau perintah lain, tidak memakai jaringan, dan tidak menulis apa pun. Karena itu ia berjalan sama di ketiga host.

**Menelusuri folder.** Saat menelusuri, alat melewati folder bernama `.git`, `target`, `.build`, `DerivedData`, `xcuserdata`, `graphify-out`, dan `.claude`, dan tidak mengikuti symlink. Entri `.git` yang berupa file (worktree) juga dilewati. Hasil penelusuran diurutkan menurut path dalam urutan byte, supaya urutan masalah tetap.

**Membaca baris.** File dibaca sebagai teks UTF-8 dan dipecah dengan `str::lines`, jadi akhir baris `\r\n` di checkout Windows tidak mengubah hasil. File yang bukan UTF-8 dilewati oleh aturan yang mencari teks.

**Satu masalah per baris.** Aturan yang mencari teks (`A-6`, `A-7`, `A-8`, `A-10`, `A-11`, `A-12`) memberi satu masalah untuk tiap baris yang melanggar, berurutan menurut nomor baris. Baris yang memuat lebih dari satu pola terlarang memberi satu masalah, untuk pola pertama dalam urutan daftar di tabel aturan. `<teks>` di pesan adalah pola yang ditemukan, ditulis persis seperti di tabel aturan (misalnya `shared/config` atau `target_os`).

**Anggota dan paketnya** (`A-1`, `A-5`). Daftar anggota adalah isi larik `members` di tabel `[workspace]` pada `Cargo.toml` akar. Nama paket satu anggota adalah nilai `name` di tabel `[package]` pada `<anggota>/Cargo.toml`. Tabel dibaca per baris: baris yang diawali `[` membuka tabel, baris kosong dan baris yang diawali `#` dilewati. Cara baca ini sama dengan yang sudah dipakai `workspace_layout.rs`.

**Graf dependensi** (`A-1` sampai `A-4`). `Cargo.lock` dipecah menjadi blok `[[package]]`. Tiap blok memberi `name`, `version`, ada tidaknya baris `source`, dan larik `dependencies`. Satu entri dependensi berbentuk `nama`, `nama versi`, atau `nama versi (sumber)`. Entri tanpa versi menunjuk satu satunya paket dengan nama itu; entri dengan versi menunjuk paket dengan nama dan versi itu. Entri yang tidak menunjuk tepat satu paket memberi masalah `Cargo.lock: A-4: cannot resolve dependency entry <entri>`. Dependensi langsung satu anggota adalah entri di bloknya. Yang namanya nama anggota diperiksa `A-2`, sisanya diperiksa `A-3`. Penutupan transitif adalah semua paket yang terjangkau dari blok anggota itu, tidak termasuk dirinya. `Cargo.lock` mencatat dependensi biasa, build, dan uji untuk semua target dan semua fitur, jadi penutupan ini sama luas atau lebih luas dari `cargo tree --target all --all-features`.

**Bagian bukan uji** (`A-8`, `A-9`). Sebuah file adalah file uji bila salah satu segmen path foldernya bernama `tests` atau `Tests`. Untuk file `.rs` lain, bagian bukan uji adalah teks sebelum baris pertama yang sesudah dipangkas sama dengan `#[cfg(test)]`. Untuk file `.swift` lain, seluruh isinya.

**Teks sapaan** (`A-8`). Teks yang dicari adalah `Hello`, koma, lalu satu spasi. Bagian bukan uji di `tooling/` ikut diperiksa, jadi setiap kode alat yang butuh teks itu menulisnya `concat!("Hello", ", ")`, dan rustdoc tidak menulisnya utuh, supaya alat lulus aturannya sendiri. Sekarang hanya modul `architecture` yang membutuhkannya. Aturan ini hanya menangkap teks literal. Bentuk lain masuk `H-1`.

**Jembatan** (`A-9`). Nama yang diekspor Core adalah segmen terakhir tiap baris `pub use <path>::<nama>;` di `crates/core/src/lib.rs`. Baris `pub use` dengan kurung kurawal, `as`, atau `*` memberi masalah `crates/core/src/lib.rs: A-9: cannot read re-export <baris>`. Untuk tiap baris yang diawali `pub fn <nama>(` di bagian bukan uji `crates/macos-ffi/src/lib.rs`:

* Tanda tangan adalah teks dari `pub fn` sampai sebelum `{` pembuka, dengan setiap deret spasi dan baris baru dijadikan satu spasi dan spasi di ujung dibuang. Bagian bukan uji salah satu file `.rs` di `crates/core/src/`, sesudah dinormalkan dengan cara yang sama, harus memuat tanda tangan itu diikuti tepat ` {`. Jadi `-> String2` di Core tidak cocok dengan `-> String` di jembatan.
* Badan adalah baris sesudah baris yang berakhir `{` sampai sebelum baris pertama yang sama dengan `}`. Sesudah tiap baris dipangkas dan digabung tanpa pemisah, badan harus tepat `desktop_core::<nama>(<argumen>)`, dengan `<argumen>` adalah nama parameter dari tanda tangan, berurutan, dipisah koma dan satu spasi.
* Nama Swift adalah nama itu dalam bentuk camel dengan huruf awal kecil (`get_name` menjadi `getName`). Salah satu file `.swift` di `bindings/swift/Sources/DesktopMacosFFI/` harus memuat baris yang diawali `public func <namaSwift>(`.

Keempat pemeriksaan berjalan berurutan per fungsi (nama, tanda tangan, badan, binding), dan pemeriksaan pertama yang gagal adalah satu satunya masalah untuk fungsi itu.

**Impor Swift** (`A-10`). Sebuah baris mengimpor modul `X` bila, sesudah dipangkas, baris itu berakhir dengan `import X`. File yang diperiksa adalah file `.swift` di bawah `apps/macos/`.

**Tautan Markdown** (`A-12`). File yang diperiksa adalah setiap file `.md` di repo, kecuali di bawah `docs/scope/coordination/`. Blok kode berpagar dibuang (baris yang sesudah dipangkas diawali tiga backtick membuka atau menutup blok). Di baris lain, teks di antara sepasang backtick dibuang. Tautan adalah teks di antara `](` dan `)` berikutnya yang tidak memuat spasi. Jadi tautan berjudul seperti `(path "judul")` tidak terbaca dan dilewati. Tautan juga dilewati bila diawali `#`, `<`, atau `/`, atau diawali skema (huruf, lalu huruf, angka, `+`, `.`, atau `-`, lalu `:`). File yang belum dilacak git ikut diperiksa, karena alat tidak memanggil `git`; catatan sementara dengan tautan rusak di dalam repo ikut menggagalkan gerbang. Bagian sesudah `#` dibuang, `%XX` diubah menjadi karakternya, lalu path dibaca relatif terhadap folder file itu.

**Rujukan di `AGENTS.md`** (`A-13`). Berlaku untuk setiap file bernama `AGENTS.md`:

* Rujukan spec adalah teks berbentuk backtick, lalu `_root`, `macos`, `windows`, atau `linux`, lalu `/`, lalu empat angka, lalu backtick. Folder `docs/specs/<folder>/` harus memuat entri yang namanya diawali empat angka itu dan `-`.
* Nama paket adalah deret huruf kecil, angka, dan `-` yang mengikuti teks spasi, `-p`, spasi. Nama kosong dilewati. Nama itu harus nama paket satu anggota.
* Calon path adalah isi tiap pasang backtick di luar blok kode, dan tiap kata yang dipisah spasi di dalam blok kode. Calon diperiksa bila hanya berisi huruf, angka, `_`, `.`, `/`, dan `-`, memuat `/`, tidak diawali `/`, dan segmen terakhirnya memuat `.`. Calon yang sama dengan `.cargo/config.toml` atau `.cargo/config` dilewati, karena file itu disebut sebagai larangan. Calon harus ada relatif terhadap folder `AGENTS.md` itu atau relatif terhadap akar repo.

Folder dan path berisi `<`, `*`, atau spasi tidak diperiksa, jadi `AGENTS.md` tetap boleh menyebut lokasi rencana seperti `docs/specs/windows/`.

### Bentuk alat

Nilai di kolom kedua yang datang dari spec `_root/0003` tidak diputuskan ulang di sini.

| Hal | Nilai |
| --- | --- |
| Modul baru | `tooling/codegen/src/architecture/mod.rs`. Pembagian ke modul privat lain di folder yang sama bebas. |
| Pendaftaran | Di `tooling/codegen/src/lib.rs`: baris `pub mod architecture;` di antara baris `pub mod` lain, dan entri `architecture::SOURCE` di `SOURCES`. Pada 2026-11-15 `SOURCES` sudah memuat `tokens`, `config`, dan `localization`, dan `assets` menyusul sebelum spec ini dibangun. Letak entri di larik bebas; tambahkan di akhir supaya baris `sources:` yang sudah ada tidak berubah urutannya. Bila larik melewati lebar baris, `rustfmt` memecahnya ke beberapa baris, dan itu tetap dihitung sebagai perubahan pendaftaran. `main.rs` tidak diubah, karena subperintah dan baris `sources:` sudah mengikuti `SOURCES`. |
| Permukaan publik | Lihat blok kode di bawah. Setiap item publik punya rustdoc berbahasa Inggris. |
| `plan` | Menerima akar repo. Menjalankan semua aturan dan mengembalikan semua masalah, bukan hanya yang pertama, terurut menurut angka ID aturan sebagai bilangan (`A-2` sebelum `A-10`), lalu path dalam urutan byte, lalu nomor baris. Bila tidak ada masalah, mengembalikan `Plan` dengan `outputs` kosong dan `summary` berisi `<r> rules, <m> members`, dengan `<r>` panjang `RULES` dan `<m>` panjang `MEMBERS`. |
| File yang tidak terbaca | `Cargo.toml` akar, `Cargo.lock`, atau manifest anggota yang tidak terbaca memberi `<path>: cannot read: <kesalahan OS>`, dan aturan yang butuh file itu dilewati. Aturan lain tetap berjalan. |
| Uji | `tooling/codegen/tests/architecture_contract.rs`, dibuka komentar dokumen `//!`, hanya memakai API publik `desktop_codegen` dan pustaka standar. |
| Salinan repo di uji | Folder di bawah `env!("CARGO_TARGET_TMPDIR")` bernama `architecture_contract_<nama uji>`, dihapus dan dibuat ulang di awal uji. Isinya salinan pohon repo dari `repo_root()` dengan aturan lewati yang sama seperti alat. |
| Dependensi | Tidak ada yang baru. `serde` dan `serde_json` tidak dipakai modul ini. |

```rust
// tooling/codegen/src/architecture/mod.rs
pub const SOURCE: Source;                 // name: "architecture"
pub const RULES: &[&str];                 // ID aturan yang sudah dipasang, berurutan
pub enum Class { Binding, Ui, OsSdk }     // Debug, Clone, Copy, PartialEq, Eq
impl Class {
    pub fn name(self) -> &'static str;                 // "binding", "ui", "os-sdk"
    pub fn patterns(self) -> &'static [&'static str];  // kolom Pola
    pub fn matches(self, package: &str) -> bool;
}
pub struct MemberRule {                   // Debug, Clone, Copy, PartialEq, Eq
    pub package: &'static str,
    pub members: &'static [&'static str],
    pub external: &'static [&'static str],
    pub forbidden: &'static [Class],
}
pub const MEMBERS: &[MemberRule];         // tiga baris tabel di atas, dalam urutan itu
```

`RULES` hanya memuat aturan yang sudah dipasang, jadi angka di baris sukses jujur di tiap milestone: `4 rules` pada akhir milestone 1, `11 rules` pada akhir milestone 2, dan `13 rules` pada akhir milestone 3.

### Uji kontrak

| Uji | Yang dibuktikan | Langkah |
| --- | --- | --- |
| `every_source_passes_check_on_the_repo` | Untuk setiap entri `SOURCES`, `check(&repo_root(), sumber)` lulus dan barisnya diawali `<nama>: ok, `. Baris sumber `architecture` berakhir `0 outputs up to date`. | 1 |
| `rejects_broken_repos` | Tiap kasus `N` di tabel berikut, pada salinan repo, membuat `plan` mengembalikan paling sedikit satu masalah yang pesannya diawali ID di kolom Aturan. Masalah lain boleh ada. | 2, 4, 5 |
| `accepts_allowed_shapes` | Tiap kasus `P` di tabel berikut membuat `plan` lulus. | 2, 4, 5 |

**Kasus yang ditolak.** Tiap kasus berangkat dari salinan repo yang utuh, mengubahnya seperti kolom Perubahan, lalu memulihkannya sebelum kasus berikutnya. "Tambah sisi `X` ke `Y`" berarti menambah entri `X` ke larik `dependencies` blok paket `Y` di `Cargo.lock` (larik dibuat bila belum ada).

| No. | Perubahan pada salinan | Aturan | Langkah |
| --- | --- | --- | --- |
| `N-1` | Tambah `"crates/extra"` ke `members`, buat `crates/extra/Cargo.toml` berisi paket `desktop-extra`, dan tambah blok paket `desktop-extra` versi `0.1.0` ke `Cargo.lock` | `A-1` | 2 |
| `N-2` | Tambah sisi `desktop-macos-ffi` ke `desktop-core` | `A-2` | 2 |
| `N-3` | Tambah sisi `desktop-codegen` ke `desktop-macos-ffi` | `A-2` | 2 |
| `N-4` | Tambah sisi `itoa` ke `desktop-core` | `A-3` | 2 |
| `N-5` | Hapus entri `serde_json` dari blok `desktop-codegen` | `A-3` | 2 |
| `N-6` | Tambah blok paket `gtk4` versi `0.0.0` ke `Cargo.lock`, lalu tambah sisi `gtk4` ke `serde_json`. Pesan harus menyebut `desktop-codegen`, `gtk4`, dan `class ui`. | `A-4` | 2 |
| `N-7` | Hapus baris `unsafe_code = "forbid"` dari `crates/macos-ffi/Cargo.toml` | `A-5` | 4 |
| `N-8` | Buat file kosong `rustfmt.toml` di akar | `A-5` | 4 |
| `N-9` | Tambah baris `#[cfg(target_os = "macos")]` di akhir `crates/core/src/greeting.rs` | `A-6` | 4 |
| `N-10` | Buat `crates/core/tests/outside.rs` berisi `const X: &str = include_str!("../../../Cargo.toml");` | `A-6` | 4 |
| `N-11` | Tambah baris `// shared/config/public.json` di akhir `crates/macos-ffi/src/lib.rs` | `A-7` | 4 |
| `N-12` | Buat `apps/macos/Presentation/Local.swift` berisi `let text = "Hello, " + name` | `A-8` | 4 |
| `N-13` | Di `crates/macos-ffi/src/lib.rs`, ganti `desktop_core::hello(name)` pertama dengan `format!("Hello, {name}!")` | `A-8` dan `A-9` | 4 |
| `N-14` | Sisipkan fungsi `pub fn extra() -> String {`, `String::new()`, `}` sebelum baris `#[cfg(test)]` di `crates/macos-ffi/src/lib.rs` | `A-9` | 4 |
| `N-15` | Ganti `desktop_core::hello(name)` pertama dengan `desktop_core::hello(name.trim().to_string())` | `A-9` | 4 |
| `N-16` | Ganti `pub fn hello(name: String) -> String {` di `crates/macos-ffi/src/lib.rs` dengan `pub fn hello(name: &str) -> String {` | `A-9` | 4 |
| `N-17` | Di file binding `desktop_macos_ffi.swift`, ganti `public func hello(` dengan `public func hallo(` | `A-9` | 4 |
| `N-18` | Buat `apps/macos/Presentation/Direct.swift` berisi `import DesktopMacosFFI` | `A-10` | 4 |
| `N-19` | Tambah baris `// .package(url: "https://example.com/x.git", from: "1.0.0")` di akhir `apps/macos/Package.swift` | `A-11` | 4 |
| `N-20` | Tambah baris `[rusak](docs/tidak-ada.md)` di akhir `README.md` | `A-12` | 5 |
| `N-21` | Tambah baris berisi `_root/9999` di dalam backtick di akhir `AGENTS.md` | `A-13` | 5 |
| `N-22` | Tambah baris berisi `cargo test -p desktop-nope` di akhir `AGENTS.md` | `A-13` | 5 |
| `N-23` | Tambah baris berisi `crates/core/tests/tidak_ada.rs` di dalam backtick di akhir `crates/core/AGENTS.md` | `A-13` | 5 |

**Kasus yang diterima.**

| No. | Keadaan salinan | Langkah |
| --- | --- | --- |
| `P-1` | Salinan repo tanpa perubahan | 2 |
| `P-2` | Tambah baris `// shared/config` di akhir `tooling/codegen/src/lib.rs` (di luar cakupan `A-7`) | 4 |
| `P-3` | Buat `apps/macos/Tests/PresentationTests/Extra.swift` berisi `let text = "Hello, Ayu!"` (file uji boleh memuat teks sapaan) | 4 |
| `P-4` | Tambah baris berisi tautan rusak `[x](tidak-ada.md)` di dalam sepasang backtick di akhir `README.md` | 5 |
| `P-5` | Tambah baris berisi `docs/specs/windows/` dan `.cargo/config.toml`, masing masing di dalam backtick, di akhir `AGENTS.md` | 5 |

### Aturan yang diperiksa manusia

Aturan ini butuh penilaian, jadi tidak ada pemeriksaan otomatisnya. Penulis perubahan memeriksanya sebelum meminta merge, dan pemeriksa (`/check verify` atau reviewer) memeriksanya lagi.

| ID | Aturan | Sumber aturan | Kapan ditinjau | Bukti |
| --- | --- | --- | --- | --- |
| `H-1` | Perilaku bersama tinggal di Core. Aplikasi dan jembatan mengirim masukan apa adanya dan menampilkan hasil apa adanya, tanpa menyalin logika Core dalam bentuk apa pun, termasuk yang tidak tertangkap `A-8`. | Peta arsitektur, tabel "Pemilik dan arah dependensi"; spec `_root/0002` R-8 | Setiap perubahan di `apps/` dan `crates/macos-ffi/` | Catatan pemeriksa di `verify.md` atau di review |
| `H-2` | API publik Core, anggota baru, dan dependensi langsung baru punya spec yang diterima sebelum kodenya ditulis. | Spec `_root/0002` R-2; spec ini `A-3` | Setiap perubahan `crates/core/src/lib.rs`, `members`, atau `MEMBERS` | Tautan ke spec di pesan commit atau di laporan |
| `H-3` | Versi `desktop-core` naik sesuai R-5, dan tabel "Riwayat kontrak" mendapat barisnya. | Spec `_root/0002` R-5 | Setiap perubahan permukaan publik atau hasil fungsi Core | Baris baru di "Riwayat kontrak" |
| `H-4` | Vektor kontrak di spec sama dengan yang diuji di `hello_contract.rs` dan di uji Swift. | Spec `_root/0002`, bagian "Vektor kontrak" | Setiap perubahan vektor di spec atau di uji | Perbandingan tabel dengan berkas uji |
| `H-5` | Tabel prosa arah dependensi di spec `_root/0001` dan di peta arsitektur bermakna sama dengan `MEMBERS` dan kelas di spec ini. | Spec `_root/0001`; spec ini | Setiap perubahan salah satu dari ketiganya | Ketiganya berubah di commit yang sama |
| `H-6` | Komentar kode, rustdoc, nama uji, deskripsi paket, dan pesan alat berbahasa Inggris. Dokumen di `docs/`, `AGENTS.md`, dan `README.md` berbahasa Indonesia. | Spec ini | Setiap perubahan | Dibaca saat review |
| `H-7` | Setiap entri `.graphifyignore` punya alasan yang tercatat di spec pemilik file itu. | Spec `_root/0001`, "Kasus gagal yang sudah diputuskan" | Setiap perubahan `.graphifyignore` | Kalimat alasan di spec pemilik |
| `H-8` | Tidak ada kode, dependensi, folder, atau skrip untuk fitur di luar fase fondasi. | Kriteria 32; peta arsitektur, "Batas fase fondasi" | Setiap spec baru dan setiap anggota atau modul baru | AC kriteria 32 di spec itu |
| `H-9` | Setiap `#[allow]` dan `#[expect]` membawa `reason` yang menyebut spec pemilik alasannya. | Spec ini; contoh di `crates/core/src/greeting.rs` | Setiap perubahan kode Rust | Dibaca saat review |
| `H-10` | Interaksi native yang tidak tercakup uji otomatis (fokus keyboard, pembaca layar, skala teks, tampilan terang dan gelap) ditulis sebagai langkah manual `owner: user` di spec OS. | [Penerimaan dan bukti fondasi](../../../quality/README.md), "Pemeriksaan yang direncanakan" | Setiap spec yang mengubah jendela | Bagian `### UI / manual` di `verify.md` spec itu |

### Kapan pemeriksaan berjalan

| Kapan | Siapa | Perintah |
| --- | --- | --- |
| Sebelum commit yang mengubah kode, manifest, `Cargo.lock`, sumber bersama, atau Markdown | Penulis perubahan | `cargo test --workspace --locked`. Untuk perubahan Markdown saja, `cargo run -p desktop-codegen --locked -- architecture check` cukup. |
| Sebelum melapor selesai dan sebelum merge ke `main` | Penulis perubahan dan lead | `cargo build --workspace --locked`, `cargo test --workspace --locked`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings` |
| Saat `crates/core/` atau `crates/macos-ffi/` berubah, di Mac | Penulis perubahan | Perintah di bagian "Membuat ulang binding" spec `macos/0001`, lalu perintah `swift test` di AC-2 spec itu |
| Di CI | Scope 012 | Perintah yang sama di tiga runner, ditambah perintah binding di runner macOS |

Hook `pre-commit` tetap hanya memperbarui Graphify. `test-preferences.json` tidak diubah: `"gate": null` benar, karena field itu hanya untuk proyek tanpa penguji, dan di sini gerbangnya adalah suite `cargo test`.

### Cara menambah atau mengubah aturan

* **Anggota baru.** Spec yang menambah anggota menulis barisnya untuk `MEMBERS` (anggota yang boleh, dependensi luar langsung, kelas terlarang) dan baris prosanya untuk tabel spec `_root/0001`. Developer menambah keduanya, baris `members`, dan `Cargo.lock` di commit yang sama. Tanpa baris `MEMBERS`, `A-1` gagal.
* **Dependensi langsung baru.** Spec pemilik anggota menyebut nama paketnya dan alasannya. Developer menambah nama itu ke kolom `external` baris anggota di commit yang sama dengan manifest. Bila penutupannya memuat paket kelas terlarang, pilih paket lain atau perbarui spec ini.
* **Dependensi pertama Core.** Spec `_root` yang memberi Core dependensi pertamanya menambah nama itu ke baris `desktop-core`, menghapus uji `manifest_declares_no_dependencies_of_any_kind` di `contract_surface.rs`, dan mengganti bukti "tepat satu baris" dengan `architecture check`. Sejak itu `A-3` dan `A-4` yang menjaga Core.
* **Kelas atau pola baru.** Spec Windows dan Linux menambah nama crate UI dan SDK yang mereka pakai ke pola kelas lewat pembaruan spec ini.
* **Aturan `A` baru.** Lewat pembaruan spec ini: satu baris di tabel aturan, satu ID di `RULES`, paling sedikit satu kasus `N`, dan angka di AC-1.
* **Pengecualian lint.** Anggota yang tidak bisa memakai `unsafe_code = "forbid"` membutuhkan pembaruan spec ini lebih dulu. Sekarang tidak ada pengecualian, dan alat tidak membawa mekanisme pengecualian.

### Value sourcing

| Aksi | Nilai yang dihasilkan atau ditampilkan | Sumber |
| --- | --- | --- |
| `A-1` | Daftar anggota dan nama paketnya | `members` di `Cargo.toml` akar; `name` di `[package]` tiap `<anggota>/Cargo.toml` |
| `A-1` sampai `A-4` | Baris aturan per paket | Konstanta `MEMBERS`, ditulis dari tabel **Aturan dependensi yang dibaca mesin** |
| `A-2` sampai `A-4` | Dependensi langsung dan penutupan transitif | Blok `[[package]]` di `Cargo.lock` |
| `A-4` | Kelas satu paket | `Class::patterns`, ditulis dari tabel kelas |
| `A-9` | Nama yang diekspor Core, tanda tangan fungsi Core | `crates/core/src/lib.rs`; file `.rs` di `crates/core/src/` |
| `A-9` | Nama fungsi di binding | File `.swift` di `bindings/swift/Sources/DesktopMacosFFI/` |
| `A-13` | Nama paket anggota | Sama dengan `A-1` |
| Baris sukses | `<r> rules, <m> members` | Panjang `RULES` dan `MEMBERS` |
| Baris masalah | `<path>: <ID>: <pesan>` | Path dari akar repo yang sedang dibaca; pesan dari tabel aturan |
| Baris `architecture: ok, ...` dan `architecture: failed, <n> problems` | Susunan baris dan kode keluar | `check`, `generate`, dan `main` milik spec `_root/0003` |

### Invarian

* Alat tidak menulis ke repo dan tidak menjalankan proses lain. `architecture generate` selalu menulis nol file.
* `desktop-codegen` tidak bergantung pada anggota mana pun, dan tidak ada anggota yang bergantung padanya. Aturan `A-2` memeriksa hal itu untuk alat ini juga.
* Uji di `crates/core/` hanya membaca file di `crates/core/`. Uji yang membaca file lain tinggal di `tooling/codegen/tests/`.
* Setiap sumber di `SOURCES` ikut diperiksa `cargo test --workspace --locked`, tanpa perlu didaftarkan di tempat lain.
* Satu aturan, satu pemeriksaan. Yang dikunci `workspace_layout.rs`, `contract_surface.rs`, dan `macos_bridge_ownership.rs` tidak diulang di sumber `architecture`, dengan satu pengecualian: `A-5` memeriksa lint setiap anggota, termasuk anggota yang belum punya uji manifest sendiri seperti `desktop-codegen` dan anggota yang menyusul. Untuk Core dan jembatan, `A-5` tumpang tindih dengan `manifest_forbids_unsafe_code_and_warns_on_missing_docs` dan `bridge_manifest_forbids_unsafe_code`. Kedua uji itu tetap, karena keduanya bukti AC spec pemiliknya (spec `_root/0002` AC-6 dan AC-7, spec `macos/0001` AC-6).

### Keamanan

Fitur ini tidak menambah rahasia, kredensial, variabel lingkungan, atau akses jaringan. Alat hanya membaca file di bawah akar repo dan tidak menerima path dari argumen. Larangan dependensi langsung di luar daftar (`A-3`) dan larangan paket Swift jarak jauh (`A-11`) membuat setiap paket pihak ketiga baru terlihat di review sebelum masuk.

### Kasus gagal yang sudah diputuskan

| Kejadian | Perilaku yang diharapkan |
| --- | --- |
| Dependensi baru ditambah ke manifest tanpa baris `MEMBERS` | `A-3` gagal dan menyebut nama paketnya. Tambah nama itu lewat spec pemilik anggota. |
| Kenaikan versi paket membawa paket kelas terlarang lewat jalur transitif | `A-4` gagal dan menyebut paketnya. Tahan versi lama, pilih paket lain, atau perbarui spec ini. |
| Fungsi baru di jembatan tanpa binding yang dibuat ulang | `A-9` gagal dengan `regenerate the binding`. Jalankan perintah "Membuat ulang binding" di Mac. |
| Dokumen menautkan file yang belum ada di branch itu | `A-12` gagal. Tautan ditulis sesudah filenya ada, atau keduanya masuk commit yang sama. |
| Spec atau file yang disebut `AGENTS.md` dipindah | `A-13` gagal. Perbarui `AGENTS.md` di commit yang sama. |
| `Cargo.lock` usang terhadap manifest | Perintah `cargo` dengan `--locked` gagal lebih dulu, sebelum alat berjalan. Perbarui lockfile menurut spec `_root/0001`. |
| Sumber lain di `SOURCES` gagal `check` | `every_source_passes_check_on_the_repo` gagal dan mencetak masalah sumber itu. Obatnya ada di spec sumber itu. |
| Aturan menolak hal yang benar | Bukan alasan untuk melewati gerbang. Perbarui spec ini lebih dulu, lalu alatnya. |
| Alat gagal dibangun atau memberi hasil lain di host Windows atau Linux | Temuan untuk AC-11. Alat diperbaiki; aturan tidak dilonggarkan untuk host itu. |

### Skenario uji kritis

* Jalur utama: repo saat ini lulus semua aturan dan semua sumber, memverifikasi **AC-1**, **AC-3**.
* Dependensi terlarang: sisi antar anggota, dependensi langsung di luar daftar, dan paket kelas terlarang lewat jalur transitif ditolak, memverifikasi **AC-2**.
* Keluaran menyimpang: skema yang disunting tangan menggagalkan uji gerbang, memverifikasi **AC-3**.
* Batas Core dan jalur macOS: cabang OS di Core, konfigurasi di jembatan, teks sapaan di luar Core, jembatan yang mengubah masukan, dan impor di luar `Interop` ditolak, memverifikasi **AC-4**, **AC-7**.
* Lint dan format: manifest tanpa `unsafe_code = "forbid"` dan file `rustfmt.toml` ditolak, memverifikasi **AC-8**.
* Panduan: tautan rusak dan rujukan usang di `AGENTS.md` ditolak, sedangkan lokasi rencana dan contoh di dalam backtick diterima, memverifikasi **AC-5**.
* Cakupan: hanya file di tabel milestone yang berubah dan alat tidak mendapat dependensi, memverifikasi **AC-10**.
* Akses: tidak ada peran atau izin di fitur ini. Yang ditolak adalah repo yang melanggar aturan, memverifikasi **AC-2**, **AC-4**.

## Perubahan dokumen

Langkah 5 Build plan mengubah lima dokumen berikut. Bagian lain dokumen itu tidak diubah. Kelima file biasanya terlarang bagi sub sesi, jadi brief developer untuk milestone 3 perlu izin eksplisit. Bila salah satu kalimat lama sudah berubah saat langkah ini dikerjakan, pertahankan maksud teks baru dan catat bedanya di `verify.md`.

**`AGENTS.md` di akar**, bagian `# Aturan pengembangan`:

1. Di blok kode `## Commands`, sesudah baris `cargo clippy --workspace --all-targets --locked -- -D warnings`, tambahkan satu baris kosong lalu dua baris ini:

   ```bash
   # Pemeriksaan arsitektur saja, satu baris per pelanggaran. cargo test --workspace sudah menjalankannya
   cargo run -p desktop-codegen --locked -- architecture check
   ```

2. Di `## Rules`, ganti butir yang diawali `- **Uji mengunci aturan ini.**` dengan tujuh butir ini:

   ```markdown
   - **Uji mengunci aturan ini.** `tooling/codegen/tests/workspace_layout.rs` gagal bila aturan manifest di atas dilanggar, atau bila tabel arah dependensi di `docs/architecture/README.md` berbeda dari tabel di spec `_root/0001`. Ubah kedua tabel itu bersama sama. `tooling/codegen/tests/macos_bridge_ownership.rs` mengunci kepemilikan di batas FFI (spec `macos/0001`, AC-6).
   - **Pemeriksaan arsitektur.** Aturan `A-1` sampai `A-13` di spec `_root/0007` diperiksa sumber `architecture` di `desktop-codegen`. Pelanggaran menggagalkan `cargo test --workspace --locked`. Aturan baru masuk lewat pembaruan spec itu, bersama kasus ujinya.
   - **Anggota dan dependensi baru.** Sisi antar anggota dan dependensi langsung tiap anggota tercatat di `tooling/codegen/src/architecture/mod.rs`. Menambah satu butuh spec pemilik anggota itu lebih dulu, lalu barisnya diubah di commit yang sama dengan manifest (spec `_root/0007`, bagian "Cara menambah atau mengubah aturan").
   - **Keluaran generator.** File turunan tidak disunting tangan. Sunting sumbernya, jalankan `cargo run -p desktop-codegen --locked -- <sumber> generate`, lalu commit sumber bersama keluarannya (spec `_root/0003`, O-1 sampai O-7).
   - **Lint dan format.** Setiap anggota membawa `[lints.rust]` dengan `unsafe_code = "forbid"` dan `missing_docs = "warn"`. Tidak ada `[workspace.lints]`, `rustfmt.toml`, atau `clippy.toml`. Gaya dan lint mengikuti bawaan toolchain yang dipin (spec `_root/0007`, `A-5`).
   - **Bahasa.** Komentar kode, rustdoc, nama uji, deskripsi paket, dan pesan alat berbahasa Inggris. Dokumen di `docs/`, `AGENTS.md`, dan `README.md` berbahasa Indonesia (spec `_root/0007`, `H-6`).
   - **Keputusan.** Keputusan arsitektur dicatat sebagai spec di `docs/specs/`, dengan alasannya di `rationale.md` spec itu. Repo ini tidak memakai folder ADR.
   ```

3. Sebelum heading `## Agent skills`, tambahkan bagian ini:

   ```markdown
   ## Tinjauan manusia

   Aturan berikut butuh penilaian orang. Penulis perubahan memeriksanya sebelum meminta merge, dan pemeriksa membacanya lagi. Rinciannya ada di spec `_root/0007` bagian "Aturan yang diperiksa manusia".

   - **H-1** Perilaku bersama tinggal di Core. Aplikasi dan jembatan tidak menyalin logika Core dalam bentuk apa pun.
   - **H-2** API publik Core, anggota baru, dan dependensi langsung baru punya spec yang diterima sebelum kodenya ditulis.
   - **H-3** Versi `desktop-core` dan tabel "Riwayat kontrak" ikut berubah saat kontrak berubah.
   - **H-4** Vektor kontrak di spec sama dengan yang diuji di Rust dan di Swift.
   - **H-5** Tabel arah dependensi di spec `_root/0001` dan di peta arsitektur bermakna sama dengan tabel di alat.
   - **H-6** Kode berbahasa Inggris, dokumen berbahasa Indonesia.
   - **H-7** Setiap entri `.graphifyignore` punya alasan di spec pemilik file itu.
   - **H-8** Tidak ada kode, dependensi, folder, atau skrip untuk fitur di luar fase fondasi.
   - **H-9** Setiap `#[allow]` dan `#[expect]` membawa `reason` yang menyebut spec.
   - **H-10** Interaksi native yang tidak tercakup uji otomatis ditulis sebagai langkah manual di spec OS.
   ```

**`crates/core/AGENTS.md`**:

1. Di tabel `## Key files`, hapus baris `tests/workspace_layout.rs` dan baris `tests/macos_bridge_ownership.rs`.
2. Di `## Conventions`, tambahkan butir terakhir:

   ```markdown
   - File `.rs` di crate ini tidak bercabang per OS dan tidak membaca path di luar `crates/core/`. Uji yang membaca file lain tinggal di `tooling/codegen/tests/` (spec `_root/0007`, `A-6`).
   ```

3. Di `## Gotchas`, ganti butir pertama (yang diawali `` - `tests/contract_surface.rs`, ``) dengan:

   ```markdown
   - `tests/contract_surface.rs` membaca `Cargo.toml`, `src/lib.rs`, dan `src/greeting.rs` crate ini sebagai teks. Mengubah baris `mod` atau `pub use` di `lib.rs`, tabel lint, atau rujukan spec di dokumen crate bisa membuat `cargo test -p desktop-core` gagal walau kode Core benar. Perbarui spec lebih dulu, lalu ujinya.
   ```

4. Di `## Gotchas`, ganti butir terakhir (yang diawali `- Rustdoc dan deskripsi paket Core berbahasa Inggris untuk sementara`) dengan:

   ```markdown
   - Rustdoc, komentar, dan deskripsi paket berbahasa Inggris (spec `_root/0007`, `H-6`).
   ```

5. Di `## Related specs`, tambahkan butir:

   ```markdown
   - [0007. Aturan pengembangan dan pemeriksaan arsitektur](../../docs/specs/_root/0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md)
   ```

**`README.md` di akar**:

1. Ganti dua kalimat "Repositori saat ini berisi dokumentasi perencanaan. Semua fitur berstatus `planned`." dengan:

   ```markdown
   Workspace Rust, Rust Core, dan aplikasi macOS sudah ada di repositori ini. Windows dan Linux menunggu host masing masing. Status tiap fitur tercatat di indeks itu.
   ```

2. Ganti kalimat "Folder spec akan terisi saat pekerjaan desain dimulai." dengan:

   ```markdown
   Spec yang sudah ditulis ada di folder itu.
   ```

3. Di tabel "Rujukan bersama", sebelum baris terakhir, tambahkan:

   ```markdown
   | Aturan pengembangan, perintah uji, dan pemeriksaan arsitektur | [Aturan pengembangan](AGENTS.md) |
   ```

**`docs/architecture/README.md`**:

1. Di tabel "Tata letak yang dituju", ganti baris `tooling/codegen/`, `tooling/validation/` dengan:

   ```markdown
   | `tooling/codegen/` | Generator, validasi sumber bersama, dan pemeriksaan arsitektur dalam satu paket. |
   ```

2. Di akhir subbagian "Aturan arah dependensi", sesudah paragraf yang diakhiri tautan ke spec 0001, tambahkan paragraf:

   ```markdown
   Aturan di tabel ini diperiksa otomatis oleh `cargo run -p desktop-codegen --locked -- architecture check` dan oleh `cargo test --workspace --locked`. Daftar aturan, kelas paket terlarang, dan aturan yang ditinjau orang ada di spec 0007.
   ```

   Jadikan kata `spec 0007` tautan ke `docs/specs/_root/0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md`; dari dokumen itu path relatifnya diawali `../specs/_root/`.

3. Di bagian "Aturan pengembangan yang akan dicatat", ganti kalimat "Saat scaffold sudah ada, `AGENTS.md` di root akan merujuk aturan pemilik, dependensi, kontrak, dan tes." dengan:

   ```markdown
   `AGENTS.md` di root merujuk aturan pemilik, dependensi, kontrak, dan tes, beserta perintah yang memeriksanya.
   ```

4. Di bagian yang sama, ganti kalimat "Keputusan arsitektur penting beserta alasannya akan dicatat sebagai ADR di `docs/decisions/`." dengan:

   ```markdown
   Keputusan arsitektur penting dicatat sebagai spec di `docs/specs/`, dengan alasannya di `rationale.md` spec itu. Repo ini tidak memakai folder ADR terpisah.
   ```

Judul bagian "Aturan pengembangan yang akan dicatat" dan tabel "Aturan arah dependensi" tidak diubah, karena spec `_root/0001` dan uji `workspace_layout.rs` merujuk keduanya.

**`docs/contracts/README.md`**, bagian "Eksekusi dan uji":

1. Ganti kalimat "Core mengutamakan safe Rust, dependensi minimal dan kepemilikan eksplisit." dengan:

   ```markdown
   Core hanya memakai safe Rust, dengan dependensi minimal dan kepemilikan eksplisit.
   ```

2. Ganti kalimat "Jika bagian `unsafe` diperlukan, invariannya perlu tercatat." dengan:

   ```markdown
   Kode `unsafe` di Core dan di jembatan macOS ditolak saat build lewat `unsafe_code = "forbid"`. Membukanya butuh pembaruan spec `_root` lebih dulu.
   ```

## File yang boleh diubah tiap milestone

| Milestone | File |
| --- | --- |
| 1 | `tooling/codegen/src/lib.rs` (hanya baris `pub mod architecture;`, larik `SOURCES`, dan, bila perlu, kalimat dokumen crate yang menyebut sumber), file di bawah `tooling/codegen/src/architecture/`, `tooling/codegen/tests/architecture_contract.rs`, `verify.md` spec ini, baris `**Status**` di `index.md` spec ini, dan empat file `graphify-out/` dari hook |
| 2 | Pemindahan `crates/core/tests/workspace_layout.rs` dan `crates/core/tests/macos_bridge_ownership.rs` ke `tooling/codegen/tests/`, file di bawah `tooling/codegen/src/architecture/`, `tooling/codegen/tests/architecture_contract.rs`, `verify.md` spec ini, baris `**Status**` di `index.md` spec ini, dan empat file `graphify-out/` dari hook |
| 3 | `AGENTS.md`, `crates/core/AGENTS.md`, `README.md`, `docs/architecture/README.md`, `docs/contracts/README.md`, file di bawah `tooling/codegen/src/architecture/`, `tooling/codegen/tests/architecture_contract.rs`, `verify.md` spec ini, baris `**Status**` di `index.md` spec ini, dan empat file `graphify-out/` dari hook |
| 4 | `verify.md` spec ini, baris `**Status**` di `index.md` spec ini, dan empat file `graphify-out/` dari hook |

## Build plan

Pendekatan build proyek adalah Tracer Bullet. Untuk fitur ini benang tipisnya adalah satu jalur dari `Cargo.lock`, lewat sumber `architecture`, sampai uji yang berjalan di gerbang. Sesudah itu aturan kode dan uji lintas crate menyusul, lalu panduan dan dokumen mengikuti. Satu langkah menjadi satu commit dengan subject `type(scope): spec _root/0007 step <n>, ...`, kecuali langkah 2, 4, dan 5 yang boleh memakai commit kedua untuk `verify.md` dengan subject yang sama. Milestone 1 butuh milestone 1 spec `_root/0003` di `main`, dan `tooling/codegen/src/lib.rs` hanya boleh punya satu penulis per wave.

**Milestone 1: Sumber `architecture` dan aturan dependensi** (langkah 1 dan 2)

1. Buat modul `tooling/codegen/src/architecture/` dengan `SOURCE`, `RULES` berisi `A-1` sampai `A-4`, `Class`, `MemberRule`, dan `MEMBERS` tiga baris, lalu daftarkan di `lib.rs`. Buat `tooling/codegen/tests/architecture_contract.rs` dengan uji `every_source_passes_check_on_the_repo`. Sebelum commit, `cargo run -p desktop-codegen --locked -- architecture check` harus mencetak `architecture: ok, 4 rules, 3 members, 0 outputs up to date`, dan empat perintah gerbang harus lulus, satisfies **AC-1**, **AC-3**
2. Tambah uji `rejects_broken_repos` dengan kasus `N-1` sampai `N-6` dan uji `accepts_allowed_shapes` dengan `P-1`, beserta pembantu yang menyalin repo ke folder sementara. Dengan `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, jalankan perintah AC-1 (dengan angka `4 rules`), AC-2, dan AC-3, catat perintah dan keluarannya di `verify.md` bagian `## Milestone 1: Sumber architecture dan aturan dependensi`, lalu hapus `target/`, satisfies **AC-2**, **AC-3**, **AC-10**

**Milestone 2: Aturan kode dan uji lintas crate** (langkah 3 dan 4, sesudah milestone 1)

3. Pindahkan `workspace_layout.rs` dan `macos_bridge_ownership.rs` dengan `git mv` ke `tooling/codegen/tests/`. Di `macos_bridge_ownership.rs`, ubah dua path `include_str!` menjadi `../../../crates/macos-ffi/src/lib.rs` dan `../../../crates/macos-ffi/Cargo.toml`. Isi lain kedua file tidak diubah. `cargo test --workspace --locked` harus lulus dengan jumlah uji yang sama di kedua file, satisfies **AC-7**
4. Tambah aturan `A-5` sampai `A-11` ke modul dan ke `RULES`, tambah kasus `N-7` sampai `N-19` serta `P-2` dan `P-3`. Baris sukses menjadi `11 rules`. Jalankan perintah AC-4, AC-7, dan AC-8, dan catat di `verify.md` bagian `## Milestone 2: Aturan kode dan uji lintas crate`, satisfies **AC-4**, **AC-7**, **AC-8**, **AC-10**

**Milestone 3: Panduan dan dokumen mengikuti** (langkah 5, sesudah milestone 2)

5. Ubah lima dokumen sesuai bagian **Perubahan dokumen**, tambah aturan `A-12` dan `A-13` ke modul dan ke `RULES`, tambah kasus `N-20` sampai `N-23` serta `P-4` dan `P-5`, semuanya dalam satu commit supaya gerbang tetap hijau. Baris sukses menjadi `13 rules`. Jalankan perintah AC-1, AC-5, AC-6, AC-9, dan AC-10, dan catat di `verify.md` bagian `## Milestone 3: Panduan dan dokumen mengikuti`, satisfies **AC-1**, **AC-5**, **AC-6**, **AC-9**, **AC-10**

**Milestone 4: Bukti di host Windows dan Linux** (langkah 6, `BLOCKED`, `owner: user`)

6. Pada host Windows `x86_64-pc-windows-msvc` dan host Linux `x86_64-unknown-linux-gnu`, jalankan dua perintah AC-11 dan catat OS, arsitektur, versi, perintah, dan keluaran di `verify.md` bagian `## Milestone 4: Bukti di host Windows dan Linux`. Sesi koordinasi di Mac tidak menjalankan langkah ini, satisfies **AC-11**

## Consequences

**Positive**:

* Dependensi terlarang, termasuk yang masuk lewat jalur transitif atau lewat fitur dan target lain, tertangkap oleh gerbang yang sudah ada, tanpa skrip, hook, atau CI baru.
* Bukti arah dependensi tidak lagi berhenti berlaku saat Core mendapat dependensi pertamanya.
* Setiap paket pihak ketiga baru butuh satu baris yang terlihat di review, di Rust maupun di Swift.
* `cargo test -p desktop-core` kembali hanya menguji Core.
* Sumber bersama yang baru ikut diperiksa gerbang begitu didaftarkan di `SOURCES`.
* `AGENTS.md` tidak bisa diam diam usang: tautan, rujukan spec, nama paket, dan path file di dalamnya diperiksa.

**Negative / tradeoffs**:

* Aturan dependensi sekarang ada di tiga tempat: tabel prosa spec `_root/0001`, salinannya di peta arsitektur, dan `MEMBERS`. Dua yang pertama dikunci uji, yang ketiga dijaga `A-1` untuk daftar anggotanya dan `H-5` untuk maknanya.
* Kelas terlarang adalah daftar nama. Paket UI atau SDK yang namanya tidak cocok pola lolos dari `A-4`. `A-3` mengurangi risikonya, karena setiap dependensi langsung baru tetap harus didaftarkan.
* Pemeriksaan teks bergantung pada bentuk hasil `rustfmt` dan pada bentuk `Cargo.lock` versi 4. Perubahan bentuk itu saat toolchain naik bisa membuat aturan gagal walau kodenya benar.
* Tautan rusak di dokumen mana pun, termasuk spec, sekarang menggagalkan `cargo test --workspace --locked`. Itu disengaja, tetapi berarti perubahan dokumen bisa memblokir merge kode.
* Paket bernama `desktop-codegen` sekarang juga memeriksa arsitektur, jadi namanya lebih sempit dari isinya.
* Tanpa `[workspace.lints]`, tiap anggota baru mengulang tiga baris lint.
* Perintah di spec lain yang menyebut `cargo test -p desktop-core --locked --test workspace_layout` tidak lagi berlaku apa adanya. Catatan `verify.md` lama tetap benar untuk commit saat dicatat.

**Neutral**:

* `architecture generate` ada karena pola `Source`, tetapi tidak pernah menulis.
* `test-preferences.json` menyebut `crates/<crate>/tests` sebagai folder uji. Uji alat tinggal di `tooling/codegen/tests/`.
* Lima dokumen di langkah 5 biasanya terlarang bagi sub sesi. Milestone 3 butuh izin eksplisit di brief developer.
* `/develop` menulis `verify.md` saat build. Spec ini tidak membuatnya.

## Follow-up

* [x] Jawaban atas Follow-up Scope 002 di spec `_root/0001`: daftar larangan Core termasuk transitif (`A-3`, `A-4`), sisi yang diizinkan per anggota (`A-2`), keluaran generator yang menyimpang (AC-3), kebijakan `[workspace.lints]` (tidak dibuat, `A-5`), konfigurasi `rustfmt` dan `clippy` (tidak ada), isi `AGENTS.md` (bagian **Perubahan dokumen**), dan spec menggantikan ADR.
* [x] Jawaban atas Follow-up Scope 002 di spec `_root/0002`: AC-3 menjadi `A-8`, R-1 tetap dikunci `contract_surface.rs`, R-8 menjadi `A-9` dan `H-1`, lint Core tidak dipindah, dan bahasa komentar kode ditetapkan Inggris.
* [x] Jawaban atas Follow-up Scope 002 di spec `_root/0003`: `tokens check` berjalan di gerbang lewat `every_source_passes_check_on_the_repo`, aturan "sunting sumber, jalankan `generate`, commit bersama" masuk `AGENTS.md`, dan pemeriksaan arsitektur masuk paket `desktop-codegen`.
* [x] Jawaban atas Follow-up Scope 002 di spec `_root/0004`: `K-4` menjadi `A-7`.
* [x] Jawaban atas Follow-up Scope 002 di spec `macos/0001`, `macos/0002`, dan `macos/0003`: AC-7 menjadi `A-9`, AC-8 menjadi `A-8` dan `A-10`, lint jembatan tidak dipindah. AC-3 dan AC-6 spec `macos/0002` sekarang cukup merujuk `A-2`, `A-4`, `A-10`, dan `workspace_layout.rs` di lokasi barunya. Perbandingan byte binding tetap perintah di Mac sampai scope 012.
* [ ] Lead: tandai baris Follow-up Scope 002 di spec `_root/0001`, `_root/0002`, `_root/0003`, `_root/0004`, `macos/0001`, `macos/0002`, dan `macos/0003` sebagai dijawab spec ini. File itu di luar ownership sesi penulis spec.
* [ ] Lead dan pemilik spec `_root/0001`: tabel "Lokasi dan pemilik" masih menyebut `tooling/validation/`. Folder itu tidak akan dibuat. Kalimat "Uji API publik Core tinggal di `crates/core/tests/`" tetap benar; tambahkan bahwa uji lintas crate tinggal di `tooling/codegen/tests/`.
* [ ] Pemilik spec `_root/0003`, `_root/0006`, `macos/0002`, dan `macos/0003`: teks yang menyebut `crates/core/tests/workspace_layout.rs` atau `cargo test -p desktop-core --locked --test workspace_layout` perlu menunjuk `tooling/codegen/tests/` sesudah langkah 3.
* [ ] Scope 010 (spec `_root/0006`): aturan L-1 (teks antarmuka yang punya kunci tidak ditulis literal di kode UI) belum bisa menjadi aturan `A`, karena belum ada aplikasi yang memakai katalog. Spec OS yang memasang katalog menambahkannya lewat pembaruan spec ini. "Core tanpa pustaka lokalisasi" sudah dijaga `A-3`.
* [ ] Follow-up Scope 002 di [spec `macos/0004`](../../macos/0004-konfigurasi-publik/index.md) (AC-2, tiga setelan turunan tidak kembali ke `project.pbxproj`): belum dijadikan aturan `A`, karena `project.pbxproj` di `main` masih memuat ketiga setelan itu sampai milestone 1 spec itu dibangun, sehingga aturannya akan gagal sekarang. Sesudah milestone itu ada di `main`, tambah aturan `A-14` lewat pembaruan spec ini: `apps/macos/DesktopMac.xcodeproj/project.pbxproj` tidak memuat `INFOPLIST_KEY_CFBundleDisplayName`, `MARKETING_VERSION`, atau `PRODUCT_BUNDLE_IDENTIFIER`.
* [ ] Pemilik spec `_root/0003`: tester scope 008 mencatat bahwa kode keluar `1` dan bentuk stderr bin `desktop-codegen` tidak bisa diuji, karena `main` memakai akar repo yang ditetapkan saat kompilasi, dan mengusulkan variabel `DESKTOP_CODEGEN_ROOT` khusus uji. Spec ini tidak membutuhkannya: uji `architecture` memanggil `plan` pada salinan repo lewat pustaka, dan AC-4 memakai perintah biasa pada repo. Keputusan itu tetap milik spec `_root/0003` (aturan O-2 melarang variabel lingkungan mengubah keluaran).
* [ ] Spec macOS scope 002: pemeriksaan yang butuh Mac. Perbandingan byte binding dengan keluaran generator pada setiap perubahan `crates/core/` atau `crates/macos-ffi/`, dan apakah itu menjadi uji atau tetap perintah sampai scope 012.
* [ ] Scope 006 dan 007 (menunggu host Windows dan Linux): baris `MEMBERS` untuk `desktop-windows` dan `desktop-linux`, nama crate UI dan SDK yang mereka pakai sebagai pola kelas, aturan netral host sebagai aturan `A` (dependensi OS di bawah tabel `target`), dan apakah aplikasi itu butuh pengecualian `unsafe_code`. Bila butuh, kebijakan `[workspace.lints]` ditinjau ulang lewat pembaruan spec ini.
* [ ] Scope 012: skrip `scripts/validate.sh` dan CI menjalankan empat perintah gerbang. `architecture check` tidak perlu dipanggil terpisah.
* [ ] User (`.githooks/` terlarang bagi sub sesi): hook `pre-merge-commit` dan gabungan otomatis `graphify-out/manifest.json` bukan bagian spec ini. Putuskan apakah hook itu perlu diubah, dan apakah kamu ingin hook `pre-push` yang menjalankan gerbang.
* [ ] Nilai dari user `N2`: bahasa komentar kode ditetapkan Inggris sebagai rekomendasi. Bila kamu memilih Indonesia, yang berubah adalah `H-6`, butir "Bahasa" di `AGENTS.md`, dan rustdoc yang sudah ada.
* [ ] Nilai dari user `N3`: gaya pesan commit dan awalan branch tidak diputuskan di sini dan tidak ditulis di `AGENTS.md`. Yang sudah dipakai AC beberapa spec hanyalah subject `spec <folder>/<NNNN> step <n>`.
* [ ] Lead atau `/sync`: `test-preferences.json` bisa menyebut `tooling/codegen/tests/` dan sumber `architecture` di `notes`, dan `AGENTS.md` akar belum menyebut `desktop-codegen` di bagian Stack.
* [ ] `docs/quality/README.md` masih berkata semua kriteria berupa target dan menyebut `tooling/validation` lewat rancangan skrip. Itu di luar tiga dokumen yang diminta; lead memperbaruinya bila perlu.
* [ ] AC-11 dan milestone 4 menunggu host Windows dan Linux (`owner: user`).
