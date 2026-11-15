# 010 - Satu contoh lokalisasi

Penyempurnaan alur · in-progress

Gunakan satu teks antarmuka pada layar sapaan untuk membuktikan alur dari katalog bersama ke resource bahasa native.

**Done when:** Ketiga aplikasi menampilkan teks UI dari kunci yang sama, dengan aturan fallback, placeholder, dan deteksi terjemahan yang hilang.

**Pointer:** spec bersama [_root/0006](../specs/_root/0006-satu-contoh-lokalisasi/index.md) · code in `shared/localization/`, `tooling/codegen/src/localization/`

* [x] `/architect scope 010 contoh lokalisasi untuk _root` (spec)
* [x] Build it: `/develop scope 010 contoh lokalisasi untuk _root`
  * [x] Milestone 1: Katalog dan sumber `localization`. Build plan langkah 1 dan 2. AC-1 sampai AC-10. needs scope 008 milestone 1 (spec _root/0003) di `main`; tidak paralel dengan build spec _root/0003 atau _root/0004
  * [x] Milestone 2: Dokumen rujukan mencatat keadaan nyata. Build plan langkah 3. AC-11. after milestone 1
* [x] Verify it: `/check verify scope 010 contoh lokalisasi untuk _root`
* [x] Test it: `/test scope 010 contoh lokalisasi untuk _root`
* [ ] `/architect scope 010 contoh lokalisasi untuk macOS` (spec). after milestone 1 di atas; needs pembaruan AC-12 spec macos/0001

**Tujuan spec** `docs/specs/_root/` untuk keputusan bersama. Penerapan per OS ada di `docs/specs/macos/`, `docs/specs/windows/`, dan `docs/specs/linux/`.

[Kembali ke indeks scope](scope.md)
