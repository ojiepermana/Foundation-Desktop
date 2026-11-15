//! The `architecture` source: the machine checked development rules of
//! spec `_root/0007`
//! (`docs/specs/_root/0007-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md`).
//!
//! The source reads the repo as text and has no outputs, so
//! `architecture generate` never writes a file. It never runs `cargo`,
//! `git`, or any other process, and never uses the network. Every problem
//! starts with the ID of the rule it breaks.

use std::path::Path;

use crate::{Plan, Problem, Source};

mod code;
mod deps;
mod docs;
mod manifest;

/// The `architecture` source, run as
/// `desktop-codegen architecture <check|generate>`.
pub const SOURCE: Source = Source {
    name: "architecture",
    plan,
};

/// The IDs of the rules this source checks, in order.
pub const RULES: &[&str] = &[
    "A-1", "A-2", "A-3", "A-4", "A-5", "A-6", "A-7", "A-8", "A-9", "A-10", "A-11", "A-12", "A-13",
];

/// A class of packages a workspace member may be forbidden to reach.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    /// Binding generators.
    Binding,
    /// UI libraries.
    Ui,
    /// OS specific SDKs.
    OsSdk,
}

impl Class {
    /// The name used in problem messages: `binding`, `ui`, or `os-sdk`.
    pub fn name(self) -> &'static str {
        match self {
            Class::Binding => "binding",
            Class::Ui => "ui",
            Class::OsSdk => "os-sdk",
        }
    }

    /// The package name patterns of the class.
    pub fn patterns(self) -> &'static [&'static str] {
        match self {
            Class::Binding => &["uniffi"],
            Class::Ui => &[
                "gtk",
                "gdk",
                "gsk",
                "libadwaita",
                "glib",
                "gio",
                "gobject",
                "pango",
                "cairo",
                "graphene",
                "windows-reactor",
            ],
            Class::OsSdk => &[
                "windows",
                "winapi",
                "objc",
                "objc2",
                "cocoa",
                "core-foundation",
            ],
        }
    }

    /// Whether `package` is in the class: it equals a pattern, or starts
    /// with one followed right away by `-`, `_`, or a digit.
    pub fn matches(self, package: &str) -> bool {
        self.patterns().iter().any(|pattern| {
            package.strip_prefix(pattern).is_some_and(|rest| {
                rest.is_empty()
                    || rest.starts_with(['-', '_'])
                    || rest.starts_with(|c: char| c.is_ascii_digit())
            })
        })
    }
}

/// The dependency rule of one workspace member.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemberRule {
    /// The package name of the member.
    pub package: &'static str,
    /// The workspace members it may depend on directly.
    pub members: &'static [&'static str],
    /// The exact list of its direct dependencies outside the workspace.
    pub external: &'static [&'static str],
    /// The classes no package in its transitive closure may belong to.
    pub forbidden: &'static [Class],
}

/// One rule per workspace member, written from the table in spec
/// `_root/0007`. The spec that owns a member owns its row.
pub const MEMBERS: &[MemberRule] = &[
    MemberRule {
        package: "desktop-core",
        members: &[],
        external: &[],
        forbidden: &[Class::Binding, Class::Ui, Class::OsSdk],
    },
    MemberRule {
        package: "desktop-macos-ffi",
        members: &["desktop-core"],
        external: &["uniffi"],
        forbidden: &[Class::Ui],
    },
    MemberRule {
        package: "desktop-codegen",
        members: &[],
        external: &["serde", "serde_json"],
        forbidden: &[Class::Ui, Class::OsSdk],
    },
];

/// One problem with the keys it is sorted by.
struct Found {
    rule: u32,
    path: String,
    line: usize,
    message: String,
}

/// Collects problems and sorts them by rule number, path, and line.
#[derive(Default)]
struct Findings(Vec<Found>);

impl Findings {
    /// Records a problem of rule `rule` (`0` for read errors) at `path`.
    fn push(&mut self, rule: u32, path: &str, line: usize, message: String) {
        self.0.push(Found {
            rule,
            path: path.to_string(),
            line,
            message,
        });
    }

    /// The problems in the order of spec `_root/0007`.
    fn into_problems(mut self) -> Vec<Problem> {
        self.0.sort_by(|a, b| {
            (a.rule, a.path.as_str(), a.line).cmp(&(b.rule, b.path.as_str(), b.line))
        });
        self.0
            .dedup_by(|a, b| a.path == b.path && a.message == b.message);
        self.0
            .into_iter()
            .map(|found| Problem::new(&found.path, found.message))
            .collect()
    }
}

/// Runs every rule on the repo under `root`.
fn plan(root: &Path) -> Result<Plan, Vec<Problem>> {
    let mut findings = Findings::default();
    deps::check(root, &mut findings);
    let files = code::walk(root);
    code::check(root, &files, &mut findings);
    docs::check(root, &files, &mut findings);
    if !findings.0.is_empty() {
        return Err(findings.into_problems());
    }
    Ok(Plan {
        outputs: Vec::new(),
        summary: format!("{} rules, {} members", RULES.len(), MEMBERS.len()),
    })
}
