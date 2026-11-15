# 004 - Sapaan di jendela macOS

Alur native · in-progress

Hubungkan kolom nama, tombol, dan label SwiftUI ke [kontrak 003](003%20-%20Kontrak%20sapaan%20bersama.md) melalui binding Swift. Ini alur pertama yang bisa dibuktikan dari Mac.

**Done when:** Binding dapat dibuat ulang, aplikasi terbuka di macOS, dan menekan tombol setelah mengisi nama menampilkan nilai yang dikembalikan Core.

**Pointer:** spec macOS [macos/0001](../specs/macos/0001-sapaan-di-jendela-macos/index.md) · code in `crates/macos-ffi/`, `bindings/swift/`, `apps/macos/`

* [x] `/architect scope 004 sapaan di jendela macOS` (spec)
* [ ] Build it: `/develop scope 004 sapaan di jendela macOS`
  * [x] Milestone 1: Jembatan, binding, dan uji tanpa jendela. Build plan langkah 1 sampai 3. AC-1, AC-2, AC-3, AC-6, AC-7, AC-9. needs scope 001 milestone 1 dan scope 003 milestone 1
  * [x] Milestone 2: Jendela SwiftUI dan proyek Xcode. Build plan langkah 4. AC-4, AC-8. after milestone 1
  * [x] Milestone 3: Bukti build, kombinasi versi, dan kebersihan repo. Build plan langkah 5. AC-10, AC-11, AC-12. after milestone 2
  * [ ] Milestone 4: Sesi GUI di Mac. Build plan langkah 6. AC-5, AC-13. after milestone 2; needs sesi GUI di Mac (`BLOCKED`, `owner: user`)
* [x] Verify it: `/check verify scope 004 sapaan di jendela macOS` (bukti tanpa jendela, AC-1 sampai AC-4 dan AC-6 sampai AC-12; AC-5 dan AC-13 menunggu langkah manual `F004-1` dan `F004-2`)
* [x] Test it: `/test scope 004 sapaan di jendela macOS`

**Tujuan spec** `docs/specs/macos/`

[Kembali ke indeks scope](scope.md)
