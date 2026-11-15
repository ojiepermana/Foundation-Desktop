# 000 - Panduan scope

Pengguna mengetik nama di jendela native, menekan tombol, lalu melihat sapaan dari satu Rust Core. Fondasi ini memberi tim macOS, Windows, dan Linux alur kecil untuk membuktikan kontrak, tampilan, resource, dan build di platform masing masing.

**Build approach:** Tracer Bullet. Buktikan satu alur dari input sampai hasil Core di macOS. Ulangi alur itu di host Windows dan Linux ketika tersedia.

**Workflow:** Beta. Sesudah implementasi tiap fitur, jalankan `/check verify` pada target terkait, lalu `/test`.

## Cara menurunkan scope menjadi spec

Nomor scope tetap sama di semua OS. Saat spec dibuat nanti, cantumkan nomor dan tautan fitur di dalam spec, lalu tambahkan tautan balik di berkas fitur terkait dengan nama OS. Nomor berkas spec mengikuti urutan di folder masing masing. Capaian implementasi dan bukti uji dicatat per target.

| Scope | Tempat spec mendatang |
| --- | --- |
| 001, 002, 003, 008, 009, 010, 011, 012 | Keputusan yang dimiliki bersama masuk ke `docs/specs/_root/`. Untuk setiap OS, spec di `docs/specs/macos/`, `docs/specs/windows/`, atau `docs/specs/linux/` merujuk keputusan itu dan menjelaskan penerapan serta bukti lokalnya. |
| 004 | `docs/specs/macos/` |
| 005, 006 | `docs/specs/windows/` |
| 007 | `docs/specs/linux/` |

Di Mac, kamu bisa mengambil bagian bersama dan macOS dari 001 sampai 003, membuktikan alur 004, lalu mengambil bagian macOS dari 008 sampai 012. Di Windows, 005 mendahului 006. Di Linux, 007 cukup bergantung pada fondasi bersama. Pekerjaan pada satu OS dapat berjalan sambil menunggu host lain. Fitur bersama selesai ketika bagian bersama dan semua target yang berlaku sudah memiliki bukti.

Contohnya, `/architect scope 008 tema bersama untuk _root` merinci arti token. Kelak, `/architect scope 008 tema bersama untuk macOS` merinci cara SwiftUI memakai token itu dan cara mengujinya di Mac. Spec Windows dan Linux menggunakan nomor scope 008 juga.

Rujukan platform memuat batas teknis yang perlu dibawa ke spec. [macOS](../platforms/macos/README.md) · [Windows](../platforms/windows/README.md) · [Linux](../platforms/linux/README.md)

## Status fitur

`planned` berarti pekerjaan implementasi masih direncanakan. `needs a decision` menandai keputusan yang perlu dirinci dalam spec. Baris dapat berpindah ke `in-progress` ketika pekerjaan dimulai dan ke `done` setelah verifikasi serta tes yang berlaku.

[Kembali ke indeks scope](scope.md)
