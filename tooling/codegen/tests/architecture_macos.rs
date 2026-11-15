//! Contract tests for the macOS text rules `C-1` to `C-4` of spec
//! `macos/0008`.
//!
//! These tests use only the public paths of `desktop_codegen` and the
//! standard library. `rejects_macos_violations` runs the `D` cases and
//! `accepts_macos_shapes` the `B` cases of
//! `docs/specs/macos/0008-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md`
//! on a copy of the repo under a temporary root, restoring every changed
//! file before the next case (AC-6, AC-7).

use std::fs;
use std::path::{Path, PathBuf};

use desktop_codegen::architecture;
use desktop_codegen::repo_root;

/// Folders the copy skips: the folders the tool skips, except `.claude`,
/// which the root `AGENTS.md` links to (the same copy as
/// `architecture_contract.rs`).
const SKIPPED: [&str; 6] = [
    ".git",
    "target",
    ".build",
    "DerivedData",
    "xcuserdata",
    "graphify-out",
];

/// Copies the repo tree from `from` to `to`, skipping [`SKIPPED`] names
/// and symlinks.
fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("the copy folder can be made");
    let entries = fs::read_dir(from).expect("the repo folder is readable");
    for entry in entries {
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

/// Makes a fresh copy of the repo under `architecture_macos_<name>`.
fn copy_repo(name: &str) -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("architecture_macos_{name}"));
    if root.exists() {
        fs::remove_dir_all(&root).expect("the old copy can be removed");
    }
    copy_tree(&repo_root(), &root);
    root
}

/// One change to the copy of the repo.
#[derive(Clone, Copy)]
enum Edit {
    /// Append the line `.1` to file `.0`.
    Append(&'static str, &'static str),
    /// Create file `.0` with text `.1`, making its folders.
    Create(&'static str, &'static str),
    /// Insert the line `.1` before the last line of file `.0`.
    BeforeLastLine(&'static str, &'static str),
}

impl Edit {
    /// The file the edit changes, from the repo root.
    fn path(self) -> &'static str {
        match self {
            Edit::Append(path, _) | Edit::Create(path, _) | Edit::BeforeLastLine(path, _) => path,
        }
    }
}

/// Applies one edit to the text of its file.
fn apply(edit: Edit, text: &str) -> String {
    match edit {
        Edit::Append(_, line) => {
            let separator = if text.is_empty() || text.ends_with('\n') {
                ""
            } else {
                "\n"
            };
            format!("{text}{separator}{line}\n")
        }
        Edit::Create(_, contents) => contents.to_string(),
        Edit::BeforeLastLine(_, line) => {
            let body = text.strip_suffix('\n').unwrap_or(text);
            let cut = body.rfind('\n').map_or(0, |at| at + 1);
            format!("{}{line}\n{}", &text[..cut], &text[cut..])
        }
    }
}

/// Applies `edits` to the copy at `root`, runs `plan`, then restores every
/// changed file and removes every created one.
fn run_case(root: &Path, edits: &[Edit]) -> Result<String, Vec<String>> {
    let mut originals: Vec<(PathBuf, Option<Vec<u8>>)> = Vec::new();
    let mut created_folders: Vec<PathBuf> = Vec::new();
    for edit in edits {
        let path = root.join(edit.path());
        if !originals.iter().any(|(seen, _)| *seen == path) {
            originals.push((path.clone(), fs::read(&path).ok()));
        }
        if let Some(parent) = path.parent() {
            let mut missing = Vec::new();
            let mut folder = parent.to_path_buf();
            while !folder.exists() {
                missing.push(folder.clone());
                if !folder.pop() {
                    break;
                }
            }
            fs::create_dir_all(parent).expect("the folder can be made");
            created_folders.extend(missing);
        }
        let text = fs::read_to_string(&path).unwrap_or_default();
        fs::write(&path, apply(*edit, &text)).expect("the copy can be edited");
    }
    let result = (architecture::SOURCE.plan)(root)
        .map(|plan| plan.summary)
        .map_err(|problems| {
            problems
                .iter()
                .map(|problem| problem.message.clone())
                .collect()
        });
    for (path, original) in originals {
        match original {
            Some(bytes) => fs::write(&path, bytes).expect("the file can be restored"),
            None => fs::remove_file(&path).expect("the created file can be removed"),
        }
    }
    for folder in created_folders {
        let _ = fs::remove_dir(folder);
    }
    result
}

/// The `D` cases of spec `macos/0008`: (number, edits, rule).
const REJECTED: &[(&str, &[Edit], &str)] = &[
    (
        "D-1",
        &[Edit::Append(
            "apps/macos/Presentation/GreetingView.swift",
            ".padding(20)",
        )],
        "C-1",
    ),
    (
        "D-2",
        &[Edit::Append(
            "apps/macos/App/DesktopMacApp.swift",
            "let accentColor = Color.red",
        )],
        "C-1",
    ),
    (
        "D-3",
        &[Edit::Append(
            "apps/macos/Presentation/GreetingView.swift",
            "// VStack(spacing: 8)",
        )],
        "C-1",
    ),
    (
        "D-4",
        &[Edit::Append(
            "apps/macos/Presentation/GreetingView.swift",
            "// .padding(.horizontal, 4)",
        )],
        "C-1",
    ),
    (
        "D-5",
        &[Edit::Append(
            "apps/macos/Presentation/GreetingView.swift",
            "// .controlSize(.large)",
        )],
        "C-2",
    ),
    (
        "D-6",
        &[Edit::Append(
            "apps/macos/App/DesktopMacApp.swift",
            "// .frame(minHeight: 40)",
        )],
        "C-2",
    ),
    (
        "D-7",
        &[Edit::Create(
            "apps/macos/Interop/Look.swift",
            "import DesignSystem\n",
        )],
        "C-3",
    ),
    (
        "D-8",
        &[Edit::BeforeLastLine(
            "apps/macos/DesktopMac.xcodeproj/project.pbxproj",
            "MARKETING_VERSION = 0.1.0;",
        )],
        "C-4",
    ),
];

#[test]
fn rejects_macos_violations() {
    let root = copy_repo("rejects_macos_violations");
    let mut failures = Vec::new();
    for (number, edits, rule) in REJECTED {
        let found = match run_case(&root, edits) {
            Ok(summary) => vec![format!("(accepted: {summary})")],
            Err(messages) => messages,
        };
        let prefix = format!("{rule}: ");
        if !found.iter().any(|message| message.starts_with(&prefix)) {
            failures.push(format!(
                "{number}: expected a {rule} problem, got {found:?}"
            ));
        }
    }
    fs::remove_dir_all(&root).expect("the copy can be removed");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// The `B` cases of spec `macos/0008`: edits that `plan` must accept.
const ACCEPTED: &[(&str, &[Edit])] = &[
    ("B-1", &[]),
    (
        "B-2",
        &[Edit::Create(
            "apps/macos/Tests/DesignSystemTests/Extra.swift",
            "import DesignSystem\nlet probe = Color.red\n",
        )],
    ),
    (
        "B-3",
        &[Edit::Append(
            "apps/macos/DesignSystem/Theme.swift",
            "// Color.red",
        )],
    ),
];

#[test]
fn accepts_macos_shapes() {
    let root = copy_repo("accepts_macos_shapes");
    let mut failures = Vec::new();
    for (number, edits) in ACCEPTED {
        if let Err(messages) = run_case(&root, edits) {
            failures.push(format!("{number}: {messages:?}"));
        }
    }
    fs::remove_dir_all(&root).expect("the copy can be removed");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
