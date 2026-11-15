# Arsitektur fondasi desktop

Tim aplikasi akan memelihara tiga antarmuka native dengan satu sumber perilaku bersama. Pekerjaan pertama dibatasi pada sapaan agar arah dependensi bisa dibuktikan lewat tindakan pengguna di jendela nyata.

## Pemilik dan arah dependensi

| Pemilik | Tanggung jawab | Batas |
| --- | --- | --- |
| Rust Core | Perilaku bersama, aturan domain, validasi, algoritme, API publik, dan makna kesalahan | Dapat dibangun dan diuji sendiri. Bebas dari pustaka UI dan SDK khusus OS, termasuk dependensi transitif. |
| Aplikasi macOS | Jendela SwiftUI, state presentasi, interaksi dan integrasi AppKit bila perlu | Adapter Swift memakai binding hasil UniFFI. Crate penghubung tipis memanggil Core. |
| Aplikasi Windows | Jendela WinUI 3, state presentasi Rust dan akses Windows API | Aplikasi mengimpor Core langsung lewat Cargo. Kelayakan `windows-reactor` perlu dibuktikan di Windows. |
| Aplikasi Linux | Jendela GTK 4 dan libadwaita, state presentasi Rust dan integrasi desktop | Aplikasi mengimpor Core langsung lewat Cargo. |
| Sistem desain | Identitas visual dan token semantik | Setiap UI memetakan token ke kontrol native. |
| Resource bersama | Skema konfigurasi publik, katalog bahasa dan aset merek | OS memiliki pengaturan khusus platform dan kemasan lokalnya. |
| Tooling bersama | Pembuatan binding dan resource, validasi dan pemeriksaan dependensi | Hasil generator bisa dibuat ulang dari sumber yang memiliki versi. |

Jendela, navigasi, izin, event loop, dan pelepasan resource milik OS masing masing. Core bisa dipanggil tanpa jendela dan tanpa asumsi tentang titik masuk aplikasi. Ketiga target memakai kontrol dan sistem render native. Shell web, WebView, Electron, Tauri, dan emulasi UI berada di luar rancangan.

## Tata letak yang dituju

Path ini menunjukkan tempat kode **saat dibutuhkan**. Root workspace, `crates/core/`, `crates/macos-ffi/`, `bindings/swift/`, dan `apps/macos/` sudah ada. Lokasi lain masih berupa rencana sampai scope pemiliknya dikerjakan.

| Lokasi yang direncanakan | Isi |
| --- | --- |
| `apps/macos/` | Proyek Xcode dengan bagian App, Presentation, DesignSystem, Platform, Interop, dan Resources. |
| `apps/windows/`, `apps/linux/` | Crate aplikasi masing masing, dengan `src/main.rs`, modul app, presentation, designsystem, platform, serta resources. |
| `crates/core/`, `crates/macos-ffi/` | Implementasi bersama dan penghubung khusus macOS. Keduanya serta aplikasi Rust Windows dan Linux menjadi anggota Cargo Workspace. |
| `bindings/swift/` | Keluaran UniFFI untuk Swift. |
| `design-system/tokens/`, `design-system/schemas/` | Definisi visual dan skema validasinya. |
| `shared/config/`, `shared/localization/`, `shared/assets/`, `shared/specifications/` | Sumber bersama. Kontrak perilaku tetap dimiliki oleh API Rust. Spec pekerjaan per scope akan berada di `docs/specs/`. |
| `tooling/codegen/`, `tooling/validation/` | Generator dan pemeriksaan. |
| `tests/fixtures/`, `tests/contracts/` | Contoh input dan uji kontrak yang bisa digunakan kembali. |
| `scripts/`, `.github/workflows/` | Perintah lokal untuk build, generate, validate, dan test serta job CI pada host target. |

Root workspace memuat manifest Cargo virtual, `Cargo.lock` yang terlacak git, dan `rust-toolchain.toml` yang mengunci Rust `1.99.1` dengan edisi `2024`. Modul baru layak dibuat saat ada tanggung jawab nyata. Dependensi mengarah melalui antarmuka publik dan tidak boleh membentuk siklus.

Target bersama adalah Rust stable. Sebelum mengunci versi, periksa ketersediaan, status pemeliharaan, dukungan versi OS dan CPU, serta kecocokan compiler, SDK, pustaka dan runtime sebagai satu kombinasi. Dahulukan komponen stabil yang didukung. Untuk kandidat eksperimental seperti `windows-reactor`, keputusan bergantung pada hasil percobaan native.

### Yang sudah ada

| Lokasi | Isi |
| --- | --- |
| `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` | Manifest workspace virtual dengan daftar anggota eksplisit, lockfile bersama, dan pin toolchain. |
| `crates/core/` | Paket `desktop-core`, Rust Core tanpa dependensi, berisi fungsi publik `hello` dari [spec 0002](../specs/_root/0002-kontrak-sapaan-bersama/index.md). |
| `crates/macos-ffi/` | Paket `desktop-macos-ffi`, jembatan UniFFI tipis yang meneruskan API publik Core ke Swift, beserta generator binding. |
| `bindings/swift/` | Paket Swift lokal berisi binding hasil UniFFI yang terlacak git. |
| `apps/macos/` | Proyek Xcode `DesktopMac` dan paket Swift lokal dengan modul `Interop` dan `Presentation`. |

Nama paket yang dicadangkan untuk lokasi yang masih rencana: `desktop-windows` di `apps/windows/` dan `desktop-linux` di `apps/linux/`. Rincian lokasi macOS ada di [spec macOS 0001](../specs/macos/0001-sapaan-di-jendela-macos/index.md).

### Aturan arah dependensi

Dependensi mengarah ke Core dan tidak pernah membentuk siklus. Aturan berlaku untuk dependensi biasa, dependensi build, dan dependensi test.

| Dari | Boleh bergantung pada | Dilarang |
| --- | --- | --- |
| `desktop-core` | Tidak ada anggota workspace. | Anggota lain mana pun; pustaka UI; SDK khusus OS; alat binding seperti `uniffi`. Berlaku langsung maupun transitif. |
| `desktop-macos-ffi` | `desktop-core`, alat binding. | Crate aplikasi; pustaka UI. |
| `desktop-windows` | `desktop-core`; pustaka UI dan SDK Windows di bawah tabel `target` untuk Windows. | `desktop-macos-ffi`, `desktop-linux`. |
| `desktop-linux` | `desktop-core`; GTK dan libadwaita di bawah tabel `target` untuk Linux. | `desktop-macos-ffi`, `desktop-windows`. |
| `apps/macos/` (Xcode) | `bindings/swift/` dan pustaka hasil build `desktop-macos-ffi`. | Memanggil Core lewat jalur lain. |

Tidak ada anggota yang bergantung pada crate aplikasi. Anggota yang butuh pustaka satu OS menaruh dependensi itu di bawah tabel `target` untuk OS tersebut dan memberi kode yang memakainya atribut `cfg` yang sama, sehingga `cargo build --workspace --locked` berhasil di ketiga host. Bukti untuk Core adalah `cargo tree -p desktop-core --target all --all-features --prefix none --locked`, yang mencetak tepat satu baris. Rincian dan alasannya ada di [spec 0001](../specs/_root/0001-batas-workspace-dan-pemilik/index.md).

## Aturan pengembangan yang akan dicatat

Saat scaffold sudah ada, `AGENTS.md` di root akan merujuk aturan pemilik, dependensi, kontrak, dan tes. Sebelum mengubah kode, pengembang bisa mencari modul dan keputusan yang sudah mengatur perilaku itu. Saat mengubah kontrak, token, atau resource bersama, pengembang perlu menilai dampaknya pada ketiga aplikasi. Adapter OS boleh berkembang di aplikasinya sendiri.

Sesudah perubahan, hasil tes, pemeriksaan arsitektur, pembuatan ulang artefak, dan build OS yang terdampak menjadi bukti. Keputusan arsitektur penting beserta alasannya akan dicatat sebagai ADR di `docs/decisions/`. Panduan di `docs/` akan mengikuti struktur dan perilaku yang benar benar diterapkan.

Untuk layanan mendatang, setiap modul memiliki satu pemilik dan tujuan, antarmuka publik, kesalahan terstruktur, siklus hidup, serta perilaku yang dapat diuji. Modul yang sudah ada dipertimbangkan sebelum menambah crate. Penyimpanan, orkestrasi API, sinkronisasi, dan pemrosesan data bersama tetap mengikuti batas Core saat kelak masuk scope.

## Batas fase fondasi

Fase ini mencakup satu fungsi sapaan, tiga UI native dan kebutuhan bersama untuk membangun serta mengujinya. Kemampuan berikut menunggu analisis, PRD dan rencana tersendiri.

* Autentikasi, pengelolaan pengguna dan pelanggan, modul ERP, dan layanan enkripsi.
* Klien REST, SDK OpenAPI, basis data termasuk SQLite, sinkronisasi offline, dan resolusi konflik.
* Pengolahan gambar, unggahan dan sinkronisasi latar belakang, system tray, serta global keyboard hook.
* Pembaruan otomatis, plugin, mesin feature flag, navigasi dan pengelolaan jendela lanjutan, serta pustaka komponen besar.
* Platform desktop tambahan, aplikasi mobile, installer produksi dan jalur rilis produksi.

[Scope dan urutan kerja](../scope/scope.md) · [Kontrak Rust](../contracts/README.md)
