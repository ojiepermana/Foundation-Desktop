# 001 - Batas workspace dan pemilik

Dasar · in-progress

Tentukan tempat untuk Core, penghubung macOS, aplikasi native, dan resource agar setiap tanggung jawab punya satu pemilik.

**Done when:** Workspace dapat membangun Core sendiri dan arah dependensi tiap aplikasi sesuai dengan [peta arsitektur](../architecture/README.md).

**Pointer:** spec bersama [_root/0001](../specs/_root/0001-batas-workspace-dan-pemilik/index.md) · code in `crates/core/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `docs/architecture/README.md`

* [x] `/architect scope 001 batas workspace dan pemilik untuk _root` (spec)
* [ ] Build it: `/develop scope 001 batas workspace dan pemilik`
  * [x] Milestone 1: Workspace dan kerangka Core. Build plan langkah 1 dan 2. AC-1 sampai AC-7
  * [x] Milestone 2: Peta arsitektur mencatat tata letak nyata. Build plan langkah 3. AC-8. after milestone 1
  * [ ] Milestone 3: Bukti Core di host Windows dan Linux. Build plan langkah 4. AC-9. after milestone 1; needs host Windows dan host Linux (`BLOCKED`, `owner: user`)
* [x] Verify it: `/check verify scope 001 batas workspace dan pemilik` (bukti di Mac, AC-1 sampai AC-8; AC-9 menunggu langkah manual `F001-1` di host Windows dan Linux)
* [x] Test it: `/test scope 001 batas workspace dan pemilik`

**macOS** · spec [macos/0002](../specs/macos/0002-batas-workspace-dan-pemilik.md)

* [x] `/architect scope 001 batas workspace dan pemilik untuk macOS` (spec)
* [ ] Build it: `/develop scope 001 batas workspace dan pemilik untuk macOS`
  * [ ] Milestone 1: Peta arsitektur mencatat lokasi macOS. Build plan langkah 1. AC-4. needs scope 004 milestone 2; needs AC-8 spec _root/0001 diberi batas "pada akhir milestone 2"
* [ ] Verify it: `/check verify scope 001 batas workspace dan pemilik untuk macOS`. needs Verify it scope 004

**Tujuan spec** `docs/specs/_root/` untuk keputusan bersama. Penerapan per OS ada di `docs/specs/macos/`, `docs/specs/windows/`, dan `docs/specs/linux/`.

[Kembali ke indeks scope](scope.md)
