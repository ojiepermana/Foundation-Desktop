# Graph Report - desktop  (2026-11-15)

## Corpus Check
- Corpus is ~7,544 words - fits in a single context window. You may not need a graph.

## Summary
- 203 nodes · 456 edges · 10 communities
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 7 edges (avg confidence: 0.86)
- Token cost: pemakaian token ekstraksi semantik oleh agen tidak dilaporkan.

## Graph Freshness
- Built from commit: `db3e62a3`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Pembaruan semantik hook Git

Dibandingkan dengan graf struktural sebelumnya, pembaruan lima berkas menambah 18 konsep dan 41 hubungan. Seluruh hubungan sebelumnya tetap ada.

- Hook `pre-commit` membuat snapshot indeks, menjalankan pembaruan lokal, lalu menempatkan empat artefak Graphify di indeks yang digunakan untuk commit.
- Hook `pre-merge-commit` memanggil `pre-commit` dan meminta `git merge --continue` jika pembaruan graf mengubah tree yang akan digabungkan.
- Setelah `git commit --only`, hook `post-commit` membandingkan artefak di indeks dengan commit sebelumnya sebelum memperbarui entri yang tertinggal.

## Community Hubs (Navigation)
- Kontrak Rust dan macOS
- Arsitektur dan aturan pengembangan
- GTK dan sesi Linux
- Konfigurasi dan resource bersama
- WinUI dan integrasi Windows
- Build dan bukti penerimaan
- Desain dan kontrol native
- Spec dan verifikasi macOS
- Hook Git dan Graphify

## God Nodes (most connected - your core abstractions)
1. `Fondasi Linux` - 23 edges
2. `Scope fondasi desktop native` - 23 edges
3. `Fondasi macOS` - 21 edges
4. `Fondasi Windows` - 19 edges
5. `Fondasi desktop native` - 17 edges
6. `Arsitektur fondasi desktop` - 16 edges
7. `Rust Core` - 14 edges
8. `hello(name: String) → String` - 14 edges
9. `Penerimaan dan bukti fondasi` - 14 edges
10. `Kontrak Rust bersama` - 13 edges

## Surprising Connections (you probably didn't know these)
- `Pembaruan semantik dengan skill Graphify` --conceptually_related_to--> `Penggunaan ulang bukti semantik`  [INFERRED]
  AGENTS.md → scripts/update_graphify.py
- `Pembaruan Graphify sebelum commit` --references--> `Ekstraksi struktur lokal`  [EXTRACTED]
  AGENTS.md → scripts/update_graphify.py
- `Fondasi desktop native` --cites--> `004 - Sapaan di jendela macOS`  [EXTRACTED]
  README.md → docs/scope/004 - Sapaan di jendela macOS.md
- `Fondasi desktop native` --references--> `Rust Core`  [EXTRACTED]
  README.md → docs/architecture/README.md
- `Fondasi desktop native` --references--> `hello(name: String) → String`  [EXTRACTED]
  README.md → docs/contracts/README.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Integritas indeks pada tiga hook Git** — githooks_pre_commit__entry, githooks_pre_merge_commit__entry, githooks_post_commit__entry, agents_snapshot_indeks_git [INFERRED 0.85]
- **Jalur sapaan melalui penghubung Swift** — docs_platforms_macos_readme_swiftui, docs_platforms_macos_readme_swift_viewmodel_dan_adapter, docs_platforms_macos_readme_binding_swift_yang_dihasilkan, docs_platforms_macos_readme_macos_ffi, docs_architecture_readme_rust_core [EXTRACTED 1.00]
- **Pemeriksaan runtime pada tiga host** — docs_quality_readme_bukti_runtime_per_host, docs_platforms_macos_readme_uji_jendela_di_mac, docs_platforms_windows_readme_uji_jendela_windows, docs_platforms_linux_readme_uji_sesi_grafis_linux [EXTRACTED 1.00]
- **Pemetaan visual native bersama** — docs_design_system_readme_token_desain_bersama, docs_design_system_readme_tema_swiftui, docs_design_system_readme_fluent_design, docs_design_system_readme_css_gtk [EXTRACTED 1.00]

## Communities (10 total, 0 thin omitted)

### Community 0 - "Kontrak Rust dan macOS"
Cohesion: 0.12
Nodes (31): Cargo Workspace, Rust Core, API publik Rust, Eksekusi dan uji, hello(name: String) → String, Kepemilikan dan kompatibilitas, Kepemilikan memori dan pemetaan kesalahan, Kontrak Rust bersama (+23 more)

### Community 1 - "Arsitektur dan aturan pengembangan"
Cohesion: 0.10
Nodes (23): Aktivasi hook per clone, Aturan commit dan integrasi, Verifikasi diff dan dokumentasi, AGENTS.md, Architecture Decision Record, Arsitektur fondasi desktop, Aturan pengembangan yang akan dicatat, Batas fase fondasi (+15 more)

### Community 2 - "GTK dan sesi Linux"
Cohesion: 0.15
Nodes (22): Bukti yang diperlukan di Linux, Compositor Wayland, Evaluasi Flatpak di masa mendatang, Fondasi Linux, GTK 4, gtk-rs, Jendela native dan Rust Core, Kompatibilitas X11 (+14 more)

### Community 4 - "Konfigurasi dan resource bersama"
Cohesion: 0.15
Nodes (17): C18 · Aturan konfigurasi bersama terdokumentasi, C19 · Tersedia aturan lokalisasi bersama, C20 · Ada contoh minimal penggunaan lokalisasi, C21 · Kepemilikan aset bersama terdokumentasi, 009 - Konfigurasi publik, 010 - Satu contoh lokalisasi, 011 - Aset merek bersama, Scope fondasi desktop native (+9 more)

### Community 5 - "WinUI dan integrasi Windows"
Cohesion: 0.15
Nodes (20): Rust stable, Alur dan kepemilikan, Bukti build, runtime dan distribusi, Fondasi Windows, Matriks kompatibilitas Windows, Percobaan WinUI 3 di host Windows, Pesan UI bertipe, Target Windows x64 dan kelayakan ARM64 (+12 more)

### Community 6 - "Build dan bukti penerimaan"
Cohesion: 0.18
Nodes (14): Uji jendela Windows, Bukti build per arsitektur, Bukti native per host, Bukti runtime per host, C28 · Skrip build berfungsi pada host yang sesuai, C29 · Alur kerja GitHub Actions untuk fondasi sudah dikonfigurasi, C30 · Pengujian yang relevan sudah dijalankan, C31 · Laporan menyajikan hasil aktual dari verifikasi yang dijalankan (+6 more)

### Community 7 - "Desain dan kontrol native"
Cohesion: 0.29
Nodes (15): Aksesibilitas kontrol native, Alias warna semantik, CSS GTK, Fluent Design, Sistem desain bersama, Skema token desain, Tampilan terang dan gelap, Tema SwiftUI (+7 more)

### Community 8 - "Spec dan verifikasi macOS"
Cohesion: 0.24
Nodes (11): C04 · UniFFI berhasil menghasilkan binding Swift untuk macOS, C05 · Aplikasi macOS berhasil memanggil fungsi Rust tersebut, C23 · Kepemilikan pada batas FFI macOS terdokumentasi dan divalidasi bila relevan, 000 - Panduan scope, Bukti per target, Cara menurunkan scope menjadi spec, Spec bersama dan spec per OS, Status fitur (+3 more)

### Community 9 - "Hook Git dan Graphify"
Cohesion: 0.13
Nodes (21): Integrasi branch ke main, Pembaruan Graphify sebelum commit, Pembaruan semantik dengan skill Graphify, Pemeriksaan SHA origin/main, Snapshot indeks Git, post-commit script, Rekonsiliasi indeks sesudah git commit --only, pre-commit script (+13 more)

## Knowledge Gaps
- **34 isolated node(s):** `Eksekusi dan uji`, `Kepemilikan dan kompatibilitas`, `Batas Swift dan Rust`, `Bukti penerimaan pada Mac`, `Distribusi` (+29 more)
  These have ≤1 connection - possible missing edges or undocumented components.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Scope fondasi desktop native` connect `Konfigurasi dan resource bersama` to `Kontrak Rust dan macOS`, `Arsitektur dan aturan pengembangan`, `GTK dan sesi Linux`, `Indeks scope dan rujukan`, `WinUI dan integrasi Windows`, `Build dan bukti penerimaan`, `Desain dan kontrol native`, `Spec dan verifikasi macOS`?**
  _High betweenness centrality (0.306) - this node is a cross-community bridge._
- **Why does `Fondasi desktop native` connect `Arsitektur dan aturan pengembangan` to `Kontrak Rust dan macOS`, `GTK dan sesi Linux`, `Indeks scope dan rujukan`, `Konfigurasi dan resource bersama`, `WinUI dan integrasi Windows`, `Build dan bukti penerimaan`, `Desain dan kontrol native`, `Spec dan verifikasi macOS`?**
  _High betweenness centrality (0.227) - this node is a cross-community bridge._
- **Why does `Aturan commit dan integrasi` connect `Arsitektur dan aturan pengembangan` to `Hook Git dan Graphify`?**
  _High betweenness centrality (0.157) - this node is a cross-community bridge._
- **What connects `Eksekusi dan uji`, `Kepemilikan dan kompatibilitas`, `Batas Swift dan Rust` to the rest of the system?**
  _34 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Kontrak Rust dan macOS` be split into smaller, more focused modules?**
  _Cohesion score 0.11827956989247312 - nodes in this community are weakly interconnected._
- **Should `Arsitektur dan aturan pengembangan` be split into smaller, more focused modules?**
  _Cohesion score 0.1 - nodes in this community are weakly interconnected._
- **Should `GTK dan sesi Linux` be split into smaller, more focused modules?**
  _Cohesion score 0.1471861471861472 - nodes in this community are weakly interconnected._
