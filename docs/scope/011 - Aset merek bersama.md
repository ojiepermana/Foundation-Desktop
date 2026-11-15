# 011 - Aset merek bersama

Penyempurnaan alur · in-progress

Tentukan pemilik berkas merek dan cara menghasilkan varian yang diperlukan setiap OS. [Resource bersama](../shared-resources/README.md)

**Done when:** Aturan sumber, penamaan, versi, dan tujuan ekspor menjelaskan bagaimana aset masuk ke bundle macOS, resource Windows, atau direktori ikon Linux.

**Pointer:** spec bersama [_root/0005](../specs/_root/0005-aset-merek-bersama/index.md)

* [x] `/architect scope 011 aset merek untuk _root` (spec)
* [ ] Build it: `/develop scope 011 aset merek bersama`
  * [ ] Milestone 1: Sumber, manifest, aturan, dan pemeriksaan aset merek. Build plan langkah 1 dan 2. AC-1 sampai AC-7, AC-9. needs scope 008 milestone 1 (spec _root/0003); after build scope 010 milestone 1 (file bersama `tooling/codegen/src/lib.rs`, `.gitattributes`)
  * [ ] Milestone 2: Dokumen rujukan menunjuk aturan aset. Build plan langkah 3. AC-8, AC-9. after milestone 1
  * [ ] Milestone 3: Berkas merek nyata dari user. Build plan langkah 4. AC-10. after milestone 1; needs ikon SVG dari user (`BLOCKED`, `owner: user`)
* [ ] Verify it: `/check verify scope 011 aset merek bersama`
* [ ] Test it: `/test scope 011 aset merek bersama`
* [ ] `/architect scope 011 aset merek untuk macOS` (spec). after milestone 1 di atas; needs pembaruan AC-12 spec macos/0001

**Tujuan spec** `docs/specs/_root/` untuk keputusan bersama. Penerapan per OS ada di `docs/specs/macos/`, `docs/specs/windows/`, dan `docs/specs/linux/`.

[Kembali ke indeks scope](scope.md)
