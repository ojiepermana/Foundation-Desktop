# 002 - Aturan pengembangan dan pemeriksaan arsitektur

Dasar · in-progress

Setelah scaffold tersedia, `/audit` mengidentifikasi konvensi pada struktur yang nyata. Temuannya menjadi masukan untuk `/architect scope 002 aturan pengembangan untuk _root` dan spec pemeriksaan tiap OS.

**Done when:** Panduan root menunjukkan aturan dan cara uji, pemeriksaan mendeteksi dependensi terlarang serta keluaran generator yang menyimpang, dan aturan yang perlu diperiksa manusia terdokumentasi.

**Pointer:** spec bersama [_root/0007](../specs/_root/0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md) · code in `tooling/codegen/src/architecture/`

* [x] `/audit` (hasil: bagian "Aturan pengembangan" di [AGENTS.md](../../AGENTS.md) dan [crates/core/AGENTS.md](../../crates/core/AGENTS.md))
* [x] `/architect scope 002 aturan pengembangan untuk _root` (spec)
* [ ] Build it: `/develop scope 002 aturan pengembangan untuk _root`
  * [x] Milestone 1: Sumber `architecture` dan aturan dependensi. Build plan langkah 1 dan 2. AC-1 (angka `4 rules`), AC-2, AC-3, AC-10. after build scope 011 (penulis tunggal `tooling/codegen/src/lib.rs`)
  * [x] Milestone 2: Aturan kode dan uji lintas crate. Build plan langkah 3 dan 4. AC-4, AC-7, AC-8, AC-10. after milestone 1
  * [ ] Milestone 3: Panduan dan dokumen mengikuti. Build plan langkah 5. AC-1, AC-5, AC-6, AC-9, AC-10. after milestone 2; perubahan lima dokumen disetujui user 2026-11-15, izin eksplisit ada di brief developer
  * [ ] Milestone 4: Bukti di host Windows dan Linux. Build plan langkah 6. AC-11. after milestone 3; needs host Windows dan host Linux (`BLOCKED`, `owner: user`, langkah manual `F002-1`)
* [ ] Verify it: `/check verify scope 002 aturan pengembangan untuk _root`
* [ ] Test it: `/test scope 002 aturan pengembangan untuk _root`

**macOS** · spec [macos/0008](../specs/macos/0008-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md)

* [x] `/architect scope 002 aturan pengembangan untuk macOS` (spec)
* [ ] Build it: `/develop scope 002 aturan pengembangan untuk macOS`
  * [ ] Milestone 1: Gerbang Mac terbukti. Build plan langkah 1. AC-1, AC-2, AC-3, AC-4, AC-5, AC-9. after spec macos/0008
  * [ ] Milestone 2: Panduan menunjuk gerbang. Build plan langkah 2. AC-8, AC-9. after milestone 1 dan milestone 3 spec _root/0007; needs persetujuan user untuk `AGENTS.md` dan izin eksplisit di brief developer
  * [ ] Milestone 3: Aturan tema macOS. Build plan langkah 3. AC-6, AC-9. after milestone 3 spec _root/0007 dan milestone 2 spec macos/0005 di `main`
  * [ ] Milestone 4: Setelan Xcode dari generator. Build plan langkah 4. AC-7, AC-9. after milestone 3 dan milestone 1 spec macos/0004 di `main`
* [ ] Verify it: `/check verify scope 002 aturan pengembangan untuk macOS`
* [ ] Test it: `/test scope 002 aturan pengembangan untuk macOS`

**Tujuan spec** `docs/specs/_root/` untuk keputusan bersama. Penerapan per OS ada di `docs/specs/macos/`, `docs/specs/windows/`, dan `docs/specs/linux/`.

[Kembali ke indeks scope](scope.md)
