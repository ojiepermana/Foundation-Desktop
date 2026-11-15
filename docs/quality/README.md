# Penerimaan dan bukti fondasi

Nomor di bawah ini mengikuti 32 kriteria penerimaan fondasi. Saat menyusun spec dari [scope bersama](../scope/scope.md), kamu bisa merujuk ke nomor yang relevan dan menjelaskan bukti yang akan dikumpulkan untuk OS tersebut. Semua kriteria masih berupa target. Fondasi lengkap setelah integrasi wajib pada ketiga OS memiliki bukti.

## Kriteria penerimaan

| No. | Hasil yang diharapkan | Scope |
| --- | --- | --- |
| 1 | Cargo Workspace berhasil dibangun. | [001](../scope/001%20-%20Batas%20workspace%20dan%20pemilik.md), [012](../scope/012%20-%20Build%20dan%20penerimaan%20per%20target.md) |
| 2 | Rust Core dapat dibangun dan diuji secara mandiri dari framework UI native. | [001](../scope/001%20-%20Batas%20workspace%20dan%20pemilik.md), [003](../scope/003%20-%20Kontrak%20sapaan%20bersama.md) |
| 3 | Satu implementasi Rust menjadi sumber perilaku sapaan Hello World. | [003](../scope/003%20-%20Kontrak%20sapaan%20bersama.md) |
| 4 | UniFFI berhasil menghasilkan binding Swift untuk macOS. | [004](../scope/004%20-%20Sapaan%20di%20jendela%20macOS.md) |
| 5 | Aplikasi macOS berhasil memanggil fungsi Rust tersebut. | [004](../scope/004%20-%20Sapaan%20di%20jendela%20macOS.md) |
| 6 | Aplikasi Windows menggunakan Rust dan WinUI 3 native. | [006](../scope/006%20-%20Sapaan%20di%20jendela%20Windows.md) |
| 7 | Integrasi Windows memakai `windows-reactor` atau padanan native Rust yang disetujui secara eksplisit. | [005](../scope/005%20-%20Kelayakan%20WinUI%20dari%20Rust.md) |
| 8 | Aplikasi Windows memanggil Rust Core langsung melalui Cargo. | [006](../scope/006%20-%20Sapaan%20di%20jendela%20Windows.md) |
| 9 | Aplikasi Windows dapat berjalan tanpa lapisan aplikasi C#. | [005](../scope/005%20-%20Kelayakan%20WinUI%20dari%20Rust.md), [006](../scope/006%20-%20Sapaan%20di%20jendela%20Windows.md) |
| 10 | Aplikasi Linux menggunakan GTK 4 dan libadwaita. | [007](../scope/007%20-%20Sapaan%20di%20jendela%20Linux.md) |
| 11 | Aplikasi Linux memanggil Rust Core langsung melalui Cargo. | [007](../scope/007%20-%20Sapaan%20di%20jendela%20Linux.md) |
| 12 | Aplikasi Linux berjalan pada backend Wayland native. | [007](../scope/007%20-%20Sapaan%20di%20jendela%20Linux.md) |
| 13 | Kompatibilitas X11 diperiksa dan hasil atau keterbatasannya dicatat. | [007](../scope/007%20-%20Sapaan%20di%20jendela%20Linux.md) |
| 14 | Antarmuka pada ketiga OS menampilkan hasil aktual dari fungsi Rust yang sama. | [004](../scope/004%20-%20Sapaan%20di%20jendela%20macOS.md), [006](../scope/006%20-%20Sapaan%20di%20jendela%20Windows.md), [007](../scope/007%20-%20Sapaan%20di%20jendela%20Linux.md) |
| 15 | Token desain memiliki satu sumber acuan. | [008](../scope/008%20-%20Tema%20bersama%20dan%20kontrol%20native.md) |
| 16 | Ketiga aplikasi memakai token hasil generasi atau pemetaan yang dapat diulang. | [008](../scope/008%20-%20Tema%20bersama%20dan%20kontrol%20native.md) |
| 17 | Antarmuka memperlihatkan tampilan terang dan gelap. | [008](../scope/008%20-%20Tema%20bersama%20dan%20kontrol%20native.md) |
| 18 | Aturan konfigurasi bersama terdokumentasi. | [009](../scope/009%20-%20Konfigurasi%20publik.md) |
| 19 | Tersedia aturan lokalisasi bersama. | [010](../scope/010%20-%20Satu%20contoh%20lokalisasi.md) |
| 20 | Ada contoh minimal penggunaan lokalisasi. | [010](../scope/010%20-%20Satu%20contoh%20lokalisasi.md) |
| 21 | Kepemilikan aset bersama terdokumentasi. | [011](../scope/011%20-%20Aset%20merek%20bersama.md) |
| 22 | Tersedia aturan kontrak publik Rust yang dipakai bersama. | [003](../scope/003%20-%20Kontrak%20sapaan%20bersama.md) |
| 23 | Kepemilikan pada batas FFI macOS terdokumentasi dan divalidasi bila relevan. | [004](../scope/004%20-%20Sapaan%20di%20jendela%20macOS.md) |
| 24 | Kompatibilitas `windows-reactor` telah dievaluasi pada Windows. | [005](../scope/005%20-%20Kelayakan%20WinUI%20dari%20Rust.md) |
| 25 | Pemeriksaan kepatuhan arsitektur telah dikonfigurasi. | [002](../scope/002%20-%20Aturan%20pengembangan%20dan%20pemeriksaan%20arsitektur.md) |
| 26 | `AGENTS.md` di akar repo memuat aturan pengembangan yang bisa diperiksa. | [002](../scope/002%20-%20Aturan%20pengembangan%20dan%20pemeriksaan%20arsitektur.md) |
| 27 | Dokumentasi arsitektur tersedia. | [001](../scope/001%20-%20Batas%20workspace%20dan%20pemilik.md) |
| 28 | Skrip build berfungsi pada host yang sesuai. | [012](../scope/012%20-%20Build%20dan%20penerimaan%20per%20target.md) |
| 29 | Alur kerja GitHub Actions untuk fondasi sudah dikonfigurasi. | [012](../scope/012%20-%20Build%20dan%20penerimaan%20per%20target.md) |
| 30 | Pengujian yang relevan sudah dijalankan. | [012](../scope/012%20-%20Build%20dan%20penerimaan%20per%20target.md) |
| 31 | Laporan menyajikan hasil aktual dari verifikasi yang dijalankan. | [012](../scope/012%20-%20Build%20dan%20penerimaan%20per%20target.md) |
| 32 | Implementasi hanya memuat layanan yang dibutuhkan oleh fondasi. Layanan untuk fitur mendatang tetap di luar cakupan. | [001](../scope/001%20-%20Batas%20workspace%20dan%20pemilik.md), [002](../scope/002%20-%20Aturan%20pengembangan%20dan%20pemeriksaan%20arsitektur.md) |

## Pemeriksaan yang direncanakan

| Area | Yang dapat diotomatisasi |
| --- | --- |
| Core | Format dan lint Rust, uji unit, uji API publik dan kontrak, serta build mandiri. Graf dependensi menunjukkan tidak ada pustaka UI yang masuk ke Core secara langsung maupun transitif. |
| Integrasi | macOS menghasilkan binding Swift secara konsisten. Windows dan Linux mengimpor Core langsung dan dapat dibangun secara terpisah dari crate FFI macOS. |
| Sumber bersama | Validasi skema token, skema konfigurasi yang tersedia, kelengkapan kunci dan placeholder lokalisasi, serta konsistensi keluaran generator. |
| Repo | Pemeriksaan isolasi platform, keberadaan dokumentasi yang diperlukan, skrip build, dan build native per OS. |

Untuk file hasil generasi, sumber acuan, pemilik, dan keluaran dicatat terpisah. CI bisa menjalankan generator dari sumber yang sama dan membandingkan hasilnya. Perubahan dilakukan pada sumber atau generator agar hasil dapat dihasilkan ulang tanpa suntingan manual. Tinjauan manusia tetap diperlukan untuk menilai kesesuaian pemilik suatu perilaku, keputusan arsitektur, dan interaksi native yang belum tercakup uji otomatis.

Rancangan direktori `scripts/` mencakup `build-rust.sh`, `build-macos.sh`, `build-windows.ps1`, `build-linux.sh`, `generate-bindings.sh`, `generate-design-tokens.sh`, `validate.sh`, dan `test.sh`. Spesifikasi build nantinya mencatat prasyarat dan perintah yang sudah dibuktikan. GitHub Actions memakai runner sesuai OS, versi toolchain yang sudah dinilai kompatibel, dan izin minimum. Kredensial penandatanganan berada di luar repo. Pipeline rilis produksi berada di luar tahap ini.

## Bukti native per host

| Host build dan uji | Target yang direncanakan | Bukti runtime |
| --- | --- | --- |
| macOS dengan Xcode | Apple Silicon ARM64, Intel `x86_64` bila didukung. Catat versi minimum macOS. | Buka jendela SwiftUI, kirim nama melalui binding UniFFI, amati hasil Rust pada label, lalu periksa integrasi token. |
| Windows dengan Windows App SDK | `x86_64`, ARM64 bila didukung. Catat versi Windows, toolchain, dan runtime yang cocok dengan `windows-reactor`. | Jalankan jendela WinUI 3 di sesi desktop, periksa input dan event bertipe, hasil panggilan Core, tema, dan penutupan. |
| Linux dengan pustaka GTK | `x86_64`, `aarch64` bila didukung. Catat pustaka native dan versi minimum GTK 4 serta libadwaita. | Buka aplikasi GTK dalam sesi Wayland native dan uji kompatibilitas X11 bila tersedia. Periksa hasil Core dan token pada jendela. |

Setiap hasil mencantumkan OS, arsitektur, versi, perintah, hasil, dan batas pengujian. Build yang berhasil memberi bukti kompilasi pada target itu. Interaksi pengguna memerlukan pengujian runtime tersendiri. Bila runner Windows hanya dapat membangun aplikasi atau runner Linux hanya dapat menguji tanpa layar, pengujian jendela tetap berstatus belum terverifikasi sampai ada sesi yang sesuai.

## Laporan ketika fondasi selesai

Laporan merangkum arsitektur dan struktur repo yang terealisasi, implementasi Core, integrasi macOS, Windows dan Linux, sistem desain, serta sumber daya bersama. Sertakan versi teknologi yang dipilih, hasil uji kompatibilitas, perintah build yang berfungsi, hasil tes yang dijalankan, dan konfigurasi CI.

Catat setiap kegagalan, keterbatasan yang diketahui, host yang masih dibutuhkan, serta target yang belum diuji. Milestone arsitektur berikutnya dapat merujuk langsung ke bukti atau kriteria yang masih terbuka.
