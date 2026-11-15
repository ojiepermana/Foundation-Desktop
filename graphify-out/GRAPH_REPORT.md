# Graph Report - desktop  (2026-11-15)

## Corpus Check
- Corpus is ~29,055 words - fits in a single context window. You may not need a graph.

## Summary
- 330 nodes · 573 edges · 24 communities (20 shown, 4 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 7 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Scope fondasi desktop native
- pre-commit script
- Fondasi macOS
- Resource bersama
- GTK dan sesi Linux
- Alur lead
- 0001. Batas workspace dan pemilik
- Penerimaan dan bukti fondasi
- Sistem desain bersama
- 0001. Batas workspace dan pemilik: alasan
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
- `Fondasi desktop native` --cites--> `Sistem desain bersama`  [EXTRACTED]
  README.md → docs/design-system/README.md
- `Fondasi desktop native` --cites--> `Fondasi Linux`  [EXTRACTED]
  README.md → docs/platforms/linux/README.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Integritas indeks pada tiga hook Git** — githooks_pre_commit__entry, githooks_pre_merge_commit__entry, githooks_post_commit__entry, agents_snapshot_indeks_git [INFERRED 0.85]
- **Jalur sapaan melalui penghubung Swift** — docs_platforms_macos_readme_swiftui, docs_platforms_macos_readme_swift_viewmodel_dan_adapter, docs_platforms_macos_readme_binding_swift_yang_dihasilkan, docs_platforms_macos_readme_macos_ffi, docs_architecture_readme_rust_core [EXTRACTED 1.00]
- **Pemeriksaan runtime pada tiga host** — docs_quality_readme_bukti_runtime_per_host, docs_platforms_macos_readme_uji_jendela_di_mac, docs_platforms_windows_readme_uji_jendela_windows, docs_platforms_linux_readme_uji_sesi_grafis_linux [EXTRACTED 1.00]
- **Pemetaan visual native bersama** — docs_design_system_readme_token_desain_bersama, docs_design_system_readme_tema_swiftui, docs_design_system_readme_fluent_design, docs_design_system_readme_css_gtk [EXTRACTED 1.00]

## Communities (24 total, 4 thin omitted)

### Community 0 - "Scope fondasi desktop native"
Cohesion: 0.08
Nodes (47): Architecture Decision Record, Arsitektur fondasi desktop, Aturan pengembangan yang akan dicatat, Batas fase fondasi, Cargo Workspace, Pemilik dan arah dependensi, Rust Core, Rust stable (+39 more)

### Community 1 - "pre-commit script"
Cohesion: 0.10
Nodes (25): Aktivasi hook per clone, Aturan commit dan integrasi, Integrasi branch ke main, Pembaruan Graphify sebelum commit, Pembaruan semantik dengan skill Graphify, Pemeriksaan SHA origin/main, Snapshot indeks Git, Verifikasi diff dan dokumentasi (+17 more)

### Community 2 - "Fondasi macOS"
Cohesion: 0.11
Nodes (28): Kepemilikan memori dan pemetaan kesalahan, AppKit, Apple Silicon ARM64 dan Intel bersyarat, Batas Swift dan Rust, Binding Swift yang dihasilkan, Bukti penerimaan pada Mac, Distribusi, Fondasi macOS (+20 more)

### Community 3 - "Resource bersama"
Cohesion: 0.10
Nodes (24): Pemilik tunggal dan arah dependensi, Tooling bersama, C18 · Aturan konfigurasi bersama terdokumentasi, C19 · Tersedia aturan lokalisasi bersama, C20 · Ada contoh minimal penggunaan lokalisasi, C21 · Kepemilikan aset bersama terdokumentasi, C25 · Pemeriksaan kepatuhan arsitektur telah dikonfigurasi, C26 · `AGENTS.md` di akar repo memuat aturan pengembangan yang bisa diperiksa (+16 more)

### Community 4 - "GTK dan sesi Linux"
Cohesion: 0.15
Nodes (22): Bukti yang diperlukan di Linux, Compositor Wayland, Evaluasi Flatpak di masa mendatang, Fondasi Linux, GTK 4, gtk-rs, Jendela native dan Rust Core, Kompatibilitas X11 (+14 more)

### Community 6 - "Alur lead"
Cohesion: 0.11
Nodes (18): 1. Tentukan SCOPE, 2. Analisis dan rencana wave, 3. Mulai sub sesi, 4. Komunikasi, 5. Laporan, 6. Gerbang centang dan merge, 7. Log, 8. Akhir wave (+10 more)

### Community 7 - "0001. Batas workspace dan pemilik"
Cohesion: 0.11
Nodes (19): 0001. Batas workspace dan pemilik, Aturan arah dependensi, Aturan untuk manifest root, Build plan, Consequences, Decision, Follow-up, Invarian (+11 more)

### Community 8 - "Penerimaan dan bukti fondasi"
Cohesion: 0.18
Nodes (14): Uji jendela Windows, Bukti build per arsitektur, Bukti native per host, Bukti runtime per host, C28 · Skrip build berfungsi pada host yang sesuai, C29 · Alur kerja GitHub Actions untuk fondasi sudah dikonfigurasi, C30 · Pengujian yang relevan sudah dijalankan, C31 · Laporan menyajikan hasil aktual dari verifikasi yang dijalankan (+6 more)

### Community 9 - "Sistem desain bersama"
Cohesion: 0.31
Nodes (14): Aksesibilitas kontrol native, Alias warna semantik, CSS GTK, Fluent Design, Sistem desain bersama, Skema token desain, Tampilan terang dan gelap, Tema SwiftUI (+6 more)

### Community 10 - "0001. Batas workspace dan pemilik: alasan"
Cohesion: 0.18
Nodes (11): 0001. Batas workspace dan pemilik: alasan, Bukti percobaan, Context, Cross check, Keputusan yang diambil, Opsi 1: Satu workspace, anggota eksplisit, aturan netral host, Opsi 2: Satu workspace dengan `default-members` hanya Core, Opsi 3: Aplikasi OS di luar workspace (+3 more)

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
- **129 isolated node(s):** `Pemakaian`, `Konteks saat ini`, `1. Tentukan SCOPE`, `2. Analisis dan rencana wave`, `3. Mulai sub sesi` (+124 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **4 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Scope fondasi desktop native` connect `Scope fondasi desktop native` to `Fondasi macOS`, `Resource bersama`, `GTK dan sesi Linux`, `scope.md`, `Penerimaan dan bukti fondasi`, `Sistem desain bersama`?**
  _High betweenness centrality (0.123) - this node is a cross-community bridge._
- **Why does `Fondasi desktop native` connect `Scope fondasi desktop native` to `pre-commit script`, `Fondasi macOS`, `Resource bersama`, `GTK dan sesi Linux`, `scope.md`, `Penerimaan dan bukti fondasi`, `Sistem desain bersama`?**
  _High betweenness centrality (0.099) - this node is a cross-community bridge._
- **Why does `0001. Batas workspace dan pemilik` connect `0001. Batas workspace dan pemilik` to `scope.md`?**
  _High betweenness centrality (0.074) - this node is a cross-community bridge._
- **What connects `Pemakaian`, `Konteks saat ini`, `1. Tentukan SCOPE` to the rest of the system?**
  _129 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Scope fondasi desktop native` be split into smaller, more focused modules?**
  _Cohesion score 0.08233117483811286 - nodes in this community are weakly interconnected._
- **Should `pre-commit script` be split into smaller, more focused modules?**
  _Cohesion score 0.1032258064516129 - nodes in this community are weakly interconnected._
- **Should `Fondasi macOS` be split into smaller, more focused modules?**
  _Cohesion score 0.10846560846560846 - nodes in this community are weakly interconnected._
Pembaruan hook menggunakan indeks struktural lokal. Hash semantik kosong menandai dokumen yang menunggu ekstraksi semantik.
