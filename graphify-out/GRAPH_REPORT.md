# Graph Report - desktop  (2026-11-15)

## Corpus Check
- 58 files · ~55,493 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 445 nodes · 682 edges · 36 communities (31 shown, 5 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 12 edges (avg confidence: 0.86)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Scope fondasi desktop native
- pre-commit script
- Fondasi macOS
- hello
- Penerimaan dan bukti fondasi
- workspace_layout.rs
- 004 - Sapaan di jendela macOS
- Alur lead
- Fondasi Linux
- 0001. Batas workspace dan pemilik
- Sistem desain bersama
- contract_surface.rs
- 0001. Sapaan di jendela macOS: catatan keputusan
- 0001. Batas workspace dan pemilik: alasan
- Milestone 1: Workspace dan kerangka Core
- Milestone 1: fungsi hello dan uji Done when
- Log koordinasi: `<nama-SCOPE>`
- Log koordinasi: `000 - Panduan scope`
- 0001. Sapaan di jendela macOS
- 0002. Kontrak sapaan bersama
- Peran: architect
- Peran: checker
- Peran: debugger
- Peran: developer
- Peran: tester
- Options considered
- Arsitektur fondasi desktop
- Langkah manual: `<nama-SCOPE>`
- Langkah manual: `000 - Panduan scope`
- brief.md
- brief-umum.md
- report.md
- 000 - Panduan scope.brief-umum.md
- desktop-core

## God Nodes (most connected - your core abstractions)
1. `Fondasi Linux` - 23 edges
2. `Fondasi macOS` - 21 edges
3. `Scope fondasi desktop native` - 20 edges
4. `Fondasi Windows` - 19 edges
5. `Fondasi desktop native` - 16 edges
6. `Penerimaan dan bukti fondasi` - 14 edges
7. `Sistem desain bersama` - 13 edges
8. `Resource bersama` - 13 edges
9. `004 - Sapaan di jendela macOS` - 12 edges
10. `007 - Sapaan di jendela Linux` - 12 edges

## Surprising Connections (you probably didn't know these)
- `Pembaruan semantik dengan skill Graphify` --conceptually_related_to--> `Penggunaan ulang bukti semantik`  [INFERRED]
  AGENTS.md → scripts/update_graphify.py
- `Pembaruan Graphify sebelum commit` --references--> `Ekstraksi struktur lokal`  [EXTRACTED]
  AGENTS.md → scripts/update_graphify.py
- `Fondasi desktop native` --cites--> `Aturan commit dan integrasi`  [EXTRACTED]
  README.md → AGENTS.md
- `Fondasi desktop native` --cites--> `Arsitektur fondasi desktop`  [EXTRACTED]
  README.md → docs/architecture/README.md
- `Fondasi desktop native` --cites--> `Sistem desain bersama`  [EXTRACTED]
  README.md → docs/design-system/README.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Integritas indeks pada tiga hook Git** — githooks_pre_commit__entry, githooks_pre_merge_commit__entry, githooks_post_commit__entry, agents_snapshot_indeks_git [INFERRED 0.85]
- **Pemeriksaan runtime pada tiga host** — docs_quality_readme_bukti_runtime_per_host, docs_platforms_macos_readme_uji_jendela_di_mac, docs_platforms_windows_readme_uji_jendela_windows, docs_platforms_linux_readme_uji_sesi_grafis_linux [EXTRACTED 1.00]
- **Pemetaan visual native bersama** — docs_design_system_readme_token_desain_bersama, docs_design_system_readme_tema_swiftui, docs_design_system_readme_fluent_design, docs_design_system_readme_css_gtk [EXTRACTED 1.00]

## Communities (36 total, 5 thin omitted)

### Community 0 - "Scope fondasi desktop native"
Cohesion: 0.09
Nodes (37): Fluent Design, Alur dan kepemilikan, Bukti build, runtime dan distribusi, Fondasi Windows, Matriks kompatibilitas Windows, Percobaan WinUI 3 di host Windows, Pesan UI bertipe, Target Windows x64 dan kelayakan ARM64 (+29 more)

### Community 1 - "pre-commit script"
Cohesion: 0.11
Nodes (24): Aktivasi hook per clone, Aturan commit dan integrasi, Integrasi branch ke main, Pembaruan Graphify sebelum commit, Pembaruan semantik dengan skill Graphify, Pemeriksaan SHA origin/main, Snapshot indeks Git, Verifikasi diff dan dokumentasi (+16 more)

### Community 2 - "Fondasi macOS"
Cohesion: 0.13
Nodes (26): API publik Rust, Eksekusi dan uji, hello(name: String) → String, Kepemilikan dan kompatibilitas, Kepemilikan memori dan pemetaan kesalahan, Kontrak Rust bersama, Safe Rust, Uji kontrak Core mandiri (+18 more)

### Community 4 - "hello"
Cohesion: 0.12
Nodes (11): hello(), String, long_name_is_not_cut(), assert_wraps_name(), braces_in_name_stay_plain_text(), every_vector_is_prefix_then_name_unchanged_then_bang(), long_multi_byte_name_is_not_cut(), String (+3 more)

### Community 5 - "Penerimaan dan bukti fondasi"
Cohesion: 0.11
Nodes (21): Bukti build per arsitektur, Bukti native per host, Bukti runtime per host, C01 · Cargo Workspace berhasil dibangun, C25 · Pemeriksaan kepatuhan arsitektur telah dikonfigurasi, C26 · `AGENTS.md` di akar repo memuat aturan pengembangan yang bisa diperiksa, C27 · Dokumentasi arsitektur tersedia, C28 · Skrip build berfungsi pada host yang sesuai (+13 more)

### Community 6 - "workspace_layout.rs"
Cohesion: 0.16
Nodes (17): architecture_rule_table_matches_the_spec_row_by_row(), every_member_inherits_shared_fields_and_uses_desktop_prefix(), first_table_after(), members(), members_are_listed_one_by_one_without_globs(), minor_of(), one_cargo_lock_for_the_whole_workspace(), repo_root() (+9 more)

### Community 7 - "004 - Sapaan di jendela macOS"
Cohesion: 0.12
Nodes (21): C02 · Rust Core dapat dibangun dan diuji secara mandiri dari framework UI native, C03 · Satu implementasi Rust menjadi sumber perilaku sapaan Hello World, C04 · UniFFI berhasil menghasilkan binding Swift untuk macOS, C05 · Aplikasi macOS berhasil memanggil fungsi Rust tersebut, C10 · Aplikasi Linux menggunakan GTK 4 dan libadwaita, C11 · Aplikasi Linux memanggil Rust Core langsung melalui Cargo, C12 · Aplikasi Linux berjalan pada backend Wayland native, C13 · Kompatibilitas X11 diperiksa dan hasil atau keterbatasannya dicatat (+13 more)

### Community 8 - "Alur lead"
Cohesion: 0.11
Nodes (18): 1. Tentukan SCOPE, 2. Analisis dan rencana wave, 3. Mulai sub sesi, 4. Komunikasi, 5. Laporan, 6. Gerbang centang dan merge, 7. Log, 8. Akhir wave (+10 more)

### Community 9 - "Fondasi Linux"
Cohesion: 0.17
Nodes (19): Bukti yang diperlukan di Linux, Compositor Wayland, Evaluasi Flatpak di masa mendatang, Fondasi Linux, GTK 4, gtk-rs, Jendela native dan Rust Core, Kompatibilitas X11 (+11 more)

### Community 10 - "0001. Batas workspace dan pemilik"
Cohesion: 0.11
Nodes (19): 0001. Batas workspace dan pemilik, Aturan arah dependensi, Aturan untuk manifest root, Build plan, Consequences, Decision, Follow-up, Invarian (+11 more)

### Community 11 - "Sistem desain bersama"
Cohesion: 0.31
Nodes (14): Aksesibilitas kontrol native, Alias warna semantik, CSS GTK, Sistem desain bersama, Skema token desain, Tampilan terang dan gelap, Tema SwiftUI, Tipografi dan jarak (+6 more)

### Community 12 - "contract_surface.rs"
Cohesion: 0.22
Nodes (5): manifest_declares_no_dependencies_of_any_kind(), manifest_forbids_unsafe_code_and_warns_on_missing_docs(), manifest_tables(), String, Vec

### Community 13 - "0001. Sapaan di jendela macOS: catatan keputusan"
Cohesion: 0.18
Nodes (11): 0001. Sapaan di jendela macOS: catatan keputusan, Bukti percobaan, Context, Cross check, Keputusan yang lebih kecil, Opsi 1: Binding terlacak sebagai paket Swift lokal, modul Swift yang diuji `swift test`, target Xcode tipis, Opsi 2: Semua kode Swift di target aplikasi Xcode, binding lewat bridging header, Opsi 3: XCFramework sebagai target biner (+3 more)

### Community 14 - "0001. Batas workspace dan pemilik: alasan"
Cohesion: 0.18
Nodes (11): 0001. Batas workspace dan pemilik: alasan, Bukti percobaan, Context, Cross check, Keputusan yang diambil, Opsi 1: Satu workspace, anggota eksplisit, aturan netral host, Opsi 2: Satu workspace dengan `default-members` hanya Core, Opsi 3: Aplikasi OS di luar workspace (+3 more)

### Community 15 - "Milestone 1: Workspace dan kerangka Core"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: Workspace dan kerangka Core, Milestone 2: Peta arsitektur mencatat tata letak nyata, UI / manual, UI / manual (+1 more)

### Community 16 - "Milestone 1: fungsi hello dan uji Done when"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: fungsi hello dan uji Done when, Milestone 2: masukan tepi, aturan kontrak, dan bukti, UI / manual, UI / manual (+1 more)

### Community 17 - "Log koordinasi: `<nama-SCOPE>`"
Cohesion: 0.22
Nodes (8): Blocker terbuka, Dicentang pada wave ini, Keputusan, Keputusan delegasi, Log koordinasi: `<nama-SCOPE>`, Penutupan, Riwayat wave, Sub sesi wave `<n>`

### Community 18 - "Log koordinasi: `000 - Panduan scope`"
Cohesion: 0.22
Nodes (8): Blocker terbuka, Dicentang pada wave ini, Keputusan, Keputusan delegasi, Log koordinasi: `000 - Panduan scope`, Penutupan, Riwayat wave, Sub sesi wave `1`

### Community 19 - "0001. Sapaan di jendela macOS"
Cohesion: 0.22
Nodes (9): 0001. Sapaan di jendela macOS, Build plan, Consequences, Decision, Feature design, Follow-up, Rationale, Requirements (+1 more)

### Community 20 - "0002. Kontrak sapaan bersama"
Cohesion: 0.22
Nodes (9): 0002. Kontrak sapaan bersama, Build plan, Consequences, Decision, Feature design, Follow-up, Rationale, Requirements (+1 more)

### Community 21 - "Peran: architect"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: architect, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 22 - "Peran: checker"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: checker, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 23 - "Peran: debugger"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: debugger, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 24 - "Peran: developer"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: developer, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 25 - "Peran: tester"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: tester, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 26 - "Options considered"
Cohesion: 0.25
Nodes (8): 0002. Kontrak sapaan bersama: catatan keputusan, Context, Keputusan yang lebih kecil, Option 1: Fungsi total, nama dipakai apa adanya, Option 2: Validasi di Core dengan kesalahan bertipe, Option 3: Rapikan diam diam, tanda tangan tetap, Options considered, Rationale

### Community 27 - "Arsitektur fondasi desktop"
Cohesion: 0.29
Nodes (7): Arsitektur fondasi desktop, Aturan arah dependensi, Aturan pengembangan yang akan dicatat, Batas fase fondasi, Pemilik dan arah dependensi, Tata letak yang dituju, Yang sudah ada

### Community 28 - "Langkah manual: `<nama-SCOPE>`"
Cohesion: 0.33
Nodes (5): `F<fitur>-<n>` · `<judul>`, Fitur `<nomor>`: `<nama>` (spec `<NNNN>`), Langkah manual: `<nama-SCOPE>`, Nilai dari user, Ringkasan

### Community 29 - "Langkah manual: `000 - Panduan scope`"
Cohesion: 0.33
Nodes (5): `F001-1` · Bangun dan uji Core di host Windows dan host Linux, Fitur `001`: `Batas workspace dan pemilik` (spec `_root/0001`), Langkah manual: `000 - Panduan scope`, Nilai dari user, Ringkasan

## Knowledge Gaps
- **175 isolated node(s):** `Pemakaian`, `Konteks saat ini`, `1. Tentukan SCOPE`, `2. Analisis dan rencana wave`, `3. Mulai sub sesi` (+170 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **5 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Fondasi desktop native` connect `Fondasi macOS` to `Scope fondasi desktop native`, `pre-commit script`, `scope.md`, `Penerimaan dan bukti fondasi`, `004 - Sapaan di jendela macOS`, `Fondasi Linux`, `Sistem desain bersama`, `Arsitektur fondasi desktop`?**
  _High betweenness centrality (0.073) - this node is a cross-community bridge._
- **Why does `Scope fondasi desktop native` connect `Scope fondasi desktop native` to `Fondasi macOS`, `scope.md`, `Penerimaan dan bukti fondasi`, `004 - Sapaan di jendela macOS`, `Fondasi Linux`, `Sistem desain bersama`?**
  _High betweenness centrality (0.064) - this node is a cross-community bridge._
- **Why does `0001. Batas workspace dan pemilik` connect `0001. Batas workspace dan pemilik` to `scope.md`?**
  _High betweenness centrality (0.047) - this node is a cross-community bridge._
- **What connects `Pemakaian`, `Konteks saat ini`, `1. Tentukan SCOPE` to the rest of the system?**
  _175 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Scope fondasi desktop native` be split into smaller, more focused modules?**
  _Cohesion score 0.08636977058029689 - nodes in this community are weakly interconnected._
- **Should `pre-commit script` be split into smaller, more focused modules?**
  _Cohesion score 0.10804597701149425 - nodes in this community are weakly interconnected._
- **Should `Fondasi macOS` be split into smaller, more focused modules?**
  _Cohesion score 0.12615384615384614 - nodes in this community are weakly interconnected._
Pembaruan hook menggunakan indeks struktural lokal. Hash semantik kosong menandai dokumen yang menunggu ekstraksi semantik.
