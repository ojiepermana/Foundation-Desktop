# 0008. Aturan pengembangan dan pemeriksaan arsitektur di macOS

**Scope**: [002 - Aturan pengembangan dan pemeriksaan arsitektur](../../../scope/002%20-%20Aturan%20pengembangan%20dan%20pemeriksaan%20arsitektur.md)
**Date**: 2026-11-15
**Status**: In Progress

## Summary

Spec ini menetapkan bagian macOS dari pemeriksaan arsitektur. Ada dua isi. Pertama, gerbang Mac (lima perintah `G-1` sampai `G-5`) yang hanya bisa berjalan di Mac: membangun jembatan, membandingkan binding Swift dengan keluaran generator tanpa menyentuh file yang dilacak git, menjalankan uji Swift, membangun aplikasi lewat Xcode, lalu memastikan repo tetap bersih. Gerbang ini dijalankan sebelum merge bila perubahan menyentuh path yang memengaruhi aplikasi macOS. Kedua, empat aturan teks `C-1` sampai `C-4` untuk kode aplikasi macOS. Aturan itu ditambahkan ke sumber `architecture` di `desktop-codegen`, jadi berjalan di `cargo test --workspace --locked` di host mana pun. Spec ini tidak menambah skrip, CI, atau dependensi. Scope 012 membungkus perintah yang sama.

## Requirements

**User stories**:

* Sebagai developer di Mac, saya ingin satu urutan perintah yang berkata apakah binding Swift, uji Swift, dan build Xcode masih benar sesudah perubahan saya, tanpa mengubah file yang sudah dilacak.
* Sebagai lead, saya ingin tahu dari daftar file yang berubah apakah gerbang Mac wajib dijalankan sebelum merge.
* Sebagai developer aplikasi macOS, saya ingin warna, huruf, jarak, dan kontrol native yang ditetapkan spec tema tetap terjaga sesudah fitur berikutnya, tanpa menunggu tinjauan.
* Sebagai penulis spec scope 012, saya ingin perintah yang sudah pasti, supaya skrip dan job CI hanya membungkusnya.

**Acceptance criteria**:

Angka kriteria merujuk tabel di [Penerimaan dan bukti fondasi](../../../quality/README.md). Semua perintah dijalankan dari akar repo pada Mac Apple Silicon. "Spec bersama" berarti [spec `_root/0007`](../../_root/0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md). `G-1` sampai `G-5` adalah perintah di bagian **Gerbang Mac**, dan `R` adalah pola di bagian **Kapan gerbang wajib**.

* **AC-1** (kriteria 4 dan 25; baris "Integrasi" tabel "Pemeriksaan yang direncanakan"): binding Swift di git sama byte demi byte dengan keluaran generator. Pada working tree yang bersih, `G-1` dan `G-2` keluar dengan kode 0, perintah `diff -r` terakhir di `G-2` tidak mencetak apa pun, dan `git status --short` tidak mencetak apa pun. Jadi `G-2` tidak pernah menulis ke `bindings/swift/`.
* **AC-2** (kriteria 25, binding yang menyimpang tertangkap): pada working tree yang bersih, sesudah `G-1` dan `G-2` lulus, tiga perubahan berikut masing masing membuat `diff -r bindings/swift/Sources target/binding-check` keluar dengan kode 1. (a) Sesudah `printf '\n' >> bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift`, keluarannya memuat baris `diff -r bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift target/binding-check/DesktopMacosFFI/desktop_macos_ffi.swift`. (b) Sesudah `touch bindings/swift/Sources/desktop_macos_ffiFFI/stale.h`, keluarannya memuat `Only in bindings/swift/Sources/desktop_macos_ffiFFI: stale.h`. (c) Sesudah `rm bindings/swift/Sources/desktop_macos_ffiFFI/module.modulemap`, keluarannya memuat `Only in target/binding-check/desktop_macos_ffiFFI: module.modulemap`. Sesudah tiap perubahan dipulihkan (`git checkout -- bindings/swift` dan `rm -f bindings/swift/Sources/desktop_macos_ffiFFI/stale.h`), perintah yang sama keluar dengan kode 0.
* **AC-3** (kriteria 25; mendukung kriteria 4 dan 5): gerbang Mac lulus seluruhnya. Pada working tree yang bersih, `G-1` sampai `G-5` dijalankan berurutan dan semuanya keluar dengan kode 0. Keluaran `G-3` memuat satu baris `Test run with <n> tests in <m> suites passed` (atau `1 suite`) untuk tiap target uji, dan `grep -c failed` atas keluaran itu mencetak `0`. Angka `<n>` dan `<m>` tidak dikunci di sini, karena spec `macos/0005` dan `macos/0006` menambah uji dan suite; spec pemilik tiap suite yang mengunci angkanya. Pada akhir milestone 1 spec ini (di `main` `9b84e6c`) barisnya `Test run with 7 tests in 1 suite passed` dan `Test run with 4 tests in 1 suite passed`. Keluaran `G-4` memuat `** BUILD SUCCEEDED **`. `G-5` tidak mencetak apa pun. Lama tiap perintah dicatat di `verify.md`.
* **AC-4** (kriteria 25, kapan gerbang wajib): pola `R` memilih path yang tepat. Dengan `R` diisi pola dari bagian **Kapan gerbang wajib**, `printf 'docs/scope/scope.md\nREADME.md\nAGENTS.md\ndocs/specs/macos/0001-sapaan-di-jendela-macos/index.md\n' | grep -cE "$R"` mencetak `0`. `printf 'crates/core/src/lib.rs\nCargo.lock\napps/macos/Package.swift\nbindings/swift/Package.swift\ntooling/codegen/src/tokens/mod.rs\nshared/config/public.json\ndesign-system/tokens/colors.json\nrust-toolchain.toml\n' | grep -cE "$R"` mencetak `8`.
* **AC-5** (aturan netral host [spec `_root/0001`](../../_root/0001-batas-workspace-dan-pemilik/index.md)): gerbang Mac tidak masuk `cargo test`. `git grep -nE 'Command::new\("(xcodebuild|swift|xcrun|cargo|diff)"' -- '*.rs'` tidak mencetak apa pun. Pada working tree yang bersih tanpa `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/`, `cargo test --workspace --locked` keluar dengan kode 0 tanpa menjalankan Xcode atau Swift.
* **AC-6** (kriteria 25; Follow-up Scope 002 spec [`macos/0005`](../0005-tema-bersama-dan-kontrol-native/index.md)): aturan `C-1` sampai `C-3` dipasang. Sesudah milestone 3, `cargo run -p desktop-codegen --locked -- architecture check` keluar dengan kode 0, dan pada akhir milestone 3 spec ini angka `<r> rules` di baris suksesnya bertambah tepat tiga dibanding angka yang dicatat sebelum langkah 3. `cargo test -p desktop-codegen --locked --test architecture_macos` keluar dengan kode 0 dan mencetak `test result: ok. 2 passed`: uji `rejects_macos_violations` menolak kasus `D-1` sampai `D-7`, dan uji `accepts_macos_shapes` menerima `B-1` sampai `B-3` di bagian **Kasus uji**. Angka `<r>` sebelum langkah 3 dicatat di `verify.md` sebagai pembanding.
* **AC-7** (kriteria 25; Follow-up Scope 002 spec [`macos/0004`](../0004-konfigurasi-publik/index.md)): aturan `C-4` dipasang. Sesudah milestone 4, `architecture check` keluar dengan kode 0 dan pada akhir milestone 4 spec ini angka `<r> rules` bertambah tepat satu dibanding sebelum langkah 4. Uji `architecture_macos` juga menolak `D-8`.
* **AC-8** (kriteria 26): `AGENTS.md` di akar memuat teks dari bagian **Perubahan dokumen**. `grep -c 'macos/0008' AGENTS.md` mencetak `2` atau lebih, dan `grep -c 'diff -r bindings/swift/Sources target/binding-check' AGENTS.md` mencetak `0`, karena perintah lengkap tinggal di spec ini. `cargo run -p desktop-codegen --locked -- architecture check` tetap keluar dengan kode 0, jadi aturan `A-13` spec bersama menerima teks baru.
* **AC-9** (kriteria 32): fitur ini hanya menambah yang dibutuhkan. Setiap commit fitur ini memakai subject yang memuat `spec macos/0008 step <n>`, dan `git log --name-only --format= --grep='spec macos/0008 step' | sort -u` hanya mencetak path dari tabel **File yang boleh diubah tiap milestone**. Daftar itu tidak memuat `Cargo.toml`, `Cargo.lock`, `.gitattributes`, `.gitignore`, atau path di bawah `scripts/`, `.github/`, `.githooks/`, `apps/`, `bindings/`, dan `crates/`. `cargo tree -p desktop-codegen --depth 1 --prefix none --locked | wc -l` mencetak angka yang sama sebelum langkah 1 dan sesudah langkah 4.

Tidak ada langkah manual. Semua AC dijalankan sesi tanpa jendela. Host Windows dan Linux tidak punya bagian di spec ini: aturan `C` hanya membaca teks dan ikut dibuktikan di host itu lewat AC-11 spec bersama.

## Decision

**Chosen option**: Opsi 1: spec macOS sendiri dengan gerbang Mac berupa perintah, dan aturan teks macOS di sumber `architecture`.

Keputusan turunannya:

| Hal | Keputusan |
| --- | --- |
| Cek binding | Generator menulis ke `target/binding-check/`, lalu `diff -r` membandingkannya dengan `bindings/swift/Sources/`. File yang dilacak tidak pernah ditulis. Perintah "Membuat ulang binding" spec `macos/0001` tetap menjadi cara memperbaikinya. |
| Bentuk gerbang | Lima perintah shell `G-1` sampai `G-5`, bukan uji Rust. Tidak ada uji yang memanggil `cargo`, `swift`, atau `xcodebuild`. |
| Isi gerbang | Jembatan, cek binding, `swift test`, build Xcode `Debug`, lalu repo bersih. Build `Release` tetap di verifikasi spec `macos/0001` AC-4. |
| Kapan wajib | Bila daftar file yang berubah memuat satu path yang cocok dengan pola `R`. Perubahan yang hanya menyentuh dokumen tidak butuh gerbang Mac. |
| Siapa menjalankan | Penulis perubahan sebelum melapor selesai, dan lead sebelum merge ke `main`, keduanya di Mac. Scope 012 menjalankannya di runner macOS. |
| Aturan teks macOS | `C-1` sampai `C-4` di modul privat `tooling/codegen/src/architecture/macos.rs`, terdaftar di `RULES` sumber `architecture`. Berjalan di setiap host. |
| Kapan aturan `C` dipasang | `C-1` sampai `C-3` sesudah milestone 2 spec `macos/0005`, `C-4` sesudah milestone 1 spec `macos/0004`. Sebelum itu aturannya akan gagal pada kode yang sekarang. |
| Format Swift | Tidak ada pemformat atau linter Swift di gerbang. |

Nilai dari spec lain yang dipakai di sini, tanpa diputuskan ulang:

| Nilai | Isi | Pemilik |
| --- | --- | --- |
| Perintah generator binding dan bin `uniffi-bindgen-swift` di balik fitur `bindgen` | Bagian **Membuat ulang binding** | Spec [`macos/0001`](../0001-sapaan-di-jendela-macos/index.md) |
| Perintah `swift test` dan `xcodebuild` | Bagian **Perintah build dan uji**, AC-2 dan AC-4 | Spec `macos/0001` |
| Sumber `architecture`, `RULES`, bentuk pesan `<path>: <ID>: <pesan>`, cara membaca impor Swift dan baris | Bagian "Rancangan pemeriksaan" | Spec bersama |
| Pola gaya literal dan kontrol native | AC-3 dan AC-6 | Spec `macos/0005` |
| Tiga setelan yang pindah ke `apps/macos/Config/Shared.xcconfig` | AC-2 | Spec `macos/0004` |
| Aturan netral host | Bagian "Aturan arah dependensi" | Spec `_root/0001` |

## Rationale

Alasan, opsi yang ditimbang, daftar keputusan beserta pilihan keduanya, dan bukti percobaan ada di [rationale.md](rationale.md).

## Rancangan

### Gerbang Mac

Jalankan dari akar repo, berurutan. Setiap perintah harus keluar dengan kode 0 sebelum perintah berikutnya. `G-3` butuh pustaka dari `G-1`. `G-2` hanya menulis di bawah `target/`, yang diabaikan git. `G-1` sampai `G-4` bisa dijalankan di tengah pekerjaan yang belum di-commit. `G-5` bermakna hanya bila working tree bersih sebelum `G-1`, jadi gerbang lengkap dijalankan sesudah commit.

```bash
# G-1. Jembatan dan Core untuk arm64
cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin

# G-2. Binding sama dengan keluaran generator, tanpa menulis ke bindings/swift
rm -rf target/binding-check
cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- \
  --swift-sources target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a \
  target/binding-check/DesktopMacosFFI
cargo run -p desktop-macos-ffi --features bindgen --bin uniffi-bindgen-swift --locked -- \
  --headers --modulemap --module-name desktop_macos_ffiFFI --modulemap-filename module.modulemap \
  target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a \
  target/binding-check/desktop_macos_ffiFFI
diff -r bindings/swift/Sources target/binding-check

# G-3. Uji Swift tanpa jendela
swift test --package-path apps/macos -Xlinker -L"$PWD/target/aarch64-apple-darwin/debug"

# G-4. Aplikasi lewat Xcode
xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac \
  -configuration Debug -derivedDataPath apps/macos/DerivedData build

# G-5. Repo tetap bersih: harus tidak mencetak apa pun. Butuh working tree yang bersih sebelum G-1
git status --short
```

Argumen generator di `G-2` sama dengan perintah **Membuat ulang binding** spec `macos/0001`. Yang berbeda hanya folder tujuannya. Karena `diff -r` membandingkan dua folder, file yang hilang, file lebih, dan isi yang berbeda semuanya tertangkap.

| Perintah | Gagal berarti | Yang dilakukan |
| --- | --- | --- |
| `G-1` | Jembatan atau Core tidak terbangun untuk `aarch64-apple-darwin` | Perbaiki kode Rust. |
| `G-2` | Binding di git tidak sama dengan keluaran generator | Jalankan **Membuat ulang binding** spec `macos/0001`, lalu commit `bindings/swift/` bersama perubahan Rust. |
| `G-3` | Kontrak sapaan atau ViewModel gagal lewat binding, atau binding usang (`UniFFI API checksum mismatch`) | Perbaiki Swift, atau buat ulang binding. |
| `G-4` | Proyek Xcode, modul Swift, atau fase skrip Cargo tidak terbangun | Perbaiki proyek atau kode Swift. |
| `G-5` | Build membuat atau mengubah file yang tidak diabaikan git | File yang dibuat Xcode dibahas di Follow-up spec `macos/0001`. Jangan di-commit tanpa pembaruan spec. |

### Kapan gerbang wajib

Gerbang Mac wajib bila daftar file yang berubah di branch memuat path yang cocok dengan pola ini:

```text
^(crates/|bindings/|apps/macos/|tooling/codegen/|design-system/|shared/|Cargo\.toml$|Cargo\.lock$|rust-toolchain\.toml$|\.gitattributes$|\.gitignore$)
```

Perintahnya, dengan `R` berisi pola di atas: `git diff --name-only main...HEAD | grep -E "$R"`. `main` adalah `main` lokal yang baru diambil dari `origin/main`, seperti di aturan commit `AGENTS.md`. Bila perintah itu mencetak satu baris atau lebih, jalankan `G-1` sampai `G-5`. Bila tidak mencetak apa pun, cukup gerbang `cargo` biasa.

Alasan tiap path: `crates/` dan `Cargo.lock` mengubah jembatan dan versi UniFFI, `bindings/` dan `apps/macos/` adalah kode yang dibangun Xcode, `tooling/codegen/`, `design-system/`, dan `shared/` menghasilkan file Swift dan setelan Xcode lewat emitter macOS, dan `Cargo.toml`, `rust-toolchain.toml`, serta `.gitattributes` mengubah cara semua itu dibangun dan dibaca, dan `.gitignore` menentukan hasil `G-5`. `.cargo/` tidak ada di pola karena spec `_root/0001` melarang folder itu dan uji `repo_has_no_cargo_config` sudah menolaknya. Keluaran generator dan resource macOS dari spec lain sengaja tidak mendapat baris sendiri, karena semuanya sudah tercakup: `apps/macos/DesignSystem/Tokens.swift` (spec `macos/0005`), `apps/macos/Presentation/Resources/<locale>.lproj/` (spec `macos/0006`), `apps/macos/Config/Shared.xcconfig` (spec `macos/0004`), dan ikon di `apps/macos/Resources/` (spec macOS scope 011) ada di bawah `apps/macos/`, sedangkan sumbernya ada di bawah `design-system/`, `shared/` (termasuk `shared/assets/`), dan `tooling/codegen/`.

### Aturan teks macOS

Aturan ini ditambahkan ke sumber `architecture` milik spec bersama, di modul privat `tooling/codegen/src/architecture/macos.rs`. Pendaftarannya mengikuti kode di `main` (`c3448d9`):

* `mod.rs` mendapat `mod macos;`, dan `plan` memanggil `macos::check(root, &mut findings)` sesudah `code::check`.
* `RULES` mendapat `"C-1"`, `"C-2"`, `"C-3"` di akhir larik (langkah 3), lalu `"C-4"` (langkah 4). Entri `A-` tidak diubah.
* `macos::check` memakai `walk` dan `File` (field `path` dan `text`, metode `is_test`) dari `code.rs`. Keduanya, kedua field, dan `is_test` menjadi `pub(super)` tanpa perubahan lain, jadi folder yang dilewati, cara membaca UTF-8, dan definisi file uji sama dengan aturan `A`. `scan_lines` tidak dipakai, karena ia hanya mencocokkan teks persis dan selalu mencatat huruf `'A'`. `macos.rs` punya pencocok baris sendiri untuk pola di tabel di bawah, dengan bentuk yang sama: satu masalah per baris, pola pertama dalam urutan daftar.
* `Found` di `mod.rs` mendapat field `letter: char`. `Findings::push` tetap seperti sekarang dan mencatat huruf `'A'`, jadi `deps.rs` dan `code.rs` tidak berubah. Fungsi baru `Findings::push_rule(letter: char, rule: u32, path: &str, line: usize, message: String)` dipakai `macos.rs` dengan huruf `'C'`.

Tidak ada item publik baru. Cara menelusuri folder, membaca baris, mendeteksi impor Swift, dan menulis pesan mengikuti spec bersama, termasuk "satu masalah per baris" dan `<teks>` sebagai pola yang ditemukan. Path di depan pesan adalah file yang melanggar.

Tiga tambahan untuk sumber `architecture` sesudah aturan `C` masuk:

* **Urutan masalah.** Kunci urutan di `Findings::into_problems` menjadi `(letter, rule, path, line)`: huruf ID (`A` sebelum `C`), lalu angka ID sebagai bilangan, lalu path dalam urutan byte, lalu nomor baris. Galat baca tetap `rule` `0` dengan huruf `'A'`, jadi tetap paling depan. Untuk masalah yang semuanya `A-`, hasilnya sama dengan sekarang, dan `architecture_contract.rs` tidak berubah.
* **File yang dibaca `C-4` tidak ada atau tidak terbaca.** Masalahnya `apps/macos/DesktopMac.xcodeproj/project.pbxproj: C-4: cannot read: <kesalahan OS>`, dicatat dengan huruf `'C'` dan angka `4`, dan aturan lain tetap berjalan. `C-4` hanya mencari tiga teks itu per baris. Bentuk lain `project.pbxproj` (target, fase build, grup, atau katalog aset yang mungkin ditambah spec macOS scope 011) tidak dibaca, jadi perubahan seperti itu tidak memengaruhi aturan ini. Folder yang dibaca `C-1` sampai `C-3` yang tidak ada berarti tidak ada file untuk diperiksa, dan aturannya lulus.
* **File uji Swift** (`C-3`) adalah file yang salah satu segmen path foldernya bernama `Tests` atau `tests`, sama dengan definisi bagian bukan uji di spec bersama.

| ID | Aturan | Sumber aturan | File yang diperiksa | Pesan |
| --- | --- | --- | --- | --- |
| `C-1` | Tidak ada warna, huruf, atau jarak literal. Baris tidak memuat pola gaya di bawah. | Spec `macos/0005` AC-3, `K-3` | `.swift` di bawah `apps/macos/Presentation/` dan `apps/macos/App/` | `C-1: style literal <teks> outside the generated tokens` |
| `C-2` | Kontrol native tidak diganti. Baris tidak memuat pola kontrol di bawah. | Spec `macos/0005` AC-6, `K-6` | Sama dengan `C-1` | `C-2: native control override <teks>` |
| `C-3` | Hanya file di bawah `apps/macos/Presentation/` dan file uji Swift yang mengimpor `DesignSystem`. | Spec `macos/0005` AC-3 | `.swift` di bawah `apps/macos/` | `C-3: only apps/macos/Presentation imports DesignSystem` |
| `C-4` | Proyek Xcode tidak menulis ulang setelan yang dibuat generator. Baris tidak memuat `INFOPLIST_KEY_CFBundleDisplayName`, `MARKETING_VERSION`, atau `PRODUCT_BUNDLE_IDENTIFIER`. | Spec `macos/0004` AC-2, `K-1` | `apps/macos/DesktopMac.xcodeproj/project.pbxproj` | `C-4: <teks> belongs in apps/macos/Config/Shared.xcconfig` |

**Pola gaya** (`C-1`), dalam urutan ini. Pola ditulis tanpa ekspresi reguler, karena alat tidak punya pustaka regex dan tidak mendapat dependensi baru.

| No. | Pola | `<teks>` di pesan |
| --- | --- | --- |
| 1 | `spacing: ` lalu satu angka ASCII | `spacing:` |
| 2 | `padding(` lalu satu angka ASCII | `padding(` |
| 3 | `padding(.`, lalu satu huruf kecil ASCII atau lebih, lalu `,`, lalu nol spasi atau lebih, lalu satu angka ASCII | `padding(.` |
| 4 | `minLength:` lalu nol spasi atau lebih, lalu satu angka ASCII | `minLength:` |
| 5 sampai 16 | Teks persis `Color(`, `Color.`, `NSColor`, `Font.`, `.font(.`, `fontWeight`, `.bold(`, `.foregroundStyle(.`, `.foregroundColor`, `.tint(.`, `.background(.`, `size:` | Teks itu sendiri |

**Pola kontrol** (`C-2`): teks persis `focusable`, `focusEffectDisabled`, `focusRing`, `controlSize`, `frame(height`, `frame(Height`, `frame(minHeight`, `frame(minheight`, `frame(maxHeight`, `frame(maxheight`, `accessibilityHidden`, `.disabled(`, `ButtonStyle {`, `TextFieldStyle {`, dalam urutan itu. `<teks>` adalah pola itu.

Kedua daftar adalah salinan pola `grep -E` di AC-3 dan AC-6 spec `macos/0005`. Bila spec itu mengubah polanya, daftar di sini ikut diubah lewat pembaruan spec ini. Spec `macos/0006` mengubah AC-6 itu hanya pada hitungan baris tombol (`Button("Greet")` menjadi `Button(LocalizedText.string(.greetingSubmit))`), bukan pada pola yang disalin di sini. Isi file `GreetingView.swift`, `DesktopMacApp.swift`, dan `LocalizedText.swift` dari spec `macos/0004`, `macos/0005`, dan `macos/0006` tidak cocok dengan satu pun pola `C-1` dan `C-2`.

### Kasus uji

File uji `tooling/codegen/tests/architecture_macos.rs`, dibuka komentar dokumen `//!`, hanya memakai API publik `desktop_codegen` dan pustaka standar. Tiap kasus berangkat dari salinan repo di bawah `env!("CARGO_TARGET_TMPDIR")` (folder `architecture_macos_<nama uji>`, dengan cara salin yang sama seperti `architecture_contract.rs`; pembantunya `copy_repo`, `Edit`, `apply`, dan `run_case` boleh disalin, karena file uji tidak berbagi kode), mengubah salinan seperti kolom Perubahan, lalu memanggil `(architecture::SOURCE.plan)` pada salinan itu. Uji `rejects_macos_violations` menjalankan semua kasus `D`: tiap kasus lulus bila paling sedikit satu masalah pesannya diawali ID di kolom Aturan. Uji `accepts_macos_shapes` menjalankan semua kasus `B`: tiap kasus lulus bila `plan` tidak mengembalikan masalah. Kasus `D-8` ditambahkan ke uji pertama di langkah 4.

| No. | Perubahan pada salinan | Aturan | Langkah |
| --- | --- | --- | --- |
| `D-1` | Tambah baris `.padding(20)` di akhir `apps/macos/Presentation/GreetingView.swift` | `C-1` | 3 |
| `D-2` | Tambah baris `let accentColor = Color.red` di akhir `apps/macos/App/DesktopMacApp.swift` | `C-1` | 3 |
| `D-3` | Tambah baris `// VStack(spacing: 8)` di akhir `apps/macos/Presentation/GreetingView.swift` | `C-1` | 3 |
| `D-4` | Tambah baris `// .padding(.horizontal, 4)` di akhir `apps/macos/Presentation/GreetingView.swift` | `C-1` | 3 |
| `D-5` | Tambah baris `// .controlSize(.large)` di akhir `apps/macos/Presentation/GreetingView.swift` | `C-2` | 3 |
| `D-6` | Tambah baris `// .frame(minHeight: 40)` di akhir `apps/macos/App/DesktopMacApp.swift` | `C-2` | 3 |
| `D-7` | Buat `apps/macos/Interop/Look.swift` berisi `import DesignSystem` | `C-3` | 3 |
| `D-8` | Tambah baris `MARKETING_VERSION = 0.1.0;` sebelum baris terakhir `apps/macos/DesktopMac.xcodeproj/project.pbxproj` | `C-4` | 4 |
| `B-1` | Salinan repo tanpa perubahan | | 3 |
| `B-2` | Buat `apps/macos/Tests/DesignSystemTests/Extra.swift` berisi `import DesignSystem` dan `let probe = Color.red` | | 3 |
| `B-3` | Tambah baris `// Color.red` di akhir `apps/macos/DesignSystem/Theme.swift` (di luar folder yang diperiksa `C-1`) | | 3 |

Komentar dipakai di beberapa kasus supaya baris yang ditambahkan tidak perlu menjadi Swift yang sah; aturan membaca teks, bukan sintaks.

### Value sourcing

| Aksi | Nilai | Sumber |
| --- | --- | --- |
| `G-2` | Binding yang diharapkan | Keluaran generator dari `target/aarch64-apple-darwin/debug/libdesktop_macos_ffi.a` hasil `G-1` |
| `G-2` | Binding yang dilacak | `bindings/swift/Sources/` di working tree |
| Kapan wajib | Daftar file yang berubah | `git diff --name-only main...HEAD` |
| `C-1`, `C-2` | Pola | Tabel di atas, salinan dari AC-3 dan AC-6 spec `macos/0005` |
| `C-3` | Siapa mengimpor apa | Baris impor `.swift`, dibaca seperti `A-10` spec bersama |
| `C-4` | Setelan terlarang | AC-2 spec `macos/0004` |
| Baris sukses `architecture` | Jumlah aturan | Panjang `RULES`, yang bertambah tiga di langkah 3 dan satu di langkah 4 |

### Invarian

* Gerbang Mac tidak pernah menulis ke file yang dilacak git. `G-2` menulis hanya di bawah `target/binding-check/`.
* Tidak ada uji Rust yang memanggil `cargo`, `swift`, `xcrun`, `xcodebuild`, atau `diff`. Gerbang Mac tidak pernah ikut `cargo test --workspace --locked`.
* Aturan `C` hanya membaca teks, jadi lulus atau gagal sama di ketiga host.
* Satu aturan, satu pemeriksaan: `A-10` spec bersama tetap memegang impor `DesktopMacosFFI`; `C-3` hanya memegang `DesignSystem`.

### Keamanan

Spec ini tidak menambah rahasia, kredensial, variabel lingkungan, atau akses jaringan. `G-1` sampai `G-4` hanya memakai dependensi yang sudah dikunci `Cargo.lock` dan paket Swift lokal. `A-11` spec bersama tetap melarang paket Swift jarak jauh.

### Kasus gagal yang sudah diputuskan

| Kejadian | Perilaku yang diharapkan |
| --- | --- |
| Perubahan Rust mengubah API jembatan tanpa binding dibuat ulang | `G-2` gagal. `A-9` spec bersama juga gagal di host mana pun bila fungsinya hilang dari binding. |
| Hanya rustdoc fungsi jembatan yang berubah | `G-2` gagal, karena rustdoc ikut masuk binding. Buat ulang binding. |
| Versi UniFFI naik | `G-2` gagal sampai binding dibuat ulang dan tabel **Kombinasi yang dipakai** spec `macos/0001` diperbarui. |
| `target/binding-check/` tertinggal dari percobaan lama | `G-2` menghapusnya lebih dulu. |
| Gerbang dijalankan di host selain Mac | Bukan bagian gerbang host itu. Host lain menjalankan gerbang `cargo` dan aturan `C` saja. |
| Pola `C-1` menolak kode yang benar | Perbarui spec `macos/0005` dan spec ini lebih dulu; aturan tidak dilonggarkan di kode. |
| Spec lain menambah folder Swift di `apps/macos/` | `C-1` dan `C-2` hanya membaca `Presentation/` dan `App/`. Spec itu memutuskan apakah folder barunya ikut, lewat pembaruan spec ini. |

### Skenario uji kritis

* Jalur utama: gerbang Mac lulus pada `main` dan repo tetap bersih, memverifikasi **AC-1**, **AC-3**.
* Binding menyimpang: isi berbeda, file lebih, dan file hilang tertangkap `diff -r`, memverifikasi **AC-2**.
* Pemicu: dokumen saja tidak memicu gerbang, perubahan kode macOS memicunya, memverifikasi **AC-4**.
* Netral host: `cargo test --workspace` tidak menyentuh Xcode atau Swift, memverifikasi **AC-5**.
* Aturan teks: literal gaya, pengganti kontrol, impor `DesignSystem` di luar `Presentation`, dan setelan generator di proyek ditolak, memverifikasi **AC-6**, **AC-7**.
* Panduan dan cakupan, memverifikasi **AC-8**, **AC-9**.

## Perubahan dokumen

Langkah 2 Build plan mengubah `AGENTS.md` di akar. Bagian lain tidak diubah. File itu tulisan user dan terlarang bagi sub sesi, jadi brief developer untuk milestone 2 perlu izin eksplisit dan persetujuan user. Langkah ini berjalan sesudah milestone 3 spec bersama, yang mengubah bagian lain dari file yang sama.

1. Di blok kode `## Commands`, ganti baris komentar `# macOS, dari spec macos/0001 bagian "Perintah build dan uji". swift test butuh jembatan yang sudah dibangun` dengan dua baris ini:

   ```bash
   # macOS: gerbang G-1 sampai G-5 di spec macos/0008 (wajib bila perubahan menyentuh kode yang dibangun Xcode)
   # G-2 (binding sama dengan keluaran generator) ada di spec itu. swift test butuh jembatan yang sudah dibangun
   ```

   Tiga perintah di bawahnya tidak diubah.

2. Di `## Rules`, sesudah butir yang diawali `- **Binding Swift.**`, tambahkan butir:

   ```markdown
   - **Gerbang Mac.** Sebelum melapor selesai dan sebelum merge, jalankan `G-1` sampai `G-5` di spec `macos/0008` bila `git diff --name-only main...HEAD` memuat path yang cocok dengan pola `R` di spec itu (kode Rust, binding, aplikasi macOS, alat dan sumber bersama, serta file build di akar). `G-2` membandingkan binding tanpa mengubah file yang dilacak. Perintah itu tidak masuk `cargo test`, jadi host lain tidak menjalankannya. Aturan teks macOS `C-1` sampai `C-4` di spec yang sama ikut sumber `architecture` dan berjalan di setiap host.
   ```

## File yang boleh diubah tiap milestone

Path `verify.md` dan `index.md` di tabel ini relatif terhadap folder spec ini, `docs/specs/macos/0008-aturan-pengembangan-dan-pemeriksaan-arsitektur/`.

| Milestone | File |
| --- | --- |
| 1 | `verify.md` spec ini, baris `**Status**` di `index.md` spec ini, dan empat file `graphify-out/` dari hook |
| 2 | `AGENTS.md`, `verify.md` spec ini, baris `**Status**` di `index.md` spec ini, dan empat file `graphify-out/` dari hook |
| 3 | `tooling/codegen/src/architecture/macos.rs`, baris `mod macos;`, pemanggilan, entri `RULES`, field `letter`, `Findings::push_rule`, dan kunci urutan di `tooling/codegen/src/architecture/mod.rs`, visibilitas `pub(super)` untuk `walk`, `File`, field `path` dan `text`, dan `File::is_test` di `tooling/codegen/src/architecture/code.rs`, `tooling/codegen/tests/architecture_macos.rs`, `verify.md` spec ini, baris `**Status**` di `index.md` spec ini, dan empat file `graphify-out/` dari hook |
| 4 | Sama dengan milestone 3 |

## Build plan

Pendekatan build proyek adalah Tracer Bullet. Untuk fitur ini benang tipisnya adalah gerbang Mac yang dibuktikan pada `main` tanpa mengubah kode, lalu panduan menunjuknya, lalu aturan teks menyusul begitu kode yang mereka jaga sudah ada. Satu langkah menjadi satu commit dengan subject `type(scope): spec macos/0008 step <n>, ...`.

**Milestone 1: Gerbang Mac terbukti** (langkah 1)

1. Pada working tree yang bersih, dengan `CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, jalankan `G-1` sampai `G-5` dan perintah AC-1 sampai AC-5, catat perintah, keluaran, dan lama tiap perintah di `verify.md` bagian `## Milestone 1: Gerbang Mac terbukti`, pulihkan setiap perubahan AC-2, lalu hapus `target/`, `apps/macos/.build/`, dan `apps/macos/DerivedData/`, satisfies **AC-1**, **AC-2**, **AC-3**, **AC-4**, **AC-5**, **AC-9**

**Milestone 2: Panduan menunjuk gerbang** (langkah 2, after milestone 1 dan milestone 3 spec `_root/0007`; needs persetujuan user untuk `AGENTS.md`)

2. Ubah `AGENTS.md` sesuai bagian **Perubahan dokumen**, jalankan perintah AC-8, dan catat di `verify.md` bagian `## Milestone 2: Panduan menunjuk gerbang`, satisfies **AC-8**, **AC-9**

**Milestone 3: Aturan tema macOS** (langkah 3, after milestone 3 spec `_root/0007` dan milestone 2 spec `macos/0005` di `main`)

3. Catat angka `<r>` baris sukses `architecture check` di `verify.md`. Buat modul `macos.rs` dengan `C-1` sampai `C-3`, daftarkan di sumber `architecture`, terapkan urutan masalah di bagian **Aturan teks macOS**, dan buat `architecture_macos.rs` dengan uji `rejects_macos_violations` (kasus `D-1` sampai `D-7`) dan `accepts_macos_shapes` (kasus `B-1` sampai `B-3`). Jalankan gerbang `cargo` dan perintah AC-6, lalu `G-1` sampai `G-5` (langkah ini menyentuh `tooling/codegen/`), dan catat di `verify.md` bagian `## Milestone 3: Aturan tema macOS`, satisfies **AC-6**, **AC-9**

**Milestone 4: Setelan Xcode dari generator** (langkah 4, after milestone 3 dan milestone 1 spec `macos/0004` di `main`)

4. Tambah `C-4` ke modul dan ke `RULES`, tambah kasus `D-8`, jalankan gerbang `cargo`, perintah AC-7, dan `G-1` sampai `G-5`, lalu catat di `verify.md` bagian `## Milestone 4: Setelan Xcode dari generator`, satisfies **AC-7**, **AC-9**

## Consequences

**Positive**:

* Binding yang usang tertangkap dengan satu perintah yang tidak mengubah file, jadi `G-1` sampai `G-4` bisa dijalankan di tengah pekerjaan yang belum di-commit. `G-5` menunggu commit.
* Lead tahu dari daftar file apakah merge butuh gerbang Mac, tanpa menebak.
* Aturan tema dan konfigurasi macOS berjalan di setiap host lewat gerbang `cargo`, jadi pelanggaran tertangkap juga saat Windows dan Linux bergabung.
* Scope 012 tinggal membungkus lima perintah dan satu pola.

**Negative / tradeoffs**:

* Gerbang Mac masih dijalankan orang sampai CI scope 012 ada, dan butuh sekitar dua setengah menit dengan cache hangat.
* Pola `R` harus ikut diubah saat lokasi baru memengaruhi build macOS. Path yang lupa ditambahkan membuat gerbang tidak terpicu.
* Pola `C-1` dan `C-2` adalah salinan pola spec `macos/0005`. Dua salinan harus diubah bersama.
* Aturan teks membaca baris, jadi literal di dalam komentar juga ditolak.
* `C-1` sampai `C-4` menambah empat ID ke sumber milik spec bersama. Angka di baris sukses `architecture` naik.

**Neutral**:

* Build `Release` tidak masuk gerbang. Perubahan yang hanya memengaruhi `Release` (misalnya setelan konfigurasi Xcode) diperiksa lewat AC-4 spec `macos/0001` saat diverifikasi.
* `AGENTS.md` biasanya terlarang bagi sub sesi. Milestone 2 butuh izin eksplisit di brief developer.
* `/develop` menulis `verify.md` saat build. Spec ini tidak membuatnya.

## Follow-up

* [x] Jawaban atas Follow-up "Spec macOS scope 002" di spec bersama: perbandingan byte binding menjadi `G-2`, tetap perintah dan bukan uji, dijalankan menurut pola `R`.
* [x] Jawaban atas Follow-up Scope 002 di spec `macos/0005`: AC-3 dan AC-6 menjadi `C-1` sampai `C-3`.
* [x] Jawaban atas Follow-up Scope 002 di spec `macos/0004`: AC-2 menjadi `C-4`.
* [ ] Lead: centang ketiga baris Follow-up itu di spec sumbernya.
* [ ] Pemilik spec bersama (`_root/0007`), lewat `architect (update)` yang lead urutkan:
  * baris Follow-up `A-14` diganti rujukan ke `C-4` spec ini;
  * AC-1 spec itu berbatas "pada akhir milestone 3" (`13 rules`). Milestone 3 dan 4 spec ini menambah tepat tiga lalu tepat satu aturan; bila AC-1 diperiksa ulang sesudah itu, tulis angkanya bersama aturan `C`;
  * urutan masalah di bagian "Bentuk alat" spec itu mendapat huruf ID sebagai kunci pertama, seperti bagian **Aturan teks macOS** spec ini;
  * baris "Binding Swift sama byte demi byte" di tabel "Peta aturan ke pemeriksaan" dan baris "Saat `crates/core/` atau `crates/macos-ffi/` berubah, di Mac" di tabel "Kapan pemeriksaan berjalan" menunjuk `G-1` sampai `G-5` spec ini.
* [x] Pemilik spec `macos/0001`, dalam pembaruan gabungan yang lead urutkan (diterapkan di `12c98a0`, 2026-11-15; AC-1 dan **Membuat ulang binding** merujuk `G-2`):
  * bagian **Membuat ulang binding** tetap cara memperbaiki binding; tambahkan satu kalimat bahwa pemeriksaannya adalah `G-2` spec ini;
  * AC-1 boleh merujuk `G-2`.
  * Tidak ada path atau perintah yang berubah, jadi hitungan path di AC-12 spec itu tetap.
* [x] Scope 012 (`arc-f012-spec`, spec `_root/0008`). Dijawab [spec `_root/0008`](../../_root/0008-build-dan-penerimaan-per-target/index.md): `build-macos.sh` membungkus `G-1` sampai `G-5` apa adanya, job macOS memakai `R` sebagai filter `paths:`, dan build `Release` sesudah `G-5` diputuskan di sana tanpa pemicu tambahan (2026-11-15):
  * skrip dan job runner macOS menjalankan `G-1` sampai `G-5` persis seperti di sini, tanpa perintah lain untuk hal yang sama;
  * filter path job macOS memakai pola `R`;
  * bila CI butuh pemicu tambahan, misalnya build `Release` sebelum rilis, itu keputusan spec 0008.
* [ ] Spec macOS berikutnya yang menambah folder Swift di `apps/macos/` (misalnya `Platform/` atau `Resources/`): tentukan apakah `C-1` dan `C-2` ikut membaca folder itu, lewat pembaruan spec ini.
* [ ] Bila tim kelak ingin format Swift yang seragam, `swift format lint` sudah ada di toolchain Xcode. Menambahkannya ke gerbang butuh pembaruan spec ini dan perapian semua file Swift.
