//! Contract tests for the `architecture` source of spec `_root/0007`.
//!
//! These tests use only the public paths of `desktop_codegen` and the
//! standard library. `every_source_passes_check_on_the_repo` runs `check`
//! for every entry of `SOURCES`, so a registered source cannot slip past
//! the gate (AC-1, AC-3). `rejects_broken_repos` and
//! `accepts_allowed_shapes` run the cases `N-1` to `N-23` and `P-1` to
//! `P-5` of
//! `docs/specs/_root/0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md`
//! on a copy of the repo under a temporary root, restoring every changed
//! file before the next case (AC-2).

use std::fs;
use std::path::{Path, PathBuf};

use desktop_codegen::architecture;
use desktop_codegen::{SOURCES, check, repo_root};

#[test]
fn every_source_passes_check_on_the_repo() {
    let root = repo_root();
    let mut failures = Vec::new();
    for source in SOURCES {
        match check(&root, source) {
            Ok(line) => {
                if !line.starts_with(&format!("{}: ok, ", source.name)) {
                    failures.push(format!("{}: unexpected line: {line}", source.name));
                }
                if source.name == "architecture" && !line.ends_with("0 outputs up to date") {
                    failures.push(format!("architecture: unexpected line: {line}"));
                }
            }
            Err(problems) => {
                for problem in problems {
                    failures.push(format!("{}: {problem}", source.name));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Folders the copy skips. These are the folders the tool skips, except
/// `.claude`: the tool does not scan it, but the root `AGENTS.md` links
/// into it, so the copy keeps it for the `A-12` existence check.
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

/// Makes a fresh copy of the repo under `architecture_contract_<name>`.
fn copy_repo(name: &str) -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("architecture_contract_{name}"));
    if root.exists() {
        fs::remove_dir_all(&root).expect("the old copy can be removed");
    }
    copy_tree(&repo_root(), &root);
    root
}

/// One change to the copy of the repo.
#[derive(Clone, Copy)]
enum Edit {
    /// Replace the first `.1` with `.2` in file `.0`.
    Replace(&'static str, &'static str, &'static str),
    /// Append text `.1` to file `.0`.
    Append(&'static str, &'static str),
    /// Create file `.0` with text `.1`, making its folders.
    Create(&'static str, &'static str),
    /// Add entry `.1` to the `dependencies` of lock block `.0`.
    Edge(&'static str, &'static str),
    /// Remove entry `.1` from the `dependencies` of lock block `.0`.
    Unedge(&'static str, &'static str),
    /// Delete file `.0`.
    Delete(&'static str),
    /// Add a lock block for package `.0` at version `.1`.
    Block(&'static str, &'static str),
}

impl Edit {
    /// The file the edit changes, from the repo root.
    fn path(self) -> &'static str {
        match self {
            Edit::Replace(path, ..)
            | Edit::Append(path, _)
            | Edit::Create(path, _)
            | Edit::Delete(path) => path,
            Edit::Edge(..) | Edit::Unedge(..) | Edit::Block(..) => "Cargo.lock",
        }
    }
}

/// The byte range of the lock block of `package`.
fn lock_block(lock: &str, package: &str) -> (usize, usize) {
    let start = lock
        .find(&format!("[[package]]\nname = \"{package}\"\n"))
        .unwrap_or_else(|| panic!("no lock block for {package}"));
    let end = lock[start + 1..]
        .find("\n[[")
        .map_or(lock.len(), |offset| start + 1 + offset + 1);
    (start, end)
}

/// Applies one edit to the text of its file.
fn apply(edit: Edit, text: &str) -> String {
    match edit {
        Edit::Replace(_, find, replace) => {
            assert!(text.contains(find), "{find:?} is not in the file");
            text.replacen(find, replace, 1)
        }
        Edit::Append(_, extra) | Edit::Create(_, extra) => format!("{text}{extra}"),
        Edit::Delete(_) => unreachable!("run_case deletes the file itself"),
        Edit::Edge(package, dependency) => {
            let (start, end) = lock_block(text, package);
            let block = &text[start..end];
            let changed = if block.contains("dependencies = [\n") {
                block.replacen(
                    "dependencies = [\n",
                    &format!("dependencies = [\n \"{dependency}\",\n"),
                    1,
                )
            } else {
                let after_version = block
                    .find("version = ")
                    .and_then(|at| block[at..].find('\n').map(|end| at + end + 1))
                    .expect("the block has a version line");
                format!(
                    "{}dependencies = [\n \"{dependency}\",\n]\n{}",
                    &block[..after_version],
                    &block[after_version..]
                )
            };
            format!("{}{changed}{}", &text[..start], &text[end..])
        }
        Edit::Unedge(package, dependency) => {
            let (start, end) = lock_block(text, package);
            let line = format!(" \"{dependency}\",\n");
            assert!(
                text[start..end].contains(&line),
                "{dependency} is not a dependency of {package}"
            );
            let changed = text[start..end].replacen(&line, "", 1);
            format!("{}{changed}{}", &text[..start], &text[end..])
        }
        Edit::Block(package, version) => {
            format!("{text}\n[[package]]\nname = \"{package}\"\nversion = \"{version}\"\n")
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
        if let Edit::Delete(_) = edit {
            fs::remove_file(&path).expect("the file can be deleted");
            continue;
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

/// One `N` case: edits, the rule a problem must start with, and texts that
/// problem must contain.
struct Rejected {
    number: &'static str,
    edits: &'static [Edit],
    rules: &'static [&'static str],
    contains: &'static [&'static str],
}

/// The `N` cases of spec `_root/0007`.
const REJECTED: &[Rejected] = &[
    Rejected {
        number: "N-1",
        edits: &[
            Edit::Replace(
                "Cargo.toml",
                "\"tooling/codegen\"]",
                "\"tooling/codegen\", \"crates/extra\"]",
            ),
            Edit::Create(
                "crates/extra/Cargo.toml",
                "[package]\nname = \"desktop-extra\"\nversion = \"0.1.0\"\n",
            ),
            Edit::Block("desktop-extra", "0.1.0"),
        ],
        rules: &["A-1"],
        contains: &[],
    },
    Rejected {
        number: "N-2",
        edits: &[Edit::Edge("desktop-core", "desktop-macos-ffi")],
        rules: &["A-2"],
        contains: &[],
    },
    Rejected {
        number: "N-3",
        edits: &[Edit::Edge("desktop-macos-ffi", "desktop-codegen")],
        rules: &["A-2"],
        contains: &[],
    },
    Rejected {
        number: "N-4",
        edits: &[Edit::Edge("desktop-core", "itoa")],
        rules: &["A-3"],
        contains: &[],
    },
    Rejected {
        number: "N-5",
        edits: &[Edit::Unedge("desktop-codegen", "serde_json")],
        rules: &["A-3"],
        contains: &[],
    },
    Rejected {
        number: "N-6",
        edits: &[
            Edit::Block("gtk4", "0.0.0"),
            Edit::Edge("serde_json", "gtk4"),
        ],
        rules: &["A-4"],
        contains: &["desktop-codegen", "gtk4", "class ui"],
    },
    Rejected {
        number: "N-7",
        edits: &[Edit::Replace(
            "crates/macos-ffi/Cargo.toml",
            "unsafe_code = \"forbid\"\n",
            "",
        )],
        rules: &["A-5"],
        contains: &[],
    },
    Rejected {
        number: "N-8",
        edits: &[Edit::Create("rustfmt.toml", "")],
        rules: &["A-5"],
        contains: &[],
    },
    Rejected {
        number: "N-9",
        edits: &[Edit::Append(
            "crates/core/src/greeting.rs",
            "#[cfg(target_os = \"macos\")]\n",
        )],
        rules: &["A-6"],
        contains: &[],
    },
    Rejected {
        number: "N-10",
        edits: &[Edit::Create(
            "crates/core/tests/outside.rs",
            "const X: &str = include_str!(\"../../../Cargo.toml\");\n",
        )],
        rules: &["A-6"],
        contains: &[],
    },
    Rejected {
        number: "N-11",
        edits: &[Edit::Append(
            "crates/macos-ffi/src/lib.rs",
            "// shared/config/public.json\n",
        )],
        rules: &["A-7"],
        contains: &[],
    },
    Rejected {
        number: "N-12",
        edits: &[Edit::Create(
            "apps/macos/Presentation/Local.swift",
            "let text = \"Hello, \" + name\n",
        )],
        rules: &["A-8"],
        contains: &[],
    },
    Rejected {
        number: "N-13",
        edits: &[Edit::Replace(
            "crates/macos-ffi/src/lib.rs",
            "desktop_core::hello(name)",
            "format!(\"Hello, {name}!\")",
        )],
        rules: &["A-8", "A-9"],
        contains: &[],
    },
    Rejected {
        number: "N-14",
        edits: &[Edit::Replace(
            "crates/macos-ffi/src/lib.rs",
            "#[cfg(test)]",
            "pub fn extra() -> String {\n    String::new()\n}\n\n#[cfg(test)]",
        )],
        rules: &["A-9"],
        contains: &[],
    },
    Rejected {
        number: "N-15",
        edits: &[Edit::Replace(
            "crates/macos-ffi/src/lib.rs",
            "desktop_core::hello(name)",
            "desktop_core::hello(name.trim().to_string())",
        )],
        rules: &["A-9"],
        contains: &[],
    },
    Rejected {
        number: "N-16",
        edits: &[Edit::Replace(
            "crates/macos-ffi/src/lib.rs",
            "pub fn hello(name: String) -> String {",
            "pub fn hello(name: &str) -> String {",
        )],
        rules: &["A-9"],
        contains: &[],
    },
    Rejected {
        number: "N-17",
        edits: &[Edit::Replace(
            "bindings/swift/Sources/DesktopMacosFFI/desktop_macos_ffi.swift",
            "public func hello(",
            "public func hallo(",
        )],
        rules: &["A-9"],
        contains: &[],
    },
    Rejected {
        number: "N-18",
        edits: &[Edit::Create(
            "apps/macos/Presentation/Direct.swift",
            "import DesktopMacosFFI\n",
        )],
        rules: &["A-10"],
        contains: &[],
    },
    Rejected {
        number: "N-19",
        edits: &[Edit::Append(
            "apps/macos/Package.swift",
            "// .package(url: \"https://example.com/x.git\", from: \"1.0.0\")\n",
        )],
        rules: &["A-11"],
        contains: &[],
    },
    Rejected {
        number: "N-20",
        edits: &[Edit::Append("README.md", "[rusak](docs/tidak-ada.md)\n")],
        rules: &["A-12"],
        contains: &[],
    },
    Rejected {
        number: "N-21",
        edits: &[Edit::Append("AGENTS.md", "`_root/9999`\n")],
        rules: &["A-13"],
        contains: &[],
    },
    Rejected {
        number: "N-22",
        edits: &[Edit::Append("AGENTS.md", "cargo test -p desktop-nope\n")],
        rules: &["A-13"],
        contains: &[],
    },
    Rejected {
        number: "N-23",
        edits: &[Edit::Append(
            "crates/core/AGENTS.md",
            "`crates/core/tests/tidak_ada.rs`\n",
        )],
        rules: &["A-13"],
        contains: &[],
    },
];

#[test]
fn rejects_broken_repos() {
    let root = copy_repo("rejects_broken_repos");
    let mut failures = Vec::new();
    for case in REJECTED {
        let found = match run_case(&root, case.edits) {
            Ok(summary) => vec![format!("(accepted: {summary})")],
            Err(messages) => messages,
        };
        for rule in case.rules {
            let prefix = format!("{rule}: ");
            let hit = found.iter().any(|message| {
                message.starts_with(&prefix)
                    && case.contains.iter().all(|text| message.contains(text))
            });
            if !hit {
                failures.push(format!(
                    "{}: expected a {rule} problem, got {found:?}",
                    case.number
                ));
            }
        }
    }
    // A member manifest that cannot be read gives exactly one `cannot read`
    // problem. The A-1 rule check needs that member's package name, so it
    // is skipped ("File yang tidak terbaca" in spec `_root/0007`).
    let found = match run_case(&root, &[Edit::Delete("crates/macos-ffi/Cargo.toml")]) {
        Ok(summary) => vec![format!("(accepted: {summary})")],
        Err(messages) => messages,
    };
    if found.len() != 1 || !found[0].starts_with("cannot read: ") {
        failures.push(format!(
            "unreadable member manifest: expected exactly one cannot read problem, got {found:?}"
        ));
    }
    fs::remove_dir_all(&root).expect("the copy can be removed");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// The `P` cases of spec `_root/0007`: edits that `plan` must accept.
const ACCEPTED: &[(&str, &[Edit])] = &[
    ("P-1", &[]),
    (
        "P-2",
        &[Edit::Append(
            "tooling/codegen/src/lib.rs",
            "// shared/config\n",
        )],
    ),
    (
        "P-3",
        &[Edit::Create(
            "apps/macos/Tests/PresentationTests/Extra.swift",
            "let text = \"Hello, Ayu!\"\n",
        )],
    ),
    ("P-4", &[Edit::Append("README.md", "`[x](tidak-ada.md)`\n")]),
    (
        "P-5",
        &[Edit::Append(
            "AGENTS.md",
            "`docs/specs/windows/` `.cargo/config.toml`\n",
        )],
    ),
];

#[test]
fn accepts_allowed_shapes() {
    let root = copy_repo("accepts_allowed_shapes");
    let mut failures = Vec::new();
    for (number, edits) in ACCEPTED {
        if let Err(messages) = run_case(&root, edits) {
            failures.push(format!("{number}: {messages:?}"));
        }
    }
    fs::remove_dir_all(&root).expect("the copy can be removed");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
