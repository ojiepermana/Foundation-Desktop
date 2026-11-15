# Generator dan pemeriksaan (`desktop-codegen`)

## Overview

Paket `desktop-codegen` memvalidasi sumber bersama dan menulis ulang berkas turunannya. Ia juga menjalankan pemeriksaan arsitektur. Setiap sumber punya dua perintah: `check` membandingkan keluaran dengan berkas di disk byte demi byte, dan `generate` menulis yang hilang atau berbeda. Dependensinya hanya `serde` dan `serde_json`.

## Key files

| File | Owns |
|---|---|
| `Cargo.toml` | Manifest paket. Dependensi langsung dikunci aturan `A-3` spec `_root/0007`. |
| `src/lib.rs` | `Source`, `Output`, `Problem`, `Plan`, `check`, `generate`, dan larik `SOURCES`. Sumber baru didaftarkan di akhir larik itu. |
| `src/main.rs` | Biner `desktop-codegen <source> <check|generate>`. Kode keluar 0 lulus, 1 gagal, 2 salah pakai. |
| `src/json.rs` | Penulis JSON kanonis untuk keluaran. |
| `src/tokens/` | Sumber `tokens` (spec `_root/0003`). `swift.rs` menulis `apps/macos/DesignSystem/Tokens.swift` (spec `macos/0005`). |
| `src/config/` | Sumber `config` (spec `_root/0004`). `macos.rs` menulis `apps/macos/Config/Shared.xcconfig` (spec `macos/0004`). |
| `src/localization/` | Sumber `localization` (spec `_root/0006`). `macos.rs` menulis `Localizable.strings` per bahasa (spec `macos/0006`). |
| `src/assets/` | Sumber `assets` (spec `_root/0005`). `macos.rs` menulis `apps/macos/Resources/AppIcon.icon/` (spec `macos/0007`). |
| `src/architecture/` | Sumber `architecture`: aturan `A-1` sampai `A-13` (spec `_root/0007`) dan `C-1` sampai `C-4` di `macos.rs` (spec `macos/0008`). Tidak punya keluaran. |
| `tests/` | Uji kontrak per sumber (`*_contract.rs`, milik developer) dan uji aturan (`*_rules.rs`, milik tester). `workspace_layout.rs` dan `macos_bridge_ownership.rs` membaca berkas di luar paket ini. |

## Commands

Jalankan dari root repo.

```bash
cargo test -p desktop-codegen --locked

# Satu sumber: tokens, config, localization, assets, atau architecture
cargo run -p desktop-codegen --locked -- tokens check
cargo run -p desktop-codegen --locked -- tokens generate

# Semua gerbang bersama, termasuk check tiap sumber (spec _root/0008)
bash scripts/validate.sh
```

## Conventions

- Bentuk alat (`Source`, `Plan`, baris ringkasan, kode keluar) ditetapkan spec `_root/0003`. Sumber baru mengikuti bentuk itu dan butuh spec pemiliknya lebih dulu.
- Keluaran mengikuti aturan O-1 sampai O-7 spec `_root/0003`: penanda di awal berkas, isi kanonis dan deterministik, akhir baris LF, dan satu baris `text eol=lf` per jenis berkas di `.gitattributes`.
- Sumber yang rusak tidak menulis apa pun. `generate` hanya menulis berkas yang hilang atau berbeda.
- Emitter per OS tinggal di modul sumbernya (`macos.rs`, `swift.rs`) dan menulis ke `apps/<os>/`. Spec OS yang memilikinya menetapkan path dan isinya.
- Aturan arsitektur baru masuk lewat pembaruan spec pemiliknya, bersama kasus ujinya. ID aturan dan pesan mengikuti tabel aturan di spec itu.
- Uji menyalin repo ke `CARGO_TARGET_TMPDIR` lalu mengubah salinannya. Uji tidak memanggil `cargo`, `swift`, `xcodebuild`, `xcrun`, atau `diff` (spec `macos/0008`, AC-5).
- Kode, rustdoc, nama uji, dan pesan alat berbahasa Inggris (spec `_root/0007`, `H-6`).

## Gotchas

- `cargo test --workspace --locked` menjalankan `check` setiap sumber lewat `every_source_passes_check_on_the_repo`. Menyunting sumber tanpa `generate`, atau menyunting keluaran dengan tangan, menggagalkan gerbang.
- `architecture check` memeriksa setiap tautan Markdown di repo (kecuali `docs/scope/coordination/`) dan rujukan di tiap `AGENTS.md`, termasuk berkas ini. Path atau nomor spec yang salah di sini menggagalkan `cargo test`.
- Pola format sapaan hanya boleh ada di `crates/core/src/greeting.rs`. Kasus uji yang butuh teks itu menulisnya dengan `concat!`, supaya `git grep --untracked -lF 'Hello, {' -- '*.rs'` tetap mencetak satu baris.
- `every_output_follows_the_output_rules` di tiap `*_rules.rs` berjalan atas setiap keluaran `Plan`. Emitter baru harus lulus uji itu tanpa mengubahnya.
- Perubahan di paket ini cocok dengan pola `R` spec `macos/0008`, jadi gerbang Mac `G-1` sampai `G-5` ikut dijalankan sebelum merge.

## Related specs

- [0003. Tema bersama dan kontrol native](../../docs/specs/_root/0003-tema-bersama-dan-kontrol-native/index.md)
- [0004. Konfigurasi publik](../../docs/specs/_root/0004-konfigurasi-publik/index.md)
- [0005. Aset merek bersama](../../docs/specs/_root/0005-aset-merek-bersama/index.md)
- [0006. Satu contoh lokalisasi](../../docs/specs/_root/0006-satu-contoh-lokalisasi/index.md)
- [0007. Aturan pengembangan dan pemeriksaan arsitektur](../../docs/specs/_root/0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md)
- [0008. Build dan penerimaan per target](../../docs/specs/_root/0008-build-dan-penerimaan-per-target/index.md)

_Drafted by /sync from the introducing change, worth a quick human pass._
