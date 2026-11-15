//! Rules `A-1` to `A-4`: workspace members and their dependency graph, read
//! from `Cargo.toml` and `Cargo.lock`.

use std::collections::{BTreeSet, VecDeque};
use std::fs;
use std::path::Path;

use super::manifest::{self, Package};
use super::{Findings, MEMBERS, MemberRule};

/// The root manifest, from the repo root.
const ROOT_MANIFEST: &str = "Cargo.toml";

/// The lockfile, from the repo root.
const LOCKFILE: &str = "Cargo.lock";

/// One workspace member: its folder and its package name.
pub(super) struct Member {
    pub(super) dir: String,
    pub(super) package: String,
}

/// Reads the workspace members and their package names. Members whose
/// manifest cannot be read are reported and left out.
pub(super) fn members(root: &Path, findings: &mut Findings) -> Option<Vec<Member>> {
    let text = read(root, ROOT_MANIFEST, findings)?;
    let mut found = Vec::new();
    for dir in manifest::members(&text) {
        let path = format!("{dir}/Cargo.toml");
        let Some(member) = read(root, &path, findings) else {
            continue;
        };
        let package = manifest::value(&manifest::table(&member, "[package]"), "name")
            .unwrap_or_default()
            .to_string();
        found.push(Member { dir, package });
    }
    Some(found)
}

/// Reads one file as text, or records `cannot read`.
pub(super) fn read(root: &Path, path: &str, findings: &mut Findings) -> Option<String> {
    match fs::read_to_string(root.join(path)) {
        Ok(text) => Some(text),
        Err(error) => {
            findings.push(0, path, 0, format!("cannot read: {error}"));
            None
        }
    }
}

/// Runs `A-1` to `A-4`.
pub(super) fn check(root: &Path, findings: &mut Findings) {
    let Some(members) = members(root, findings) else {
        return;
    };
    let Some(lockfile) = read(root, LOCKFILE, findings) else {
        return;
    };
    let packages = manifest::packages(&lockfile);
    let names: Vec<&str> = members
        .iter()
        .map(|member| member.package.as_str())
        .collect();

    // A-1: one rule per member, one member per rule, every member locked.
    for member in &members {
        if !MEMBERS.iter().any(|rule| rule.package == member.package) {
            findings.push(
                1,
                ROOT_MANIFEST,
                0,
                format!(
                    "A-1: member {} has no rule in architecture::MEMBERS",
                    member.package
                ),
            );
        }
    }
    for rule in MEMBERS {
        if !names.contains(&rule.package) {
            findings.push(
                1,
                ROOT_MANIFEST,
                0,
                format!("A-1: rule {} names no workspace member", rule.package),
            );
        }
    }
    for member in &members {
        if member_block(&packages, &member.package).is_none() {
            findings.push(
                1,
                LOCKFILE,
                0,
                format!(
                    "A-1: member {} is missing from the lockfile, run cargo build",
                    member.package
                ),
            );
        }
    }

    for member in &members {
        let Some(rule) = MEMBERS.iter().find(|rule| rule.package == member.package) else {
            continue;
        };
        let Some(block) = member_block(&packages, &member.package) else {
            continue;
        };
        check_member(member, rule, block, &packages, &names, findings);
    }
}

/// The lock block of a workspace member: its name and no `source`.
fn member_block(packages: &[Package], package: &str) -> Option<usize> {
    packages
        .iter()
        .position(|block| block.name == package && !block.has_source)
}

/// `A-2` to `A-4` for one member.
fn check_member(
    member: &Member,
    rule: &MemberRule,
    block: usize,
    packages: &[Package],
    names: &[&str],
    findings: &mut Findings,
) {
    let manifest = format!("{}/Cargo.toml", member.dir);
    let mut external = BTreeSet::new();
    for entry in &packages[block].dependencies {
        let name = entry.split(' ').next().unwrap_or_default();
        if names.contains(&name) {
            // A-2: edges between members.
            if !rule.members.contains(&name) {
                findings.push(
                    2,
                    LOCKFILE,
                    0,
                    format!("A-2: {} depends on workspace member {name}", rule.package),
                );
            }
        } else {
            external.insert(name);
        }
    }

    // A-3: the exact list of direct external dependencies.
    for name in &external {
        if !rule.external.contains(name) {
            findings.push(
                3,
                &manifest,
                0,
                format!(
                    "A-3: {} depends on {name}, which is not in its allowed list",
                    rule.package
                ),
            );
        }
    }
    for name in rule.external {
        if !external.contains(name) {
            findings.push(
                3,
                &manifest,
                0,
                format!(
                    "A-3: {} no longer depends on {name}, remove it from its allowed list",
                    rule.package
                ),
            );
        }
    }

    // A-4: no forbidden class in the transitive closure.
    let mut seen = BTreeSet::from([block]);
    let mut queue = VecDeque::from([block]);
    let mut reached = BTreeSet::new();
    while let Some(index) = queue.pop_front() {
        for entry in &packages[index].dependencies {
            match manifest::resolve(packages, entry) {
                Some(next) => {
                    if seen.insert(next) {
                        reached.insert(packages[next].name.as_str());
                        queue.push_back(next);
                    }
                }
                None => findings.push(
                    4,
                    LOCKFILE,
                    0,
                    format!("A-4: cannot resolve dependency entry {entry}"),
                ),
            }
        }
    }
    for name in reached {
        for class in rule.forbidden {
            if class.matches(name) {
                findings.push(
                    4,
                    LOCKFILE,
                    0,
                    format!(
                        "A-4: {} reaches {name} (class {})",
                        rule.package,
                        class.name()
                    ),
                );
            }
        }
    }
}
