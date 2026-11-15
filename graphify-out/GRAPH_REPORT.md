# Graph Report - desktop  (2026-11-15)

## Corpus Check
- Corpus is ~22,996 words - fits in a single context window. You may not need a graph.

## Summary
- 298 nodes · 537 edges · 24 communities (20 shown, 4 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 7 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Fondasi macOS
- Scope fondasi desktop native
- GTK dan sesi Linux
- pre-commit script
- Arsitektur fondasi desktop
- Alur lead
- Fondasi Windows
- Desain dan kontrol native
- Resource bersama
- 004 - Sapaan di jendela macOS
- Log koordinasi: `<nama-SCOPE>`
- Log koordinasi: `000 - Panduan scope`
- Peran: architect
- Peran: checker
- Peran: debugger
- Peran: developer
- Peran: tester
- Langkah manual: `<nama-SCOPE>`
- Langkah manual: `000 - Panduan scope`
- brief.md
- brief-umum.md
- report.md
- 000 - Panduan scope.brief-umum.md

## God Nodes (most connected - your core abstractions)
1. `Fondasi Linux` - 23 edges
2. `Scope fondasi desktop native` - 23 edges
3. `Fondasi macOS` - 21 edges
4. `Fondasi Windows` - 19 edges
5. `Fondasi desktop native` - 17 edges
6. `Arsitektur fondasi desktop` - 16 edges
7. `Penerimaan dan bukti fondasi` - 14 edges
8. `Rust Core` - 14 edges
9. `hello(name: String) → String` - 14 edges
10. `Kontrak Rust bersama` - 13 edges

## Surprising Connections (you probably didn't know these)
- `Pembaruan semantik dengan skill Graphify` --conceptually_related_to--> `Penggunaan ulang bukti semantik`  [INFERRED]
  AGENTS.md → scripts/update_graphify.py
- `Pembaruan Graphify sebelum commit` --references--> `Ekstraksi struktur lokal`  [EXTRACTED]
  AGENTS.md → scripts/update_graphify.py
- `Fondasi desktop native` --cites--> `Aturan commit dan integrasi`  [EXTRACTED]
  README.md → AGENTS.md
- `Fondasi desktop native` --cites--> `Arsitektur fondasi desktop`  [EXTRACTED]
  README.md → docs/architecture/README.md
- `Fondasi desktop native` --references--> `Rust Core`  [EXTRACTED]
  README.md → docs/architecture/README.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Integritas indeks pada tiga hook Git** — githooks_pre_commit__entry, githooks_pre_merge_commit__entry, githooks_post_commit__entry, agents_snapshot_indeks_git [INFERRED 0.85]
- **Jalur sapaan melalui penghubung Swift** — docs_platforms_macos_readme_swiftui, docs_platforms_macos_readme_swift_viewmodel_dan_adapter, docs_platforms_macos_readme_binding_swift_yang_dihasilkan, docs_platforms_macos_readme_macos_ffi, docs_architecture_readme_rust_core [EXTRACTED 1.00]
- **Pemeriksaan runtime pada tiga host** — docs_quality_readme_bukti_runtime_per_host, docs_platforms_macos_readme_uji_jendela_di_mac, docs_platforms_windows_readme_uji_jendela_windows, docs_platforms_linux_readme_uji_sesi_grafis_linux [EXTRACTED 1.00]
- **Pemetaan visual native bersama** — docs_design_system_readme_token_desain_bersama, docs_design_system_readme_tema_swiftui, docs_design_system_readme_fluent_design, docs_design_system_readme_css_gtk [EXTRACTED 1.00]

## Communities (24 total, 4 thin omitted)

### Community 0 - "Fondasi macOS"
Cohesion: 0.11
Nodes (33): Cargo Workspace, Rust Core, API publik Rust, Eksekusi dan uji, hello(name: String) → String, Kepemilikan dan kompatibilitas, Kepemilikan memori dan pemetaan kesalahan, Kontrak Rust bersama (+25 more)

### Community 1 - "Scope fondasi desktop native"
Cohesion: 0.11
Nodes (27): Uji jendela Windows, Bukti build per arsitektur, Bukti native per host, Bukti runtime per host, C01 · Cargo Workspace berhasil dibangun, C02 · Rust Core dapat dibangun dan diuji secara mandiri dari framework UI native, C25 · Pemeriksaan kepatuhan arsitektur telah dikonfigurasi, C26 · `AGENTS.md` di akar repo memuat aturan pengembangan yang bisa diperiksa (+19 more)

### Community 3 - "GTK dan sesi Linux"
Cohesion: 0.15
Nodes (22): Bukti yang diperlukan di Linux, Compositor Wayland, Evaluasi Flatpak di masa mendatang, Fondasi Linux, GTK 4, gtk-rs, Jendela native dan Rust Core, Kompatibilitas X11 (+14 more)

### Community 4 - "pre-commit script"
Cohesion: 0.17
Nodes (17): Pembaruan Graphify sebelum commit, Pembaruan semantik dengan skill Graphify, Snapshot indeks Git, post-commit script, Rekonsiliasi indeks sesudah git commit --only, pre-commit script, Empat artefak Graphify dalam satu commit, Isolasi riwayat kueri lokal (+9 more)

### Community 5 - "Arsitektur fondasi desktop"
Cohesion: 0.12
Nodes (17): Aktivasi hook per clone, Aturan commit dan integrasi, Integrasi branch ke main, Pemeriksaan SHA origin/main, Verifikasi diff dan dokumentasi, AGENTS.md, Architecture Decision Record, Arsitektur fondasi desktop (+9 more)

### Community 6 - "Alur lead"
Cohesion: 0.11
Nodes (18): 1. Tentukan SCOPE, 2. Analisis dan rencana wave, 3. Mulai sub sesi, 4. Komunikasi, 5. Laporan, 6. Gerbang centang dan merge, 7. Log, 8. Akhir wave (+10 more)

### Community 7 - "Fondasi Windows"
Cohesion: 0.16
Nodes (19): Alur dan kepemilikan, Bukti build, runtime dan distribusi, Fondasi Windows, Matriks kompatibilitas Windows, Percobaan WinUI 3 di host Windows, Pesan UI bertipe, Target Windows x64 dan kelayakan ARM64, Tema, resource dan perilaku native (+11 more)

### Community 8 - "Desain dan kontrol native"
Cohesion: 0.29
Nodes (15): Aksesibilitas kontrol native, Alias warna semantik, CSS GTK, Fluent Design, Sistem desain bersama, Skema token desain, Tampilan terang dan gelap, Tema SwiftUI (+7 more)

### Community 9 - "Resource bersama"
Cohesion: 0.22
Nodes (13): C19 · Tersedia aturan lokalisasi bersama, C20 · Ada contoh minimal penggunaan lokalisasi, C21 · Kepemilikan aset bersama terdokumentasi, 010 - Satu contoh lokalisasi, 011 - Aset merek bersama, Aset merek, Aset merek berversi, Fallback dan validasi placeholder (+5 more)

### Community 10 - "004 - Sapaan di jendela macOS"
Cohesion: 0.24
Nodes (11): C04 · UniFFI berhasil menghasilkan binding Swift untuk macOS, C05 · Aplikasi macOS berhasil memanggil fungsi Rust tersebut, C23 · Kepemilikan pada batas FFI macOS terdokumentasi dan divalidasi bila relevan, 000 - Panduan scope, Bukti per target, Cara menurunkan scope menjadi spec, Spec bersama dan spec per OS, Status fitur (+3 more)

### Community 11 - "Log koordinasi: `<nama-SCOPE>`"
Cohesion: 0.22
Nodes (8): Blocker terbuka, Dicentang pada wave ini, Keputusan, Keputusan delegasi, Log koordinasi: `<nama-SCOPE>`, Penutupan, Riwayat wave, Sub sesi wave `<n>`

### Community 12 - "Log koordinasi: `000 - Panduan scope`"
Cohesion: 0.22
Nodes (8): Blocker terbuka, Dicentang pada wave ini, Keputusan, Keputusan delegasi, Log koordinasi: `000 - Panduan scope`, Penutupan, Riwayat wave, Sub sesi wave `1`

### Community 13 - "Peran: architect"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: architect, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 14 - "Peran: checker"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: checker, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 15 - "Peran: debugger"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: debugger, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 16 - "Peran: developer"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: developer, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 17 - "Peran: tester"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: tester, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 18 - "Langkah manual: `<nama-SCOPE>`"
Cohesion: 0.33
Nodes (5): `F<fitur>-<n>` · `<judul>`, Fitur `<nomor>`: `<nama>` (spec `<NNNN>`), Langkah manual: `<nama-SCOPE>`, Nilai dari user, Ringkasan

### Community 19 - "Langkah manual: `000 - Panduan scope`"
Cohesion: 0.50
Nodes (3): Langkah manual: `000 - Panduan scope`, Nilai dari user, Ringkasan

## Knowledge Gaps
- **103 isolated node(s):** `Pemakaian`, `Konteks saat ini`, `1. Tentukan SCOPE`, `2. Analisis dan rencana wave`, `3. Mulai sub sesi` (+98 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **4 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Scope fondasi desktop native` connect `Scope fondasi desktop native` to `Fondasi macOS`, `scope.md`, `GTK dan sesi Linux`, `Arsitektur fondasi desktop`, `Fondasi Windows`, `Desain dan kontrol native`, `Resource bersama`, `004 - Sapaan di jendela macOS`?**
  _High betweenness centrality (0.141) - this node is a cross-community bridge._
- **Why does `Fondasi desktop native` connect `Scope fondasi desktop native` to `Fondasi macOS`, `scope.md`, `GTK dan sesi Linux`, `Arsitektur fondasi desktop`, `Fondasi Windows`, `Desain dan kontrol native`, `Resource bersama`, `004 - Sapaan di jendela macOS`?**
  _High betweenness centrality (0.105) - this node is a cross-community bridge._
- **Why does `Aturan commit dan integrasi` connect `Arsitektur fondasi desktop` to `Scope fondasi desktop native`, `pre-commit script`?**
  _High betweenness centrality (0.073) - this node is a cross-community bridge._
- **What connects `Pemakaian`, `Konteks saat ini`, `1. Tentukan SCOPE` to the rest of the system?**
  _103 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Fondasi macOS` be split into smaller, more focused modules?**
  _Cohesion score 0.10795454545454546 - nodes in this community are weakly interconnected._
- **Should `Scope fondasi desktop native` be split into smaller, more focused modules?**
  _Cohesion score 0.10541310541310542 - nodes in this community are weakly interconnected._
- **Should `GTK dan sesi Linux` be split into smaller, more focused modules?**
  _Cohesion score 0.1471861471861472 - nodes in this community are weakly interconnected._
Pembaruan hook menggunakan indeks struktural lokal. Hash semantik kosong menandai dokumen yang menunggu ekstraksi semantik.
