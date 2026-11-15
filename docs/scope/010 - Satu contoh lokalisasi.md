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

**macOS** · spec [macos/0006](../specs/macos/0006-satu-contoh-lokalisasi/index.md)

* [x] `/architect scope 010 contoh lokalisasi untuk macOS` (spec)
* [ ] Build it: `/develop scope 010 contoh lokalisasi untuk macOS`
  * [x] Milestone 1: Resource dari katalog sampai tombol. Build plan langkah 1 sampai 3. AC-1 sampai AC-10. needs pembaruan gabungan spec macos/0001 (AC-2, AC-3, AC-8, AC-12) dan AC-6 spec macos/0005; tidak paralel dengan build spec macos/0005 (`Package.swift`, `GreetingView.swift`) atau build lain yang mengubah `.gitattributes`
  * [ ] Milestone 2: Bahasa di jendela nyata. Build plan langkah 4. AC-11. after milestone 1; needs sesi GUI di Mac (`BLOCKED`, `owner: user`, langkah manual `F010-1`)
* [x] Verify it: `/check verify scope 010 contoh lokalisasi untuk macOS` (bukti tanpa jendela, AC-1 sampai AC-10; AC-11 menunggu langkah manual `F010-1`)
* [ ] Test it: `/test scope 010 contoh lokalisasi untuk macOS`

**Tujuan spec** `docs/specs/_root/` untuk keputusan bersama. Penerapan per OS ada di `docs/specs/macos/`, `docs/specs/windows/`, dan `docs/specs/linux/`.

[Kembali ke indeks scope](scope.md)
