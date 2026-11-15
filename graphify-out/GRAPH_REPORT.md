# Graph Report - desktop  (2026-11-15)

## Corpus Check
- 137 files · ~199,708 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1783 nodes · 2940 edges · 104 communities (96 shown, 8 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 48 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- desktop_macos_ffi.swift
- Problem
- quality/README.md
- Findings
- tokens_rules.rs
- GreetingViewModel
- catalog.schema.json
- tokens/schema.rs
- localization/mod.rs
- spacing.schema.json
- assets_rules.rs
- config_contract.rs
- brand.schema.json
- typography.schema.json
- public.schema.json
- config_rules.rs
- localization_rules.rs
- text.primary
- Rancangan pemeriksaan
- hello
- Rancangan konfigurasi
- assets_contract.rs
- Fondasi Linux
- pre-commit script
- Rancangan
- Rancangan aset merek
- assets/mod.rs
- workspace_layout.rs
- Alur lead
- 0001. Batas workspace dan pemilik
- Fondasi Windows
- Langkah manual: `000 - Panduan scope`
- architecture_contract.rs
- Fondasi desktop native
- Penerimaan dan bukti fondasi
- 000 - Panduan scope
- spacing.json
- Fondasi macOS
- svg.rs
- Milestone 1: Jembatan, binding, dan uji tanpa jendela
- 0002. Batas workspace dan pemilik di macOS
- 0003. Kontrak sapaan bersama di macOS
- properties
- 0007. Aturan pengembangan dan pemeriksaan arsitektur: alasan
- catalog.json
- properties
- 0004. Konfigurasi publik: alasan
- tokens_contract.rs
- contract_surface.rs
- colors.schema.json
- 001 - Batas workspace dan pemilik
- 0001. Sapaan di jendela macOS: catatan keputusan
- 0004. Konfigurasi publik di macOS: alasan
- 0005. Tema bersama dan kontrol native di macOS: catatan keputusan
- 0001. Batas workspace dan pemilik: alasan
- 0003. Tema bersama dan kontrol native: catatan keputusan
- 0005. Aset merek bersama: alasan
- Aset merek bersama
- Log koordinasi: `000 - Panduan scope`
- Milestone 1: Workspace dan kerangka Core
- Milestone 1: fungsi hello dan uji Done when
- 0003. Tema bersama dan kontrol native
- Milestone 1: Alat dan sumber token
- Milestone 1: Sumber nilai, pemeriksaan, dan skema hasil generator
- Milestone 1: Sumber, manifest, aturan, dan pemeriksaan aset merek
- 0006. Satu contoh lokalisasi
- Options considered
- Milestone 1: Katalog dan sumber localization
- Milestone 1: Sumber architecture dan aturan dependensi
- macos_bridge_ownership.rs
- Log koordinasi: `<nama-SCOPE>`
- required
- Verify: spec macos/0002 dan macos/0003 · 2026-11-15
- 0001. Sapaan di jendela macOS
- 0005. Tema bersama dan kontrol native di macOS
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
- brand.json
- Langkah manual: `<nama-SCOPE>`
- public.json
- macos-ffi/src/lib.rs
- Resource bersama
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
1. `Problem` - 37 edges
2. `fixture()` - 34 edges
3. `assert_rejected()` - 29 edges
4. `edit()` - 25 edges
5. `generate()` - 22 edges
6. `Fondasi Linux` - 20 edges
7. `Findings` - 19 edges
8. `check_colors()` - 19 edges
9. `Fondasi macOS` - 18 edges
10. `Fondasi desktop native` - 16 edges

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

## Communities (104 total, 8 thin omitted)

### Community 0 - "desktop_macos_ffi.swift"
Cohesion: 0.05
Nodes (66): Array, createReader(), createWriter(), Data, FfiConverter, FfiConverterByRefBytes, FfiConverterPrimitive, FfiConverterRustBuffer (+58 more)

### Community 1 - "Problem"
Cohesion: 0.08
Nodes (61): BTreeSet, Deserialize, Display, Item, Iterator, Build, check_app_identifier(), check_entries() (+53 more)

### Community 2 - "quality/README.md"
Cohesion: 0.10
Nodes (15): Commands, Conventions, Gotchas, Key files, Overview, Related specs, Rust Core (`desktop-core`), C18 · Aturan konfigurasi bersama terdokumentasi (+7 more)

### Community 3 - "Findings"
Cohesion: 0.08
Nodes (45): check(), check_bridge(), check_config(), check_core(), check_greeting(), check_imports(), check_lints(), check_remote_packages() (+37 more)

### Community 4 - "tokens_rules.rs"
Cohesion: 0.09
Nodes (50): ExitCode, Into, check(), generate(), generate_command(), Output, Plan, repo_root() (+42 more)

### Community 5 - "GreetingViewModel"
Cohesion: 0.07
Nodes (23): App, DesktopMacApp, .body, CoreGreeter, Greeter, String, GreetingView, .body (+15 more)

### Community 6 - "catalog.schema.json"
Cohesion: 0.05
Nodes (45): entries, locales, sourceLocale, text, additionalProperties, additionalProperties, minLength, properties (+37 more)

### Community 7 - "tokens/schema.rs"
Cohesion: 0.08
Nodes (37): A, D, Error, PhantomData, contents(), String, Value, schema() (+29 more)

### Community 8 - "localization/mod.rs"
Cohesion: 0.12
Nodes (39): Catalog, CatalogFile, check_entry(), check_locales(), Entry, EntryFile, is_valid_description(), is_valid_key() (+31 more)

### Community 9 - "spacing.schema.json"
Cohesion: 0.05
Nodes (42): additionalProperties, additionalProperties, properties, required, type, additionalProperties, propertyNames, type (+34 more)

### Community 10 - "assets_rules.rs"
Cohesion: 0.13
Nodes (39): AsRef, asset_id_edges(), assets_command_exit_codes(), broken_source_writes_nothing(), clean(), committed_plan(), early_s_rule_failures_skip_the_later_rules(), edit() (+31 more)

### Community 11 - "config_contract.rs"
Cohesion: 0.17
Nodes (39): assert_accepted(), assert_rejected(), edit(), fixture(), Path, PathBuf, t01_rejects_an_extra_local_json(), t02_rejects_an_extra_env_file() (+31 more)

### Community 12 - "brand.schema.json"
Cohesion: 0.05
Nodes (39): any, app-icon, assets, light-dark, variants, additionalProperties, description, properties (+31 more)

### Community 13 - "typography.schema.json"
Cohesion: 0.06
Nodes (34): additionalProperties, additionalProperties, properties, required, type, $comment, pattern, type (+26 more)

### Community 14 - "public.schema.json"
Cohesion: 0.06
Nodes (34): app_identifier, build, product_name, additionalProperties, description, maxLength, pattern, type (+26 more)

### Community 15 - "config_rules.rs"
Cohesion: 0.15
Nodes (30): app_identifier_shape_edges_are_rejected(), app_identifier_short_shapes_are_accepted(), broken_source_writes_nothing(), committed_plan(), config_command_exit_codes(), content_rules_are_skipped_when_public_json_is_unreadable(), edit(), every_output_follows_the_output_rules() (+22 more)

### Community 16 - "localization_rules.rs"
Cohesion: 0.14
Nodes (29): at(), broken_or_unreadable_catalog_writes_nothing(), catalog(), committed_plan(), edited(), entry(), entry_problems_follow_the_check_order(), every_output_follows_the_output_rules() (+21 more)

### Community 17 - "text.primary"
Cohesion: 0.07
Nodes (26): dark, description, light, dark, description, light, base, brand.primary (+18 more)

### Community 18 - "Rancangan pemeriksaan"
Cohesion: 0.08
Nodes (25): 0007. Aturan pengembangan dan pemeriksaan arsitektur, Aturan dependensi yang dibaca mesin, Aturan yang diperiksa manusia, Aturan yang diperiksa mesin, Bentuk alat, Build plan, Cara menambah atau mengubah aturan, Cara tiap aturan membaca repo (+17 more)

### Community 19 - "hello"
Cohesion: 0.12
Nodes (11): hello(), String, long_name_is_not_cut(), assert_wraps_name(), braces_in_name_stay_plain_text(), every_vector_is_prefix_then_name_unchanged_then_bang(), long_multi_byte_name_is_not_cut(), String (+3 more)

### Community 20 - "Rancangan konfigurasi"
Cohesion: 0.09
Nodes (23): 0004. Konfigurasi publik, Aturan versi skema, Batas pengaturan khusus OS, Bentuk alat, Build plan, Cara aplikasi memakai nilai, Consequences, Decision (+15 more)

### Community 21 - "assets_contract.rs"
Cohesion: 0.19
Nodes (20): accepts_valid_sources(), Change, changed_icon(), fixture(), folder_case(), make_light_dark(), messages(), read() (+12 more)

### Community 22 - "Fondasi Linux"
Cohesion: 0.15
Nodes (22): Bukti yang diperlukan di Linux, Compositor Wayland, Evaluasi Flatpak di masa mendatang, Fondasi Linux, GTK 4, gtk-rs, Jendela native dan Rust Core, Kompatibilitas X11 (+14 more)

### Community 23 - "pre-commit script"
Cohesion: 0.13
Nodes (17): Validasi keluaran generator, post-commit script, Rekonsiliasi indeks sesudah git commit --only, pre-commit script, Empat artefak Graphify dalam satu commit, Isolasi riwayat kueri lokal, Preservasi edit unstaged pada artefak, Validasi artefak sebelum staging (+9 more)

### Community 24 - "Rancangan"
Cohesion: 0.10
Nodes (21): 0004. Konfigurasi publik di macOS, Alur nilai, Batas setelan di macOS, Build plan, Consequences, Decision, Follow-up, Invarian (+13 more)

### Community 25 - "Rancangan aset merek"
Cohesion: 0.10
Nodes (21): 0005. Aset merek bersama, Aturan sumber, Bentuk alat, Build plan, Consequences, Decision, Follow-up, Invarian (+13 more)

### Community 26 - "assets/mod.rs"
Cohesion: 0.27
Nodes (18): Asset, AssetFile, Brand, check_folder(), check_sources(), entries(), is_valid_id(), Kind (+10 more)

### Community 27 - "workspace_layout.rs"
Cohesion: 0.16
Nodes (17): architecture_rule_table_matches_the_spec_row_by_row(), every_member_inherits_shared_fields_and_uses_desktop_prefix(), first_table_after(), members(), members_are_listed_one_by_one_without_globs(), minor_of(), one_cargo_lock_for_the_whole_workspace(), repo_root() (+9 more)

### Community 28 - "Alur lead"
Cohesion: 0.11
Nodes (18): 1. Tentukan SCOPE, 2. Analisis dan rencana wave, 3. Mulai sub sesi, 4. Komunikasi, 5. Laporan, 6. Gerbang centang dan merge, 7. Log, 8. Akhir wave (+10 more)

### Community 29 - "0001. Batas workspace dan pemilik"
Cohesion: 0.11
Nodes (19): 0001. Batas workspace dan pemilik, Aturan arah dependensi, Aturan untuk manifest root, Build plan, Consequences, Decision, Follow-up, Invarian (+11 more)

### Community 30 - "Fondasi Windows"
Cohesion: 0.18
Nodes (18): Alur dan kepemilikan, Bukti build, runtime dan distribusi, Fondasi Windows, Matriks kompatibilitas Windows, Percobaan WinUI 3 di host Windows, Pesan UI bertipe, Target Windows x64 dan kelayakan ARM64, Tema, resource dan perilaku native (+10 more)

### Community 31 - "Langkah manual: `000 - Panduan scope`"
Cohesion: 0.11
Nodes (17): `F001-1` · Bangun dan uji Core di host Windows dan host Linux, `F002-1` · Bangun, uji, dan jalankan pemeriksaan arsitektur di host Windows dan host Linux, `F004-1` · Alur sapaan di jendela, `F004-2` · Penggunaan nyata, `F008-1` · Bangun, uji, dan periksa token di host Windows dan host Linux, `F008-2` · Tampilan terang dan gelap dari token dan kontrol native, `F009-1` · Judul jendela dari konfigurasi publik dan nama menu aplikasi, `F011-1` · Mengganti ikon penanda dengan ikon aplikasi nyata (+9 more)

### Community 32 - "architecture_contract.rs"
Cohesion: 0.23
Nodes (14): accepts_allowed_shapes(), apply(), copy_repo(), copy_tree(), Edit, lock_block(), Rejected, rejects_broken_repos() (+6 more)

### Community 33 - "Fondasi desktop native"
Cohesion: 0.17
Nodes (16): Aturan commit dan integrasi, API publik Rust, Eksekusi dan uji, hello(name: String) → String, Kepemilikan dan kompatibilitas, Kontrak Rust bersama, Safe Rust, Uji kontrak Core mandiri (+8 more)

### Community 34 - "Penerimaan dan bukti fondasi"
Cohesion: 0.17
Nodes (16): Uji jendela di Mac, Uji jendela Windows, Bukti build per arsitektur, Bukti native per host, Bukti runtime per host, C28 · Skrip build berfungsi pada host yang sesuai, C29 · Alur kerja GitHub Actions untuk fondasi sudah dikonfigurasi, C30 · Pengujian yang relevan sudah dijalankan (+8 more)

### Community 35 - "000 - Panduan scope"
Cohesion: 0.15
Nodes (16): C04 · UniFFI berhasil menghasilkan binding Swift untuk macOS, C05 · Aplikasi macOS berhasil memanggil fungsi Rust tersebut, C14 · Antarmuka pada ketiga OS menampilkan hasil aktual dari fungsi Rust yang sama, C15 · Token desain memiliki satu sumber acuan, C16 · Ketiga aplikasi memakai token hasil generasi atau pemetaan yang dapat diulang, C17 · Antarmuka memperlihatkan tampilan terang dan gelap, C23 · Kepemilikan pada batas FFI macOS terdokumentasi dan divalidasi bila relevan, 000 - Panduan scope (+8 more)

### Community 36 - "spacing.json"
Cohesion: 0.13
Nodes (14): base, space.large, space.medium, description, ref, description, ref, $schema (+6 more)

### Community 37 - "Fondasi macOS"
Cohesion: 0.20
Nodes (15): Kepemilikan memori dan pemetaan kesalahan, AppKit, Apple Silicon ARM64 dan Intel bersyarat, Batas Swift dan Rust, Binding Swift yang dihasilkan, Bukti penerimaan pada Mac, Distribusi, Fondasi macOS (+7 more)

### Community 38 - "svg.rs"
Cohesion: 0.31
Nodes (13): FnMut, check_elements(), check_entities(), check_events(), check_references(), check_root(), check_svg(), is_valid_view_box() (+5 more)

### Community 39 - "Milestone 1: Jembatan, binding, dan uji tanpa jendela"
Cohesion: 0.14
Nodes (13): Acceptance-criteria coverage, Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Commands, Milestone 1: Jembatan, binding, dan uji tanpa jendela, Milestone 2: Jendela SwiftUI dan proyek Xcode (+5 more)

### Community 40 - "0002. Batas workspace dan pemilik di macOS"
Cohesion: 0.14
Nodes (14): 0002. Batas workspace dan pemilik di macOS, Build plan, Consequences, Context, Decision, Feature design, Follow-up, Opsi 1: Rujuk bukti yang ada, tambah satu langkah untuk peta arsitektur (+6 more)

### Community 41 - "0003. Kontrak sapaan bersama di macOS"
Cohesion: 0.14
Nodes (14): 0003. Kontrak sapaan bersama di macOS, Build plan, Consequences, Context, Decision, Feature design, Follow-up, Opsi 1: Rujuk bukti yang ada, tanpa langkah build (+6 more)

### Community 42 - "properties"
Cohesion: 0.17
Nodes (13): propertyNames, type, properties, base, $schema, schemaVersion, semantic, pattern (+5 more)

### Community 43 - "0007. Aturan pengembangan dan pemeriksaan arsitektur: alasan"
Cohesion: 0.15
Nodes (12): 0007. Aturan pengembangan dan pemeriksaan arsitektur: alasan, Bukti percobaan, Context, Cross check, Jawaban atas masukan `/audit`, Keputusan yang diambil, Opsi 1: Sumber `architecture` di `desktop-codegen`, Opsi 2: Paket pemeriksa sendiri di `tooling/validation/` (+4 more)

### Community 44 - "catalog.json"
Cohesion: 0.15
Nodes (12): en, id, entries, greeting.submit, description, text, $schema, locales (+4 more)

### Community 45 - "properties"
Cohesion: 0.17
Nodes (12): properties, pattern, type, pattern, type, type, description, on (+4 more)

### Community 46 - "0004. Konfigurasi publik: alasan"
Cohesion: 0.17
Nodes (12): 0004. Konfigurasi publik: alasan, Bukti yang diperiksa, Context, Cross check, Keputusan yang diambil, Opsi 1: Satu file nilai JSON dengan skema tertutup di `shared/config/`, Opsi 2: File nilai TOML dengan aturan di kode validasi, Opsi 3: Tanpa file bersama, nilai tetap di proyek native (+4 more)

### Community 47 - "tokens_contract.rs"
Cohesion: 0.27
Nodes (9): Case, check_reports_stale_and_missing_outputs_and_generate_repairs_them(), copy_sources(), File, read(), rejects_broken_sources(), resolves_light_and_dark(), Path (+1 more)

### Community 48 - "contract_surface.rs"
Cohesion: 0.22
Nodes (5): manifest_declares_no_dependencies_of_any_kind(), manifest_forbids_unsafe_code_and_warns_on_missing_docs(), manifest_tables(), String, Vec

### Community 49 - "colors.schema.json"
Cohesion: 0.18
Nodes (10): additionalProperties, $comment, base, $schema, schemaVersion, semantic, required, $schema (+2 more)

### Community 50 - "001 - Batas workspace dan pemilik"
Cohesion: 0.18
Nodes (11): C01 · Cargo Workspace berhasil dibangun, C02 · Rust Core dapat dibangun dan diuji secara mandiri dari framework UI native, C03 · Satu implementasi Rust menjadi sumber perilaku sapaan Hello World, C22 · Tersedia aturan kontrak publik Rust yang dipakai bersama, C25 · Pemeriksaan kepatuhan arsitektur telah dikonfigurasi, C26 · `AGENTS.md` di akar repo memuat aturan pengembangan yang bisa diperiksa, C27 · Dokumentasi arsitektur tersedia, C32 · Implementasi hanya memuat layanan yang dibutuhkan oleh fondasi. Layanan untuk fitur mendatang tetap di luar cakupan (+3 more)

### Community 51 - "0001. Sapaan di jendela macOS: catatan keputusan"
Cohesion: 0.18
Nodes (11): 0001. Sapaan di jendela macOS: catatan keputusan, Bukti percobaan, Context, Cross check, Keputusan yang lebih kecil, Opsi 1: Binding terlacak sebagai paket Swift lokal, modul Swift yang diuji `swift test`, target Xcode tipis, Opsi 2: Semua kode Swift di target aplikasi Xcode, binding lewat bridging header, Opsi 3: XCFramework sebagai target biner (+3 more)

### Community 52 - "0004. Konfigurasi publik di macOS: alasan"
Cohesion: 0.18
Nodes (11): 0004. Konfigurasi publik di macOS: alasan, Bukti percobaan, Context, Cross check, Keputusan yang diambil, Opsi 1: Berkas xcconfig hasil generator sebagai dasar setelan target, Opsi 2: Pemeriksaan teks atas `project.pbxproj`, Opsi 3: Fase skrip yang membaca `public.json` saat build (+3 more)

### Community 53 - "0005. Tema bersama dan kontrol native di macOS: catatan keputusan"
Cohesion: 0.18
Nodes (11): 0005. Tema bersama dan kontrol native di macOS: catatan keputusan, Bukti percobaan, Context, Cross check, Keputusan yang lebih kecil, Opsi 1: Warna dinamis di kode, modul `DesignSystem` di paket Swift yang sudah ada, Opsi 2: Asset catalog hasil generator, Opsi 3: Memilih nilai di tiap tampilan lewat `colorScheme` (+3 more)

### Community 54 - "0001. Batas workspace dan pemilik: alasan"
Cohesion: 0.18
Nodes (11): 0001. Batas workspace dan pemilik: alasan, Bukti percobaan, Context, Cross check, Keputusan yang diambil, Opsi 1: Satu workspace, anggota eksplisit, aturan netral host, Opsi 2: Satu workspace dengan `default-members` hanya Core, Opsi 3: Aplikasi OS di luar workspace (+3 more)

### Community 55 - "0003. Tema bersama dan kontrol native: catatan keputusan"
Cohesion: 0.18
Nodes (11): 0003. Tema bersama dan kontrol native: catatan keputusan, Bukti percobaan, Context, Cross check, Keputusan yang lebih kecil, Opsi 1: Alat Rust di workspace, bentuk token sebagai tipe, skema dihasilkan, Opsi 2: Skema JSON tulisan tangan dan crate mesin JSON Schema, Opsi 3: Alat siap pakai dari luar dengan format standar (+3 more)

### Community 56 - "0005. Aset merek bersama: alasan"
Cohesion: 0.18
Nodes (11): 0005. Aset merek bersama: alasan, Bukti percobaan, Context, Cross check, Keputusan yang diambil, Opsi 1: Dokumen aturan saja, Opsi 2: Aturan, manifest, dan satu sumber penanda, diperiksa `desktop-codegen`, dengan ekspor biner oleh alat native tiap OS, Opsi 3: Generator ekspor biner bersama (+3 more)

### Community 57 - "Aset merek bersama"
Cohesion: 0.18
Nodes (11): Aset merek bersama, Format sumber, Mengganti atau menambah aset, Pemeriksaan, Pemilik, Penamaan, Resolusi, Struktur (+3 more)

### Community 58 - "Log koordinasi: `000 - Panduan scope`"
Cohesion: 0.20
Nodes (9): Blocker terbuka, Dicentang pada wave ini, Keputusan, Keputusan delegasi, Log koordinasi: `000 - Panduan scope`, Penutupan, Riwayat wave, Sub sesi wave `1` (+1 more)

### Community 59 - "Milestone 1: Workspace dan kerangka Core"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: Workspace dan kerangka Core, Milestone 2: Peta arsitektur mencatat tata letak nyata, UI / manual, UI / manual (+1 more)

### Community 60 - "Milestone 1: fungsi hello dan uji Done when"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: fungsi hello dan uji Done when, Milestone 2: masukan tepi, aturan kontrak, dan bukti, UI / manual, UI / manual (+1 more)

### Community 61 - "0003. Tema bersama dan kontrol native"
Cohesion: 0.20
Nodes (10): 0003. Tema bersama dan kontrol native, Build plan, Consequences, Decision, Feature design, Follow-up, Perubahan dokumen, Rationale (+2 more)

### Community 62 - "Milestone 1: Alat dan sumber token"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: Alat dan sumber token, Milestone 2: Dokumen mengikuti keadaan nyata, UI / manual, UI / manual (+1 more)

### Community 63 - "Milestone 1: Sumber nilai, pemeriksaan, dan skema hasil generator"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: Sumber nilai, pemeriksaan, dan skema hasil generator, Milestone 2: Dokumen rujukan mencatat sumber yang nyata, UI / manual, UI / manual (+1 more)

### Community 64 - "Milestone 1: Sumber, manifest, aturan, dan pemeriksaan aset merek"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: Sumber, manifest, aturan, dan pemeriksaan aset merek, Milestone 2: Dokumen rujukan menunjuk aturan aset, UI / manual, UI / manual (+1 more)

### Community 65 - "0006. Satu contoh lokalisasi"
Cohesion: 0.20
Nodes (10): 0006. Satu contoh lokalisasi, Build plan, Consequences, Decision, Feature design, Follow-up, Perubahan dokumen rujukan, Rationale (+2 more)

### Community 66 - "Options considered"
Cohesion: 0.20
Nodes (10): 0006. Satu contoh lokalisasi: catatan keputusan, Bukti percobaan, Context, Keputusan yang lebih kecil, Option 1: Satu katalog JSON netral untuk semua bahasa, resource native dibuat dari katalog itu, Option 2: Satu berkas JSON per bahasa, Option 3: Format native satu OS sebagai sumber bersama, Option 4: Pustaka lokalisasi Rust yang dipakai ketiga aplikasi saat berjalan (+2 more)

### Community 67 - "Milestone 1: Katalog dan sumber localization"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: Katalog dan sumber localization, Milestone 2: Dokumen rujukan mencatat keadaan nyata, UI / manual, UI / manual (+1 more)

### Community 68 - "Milestone 1: Sumber architecture dan aturan dependensi"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: Sumber architecture dan aturan dependensi, Milestone 2: Aturan kode dan uji lintas crate, UI / manual, UI / manual (+1 more)

### Community 69 - "macos_bridge_ownership.rs"
Cohesion: 0.33
Nodes (7): bridge_sources_hold_no_unsafe(), files_with_extension(), hand_written_swift_uses_no_pointers(), repo_root(), Path, PathBuf, Vec

### Community 70 - "Log koordinasi: `<nama-SCOPE>`"
Cohesion: 0.22
Nodes (8): Blocker terbuka, Dicentang pada wave ini, Keputusan, Keputusan delegasi, Log koordinasi: `<nama-SCOPE>`, Penutupan, Riwayat wave, Sub sesi wave `<n>`

### Community 71 - "required"
Cohesion: 0.28
Nodes (9): additionalProperties, required, type, additionalProperties, description, value, additionalProperties, dark (+1 more)

### Community 72 - "Verify: spec macos/0002 dan macos/0003 · 2026-11-15"
Cohesion: 0.22
Nodes (9): Catatan, Host dan versi, Keadaan `verify.md` yang dirujuk, Spec macos/0002 (scope 001), Spec macos/0003 (scope 003), Tafsiran yang diambil, Verdict, Verify: spec macos/0002 dan macos/0003 · 2026-11-15 (+1 more)

### Community 73 - "0001. Sapaan di jendela macOS"
Cohesion: 0.22
Nodes (9): 0001. Sapaan di jendela macOS, Build plan, Consequences, Decision, Feature design, Follow-up, Rationale, Requirements (+1 more)

### Community 74 - "0005. Tema bersama dan kontrol native di macOS"
Cohesion: 0.22
Nodes (9): 0005. Tema bersama dan kontrol native di macOS, Build plan, Consequences, Decision, Feature design, Follow-up, Rationale, Requirements (+1 more)

### Community 75 - "0002. Kontrak sapaan bersama"
Cohesion: 0.22
Nodes (9): 0002. Kontrak sapaan bersama, Build plan, Consequences, Decision, Feature design, Follow-up, Rationale, Requirements (+1 more)

### Community 76 - "Aturan pengembangan"
Cohesion: 0.25
Nodes (8): Agent skills, Aturan pengembangan, Build approach, Commands, Context files, Rules, Specs, Stack

### Community 77 - "Peran: architect"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: architect, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 78 - "Peran: checker"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: checker, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 79 - "Peran: debugger"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: debugger, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 80 - "Peran: developer"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: developer, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 81 - "Peran: tester"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: tester, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 82 - "Options considered"
Cohesion: 0.25
Nodes (8): 0002. Kontrak sapaan bersama: catatan keputusan, Context, Keputusan yang lebih kecil, Option 1: Fungsi total, nama dipakai apa adanya, Option 2: Validasi di Core dengan kesalahan bertipe, Option 3: Rapikan diam diam, tanda tangan tetap, Options considered, Rationale

### Community 83 - "Konfigurasi publik"
Cohesion: 0.25
Nodes (7): Batas pengaturan khusus OS, Cara aplikasi memakai nilai, Cara mengubah, Konfigurasi publik, Riwayat skema, Skema versi 1, Yang tidak boleh ada di folder ini

### Community 84 - "typography.json"
Cohesion: 0.29
Nodes (6): description, weight, roles, body, $schema, schemaVersion

### Community 85 - "Arsitektur fondasi desktop"
Cohesion: 0.29
Nodes (7): Arsitektur fondasi desktop, Aturan arah dependensi, Aturan pengembangan yang akan dicatat, Batas fase fondasi, Pemilik dan arah dependensi, Tata letak yang dituju, Yang sudah ada

### Community 86 - "brand.json"
Cohesion: 0.29
Nodes (6): placeholder, variants, assets, app-icon, $schema, schemaVersion

### Community 87 - "Langkah manual: `<nama-SCOPE>`"
Cohesion: 0.33
Nodes (5): `F<fitur>-<n>` · `<judul>`, Fitur `<nomor>`: `<nama>` (spec `<NNNN>`), Langkah manual: `<nama-SCOPE>`, Nilai dari user, Ringkasan

### Community 88 - "public.json"
Cohesion: 0.33
Nodes (5): build, app_identifier, product_name, $schema, schemaVersion

### Community 90 - "Resource bersama"
Cohesion: 0.50
Nodes (4): Aset merek, Konfigurasi, Lokalisasi, Resource bersama

### Community 92 - "dark"
Cohesion: 0.67
Nodes (3): pattern, type, dark

### Community 93 - "light"
Cohesion: 0.67
Nodes (3): pattern, type, light

## Knowledge Gaps
- **671 isolated node(s):** `Pemakaian`, `Konteks saat ini`, `1. Tentukan SCOPE`, `2. Analisis dan rencana wave`, `3. Mulai sub sesi` (+666 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **8 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Problem` connect `Problem` to `Findings`, `tokens_rules.rs`, `svg.rs`, `localization/mod.rs`, `assets_rules.rs`, `assets_contract.rs`, `assets/mod.rs`?**
  _High betweenness centrality (0.047) - this node is a cross-community bridge._
- **Why does `generate()` connect `tokens_rules.rs` to `Problem`, `assets_rules.rs`, `tokens_contract.rs`, `localization_rules.rs`, `config_rules.rs`?**
  _High betweenness centrality (0.018) - this node is a cross-community bridge._
- **Why does `0005. Aset merek bersama` connect `Rancangan aset merek` to `quality/README.md`?**
  _High betweenness centrality (0.012) - this node is a cross-community bridge._
- **Are the 13 inferred relationships involving `generate()` (e.g. with `main()` and `broken_source_writes_nothing()`) actually correct?**
  _`generate()` has 13 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Pemakaian`, `Konteks saat ini`, `1. Tentukan SCOPE` to the rest of the system?**
  _671 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `desktop_macos_ffi.swift` be split into smaller, more focused modules?**
  _Cohesion score 0.050883898709985664 - nodes in this community are weakly interconnected._
- **Should `Problem` be split into smaller, more focused modules?**
  _Cohesion score 0.07876712328767123 - nodes in this community are weakly interconnected._
Pembaruan hook menggunakan indeks struktural lokal. Hash semantik kosong menandai dokumen yang menunggu ekstraksi semantik.
