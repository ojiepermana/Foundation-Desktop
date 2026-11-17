# 001 - Batas workspace dan pemilik

Dasar · done

Tentukan tempat untuk Core, penghubung macOS, aplikasi native, dan resource agar setiap tanggung jawab punya satu pemilik.

**Done when:** Workspace dapat membangun Core sendiri dan arah dependensi tiap aplikasi sesuai dengan [peta arsitektur](../architecture/README.md).

**Batas penyelesaian (2026-11-17):** Pekerjaan bersama dan macOS selesai. Atas instruksi user, bukti Windows dan Linux (AC-9) berstatus `POSTPONED` sampai pengembangan di kedua platform tersebut. Penundaan ini tidak menyatakan AC-9 lulus.

**Pointer:** spec bersama [_root/0001](../specs/_root/0001-batas-workspace-dan-pemilik/index.md) · code in `crates/core/`, `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `docs/architecture/README.md`

* [x] `/architect scope 001 batas workspace dan pemilik untuk _root` (spec)
* [x] Build it: `/develop scope 001 batas workspace dan pemilik` (milestone aktif selesai; milestone 3 dipindah ke Postponed)
  * [x] Milestone 1: Workspace dan kerangka Core. Build plan langkah 1 dan 2. AC-1 sampai AC-7
  * [x] Milestone 2: Peta arsitektur mencatat tata letak nyata. Build plan langkah 3. AC-8. after milestone 1
* [x] Verify it: `/check verify scope 001 batas workspace dan pemilik` (bukti di Mac, AC-1 sampai AC-8; AC-9 `POSTPONED` sesuai instruksi user)
* [x] Test it: `/test scope 001 batas workspace dan pemilik`

**macOS** · spec [macos/0002](../specs/macos/0002-batas-workspace-dan-pemilik.md)

* [x] `/architect scope 001 batas workspace dan pemilik untuk macOS` (spec)
* [x] Build it: `/develop scope 001 batas workspace dan pemilik untuk macOS`
  * [x] Milestone 1: Peta arsitektur mencatat lokasi macOS. Build plan langkah 1. AC-4. needs scope 004 milestone 2
* [x] Verify it: `/check verify scope 001 batas workspace dan pemilik untuk macOS` (catatan di [docs/reviews](../reviews/2026-11-15-verify-macos-0002-0003.md))

## Postponed

* [ ] Milestone 3: Bukti Core di host Windows dan Linux. Build plan langkah 4, AC-9, langkah manual `F001-1`. Ditunda oleh user pada 2026-11-17 untuk dijalankan saat pengembangan [Windows (006)](006%20-%20Sapaan%20di%20jendela%20Windows.md) dan [Linux (007)](007%20-%20Sapaan%20di%20jendela%20Linux.md). Belum dijalankan pada kedua host; bukan gerbang penutupan scope 001 saat ini.

**Tujuan spec** `docs/specs/_root/` untuk keputusan bersama. Penerapan per OS ada di `docs/specs/macos/`, `docs/specs/windows/`, dan `docs/specs/linux/`.

[Kembali ke indeks scope](scope.md)
