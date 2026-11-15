//! Checks for the workspace rules of spec `_root/0001`.
//!
//! These tests lock only what
//! `docs/specs/_root/0001-batas-workspace-dan-pemilik/index.md` says holds
//! for good ("Invarian", "Aturan untuk manifest root", AC-1 to AC-8). They
//! do not lock what the spec plans to change: the member list,
//! `[workspace.dependencies]`, `[workspace.lints]`, `[profile.*]`, the
//! packages in `Cargo.lock`, or the pinned version itself. Root files are
//! read at compile time with `include_str!`; member manifests and the repo
//! tree are read at run time so new members are checked too. The commands
//! in `verify.md` stay the proof for AC-1, AC-2, AC-5, and AC-6, and the
//! Core dependency ban (AC-3) is locked in `contract_surface.rs`.

use std::fs;
use std::path::{Path, PathBuf};

const ROOT_MANIFEST: &str = include_str!("../../../Cargo.toml");
const TOOLCHAIN: &str = include_str!("../../../rust-toolchain.toml");
const GITIGNORE: &str = include_str!("../../../.gitignore");
const LOCKFILE: &str = include_str!("../../../Cargo.lock");
const ARCHITECTURE: &str = include_str!("../../../docs/architecture/README.md");
const SPEC: &str =
    include_str!("../../../docs/specs/_root/0001-batas-workspace-dan-pemilik/index.md");

/// The fields every package inherits from `[workspace.package]`.
const SHARED_FIELDS: [&str; 4] = ["version", "edition", "rust-version", "publish"];

/// The repo root, two levels above this crate.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("the repo root exists")
}

/// Returns the body lines of the TOML table named `header` (for example
/// `[workspace]`), trimmed, without blank lines or comments.
fn table<'a>(toml: &'a str, header: &str) -> Vec<&'a str> {
    let mut inside = false;
    let mut body = Vec::new();
    for line in toml.lines().map(str::trim) {
        if line.starts_with('[') {
            inside = line == header;
        } else if inside && !line.is_empty() && !line.starts_with('#') {
            body.push(line);
        }
    }
    body
}

/// Returns the value of `key = value` in `lines`, if present.
fn value<'a>(lines: &[&'a str], key: &str) -> Option<&'a str> {
    lines.iter().find_map(|line| {
        let (left, right) = line.split_once('=')?;
        (left.trim() == key).then(|| right.trim())
    })
}

/// Returns the quoted entries of the `members` array, which may span lines.
fn members() -> Vec<String> {
    let after_key = ROOT_MANIFEST
        .split_once("\nmembers")
        .map(|(_, rest)| rest)
        .expect("AC-1: the root manifest has a members key");
    let list = after_key
        .split_once('[')
        .and_then(|(_, rest)| rest.split_once(']'))
        .map(|(inside, _)| inside)
        .expect("AC-1: members is an array");
    list.split(',')
        .map(|entry| entry.trim().trim_matches('"').to_string())
        .filter(|entry| !entry.is_empty())
        .collect()
}

/// Returns the major and minor parts of a version such as `1.99.1`.
fn minor_of(version: &str) -> String {
    version
        .trim_matches('"')
        .split('.')
        .take(2)
        .collect::<Vec<_>>()
        .join(".")
}

/// Returns the first Markdown table after the line `heading`.
fn first_table_after<'a>(markdown: &'a str, heading: &str) -> Vec<&'a str> {
    markdown
        .lines()
        .skip_while(|line| *line != heading)
        .skip(1)
        .skip_while(|line| !line.starts_with('|'))
        .take_while(|line| line.starts_with('|'))
        .collect()
}

/// Covers AC-1 and AC-7: the root manifest is virtual (no package of its
/// own) and uses resolver 3.
#[test]
fn root_manifest_is_virtual_and_uses_resolver_3() {
    assert!(
        ROOT_MANIFEST.lines().all(|line| line.trim() != "[package]"),
        "AC-7: the root manifest has no [package] table"
    );
    assert_eq!(
        value(&table(ROOT_MANIFEST, "[workspace]"), "resolver"),
        Some("\"3\""),
        "AC-1: resolver = \"3\" in [workspace]"
    );
}

/// Covers AC-1: members are written one by one, without globs, each one
/// is a folder with its own manifest, and Core is always one of them.
#[test]
fn members_are_listed_one_by_one_without_globs() {
    let members = members();
    assert!(
        members.iter().any(|member| member == "crates/core"),
        "AC-1: crates/core is a member: {members:?}"
    );
    for member in &members {
        assert!(
            !member.contains(['*', '?', '[', ']']),
            "AC-1: member {member} is written out, not a glob"
        );
        assert!(
            repo_root().join(member).join("Cargo.toml").is_file(),
            "AC-1: member {member} has a Cargo.toml"
        );
    }
}

/// Covers AC-7: the workspace sets neither `default-members` nor `exclude`,
/// so `--workspace` and plain `cargo build` mean the same set.
#[test]
fn workspace_has_no_default_members_or_exclude() {
    let workspace = table(ROOT_MANIFEST, "[workspace]");
    for key in ["default-members", "exclude"] {
        assert_eq!(
            value(&workspace, key),
            None,
            "AC-7: no {key} in [workspace]"
        );
    }
}

/// Covers AC-7 and the security rule: `[workspace.package]` sets the four
/// shared fields, and packages stay unpublished.
#[test]
fn workspace_package_sets_shared_fields_and_stays_unpublished() {
    let package = table(ROOT_MANIFEST, "[workspace.package]");
    for field in SHARED_FIELDS {
        assert!(
            value(&package, field).is_some(),
            "AC-7: {field} in [workspace.package]: {package:?}"
        );
    }
    assert_eq!(
        value(&package, "publish"),
        Some("false"),
        "AC-7: publish = false in [workspace.package]"
    );
}

/// Covers AC-1 and the root manifest rules: every member is named
/// `desktop-` plus its role and inherits the four shared fields.
#[test]
fn every_member_inherits_shared_fields_and_uses_desktop_prefix() {
    for member in members() {
        let path = repo_root().join(&member).join("Cargo.toml");
        let manifest = fs::read_to_string(&path).expect("member manifest is readable");
        let package = table(&manifest, "[package]");
        let name = value(&package, "name").unwrap_or_default();
        assert!(
            name.starts_with("\"desktop-"),
            "AC-1: {member} is named desktop- plus its role: {name}"
        );
        for field in SHARED_FIELDS {
            let inherited = value(&package, &format!("{field}.workspace")) == Some("true")
                || value(&package, field).is_some_and(|v| v.replace(' ', "") == "{workspace=true}");
            assert!(inherited, "AC-1: {member} inherits {field}: {package:?}");
        }
    }
}

/// Covers AC-4: the toolchain pin and `rust-version` share one minor
/// version, and the pin brings `rustfmt` and `clippy`.
#[test]
fn toolchain_pin_matches_rust_version_and_brings_rustfmt_and_clippy() {
    let toolchain = table(TOOLCHAIN, "[toolchain]");
    let channel = value(&toolchain, "channel").expect("AC-4: channel in rust-toolchain.toml");
    let rust_version = value(&table(ROOT_MANIFEST, "[workspace.package]"), "rust-version")
        .expect("AC-4: rust-version in [workspace.package]");
    assert_eq!(
        minor_of(channel),
        minor_of(rust_version),
        "AC-4: channel {channel} and rust-version {rust_version} share one minor"
    );
    let components = value(&toolchain, "components").expect("AC-4: components in the pin");
    for component in ["\"rustfmt\"", "\"clippy\""] {
        assert!(
            components.contains(component),
            "AC-4: {component} in components = {components}"
        );
    }
}

/// Covers AC-5: build output is ignored, and the lockfile is not.
#[test]
fn gitignore_ignores_target_but_not_the_lockfile() {
    let lines: Vec<&str> = GITIGNORE.lines().map(str::trim).collect();
    assert!(lines.contains(&"/target"), "AC-5: /target in .gitignore");
    for pattern in ["Cargo.lock", "/Cargo.lock", "*.lock"] {
        assert!(
            !lines.contains(&pattern),
            "AC-4: .gitignore must not ignore the lockfile ({pattern})"
        );
    }
}

/// Covers AC-7 and the invariant on `target/`: the repo sets no Cargo
/// config, so it never sets `CARGO_TARGET_DIR` or other build overrides.
#[test]
fn repo_has_no_cargo_config() {
    for file in [".cargo/config.toml", ".cargo/config"] {
        assert!(
            !repo_root().join(file).exists(),
            "AC-7: no {file} in the repo"
        );
    }
}

/// Covers AC-4 and the invariant "one `Cargo.lock` for the whole
/// workspace": the root lockfile lists Core, and no other lockfile exists.
#[test]
fn one_cargo_lock_for_the_whole_workspace() {
    assert!(
        LOCKFILE.contains("\n[[package]]\nname = \"desktop-core\"\n"),
        "AC-4: the root Cargo.lock lists desktop-core"
    );
    let root = repo_root();
    let mut others = Vec::new();
    let mut pending = vec![root.clone()];
    while let Some(dir) = pending.pop() {
        for entry in fs::read_dir(&dir).expect("repo folder is readable") {
            let path = entry.expect("folder entry is readable").path();
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            if path.is_dir() && !path.is_symlink() {
                if !(dir == root && (name == "target" || name == ".git")) {
                    pending.push(path);
                }
            } else if name == "Cargo.lock" && dir != root {
                others.push(path);
            }
        }
    }
    assert!(others.is_empty(), "AC-4: extra lockfiles: {others:?}");
}

/// Covers AC-8: the rule table in the architecture doc equals the table in
/// the spec row by row, and the doc links to the spec.
#[test]
fn architecture_rule_table_matches_the_spec_row_by_row() {
    let in_doc = first_table_after(ARCHITECTURE, "### Aturan arah dependensi");
    let in_spec = first_table_after(SPEC, "## Aturan arah dependensi");
    assert!(
        in_spec
            .iter()
            .any(|row| row.starts_with("| `desktop-core` |")),
        "AC-8: the spec table has the desktop-core row"
    );
    assert_eq!(in_doc, in_spec, "AC-8: rule tables differ");
    assert!(
        ARCHITECTURE.contains("](../specs/_root/0001-batas-workspace-dan-pemilik/index.md)"),
        "AC-8: the architecture doc links to spec 0001"
    );
}
