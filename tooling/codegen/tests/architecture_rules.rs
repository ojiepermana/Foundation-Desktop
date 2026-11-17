//! Rule tests for the `architecture` source of spec `_root/0007`, beyond the
//! three contract tests in `architecture_contract.rs`.
//!
//! They lock the order of problems (read errors first, then the rule number
//! as a number, then path, then line), which rules are skipped when
//! `Cargo.toml`, `Cargo.lock`, or a member manifest cannot be read, the
//! edges of rules `A-4` to `A-13`, how `A-12` and `A-13` read Markdown, that
//! the plan does not depend on the root path and `generate` writes nothing,
//! that `RULES` and the success line agree, and the command line. The rules
//! live in
//! `docs/specs/_root/0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md`.
//! Every case runs on its own copy of the repo under
//! `env!("CARGO_TARGET_TMPDIR")`; the repo itself is never edited. No test
//! locks the number of rules or members.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use desktop_codegen::architecture::{self, Class, MEMBERS, RULES};
use desktop_codegen::{Plan, Problem, check, generate, repo_root};

/// Folders the copy skips, matching the architecture tool.
const SKIPPED: [&str; 7] = [
    ".git",
    "target",
    ".build",
    "DerivedData",
    "xcuserdata",
    "graphify-out",
    ".claude",
];

/// The bridge source.
const BRIDGE: &str = "crates/macos-ffi/src/lib.rs";

/// One copy of the repo under the target temp folder, removed on drop.
struct Copy {
    root: PathBuf,
}

impl Copy {
    /// Makes a fresh copy named `architecture_rules_<name>`.
    fn new(name: &str) -> Self {
        let root =
            Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("architecture_rules_{name}"));
        if root.exists() {
            fs::remove_dir_all(&root).expect("the old copy can be removed");
        }
        copy_tree(&repo_root(), &root);
        Self { root }
    }

    /// Replaces the first `find` with `replace` in `path`.
    fn replace(&self, path: &str, find: &str, replace: &str) -> &Self {
        let file = self.root.join(path);
        let text = fs::read_to_string(&file).expect("the file is readable");
        assert!(text.contains(find), "{find:?} is not in {path}");
        fs::write(&file, text.replacen(find, replace, 1)).expect("the file can be written");
        self
    }

    /// Appends `text` to `path`.
    fn append(&self, path: &str, text: &str) -> &Self {
        let file = self.root.join(path);
        let mut old = fs::read_to_string(&file).expect("the file is readable");
        old.push_str(text);
        fs::write(&file, old).expect("the file can be written");
        self
    }

    /// Creates `path` with `text`, making its folders.
    fn create(&self, path: &str, text: &str) -> &Self {
        let file = self.root.join(path);
        fs::create_dir_all(file.parent().expect("a parent")).expect("the folder can be made");
        fs::write(&file, text).expect("the file can be written");
        self
    }

    /// Deletes the file at `path`.
    fn delete(&self, path: &str) -> &Self {
        fs::remove_file(self.root.join(path)).expect("the file can be deleted");
        self
    }

    /// The plan of the copy.
    fn plan(&self) -> Result<Plan, Vec<Problem>> {
        (architecture::SOURCE.plan)(&self.root)
    }

    /// The problems of the copy, in order, which must exist.
    fn problems(&self) -> Vec<Problem> {
        self.plan().expect_err("plan must fail")
    }

    /// `(path, rule ID)` of each problem in order; read errors give
    /// `cannot read` as their ID.
    fn ids(&self) -> Vec<(String, String)> {
        ids(&self.problems())
    }

    /// Fails with every problem when the copy does not pass.
    fn assert_passes(&self, label: &str) {
        if let Err(problems) = self.plan() {
            let lines: Vec<String> = problems.iter().map(ToString::to_string).collect();
            panic!("{label} must pass:\n{}", lines.join("\n"));
        }
    }
}

impl Drop for Copy {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

/// Copies the repo tree from `from` to `to`, skipping [`SKIPPED`] names
/// and symlinks.
fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("the copy folder can be made");
    for entry in fs::read_dir(from).expect("the repo folder is readable") {
        let entry = entry.expect("the folder entry is readable");
        let name = entry.file_name();
        if SKIPPED.iter().any(|skipped| name == *skipped) {
            continue;
        }
        let kind = entry.file_type().expect("the entry type is readable");
        let target = to.join(&name);
        if kind.is_dir() {
            copy_tree(&entry.path(), &target);
        } else if kind.is_file() {
            fs::copy(entry.path(), &target).expect("the file can be copied");
        }
    }
}

/// `(path, rule ID)` of each problem in the given order.
fn ids(problems: &[Problem]) -> Vec<(String, String)> {
    problems
        .iter()
        .map(|problem| {
            let id = problem.message.split(':').next().unwrap_or_default();
            (problem.path.clone(), id.to_string())
        })
        .collect()
}

/// Expected `(path, rule ID)` pairs.
fn expect(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    pairs
        .iter()
        .map(|(path, id)| ((*path).to_string(), (*id).to_string()))
        .collect()
}

/// Covers AC-2 and AC-4 (`plan` order): problems come by rule number as a
/// number (`A-5` before `A-10` before `A-12`), then path in byte order,
/// then line, not in the order of the patterns.
#[test]
fn problems_are_sorted_by_rule_number_then_path_then_line() {
    let copy = Copy::new("order");
    copy.append("README.md", "[x](missing.md)\n")
        .create(
            "apps/macos/Presentation/Direct.swift",
            "import DesktopMacosFFI\n",
        )
        .append("crates/core/src/lib.rs", "// cfg(windows)\n")
        .append(
            "crates/core/src/greeting.rs",
            "// cfg(unix)\n// target_os\n",
        )
        .create("rustfmt.toml", "");
    let problems = copy.problems();
    assert_eq!(
        ids(&problems),
        expect(&[
            ("rustfmt.toml", "A-5"),
            ("crates/core/src/greeting.rs", "A-6"),
            ("crates/core/src/greeting.rs", "A-6"),
            ("crates/core/src/lib.rs", "A-6"),
            ("apps/macos/Presentation/Direct.swift", "A-10"),
            ("README.md", "A-12"),
        ]),
        "AC-4: order: {problems:?}"
    );
    assert!(
        problems[1].message.ends_with("(cfg(unix))"),
        "line order: {}",
        problems[1]
    );
    assert!(
        problems[2].message.ends_with("(target_os)"),
        "line order: {}",
        problems[2]
    );
}

/// Covers AC-2 and AC-5 (fix `c69fd44`): an unreadable member manifest is
/// reported once and first, the package parts of `A-1` and `A-13` are
/// skipped, and the other rules still run.
#[test]
fn unreadable_member_manifest_is_reported_once_and_skips_package_checks() {
    let copy = Copy::new("member_unreadable");
    copy.delete("crates/macos-ffi/Cargo.toml")
        .append("AGENTS.md", "Run cargo test -p desktop-nope here.\n")
        .append(BRIDGE, "// shared/config\n")
        .append("README.md", "[x](missing.md)\n");
    assert_eq!(
        copy.ids(),
        expect(&[
            ("crates/macos-ffi/Cargo.toml", "cannot read"),
            (BRIDGE, "A-7"),
            ("README.md", "A-12"),
        ]),
        "AC-2: one read problem, package checks skipped, other rules run"
    );
}

/// Covers AC-2 ("File yang tidak terbaca"): without the root manifest the
/// member rules and the package part of `A-13` are skipped, and the config
/// file part of `A-5` and the text rules still run.
#[test]
fn unreadable_root_manifest_skips_member_rules() {
    let copy = Copy::new("root_unreadable");
    copy.delete("Cargo.toml")
        .create("rustfmt.toml", "")
        .append(BRIDGE, "// shared/config\n")
        .append("AGENTS.md", "Run cargo test -p desktop-nope here.\n");
    assert_eq!(
        copy.ids(),
        expect(&[
            ("Cargo.toml", "cannot read"),
            ("rustfmt.toml", "A-5"),
            (BRIDGE, "A-7"),
        ]),
        "AC-2: one read problem, member rules skipped"
    );
}

/// Covers AC-2 ("File yang tidak terbaca"): without the lockfile `A-1` to
/// `A-4` are skipped and `A-5` still runs.
#[test]
fn unreadable_lockfile_skips_the_graph_rules() {
    let copy = Copy::new("lock_unreadable");
    copy.delete("Cargo.lock").create("rustfmt.toml", "");
    assert_eq!(
        copy.ids(),
        expect(&[("Cargo.lock", "cannot read"), ("rustfmt.toml", "A-5")]),
        "AC-2: graph rules skipped"
    );
}

/// Covers AC-4 ("Satu masalah per baris"): a line with two patterns gives
/// one problem, for the first pattern in list order.
#[test]
fn one_problem_per_line_for_the_first_pattern() {
    let copy = Copy::new("first_pattern");
    copy.append(BRIDGE, "// public.json and shared/config\n");
    let problems = copy.problems();
    assert_eq!(
        ids(&problems),
        expect(&[(BRIDGE, "A-7")]),
        "AC-4: one problem"
    );
    assert!(
        problems[0].message.ends_with("(shared/config)"),
        "AC-4: first pattern in list order: {}",
        problems[0]
    );
}

/// Covers AC-2 (`A-4` classes): a name is in a class when it equals a
/// pattern or continues with `-`, `_`, or a digit.
#[test]
fn class_patterns_match_at_name_boundaries() {
    for (class, name) in [
        (Class::Ui, "gtk"),
        (Class::Ui, "gtk4"),
        (Class::Ui, "gtk-rs"),
        (Class::Ui, "gtk_sys"),
        (Class::Ui, "windows-reactor"),
        (Class::OsSdk, "windows"),
        (Class::OsSdk, "windows-sys"),
        (Class::OsSdk, "objc2-foundation"),
        (Class::OsSdk, "core-foundation-sys"),
        (Class::Binding, "uniffi_core"),
    ] {
        assert!(class.matches(name), "AC-2: {name} is {}", class.name());
    }
    for (class, name) in [
        (Class::Ui, "gioco"),
        (Class::Ui, "glibc"),
        (Class::Ui, "gtkx"),
        (Class::OsSdk, "windowsx"),
        (Class::OsSdk, "cocoapods"),
        (Class::Binding, "uniffied"),
    ] {
        assert!(!class.matches(name), "AC-2: {name} is not {}", class.name());
    }
}

/// Covers AC-8 (`A-5`): the lint lines must be set exactly and under
/// `[lints.rust]`, and config files are found at any depth.
#[test]
fn lint_and_config_file_edges() {
    let deny = Copy::new("lint_deny");
    deny.replace(
        "crates/core/Cargo.toml",
        "missing_docs = \"warn\"",
        "missing_docs = \"deny\"",
    );
    assert_eq!(
        deny.ids(),
        expect(&[("crates/core/Cargo.toml", "A-5")]),
        "AC-8: deny"
    );

    let clippy_table = Copy::new("lint_table");
    clippy_table.replace("crates/core/Cargo.toml", "[lints.rust]", "[lints.clippy]");
    assert_eq!(
        clippy_table.ids(),
        expect(&[
            ("crates/core/Cargo.toml", "A-5"),
            ("crates/core/Cargo.toml", "A-5")
        ]),
        "AC-8: both lines missing from [lints.rust]"
    );

    let nested = Copy::new("lint_nested");
    nested.create("tooling/codegen/.clippy.toml", "");
    assert_eq!(
        nested.ids(),
        expect(&[("tooling/codegen/.clippy.toml", "A-5")]),
        "AC-8: nested"
    );
}

/// Covers AC-4 (`A-6`): OS branching is banned only in `crates/core/src/`,
/// reaching outside is banned in every `.rs` file of `crates/core/`.
#[test]
fn core_boundary_edges() {
    let os_in_tests = Copy::new("core_os_tests");
    os_in_tests.create("crates/core/tests/os.rs", "// target_os\n");
    os_in_tests.assert_passes("AC-4: target_os in crates/core/tests");

    let outside_in_tests = Copy::new("core_outside_tests");
    outside_in_tests.create("crates/core/tests/outside.rs", "// CARGO_MANIFEST_DIR\n");
    assert_eq!(
        outside_in_tests.ids(),
        expect(&[("crates/core/tests/outside.rs", "A-6")]),
        "AC-4: CARGO_MANIFEST_DIR in crates/core/tests"
    );

    let windows = Copy::new("core_windows");
    windows.append("crates/core/src/greeting.rs", "// cfg(windows)\n");
    assert_eq!(
        windows.ids(),
        expect(&[("crates/core/src/greeting.rs", "A-6")]),
        "AC-4"
    );
}

/// Covers AC-4 (`A-8`): the greeting text is banned in the non test part
/// of a source, allowed after `#[cfg(test)]`, and only the exact text with
/// one space counts.
#[test]
fn greeting_text_edges() {
    let copy = Copy::new("greeting");
    copy.create(
        "tooling/codegen/src/extra.rs",
        "const A: &str = \"Hello, x\";\n#[cfg(test)]\nmod t {\n    const B: &str = \"Hello, y\";\n}\n",
    )
    .create("crates/macos-ffi/src/extra.rs", "const C: &str = \"Hello,x\";\n");
    assert_eq!(
        copy.ids(),
        expect(&[("tooling/codegen/src/extra.rs", "A-8")]),
        "AC-4: only the non test line"
    );
}

/// Covers AC-4 (`A-9`): a Core re-export with braces or `as` cannot be read.
#[test]
fn core_re_export_must_be_plain() {
    for (name, replacement) in [
        ("braces", "pub use greeting::{hello};"),
        ("alias", "pub use greeting::hello as hello;"),
    ] {
        let copy = Copy::new(&format!("reexport_{name}"));
        copy.replace(
            "crates/core/src/lib.rs",
            "pub use greeting::hello;",
            replacement,
        );
        let problems = copy.problems();
        assert!(
            problems
                .iter()
                .any(|problem| problem.path == "crates/core/src/lib.rs"
                    && problem.message.starts_with("A-9: cannot read re-export ")),
            "AC-4: {name}: {problems:?}"
        );
    }
}

/// Covers AC-4 (`A-10`): an import counts when the line ends with it, also
/// after an attribute, and the C module may not be imported even in
/// `Interop`.
#[test]
fn swift_import_edges() {
    let testable = Copy::new("import_testable");
    testable.create(
        "apps/macos/Presentation/Extra.swift",
        "@testable import DesktopMacosFFI\n",
    );
    assert_eq!(
        testable.ids(),
        expect(&[("apps/macos/Presentation/Extra.swift", "A-10")]),
        "AC-4: @testable import"
    );

    let longer = Copy::new("import_longer");
    longer.create(
        "apps/macos/Presentation/Extra.swift",
        "import DesktopMacosFFIExtra\n",
    );
    longer.assert_passes("AC-4: a longer module name");

    let c_module = Copy::new("import_c_module");
    c_module.create(
        "apps/macos/Interop/Extra.swift",
        "import desktop_macos_ffiFFI\n",
    );
    assert_eq!(
        c_module.ids(),
        expect(&[("apps/macos/Interop/Extra.swift", "A-10")]),
        "AC-4: the C module"
    );
}

/// Covers AC-4 (`A-11`): `.package(id:` in `Package.swift` and a remote
/// reference in `.pbxproj` are rejected.
#[test]
fn remote_package_edges() {
    let by_id = Copy::new("remote_id");
    by_id.append(
        "apps/macos/Package.swift",
        "// .package(id: \"a.b\", from: \"1.0.0\")\n",
    );
    assert_eq!(
        by_id.ids(),
        expect(&[("apps/macos/Package.swift", "A-11")]),
        "AC-4: id"
    );

    let pbxproj = "apps/macos/DesktopMac.xcodeproj/project.pbxproj";
    let xcode = Copy::new("remote_pbxproj");
    xcode.append(pbxproj, "// XCRemoteSwiftPackageReference\n");
    assert_eq!(xcode.ids(), expect(&[(pbxproj, "A-11")]), "AC-4: pbxproj");
}

/// Covers AC-5 (`A-12`): fenced blocks, paired backticks, titled links,
/// schemes, `<`, `/`, and `#` links are skipped; `%20` is decoded, anchors
/// are dropped, folders count, paths are relative to the file, and
/// `docs/scope/coordination/` and `.claude/` are not checked.
#[test]
fn markdown_links_that_are_accepted() {
    let copy = Copy::new("links_ok");
    copy.create(
        "docs/extra.md",
        concat!(
            "[a](../README.md) [b](../docs/) [c](scope/001%20-%20Batas%20workspace%20dan%20pemilik.md)\n",
            "[d](../AGENTS.md#anything) [e](https://example.invalid/x) [f](mailto:a@b.c)\n",
            "[g](<missing.md>) [h](/missing.md) [i](missing.md \"title\") [j](#local)\n",
            "`[k](missing.md)`\n",
            "```\n",
            "[l](missing.md)\n",
            "```\n",
        ),
    )
    .create("docs/scope/coordination/extra.md", "[x](missing.md)\n")
    .create(".claude/extra.md", "[x](missing.md)\n");
    copy.assert_passes("AC-5: accepted links");
}

/// Covers AC-5 (`A-12`): an unpaired backtick does not hide a link, `%20`
/// is decoded before the path is looked up, and an anchor does not save a
/// missing file. One problem per line, with its line number.
#[test]
fn markdown_links_that_are_rejected() {
    let copy = Copy::new("links_broken");
    copy.create(
        "docs/extra.md",
        concat!(
            "[a](missing.md)\n",
            "`unpaired [b](missing2.md)\n",
            "[c](scope/001%20-%20Nope.md)\n",
            "[d](missing.md#x)\n",
        ),
    );
    let problems = copy.problems();
    assert_eq!(
        ids(&problems),
        expect(&[
            ("docs/extra.md", "A-12"),
            ("docs/extra.md", "A-12"),
            ("docs/extra.md", "A-12"),
            ("docs/extra.md", "A-12"),
        ]),
        "AC-5: {problems:?}"
    );
    for (index, problem) in problems.iter().enumerate() {
        assert!(
            problem
                .message
                .ends_with(&format!(" on line {}", index + 1)),
            "AC-5: line number: {problem}"
        );
    }
}

/// Covers AC-5 (`A-13`): paths are tried relative to the `AGENTS.md` folder
/// and to the repo root, candidates come from backticks outside fences and
/// words inside fences, and spec references and package names are checked
/// on every line.
#[test]
fn agents_references() {
    let accepted = Copy::new("agents_ok");
    accepted.append(
        "crates/core/AGENTS.md",
        concat!(
            "\n`src/lib.rs` `crates/core/src/lib.rs` `docs/specs` `<os>/x.md` `_root/0001` `_root/001`\n",
            "Run cargo test -p desktop-core.\n",
            "```\n",
            "cargo test -p desktop-codegen tooling/codegen/src/lib.rs\n",
            "```\n",
        ),
    );
    accepted.assert_passes("AC-5: accepted references");

    let rejected = Copy::new("agents_broken");
    rejected.append(
        "crates/core/AGENTS.md",
        concat!(
            "\n`src/nope.rs`\n",
            "`macos/9999`\n",
            "```\n",
            "cargo test -p desktop-nope tooling/codegen/src/nope.rs\n",
            "```\n",
        ),
    );
    let problems = rejected.problems();
    let messages: Vec<&str> = problems.iter().map(|p| p.message.as_str()).collect();
    assert_eq!(
        messages,
        [
            "A-13: path src/nope.rs does not exist",
            "A-13: spec macos/9999 does not exist",
            "A-13: package desktop-nope is not a workspace member",
            "A-13: path tooling/codegen/src/nope.rs does not exist",
        ],
        "AC-5: {problems:?}"
    );
    assert!(problems.iter().all(|p| p.path == "crates/core/AGENTS.md"));
}

/// Covers AC-1: the copy gives the same plan as the repo, with no outputs,
/// and `generate` writes nothing.
#[test]
fn plan_does_not_depend_on_the_root_and_generate_writes_nothing() {
    let copy = Copy::new("elsewhere");
    let here = (architecture::SOURCE.plan)(&repo_root()).expect("the repo passes");
    let there = copy.plan().expect("the copy passes");
    assert_eq!(there, here, "AC-1: plan depends on the root");
    assert!(here.outputs.is_empty(), "AC-1: no outputs");
    let (written, line) = generate(&copy.root, &architecture::SOURCE).expect("generate succeeds");
    assert!(written.is_empty(), "AC-1: generate wrote {written:?}");
    assert!(line.ends_with(" 0 outputs, 0 written"), "AC-1: {line}");
}

/// Covers AC-1: the success line counts `RULES` and `MEMBERS`, the rule IDs
/// are unique, and every member rule names a distinct package.
#[test]
fn rules_and_success_line_agree() {
    let line = check(&repo_root(), &architecture::SOURCE).expect("the repo passes");
    assert_eq!(
        line,
        format!(
            "architecture: ok, {} rules, {} members, 0 outputs up to date",
            RULES.len(),
            MEMBERS.len()
        ),
        "AC-1: success line"
    );
    let mut ids: Vec<&str> = RULES.to_vec();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), RULES.len(), "AC-1: rule IDs are unique");
    let mut packages: Vec<&str> = MEMBERS.iter().map(|rule| rule.package).collect();
    packages.sort_unstable();
    packages.dedup();
    assert_eq!(packages.len(), MEMBERS.len(), "AC-1: one rule per package");
}

/// Runs the command line with `args` and returns code, stdout, and stderr.
fn run(args: &[&str]) -> (Option<i32>, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_desktop-codegen"))
        .args(args)
        .output()
        .expect("the binary runs");
    (
        out.status.code(),
        String::from_utf8(out.stdout).expect("stdout is UTF-8"),
        String::from_utf8(out.stderr).expect("stderr is UTF-8"),
    )
}

/// Covers AC-1: `architecture check` exits with 0 and prints exactly the
/// success line; `architecture` and `architecture nope` exit with 2 and the
/// usage lines list `architecture`.
#[test]
fn architecture_command_exit_codes() {
    let (code, stdout, stderr) = run(&["architecture", "check"]);
    let line = check(&repo_root(), &architecture::SOURCE).expect("the repo passes");
    assert_eq!(code, Some(0), "AC-1: exit code, stderr: {stderr}");
    assert_eq!(stdout, format!("{line}\n"), "AC-1: one success line");
    assert_eq!(stderr, "", "AC-1: nothing on stderr");
    for args in [&["architecture"][..], &["architecture", "nope"]] {
        let (code, stdout, stderr) = run(args);
        assert_eq!(code, Some(2), "AC-1: {args:?}");
        assert_eq!(stdout, "", "AC-1: {args:?}");
        assert!(
            stderr
                .lines()
                .nth(1)
                .is_some_and(|line| line.starts_with("sources: ") && line.contains("architecture")),
            "AC-1: the sources line lists architecture: {stderr}"
        );
    }
}

/// Covers AC-4 ("Satu masalah per baris"): two lines that break the same
/// rule with the same pattern give two problems, one per line.
///
/// Fails today: `plan` drops problems with the same path and message, and
/// the `A-6` to `A-11` messages carry no line number. Sent to the developer
/// with a one line fix (also compare the line when deduplicating). Run it
/// with `-- --ignored` after the fix, then remove the `ignore` attribute.
#[test]
fn every_violating_line_gives_its_own_problem() {
    let copy = Copy::new("two_lines");
    copy.append(BRIDGE, "// shared/config\n// shared/config\n");
    assert_eq!(
        copy.ids(),
        expect(&[(BRIDGE, "A-7"), (BRIDGE, "A-7")]),
        "AC-4: one problem for each violating line"
    );
}
