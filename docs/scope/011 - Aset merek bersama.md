# 011 - Aset merek bersama

Penyempurnaan alur · in-progress

Tentukan pemilik berkas merek dan cara menghasilkan varian yang diperlukan setiap OS. [Resource bersama](../shared-resources/README.md)

**Done when:** Aturan sumber, penamaan, versi, dan tujuan ekspor menjelaskan bagaimana aset masuk ke bundle macOS, resource Windows, atau direktori ikon Linux.

**Pointer:** spec bersama [_root/0005](../specs/_root/0005-aset-merek-bersama/index.md) · code in `shared/assets/`, `tooling/codegen/src/assets/`

* [x] `/architect scope 011 aset merek untuk _root` (spec)
* [ ] Build it: `/develop scope 011 aset merek bersama`
  * [x] Milestone 1: Sumber, manifest, aturan, dan pemeriksaan aset merek. Build plan langkah 1 dan 2. AC-1 sampai AC-7, AC-9. needs scope 008 milestone 1 (spec _root/0003); after build scope 010 milestone 1 (file bersama `tooling/codegen/src/lib.rs`, `.gitattributes`)
  * [x] Milestone 2: Dokumen rujukan menunjuk aturan aset. Build plan langkah 3. AC-8, AC-9. after milestone 1
  * [ ] Milestone 3: Berkas merek nyata dari user. Build plan langkah 4. AC-10. after milestone 1; needs ikon SVG dari user (`BLOCKED`, `owner: user`)
* [x] Verify it: `/check verify scope 011 aset merek bersama` (AC-1 sampai AC-9; AC-10 menunggu langkah manual `F011-1`, ikon SVG nyata dari user)
* [x] Test it: `/test scope 011 aset merek bersama`

**macOS** · spec [macos/0007](../specs/macos/0007-aset-merek-bersama/index.md)

* [x] `/architect scope 011 aset merek untuk macOS` (spec)
* [ ] Build it: `/develop scope 011 aset merek untuk macOS`
  * [ ] Milestone 1: Ikon penanda dari sumber bersama sampai bundel Mac. Build plan langkah 1 dan 2. AC-1 sampai AC-10. needs pembaruan gabungan spec macos/0001; tidak bersamaan dengan build lain yang menyentuh `.gitattributes`, `tooling/codegen/src/assets/`, atau `project.pbxproj` (urut terhadap build spec macos/0004); brief butuh izin `.gitattributes` dan `apps/macos/DesktopMac.xcodeproj/project.pbxproj`
  * [ ] Milestone 2: Ikon terlihat di sesi GUI. Build plan langkah 3. AC-11. after milestone 1; needs sesi GUI di Mac (`BLOCKED`, `owner: user`, langkah manual `F011-2`)
  * [ ] Milestone 3: Ikon nyata dari user. Build plan langkah 4. AC-12. after milestone 1 dan langkah manual `F011-1` (`BLOCKED`, `owner: user`, langkah manual `F011-3`)
* [ ] Verify it: `/check verify scope 011 aset merek untuk macOS`
* [ ] Test it: `/test scope 011 aset merek untuk macOS`

**Tujuan spec** `docs/specs/_root/` untuk keputusan bersama. Penerapan per OS ada di `docs/specs/macos/`, `docs/specs/windows/`, dan `docs/specs/linux/`.

[Kembali ke indeks scope](scope.md)
