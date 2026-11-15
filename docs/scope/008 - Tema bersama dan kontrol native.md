# 008 - Tema bersama dan kontrol native

Penyempurnaan alur · in-progress

Terapkan warna, tipografi dan jarak yang sama maknanya pada jendela sapaan di setiap OS. [Sistem desain](../design-system/README.md)

**Done when:** Tampilan terang dan gelap memakai token yang bisa dihasilkan ulang, serta kontrol mempertahankan fokus, penskalaan, dan aksesibilitas native di tiap OS.

**Pointer:** spec bersama [_root/0003](../specs/_root/0003-tema-bersama-dan-kontrol-native/index.md) · code in `tooling/codegen/`, `design-system/`

* [x] `/architect scope 008 tema bersama untuk _root` (spec)
* [ ] Build it: `/develop scope 008 tema bersama untuk _root`
  * [x] Milestone 1: Alat dan sumber token. Build plan langkah 1 dan 2. AC-1 sampai AC-10
  * [x] Milestone 2: Dokumen mengikuti keadaan nyata. Build plan langkah 3. AC-11. after milestone 1
  * [ ] Milestone 3: Bukti di host Windows dan Linux. Build plan langkah 4. AC-12. after milestone 1; needs host Windows dan host Linux (`BLOCKED`, `owner: user`)
* [x] Verify it: `/check verify scope 008 tema bersama untuk _root` (bukti di Mac, AC-1 sampai AC-11; AC-12 menunggu langkah manual `F008-1` di host Windows dan Linux)
* [x] Test it: `/test scope 008 tema bersama untuk _root`

**macOS** · spec [macos/0005](../specs/macos/0005-tema-bersama-dan-kontrol-native/index.md)

* [x] `/architect scope 008 tema bersama untuk macOS` (spec)
* [ ] Build it: `/develop scope 008 tema bersama untuk macOS`
  * [x] Milestone 1: Token sampai ke Swift. Build plan langkah 1. AC-1, AC-2, AC-4. needs pembaruan gabungan spec macos/0001 dan macos/0004; after build scope 011 dan build spec _root/0007 (file bersama `tooling/codegen/src/lib.rs`, `.gitattributes`); memegang `tooling/codegen/src/tokens/`, `tooling/codegen/tests/tokens_contract.rs`, `tooling/codegen/tests/tokens_rules.rs`
  * [x] Milestone 2: Jendela sapaan memakai token. Build plan langkah 2 dan 3. AC-3, AC-5, AC-6, AC-9, AC-10, AC-11. after milestone 1
  * [ ] Milestone 3: Sesi GUI di Mac. Build plan langkah 4. AC-7, AC-8. after milestone 2; needs sesi GUI di Mac (`BLOCKED`, `owner: user`, langkah manual `F008-2`)
* [x] Verify it: `/check verify scope 008 tema bersama untuk macOS` (bukti tanpa jendela, AC-1 sampai AC-6 dan AC-9 sampai AC-11; AC-7 dan AC-8 menunggu langkah manual `F008-2`)
* [ ] Test it: `/test scope 008 tema bersama untuk macOS`

**Tujuan spec** `docs/specs/_root/` untuk keputusan bersama. Penerapan per OS ada di `docs/specs/macos/`, `docs/specs/windows/`, dan `docs/specs/linux/`.

[Kembali ke indeks scope](scope.md)
