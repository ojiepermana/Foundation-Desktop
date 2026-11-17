//! Rule tests for the macOS text rules `C-1` to `C-4` of spec
//! `macos/0008`, beyond the cases `D-1` to `D-8` and `B-1` to `B-3` in
//! `architecture_macos.rs`.
//!
//! They lock every pattern of `C-1` and `C-2` at its edge (rejected, and
//! the neighbouring forms that must pass), one problem per line and per
//! rule, which folders each rule reads, how `C-3` reads an import, `C-4`
//! for each setting and for an unreadable project file, the order of
//! problems across the `A` and `C` rules, and where `RULES` lists the `C`
//! rules. The rules live in
//! `docs/specs/macos/0008-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md`.
//! Every case runs on its own copy of the repo under
//! `env!("CARGO_TARGET_TMPDIR")`; the repo itself is never edited.

use std::fs;
use std::path::{Path, PathBuf};

use desktop_codegen::architecture::{self, RULES};
use desktop_codegen::{Problem, repo_root};

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

/// A view file read by `C-1` and `C-2`.
const VIEW: &str = "apps/macos/Presentation/GreetingView.swift";

/// The Xcode project file read by `C-4`.
const PROJECT: &str = "apps/macos/DesktopMac.xcodeproj/project.pbxproj";

/// One copy of the repo under the target temp folder, removed on drop.
struct Copy {
    root: PathBuf,
}

impl Copy {
    /// Makes a fresh copy named `architecture_macos_rules_<name>`.
    fn new(name: &str) -> Self {
        let root =
            Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("architecture_macos_rules_{name}"));
        if root.exists() {
            fs::remove_dir_all(&root).expect("the old copy can be removed");
        }
        copy_tree(&repo_root(), &root);
        Self { root }
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

    /// The problems of the copy in order, empty when it passes.
    fn problems(&self) -> Vec<Problem> {
        (architecture::SOURCE.plan)(&self.root)
            .err()
            .unwrap_or_default()
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

/// The messages of the problems.
fn messages(problems: &[Problem]) -> Vec<String> {
    problems.iter().map(ToString::to_string).collect()
}

/// Runs one copy per `(label, line)`: each line is appended to the view
/// file, and the problems are returned with the label.
fn per_line(name: &str, lines: &[&str]) -> Vec<(String, Vec<String>)> {
    let copy = Copy::new(name);
    let base = fs::read_to_string(copy.root.join(VIEW)).expect("readable");
    lines
        .iter()
        .map(|line| {
            fs::write(copy.root.join(VIEW), format!("{base}{line}\n")).expect("writable");
            ((*line).to_string(), messages(&copy.problems()))
        })
        .collect()
}

/// Covers AC-6 (`C-1`): every style pattern is rejected with its `<teks>`,
/// in the middle of a line and after a tab.
#[test]
fn every_style_pattern_is_rejected() {
    let cases = [
        ("VStack(spacing: 8)", "spacing:"),
        ("\t.padding(20)", "padding("),
        ("x .padding(.horizontal, 4)", "padding(."),
        (".padding(.top,4)", "padding(."),
        ("Spacer(minLength: 0)", "minLength:"),
        ("Spacer(minLength:   12)", "minLength:"),
        ("let c = Color(red: 1)", "Color("),
        ("let c = Color.red", "Color."),
        ("let c: NSColor", "NSColor"),
        ("NSColor.red", "Color."),
        ("let f = Font.body", "Font."),
        (".font(.title)", ".font(."),
        (".fontWeight(.bold)", "fontWeight"),
        (".bold()", ".bold("),
        (".foregroundStyle(.red)", ".foregroundStyle(."),
        ("view.foregroundColor = x", ".foregroundColor"),
        (".tint(.blue)", ".tint(."),
        (".background(.red)", ".background(."),
        ("Font.system(size: 12)", "Font."),
        ("x(size: 3)", "size:"),
    ];
    let lines: Vec<&str> = cases.iter().map(|(line, _)| *line).collect();
    for ((line, found), (_, expected)) in per_line("style", &lines).into_iter().zip(cases) {
        let want = format!("{VIEW}: C-1: style literal {expected} outside the generated tokens");
        assert_eq!(found, [want], "AC-6: {line:?}");
    }
}

/// Covers AC-6 (`C-1`, `C-2`): forms next to the patterns pass: a token in
/// place of a number, an upper case edge, no space after `spacing:`, and a
/// minimum width, which spec `macos/0005` keeps as a macOS constant.
#[test]
fn neighbouring_forms_pass() {
    let lines = [
        "VStack(spacing: Tokens.Spacing.gapControl)",
        "VStack(spacing:8)",
        ".padding(Tokens.Spacing.insetWindow)",
        ".padding(.horizontal, Tokens.Spacing.gapControl)",
        ".padding(.Horizontal, 4)",
        "Spacer(minLength: Tokens.Spacing.gapControl)",
        ".frame(minWidth: 320)",
        ".foregroundStyle(Tokens.Colors.textPrimary.color)",
        ".tint(Tokens.Colors.accent.color)",
        ".background(Tokens.Colors.background.color)",
        ".font(Tokens.Typography.body.font)",
    ];
    for (line, found) in per_line("neighbours", &lines) {
        assert!(found.is_empty(), "AC-6: {line:?} passes: {found:?}");
    }
}

/// Covers AC-6 (`C-2`): every control pattern is rejected with its text.
#[test]
fn every_control_pattern_is_rejected() {
    let patterns = [
        "focusable",
        "focusEffectDisabled",
        "focusRing",
        "controlSize",
        "frame(height",
        "frame(Height",
        "frame(minHeight",
        "frame(minheight",
        "frame(maxHeight",
        "frame(maxheight",
        "accessibilityHidden",
        ".disabled(",
        "ButtonStyle {",
        "TextFieldStyle {",
    ];
    let lines: Vec<String> = patterns
        .iter()
        .map(|pattern| format!("// x{pattern}"))
        .collect();
    let refs: Vec<&str> = lines.iter().map(String::as_str).collect();
    for ((line, found), pattern) in per_line("control", &refs).into_iter().zip(patterns) {
        assert_eq!(
            found,
            [format!("{VIEW}: C-2: native control override {pattern}")],
            "AC-6: {line:?}"
        );
    }
}

/// Covers AC-6 ("Aturan teks macOS"): one problem per line for the first
/// pattern in list order, and one problem per rule when a line breaks both
/// `C-1` and `C-2`.
#[test]
fn one_problem_per_line_and_rule() {
    let cases = [
        (
            "Color.red .padding(20)",
            vec!["C-1: style literal padding( outside the generated tokens"],
        ),
        (
            ".controlSize(.large) .focusable()",
            vec!["C-2: native control override focusable"],
        ),
        (
            ".padding(4).focusable()",
            vec![
                "C-1: style literal padding( outside the generated tokens",
                "C-2: native control override focusable",
            ],
        ),
    ];
    let lines: Vec<&str> = cases.iter().map(|(line, _)| *line).collect();
    for ((line, found), (_, expected)) in per_line("one_per_line", &lines).into_iter().zip(cases) {
        let want: Vec<String> = expected.iter().map(|m| format!("{VIEW}: {m}")).collect();
        assert_eq!(found, want, "AC-6: {line:?}");
    }
}

/// Covers AC-6 (files read by `C-1` and `C-2`): `Presentation/` and `App/`
/// are read, `Interop/`, `DesignSystem/`, and `Tests/` are not.
#[test]
fn style_and_control_rules_read_only_the_view_folders() {
    let copy = Copy::new("folders");
    copy.create("apps/macos/App/Extra.swift", "let c = Color.red\n")
        .create(
            "apps/macos/Interop/Extra.swift",
            "let c = Color.red\n.focusable()\n",
        )
        .create("apps/macos/DesignSystem/Extra.swift", "let c = Color.red\n")
        .create(
            "apps/macos/Tests/PresentationTests/Extra.swift",
            "let c = Color.red\n",
        );
    assert_eq!(
        messages(&copy.problems()),
        ["apps/macos/App/Extra.swift: C-1: style literal Color. outside the generated tokens"],
        "AC-6: only App and Presentation"
    );
}

/// Covers AC-6 (`C-3`): an import counts when the line ends with
/// `import DesignSystem`, read like `A-10`, so `@testable` and a comment
/// ending with it count and a longer module name does not; `Presentation`
/// and test files may import it.
#[test]
fn design_system_import_edges() {
    let copy = Copy::new("imports");
    copy.create(
        "apps/macos/Interop/A.swift",
        "@testable import DesignSystem\n",
    )
    .create("apps/macos/Interop/B.swift", "// import DesignSystem\n")
    .create("apps/macos/Interop/C.swift", "import DesignSystemExtra\n")
    .create("apps/macos/App/D.swift", "import DesignSystem\n")
    .create(
        "apps/macos/Presentation/E.swift",
        "@testable import DesignSystem\n",
    )
    .create("apps/macos/Tests/Extra/F.swift", "import DesignSystem\n");
    let message = "C-3: only apps/macos/Presentation imports DesignSystem";
    assert_eq!(
        messages(&copy.problems()),
        [
            format!("apps/macos/App/D.swift: {message}"),
            format!("apps/macos/Interop/A.swift: {message}"),
            format!("apps/macos/Interop/B.swift: {message}"),
        ],
        "AC-6: C-3 edges"
    );
}

/// Covers AC-7 (`C-4`): each generated setting is rejected, one problem per
/// line for the first setting in list order.
#[test]
fn each_generated_setting_is_rejected_once_per_line() {
    let copy = Copy::new("settings");
    copy.append(
        PROJECT,
        concat!(
            "// INFOPLIST_KEY_CFBundleDisplayName = X;\n",
            "// MARKETING_VERSION = 1.0.0;\n",
            "// PRODUCT_BUNDLE_IDENTIFIER = a.b;\n",
            "// MARKETING_VERSION = 1; PRODUCT_BUNDLE_IDENTIFIER = a.b;\n",
        ),
    );
    let found: Vec<String> = messages(&copy.problems());
    let tail = |setting: &str| {
        format!("{PROJECT}: C-4: {setting} belongs in apps/macos/Config/Shared.xcconfig")
    };
    assert_eq!(
        found,
        [
            tail("INFOPLIST_KEY_CFBundleDisplayName"),
            tail("MARKETING_VERSION"),
            tail("PRODUCT_BUNDLE_IDENTIFIER"),
            tail("MARKETING_VERSION"),
        ],
        "AC-7: one per line, first setting in list order"
    );
}

/// Covers AC-7 ("File yang dibaca `C-4`"): an unreadable project file gives
/// `C-4: cannot read` and only `C-4` is skipped; the other rules still run.
#[test]
fn unreadable_project_skips_only_c4() {
    let copy = Copy::new("no_project");
    fs::remove_file(copy.root.join(PROJECT)).expect("the project file can be removed");
    copy.append(VIEW, "let c = Color.red\n");
    let found = messages(&copy.problems());
    assert_eq!(found.len(), 2, "AC-7: {found:?}");
    assert!(
        found[0].contains("C-1: style literal Color."),
        "AC-7: {found:?}"
    );
    assert!(
        found[1].starts_with(&format!("{PROJECT}: C-4: cannot read: ")),
        "AC-7: {found:?}"
    );
}

/// Covers AC-6 and AC-7 ("Urutan masalah"): the letter first (`A` before
/// `C`), then the number as a number, then path and line.
#[test]
fn problems_are_sorted_by_letter_then_number() {
    let copy = Copy::new("order");
    copy.append("README.md", "[x](missing.md)\n")
        .create("rustfmt.toml", "")
        .append(PROJECT, "// MARKETING_VERSION = 1;\n")
        .create("apps/macos/Interop/Look.swift", "import DesignSystem\n")
        .append(VIEW, "let c = Color.red\n");
    let ids: Vec<String> = copy
        .problems()
        .iter()
        .map(|problem| {
            problem
                .message
                .split(':')
                .next()
                .unwrap_or_default()
                .to_string()
        })
        .collect();
    assert_eq!(ids, ["A-5", "A-12", "C-1", "C-3", "C-4"], "AC-6: order");
}

/// Covers AC-6 and AC-7: `RULES` lists every `A` rule before the `C` rules,
/// and the `C` rules are `C-1` to `C-4` in order.
#[test]
fn rules_list_the_c_rules_after_the_a_rules() {
    let first_c = RULES
        .iter()
        .position(|rule| rule.starts_with("C-"))
        .expect("RULES lists C rules");
    assert!(
        RULES[..first_c].iter().all(|rule| rule.starts_with("A-")),
        "AC-6: A rules first: {RULES:?}"
    );
    let c_rules: Vec<&str> = RULES
        .iter()
        .copied()
        .filter(|rule| rule.starts_with("C-"))
        .collect();
    assert_eq!(c_rules, ["C-1", "C-2", "C-3", "C-4"], "AC-6, AC-7: C rules");
    assert!(
        RULES[first_c..first_c + 4] == ["C-1", "C-2", "C-3", "C-4"],
        "AC-6: C rules are listed together: {RULES:?}"
    );
}
