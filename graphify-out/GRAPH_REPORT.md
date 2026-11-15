# Graph Report - desktop  (2026-11-15)

## Corpus Check
- Corpus is ~7,544 words - fits in a single context window. You may not need a graph.

## Summary
- 185 nodes · 415 edges · 13 communities (9 shown, 4 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 3 edges (avg confidence: 0.88)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Fondasi macOS
- Fondasi desktop native
- Fondasi Linux
- Scope fondasi desktop native
- Fondasi Windows
- Penerimaan dan bukti fondasi
- Sistem desain bersama
- 004 - Sapaan di jendela macOS
- update_graphify.py
- post-commit
- pre-commit
- pre-merge-commit

## God Nodes (most connected - your core abstractions)
1. `Fondasi Linux` - 23 edges
2. `Scope fondasi desktop native` - 23 edges
3. `Fondasi macOS` - 21 edges
4. `Fondasi Windows` - 19 edges
5. `Fondasi desktop native` - 17 edges
6. `Arsitektur fondasi desktop` - 16 edges
7. `Penerimaan dan bukti fondasi` - 14 edges
8. `hello(name: String) → String` - 14 edges
9. `Rust Core` - 14 edges
10. `Kontrak Rust bersama` - 13 edges

## Surprising Connections (you probably didn't know these)
- `Fondasi desktop native` --references--> `Rust Core`  [EXTRACTED]
  README.md → docs/architecture/README.md
- `Fondasi desktop native` --references--> `hello(name: String) → String`  [EXTRACTED]
  README.md → docs/contracts/README.md
- `Fondasi desktop native` --cites--> `Kontrak Rust bersama`  [EXTRACTED]
  README.md → docs/contracts/README.md
- `Fondasi desktop native` --cites--> `Sistem desain bersama`  [EXTRACTED]
  README.md → docs/design-system/README.md
- `Fondasi desktop native` --cites--> `Fondasi Linux`  [EXTRACTED]
  README.md → docs/platforms/linux/README.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Jalur sapaan melalui penghubung Swift** — docs_platforms_macos_readme_swiftui, docs_platforms_macos_readme_swift_viewmodel_dan_adapter, docs_platforms_macos_readme_binding_swift_yang_dihasilkan, docs_platforms_macos_readme_macos_ffi, docs_architecture_readme_rust_core [EXTRACTED 1.00]
- **Pemeriksaan runtime pada tiga host** — docs_quality_readme_bukti_runtime_per_host, docs_platforms_macos_readme_uji_jendela_di_mac, docs_platforms_windows_readme_uji_jendela_windows, docs_platforms_linux_readme_uji_sesi_grafis_linux [EXTRACTED 1.00]
- **Pemetaan visual native bersama** — docs_design_system_readme_token_desain_bersama, docs_design_system_readme_tema_swiftui, docs_design_system_readme_fluent_design, docs_design_system_readme_css_gtk [EXTRACTED 1.00]

## Communities (13 total, 4 thin omitted)

### Community 0 - "Fondasi macOS"
Cohesion: 0.12
Nodes (31): Cargo Workspace, Rust Core, API publik Rust, Eksekusi dan uji, hello(name: String) → String, Kepemilikan dan kompatibilitas, Kepemilikan memori dan pemetaan kesalahan, Kontrak Rust bersama (+23 more)

### Community 1 - "Fondasi desktop native"
Cohesion: 0.11
Nodes (22): Aturan commit dan integrasi, AGENTS.md, Architecture Decision Record, Arsitektur fondasi desktop, Aturan pengembangan yang akan dicatat, Batas fase fondasi, Pemilik dan arah dependensi, Pemilik tunggal dan arah dependensi (+14 more)

### Community 2 - "Fondasi Linux"
Cohesion: 0.15
Nodes (22): Bukti yang diperlukan di Linux, Compositor Wayland, Evaluasi Flatpak di masa mendatang, Fondasi Linux, GTK 4, gtk-rs, Jendela native dan Rust Core, Kompatibilitas X11 (+14 more)

### Community 4 - "Scope fondasi desktop native"
Cohesion: 0.15
Nodes (17): C18 · Aturan konfigurasi bersama terdokumentasi, C19 · Tersedia aturan lokalisasi bersama, C20 · Ada contoh minimal penggunaan lokalisasi, C21 · Kepemilikan aset bersama terdokumentasi, 009 - Konfigurasi publik, 010 - Satu contoh lokalisasi, 011 - Aset merek bersama, Scope fondasi desktop native (+9 more)

### Community 5 - "Fondasi Windows"
Cohesion: 0.18
Nodes (18): Alur dan kepemilikan, Bukti build, runtime dan distribusi, Fondasi Windows, Matriks kompatibilitas Windows, Percobaan WinUI 3 di host Windows, Pesan UI bertipe, Target Windows x64 dan kelayakan ARM64, Tema, resource dan perilaku native (+10 more)

### Community 6 - "Penerimaan dan bukti fondasi"
Cohesion: 0.19
Nodes (15): Uji jendela Windows, Bukti build per arsitektur, Bukti native per host, Bukti runtime per host, C28 · Skrip build berfungsi pada host yang sesuai, C29 · Alur kerja GitHub Actions untuk fondasi sudah dikonfigurasi, C30 · Pengujian yang relevan sudah dijalankan, C31 · Laporan menyajikan hasil aktual dari verifikasi yang dijalankan (+7 more)

### Community 7 - "Sistem desain bersama"
Cohesion: 0.31
Nodes (14): Aksesibilitas kontrol native, Alias warna semantik, CSS GTK, Fluent Design, Sistem desain bersama, Skema token desain, Tampilan terang dan gelap, Tema SwiftUI (+6 more)

### Community 8 - "004 - Sapaan di jendela macOS"
Cohesion: 0.21
Nodes (12): C04 · UniFFI berhasil menghasilkan binding Swift untuk macOS, C05 · Aplikasi macOS berhasil memanggil fungsi Rust tersebut, C14 · Antarmuka pada ketiga OS menampilkan hasil aktual dari fungsi Rust yang sama, C23 · Kepemilikan pada batas FFI macOS terdokumentasi dan divalidasi bila relevan, 000 - Panduan scope, Bukti per target, Cara menurunkan scope menjadi spec, Spec bersama dan spec per OS (+4 more)

## Knowledge Gaps
- **31 isolated node(s):** `Bekerja dari satu platform`, `Rujukan bersama`, `Pemilik dan arah dependensi`, `Tata letak yang dituju`, `Aturan pengembangan yang akan dicatat` (+26 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **4 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Scope fondasi desktop native` connect `Scope fondasi desktop native` to `Fondasi macOS`, `Fondasi desktop native`, `Fondasi Linux`, `scope.md`, `Fondasi Windows`, `Penerimaan dan bukti fondasi`, `Sistem desain bersama`, `004 - Sapaan di jendela macOS`?**
  _High betweenness centrality (0.336) - this node is a cross-community bridge._
- **Why does `Fondasi Linux` connect `Fondasi Linux` to `004 - Sapaan di jendela macOS`, `Fondasi desktop native`, `scope.md`, `Scope fondasi desktop native`?**
  _High betweenness centrality (0.132) - this node is a cross-community bridge._
- **Why does `Fondasi desktop native` connect `Fondasi desktop native` to `Fondasi macOS`, `Fondasi Linux`, `scope.md`, `Scope fondasi desktop native`, `Fondasi Windows`, `Penerimaan dan bukti fondasi`, `Sistem desain bersama`, `004 - Sapaan di jendela macOS`?**
  _High betweenness centrality (0.114) - this node is a cross-community bridge._
- **What connects `Bekerja dari satu platform`, `Rujukan bersama`, `Pemilik dan arah dependensi` to the rest of the system?**
  _31 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Fondasi macOS` be split into smaller, more focused modules?**
  _Cohesion score 0.11827956989247312 - nodes in this community are weakly interconnected._
- **Should `Fondasi desktop native` be split into smaller, more focused modules?**
  _Cohesion score 0.10507246376811594 - nodes in this community are weakly interconnected._
- **Should `Fondasi Linux` be split into smaller, more focused modules?**
  _Cohesion score 0.1471861471861472 - nodes in this community are weakly interconnected._
Pembaruan hook menggunakan indeks struktural lokal. Hash semantik kosong menandai dokumen yang menunggu ekstraksi semantik.
