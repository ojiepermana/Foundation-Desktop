# 012 - Build dan penerimaan per target

Penerimaan · in-progress

Sediakan perintah dan CI yang bisa mengulang pemeriksaan bersama dan build native. Kumpulkan hasil uji jendela dari host target. [Kriteria penerimaan](../quality/README.md)

**Done when:** Tim dapat mengulang build dan tes, menelusuri hasil aktual untuk tiap OS dan CPU, serta membuktikan tiga integrasi wajib sebelum menyatakan fondasi selesai.

**Pointer:** spec bersama [_root/0008](../specs/_root/0008-build-dan-penerimaan-per-target/index.md) · code in `scripts/`, `.github/workflows/`

* [x] `/architect scope 012 build dan penerimaan untuk _root` (spec)
* [ ] Build it: `/develop scope 012 build dan penerimaan untuk _root`
  * [ ] Milestone 1: Gerbang bersama. Build plan langkah 1 dan 2. AC-1 sampai AC-4, AC-15, AC-16. after build spec _root/0007 (di `main`); penulis tunggal `scripts/` dan `.gitattributes` pada saat itu
  * [ ] Milestone 2: Skrip macOS. Build plan langkah 3. AC-1, AC-5 sampai AC-9, AC-15. after milestone 1; after build macOS spec macos/0004 sampai macos/0008
  * [ ] Milestone 3: Workflow dan laporan. Build plan langkah 4 dan 5. AC-10, AC-13, AC-14, AC-15, AC-16. after milestone 2; needs persetujuan user untuk `docs/quality/laporan-fondasi.md` dan satu kalimat di `docs/quality/README.md`, lalu izin eksplisit di brief developer
  * [ ] Milestone 4: Bukti host lain dan CI. Build plan langkah 6. AC-11, AC-12. after milestone 3; needs host Windows dan Linux, push, dan GitHub Actions (`BLOCKED`, `owner: user`, langkah manual `F012-1`, `F012-2`)
* [ ] Verify it: `/check verify scope 012 build dan penerimaan untuk _root`
* [ ] Test it: `/test scope 012 build dan penerimaan untuk _root`

**Tujuan spec** `docs/specs/_root/` untuk keputusan bersama. Penerapan per OS ada di `docs/specs/macos/`, `docs/specs/windows/`, dan `docs/specs/linux/`.

[Kembali ke indeks scope](scope.md)
