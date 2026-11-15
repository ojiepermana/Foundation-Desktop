//! Validates the shared sources of the desktop foundation and regenerates
//! the files derived from them.
//!
//! Each shared source (the first one is `tokens`) is a [`Source`]: a name
//! plus a `plan` function that reads the hand written files under the repo
//! root, checks them, and returns every derived file as an [`Output`].
//! [`check`] compares those outputs with the files on disk byte for byte,
//! and [`generate`] writes the ones that are missing or differ. The rules
//! live in `docs/specs/_root/0003-tema-bersama-dan-kontrol-native/index.md`.

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

pub mod architecture;
pub mod assets;
pub mod config;
pub mod json;
pub mod localization;
pub mod tokens;

/// One file the generator writes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output {
    /// Path from the repo root, with `/` as the separator.
    pub path: String,
    /// The full contents of the file.
    pub contents: String,
}

/// One thing wrong with a source or with an output on disk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    /// Path from the repo root of the file the problem is about.
    pub path: String,
    /// What is wrong, without the path.
    pub message: String,
}

impl Problem {
    /// Creates a problem about the file at `path` (from the repo root).
    pub fn new(path: &str, message: impl Into<String>) -> Self {
        Self {
            path: path.to_string(),
            message: message.into(),
        }
    }
}

impl fmt::Display for Problem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path, self.message)
    }
}

/// Everything one source produces once its files are valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    /// Every derived file of the source, in a fixed order.
    pub outputs: Vec<Output>,
    /// Short description of what was read, used in the success line.
    pub summary: String,
}

/// A shared source the tool knows how to check and generate.
pub struct Source {
    /// The name used on the command line, for example `tokens`.
    pub name: &'static str,
    /// Reads the source under the given repo root and plans its outputs.
    pub plan: fn(&Path) -> Result<Plan, Vec<Problem>>,
}

/// Every source the tool knows, in the order the usage line lists them.
pub const SOURCES: &[Source] = &[
    tokens::SOURCE,
    config::SOURCE,
    localization::SOURCE,
    assets::SOURCE,
    architecture::SOURCE,
];

/// The repo root, fixed when this crate is compiled.
pub fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// Finds the source with the given command line name.
pub fn source(name: &str) -> Option<&'static Source> {
    SOURCES.iter().find(|source| source.name == name)
}

/// Checks the source and compares every output with the file on disk.
///
/// Returns the full success line, for example
/// `tokens: ok, 6 semantic tokens, 3 placeholder values, 4 outputs up to date`.
/// Never writes.
pub fn check(root: &Path, source: &Source) -> Result<String, Vec<Problem>> {
    let plan = (source.plan)(root)?;
    let problems: Vec<Problem> = plan
        .outputs
        .iter()
        .filter_map(|output| match fs::read_to_string(root.join(&output.path)) {
            Ok(contents) if contents == output.contents => None,
            Ok(_) => Some(Problem::new(
                &output.path,
                format!(
                    "differs from the generator output, run: {}",
                    generate_command(source)
                ),
            )),
            Err(_) => Some(Problem::new(
                &output.path,
                format!("missing, run: {}", generate_command(source)),
            )),
        })
        .collect();
    if problems.is_empty() {
        Ok(format!(
            "{}: ok, {}, {} outputs up to date",
            source.name,
            plan.summary,
            plan.outputs.len()
        ))
    } else {
        Err(problems)
    }
}

/// Checks the source, then writes every output that is missing or differs.
///
/// Returns the paths written and the full success line. Nothing is written
/// when the source is not valid.
pub fn generate(root: &Path, source: &Source) -> Result<(Vec<String>, String), Vec<Problem>> {
    let plan = (source.plan)(root)?;
    let mut written = Vec::new();
    let mut problems = Vec::new();
    for output in &plan.outputs {
        let target = root.join(&output.path);
        let current = fs::read_to_string(&target).ok();
        if current.as_deref() == Some(output.contents.as_str()) {
            continue;
        }
        match write_file(&target, &output.contents) {
            Ok(()) => written.push(output.path.clone()),
            Err(error) => {
                problems.push(Problem::new(&output.path, format!("cannot write: {error}")))
            }
        }
    }
    if !problems.is_empty() {
        return Err(problems);
    }
    let line = format!(
        "{}: ok, {}, {} outputs, {} written",
        source.name,
        plan.summary,
        plan.outputs.len(),
        written.len()
    );
    Ok((written, line))
}

/// The command that repairs the outputs of `source`.
fn generate_command(source: &Source) -> String {
    format!(
        "cargo run -p desktop-codegen --locked -- {} generate",
        source.name
    )
}

/// Writes `contents` to `path`, creating the parent folders first.
fn write_file(path: &Path, contents: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents)
}
