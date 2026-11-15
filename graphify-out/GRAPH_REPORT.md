# Graph Report - desktop  (2026-11-15)

## Corpus Check
- 104 files · ~129,489 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1206 nodes · 1906 edges · 79 communities (71 shown, 8 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 38 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- desktop_macos_ffi.swift
- Problem
- quality/README.md
- tokens/mod.rs
- GreetingViewModel
- spacing.schema.json
- config_contract.rs
- typography.schema.json
- public.schema.json
- text.primary
- json.rs
- hello
- Rancangan konfigurasi
- Fondasi Linux
- workspace_layout.rs
- Rancangan
- pre-commit script
- Alur lead
- 0001. Batas workspace dan pemilik
- Penerimaan dan bukti fondasi
- Fondasi Windows
- 000 - Panduan scope
- spacing.json
- Fondasi desktop native
- Milestone 1: Jembatan, binding, dan uji tanpa jendela
- 0002. Batas workspace dan pemilik di macOS
- 0003. Kontrak sapaan bersama di macOS
- properties
- Fondasi macOS
- Scope fondasi desktop native
- Langkah manual: `000 - Panduan scope`
- properties
- 0004. Konfigurasi publik: alasan
- schema.rs
- contract_surface.rs
- colors.schema.json
- 001 - Batas workspace dan pemilik
- 0001. Sapaan di jendela macOS: catatan keputusan
- 0004. Konfigurasi publik di macOS: alasan
- 0001. Batas workspace dan pemilik: alasan
- 0003. Tema bersama dan kontrol native: catatan keputusan
- macos_bridge_ownership.rs
- Log koordinasi: `000 - Panduan scope`
- Milestone 1: Workspace dan kerangka Core
- Milestone 1: fungsi hello dan uji Done when
- 0003. Tema bersama dan kontrol native
- Milestone 1: Alat dan sumber token
- 0006. Satu contoh lokalisasi
- Options considered
- Log koordinasi: `<nama-SCOPE>`
- required
- Verify: spec macos/0002 dan macos/0003 · 2026-11-15
- 0001. Sapaan di jendela macOS
- 0002. Kontrak sapaan bersama
- Aturan pengembangan
- Peran: architect
- Peran: checker
- Peran: debugger
- Peran: developer
- Peran: tester
- Options considered
- Konfigurasi publik
- typography.json
- Arsitektur fondasi desktop
- Langkah manual: `<nama-SCOPE>`
- public.json
- macos-ffi/src/lib.rs
- PackageDescription
- dark
- light
- brief.md
- brief-umum.md
- report.md
- 000 - Panduan scope.brief-umum.md
- desktop-core
- desktop-codegen

## God Nodes (most connected - your core abstractions)
1. `fixture()` - 34 edges
2. `assert_rejected()` - 29 edges
3. `edit()` - 26 edges
4. `Fondasi Linux` - 23 edges
5. `Problem` - 22 edges
6. `Fondasi macOS` - 21 edges
7. `Fondasi Windows` - 19 edges
8. `check_colors()` - 18 edges
9. `Fondasi desktop native` - 16 edges
10. `check_spacing()` - 16 edges

## Surprising Connections (you probably didn't know these)
- `Fondasi desktop native` --cites--> `Aturan commit dan integrasi`  [EXTRACTED]
  README.md → AGENTS.md
- `Fondasi desktop native` --cites--> `Arsitektur fondasi desktop`  [EXTRACTED]
  README.md → docs/architecture/README.md
- `Fondasi desktop native` --cites--> `Fondasi Linux`  [EXTRACTED]
  README.md → docs/platforms/linux/README.md
- `Fondasi desktop native` --cites--> `Fondasi macOS`  [EXTRACTED]
  README.md → docs/platforms/macos/README.md
- `Fondasi desktop native` --cites--> `Fondasi Windows`  [EXTRACTED]
  README.md → docs/platforms/windows/README.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Pemeriksaan runtime pada tiga host** — docs_quality_readme_bukti_runtime_per_host, docs_platforms_macos_readme_uji_jendela_di_mac, docs_platforms_windows_readme_uji_jendela_windows, docs_platforms_linux_readme_uji_sesi_grafis_linux [EXTRACTED 1.00]

## Communities (79 total, 8 thin omitted)

### Community 0 - "desktop_macos_ffi.swift"
Cohesion: 0.05
Nodes (66): Array, createReader(), createWriter(), Data, FfiConverter, FfiConverterByRefBytes, FfiConverterPrimitive, FfiConverterRustBuffer (+58 more)

### Community 1 - "Problem"
Cohesion: 0.08
Nodes (50): Display, ExitCode, Into, Build, check_app_identifier(), check_entries(), check_product_name(), check_public() (+42 more)

### Community 2 - "quality/README.md"
Cohesion: 0.11
Nodes (13): Aturan commit dan integrasi, Commands, Conventions, Gotchas, Key files, Overview, Related specs, Rust Core (`desktop-core`) (+5 more)

### Community 3 - "tokens/mod.rs"
Cohesion: 0.12
Nodes (41): BTreeSet, Item, Iterator, UniqueMap, BaseColor, BaseSpace, check_colors(), check_spacing() (+33 more)

### Community 4 - "GreetingViewModel"
Cohesion: 0.07
Nodes (23): App, DesktopMacApp, .body, CoreGreeter, Greeter, String, GreetingView, .body (+15 more)

### Community 5 - "spacing.schema.json"
Cohesion: 0.05
Nodes (42): additionalProperties, additionalProperties, properties, required, type, additionalProperties, propertyNames, type (+34 more)

### Community 6 - "config_contract.rs"
Cohesion: 0.17
Nodes (39): assert_accepted(), assert_rejected(), edit(), fixture(), Path, PathBuf, t01_rejects_an_extra_local_json(), t02_rejects_an_extra_env_file() (+31 more)

### Community 7 - "typography.schema.json"
Cohesion: 0.06
Nodes (34): additionalProperties, additionalProperties, properties, required, type, $comment, pattern, type (+26 more)

### Community 8 - "public.schema.json"
Cohesion: 0.06
Nodes (34): app_identifier, build, product_name, additionalProperties, description, maxLength, pattern, type (+26 more)

### Community 9 - "text.primary"
Cohesion: 0.07
Nodes (26): dark, description, light, dark, description, light, base, brand.primary (+18 more)

### Community 10 - "json.rs"
Cohesion: 0.13
Nodes (20): A, D, Deserialize, Error, PhantomData, newline(), BTreeMap, Formatter (+12 more)

### Community 11 - "hello"
Cohesion: 0.12
Nodes (11): hello(), String, long_name_is_not_cut(), assert_wraps_name(), braces_in_name_stay_plain_text(), every_vector_is_prefix_then_name_unchanged_then_bang(), long_multi_byte_name_is_not_cut(), String (+3 more)

### Community 12 - "Rancangan konfigurasi"
Cohesion: 0.09
Nodes (23): 0004. Konfigurasi publik, Aturan versi skema, Batas pengaturan khusus OS, Bentuk alat, Build plan, Cara aplikasi memakai nilai, Consequences, Decision (+15 more)

### Community 13 - "Fondasi Linux"
Cohesion: 0.15
Nodes (22): Bukti yang diperlukan di Linux, Compositor Wayland, Evaluasi Flatpak di masa mendatang, Fondasi Linux, GTK 4, gtk-rs, Jendela native dan Rust Core, Kompatibilitas X11 (+14 more)

### Community 14 - "workspace_layout.rs"
Cohesion: 0.16
Nodes (17): architecture_rule_table_matches_the_spec_row_by_row(), every_member_inherits_shared_fields_and_uses_desktop_prefix(), first_table_after(), members(), members_are_listed_one_by_one_without_globs(), minor_of(), one_cargo_lock_for_the_whole_workspace(), repo_root() (+9 more)

### Community 15 - "Rancangan"
Cohesion: 0.10
Nodes (21): 0004. Konfigurasi publik di macOS, Alur nilai, Batas setelan di macOS, Build plan, Consequences, Decision, Follow-up, Invarian (+13 more)

### Community 16 - "pre-commit script"
Cohesion: 0.14
Nodes (16): post-commit script, Rekonsiliasi indeks sesudah git commit --only, pre-commit script, Empat artefak Graphify dalam satu commit, Isolasi riwayat kueri lokal, Preservasi edit unstaged pada artefak, Validasi artefak sebelum staging, pre-merge-commit script (+8 more)

### Community 17 - "Alur lead"
Cohesion: 0.11
Nodes (18): 1. Tentukan SCOPE, 2. Analisis dan rencana wave, 3. Mulai sub sesi, 4. Komunikasi, 5. Laporan, 6. Gerbang centang dan merge, 7. Log, 8. Akhir wave (+10 more)

### Community 18 - "0001. Batas workspace dan pemilik"
Cohesion: 0.11
Nodes (19): 0001. Batas workspace dan pemilik, Aturan arah dependensi, Aturan untuk manifest root, Build plan, Consequences, Decision, Follow-up, Invarian (+11 more)

### Community 19 - "Penerimaan dan bukti fondasi"
Cohesion: 0.14
Nodes (18): Apple Silicon ARM64 dan Intel bersyarat, Uji jendela di Mac, Xcode, Uji jendela Windows, Bukti build per arsitektur, Bukti native per host, Bukti runtime per host, C28 · Skrip build berfungsi pada host yang sesuai (+10 more)

### Community 20 - "Fondasi Windows"
Cohesion: 0.18
Nodes (18): Alur dan kepemilikan, Bukti build, runtime dan distribusi, Fondasi Windows, Matriks kompatibilitas Windows, Percobaan WinUI 3 di host Windows, Pesan UI bertipe, Target Windows x64 dan kelayakan ARM64, Tema, resource dan perilaku native (+10 more)

### Community 21 - "000 - Panduan scope"
Cohesion: 0.15
Nodes (16): C04 · UniFFI berhasil menghasilkan binding Swift untuk macOS, C05 · Aplikasi macOS berhasil memanggil fungsi Rust tersebut, C14 · Antarmuka pada ketiga OS menampilkan hasil aktual dari fungsi Rust yang sama, C15 · Token desain memiliki satu sumber acuan, C16 · Ketiga aplikasi memakai token hasil generasi atau pemetaan yang dapat diulang, C17 · Antarmuka memperlihatkan tampilan terang dan gelap, C23 · Kepemilikan pada batas FFI macOS terdokumentasi dan divalidasi bila relevan, 000 - Panduan scope (+8 more)

### Community 22 - "spacing.json"
Cohesion: 0.13
Nodes (14): base, space.large, space.medium, description, ref, description, ref, $schema (+6 more)

### Community 23 - "Fondasi desktop native"
Cohesion: 0.19
Nodes (14): API publik Rust, Eksekusi dan uji, hello(name: String) → String, Kepemilikan dan kompatibilitas, Kontrak Rust bersama, Safe Rust, Uji kontrak Core mandiri, Sistem desain bersama (+6 more)

### Community 24 - "Milestone 1: Jembatan, binding, dan uji tanpa jendela"
Cohesion: 0.14
Nodes (13): Acceptance-criteria coverage, Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Commands, Milestone 1: Jembatan, binding, dan uji tanpa jendela, Milestone 2: Jendela SwiftUI dan proyek Xcode (+5 more)

### Community 25 - "0002. Batas workspace dan pemilik di macOS"
Cohesion: 0.14
Nodes (14): 0002. Batas workspace dan pemilik di macOS, Build plan, Consequences, Context, Decision, Feature design, Follow-up, Opsi 1: Rujuk bukti yang ada, tambah satu langkah untuk peta arsitektur (+6 more)

### Community 26 - "0003. Kontrak sapaan bersama di macOS"
Cohesion: 0.14
Nodes (14): 0003. Kontrak sapaan bersama di macOS, Build plan, Consequences, Context, Decision, Feature design, Follow-up, Opsi 1: Rujuk bukti yang ada, tanpa langkah build (+6 more)

### Community 27 - "properties"
Cohesion: 0.17
Nodes (13): propertyNames, type, properties, base, $schema, schemaVersion, semantic, pattern (+5 more)

### Community 28 - "Fondasi macOS"
Cohesion: 0.23
Nodes (13): Kepemilikan memori dan pemetaan kesalahan, AppKit, Batas Swift dan Rust, Binding Swift yang dihasilkan, Bukti penerimaan pada Mac, Distribusi, Fondasi macOS, Jendela dan interaksi (+5 more)

### Community 29 - "Scope fondasi desktop native"
Cohesion: 0.24
Nodes (13): C21 · Kepemilikan aset bersama terdokumentasi, Validasi keluaran generator, 011 - Aset merek bersama, Scope fondasi desktop native, Aset merek, Aset merek berversi, Fallback dan validasi placeholder, Katalog lokalisasi bersama (+5 more)

### Community 30 - "Langkah manual: `000 - Panduan scope`"
Cohesion: 0.15
Nodes (12): `F001-1` · Bangun dan uji Core di host Windows dan host Linux, `F004-1` · Alur sapaan di jendela, `F004-2` · Penggunaan nyata, `F008-1` · Bangun, uji, dan periksa token di host Windows dan host Linux, `F009-1` · Judul jendela dari konfigurasi publik dan nama menu aplikasi, Fitur `001`: `Batas workspace dan pemilik` (spec `_root/0001`), Fitur `004`: `Sapaan di jendela macOS` (spec `macos/0001`), Fitur `008`: `Tema bersama dan kontrol native` (spec `_root/0003`) (+4 more)

### Community 31 - "properties"
Cohesion: 0.17
Nodes (12): properties, pattern, type, pattern, type, type, description, on (+4 more)

### Community 32 - "0004. Konfigurasi publik: alasan"
Cohesion: 0.17
Nodes (12): 0004. Konfigurasi publik: alasan, Bukti yang diperiksa, Context, Cross check, Keputusan yang diambil, Opsi 1: Satu file nilai JSON dengan skema tertutup di `shared/config/`, Opsi 2: File nilai TOML dengan aturan di kode validasi, Opsi 3: Tanpa file bersama, nilai tetap di proyek native (+4 more)

### Community 33 - "schema.rs"
Cohesion: 0.36
Nodes (11): closed(), colors(), document(), group(), output(), outputs(), Value, Vec (+3 more)

### Community 34 - "contract_surface.rs"
Cohesion: 0.22
Nodes (5): manifest_declares_no_dependencies_of_any_kind(), manifest_forbids_unsafe_code_and_warns_on_missing_docs(), manifest_tables(), String, Vec

### Community 35 - "colors.schema.json"
Cohesion: 0.18
Nodes (10): additionalProperties, $comment, base, $schema, schemaVersion, semantic, required, $schema (+2 more)

### Community 36 - "001 - Batas workspace dan pemilik"
Cohesion: 0.18
Nodes (11): C01 · Cargo Workspace berhasil dibangun, C02 · Rust Core dapat dibangun dan diuji secara mandiri dari framework UI native, C03 · Satu implementasi Rust menjadi sumber perilaku sapaan Hello World, C22 · Tersedia aturan kontrak publik Rust yang dipakai bersama, C25 · Pemeriksaan kepatuhan arsitektur telah dikonfigurasi, C26 · `AGENTS.md` di akar repo memuat aturan pengembangan yang bisa diperiksa, C27 · Dokumentasi arsitektur tersedia, C32 · Implementasi hanya memuat layanan yang dibutuhkan oleh fondasi. Layanan untuk fitur mendatang tetap di luar cakupan (+3 more)

### Community 37 - "0001. Sapaan di jendela macOS: catatan keputusan"
Cohesion: 0.18
Nodes (11): 0001. Sapaan di jendela macOS: catatan keputusan, Bukti percobaan, Context, Cross check, Keputusan yang lebih kecil, Opsi 1: Binding terlacak sebagai paket Swift lokal, modul Swift yang diuji `swift test`, target Xcode tipis, Opsi 2: Semua kode Swift di target aplikasi Xcode, binding lewat bridging header, Opsi 3: XCFramework sebagai target biner (+3 more)

### Community 38 - "0004. Konfigurasi publik di macOS: alasan"
Cohesion: 0.18
Nodes (11): 0004. Konfigurasi publik di macOS: alasan, Bukti percobaan, Context, Cross check, Keputusan yang diambil, Opsi 1: Berkas xcconfig hasil generator sebagai dasar setelan target, Opsi 2: Pemeriksaan teks atas `project.pbxproj`, Opsi 3: Fase skrip yang membaca `public.json` saat build (+3 more)

### Community 39 - "0001. Batas workspace dan pemilik: alasan"
Cohesion: 0.18
Nodes (11): 0001. Batas workspace dan pemilik: alasan, Bukti percobaan, Context, Cross check, Keputusan yang diambil, Opsi 1: Satu workspace, anggota eksplisit, aturan netral host, Opsi 2: Satu workspace dengan `default-members` hanya Core, Opsi 3: Aplikasi OS di luar workspace (+3 more)

### Community 40 - "0003. Tema bersama dan kontrol native: catatan keputusan"
Cohesion: 0.18
Nodes (11): 0003. Tema bersama dan kontrol native: catatan keputusan, Bukti percobaan, Context, Cross check, Keputusan yang lebih kecil, Opsi 1: Alat Rust di workspace, bentuk token sebagai tipe, skema dihasilkan, Opsi 2: Skema JSON tulisan tangan dan crate mesin JSON Schema, Opsi 3: Alat siap pakai dari luar dengan format standar (+3 more)

### Community 41 - "macos_bridge_ownership.rs"
Cohesion: 0.33
Nodes (7): bridge_sources_hold_no_unsafe(), files_with_extension(), hand_written_swift_uses_no_pointers(), repo_root(), Path, PathBuf, Vec

### Community 42 - "Log koordinasi: `000 - Panduan scope`"
Cohesion: 0.20
Nodes (9): Blocker terbuka, Dicentang pada wave ini, Keputusan, Keputusan delegasi, Log koordinasi: `000 - Panduan scope`, Penutupan, Riwayat wave, Sub sesi wave `1` (+1 more)

### Community 43 - "Milestone 1: Workspace dan kerangka Core"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: Workspace dan kerangka Core, Milestone 2: Peta arsitektur mencatat tata letak nyata, UI / manual, UI / manual (+1 more)

### Community 44 - "Milestone 1: fungsi hello dan uji Done when"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: fungsi hello dan uji Done when, Milestone 2: masukan tepi, aturan kontrak, dan bukti, UI / manual, UI / manual (+1 more)

### Community 45 - "0003. Tema bersama dan kontrol native"
Cohesion: 0.20
Nodes (10): 0003. Tema bersama dan kontrol native, Build plan, Consequences, Decision, Feature design, Follow-up, Perubahan dokumen, Rationale (+2 more)

### Community 46 - "Milestone 1: Alat dan sumber token"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: Alat dan sumber token, Milestone 2: Dokumen mengikuti keadaan nyata, UI / manual, UI / manual (+1 more)

### Community 47 - "0006. Satu contoh lokalisasi"
Cohesion: 0.20
Nodes (10): 0006. Satu contoh lokalisasi, Build plan, Consequences, Decision, Feature design, Follow-up, Perubahan dokumen rujukan, Rationale (+2 more)

### Community 48 - "Options considered"
Cohesion: 0.20
Nodes (10): 0006. Satu contoh lokalisasi: catatan keputusan, Bukti percobaan, Context, Keputusan yang lebih kecil, Option 1: Satu katalog JSON netral untuk semua bahasa, resource native dibuat dari katalog itu, Option 2: Satu berkas JSON per bahasa, Option 3: Format native satu OS sebagai sumber bersama, Option 4: Pustaka lokalisasi Rust yang dipakai ketiga aplikasi saat berjalan (+2 more)

### Community 49 - "Log koordinasi: `<nama-SCOPE>`"
Cohesion: 0.22
Nodes (8): Blocker terbuka, Dicentang pada wave ini, Keputusan, Keputusan delegasi, Log koordinasi: `<nama-SCOPE>`, Penutupan, Riwayat wave, Sub sesi wave `<n>`

### Community 50 - "required"
Cohesion: 0.28
Nodes (9): additionalProperties, required, type, additionalProperties, description, value, additionalProperties, dark (+1 more)

### Community 51 - "Verify: spec macos/0002 dan macos/0003 · 2026-11-15"
Cohesion: 0.22
Nodes (9): Catatan, Host dan versi, Keadaan `verify.md` yang dirujuk, Spec macos/0002 (scope 001), Spec macos/0003 (scope 003), Tafsiran yang diambil, Verdict, Verify: spec macos/0002 dan macos/0003 · 2026-11-15 (+1 more)

### Community 52 - "0001. Sapaan di jendela macOS"
Cohesion: 0.22
Nodes (9): 0001. Sapaan di jendela macOS, Build plan, Consequences, Decision, Feature design, Follow-up, Rationale, Requirements (+1 more)

### Community 53 - "0002. Kontrak sapaan bersama"
Cohesion: 0.22
Nodes (9): 0002. Kontrak sapaan bersama, Build plan, Consequences, Decision, Feature design, Follow-up, Rationale, Requirements (+1 more)

### Community 54 - "Aturan pengembangan"
Cohesion: 0.25
Nodes (8): Agent skills, Aturan pengembangan, Build approach, Commands, Context files, Rules, Specs, Stack

### Community 55 - "Peran: architect"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: architect, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 56 - "Peran: checker"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: checker, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 57 - "Peran: debugger"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: debugger, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 58 - "Peran: developer"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: developer, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 59 - "Peran: tester"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: tester, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 60 - "Options considered"
Cohesion: 0.25
Nodes (8): 0002. Kontrak sapaan bersama: catatan keputusan, Context, Keputusan yang lebih kecil, Option 1: Fungsi total, nama dipakai apa adanya, Option 2: Validasi di Core dengan kesalahan bertipe, Option 3: Rapikan diam diam, tanda tangan tetap, Options considered, Rationale

### Community 61 - "Konfigurasi publik"
Cohesion: 0.25
Nodes (7): Batas pengaturan khusus OS, Cara aplikasi memakai nilai, Cara mengubah, Konfigurasi publik, Riwayat skema, Skema versi 1, Yang tidak boleh ada di folder ini

### Community 62 - "typography.json"
Cohesion: 0.29
Nodes (6): description, weight, roles, body, $schema, schemaVersion

### Community 63 - "Arsitektur fondasi desktop"
Cohesion: 0.29
Nodes (7): Arsitektur fondasi desktop, Aturan arah dependensi, Aturan pengembangan yang akan dicatat, Batas fase fondasi, Pemilik dan arah dependensi, Tata letak yang dituju, Yang sudah ada

### Community 64 - "Langkah manual: `<nama-SCOPE>`"
Cohesion: 0.33
Nodes (5): `F<fitur>-<n>` · `<judul>`, Fitur `<nomor>`: `<nama>` (spec `<NNNN>`), Langkah manual: `<nama-SCOPE>`, Nilai dari user, Ringkasan

### Community 65 - "public.json"
Cohesion: 0.33
Nodes (5): build, app_identifier, product_name, $schema, schemaVersion

### Community 68 - "dark"
Cohesion: 0.67
Nodes (3): pattern, type, dark

### Community 69 - "light"
Cohesion: 0.67
Nodes (3): pattern, type, light

## Knowledge Gaps
- **489 isolated node(s):** `Pemakaian`, `Konteks saat ini`, `1. Tentukan SCOPE`, `2. Analisis dan rencana wave`, `3. Mulai sub sesi` (+484 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **8 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Penerimaan dan bukti fondasi` connect `Penerimaan dan bukti fondasi` to `quality/README.md`, `Scope fondasi desktop native`, `Fondasi desktop native`?**
  _High betweenness centrality (0.028) - this node is a cross-community bridge._
- **Why does `Validasi keluaran generator` connect `Scope fondasi desktop native` to `pre-commit script`, `Penerimaan dan bukti fondasi`, `Fondasi macOS`?**
  _High betweenness centrality (0.018) - this node is a cross-community bridge._
- **Why does `0004. Konfigurasi publik` connect `Rancangan konfigurasi` to `quality/README.md`?**
  _High betweenness centrality (0.017) - this node is a cross-community bridge._
- **What connects `Pemakaian`, `Konteks saat ini`, `1. Tentukan SCOPE` to the rest of the system?**
  _489 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `desktop_macos_ffi.swift` be split into smaller, more focused modules?**
  _Cohesion score 0.050883898709985664 - nodes in this community are weakly interconnected._
- **Should `Problem` be split into smaller, more focused modules?**
  _Cohesion score 0.08020050125313283 - nodes in this community are weakly interconnected._
- **Should `quality/README.md` be split into smaller, more focused modules?**
  _Cohesion score 0.1063973063973064 - nodes in this community are weakly interconnected._
Pembaruan hook menggunakan indeks struktural lokal. Hash semantik kosong menandai dokumen yang menunggu ekstraksi semantik.
