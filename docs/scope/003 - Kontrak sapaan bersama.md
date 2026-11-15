# 003 - Kontrak sapaan bersama

Dasar · in-progress

Tetapkan satu fungsi publik dengan masukan nama dan hasil sapaan. Bagian spec tiap OS menjelaskan bagaimana UI memanggil fungsi itu. [Kontrak Rust](../contracts/README.md)

**Done when:** Uji Core tanpa UI membuktikan `hello("Ayu".to_string())` menghasilkan `Hello, Ayu!` dari satu implementasi.

**Pointer:** spec bersama [_root/0002](../specs/_root/0002-kontrak-sapaan-bersama/index.md) · code in `crates/core/src/greeting.rs`, `crates/core/tests/hello_contract.rs`

* [x] `/architect scope 003 kontrak sapaan untuk _root` (spec)
* [x] Build it: `/develop scope 003 kontrak sapaan untuk _root`
  * [x] Milestone 1: fungsi `hello` dan uji Done when. Build plan langkah 1 dan 2. AC-1, AC-2, AC-4. needs 001 milestone 1 "Workspace dan kerangka Core" (spec _root/0001 Build plan langkah 1 dan 2)
  * [x] Milestone 2: masukan tepi, aturan kontrak, dan bukti. Build plan langkah 3 sampai 6. AC-2, AC-3, AC-4, AC-5, AC-6, AC-7. after milestone 1
* [x] Verify it: `/check verify scope 003 kontrak sapaan untuk _root`
* [ ] Test it: `/test scope 003 kontrak sapaan untuk _root`
* [ ] `/architect scope 003 kontrak sapaan untuk macOS` (spec). after spec _root/0002; ditulis bersama spec macOS scope 004

**Tujuan spec** `docs/specs/_root/` untuk keputusan bersama. Penerapan per OS ada di `docs/specs/macos/`, `docs/specs/windows/`, dan `docs/specs/linux/`.

[Kembali ke indeks scope](scope.md)
