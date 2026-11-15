# Graph Report - desktop  (2026-11-15)

## Corpus Check
- Corpus is ~37,332 words - fits in a single context window. You may not need a graph.

## Summary
- 370 nodes · 605 edges · 31 communities (25 shown, 6 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 7 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- scope.md
- Fondasi macOS
- pre-commit script
- Scope fondasi desktop native
- Resource bersama
- Fondasi Linux
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
- greeting.rs
- 003 - Kontrak sapaan bersama
- Langkah manual: `<nama-SCOPE>`
- Langkah manual: `000 - Panduan scope`
- Milestone 1: Workspace dan kerangka Core
- Options considered
- brief.md
- brief-umum.md
- report.md
- 000 - Panduan scope.brief-umum.md
- desktop-core

## God Nodes (most connected - your core abstractions)
1. `Fondasi Linux` - 23 edges
2. `Fondasi macOS` - 21 edges
3. `Scope fondasi desktop native` - 21 edges
4. `Fondasi Windows` - 19 edges
5. `Fondasi desktop native` - 17 edges
6. `Arsitektur fondasi desktop` - 15 edges
7. `Penerimaan dan bukti fondasi` - 14 edges
8. `Rust Core` - 14 edges
9. `Sistem desain bersama` - 13 edges
10. `Resource bersama` - 13 edges

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

## Communities (31 total, 6 thin omitted)

### Community 0 - "scope.md"
Cohesion: 0.11
Nodes (14): C18 · Aturan konfigurasi bersama terdokumentasi, 009 - Konfigurasi publik, 0002. Kontrak sapaan bersama, Build plan, Consequences, Decision, Feature design, Follow-up (+6 more)

### Community 1 - "Fondasi macOS"
Cohesion: 0.10
Nodes (36): Architecture Decision Record, Arsitektur fondasi desktop, Aturan pengembangan yang akan dicatat, Batas fase fondasi, Cargo Workspace, Pemilik dan arah dependensi, Rust Core, Rust stable (+28 more)

### Community 2 - "pre-commit script"
Cohesion: 0.10
Nodes (25): Aktivasi hook per clone, Aturan commit dan integrasi, Integrasi branch ke main, Pembaruan Graphify sebelum commit, Pembaruan semantik dengan skill Graphify, Pemeriksaan SHA origin/main, Snapshot indeks Git, Verifikasi diff dan dokumentasi (+17 more)

### Community 3 - "Scope fondasi desktop native"
Cohesion: 0.11
Nodes (31): Alur dan kepemilikan, Bukti build, runtime dan distribusi, Fondasi Windows, Matriks kompatibilitas Windows, Percobaan WinUI 3 di host Windows, Pesan UI bertipe, Target Windows x64 dan kelayakan ARM64, Tema, resource dan perilaku native (+23 more)

### Community 4 - "Resource bersama"
Cohesion: 0.13
Nodes (20): Pemilik tunggal dan arah dependensi, C19 · Tersedia aturan lokalisasi bersama, C20 · Ada contoh minimal penggunaan lokalisasi, C21 · Kepemilikan aset bersama terdokumentasi, C25 · Pemeriksaan kepatuhan arsitektur telah dikonfigurasi, C26 · `AGENTS.md` di akar repo memuat aturan pengembangan yang bisa diperiksa, Tinjauan arsitektur oleh manusia, Validasi keluaran generator (+12 more)

### Community 5 - "Fondasi Linux"
Cohesion: 0.15
Nodes (22): Bukti yang diperlukan di Linux, Compositor Wayland, Evaluasi Flatpak di masa mendatang, Fondasi Linux, GTK 4, gtk-rs, Jendela native dan Rust Core, Kompatibilitas X11 (+14 more)

### Community 6 - "Alur lead"
Cohesion: 0.11
Nodes (18): 1. Tentukan SCOPE, 2. Analisis dan rencana wave, 3. Mulai sub sesi, 4. Komunikasi, 5. Laporan, 6. Gerbang centang dan merge, 7. Log, 8. Akhir wave (+10 more)

### Community 7 - "0001. Batas workspace dan pemilik"
Cohesion: 0.11
Nodes (19): 0001. Batas workspace dan pemilik, Aturan arah dependensi, Aturan untuk manifest root, Build plan, Consequences, Decision, Follow-up, Invarian (+11 more)

### Community 8 - "Penerimaan dan bukti fondasi"
Cohesion: 0.15
Nodes (16): Apple Silicon ARM64 dan Intel bersyarat, Xcode, Uji jendela Windows, Bukti build per arsitektur, Bukti native per host, Bukti runtime per host, C28 · Skrip build berfungsi pada host yang sesuai, C29 · Alur kerja GitHub Actions untuk fondasi sudah dikonfigurasi (+8 more)

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

### Community 19 - "003 - Kontrak sapaan bersama"
Cohesion: 0.25
Nodes (8): C01 · Cargo Workspace berhasil dibangun, C02 · Rust Core dapat dibangun dan diuji secara mandiri dari framework UI native, C03 · Satu implementasi Rust menjadi sumber perilaku sapaan Hello World, C22 · Tersedia aturan kontrak publik Rust yang dipakai bersama, C27 · Dokumentasi arsitektur tersedia, C32 · Implementasi hanya memuat layanan yang dibutuhkan oleh fondasi. Layanan untuk fitur mendatang tetap di luar cakupan, 001 - Batas workspace dan pemilik, 003 - Kontrak sapaan bersama

### Community 20 - "Langkah manual: `<nama-SCOPE>`"
Cohesion: 0.33
Nodes (5): `F<fitur>-<n>` · `<judul>`, Fitur `<nomor>`: `<nama>` (spec `<NNNN>`), Langkah manual: `<nama-SCOPE>`, Nilai dari user, Ringkasan

### Community 21 - "Langkah manual: `000 - Panduan scope`"
Cohesion: 0.33
Nodes (5): `F001-1` · Bangun dan uji Core di host Windows dan host Linux, Fitur `001`: `Batas workspace dan pemilik` (spec `_root/0001`), Langkah manual: `000 - Panduan scope`, Nilai dari user, Ringkasan

### Community 22 - "Milestone 1: Workspace dan kerangka Core"
Cohesion: 0.33
Nodes (5): Acceptance-criteria coverage, Commands, Milestone 1: Workspace dan kerangka Core, UI / manual, Verify: Batas workspace dan pemilik · spec _root/0001 · updated 2026-11-15

### Community 23 - "Options considered"
Cohesion: 0.40
Nodes (5): Keputusan yang lebih kecil, Option 1: Fungsi total, nama dipakai apa adanya, Option 2: Validasi di Core dengan kesalahan bertipe, Option 3: Rapikan diam diam, tanda tangan tetap, Options considered

## Knowledge Gaps
- **148 isolated node(s):** `Pemakaian`, `Konteks saat ini`, `1. Tentukan SCOPE`, `2. Analisis dan rencana wave`, `3. Mulai sub sesi` (+143 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **6 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Scope fondasi desktop native` connect `Scope fondasi desktop native` to `scope.md`, `Fondasi macOS`, `Resource bersama`, `Fondasi Linux`, `Penerimaan dan bukti fondasi`, `Sistem desain bersama`?**
  _High betweenness centrality (0.097) - this node is a cross-community bridge._
- **Why does `Fondasi desktop native` connect `Fondasi macOS` to `scope.md`, `pre-commit script`, `Scope fondasi desktop native`, `Resource bersama`, `Fondasi Linux`, `Penerimaan dan bukti fondasi`, `Sistem desain bersama`, `003 - Kontrak sapaan bersama`?**
  _High betweenness centrality (0.089) - this node is a cross-community bridge._
- **Why does `0001. Batas workspace dan pemilik` connect `0001. Batas workspace dan pemilik` to `scope.md`?**
  _High betweenness centrality (0.064) - this node is a cross-community bridge._
- **What connects `Pemakaian`, `Konteks saat ini`, `1. Tentukan SCOPE` to the rest of the system?**
  _148 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `scope.md` be split into smaller, more focused modules?**
  _Cohesion score 0.11095305832147938 - nodes in this community are weakly interconnected._
- **Should `Fondasi macOS` be split into smaller, more focused modules?**
  _Cohesion score 0.1 - nodes in this community are weakly interconnected._
- **Should `pre-commit script` be split into smaller, more focused modules?**
  _Cohesion score 0.1032258064516129 - nodes in this community are weakly interconnected._
Pembaruan hook menggunakan indeks struktural lokal. Hash semantik kosong menandai dokumen yang menunggu ekstraksi semantik.
