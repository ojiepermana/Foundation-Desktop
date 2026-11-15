# Graph Report - desktop  (2026-11-15)

## Corpus Check
- 174 files · ~268,807 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 2214 nodes · 3649 edges · 136 communities (125 shown, 11 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 68 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- quality/README.md
- desktop_macos_ffi.swift
- Findings
- generate
- assets/mod.rs
- localization/mod.rs
- catalog.schema.json
- config_macos_rules.rs
- spacing.schema.json
- architecture_rules.rs
- assets_rules.rs
- config_contract.rs
- brand.schema.json
- typography.schema.json
- public.schema.json
- config_rules.rs
- tokens_rules.rs
- tokens/mod.rs
- localization_rules.rs
- Problem
- text.primary
- Langkah manual: `000 - Panduan scope`
- Rancangan pemeriksaan
- hello
- Rancangan konfigurasi
- Rancangan
- assets_contract.rs
- Rancangan
- Feature design
- Rancangan aset merek
- workspace_layout.rs
- .string
- Fondasi Windows
- 0008. Aturan pengembangan dan pemeriksaan arsitektur di macOS
- Alur lead
- config/mod.rs
- 0001. Batas workspace dan pemilik
- Fondasi Linux
- 007 - Sapaan di jendela Linux
- architecture_contract.rs
- tokens_swift_rules.rs
- ThemeTests.swift
- ThemeColor
- GreetingViewModel
- spacing.json
- Fondasi macOS
- svg.rs
- BindingContractTests
- Milestone 1: Jembatan, binding, dan uji tanpa jendela
- 0002. Batas workspace dan pemilik di macOS
- 0003. Kontrak sapaan bersama di macOS
- Milestone 1: Sumber architecture dan aturan dependensi
- properties
- pre-commit script
- 0007. Aturan pengembangan dan pemeriksaan arsitektur: alasan
- catalog.json
- lib.sh
- properties
- 0001. Sapaan di jendela macOS: catatan keputusan
- 0001. Batas workspace dan pemilik: alasan
- 0003. Tema bersama dan kontrol native: catatan keputusan
- 0004. Konfigurasi publik: alasan
- 0005. Aset merek bersama: alasan
- 0008. Build dan penerimaan per target: alasan
- tokens_contract.rs
- contract_surface.rs
- colors.schema.json
- Penerimaan dan bukti fondasi
- 0004. Konfigurasi publik di macOS: alasan
- 0005. Tema bersama dan kontrol native di macOS: catatan keputusan
- 0006. Satu contoh lokalisasi di macOS: catatan keputusan
- 0007. Aset merek bersama di macOS: alasan
- 0008. Aturan pengembangan dan pemeriksaan arsitektur di macOS: alasan
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
- macos_bridge_ownership.rs
- Aturan pengembangan
- RecordingGreeter
- Log koordinasi: `<nama-SCOPE>`
- required
- Verify: spec macos/0002 dan macos/0003 · 2026-11-15
- 0001. Sapaan di jendela macOS
- 0005. Tema bersama dan kontrol native di macOS
- 0006. Satu contoh lokalisasi di macOS
- 0002. Kontrak sapaan bersama
- refresh
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
- DesktopMacApp
- TextRole
- Langkah manual: `<nama-SCOPE>`
- Milestone 1: Setelan Xcode dari sumber bersama
- Milestone 2: Jendela sapaan memakai token
- Milestone 1: Resource dari katalog sampai tombol
- Milestone 1: Ikon penanda dari sumber bersama sampai bundel Mac
- Milestone 1: Gerbang Mac terbukti
- Milestone 1: Gerbang bersama
- validate.sh
- public.json
- swift.rs
- macos-ffi/src/lib.rs
- 008 - Tema bersama dan kontrol native
- sample
- PackageDescription
- dark
- light
- test.sh
- brief.md
- brief-umum.md
- report.md
- 000 - Panduan scope.brief-umum.md
- desktop-core
- build-rust.sh
- desktop-codegen

## God Nodes (most connected - your core abstractions)
1. `Problem` - 43 edges
2. `fixture()` - 34 edges
3. `assert_rejected()` - 29 edges
4. `generate()` - 28 edges
5. `edit()` - 25 edges
6. `Findings` - 24 edges
7. `Fondasi Linux` - 19 edges
8. `check_colors()` - 19 edges
9. `Fondasi macOS` - 17 edges
10. `check_spacing()` - 16 edges

## Surprising Connections (you probably didn't know these)
- `Typography` --calls--> `TextRole`  [INFERRED]
  apps/macos/DesignSystem/Tokens.swift → apps/macos/DesignSystem/Theme.swift
- `Uji jendela di Mac` --semantically_similar_to--> `Uji sesi grafis Linux`  [INFERRED] [semantically similar]
  docs/platforms/macos/README.md → docs/platforms/linux/README.md
- `.body` --calls--> `GreetingView`  [INFERRED]
  apps/macos/App/DesktopMacApp.swift → apps/macos/Presentation/GreetingView.swift
- `.body` --calls--> `GreetingViewModel`  [INFERRED]
  apps/macos/App/DesktopMacApp.swift → apps/macos/Presentation/GreetingViewModel.swift
- `Colors` --calls--> `ThemeColor`  [INFERRED]
  apps/macos/DesignSystem/Tokens.swift → apps/macos/DesignSystem/Theme.swift

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Pemeriksaan runtime pada tiga host** — docs_quality_readme_bukti_runtime_per_host, docs_platforms_macos_readme_uji_jendela_di_mac, docs_platforms_windows_readme_uji_jendela_windows, docs_platforms_linux_readme_uji_sesi_grafis_linux [EXTRACTED 1.00]

## Communities (136 total, 11 thin omitted)

### Community 0 - "quality/README.md"
Cohesion: 0.06
Nodes (44): Aturan commit dan integrasi, Commands, Conventions, Gotchas, Key files, Overview, Related specs, Rust Core (`desktop-core`) (+36 more)

### Community 1 - "desktop_macos_ffi.swift"
Cohesion: 0.05
Nodes (65): Array, createReader(), createWriter(), Data, FfiConverter, FfiConverterByRefBytes, FfiConverterPrimitive, FfiConverterRustBuffer (+57 more)

### Community 2 - "Findings"
Cohesion: 0.06
Nodes (63): File, check(), check_bridge(), check_config(), check_core(), check_greeting(), check_imports(), check_lints() (+55 more)

### Community 3 - "generate"
Cohesion: 0.07
Nodes (52): ExitCode, Into, check(), generate(), generate_command(), Output, Plan, repo_root() (+44 more)

### Community 4 - "assets/mod.rs"
Cohesion: 0.07
Nodes (52): A, D, Error, PhantomData, icon_json(), outputs(), String, Vec (+44 more)

### Community 5 - "localization/mod.rs"
Cohesion: 0.10
Nodes (44): escaped(), file(), outputs(), String, Vec, Catalog, CatalogFile, check_entry() (+36 more)

### Community 6 - "catalog.schema.json"
Cohesion: 0.05
Nodes (45): entries, locales, sourceLocale, text, additionalProperties, additionalProperties, minLength, properties (+37 more)

### Community 7 - "config_macos_rules.rs"
Cohesion: 0.08
Nodes (42): is_plain_version(), product_name_problem(), Option, Result, Vec, shared_xcconfig(), accepted(), rejected() (+34 more)

### Community 8 - "spacing.schema.json"
Cohesion: 0.05
Nodes (42): additionalProperties, additionalProperties, properties, required, type, additionalProperties, propertyNames, type (+34 more)

### Community 9 - "architecture_rules.rs"
Cohesion: 0.13
Nodes (31): Drop, agents_references(), architecture_command_exit_codes(), Copy, copy_tree(), core_boundary_edges(), core_re_export_must_be_plain(), every_violating_line_gives_its_own_problem() (+23 more)

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

### Community 16 - "tokens_rules.rs"
Cohesion: 0.16
Nodes (29): a_file_that_does_not_fit_its_type_gives_one_problem(), broken_source_writes_nothing_even_when_outputs_are_missing(), colors_with_dark_text(), committed_plan(), contrast_at_the_threshold_passes(), contrast_just_below_the_threshold_fails_with_a_cut_down_ratio(), edit(), every_output_carries_the_marker_in_its_first_three_lines() (+21 more)

### Community 17 - "tokens/mod.rs"
Cohesion: 0.14
Nodes (28): BaseColor, BaseSpace, Color, ColorsFile, contrast(), is_valid_description(), is_valid_name(), luminance() (+20 more)

### Community 18 - "localization_rules.rs"
Cohesion: 0.14
Nodes (29): at(), broken_or_unreadable_catalog_writes_nothing(), catalog(), committed_plan(), edited(), entry(), entry_problems_follow_the_check_order(), every_output_follows_the_output_rules() (+21 more)

### Community 19 - "Problem"
Cohesion: 0.20
Nodes (20): BTreeSet, Display, Item, Iterator, Problem, check_colors(), check_spacing(), check_typography() (+12 more)

### Community 20 - "text.primary"
Cohesion: 0.07
Nodes (26): dark, description, light, dark, description, light, base, brand.primary (+18 more)

### Community 21 - "Langkah manual: `000 - Panduan scope`"
Cohesion: 0.08
Nodes (24): `F001-1` · Bangun dan uji Core di host Windows dan host Linux, `F002-1` · Bangun, uji, dan jalankan pemeriksaan arsitektur di host Windows dan host Linux, `F004-1` · Alur sapaan di jendela, `F004-2` · Penggunaan nyata, `F008-1` · Bangun, uji, dan periksa token di host Windows dan host Linux, `F008-2` · Tampilan terang dan gelap dari token dan kontrol native, `F009-1` · Judul jendela dari konfigurasi publik dan nama menu aplikasi, `F010-1` · Teks tombol mengikuti bahasa di jendela nyata (+16 more)

### Community 22 - "Rancangan pemeriksaan"
Cohesion: 0.08
Nodes (25): 0007. Aturan pengembangan dan pemeriksaan arsitektur, Aturan dependensi yang dibaca mesin, Aturan yang diperiksa manusia, Aturan yang diperiksa mesin, Bentuk alat, Build plan, Cara menambah atau mengubah aturan, Cara tiap aturan membaca repo (+17 more)

### Community 23 - "hello"
Cohesion: 0.12
Nodes (11): hello(), String, long_name_is_not_cut(), assert_wraps_name(), braces_in_name_stay_plain_text(), every_vector_is_prefix_then_name_unchanged_then_bang(), long_multi_byte_name_is_not_cut(), String (+3 more)

### Community 24 - "Rancangan konfigurasi"
Cohesion: 0.09
Nodes (23): 0004. Konfigurasi publik, Aturan versi skema, Batas pengaturan khusus OS, Bentuk alat, Build plan, Cara aplikasi memakai nilai, Consequences, Decision (+15 more)

### Community 25 - "Rancangan"
Cohesion: 0.09
Nodes (23): 0008. Build dan penerimaan per target, Bentuk keluaran, Build plan, Consequences, Daftar penerimaan per target, Decision, File yang boleh diubah tiap milestone, Follow-up (+15 more)

### Community 26 - "assets_contract.rs"
Cohesion: 0.19
Nodes (20): accepts_valid_sources(), Change, changed_icon(), fixture(), folder_case(), make_light_dark(), messages(), read() (+12 more)

### Community 27 - "Rancangan"
Cohesion: 0.10
Nodes (21): 0004. Konfigurasi publik di macOS, Alur nilai, Batas setelan di macOS, Build plan, Consequences, Decision, Follow-up, Invarian (+13 more)

### Community 28 - "Feature design"
Cohesion: 0.10
Nodes (21): 0007. Aset merek bersama di macOS, Build plan, Consequences, Decision, Emitter macOS, Feature design, Follow-up, Invarian (+13 more)

### Community 29 - "Rancangan aset merek"
Cohesion: 0.10
Nodes (21): 0005. Aset merek bersama, Aturan sumber, Bentuk alat, Build plan, Consequences, Decision, Follow-up, Invarian (+13 more)

### Community 30 - "workspace_layout.rs"
Cohesion: 0.16
Nodes (17): architecture_rule_table_matches_the_spec_row_by_row(), every_member_inherits_shared_fields_and_uses_desktop_prefix(), first_table_after(), members(), members_are_listed_one_by_one_without_globs(), minor_of(), one_cargo_lock_for_the_whole_workspace(), repo_root() (+9 more)

### Community 31 - ".string"
Cohesion: 0.16
Nodes (10): LocalizedText, .bundle, TextKey, greetingSubmit, LocalizedTextTests, Bundle, CaseIterable, Foundation (+2 more)

### Community 32 - "Fondasi Windows"
Cohesion: 0.17
Nodes (20): Alur dan kepemilikan, Bukti build, runtime dan distribusi, Fondasi Windows, Matriks kompatibilitas Windows, Percobaan WinUI 3 di host Windows, Pesan UI bertipe, Target Windows x64 dan kelayakan ARM64, Tema, resource dan perilaku native (+12 more)

### Community 33 - "0008. Aturan pengembangan dan pemeriksaan arsitektur di macOS"
Cohesion: 0.10
Nodes (20): 0008. Aturan pengembangan dan pemeriksaan arsitektur di macOS, Aturan teks macOS, Build plan, Consequences, Decision, File yang boleh diubah tiap milestone, Follow-up, Gerbang Mac (+12 more)

### Community 34 - "Alur lead"
Cohesion: 0.11
Nodes (18): 1. Tentukan SCOPE, 2. Analisis dan rencana wave, 3. Mulai sub sesi, 4. Komunikasi, 5. Laporan, 6. Gerbang centang dan merge, 7. Log, 8. Akhir wave (+10 more)

### Community 35 - "config/mod.rs"
Cohesion: 0.23
Nodes (18): Deserialize, Build, check_app_identifier(), check_entries(), check_product_name(), check_public(), check_readme_version(), plan() (+10 more)

### Community 36 - "0001. Batas workspace dan pemilik"
Cohesion: 0.11
Nodes (19): 0001. Batas workspace dan pemilik, Aturan arah dependensi, Aturan untuk manifest root, Build plan, Consequences, Decision, Follow-up, Invarian (+11 more)

### Community 37 - "Fondasi Linux"
Cohesion: 0.20
Nodes (17): Bukti yang diperlukan di Linux, Compositor Wayland, Evaluasi Flatpak di masa mendatang, Fondasi Linux, GTK 4, gtk-rs, Jendela native dan Rust Core, Kompatibilitas X11 (+9 more)

### Community 38 - "007 - Sapaan di jendela Linux"
Cohesion: 0.15
Nodes (17): C04 · UniFFI berhasil menghasilkan binding Swift untuk macOS, C05 · Aplikasi macOS berhasil memanggil fungsi Rust tersebut, C10 · Aplikasi Linux menggunakan GTK 4 dan libadwaita, C11 · Aplikasi Linux memanggil Rust Core langsung melalui Cargo, C12 · Aplikasi Linux berjalan pada backend Wayland native, C13 · Kompatibilitas X11 diperiksa dan hasil atau keterbatasannya dicatat, C14 · Antarmuka pada ketiga OS menampilkan hasil aktual dari fungsi Rust yang sama, C23 · Kepemilikan pada batas FFI macOS terdokumentasi dan divalidasi bila relevan (+9 more)

### Community 39 - "architecture_contract.rs"
Cohesion: 0.23
Nodes (14): accepts_allowed_shapes(), apply(), copy_repo(), copy_tree(), Edit, lock_block(), Rejected, rejects_broken_repos() (+6 more)

### Community 40 - "tokens_swift_rules.rs"
Cohesion: 0.17
Nodes (11): broken_source_writes_no_tokens_swift(), constants(), expected_style(), generate_and_check_round_trip_for_tokens_swift(), plan_carries_tokens_swift_after_the_schemas(), read(), Option, PathBuf (+3 more)

### Community 41 - "ThemeTests.swift"
Cohesion: 0.17
Nodes (8): AppKit, resolved(), NSColor, UInt8, ThemeTests, DesignSystem, NSAppearance, SwiftUI

### Community 42 - "ThemeColor"
Cohesion: 0.17
Nodes (12): Bool, NSColor, UInt8, ThemeColor, .color, .nsColor, Colors, Spacing (+4 more)

### Community 43 - "GreetingViewModel"
Cohesion: 0.24
Nodes (7): Greeter, GreetingView, .body, GreetingViewModel, String, GreetingViewModelTests, View

### Community 44 - "spacing.json"
Cohesion: 0.13
Nodes (14): base, space.large, space.medium, description, ref, description, ref, $schema (+6 more)

### Community 45 - "Fondasi macOS"
Cohesion: 0.17
Nodes (15): AppKit, Apple Silicon ARM64 dan Intel bersyarat, Batas Swift dan Rust, Binding Swift yang dihasilkan, Bukti penerimaan pada Mac, Distribusi, Fondasi macOS, Jendela dan interaksi (+7 more)

### Community 46 - "svg.rs"
Cohesion: 0.31
Nodes (13): FnMut, check_elements(), check_entities(), check_events(), check_references(), check_root(), check_svg(), is_valid_view_box() (+5 more)

### Community 47 - "BindingContractTests"
Cohesion: 0.22
Nodes (4): CoreGreeter, String, BindingContractTests, DesktopMacosFFI

### Community 48 - "Milestone 1: Jembatan, binding, dan uji tanpa jendela"
Cohesion: 0.14
Nodes (13): Acceptance-criteria coverage, Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Commands, Milestone 1: Jembatan, binding, dan uji tanpa jendela, Milestone 2: Jendela SwiftUI dan proyek Xcode (+5 more)

### Community 49 - "0002. Batas workspace dan pemilik di macOS"
Cohesion: 0.14
Nodes (14): 0002. Batas workspace dan pemilik di macOS, Build plan, Consequences, Context, Decision, Feature design, Follow-up, Opsi 1: Rujuk bukti yang ada, tambah satu langkah untuk peta arsitektur (+6 more)

### Community 50 - "0003. Kontrak sapaan bersama di macOS"
Cohesion: 0.14
Nodes (14): 0003. Kontrak sapaan bersama di macOS, Build plan, Consequences, Context, Decision, Feature design, Follow-up, Opsi 1: Rujuk bukti yang ada, tanpa langkah build (+6 more)

### Community 51 - "Milestone 1: Sumber architecture dan aturan dependensi"
Cohesion: 0.14
Nodes (13): Acceptance-criteria coverage, Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Commands, Milestone 1: Sumber architecture dan aturan dependensi, Milestone 2: Aturan kode dan uji lintas crate (+5 more)

### Community 52 - "properties"
Cohesion: 0.17
Nodes (13): propertyNames, type, properties, base, $schema, schemaVersion, semantic, pattern (+5 more)

### Community 53 - "pre-commit script"
Cohesion: 0.21
Nodes (10): Validasi keluaran generator, post-commit script, Rekonsiliasi indeks sesudah git commit --only, pre-commit script, Empat artefak Graphify dalam satu commit, Isolasi riwayat kueri lokal, Preservasi edit unstaged pada artefak, Validasi artefak sebelum staging (+2 more)

### Community 54 - "0007. Aturan pengembangan dan pemeriksaan arsitektur: alasan"
Cohesion: 0.15
Nodes (13): 0007. Aturan pengembangan dan pemeriksaan arsitektur: alasan, Bukti percobaan, Context, Cross check, Jawaban atas masukan `/audit`, Keputusan yang diambil, Opsi 1: Sumber `architecture` di `desktop-codegen`, Opsi 2: Paket pemeriksa sendiri di `tooling/validation/` (+5 more)

### Community 55 - "catalog.json"
Cohesion: 0.15
Nodes (12): en, id, entries, greeting.submit, description, text, $schema, locales (+4 more)

### Community 56 - "lib.sh"
Cohesion: 0.21
Nodes (7): finish(), header(), lib.sh script, step(), summary_line(), summary_row(), toolchain_step()

### Community 57 - "properties"
Cohesion: 0.17
Nodes (12): properties, pattern, type, pattern, type, type, description, on (+4 more)

### Community 58 - "0001. Sapaan di jendela macOS: catatan keputusan"
Cohesion: 0.17
Nodes (12): 0001. Sapaan di jendela macOS: catatan keputusan, Bukti percobaan, Context, Cross check, Keputusan yang lebih kecil, Opsi 1: Binding terlacak sebagai paket Swift lokal, modul Swift yang diuji `swift test`, target Xcode tipis, Opsi 2: Semua kode Swift di target aplikasi Xcode, binding lewat bridging header, Opsi 3: XCFramework sebagai target biner (+4 more)

### Community 59 - "0001. Batas workspace dan pemilik: alasan"
Cohesion: 0.17
Nodes (12): 0001. Batas workspace dan pemilik: alasan, Bukti percobaan, Context, Cross check, Keputusan yang diambil, Opsi 1: Satu workspace, anggota eksplisit, aturan netral host, Opsi 2: Satu workspace dengan `default-members` hanya Core, Opsi 3: Aplikasi OS di luar workspace (+4 more)

### Community 60 - "0003. Tema bersama dan kontrol native: catatan keputusan"
Cohesion: 0.17
Nodes (12): 0003. Tema bersama dan kontrol native: catatan keputusan, Bukti percobaan, Context, Cross check, Keputusan yang lebih kecil, Opsi 1: Alat Rust di workspace, bentuk token sebagai tipe, skema dihasilkan, Opsi 2: Skema JSON tulisan tangan dan crate mesin JSON Schema, Opsi 3: Alat siap pakai dari luar dengan format standar (+4 more)

### Community 61 - "0004. Konfigurasi publik: alasan"
Cohesion: 0.17
Nodes (12): 0004. Konfigurasi publik: alasan, Bukti yang diperiksa, Context, Cross check, Keputusan yang diambil, Opsi 1: Satu file nilai JSON dengan skema tertutup di `shared/config/`, Opsi 2: File nilai TOML dengan aturan di kode validasi, Opsi 3: Tanpa file bersama, nilai tetap di proyek native (+4 more)

### Community 62 - "0005. Aset merek bersama: alasan"
Cohesion: 0.17
Nodes (12): 0005. Aset merek bersama: alasan, Bukti percobaan, Context, Cross check, Keputusan yang diambil, Opsi 1: Dokumen aturan saja, Opsi 2: Aturan, manifest, dan satu sumber penanda, diperiksa `desktop-codegen`, dengan ekspor biner oleh alat native tiap OS, Opsi 3: Generator ekspor biner bersama (+4 more)

### Community 63 - "0008. Build dan penerimaan per target: alasan"
Cohesion: 0.17
Nodes (11): 0008. Build dan penerimaan per target: alasan, Bukti percobaan, Context, Cross check, Keputusan yang diambil, Opsi 1: Skrip bash tipis, workflow yang hanya memanggil skrip, laporan yang hidup, Opsi 2: Paket Rust kecil sebagai pengganti skrip (`cargo run -p desktop-ci`), Opsi 3: Perintah langsung di workflow, tanpa skrip (+3 more)

### Community 64 - "tokens_contract.rs"
Cohesion: 0.27
Nodes (9): Case, check_reports_stale_and_missing_outputs_and_generate_repairs_them(), copy_sources(), File, read(), rejects_broken_sources(), resolves_light_and_dark(), Path (+1 more)

### Community 65 - "contract_surface.rs"
Cohesion: 0.22
Nodes (5): manifest_declares_no_dependencies_of_any_kind(), manifest_forbids_unsafe_code_and_warns_on_missing_docs(), manifest_tables(), String, Vec

### Community 66 - "colors.schema.json"
Cohesion: 0.18
Nodes (10): additionalProperties, $comment, base, $schema, schemaVersion, semantic, required, $schema (+2 more)

### Community 67 - "Penerimaan dan bukti fondasi"
Cohesion: 0.22
Nodes (11): Uji jendela Windows, Bukti build per arsitektur, Bukti native per host, Bukti runtime per host, GitHub Actions, Kriteria penerimaan, Laporan ketika fondasi selesai, Pemeriksaan dependensi Core (+3 more)

### Community 68 - "0004. Konfigurasi publik di macOS: alasan"
Cohesion: 0.18
Nodes (11): 0004. Konfigurasi publik di macOS: alasan, Bukti percobaan, Context, Cross check, Keputusan yang diambil, Opsi 1: Berkas xcconfig hasil generator sebagai dasar setelan target, Opsi 2: Pemeriksaan teks atas `project.pbxproj`, Opsi 3: Fase skrip yang membaca `public.json` saat build (+3 more)

### Community 69 - "0005. Tema bersama dan kontrol native di macOS: catatan keputusan"
Cohesion: 0.18
Nodes (11): 0005. Tema bersama dan kontrol native di macOS: catatan keputusan, Bukti percobaan, Context, Cross check, Keputusan yang lebih kecil, Opsi 1: Warna dinamis di kode, modul `DesignSystem` di paket Swift yang sudah ada, Opsi 2: Asset catalog hasil generator, Opsi 3: Memilih nilai di tiap tampilan lewat `colorScheme` (+3 more)

### Community 70 - "0006. Satu contoh lokalisasi di macOS: catatan keputusan"
Cohesion: 0.18
Nodes (11): 0006. Satu contoh lokalisasi di macOS: catatan keputusan, Bukti percobaan, Context, Cross check, Keputusan yang lebih kecil, Opsi 1: Berkas `.strings` per bahasa sebagai resource modul `Presentation`, Opsi 2: Satu String Catalog `.xcstrings` sebagai resource modul `Presentation`, Opsi 3: Resource `.lproj` di target aplikasi Xcode (+3 more)

### Community 71 - "0007. Aset merek bersama di macOS: alasan"
Cohesion: 0.18
Nodes (11): 0007. Aset merek bersama di macOS: alasan, Bukti percobaan, Context, Cross check, Keputusan yang diambil, Opsi 1: Dokumen Icon Composer `.icon` yang ditulis `desktop-codegen`, dikompilasi `actool` saat build, Opsi 2: Katalog aset `AppIcon.appiconset` berisi PNG yang dirender alat native dan dilacak git, Opsi 3: Generator raster di `desktop-codegen` (+3 more)

### Community 72 - "0008. Aturan pengembangan dan pemeriksaan arsitektur di macOS: alasan"
Cohesion: 0.18
Nodes (11): 0008. Aturan pengembangan dan pemeriksaan arsitektur di macOS: alasan, Bukti percobaan, Context, Cross check, Keputusan yang diambil, Opsi 1: Spec macOS sendiri, gerbang berupa perintah, aturan teks di sumber `architecture`, Opsi 2: Satu AC tambahan di spec `macos/0001` dan aturan baru di spec `_root/0007`, Opsi 3: Uji Rust yang hanya berjalan di Mac (+3 more)

### Community 73 - "Aset merek bersama"
Cohesion: 0.18
Nodes (11): Aset merek bersama, Format sumber, Mengganti atau menambah aset, Pemeriksaan, Pemilik, Penamaan, Resolusi, Struktur (+3 more)

### Community 74 - "Log koordinasi: `000 - Panduan scope`"
Cohesion: 0.20
Nodes (9): Blocker terbuka, Dicentang pada wave ini, Keputusan, Keputusan delegasi, Log koordinasi: `000 - Panduan scope`, Penutupan, Riwayat wave, Sub sesi wave `1` (+1 more)

### Community 75 - "Milestone 1: Workspace dan kerangka Core"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: Workspace dan kerangka Core, Milestone 2: Peta arsitektur mencatat tata letak nyata, UI / manual, UI / manual (+1 more)

### Community 76 - "Milestone 1: fungsi hello dan uji Done when"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: fungsi hello dan uji Done when, Milestone 2: masukan tepi, aturan kontrak, dan bukti, UI / manual, UI / manual (+1 more)

### Community 77 - "0003. Tema bersama dan kontrol native"
Cohesion: 0.20
Nodes (10): 0003. Tema bersama dan kontrol native, Build plan, Consequences, Decision, Feature design, Follow-up, Perubahan dokumen, Rationale (+2 more)

### Community 78 - "Milestone 1: Alat dan sumber token"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: Alat dan sumber token, Milestone 2: Dokumen mengikuti keadaan nyata, UI / manual, UI / manual (+1 more)

### Community 79 - "Milestone 1: Sumber nilai, pemeriksaan, dan skema hasil generator"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: Sumber nilai, pemeriksaan, dan skema hasil generator, Milestone 2: Dokumen rujukan mencatat sumber yang nyata, UI / manual, UI / manual (+1 more)

### Community 80 - "Milestone 1: Sumber, manifest, aturan, dan pemeriksaan aset merek"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: Sumber, manifest, aturan, dan pemeriksaan aset merek, Milestone 2: Dokumen rujukan menunjuk aturan aset, UI / manual, UI / manual (+1 more)

### Community 81 - "0006. Satu contoh lokalisasi"
Cohesion: 0.20
Nodes (10): 0006. Satu contoh lokalisasi, Build plan, Consequences, Decision, Feature design, Follow-up, Perubahan dokumen rujukan, Rationale (+2 more)

### Community 82 - "Options considered"
Cohesion: 0.20
Nodes (10): 0006. Satu contoh lokalisasi: catatan keputusan, Bukti percobaan, Context, Keputusan yang lebih kecil, Option 1: Satu katalog JSON netral untuk semua bahasa, resource native dibuat dari katalog itu, Option 2: Satu berkas JSON per bahasa, Option 3: Format native satu OS sebagai sumber bersama, Option 4: Pustaka lokalisasi Rust yang dipakai ketiga aplikasi saat berjalan (+2 more)

### Community 83 - "Milestone 1: Katalog dan sumber localization"
Cohesion: 0.20
Nodes (9): Acceptance-criteria coverage, Acceptance-criteria coverage, Commands, Commands, Milestone 1: Katalog dan sumber localization, Milestone 2: Dokumen rujukan mencatat keadaan nyata, UI / manual, UI / manual (+1 more)

### Community 84 - "macos_bridge_ownership.rs"
Cohesion: 0.33
Nodes (7): bridge_sources_hold_no_unsafe(), files_with_extension(), hand_written_swift_uses_no_pointers(), repo_root(), Path, PathBuf, Vec

### Community 85 - "Aturan pengembangan"
Cohesion: 0.22
Nodes (9): Agent skills, Aturan pengembangan, Build approach, Commands, Context files, Rules, Specs, Stack (+1 more)

### Community 86 - "RecordingGreeter"
Cohesion: 0.28
Nodes (5): RecordingGreeter, String, Interop, Observation, Testing

### Community 87 - "Log koordinasi: `<nama-SCOPE>`"
Cohesion: 0.22
Nodes (8): Blocker terbuka, Dicentang pada wave ini, Keputusan, Keputusan delegasi, Log koordinasi: `<nama-SCOPE>`, Penutupan, Riwayat wave, Sub sesi wave `<n>`

### Community 88 - "required"
Cohesion: 0.28
Nodes (9): additionalProperties, required, type, additionalProperties, description, value, additionalProperties, dark (+1 more)

### Community 89 - "Verify: spec macos/0002 dan macos/0003 · 2026-11-15"
Cohesion: 0.22
Nodes (9): Catatan, Host dan versi, Keadaan `verify.md` yang dirujuk, Spec macos/0002 (scope 001), Spec macos/0003 (scope 003), Tafsiran yang diambil, Verdict, Verify: spec macos/0002 dan macos/0003 · 2026-11-15 (+1 more)

### Community 90 - "0001. Sapaan di jendela macOS"
Cohesion: 0.22
Nodes (9): 0001. Sapaan di jendela macOS, Build plan, Consequences, Decision, Feature design, Follow-up, Rationale, Requirements (+1 more)

### Community 91 - "0005. Tema bersama dan kontrol native di macOS"
Cohesion: 0.22
Nodes (9): 0005. Tema bersama dan kontrol native di macOS, Build plan, Consequences, Decision, Feature design, Follow-up, Rationale, Requirements (+1 more)

### Community 92 - "0006. Satu contoh lokalisasi di macOS"
Cohesion: 0.22
Nodes (9): 0006. Satu contoh lokalisasi di macOS, Build plan, Consequences, Decision, Feature design, Follow-up, Rationale, Requirements (+1 more)

### Community 93 - "0002. Kontrak sapaan bersama"
Cohesion: 0.22
Nodes (9): 0002. Kontrak sapaan bersama, Build plan, Consequences, Decision, Feature design, Follow-up, Rationale, Requirements (+1 more)

### Community 94 - "refresh"
Cohesion: 0.33
Nodes (7): Ekstraksi struktur lokal, Pemeriksaan kesegaran graf, Pemeriksaan ujung relasi, Pencatatan hash manifest, Penggunaan ulang bukti semantik, Refresh the shared Graphify artifacts inside a disposable Git index snapshot., refresh()

### Community 95 - "Peran: architect"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: architect, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 96 - "Peran: checker"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: checker, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 97 - "Peran: debugger"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: debugger, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 98 - "Peran: developer"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: developer, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 99 - "Peran: tester"
Cohesion: 0.25
Nodes (7): Command yang dijalankan, Ownership, Peran: tester, Pesan, Selesai bila, Tujuan, Verifikasi sebelum melapor

### Community 100 - "Options considered"
Cohesion: 0.25
Nodes (8): 0002. Kontrak sapaan bersama: catatan keputusan, Context, Keputusan yang lebih kecil, Option 1: Fungsi total, nama dipakai apa adanya, Option 2: Validasi di Core dengan kesalahan bertipe, Option 3: Rapikan diam diam, tanda tangan tetap, Options considered, Rationale

### Community 101 - "Konfigurasi publik"
Cohesion: 0.25
Nodes (7): Batas pengaturan khusus OS, Cara aplikasi memakai nilai, Cara mengubah, Konfigurasi publik, Riwayat skema, Skema versi 1, Yang tidak boleh ada di folder ini

### Community 102 - "typography.json"
Cohesion: 0.29
Nodes (6): description, weight, roles, body, $schema, schemaVersion

### Community 103 - "Arsitektur fondasi desktop"
Cohesion: 0.29
Nodes (7): Arsitektur fondasi desktop, Aturan arah dependensi, Aturan pengembangan yang akan dicatat, Batas fase fondasi, Pemilik dan arah dependensi, Tata letak yang dituju, Yang sudah ada

### Community 104 - "brand.json"
Cohesion: 0.29
Nodes (6): placeholder, variants, assets, app-icon, $schema, schemaVersion

### Community 105 - "DesktopMacApp"
Cohesion: 0.33
Nodes (5): App, DesktopMacApp, .body, Presentation, Scene

### Community 106 - "TextRole"
Cohesion: 0.40
Nodes (4): TextRole, .font, Equatable, Font

### Community 107 - "Langkah manual: `<nama-SCOPE>`"
Cohesion: 0.33
Nodes (5): `F<fitur>-<n>` · `<judul>`, Fitur `<nomor>`: `<nama>` (spec `<NNNN>`), Langkah manual: `<nama-SCOPE>`, Nilai dari user, Ringkasan

### Community 108 - "Milestone 1: Setelan Xcode dari sumber bersama"
Cohesion: 0.33
Nodes (5): Acceptance-criteria coverage, Commands, Milestone 1: Setelan Xcode dari sumber bersama, UI / manual, Verify: Konfigurasi publik di macOS · spec macos/0004 · updated 2026-11-15

### Community 109 - "Milestone 2: Jendela sapaan memakai token"
Cohesion: 0.33
Nodes (5): Acceptance-criteria coverage, Commands, Milestone 2: Jendela sapaan memakai token, UI / manual, Verify: Tema bersama dan kontrol native di macOS · spec macos/0005 · updated 2026-11-15

### Community 110 - "Milestone 1: Resource dari katalog sampai tombol"
Cohesion: 0.33
Nodes (5): Acceptance-criteria coverage, Commands, Milestone 1: Resource dari katalog sampai tombol, UI / manual, Verify: Satu contoh lokalisasi di macOS · spec macos/0006 · updated 2026-11-15

### Community 111 - "Milestone 1: Ikon penanda dari sumber bersama sampai bundel Mac"
Cohesion: 0.33
Nodes (5): Acceptance-criteria coverage, Commands, Milestone 1: Ikon penanda dari sumber bersama sampai bundel Mac, UI / manual, Verify: Aset merek bersama di macOS · spec macos/0007 · updated 2026-11-15

### Community 112 - "Milestone 1: Gerbang Mac terbukti"
Cohesion: 0.33
Nodes (5): Acceptance-criteria coverage, Commands, Milestone 1: Gerbang Mac terbukti, UI / manual, Verify: Aturan pengembangan dan pemeriksaan arsitektur macOS · spec macos/0008 · updated 2026-11-15

### Community 113 - "Milestone 1: Gerbang bersama"
Cohesion: 0.33
Nodes (5): Acceptance-criteria coverage, Commands, Milestone 1: Gerbang bersama, UI / manual, Verify: Build dan penerimaan per target · spec _root/0008 · updated 2026-11-15

### Community 115 - "public.json"
Cohesion: 0.33
Nodes (5): build, app_identifier, product_name, $schema, schemaVersion

### Community 116 - "swift.rs"
Cohesion: 0.53
Nodes (5): hex(), identifier(), output(), String, separate()

### Community 118 - "008 - Tema bersama dan kontrol native"
Cohesion: 0.50
Nodes (4): C15 · Token desain memiliki satu sumber acuan, C16 · Ketiga aplikasi memakai token hasil generasi atau pemetaan yang dapat diulang, C17 · Antarmuka memperlihatkan tampilan terang dan gelap, 008 - Tema bersama dan kontrol native

### Community 119 - "sample"
Cohesion: 0.83
Nodes (3): sample(), swift_emitter_maps_every_token_kind(), swift_output_follows_the_output_rules()

### Community 121 - "dark"
Cohesion: 0.67
Nodes (3): pattern, type, dark

### Community 122 - "light"
Cohesion: 0.67
Nodes (3): pattern, type, light

## Knowledge Gaps
- **814 isolated node(s):** `Pemakaian`, `Konteks saat ini`, `1. Tentukan SCOPE`, `2. Analisis dan rencana wave`, `3. Mulai sub sesi` (+809 more)
  These have ≤1 connection - possible missing edges or undocumented components.
- **11 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Problem` connect `Problem` to `Findings`, `config/mod.rs`, `assets/mod.rs`, `generate`, `localization/mod.rs`, `config_macos_rules.rs`, `architecture_rules.rs`, `assets_rules.rs`, `svg.rs`, `tokens_rules.rs`, `assets_contract.rs`?**
  _High betweenness centrality (0.037) - this node is a cross-community bridge._
- **Why does `0007. Aturan pengembangan dan pemeriksaan arsitektur` connect `Rancangan pemeriksaan` to `quality/README.md`?**
  _High betweenness centrality (0.018) - this node is a cross-community bridge._
- **Why does `generate()` connect `generate` to `tokens_contract.rs`, `config_macos_rules.rs`, `tokens_swift_rules.rs`, `architecture_rules.rs`, `assets_rules.rs`, `config_rules.rs`, `tokens_rules.rs`, `localization_rules.rs`, `Problem`?**
  _High betweenness centrality (0.013) - this node is a cross-community bridge._
- **Are the 19 inferred relationships involving `generate()` (e.g. with `main()` and `plan_does_not_depend_on_the_root_and_generate_writes_nothing()`) actually correct?**
  _`generate()` has 19 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Pemakaian`, `Konteks saat ini`, `1. Tentukan SCOPE` to the rest of the system?**
  _814 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `quality/README.md` be split into smaller, more focused modules?**
  _Cohesion score 0.05643564356435644 - nodes in this community are weakly interconnected._
- **Should `desktop_macos_ffi.swift` be split into smaller, more focused modules?**
  _Cohesion score 0.051770451770451774 - nodes in this community are weakly interconnected._
Pembaruan hook menggunakan indeks struktural lokal. Hash semantik kosong menandai dokumen yang menunggu ekstraksi semantik.
