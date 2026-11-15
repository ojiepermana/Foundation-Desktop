# 009 - Konfigurasi publik

Penyempurnaan alur · in-progress

Pisahkan nilai publik bersama dari pengaturan lokal dan rahasia penandatanganan.

**Done when:** Skema, versi, dan cara validasi serta konsumsi nilai publik terdokumentasi, termasuk batas pengaturan khusus tiap OS.

**Pointer:** spec bersama [_root/0004](../specs/_root/0004-konfigurasi-publik/index.md)

* [x] `/architect scope 009 konfigurasi publik untuk _root` (spec)
* [ ] Build it: `/develop scope 009 konfigurasi publik untuk _root`
  * [ ] Milestone 1: Sumber nilai, pemeriksaan, dan skema hasil generator. Build plan langkah 1 dan 2. AC-1, AC-2, AC-3, AC-4, AC-5, AC-6, AC-7, AC-9, AC-10. needs scope 008 milestone 1 (spec _root/0003, alat `desktop-codegen`) di `main`; tidak paralel dengannya
  * [ ] Milestone 2: Dokumen rujukan mencatat sumber yang nyata. Build plan langkah 3. AC-8, AC-9. after milestone 1
* [ ] Verify it: `/check verify scope 009 konfigurasi publik untuk _root`
* [ ] Test it: `/test scope 009 konfigurasi publik untuk _root`
* [ ] `/architect scope 009 konfigurasi publik untuk macOS` (spec). after spec _root/0004

**Tujuan spec** `docs/specs/_root/` untuk keputusan bersama. Penerapan per OS ada di `docs/specs/macos/`, `docs/specs/windows/`, dan `docs/specs/linux/`.

[Kembali ke indeks scope](scope.md)
