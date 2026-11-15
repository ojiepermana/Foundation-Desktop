# Log koordinasi: `000 - Panduan scope`

- **Scope:** `docs/scope/000 - Panduan scope.md` (panduan tanpa kotak); kotak kerja ada di file fitur `docs/scope/001 …` sampai `012 …`, cakupan run ini jalur Mac
- **Hash awal:** `0572298` · **Hash awal wave aktif:** `9b67c13`
- **Mode keputusan:** `auto` · **Ketentuan tambahan:** tidak ada
- **Aturan umum brief:** `000 - Panduan scope.brief-umum.md` · **Langkah manual:** `000 - Panduan scope.manual.md`
- **Wave aktif:** `1`, mulai `2026-11-15`
- **Diperbarui:** `2026-11-15 17:50`

## Sub sesi wave `1`

| Nama | Peran | Item | Scope | Ownership | Branch | Model | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `arc-f001-spec` | architect | 001 spec `_root` 0001 | `001 - Batas workspace dan pemilik.md` | `docs/specs/_root/0001-batas-workspace-dan-pemilik/` | `claude/arc-f001-spec-a1f824` | `fable-5-2 · high` | aktif |
| `arc-f003-spec` | architect | 003 spec `_root` 0002; ringkasan diterima lead (`82db3f0`), menunggu laporan akhir | `003 - Kontrak sapaan bersama.md` | `docs/specs/_root/0002-kontrak-sapaan-bersama/` | `claude/arc-f003-spec-14125c` | `fable-5-2 · high` | aktif |
| `arc-f004-spec` | architect | 004 spec macOS, plus penerapan macOS 001 dan 003; nomor ditetapkan saat "mulai" | `004 - Sapaan di jendela macOS.md` | `docs/specs/macos/` (folder per spec, ditetapkan saat "mulai") | `claude/arc-f004-spec-de734c` | `fable-5-2 · high` | standby |
| `dev-f001-m1` | developer | 001 Build it; milestone dari blok usulan spec `_root` 0001 | `001 - Batas workspace dan pemilik.md` | ditetapkan saat "mulai" dari Build plan | `claude/dev-f001-m1-c305cb` | `opus-5-5 · medium` | standby |
| `dev-f003-m1` | developer | 003 Build it; milestone dari blok usulan spec `_root` 0002 | `003 - Kontrak sapaan bersama.md` | ditetapkan saat "mulai" dari Build plan | `claude/dev-f003-m1-f164e9` | `opus-5-5 · medium` | standby |

## Keputusan

| Tanggal | Keputusan | Oleh |
| --- | --- | --- |
| 2026-11-15 | Mode keputusan `auto`; tidak ada ketentuan tambahan di argumen (`Panduan scope`). | lead |
| 2026-11-15 | Argumen cocok dengan `000 - Panduan scope.md` yang tidak punya kotak. Cakupan run: jalur Mac dari Panduan, yaitu spec `_root` dan macOS 001 dan 003, alur 004, `/audit` 002 sesudah scaffold, lalu bagian `_root` dan macOS 008 sampai 012. Fitur 005, 006, 007 dicatat terblokir host. | user |
| 2026-11-15 | Hanya `.claude/skills/coordinate/` yang di-commit ke `main` (`0572298`); `rust-best-practices` dan `rust-async-patterns` tetap untracked, jadi brief developer menunjuk keduanya lewat path absolut checkout utama. | user |
| 2026-11-15 | Penyesuaian skill ke repo ini: kotak memakai `* [ ]`; indeks scope adalah `scope.md`; spec bersama di `docs/specs/_root/`, penerapan macOS di `docs/specs/macos/`; nomor spec berurutan per folder (Panduan), ditetapkan lead: `_root/0001` = scope 001, `_root/0002` = scope 003. Tidak ada log koordinasi lain, jadi tidak ada keputusan lintas scope yang dibawa. | lead |
| 2026-11-15 | Tier Beta (Panduan): kotak penutup tiap fitur `Verify it` lalu `Test it`; tanpa `Review it` dan `Document it`, jadi tidak ada sesi review cadangan. Cross check spec = `Another model`. | lead |
| 2026-11-15 | Penerapan macOS fitur bersama (001, 003, 008 sampai 012) menjadi kotak baru yang lead tambahkan di file fitur sesudah spec `_root`-nya di-merge (Panduan: "Kelak, `/architect scope 008 … untuk macOS`"). Fitur bersama tetap `in-progress` di akhir run ini, karena Panduan mensyaratkan bukti semua target yang berlaku; Windows dan Linux tercatat di Blocker terbuka. | lead |
| 2026-11-15 | Gerbang merge: untuk branch yang hanya berisi spec, cek worktree bersih, diff hanya ownership plus `graphify-out/`, tidak ada penanda konflik, dan tautan relatif spec mengarah ke file yang ada. Sesudah Cargo Workspace ada di `main`: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings`, `cargo test -p <crate>`, plus perintah Build plan milestone itu. `cargo xtask …` tidak ada di repo ini. | lead |
| 2026-11-15 | Graph: hook `.githooks/pre-commit` memperbarui `graphify-out/` pada setiap commit, juga di worktree (aturan `AGENTS.md`), jadi lead tidak menjalankan `graphify update` terpisah. Konflik `graphify-out/` saat merge diselesaikan lead dengan mengambil satu sisi lalu `git commit`, karena hook membuat ulang graf. Ekstraksi semantik (`/graphify . --update`) hanya untuk file spec yang di-merge, di akhir wave; file koordinasi cukup lewat hook. | lead |
| 2026-11-15 | Rekonsiliasi: tidak ada worktree atau branch lain di repo ini; 19 sesi peer di `ListAgents` semuanya milik proyek lain (Stackly, Pool, Mobile) dan tidak dipetakan ke item mana pun. Lima di antaranya `busy` di Mac yang sama, jadi paling banyak 4 sub sesi berjalan di sini dan 2 build Rust penuh bersamaan. Tidak ada kode di `main`, jadi tidak ada kotak yang artefaknya sudah ada. | lead |
| 2026-11-15 | Nama sub sesi memakai nomor scope tiga digit (`arc-f001-spec`, `dev-f003-m1`), karena `ListAgents` di Mac ini sudah memuat sesi proyek lain bernama `arc-f4-spec`, `dev-f4-m1`, dan sejenisnya. | lead |
| 2026-11-15 | Worktree sub sesi dibuat aplikasi di `/Users/ojiepermana/worktrees/desktop/<name>-<hash>` dan dipotong dari `origin/main` (`60fb6a3`), bukan `main` lokal. Kelima worktree wave 1 sudah di-fast-forward ke `9b67c13` sebelum ada perubahan di dalamnya. Sesi berikutnya dimulai dengan `source_branch: main`, dan brief selalu membuka dengan `git merge main`. | lead |
| 2026-11-15 | Ringkasan "Accept this spec?" spec `_root/0002` dibalas `accept`: cross check (Opus) sudah diterapkan, kriteria 2, 3, 22 dan Done when scope 003 tercakup AC-1 sampai AC-7, belum ada spec `Accepted` lain, tautan relatif sah. Usul tautan dari `docs/contracts/README.md` tidak dikerjakan lead (di luar file lead) dan tetap sebagai Follow-up spec. | lead |

## Keputusan delegasi

Keputusan spec yang diambil dari opsi rekomendasi atas nama user (SKILL.md bagian "Mode keputusan"). Baris `berat` di atas.

| Tanggal | Spec · AC | Pertanyaan | Diambil | Runner-up | Berat | Cara membatalkan | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 2026-11-15 | `_root/0002` · AC-4, AC-5 | Perilaku masukan tepi dan validasi `hello` | Fungsi total: setiap `String` diterima apa adanya, tanpa validasi, pemangkasan, batas panjang, atau kesalahan; sesuai bentuk di `docs/contracts/README.md` | `Result<String, HelloError>` dengan pangkas dan tolak kosong | berat | `architect (update)` spec `_root/0002`; tanda tangan, AC-4, AC-5, V-2 sampai V-4, R-4, langkah 1 sampai 4 dibangun ulang, dan `docs/contracts/README.md` ikut berubah | berlaku |
| 2026-11-15 | `_root/0002` · R-8 | Aturan pemanggil saat kolom nama kosong | UI mengirim isi kolom apa adanya, juga saat kosong | UI boleh menonaktifkan tombol saat kosong | - | `architect (update)` spec `_root/0002`; R-8 dan Follow-up macOS berubah | berlaku |
| 2026-11-15 | `_root/0002` · AC-3, AC-4 | Letak dan path publik `hello` | Modul privat `greeting`, ekspor ulang di akar crate | `hello` langsung di `lib.rs` | - | `architect (update)` spec `_root/0002`; langkah 1 dibangun ulang | berlaku |
| 2026-11-15 | `_root/0002` · R-5 | Versi kontrak | Versi paket `desktop-core` mengikuti SemVer plus tabel riwayat di spec | Tanpa versi sampai rilis pertama | - | `architect (update)` spec `_root/0002` | berlaku |
| 2026-11-15 | `_root/0002` · AC-4 | Penjaga tanda tangan | Uji `signature_is_pinned` saat kompilasi | `cargo-semver-checks` atau snapshot `cargo public-api` | - | `architect (update)` spec `_root/0002`; langkah 2 dibangun ulang | berlaku |
| 2026-11-15 | `_root/0002` · AC-5 | Tempat vektor kontrak | Tabel normatif di spec dan tabel di `crates/core/tests/hello_contract.rs` | Fixture bersama di `tests/fixtures/` | - | `architect (update)` spec `_root/0002`; langkah 4 dibangun ulang | berlaku |
| 2026-11-15 | `_root/0002` · AC-2 | Alat uji | Harness `cargo test` tanpa dependensi dev | `proptest` | - | `architect (update)` spec `_root/0002`; langkah 3 dan 4 dibangun ulang | berlaku |
| 2026-11-15 | `_root/0002` · R-2 | Siapa yang boleh menambah API publik Core | Siapa pun, lewat spec `_root` lebih dulu | Perubahan aditif cukup tinjauan kode | - | `architect (update)` spec `_root/0002` | berlaku |
| 2026-11-15 | `_root/0002` · AC-3 | Bukti satu implementasi | `git grep` pola format plus uji bernilai literal; pemeriksaan tetap diserahkan ke scope 002 | Skrip di `scripts/` sekarang | - | `architect (update)` spec `_root/0002`; langkah 6 dibangun ulang | berlaku |
| 2026-11-15 | `_root/0002` · AC-6, AC-7 | Tempat lint Core | `[lints.rust]` di `crates/core/Cargo.toml` (`unsafe_code = "forbid"`, `missing_docs = "warn"`) | Atribut `#![...]` di `lib.rs` | - | `architect (update)` spec `_root/0002`; langkah 5 dibangun ulang | berlaku |
| 2026-11-15 | `_root/0002` · tidak ada AC | Badan fungsi | `format!("Hello, {name}!")` | Menyisipkan ke buffer `name` | - | `architect (update)` spec `_root/0002` | berlaku |

## Blocker terbuka

- 005 · `/architect scope 005 kelayakan WinUI dari Rust untuk Windows`: hardware, host Windows dengan Windows App SDK; pemilik user. Di luar cakupan run ini.
- 006 · `/architect scope 006 sapaan di jendela Windows` ← `005 - Kelayakan WinUI dari Rust.md`: hasil 005, dan host Windows; pemilik user.
- 007 · `/architect scope 007 sapaan di jendela Linux`: hardware, host Linux dengan sesi Wayland; pemilik user.
- 001, 002, 003, 008 sampai 012 · penerapan Windows dan Linux: host masing masing; pemilik user. Fitur bersama tidak ditandai `done` sebelum bukti itu ada.
- 002 · `/audit` ← `001 - Batas workspace dan pemilik.md`: scaffold (`Build it` 001) harus sudah di `main`.
- 004 · `/architect scope 004 sapaan di jendela macOS` ← spec `_root` 0001 dan 0002 di-merge.
- 008 sampai 012 · `/architect scope … untuk _root` ← alur 004 terbukti di Mac (urutan Panduan).

## Dicentang pada wave ini

- belum ada

## Riwayat wave

- belum ada

## Penutupan

- belum dimulai
