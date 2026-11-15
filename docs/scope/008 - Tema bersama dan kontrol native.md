# 008 - Tema bersama dan kontrol native

Penyempurnaan alur · in-progress

Terapkan warna, tipografi dan jarak yang sama maknanya pada jendela sapaan di setiap OS. [Sistem desain](../design-system/README.md)

**Done when:** Tampilan terang dan gelap memakai token yang bisa dihasilkan ulang, serta kontrol mempertahankan fokus, penskalaan, dan aksesibilitas native di tiap OS.

**Pointer:** spec bersama [_root/0003](../specs/_root/0003-tema-bersama-dan-kontrol-native/index.md) · code in `tooling/codegen/`, `design-system/`

* [x] `/architect scope 008 tema bersama untuk _root` (spec)
* [ ] Build it: `/develop scope 008 tema bersama untuk _root`
  * [x] Milestone 1: Alat dan sumber token. Build plan langkah 1 dan 2. AC-1 sampai AC-10
  * [ ] Milestone 2: Dokumen mengikuti keadaan nyata. Build plan langkah 3. AC-11. after milestone 1
  * [ ] Milestone 3: Bukti di host Windows dan Linux. Build plan langkah 4. AC-12. after milestone 1; needs host Windows dan host Linux (`BLOCKED`, `owner: user`)
* [ ] Verify it: `/check verify scope 008 tema bersama untuk _root`
* [x] Test it: `/test scope 008 tema bersama untuk _root`
* [ ] `/architect scope 008 tema bersama untuk macOS` (spec). after spec _root/0003; needs pembaruan AC-12 spec macos/0001

**Tujuan spec** `docs/specs/_root/` untuk keputusan bersama. Penerapan per OS ada di `docs/specs/macos/`, `docs/specs/windows/`, dan `docs/specs/linux/`.

[Kembali ke indeks scope](scope.md)
