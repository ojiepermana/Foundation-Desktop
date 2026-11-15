# Graph Report - desktop  (2026-11-15)

## Corpus Check
- Corpus is ~6,440 words - fits in a single context window. You may not need a graph.

## Summary
- 125 nodes · 306 edges · 9 communities
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 3 edges (avg confidence: 0.88)
- Token cost: tidak tersedia. Agen sesi ini tidak melaporkan pemakaian token ekstraksi.
- Sumber: 24 dokumen repo, termasuk aturan commit. Scope menjelaskan rencana dan kriteria penerimaan.

## Community Hubs (Navigation)
- Pemeriksaan dan penerimaan fondasi
- Kontrak Rust dan pemanggilan Swift
- GTK dan sesi Linux
- WinUI dan kelayakan Windows
- Konfigurasi, lokalisasi dan aset
- Token dan kontrol native
- Arsitektur dan batas workspace
- Alur spec dan verifikasi macOS
- Aturan commit dan integrasi

## God Nodes (most connected - your core abstractions)
1. `Scope fondasi desktop native` - 22 edges
2. `Fondasi Linux` - 17 edges
3. `Fondasi macOS` - 15 edges
4. `Fondasi Windows` - 14 edges
5. `Rust Core` - 14 edges
6. `hello(name: String) → String` - 14 edges
7. `Fondasi desktop native` - 14 edges
8. `Token desain bersama` - 12 edges
9. `Sistem desain bersama` - 12 edges
10. `004 - Sapaan di jendela macOS` - 11 edges

## Surprising Connections (you probably didn't know these)

README utama menautkan arsitektur, sistem desain, dan ketiga panduan OS secara eksplisit. Tiga hubungan lain merupakan inferensi Graphify.

* Uji kontrak Core dan pemeriksaan dependensinya sama-sama menilai kemandirian Core (0,85).
* Uji jendela macOS dan uji sesi grafis Linux sama-sama membutuhkan bukti interaksi pada host target (0,85).
* Validasi keluaran generator berkaitan dengan kemampuan menghasilkan ulang resource dari sumbernya (0,95).

## Hyperedges (group relationships)
- **Jalur sapaan melalui penghubung Swift** — docs_platforms_macos_readme_swiftui, docs_platforms_macos_readme_swift_viewmodel_dan_adapter, docs_platforms_macos_readme_binding_swift_yang_dihasilkan, docs_platforms_macos_readme_macos_ffi, docs_architecture_readme_rust_core [EXTRACTED 1.00]
- **Pemeriksaan runtime pada tiga host** — docs_quality_readme_bukti_runtime_per_host, docs_platforms_macos_readme_uji_jendela_di_mac, docs_platforms_windows_readme_uji_jendela_windows, docs_platforms_linux_readme_uji_sesi_grafis_linux [EXTRACTED 1.00]
- **Pemetaan visual native bersama** — docs_design_system_readme_token_desain_bersama, docs_design_system_readme_tema_swiftui, docs_design_system_readme_fluent_design, docs_design_system_readme_css_gtk [EXTRACTED 1.00]

## Communities (9 total, 0 thin omitted)

### Community 0 - "Pemeriksaan dan penerimaan fondasi"
Cohesion: 0.15
Nodes (19): Tooling bersama, Uji jendela Windows, Bukti build per arsitektur, Bukti runtime per host, C01 · Cargo Workspace berhasil dibangun, C25 · Pemeriksaan kepatuhan arsitektur telah dikonfigurasi, C26 · `AGENTS.md` di akar repo memuat aturan pengembangan yang bisa diperiksa, C28 · Skrip build berfungsi pada host yang sesuai (+11 more)

### Community 1 - "Kontrak Rust dan pemanggilan Swift"
Cohesion: 0.20
Nodes (19): API publik Rust, hello(name: String) → String, Kepemilikan memori dan pemetaan kesalahan, Kontrak Rust bersama, Uji kontrak Core mandiri, AppKit, Apple Silicon ARM64 dan Intel bersyarat, Binding Swift yang dihasilkan (+11 more)

### Community 2 - "GTK dan sesi Linux"
Cohesion: 0.20
Nodes (18): Compositor Wayland, Evaluasi Flatpak di masa mendatang, Fondasi Linux, GTK 4, gtk-rs, Kompatibilitas X11, Konteks utama GTK, libadwaita (+10 more)

### Community 3 - "WinUI dan kelayakan Windows"
Cohesion: 0.21
Nodes (17): Rust stable, Fluent Design, Fondasi Windows, Matriks kompatibilitas Windows, Pesan UI bertipe, Target Windows x64 dan kelayakan ARM64, Windows App SDK, windows-reactor (+9 more)

### Community 4 - "Konfigurasi, lokalisasi dan aset"
Cohesion: 0.23
Nodes (14): C18 · Aturan konfigurasi bersama terdokumentasi, C19 · Tersedia aturan lokalisasi bersama, C20 · Ada contoh minimal penggunaan lokalisasi, C21 · Kepemilikan aset bersama terdokumentasi, 009 - Konfigurasi publik, 010 - Satu contoh lokalisasi, 011 - Aset merek bersama, Scope fondasi desktop native (+6 more)

### Community 5 - "Token dan kontrol native"
Cohesion: 0.33
Nodes (13): Aksesibilitas kontrol native, Alias warna semantik, CSS GTK, Sistem desain bersama, Skema token desain, Tampilan terang dan gelap, Tema SwiftUI, Tipografi dan jarak (+5 more)

### Community 6 - "Arsitektur dan batas workspace"
Cohesion: 0.38
Nodes (10): AGENTS.md, Architecture Decision Record, Arsitektur fondasi desktop, Cargo Workspace, Pemilik tunggal dan arah dependensi, Rust Core, Safe Rust, C27 · Dokumentasi arsitektur tersedia (+2 more)

### Community 7 - "Alur spec dan verifikasi macOS"
Cohesion: 0.31
Nodes (9): C04 · UniFFI berhasil menghasilkan binding Swift untuk macOS, C05 · Aplikasi macOS berhasil memanggil fungsi Rust tersebut, C23 · Kepemilikan pada batas FFI macOS terdokumentasi dan divalidasi bila relevan, 000 - Panduan scope, Bukti per target, Spec bersama dan spec per OS, Tracer Bullet, Workflow Beta (+1 more)

### Community 8 - "Aturan commit dan integrasi"
Cohesion: 0.67
Nodes (6): Aturan commit dan integrasi, Integrasi branch ke main, Pembaruan Graphify sebelum commit, Pemeriksaan SHA origin/main, Snapshot indeks Git, Verifikasi diff dan dokumentasi

## Suggested Questions

1. Bagaimana satu perubahan sumber, dari indeks Git hingga push ke main, terhubung dengan pembaruan graf?
2. Bagaimana Windows membuktikan kelayakan WinUI sebelum menunjukkan sapaan dari Core?
3. Bagaimana satu nama bergerak dari jendela SwiftUI ke Rust Core dan bukti apa yang dikumpulkan di Mac?
