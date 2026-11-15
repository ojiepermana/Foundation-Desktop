# 0008. Build dan penerimaan per target

**Scope**: [012 - Build dan penerimaan per target](../../../scope/012%20-%20Build%20dan%20penerimaan%20per%20target.md)
**Date**: 2026-11-15
**Status**: Proposed

## Summary

Spec ini memberi tim satu cara mengulang build dan uji di setiap host: skrip `bash` tipis di `scripts/` yang hanya membungkus perintah yang sudah ditetapkan spec lain, termasuk gerbang Mac `G-1` sampai `G-5` dari spec `macos/0008`, dan dua workflow GitHub Actions yang hanya memanggil skrip itu. Setiap skrip mencetak host, arsitektur, versi, commit, perintah, dan hasil tiap langkah, jadi keluarannya langsung menjadi bukti. Hasil aktual per OS dan CPU dikumpulkan di satu laporan fondasi, dan fondasi baru boleh dinyatakan selesai saat integrasi macOS, Windows, dan Linux masing masing punya bukti jendela nyata. Di Mac semuanya bisa dibangun dan dibuktikan sekarang; host Windows, Linux, dan run CI pertama menunggu langkah manual kamu.

## Requirements

**User stories**:

* Sebagai developer di host mana pun, kamu ingin satu perintah yang menjalankan gerbang merge dan mencetak hasil tiap langkah, supaya kamu tahu apa yang gagal tanpa membaca spec.
* Sebagai developer aplikasi macOS, kamu ingin satu perintah untuk gerbang Mac dan satu perintah untuk membuat ulang binding, supaya urutan dan flag di spec `macos/0001` dan `macos/0008` tidak perlu disalin tangan.
* Sebagai pemeriksa, kamu ingin keluaran skrip sudah memuat OS, arsitektur, versi, commit, perintah, dan hasil, supaya catatan bukti bisa ditempel apa adanya.
* Sebagai pemilik produk, kamu ingin satu laporan yang berkata target mana sudah terbukti, mana belum, dan apa yang masih dibutuhkan, sebelum fondasi dinyatakan selesai.

**Acceptance criteria**:

Angka kriteria merujuk tabel di [Penerimaan dan bukti fondasi](../../../quality/README.md). Kecuali disebut lain, perintah dijalankan dari akar repo di Mac Apple Silicon koordinasi, pada working tree yang bersih sesudah skripnya di-commit, dan skrip dipanggil dengan `/bin/bash` (bash `3.2` bawaan macOS) supaya kecocokan bash `3.2` benar benar teruji. Jumlah sumber `desktop-codegen` tidak dikunci: skrip membacanya dari baris `sources:` alat itu, jadi angka langkah ditulis sebagai rumus. Angka yang bertanda "pada `96b8279`" diukur pada commit itu.

* **AC-1** (kriteria 28 dan 31, bentuk keluaran): setiap skrip di tabel **Skrip** selain `lib.sh` mencetak baris kepala, baris langkah, dan baris penutup persis seperti bagian **Bentuk keluaran**, dan keluar dengan kode `0` bila semua langkah PASS, `1` bila ada langkah FAIL, `2` bila salah pakai. `/bin/bash scripts/validate.sh | grep -c '^commit: [0-9a-f]\{40\} (clean)$'` mencetak `1`. `/bin/bash scripts/validate.sh --nope` keluar dengan kode `2`, tidak mencetak apa pun ke stdout, dan stderr-nya tepat satu baris `usage: bash scripts/validate.sh`.
* **AC-2** (kriteria 1, 28, dan 30, gerbang bersama): `/bin/bash scripts/validate.sh` keluar dengan kode `0` dan baris terakhirnya `== validate.sh: PASS, <n> steps, 0 failed`, dengan `<n>` = 8 + jumlah nama di baris `sources:` + jumlah berkas dari `git ls-files '*brand-export.txt'` (pada `985534e`: 8 + 5 + 0 = 13). Keluarannya memuat satu baris yang diawali `<sumber>: ok, ` untuk setiap sumber, satu baris yang diawali `desktop-core v`, keluaran langkah 7 tepat satu baris `crates/core/src/greeting.rs` (invarian AC-3 spec `_root/0002`), baris `rust tests: <p> passed, 0 failed, <i> ignored` dengan `<p>` dan `<i>` sama dengan jumlah angka `passed` dan `ignored` di semua baris `test result:` (angkanya berubah setiap kali uji ditambah; pada `985534e`: `178` dan `0`), baris `brand exports: <m> files`, dan satu baris yang diawali `macos gate: `. Sesudahnya `git status --short` tidak mencetak apa pun.
* **AC-3** (kriteria 31, kegagalan tetap tercatat): sesudah `printf '\n' >> design-system/schemas/colors.schema.json`, `/bin/bash scripts/validate.sh` keluar dengan kode `1`, semua langkah tetap dijalankan, dan baris terakhirnya `== validate.sh: FAIL, <n> steps, 2 failed (steps 5 <t>)`, dengan `<n>` sama dengan AC-2 dan `<t>` = 8 + posisi `tokens` di baris `sources:` (dengan urutan sumber sekarang: `steps 5 9`). Langkah 5 adalah `cargo test --workspace --locked`. Sesudah `git checkout -- design-system/schemas/colors.schema.json`, skrip yang sama keluar dengan kode `0`. Lalu, sesudah `printf '// Hello, {name}\n' > probe.rs` (berkas belum dilacak di akar repo), skrip yang sama keluar dengan kode `1`, baris terakhirnya `== validate.sh: FAIL, <n> steps, 1 failed (steps 7)`, dan keluaran langkah 7 memuat baris `probe.rs`; sesudah `rm probe.rs` skrip itu keluar dengan kode `0` lagi.
* **AC-4** (kriteria 1, 28, dan 30, build dan uji Rust): `/bin/bash scripts/build-rust.sh` keluar dengan kode `0` dan berakhir `== build-rust.sh: PASS, 2 steps, 0 failed`. `/bin/bash scripts/test.sh` keluar dengan kode `0`, berakhir `== test.sh: PASS, 2 steps, 0 failed`, dan memuat baris `rust tests: <p> passed, 0 failed, <i> ignored` dengan aturan hitung yang sama dengan AC-2.
* **AC-5** (kriteria 28 dan 30, gerbang Mac; jawaban Follow-up Scope 012 spec `macos/0001`, `macos/0004` sampai `macos/0008`): `/bin/bash scripts/build-macos.sh` keluar dengan kode `0` dan berakhir `== build-macos.sh: PASS, 14 steps, 0 failed`. Langkah 2 sampai 5 adalah `cargo run -p desktop-codegen --locked -- <sumber> check` untuk `tokens`, `config`, `localization`, dan `assets`, berurutan, semuanya sebelum `xcodebuild` pertama. Label langkah 6 sampai 13 sama persis dengan baris perintah `G-1` sampai `G-5` di spec `macos/0008` (perintah `G-2` menjadi empat langkah, 7 sampai 10, dengan perintah bersambung `\` ditulis satu baris), dan langkah 14 adalah `G-4` dengan `-configuration Release`. Keluarannya memuat paling sedikit satu baris yang cocok dengan `Test run with [0-9]+ tests? in [0-9]+ suites? passed`, `** BUILD SUCCEEDED **` tepat dua kali, dan baris `settings <K>: ` serta `bundle <K>: ` untuk `Debug` dan `Release` dengan bentuk dari **Bentuk keluaran**; kedua baris `bundle` memuat `archs arm64` dan `minos 14.0`. Di antara baris `-- step 13: git status --short` dan `-- step 13: PASS` tidak ada baris lain.
* **AC-6** (kriteria 31, binding menyimpang tertangkap; spec `macos/0008` AC-2): sesudah `printf '\n' >> bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift`, `/bin/bash scripts/build-macos.sh` keluar dengan kode `1` dan berakhir `== build-macos.sh: FAIL, 10 steps, 1 failed (steps 10)`. Keluaran langkah 10 memuat `diff -r bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift target/binding-check/DesktopMacosFFI/desktop_macos_ffi.swift`, keluarannya memuat baris petunjuk `G-2` dari **Bentuk keluaran**, dan tidak ada baris `-- step 11:`. Pulihkan dengan `git checkout -- bindings/swift`.
* **AC-7** (kriteria 28, membuat ulang binding; jawaban Follow-up Scope 012 spec `macos/0001`): `/bin/bash scripts/generate-bindings.sh` keluar dengan kode `0`, berakhir `== generate-bindings.sh: PASS, 5 steps, 0 failed`, dan memuat baris `bindings/swift: unchanged`. Sesudah `printf '\n' >> bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift`, skrip yang sama keluar dengan kode `0`, memuat `bindings/swift: unchanged` (generator menulis ulang berkas itu), dan `git status --short` sesudahnya tidak mencetak apa pun.
* **AC-8** (kriteria 28, token; jawaban Follow-up Scope 012 spec `_root/0003` dan `macos/0005`): `/bin/bash scripts/generate-design-tokens.sh` keluar dengan kode `0`, berakhir `== generate-design-tokens.sh: PASS, 2 steps, 0 failed`, memuat baris yang diawali `tokens: ok, `, dan `git status --short` sesudahnya tidak mencetak apa pun.
* **AC-9** (kriteria 28, penjaga host, versi Xcode, dan argumen): dengan `uname` palsu, `d=$(mktemp -d); printf '#!/bin/sh\necho Linux\n' > "$d/uname"; chmod +x "$d/uname"`, perintah `PATH="$d:$PATH" /bin/bash scripts/build-macos.sh` dan `PATH="$d:$PATH" /bin/bash scripts/generate-bindings.sh` masing masing keluar dengan kode `2`, tidak mencetak apa pun ke stdout, dan stderr-nya tepat satu baris yang diawali `<nama>.sh: needs macOS on arm64, this host is Linux`. `PATH="$d:$PATH" /bin/bash scripts/build-macos.sh --nope` keluar dengan kode `2` dengan stderr `usage: bash scripts/build-macos.sh`, karena argumen diperiksa sebelum host. Dengan `xcodebuild` palsu, `x=$(mktemp -d); printf '#!/bin/sh\necho "Xcode 25.4"\necho "Build version 25E1"\n' > "$x/xcodebuild"; chmod +x "$x/xcodebuild"`, perintah `PATH="$x:$PATH" /bin/bash scripts/build-macos.sh` keluar dengan kode `2`, tidak mencetak apa pun ke stdout, dan stderr-nya tepat `build-macos.sh: needs Xcode 26 or newer (spec macos/0007), this host has Xcode 25.4`. Sesudahnya `rm -rf "$d" "$x"`.
* **AC-10** (kriteria 29, workflow dikonfigurasi): dengan `blk() { awk -v n="$1" '/^```yaml$/{c++; if(c==n){f=1;next}} /^```$/{f=0} f' docs/specs/_root/0008-build-dan-penerimaan-per-target/index.md; }`, `blk 1 | diff - .github/workflows/foundation.yml` dan `blk 2 | diff - .github/workflows/foundation-macos.yml` tidak mencetak apa pun. `git ls-files .github` mencetak tepat dua baris, kedua path itu. Untuk tiap berkas `W` dari keduanya: `ruby -ryaml -e 'YAML.load_file(ARGV[0])' "$W"` keluar dengan kode `0`; `grep -E '^ +(- )?run: ' "$W" | grep -vcE 'run: (rustup toolchain install|bash scripts/[a-z-]+\.sh)$'` mencetak `0`, jadi workflow tidak punya perintah sendiri selain skrip; `grep -E 'uses: ' "$W" | grep -vcE 'uses: actions/checkout@[0-9a-f]{40} # v[0-9.]+$'` mencetak `0`. Daftar `paths:` di `foundation-macos.yml` sama dengan tabel **Pola `R` sebagai filter path**.
* **AC-11** (kriteria 29 dan 30, run CI nyata; `NOT_RUN`, `owner: user`, langkah manual `F012-2`): sesudah `main` di-push ke `origin`, run workflow `foundation` pada commit itu selesai dengan kedua job `rust (ubuntu-24.04)` dan `rust (windows-2025)` berhasil, run workflow `foundation-macos` pada commit itu (dari push, atau lewat Run workflow bila push tidak menyentuh path filternya) selesai dengan job `macos (macos-26)` berhasil, dan log tiap job memuat baris penutup `PASS` untuk setiap skrip yang ia jalankan. URL run, citra runner, dan baris kepala tiap skrip dicatat di laporan fondasi. Di CI, kriteria 28 untuk build Rust dibuktikan langkah 4 `validate.sh`; `build-rust.sh` dibuktikan di Mac (AC-4) dan lewat `F012-1`.
* **AC-12** (kriteria 1, 28, dan 30 pada host lain; `BLOCKED`, `owner: user`, langkah manual `F012-1`): pada host Windows `x86_64-pc-windows-msvc` (Git Bash) dan host Linux `x86_64-unknown-linux-gnu`, `bash scripts/build-rust.sh`, `bash scripts/validate.sh`, dan `bash scripts/test.sh` keluar dengan kode `0`. Keluarannya memenuhi syarat lulus `F001-1`, `F002-1`, dan `F008-1` sekaligus (pemetaannya di **Daftar penerimaan per target**).
* **AC-13** (kriteria 31, Done when "menelusuri hasil aktual untuk tiap OS dan CPU"): laporan ada di path dari bagian **Laporan fondasi**, dengan heading berurutan persis seperti di sana. Dengan `L` = path laporan dan `sec() { awk -v h="$1" '$0==h{f=1;next} /^## /{f=0} f' "$L"; }`: `sec '## Hasil per target' | grep -cE '^\| (macOS|Windows|Linux) '` mencetak `6`; `sec '## Status kriteria' | grep -cE '^\| [0-9]+ \|'` mencetak `32`; `sec '## Perintah build dan uji' | grep -cE '^\| `[a-z-]+\.sh` \|'` mencetak `6`. Setiap sel `PASS` atau `FAIL` memuat tautan ke catatan yang ada (bagian `verify.md`, catatan `docs/reviews/`, baris di file langkah manual koordinasi, atau URL run CI), dan catatan itu menyebut host, arsitektur, versi, commit, perintah, dan keluaran aktual. Baris macOS `arm64` mengutip baris penutup kelima skrip Mac dari run pada commit di baris `**Diperbarui:**`. Setiap tautan relatif di laporan menunjuk berkas yang ada: perintah di bagian **Pemeriksaan tautan** tidak mencetak apa pun, dan sesudah milestone 3 spec `_root/0007` aturan `A-12` menjaganya di gerbang.
* **AC-14** (Done when "membuktikan tiga integrasi wajib sebelum menyatakan fondasi selesai"): `sec '## Tiga integrasi wajib' | grep -cE '^\| (macOS|Windows|Linux) \|'` mencetak `3`, dan tiap baris memuat bukti yang dibutuhkan dari tabel **Tiga integrasi wajib** spec ini, langkahnya, status, dan bukti. `sec '## Aturan status' | grep -cE '^\* \*\*S-[1-5]\*\*'` mencetak `5`. Selama satu baris integrasi belum `PASS`, `grep -c '^\*\*Status fondasi:\*\* belum selesai$' "$L"` mencetak `1`.
* **AC-15** (kriteria 28, skrip berjalan sama di tiga host): `.gitattributes` memuat baris `/scripts/*.sh text eol=lf` dan `/.github/workflows/*.yml text eol=lf` sesudah baris yang sudah ada. `git ls-files --eol -- 'scripts/*.sh' .github | grep -vc 'attr/text eol=lf'` mencetak `0`. `git ls-files -s -- 'scripts/*.sh' | awk '$4!="scripts/lib.sh" && $1!="100755"'` dan `git ls-files -s -- scripts/lib.sh | awk '$1!="100644"'` tidak mencetak apa pun. Untuk setiap berkas `scripts/*.sh`, `/bin/bash -n <berkas>` keluar dengan kode `0`.
* **AC-16** (kriteria 32 dan batas spec lain): skrip hanya membungkus perintah yang sudah ada. Setiap commit fitur ini memakai subject yang memuat `spec _root/0008 step <n>`, dan `git log --name-only --format= --grep='spec _root/0008 step' | grep -v '^graphify-out/' | sort -u` hanya mencetak path dari tabel **File yang boleh diubah tiap milestone**. `Cargo.toml`, `Cargo.lock`, `.githooks/`, `scripts/update_graphify.py`, dan `.cargo/` tidak ada di daftar itu. `grep -nE 'git (commit|push|config|stash|add)|graphify|githooks|--no-verify|curl|wget' scripts/*.sh` tidak mencetak apa pun. `test ! -e scripts/build-windows.ps1 && test ! -e scripts/build-linux.sh` keluar dengan kode `0`.

## Decision

**Chosen option**: Opsi 1: skrip `bash` tipis, satu workflow yang hanya memanggil skrip, dan satu laporan fondasi yang hidup.

Keputusan turunannya:

| Hal | Keputusan |
| --- | --- |
| Bahasa skrip | `bash`, ditulis agar jalan di bash `3.2` (bawaan macOS) sampai `5.x` (Git Bash di Windows, Linux). Dipanggil `bash scripts/<nama>.sh` dari folder mana pun. |
| Skrip yang dibuat sekarang | Enam skrip dan satu pembantu di tabel **Skrip**. `build-windows.ps1` dan `build-linux.sh` dari rancangan `docs/quality/README.md` dibuat spec scope 006 dan 007, saat aplikasi Windows dan Linux ada. Sampai itu, host Windows dan Linux memakai skrip bersama. |
| Isi skrip | Hanya perintah yang sudah ditetapkan spec pemiliknya, dengan teks yang sama. Skrip tidak menambah dependensi, tidak mengunduh, tidak menyentuh hook, dan tidak meng-commit. Pemeriksaan teks sapaan satu sumber (AC-3 spec `_root/0002`) ikut di `validate.sh`, karena perintahnya sudah ada tetapi belum dijalankan gerbang mana pun. |
| Gerbang Mac | `build-macos.sh` menjalankan `G-1` sampai `G-5` spec `macos/0008` apa adanya, didahului `check` keempat sumber yang punya keluaran macOS (`tokens`, `config`, `localization`, `assets`) dan diikuti build Release. Pemeriksaan binding hanya `G-2`; spec ini tidak menambah pemeriksaan binding sendiri. |
| Kapan gerbang Mac wajib | Di mesin lokal, `validate.sh` mencetak baris `macos gate:` dari pola `R` spec `macos/0008` atas `git diff --name-only main...HEAD`; baris itu catatan, bukan langkah. Di CI, workflow `foundation-macos` hanya terpicu oleh path yang cocok dengan `R` (ditulis sebagai filter `paths:`), ditambah `scripts/**` dan berkas workflow itu sendiri. |
| Daftar sumber `desktop-codegen` | Dibaca dari baris `sources:` keluaran alat itu, bukan ditulis di skrip. Sumber baru ikut tanpa mengubah skrip. |
| Langkah gagal | `validate.sh`, `build-rust.sh`, dan `test.sh` menjalankan semua langkah lalu melapor. `build-macos.sh`, `generate-bindings.sh`, dan `generate-design-tokens.sh` berhenti di langkah pertama yang gagal, karena langkahnya saling bergantung (spec `macos/0008`: setiap perintah harus lulus sebelum berikutnya). |
| Toolchain | Langkah 1 setiap skrip memeriksa `rustc --version` diawali `rustc <channel> ` dari `rust-toolchain.toml`. CI memasangnya dengan `rustup toolchain install` tanpa argumen. |
| CI | Dua workflow. `.github/workflows/foundation.yml`: job `rust` di `ubuntu-24.04` dan `windows-2025` menjalankan `validate.sh` pada setiap push ke `main` dan pull request. `.github/workflows/foundation-macos.yml`: job `macos` di `macos-26` (Xcode bawaan citra) menjalankan `validate.sh` lalu `build-macos.sh`, dengan filter path dari `R`. Runner macOS butuh Xcode 26 atau lebih baru untuk ikon `.icon` (spec `macos/0007`); citra `macos-26` memakai 26.6 sebagai bawaan. Izin `contents: read`, satu action (`actions/checkout`) dipaku ke SHA commit, tanpa cache, tanpa rahasia. |
| Bukti dari CI | Log job dan ringkasan job (`$GITHUB_STEP_SUMMARY`) yang ditulis skrip. Tidak ada artefak unggahan. |
| Laporan fondasi | Satu berkas Markdown yang diperbarui setiap kali bukti baru masuk. Path dan isinya di bagian **Laporan fondasi**. |
| Tiga integrasi wajib | Integrasi sapaan di jendela native macOS, Windows, dan Linux, dibaca dari kalimat pembuka `docs/quality/README.md` ("integrasi wajib pada ketiga OS") dan tabel "Bukti native per host" di dokumen yang sama. |
| Langkah manual host | Satu langkah `F012-1` menjalankan skrip bersama di Windows dan Linux, dan keluarannya memenuhi `F001-1`, `F002-1`, dan `F008-1`. Teks ketiga langkah itu tidak disalin ulang. |
| `[profile.release]` | Tidak dibutuhkan. Tidak ada perubahan `Cargo.toml` (jawaban Follow-up Scope 012 spec `_root/0001`). |

Nilai dari spec lain yang dipakai di sini, tanpa diputuskan ulang:

| Nilai | Pemilik |
| --- | --- |
| Empat perintah gerbang, perintah `cargo tree` Core, aturan `--locked` | Spec [`_root/0001`](../0001-batas-workspace-dan-pemilik/index.md), `AGENTS.md` bagian Commands |
| Perintah `<sumber> check` dan `<sumber> generate`, kode keluar, baris usage `sources:` | Spec [`_root/0003`](../0003-tema-bersama-dan-kontrol-native/index.md) bagian **API surface**; spec [`_root/0007`](../0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md) AC-1 |
| Gerbang `cargo test --workspace --locked` menjalankan `check` semua sumber | Spec `_root/0007` |
| `git grep --untracked -lF 'Hello, {' -- '*.rs'` mencetak tepat `crates/core/src/greeting.rs` | Spec [`_root/0002`](../0002-kontrak-sapaan-bersama/index.md) AC-3; spec [`_root/0006`](../0006-satu-contoh-lokalisasi/index.md) AC-10; `crates/core/AGENTS.md` |
| Pemeriksaan ekspor basi `git ls-files -s -- shared/assets/source \| diff - <folder>/brand-export.txt`, dan baris `.gitattributes` untuk `brand-export.txt` milik spec OS | Spec [`_root/0005`](../0005-aset-merek-bersama/index.md) aturan **E-6** dan tabel "Kontrak untuk spec OS" |
| Perintah **Membuat ulang binding** | Spec [`macos/0001`](../../macos/0001-sapaan-di-jendela-macos/index.md) |
| `G-1` sampai `G-5` dan pola `R` | Spec [`macos/0008`](../../macos/0008-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md) |
| `config check` menjaga `MARKETING_VERSION` sama dengan versi workspace; perintah `-showBuildSettings` dan `plutil` di AC-3 dan AC-4 | Spec [`macos/0004`](../../macos/0004-konfigurasi-publik/index.md) |
| `tokens check`, `swift test`, dan `xcodebuild` di runner macOS | Spec [`macos/0005`](../../macos/0005-tema-bersama-dan-kontrol-native/index.md) |

## Rationale

Alasan, opsi yang ditimbang, daftar keputusan beserta pilihan keduanya, catatan pemeriksaan silang, dan bukti percobaan ada di [rationale.md](rationale.md).

## Rancangan

### Skrip

Semua skrip ada di `scripts/`. Label langkah adalah teks perintah persis seperti di spec pemiliknya, termasuk `$PWD` yang tidak dijabarkan.

| Skrip | Host | Langkah, berurutan | Bila langkah gagal | Membungkus |
| --- | --- | --- | --- | --- |
| `lib.sh` | (pembantu) | Tidak dijalankan sendiri. Dibaca skrip lain dengan `.`. Berisi fungsi di bagian **Kerangka `lib.sh`**. | | |
| `build-rust.sh` | semua | 1 toolchain; 2 `cargo build --workspace --locked` | lanjut | Spec `_root/0001` |
| `test.sh` | semua | 1 toolchain; 2 `cargo test --workspace --locked` | lanjut | Spec `_root/0001` |
| `validate.sh` | semua | 1 toolchain; 2 `cargo fmt --all --check`; 3 `cargo clippy --workspace --all-targets --locked -- -D warnings`; 4 `cargo build --workspace --locked`; 5 `cargo test --workspace --locked`; 6 `cargo tree -p desktop-core --target all --all-features --prefix none --locked`; 7 `git grep --untracked -lF 'Hello, {' -- '*.rs'` (harus tepat `crates/core/src/greeting.rs`); 8 `cargo run -p desktop-codegen --locked` (daftar sumber); lalu satu langkah `cargo run -p desktop-codegen --locked -- <sumber> check` per sumber, sesuai urutan baris `sources:`; lalu satu langkah `git ls-files -s -- shared/assets/source \| diff - <berkas>` per berkas dari `git ls-files '*brand-export.txt'` | lanjut | Spec `_root/0001` sampai `_root/0007` |
| `generate-design-tokens.sh` | semua | 1 toolchain; 2 `cargo run -p desktop-codegen --locked -- tokens generate` | berhenti | Spec `_root/0003` |
| `generate-bindings.sh` | Darwin `arm64` | 1 toolchain; 2 `cargo build -p desktop-macos-ffi --locked --target aarch64-apple-darwin`; 3 `rm -rf bindings/swift/Sources`; 4 dan 5 dua perintah `uniffi-bindgen-swift` persis dari **Membuat ulang binding** spec `macos/0001`; lalu baris `bindings/swift:` | berhenti | Spec `macos/0001`. Perintah `git diff` terakhir di blok itu diganti baris `bindings/swift:`, karena pemeriksaan binding milik `G-2`. |
| `build-macos.sh` | Darwin `arm64` | 1 toolchain; 2 sampai 5 `cargo run -p desktop-codegen --locked -- <sumber> check` untuk `tokens`, `config`, `localization`, `assets`; 6 `G-1`; 7 sampai 10 empat perintah `G-2`; 11 `G-3`; 12 `G-4`; 13 `G-5`; 14 `G-4` dengan `-configuration Release`; lalu baris `settings <K>:` dan `bundle <K>:` | berhenti | Spec `macos/0008`; spec `macos/0001` AC-4; spec `macos/0004` AC-3 dan AC-4; Follow-up spec `macos/0004` sampai `macos/0007` (sumber yang punya keluaran macOS diperiksa sebelum `xcodebuild`) |

Aturan bersama semua skrip:

* Baris pertama `#!/usr/bin/env bash`, lalu `set -u` dan `set -o pipefail`. Tanpa `set -e`, karena `step` yang memutuskan lanjut atau berhenti.
* Skrip pindah ke akar repo lewat path dirinya sendiri (`cd "$(dirname "$0")/.."`), jadi bisa dipanggil dari folder mana pun.
* Hanya fitur bash `3.2`. Tanpa array sama sekali (bash `3.2` dengan `set -u` gagal pada array kosong); daftar disimpan sebagai string yang dipisah baris baru. Tanpa `mapfile`, `${var,,}`, `local -n`, atau `|&`.
* Urutan pemeriksaan awal: argumen dulu, lalu host. Argumen apa pun mencetak `usage: bash scripts/<nama>.sh` ke stderr dan keluar dengan kode `2`. Skrip Darwin `arm64` di host lain mencetak `<nama>.sh: needs macOS on arm64, this host is <uname -s> <uname -m>` ke stderr dan keluar dengan kode `2`. Lalu `build-macos.sh` saja memeriksa Xcode: angka major dari baris pertama `xcodebuild -version` (`Xcode <major>.<minor>`) harus `26` atau lebih, karena ikon `.icon` spec `macos/0007` dikompilasi `actool` Xcode 26 ke atas; bila tidak, ia mencetak `build-macos.sh: needs Xcode 26 or newer (spec macos/0007), this host has <baris pertama>` ke stderr dan keluar dengan kode `2`. Semua pemeriksaan ini terjadi sebelum baris kepala.
* Stderr setiap perintah digabung ke stdout, jadi urutan baris di log sama dengan urutan kejadian.
* Skrip tidak menulis berkas di luar `target/`, `apps/macos/.build/`, `apps/macos/DerivedData/`, dan berkas keluaran generator yang ditulis `tokens generate` atau `uniffi-bindgen-swift`. Log sementara ada di `target/scripts/`.

### Kerangka `lib.sh`

Bentuk wajibnya; nama variabel boleh berbeda, perilakunya tidak.

```bash
# step <label> <perintah> [argumen...]: jalankan perintah, gabungkan stderr ke stdout, catat hasil.
step() {
  label=$1; shift
  STEPS=$((STEPS + 1))
  echo "-- step $STEPS: $label"
  start=$SECONDS
  "$@" 2>&1
  code=$?
  if [ "$code" -eq 0 ]; then result=PASS; else result=FAIL; FAILED=$((FAILED + 1)); FAILED_STEPS="$FAILED_STEPS $STEPS"; fi
  echo "-- step $STEPS: $result (exit $code, $((SECONDS - start))s)"
  summary_row "$STEPS" "$label" "$result"
  return "$code"
}

# logged <berkas> <perintah> [argumen...]: seperti menjalankan perintah, sambil menyimpan salinan di target/scripts/<berkas>.
logged() {
  mkdir -p target/scripts
  file=target/scripts/$1; shift
  "$@" 2>&1 | tee "$file"
}
```

* Skrip yang lanjut memanggil `step ...` saja. Skrip yang berhenti memanggil `step ... || finish`.
* Perintah dengan pipa atau pemeriksaan keluaran dibungkus fungsi kecil di skrip itu, dan labelnya tetap teks perintah dari spec pemilik: `brand_export_check <berkas>` menjalankan `git ls-files -s -- shared/assets/source | diff - "<berkas>"`; `must_print_nothing <perintah>` menjalankan perintah, meneruskan keluarannya, dan keluar `1` bila keluarannya tidak kosong (dipakai `G-5`); `single_greeting_source` menjalankan `git grep --untracked -lF 'Hello, {' -- '*.rs'`, meneruskan keluarannya, dan keluar `0` hanya bila keluarannya tepat satu baris `crates/core/src/greeting.rs`; `list_sources` menjalankan `cargo run -p desktop-codegen --locked`, keluar `0` hanya bila kode keluar alat itu `2` dan ada baris `^sources: ` dengan paling sedikit satu nama, dan menyimpan daftar namanya.
* `finish` mencetak baris tambahan skrip itu, baris penutup, dan keluar dengan kode `0` atau `1`.
* `toolchain_step` membaca channel dengan `sed -n 's/^channel *= *"\([^"]*\)".*/\1/p' rust-toolchain.toml | tr -d '\r'` dan lulus bila `rustc --version` diawali `rustc <channel> ` (dengan spasi, supaya `1.99.10` tidak cocok dengan `1.99.1`).
* `summary_row` menambah satu baris ke `$GITHUB_STEP_SUMMARY` bila variabel itu terisi: `| <n> | `<label>` | <PASS|FAIL> |`, dengan setiap `|` di label ditulis `\|`. Judul `### <nama>.sh` dan baris kepala ditulis sekali sebelum langkah pertama, hasil akhir sesudah langkah terakhir. Di luar CI variabel itu kosong dan tidak ada yang ditulis.

### Bentuk keluaran

Baris kepala, sebelum langkah pertama:

```text
== <nama>.sh
host: <uname -s> <uname -m>
commit: <git rev-parse HEAD> (clean|dirty)
rustc: <rustc --version>
cargo: <cargo --version>
macos: <sw_vers -productVersion>
xcode: <xcodebuild -version, dua baris digabung dengan satu spasi>
swift: <baris pertama swift --version>
```

Tiga baris terakhir hanya di Darwin. `clean` berarti `git status --porcelain` tidak mencetak apa pun saat skrip mulai.

Per langkah:

```text
-- step <n>: <label>
<keluaran perintah, stdout dan stderr>
-- step <n>: PASS (exit 0, <detik>s)
```

Hasil gagal ditulis `FAIL (exit <kode>, <detik>s)`. Label langkah 1 adalah `toolchain matches rust-toolchain.toml (<channel>)`; langkah itu mencetak keluaran `rustc --version`.

Baris tambahan, sesudah langkah terakhir dan sebelum baris penutup:

| Skrip | Baris | Sumber nilai |
| --- | --- | --- |
| `validate.sh`, `test.sh` | `rust tests: <p> passed, <f> failed, <i> ignored` | Jumlah angka di semua baris `test result:` pada `target/scripts/cargo-test.log` dari langkah `cargo test` (lewat `logged`). Bila langkah itu tidak menghasilkan baris `test result:`, tertulis `rust tests: no result lines` |
| `validate.sh` | `brand exports: <m> files` | Jumlah baris `git ls-files '*brand-export.txt'` |
| `validate.sh` | `macos gate: required, <k> changed paths match spec macos/0008 R`, `macos gate: not required` (0 path cocok), atau `macos gate: unknown, no local main branch` | `git diff --name-only main...HEAD \| grep -E "$R"`, dengan `R` disalin persis dari bagian "Kapan gerbang wajib" spec `macos/0008`; `unknown` bila `git rev-parse --verify -q main` gagal (misalnya di CI) |
| `generate-bindings.sh` | `bindings/swift: unchanged` atau `bindings/swift: changed, review and commit it with the bridge change (spec macos/0001)` | `git status --porcelain -- bindings/swift` kosong atau tidak |
| `build-macos.sh` | `G-2: bindings/swift differs from the generator output, run bash scripts/generate-bindings.sh and commit bindings/swift with the Rust change (spec macos/0008)` | Dicetak hanya bila langkah 10 FAIL |
| `build-macos.sh` | `settings <K>: INFOPLIST_KEY_CFBundleDisplayName=<v>, MARKETING_VERSION=<v>, PRODUCT_BUNDLE_IDENTIFIER=<v>, PRODUCT_NAME=<v>, CURRENT_PROJECT_VERSION=<v>` | `xcodebuild -project apps/macos/DesktopMac.xcodeproj -scheme DesktopMac -configuration <K> -showBuildSettings 2>/dev/null`, nilai tiap kunci dari `sed -n 's/^    <KUNCI> = //p'` (empat spasi, sama dengan AC-3 spec `macos/0004`) |
| `build-macos.sh` | `bundle <K>: archs <a>, minos <m>, CFBundleDisplayName <v>, CFBundleIdentifier <v>, CFBundleShortVersionString <v>, CFBundleVersion <v>, CFBundleName <v>` | Dengan `A=apps/macos/DerivedData/Build/Products/<K>/DesktopMac.app`: `lipo -archs "$A/Contents/MacOS/DesktopMac"`; angka sesudah `minos ` di `vtool -show-build "$A/Contents/MacOS/DesktopMac"`; `plutil -extract <kunci> raw -o - "$A/Contents/Info.plist"` |

Baris `settings` dan `bundle` dicetak untuk `Debug` lalu `Release`, hanya bila langkah 14 lulus. Alat pembaca fakta tidak dihitung sebagai langkah dan tidak mengubah kode keluar. Nilai yang tidak terbaca ditulis `<missing>`. Baris itu catatan; yang menilai isinya tetap AC spec `macos/0001` dan `macos/0004`.

Baris penutup, selalu baris terakhir stdout:

```text
== <nama>.sh: PASS, <n> steps, 0 failed
== <nama>.sh: FAIL, <n> steps, <f> failed (steps <nomor dipisah spasi>)
```

`<n>` adalah jumlah langkah yang benar benar dijalankan. Skrip yang berhenti lebih awal melaporkan langkah sampai yang gagal.

### Workflow

Isi `.github/workflows/foundation.yml` harus sama persis dengan blok pertama, dan isi `.github/workflows/foundation-macos.yml` dengan blok kedua, masing masing diakhiri satu baris baru.

```yaml
name: foundation

on:
  push:
    branches: [main]
  pull_request:
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: foundation-${{ github.ref }}
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}

env:
  CARGO_TERM_COLOR: never
  CARGO_INCREMENTAL: "0"
  CARGO_PROFILE_DEV_DEBUG: "0"

jobs:
  rust:
    name: rust (${{ matrix.os }})
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-24.04, windows-2025]
    runs-on: ${{ matrix.os }}
    timeout-minutes: 30
    defaults:
      run:
        shell: bash
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - run: rustup toolchain install
      - run: bash scripts/validate.sh
```

```yaml
name: foundation-macos

on:
  push:
    branches: [main]
    paths:
      - "crates/**"
      - "bindings/**"
      - "apps/macos/**"
      - "tooling/codegen/**"
      - "design-system/**"
      - "shared/**"
      - "Cargo.toml"
      - "Cargo.lock"
      - "rust-toolchain.toml"
      - ".gitattributes"
      - ".gitignore"
      - "scripts/**"
      - ".github/workflows/foundation-macos.yml"
  pull_request:
    paths:
      - "crates/**"
      - "bindings/**"
      - "apps/macos/**"
      - "tooling/codegen/**"
      - "design-system/**"
      - "shared/**"
      - "Cargo.toml"
      - "Cargo.lock"
      - "rust-toolchain.toml"
      - ".gitattributes"
      - ".gitignore"
      - "scripts/**"
      - ".github/workflows/foundation-macos.yml"
  workflow_dispatch:

permissions:
  contents: read

concurrency:
  group: foundation-macos-${{ github.ref }}
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}

env:
  CARGO_TERM_COLOR: never
  CARGO_INCREMENTAL: "0"
  CARGO_PROFILE_DEV_DEBUG: "0"

jobs:
  macos:
    name: macos (macos-26)
    runs-on: macos-26
    timeout-minutes: 45
    defaults:
      run:
        shell: bash
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - run: rustup toolchain install
      - run: bash scripts/validate.sh
      - if: ${{ !cancelled() }}
        run: bash scripts/build-macos.sh
```

**Pola `R` sebagai filter path.** Filter GitHub memakai glob, bukan regex, jadi tiap bagian `R` spec `macos/0008` ditulis ulang satu lawan satu. `*` di filter tidak melewati `/`, jadi nama berkas tanpa folder hanya cocok di akar, sama dengan `^...$` di `R`.

| Bagian `R` | Glob di `paths:` |
| --- | --- |
| `^crates/`, `^bindings/`, `^apps/macos/`, `^tooling/codegen/`, `^design-system/`, `^shared/` | `crates/**`, `bindings/**`, `apps/macos/**`, `tooling/codegen/**`, `design-system/**`, `shared/**` |
| `^Cargo\.toml$`, `^Cargo\.lock$`, `^rust-toolchain\.toml$`, `^\.gitattributes$`, `^\.gitignore$` | `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `.gitattributes`, `.gitignore` |
| (tambahan spec ini, karena skrip dan workflow lahir sesudah `R`) | `scripts/**`, `.github/workflows/foundation-macos.yml` |

`validate.sh` sudah menjalankan `cargo test --workspace --locked` dan mencetak `rust tests:`, jadi `test.sh` tidak dijalankan di CI. `build-macos.sh` tetap berjalan walau `validate.sh` gagal, supaya bukti Mac tercatat. Perubahan yang hanya menyentuh dokumen tidak memicu `foundation-macos`, sama dengan aturan `R` untuk gerbang lokal. Yang sengaja tidak ada: cache, rahasia, token tulis, artefak unggahan, runner ARM, runner Intel macOS, jadwal `schedule`, dan pipeline rilis (di luar fase fondasi menurut `docs/quality/README.md`).

### Laporan fondasi

Path: `docs/quality/laporan-fondasi.md` (keputusan D-10, diterima lead 2026-11-15). `docs/quality/README.md` bagian "Laporan ketika fondasi selesai" mendapat satu kalimat di akhir: `Laporan yang sedang berjalan ada di [laporan fondasi](laporan-fondasi.md).`

Heading dan baris pembuka, berurutan:

```text
# Laporan fondasi
**Status fondasi:** belum selesai
**Diperbarui:** <YYYY-MM-DD> pada commit `<sha 40 digit>`
## Aturan status
## Hasil per target
## Tiga integrasi wajib
## Status kriteria
## Perintah build dan uji
## Versi teknologi
## CI
## Kegagalan dan keterbatasan
## Host dan target yang masih dibutuhkan
```

**Aturan status** (disalin apa adanya ke bagian `## Aturan status` laporan):

* **S-1** Status sel adalah satu dari `PASS`, `FAIL`, `NOT_RUN`, `BLOCKED`, `tidak didukung`, atau `tidak berlaku`. `BLOCKED` selalu menyebut `owner: <siapa>` dan apa yang ditunggu.
* **S-2** `PASS` dan `FAIL` hanya ditulis bila ada catatan tertaut yang memuat OS, arsitektur, versi, commit, perintah, dan keluaran aktual. Hasil yang diharapkan tanpa run tetap `NOT_RUN`. Run dengan baris `commit:` berakhir `(dirty)` bukan bukti.
* **S-3** Build yang berhasil hanya membuktikan kompilasi di target itu. Kolom "Jendela" baru `PASS` dari sesi desktop nyata, bukan dari runner CI.
* **S-4** Satu kriteria `PASS` hanya bila setiap OS yang disebut teks kriteria itu sudah `PASS`. Kriteria yang teksnya tidak menyebut OS cukup dengan bukti Mac, dan langkah host lain yang masih terbuka ditulis di kolom "Yang masih terbuka". Selain itu statusnya yang terberat dari `FAIL`, `BLOCKED`, `NOT_RUN`.
* **S-5** `**Status fondasi:**` menjadi `selesai` hanya bila ketiga baris `## Tiga integrasi wajib` dan ke 32 baris `## Status kriteria` bernilai `PASS`. Orang yang mengubahnya mencatat tanggal dan commit di baris `**Diperbarui:**`.

**Hasil per target**: kolom `Target | Host | Skrip bersama | Uji | Build native | Jendela | CI | Catatan`. Enam baris, dengan sel pertama diawali nama OS: `macOS arm64`, `macOS x86_64`, `Windows x86_64`, `Windows ARM64`, `Linux x86_64`, `Linux aarch64`.

**Tiga integrasi wajib**: kolom `OS | Bukti yang dibutuhkan | Langkah | Status | Bukti`.

| OS | Bukti yang dibutuhkan (`docs/quality/README.md` "Bukti native per host", dan rujukan platform) | Langkah |
| --- | --- | --- |
| macOS | Jendela SwiftUI terbuka, nama dikirim lewat binding UniFFI, label menampilkan hasil Rust, lalu integrasi token diperiksa ([rujukan macOS](../../../platforms/macos/README.md), baris "Integrasi" dan "Penggunaan nyata") | `F004-1`, `F004-2`, `F008-2`, `F009-1` |
| Windows | Jendela WinUI 3 di sesi desktop: input dan event bertipe, hasil panggilan Core, tema, dan penutupan ([rujukan Windows](../../../platforms/windows/README.md), bagian "Bukti build, runtime dan distribusi") | Scope 005 dan 006 (spec Windows belum ada) |
| Linux | Aplikasi GTK dalam sesi Wayland native, hasil Core dan token di jendela, uji X11 bila tersedia ([rujukan Linux](../../../platforms/linux/README.md), baris "Integrasi" dan "Sesi grafis") | Scope 007 (spec Linux belum ada) |

**Status kriteria**: kolom `No. | Status | Bukti | Yang masih terbuka`, satu baris per kriteria 1 sampai 32, sel pertama berupa angka saja, sama dengan tabel `docs/quality/README.md`.

**Perintah build dan uji**: kolom `Skrip | macOS arm64 | Windows x86_64 | Linux x86_64 | CI`, satu baris per skrip selain `lib.sh`, sel pertama berupa nama skrip dalam backtick. Skrip Darwin `arm64` bernilai `tidak berlaku` di kolom Windows dan Linux. `build-rust.sh`, `test.sh`, `generate-design-tokens.sh`, dan `generate-bindings.sh` bernilai `tidak berlaku` di kolom CI. Sel `PASS` atau `FAIL` mengutip baris penutup skrip.

**Versi teknologi**: per host yang punya run, baris kepala run terakhir. Untuk Mac juga rujukan tabel **Kombinasi yang dipakai** spec `macos/0001`; untuk runner juga versi citra dari log CI.

**CI**: path kedua workflow, nama tiga job, runner, URL run terakhir dan hasil tiap job, atau `NOT_RUN` sampai `F012-2`.

**Kegagalan dan keterbatasan**: satu butir per sel `FAIL`, per langkah manual yang `NOT_RUN` atau `BLOCKED`, dan per nilai sementara di "Nilai dari user" file langkah manual koordinasi yang masih dipakai, masing masing dengan rujukannya.

**Host dan target yang masih dibutuhkan**: satu butir per sel `BLOCKED` di `## Hasil per target`, dengan owner dan apa yang ditunggu.

**Pengisian pertama** (langkah 5 Build plan), dari keadaan repo pada commit itu:

| Sel | Isi pertama |
| --- | --- |
| `macOS arm64`: Skrip bersama, Uji, Build native | Hasil run lima skrip Mac (`build-rust.sh`, `test.sh`, `validate.sh`, `generate-design-tokens.sh`, `build-macos.sh`) di Mac koordinasi, dengan tautan ke bagian `verify.md` spec ini; `generate-bindings.sh` dicatat di tabel "Perintah build dan uji" |
| `macOS arm64`: Jendela | `PASS` atau `FAIL` hanya bila kolom "Hasil" `F004-1` sudah berisi hasil; selain itu `NOT_RUN`, `owner: user`, `F004-1`, `F004-2`, `F008-2`, `F009-1` |
| `macOS x86_64`, semua kolom | `tidak didukung` (spec `macos/0001`, tabel **Kombinasi yang dipakai**) |
| `Windows x86_64`, `Linux x86_64` | Skrip bersama dan Uji: `BLOCKED`, `owner: user`, `F012-1`. Build native dan Jendela: `BLOCKED`, `owner: user`, scope 005 sampai 007 |
| `Windows ARM64`, `Linux aarch64` | `BLOCKED`, `owner: user`: host atau runner ARM belum ada (Follow-up visibilitas repo) |
| Kolom CI semua baris | `NOT_RUN` sampai `F012-2`; `tidak berlaku` untuk baris tanpa job |
| `## Status kriteria`, tiap kriteria | Ikuti aturan **S-4** dari bukti spec pemilik: scope di kolom ketiga tabel `docs/quality/README.md` menunjuk file fitur, file fitur menunjuk spec, dan bagian `verify.md` spec itu (atau catatan `docs/reviews/`) adalah buktinya. Tanpa bagian `verify.md` yang lulus: `NOT_RUN`. Kriteria 6 sampai 13 dan 24: `BLOCKED`, `owner: user`, menunggu host Windows atau Linux. Kriteria 14, 16, dan 17 menyebut ketiga OS: `BLOCKED` selama Windows dan Linux belum ada |
| `## Tiga integrasi wajib` | macOS: dari kolom "Hasil" langkah yang disebut tabel di atas (`NOT_RUN` bila kosong). Windows dan Linux: `BLOCKED`, `owner: user` |

### Pemeriksaan tautan

Sampai aturan `A-12` spec `_root/0007` masuk gerbang, tautan relatif laporan diperiksa dengan perintah ini dari folder `docs/quality/`. Perintah ini tidak boleh mencetak apa pun.

```bash
sed 's/`[^`]*`//g' laporan-fondasi.md | grep -o '](\([^) ]*\))' | sed 's/^](//; s/)$//; s/#.*//' | grep -vE '^(https?:|$)' | sort -u \
  | while read -r l; do p=$(printf '%b' "$(printf '%s' "$l" | sed 's/%/\\x/g')"); [ -e "$p" ] || echo "missing: $l"; done
```

### Daftar penerimaan per target

Satu daftar langkah yang membuktikan tiap target. Teks lengkap tiap langkah tetap di file langkah manual koordinasi; daftar ini hanya memetakan.

| Target | Langkah tanpa jendela | Langkah jendela | CI |
| --- | --- | --- | --- |
| macOS `arm64` | Enam skrip di Mac (AC-2 sampai AC-8), dijalankan sesi koordinasi | `F004-1`, `F004-2`, `F008-2`, `F009-1`; `F011-1` untuk ikon nyata | `macos (macos-26)` lewat `F012-2` |
| Windows `x86_64` | `F012-1`, yang menggantikan perintah `F001-1`, `F002-1`, `F008-1` | Menunggu spec scope 005 dan 006 | `rust (windows-2025)` lewat `F012-2` |
| Linux `x86_64` | `F012-1`, yang menggantikan perintah `F001-1`, `F002-1`, `F008-1` | Menunggu spec scope 007 | `rust (ubuntu-24.04)` lewat `F012-2` |
| Windows `ARM64`, Linux `aarch64` | Belum ada; menunggu host atau runner ARM | Menunggu spec scope 005 sampai 007 | Belum ada job |
| macOS `x86_64` | `tidak didukung` (spec `macos/0001`) | `tidak didukung` | Belum ada job |

Cara `F012-1` memenuhi tiga langkah lama:

| Langkah lama | Syarat lulusnya | Bukti di keluaran `F012-1` |
| --- | --- | --- |
| `F001-1` | `rustc 1.99.1`; `cargo build` dan `cargo test` Core lulus | Baris `rustc:` dan langkah 1 PASS; langkah `cargo build --workspace --locked` dan `cargo test --workspace --locked` PASS, keduanya mencakup `desktop-core` |
| `F002-1` | `cargo test -p desktop-codegen` lulus; baris `architecture: ok, ...` | Langkah `cargo test --workspace --locked` PASS; baris `architecture: ok, ` dari langkah `architecture check` |
| `F008-1` | `cargo build` dan `cargo test` `desktop-codegen` lulus; baris `tokens: ok, ...` | Langkah build dan test PASS; baris `tokens: ok, ` dari langkah `tokens check` |

### Value sourcing

| Aksi | Nilai | Sumber |
| --- | --- | --- |
| Baris kepala | Host, arsitektur, versi alat, commit, bersih atau tidak | `uname -s`, `uname -m`, `rustc --version`, `cargo --version`, `sw_vers -productVersion`, `xcodebuild -version`, `swift --version`, `git rev-parse HEAD`, `git status --porcelain` |
| Langkah 1 | Channel toolchain | `rust-toolchain.toml`, lewat perintah `sed` di **Kerangka `lib.sh`** |
| `validate.sh` | Daftar sumber `desktop-codegen` | Baris `^sources: ` di keluaran `cargo run -p desktop-codegen --locked` tanpa argumen (kode keluar `2`; bentuknya dikunci AC-1 spec `_root/0007`), nama dipisah `, ` |
| `validate.sh` | Daftar catatan ekspor | `git ls-files '*brand-export.txt'` |
| `validate.sh` langkah 7 | Satu satunya berkas `.rs` dengan teks sapaan | `git grep --untracked -lF 'Hello, {' -- '*.rs'`; nilai yang diharapkan dari AC-3 spec `_root/0002` |
| `validate.sh` | Baris `macos gate:` | Pola `R` spec `macos/0008` atas `git diff --name-only main...HEAD` |
| `validate.sh`, `test.sh` | Baris `rust tests:` | `target/scripts/cargo-test.log` |
| `build-macos.sh` | Label langkah 6 sampai 13; sumber di langkah 2 sampai 5 | Baris perintah `G-1` sampai `G-5` di spec `macos/0008`; empat sumber yang Follow-up Scope 012 spec `macos/0004` sampai `macos/0007` minta diperiksa sebelum `xcodebuild` |
| `build-macos.sh` | Baris `settings` dan `bundle` | Perintah di tabel baris tambahan |
| Workflow | Runner, versi dan SHA `actions/checkout` | Spec ini; SHA diperiksa dengan `git ls-remote https://github.com/actions/checkout 'refs/tags/v7*'` pada 2026-11-15 |
| `foundation-macos.yml` | Daftar `paths:` | Pola `R` spec `macos/0008`, lewat tabel **Pola `R` sebagai filter path** |
| Laporan | Status tiap sel | Tabel **Pengisian pertama**, lalu aturan **S-1** sampai **S-5** |
| Laporan | Isi baris "Tiga integrasi wajib" | `docs/quality/README.md` bagian "Bukti native per host" dan tiga rujukan platform |
| Laporan | Nomor kriteria | Tabel `docs/quality/README.md` |
| Laporan | URL run CI, citra runner | Halaman run GitHub Actions, dari kamu lewat `F012-2` |

### Invarian

* Workflow tidak punya logika sendiri. Setiap perintah build dan uji di CI adalah `bash scripts/<nama>.sh`, jadi run lokal dan run CI menjalankan hal yang sama.
* Skrip tidak mengubah keputusan spec lain. Bila perintah di spec pemilik berubah (termasuk `G-1` sampai `G-5` dan `R`), skrip, baris `macos gate:`, dan filter `paths:` ikut berubah lewat pembaruan spec ini di commit yang sama.
* `build-macos.sh` dan `validate.sh` tidak pernah menulis berkas yang dilacak git. Hanya `generate-bindings.sh` dan `generate-design-tokens.sh` yang menulis, dan hanya keluaran generatornya.
* Skrip tidak pernah meng-commit, push, mengubah konfigurasi git, menyentuh `.githooks/` atau `graphify-out/`, atau mengunduh di luar Cargo dan SwiftPM.
* Laporan tidak pernah memuat `PASS` tanpa catatan tertaut (aturan **S-2**).
* Invarian yang sudah punya perintah di spec lain tetapi belum dijalankan gerbang mana pun masuk `validate.sh` sebagai satu langkah, lewat pembaruan spec ini. Yang pertama adalah teks sapaan satu sumber (langkah 7).

### Keamanan

* Workflow hanya membaca isi repo (`contents: read`), tidak menyimpan kredensial git di checkout (`persist-credentials: false`), dan tidak memakai rahasia. Kredensial penandatanganan tetap di luar repo (spec `_root/0001`, `docs/quality/README.md`).
* Satu action pihak luar, `actions/checkout`, dipaku ke SHA commit dengan versi di komentar. Menaikkan versinya berarti memperbarui SHA dan komentar lewat pembaruan spec ini.
* `pull_request` dari fork berjalan dengan token baca saja dan tanpa rahasia. Run itu tetap memakai menit runner; batasnya diatur di pengaturan repo (langkah manual).

### Kasus gagal yang sudah diputuskan

| Kasus | Yang terjadi |
| --- | --- |
| Skrip dijalankan dengan working tree kotor | Tetap berjalan, baris `commit:` berakhir `(dirty)`, dan `G-5` di `build-macos.sh` gagal. Hasil itu bukan bukti (aturan **S-2**). |
| `cargo` atau alat Xcode tidak ada di `PATH` | Langkah itu FAIL dengan kode `127`, tercatat, dan skrip mengikuti aturan lanjut atau berhenti miliknya. |
| Toolchain lain aktif (misalnya override `rustup`) | Langkah 1 FAIL. `validate.sh` tetap menjalankan langkah lain supaya akibatnya terlihat; skrip yang berhenti tidak lanjut. |
| `desktop-codegen` tidak terbangun atau tidak mencetak `sources:` | Langkah 8 `validate.sh` FAIL dan tidak ada langkah per sumber. Langkah 4 dan 5 sudah menunjukkan sebabnya. |
| Mac Intel | `build-macos.sh` dan `generate-bindings.sh` keluar dengan kode `2`. Skrip lain berjalan. |
| Mac dengan Xcode lebih lama dari 26 | `build-macos.sh` keluar dengan kode `2` dan menyebut versinya. `generate-bindings.sh` tetap berjalan, karena tidak memakai `actool`. |
| Windows tanpa Git Bash | Prasyarat `F012-1`: Git for Windows terpasang. Belum ada versi PowerShell sampai scope 006. |
| Checkout Windows mengubah akhir baris menjadi CRLF | Skrip dan workflow tetap LF lewat `.gitattributes`. Berkas lain mengikuti pengaturan git host itu, dan `F012-1` membuktikan gerbang tetap lulus. Bila kelak ada `brand-export.txt`, baris `text eol=lf` untuknya ditambah spec OS pemilik ekspor itu (spec `_root/0005`); tanpa baris itu, pemeriksaan ekspor basi gagal di Windows. |
| Runner `macos-26` memakai Xcode yang berbeda dari Mac koordinasi | Dicatat di baris `xcode:` dan di "Versi teknologi". Kombinasi runner ditulis terpisah dari tabel **Kombinasi yang dipakai** spec `macos/0001`. Proyek memakai `objectVersion = 77`, yang terbuka di Xcode 16 ke atas. |
| `rustup` di runner terlalu lama untuk `rustup toolchain install` tanpa argumen | Langkah itu gagal dan run merah. Perbaikannya lewat pembaruan spec ini, bukan action pihak ketiga diam diam. |
| Teks `Hello, {` muncul di berkas `.rs` lain, termasuk berkas uji atau berkas yang belum dilacak | Langkah 7 `validate.sh` FAIL dan mencetak setiap berkas yang cocok. Uji yang butuh teks itu menyusunnya tanpa menulis `Hello, {` utuh, atau AC-3 spec `_root/0002` diperbarui lebih dulu. |
| Pemeriksaan ekspor basi gagal | Langkahnya FAIL dengan keluaran `diff`. Ekspor diulang di host OS itu (spec `_root/0005`, **E-6**). |
| Binding Swift di git berbeda dari keluaran generator | `build-macos.sh` FAIL di langkah 10 (`G-2`) dan mencetak petunjuk. `bash scripts/generate-bindings.sh` menulis ulang binding; kamu meninjau dan meng-commit `bindings/swift/` bersama perubahan Rust. |
| Run `main` berikutnya masuk saat run sebelumnya berjalan | Keduanya selesai, karena pembatalan hanya untuk `pull_request`. |
| Push ke `main` hanya mengubah dokumen | `foundation-macos` tidak berjalan untuk commit itu. Bukti CI Mac diambil dari commit terakhir yang memicunya, atau dari Run workflow pada commit itu (`F012-2`). |
| GitHub Actions dimatikan atau repo belum ada di GitHub | Tidak ada run. Kolom CI di laporan tetap `NOT_RUN`. |

### Skenario uji kritis

* Jalur bahagia: enam skrip di Mac pada working tree bersih, semua `PASS`, verifies **AC-2**, **AC-4**, **AC-5**, **AC-7**, **AC-8**
* Kegagalan: satu keluaran generator diubah, `validate.sh` tetap menjalankan semua langkah dan melaporkan dua langkah gagal, verifies **AC-3**
* Kegagalan: binding di working tree berbeda dari keluaran generator, `build-macos.sh` berhenti di `G-2` dengan petunjuk, verifies **AC-6**
* Salah pakai: argumen tak dikenal dan host yang salah keluar dengan kode `2`, verifies **AC-1**, **AC-9**
* Laporan: tiga integrasi belum `PASS`, status fondasi tetap `belum selesai`, verifies **AC-14**

## File yang boleh diubah tiap milestone

| Milestone | Path |
| --- | --- |
| 1 | `scripts/lib.sh`, `scripts/build-rust.sh`, `scripts/test.sh`, `scripts/validate.sh`, `.gitattributes` (dua baris), `docs/specs/_root/0008-build-dan-penerimaan-per-target/verify.md`, baris `**Status**` di `index.md` spec ini |
| 2 | `scripts/generate-design-tokens.sh`, `scripts/generate-bindings.sh`, `scripts/build-macos.sh`, `scripts/lib.sh`, `verify.md` spec ini |
| 3 | `.github/workflows/foundation.yml`, `.github/workflows/foundation-macos.yml`, `docs/quality/laporan-fondasi.md`, `docs/quality/README.md` (satu kalimat), `verify.md` spec ini |
| 4 | `docs/quality/laporan-fondasi.md`, `verify.md` spec ini |

Setiap commit juga membawa empat berkas `graphify-out/` dari hook; AC-16 mengecualikannya. `docs/quality/` biasanya terlarang bagi sub sesi, jadi milestone 3 dan 4 butuh izin eksplisit di brief developer.

## Build plan

Pendekatan build proyek adalah Tracer Bullet. Benang tipisnya di sini adalah satu skrip gerbang yang berjalan di semua host dengan bentuk keluaran final, lalu gerbang Mac menebal di atasnya, lalu CI dan laporan membungkus semuanya. Satu langkah menjadi satu commit dengan subject `type(scope): spec _root/0008 step <n>, ...`, plus commit kedua untuk `verify.md` bila perlu dengan subject yang sama. Jalankan AC sesudah commit langkahnya, karena AC-1 menuntut baris `commit:` berakhir `(clean)`. Selalu `export CARGO_PROFILE_DEV_DEBUG=0 CARGO_INCREMENTAL=0`, dan hapus `target/` serta `apps/macos/DerivedData/` sebelum melapor. `scripts/`, `.github/workflows/`, dan `.gitattributes` hanya boleh punya satu penulis per wave.

**Milestone 1: Gerbang bersama** (langkah 1 dan 2)


1. Ubah `**Status**` spec ini menjadi `In Progress`. Tulis `scripts/lib.sh` sesuai **Kerangka `lib.sh`** dan `scripts/validate.sh` sesuai **Skrip** dan **Bentuk keluaran**, tambah dua baris ke `.gitattributes`, dan commit `validate.sh` dengan mode `100755` (`git add --chmod=+x`). Lalu jalankan perintah AC-1, AC-2, dan AC-3, satisfies **AC-1**, **AC-2**, **AC-3**, **AC-15**
2. Tulis `scripts/build-rust.sh` dan `scripts/test.sh`, mode `100755`, dan commit. Jalankan AC-4, AC-15, dan AC-16 (bagian milestone 1), lalu catat perintah dan keluaran semuanya di `verify.md` bagian `## Milestone 1: Gerbang bersama`, satisfies **AC-4**, **AC-15**, **AC-16**

**Milestone 2: Skrip macOS** (langkah 3, sesudah milestone 1)

3. Tulis `scripts/generate-design-tokens.sh`, `scripts/generate-bindings.sh`, dan `scripts/build-macos.sh`, mode `100755`, dengan label `G-1` sampai `G-5` disalin dari spec `macos/0008` di `main`, dan commit. Jalankan AC-5 sampai AC-9 dan AC-15, lalu catat di `verify.md` bagian `## Milestone 2: Skrip macOS`, satisfies **AC-1**, **AC-5**, **AC-6**, **AC-7**, **AC-8**, **AC-9**, **AC-15**

**Milestone 3: Workflow dan laporan** (langkah 4 dan 5, sesudah milestone 2)

`docs/quality/` adalah folder tulisan user. Berkas baru `docs/quality/laporan-fondasi.md` dan satu kalimat di `docs/quality/README.md` butuh persetujuan user sebelum brief developer memberi izin; lead menanyakannya sebelum milestone 3 dibangun.


4. Tulis `.github/workflows/foundation.yml` dan `.github/workflows/foundation-macos.yml` persis dari bagian **Workflow** dan commit. Jalankan AC-10 dan AC-15. Jangan push: run nyata adalah langkah manual `F012-2`, satisfies **AC-10**, **AC-15**
5. Jalankan keenam skrip di Mac pada commit langkah 4. Tulis `docs/quality/laporan-fondasi.md` sesuai **Laporan fondasi** dan **Pengisian pertama**, tambah satu kalimat ke `docs/quality/README.md`, dan commit. Jalankan AC-13, AC-14, AC-16, dan **Pemeriksaan tautan**, lalu catat di `verify.md` bagian `## Milestone 3: Workflow dan laporan`, dengan langkah `F012-1` dan `F012-2` di `### UI / manual`, satisfies **AC-13**, **AC-14**, **AC-16**

**Milestone 4: Bukti host lain dan CI** (langkah 6, `BLOCKED`, `owner: user`)

6. Sesudah kamu menjalankan `F012-1` dan `F012-2`, pemeriksa mencatat keluaran di `verify.md` bagian `## Milestone 4: Bukti host lain dan CI` dan memperbarui baris Windows `x86_64`, Linux `x86_64`, dan kolom CI di laporan, satisfies **AC-11**, **AC-12**

## Langkah manual

Untuk file langkah manual koordinasi. Nilai yang hanya kamu tahu ada di Follow-up.

**`F012-1` · Jalankan skrip bersama di host Windows dan host Linux** (AC-12; menggantikan perintah `F001-1`, `F002-1`, `F008-1`)

* **Prasyarat:** milestone 1 spec ini di `main` dan di-clone di host itu. Windows `x86_64-pc-windows-msvc` dengan Git for Windows (Git Bash), `rustup`, dan MSVC Build Tools. Linux `x86_64-unknown-linux-gnu` dengan `bash`, `git`, `rustup`, dan `cc`. Tanpa pustaka UI.
* **Jalankan:** dari akar repo, di Git Bash atau shell Linux:
  1. `rustup toolchain install`
  2. `bash scripts/build-rust.sh`
  3. `bash scripts/validate.sh`
  4. `bash scripts/test.sh`
* **Lulus bila:** perintah 2 sampai 4 keluar dengan kode `0` dan baris terakhirnya `== <nama>.sh: PASS, ...`; baris `rustc:` diawali `rustc 1.99.1 `; keluaran perintah 3 memuat `tokens: ok, ` dan `architecture: ok, `; keluaran perintah 3 dan 4 memuat `rust tests: <p> passed, 0 failed`. Kirim seluruh keluaran ketiga skrip per host.
* **Hanya langkah ini yang bisa membuktikan:** skrip, gerbang, dan pemeriksaan teks memberi hasil yang sama di luar macOS.

**`F012-2` · Push dan jalankan workflow GitHub Actions** (AC-11)

* **Prasyarat:** milestone 3 spec ini di `main`. Repo GitHub `origin` ada dan GitHub Actions diizinkan untuk repo itu (Settings, Actions, General: izinkan action milik GitHub). Untuk repo privat, menit runner macOS dihitung lebih mahal; atur batas pemakaian bila perlu.
* **Jalankan:** kirim `main` ke `origin` mengikuti aturan 4 di `AGENTS.md`. Buka tab Actions dan tunggu run workflow `foundation` untuk commit itu. Untuk `foundation-macos`, tunggu run dari push itu; bila tidak ada (push tidak menyentuh path filternya), pilih workflow `foundation-macos`, Run workflow, cabang `main`.
* **Lulus bila:** ketiga job berhasil pada commit yang sama. Kirim URL run, hasil tiap job, versi citra runner dari langkah "Set up job", serta baris kepala dan baris penutup tiap skrip dari log.
* **Hanya langkah ini yang bisa membuktikan:** workflow benar benar berjalan di runner GitHub, termasuk Windows dan Linux.

## Consequences

**Positive**:

* Satu perintah per kebutuhan di setiap host, dengan keluaran yang sudah berbentuk bukti. Catatan `verify.md` dan laporan bisa menempel keluaran apa adanya.
* CI dan run lokal sama, karena workflow hanya memanggil skrip. Kegagalan CI bisa diulang dengan perintah yang sama di mesinmu.
* Gerbang Mac dari spec `macos/0008` berjalan otomatis di runner macOS tanpa ditulis ulang.
* Sumber `desktop-codegen` baru ikut ke gerbang tanpa mengubah skrip.
* Tiga langkah manual host lama menjadi satu run skrip per host.

**Negative / tradeoffs**:

* `bash` di Windows butuh Git Bash. Developer Windows yang hanya punya PowerShell belum punya jalur sampai scope 006.
* Menulis untuk bash `3.2` berarti tanpa array dan tanpa fitur bash modern, dan kesalahan hanya tertangkap saat skrip dijalankan dengan `/bin/bash`.
* Workflow dipaku ke `macos-26` dan Xcode bawaannya, yang berbeda dari Xcode `27.1` di Mac koordinasi. Bukti CI macOS dan bukti Mac lokal adalah dua kombinasi, dan label runner perlu diperbarui saat GitHub memensiunkannya.
* Tanpa cache, setiap run membangun dari nol. Run lebih lambat dan memakai lebih banyak menit, terutama di macOS.
* `validate.sh` menjalankan `cargo build` dan `cargo test` yang juga dijalankan `build-rust.sh` dan `test.sh`, dan `architecture check` berjalan dua kali (di uji dan sebagai langkah sendiri). Di Mac lokal, menjalankan semua skrip berarti mengulang sebagian kerja.
* Laporan fondasi diperbarui tangan. Ia bisa tertinggal dari bukti terbaru bila orang yang mencatat bukti lupa memperbaruinya.
* Perintah yang dibungkus tertulis di dua tempat (spec pemilik dan skrip), begitu juga pola `R`. Perubahan di spec pemilik perlu pembaruan spec ini juga.
* Mengambil daftar sumber dari pesan usage bergantung pada bentuk baris `sources:`. AC-1 spec `_root/0007` menguncinya, tetapi itu tetap kontrak lewat teks.

**Neutral**:

* `scripts/update_graphify.py` tetap di folder yang sama dan tidak disentuh atau dipanggil.
* Baris `macos gate:` di CI selalu `unknown`, karena checkout CI tidak memuat cabang `main` lokal. Di CI keputusan itu diambil filter `paths:`.
* `/develop` menulis `verify.md` saat build. Spec ini tidak membuatnya.

## Follow-up

* [x] Jawaban atas Follow-up Scope 012 di spec `_root/0001`: skrip dan CI membungkus perintah yang sama dengan `--locked` dan memeriksa `rust-toolchain.toml` di langkah 1; `[profile.release]` tidak dibutuhkan.
* [x] Jawaban atas Follow-up Scope 012 di spec `_root/0003`: `generate-design-tokens.sh` membungkus `tokens generate`; `validate.sh` menjalankan `check` setiap sumber, juga di ketiga runner.
* [x] Jawaban atas Follow-up Scope 012 di spec `_root/0004`: `validate.sh` dan CI menjalankan `config check`. Penyimpanan rahasia CI untuk kredensial penandatanganan tidak dibuat di fase ini (workflow tanpa rahasia); itu milik spec distribusi kelak.
* [x] Jawaban atas Follow-up Scope 012 di spec `_root/0005`: `validate.sh` menjalankan `assets check` dan pemeriksaan ekspor basi untuk tiap `brand-export.txt` yang dilacak.
* [x] Jawaban atas Follow-up Scope 012 di spec `_root/0006`: `validate.sh` dan CI menjalankan `localization check` dan `cargo test --workspace --locked`.
* [x] Jawaban atas Follow-up Scope 012 di spec `_root/0007`: `validate.sh` menjalankan empat perintah gerbang; baris `Di CI` di tabel "Kapan pemeriksaan berjalan" dipenuhi job `rust` dan `macos`, dengan perbandingan binding lewat `G-2` di `build-macos.sh`.
* [x] Jawaban atas Follow-up Scope 012 di spec `macos/0001`: `generate-bindings.sh` membungkus **Membuat ulang binding**, `build-macos.sh` membungkus **Perintah build dan uji** lewat `G-1` sampai `G-4` spec `macos/0008`; hasil per arsitektur tercatat di baris `bundle <K>:`; `MARKETING_VERSION` dijaga `config check` (spec `macos/0004`) yang berjalan sebelum `xcodebuild`.
* [x] Jawaban atas Follow-up Scope 012 di spec `macos/0004`: `build-macos.sh` menjalankan `config check` sebelum `xcodebuild` dan mencatat nilai AC-3 dan AC-4 di baris `settings <K>:` dan `bundle <K>:`.
* [x] Jawaban atas Follow-up Scope 012 di spec `macos/0006`: `build-macos.sh` menjalankan `localization check` (langkah 4) sebelum `xcodebuild`.
* [x] Jawaban atas Follow-up Scope 012 di spec `macos/0007`: runner `macos-26` dengan Xcode bawaan 26.6, dan `build-macos.sh` menjalankan `assets check` (langkah 5) sebelum `xcodebuild`. Baris `xcode:` di setiap run mencatat versi yang benar benar dipakai.
* [x] Jawaban atas Follow-up Scope 012 di spec `macos/0005`: `generate-design-tokens.sh` membungkus `tokens generate`; runner macOS menjalankan `tokens check` (lewat `validate.sh` dan langkah 2 `build-macos.sh`), `swift test` (`G-3`), dan `xcodebuild` (`G-4`) lewat `build-macos.sh`.
* [x] Jawaban atas Follow-up Scope 012 di spec `macos/0008`: `build-macos.sh` membungkus `G-1` sampai `G-5` apa adanya, tanpa pemeriksaan binding lain; job macOS memakai `R` sebagai filter `paths:` (ditambah `scripts/**` dan berkas workflow itu); build `Release` sesudah `G-5` diputuskan di sini atas AC-4 spec `macos/0001`, tanpa pemicu tambahan.
* [ ] Lead: tandai baris Follow-up Scope 012 di spec `_root/0001`, `_root/0003` sampai `_root/0007`, `macos/0001`, dan `macos/0004` sampai `macos/0008` sebagai dijawab spec ini.
* [x] Path laporan fondasi diputuskan lead (D-10, opsi 1): `docs/quality/laporan-fondasi.md`, dengan satu kalimat tautan di `docs/quality/README.md`.
* [ ] Lead: persetujuan user untuk berkas baru di `docs/quality/` dan satu kalimat di `docs/quality/README.md`, sebelum milestone 3 dibangun; sesudahnya izin eksplisit di brief developer milestone 3 dan 4.
* [ ] Lead: di file langkah manual, catat bahwa keluaran `F012-1` memenuhi `F001-1`, `F002-1`, dan `F008-1`, sehingga ketiganya bisa ditutup dari satu run per host.
* [ ] Scope 006 (Windows): `scripts/build-windows.ps1` atau perluasan skrip bash untuk build WinUI, job Windows yang membangun aplikasi, dan bukti jendela untuk baris Windows di "Tiga integrasi wajib". Menunggu host Windows.
* [ ] Scope 007 (Linux): `scripts/build-linux.sh`, pemasangan pustaka GTK 4 dan libadwaita di job Linux, uji tanpa layar yang mungkin, dan bukti Wayland serta X11 untuk baris Linux. Menunggu host Linux.
* [ ] Spec OS scope 011: baris `text eol=lf` untuk `brand-export.txt` di `.gitattributes` saat ekspor biner pertama dibuat (spec `_root/0005`), supaya pemeriksaan ekspor basi di `validate.sh` lulus di Windows.
* [ ] Nilai dari user: visibilitas repo GitHub (publik atau privat). Bila publik, runner `ubuntu-24.04-arm` dan `windows-11-arm` gratis dan bisa ditambah ke matriks `rust` lewat pembaruan spec ini untuk bukti Rust di `aarch64` dan `ARM64`. Bila privat, runner itu berbayar per menit.
* [ ] `/sync` atau lead: `AGENTS.md` bagian Commands bisa menyebut `bash scripts/validate.sh` dan `bash scripts/build-macos.sh` sebagai cara satu perintah menjalankan gerbang, dan `test-preferences.json` bisa menyebut `bash scripts/test.sh`.
* [ ] User (`.githooks/` terlarang bagi sub sesi, sama dengan Follow-up spec `_root/0007`): hook `pre-push` yang memanggil `bash scripts/validate.sh` hanya dipasang atas keputusanmu.
* [ ] Saat GitHub memensiunkan `macos-26`, `ubuntu-24.04`, atau `windows-2025`, atau saat `actions/checkout` naik versi major, perbarui blok **Workflow** spec ini dan kedua berkasnya bersama.
