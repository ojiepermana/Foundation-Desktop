//! Rules `A-5` to `A-11`: lints, the Core boundary, the greeting path, the
//! bridge, and the macOS app, read from the files in the repo as text.

use std::fs;
use std::path::Path;

use super::Findings;
use super::deps;

/// Folders the walk skips, at any depth.
const SKIPPED: [&str; 7] = [
    ".git",
    "target",
    ".build",
    "DerivedData",
    "xcuserdata",
    "graphify-out",
    ".claude",
];

/// The lint lines every member manifest sets under `[lints.rust]` (A-5).
const LINT_LINES: [&str; 2] = ["unsafe_code = \"forbid\"", "missing_docs = \"warn\""];

/// Formatter and linter config files the repo does not use (A-5).
const CONFIG_FILES: [&str; 4] = [
    "rustfmt.toml",
    ".rustfmt.toml",
    "clippy.toml",
    ".clippy.toml",
];

/// The greeting text that only `crates/core/src/greeting.rs` may hold
/// (A-8). Written in two parts so this file passes its own rule.
const GREETING: &str = concat!("Hello", ", ");

/// The only Core file that may hold [`GREETING`].
const GREETING_FILE: &str = "crates/core/src/greeting.rs";

/// The bridge source (A-9).
const BRIDGE: &str = "crates/macos-ffi/src/lib.rs";

/// The Core root module that re-exports the public API (A-9).
const CORE_LIB: &str = "crates/core/src/lib.rs";

/// The folder of the generated Swift binding (A-9).
const BINDING_DIR: &str = "bindings/swift/Sources/DesktopMacosFFI";

/// One file of the repo: its path from the root and its text, if UTF-8.
pub(super) struct File {
    /// Path from the repo root, with `/` as the separator.
    pub(super) path: String,
    /// The text of the file, or `None` when it is not UTF-8.
    pub(super) text: Option<String>,
}

impl File {
    /// The file name, the last path segment.
    pub(super) fn name(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or_default()
    }

    /// Whether a folder segment of the path is `tests` or `Tests`.
    fn is_test(&self) -> bool {
        let mut folders: Vec<&str> = self.path.split('/').collect();
        folders.pop();
        folders
            .iter()
            .any(|folder| *folder == "tests" || *folder == "Tests")
    }

    /// The part of the file that is not test code: for `.rs` files the text
    /// before the first line that trims to `#[cfg(test)]`.
    fn non_test(&self) -> Option<&str> {
        let text = self.text.as_deref()?;
        if !self.path.ends_with(".rs") {
            return Some(text);
        }
        let mut end = text.len();
        let mut offset = 0;
        for line in text.split_inclusive('\n') {
            if line.trim() == "#[cfg(test)]" {
                end = offset;
                break;
            }
            offset += line.len();
        }
        Some(&text[..end])
    }
}

/// Every file under `root`, sorted by path in byte order, skipping the
/// [`SKIPPED`] folders, `.git` files, and symlinks.
pub(super) fn walk(root: &Path) -> Vec<File> {
    let mut paths = Vec::new();
    let mut pending = vec![String::new()];
    while let Some(folder) = pending.pop() {
        let Ok(entries) = fs::read_dir(root.join(&folder)) else {
            continue;
        };
        for entry in entries.filter_map(Result::ok) {
            let name = entry.file_name().to_string_lossy().into_owned();
            if SKIPPED.contains(&name.as_str()) {
                continue;
            }
            let path = if folder.is_empty() {
                name
            } else {
                format!("{folder}/{name}")
            };
            match entry.file_type() {
                Ok(kind) if kind.is_dir() => pending.push(path),
                Ok(kind) if kind.is_file() => paths.push(path),
                _ => {}
            }
        }
    }
    paths.sort();
    paths
        .into_iter()
        .map(|path| {
            let text = fs::read(root.join(&path))
                .ok()
                .and_then(|bytes| String::from_utf8(bytes).ok());
            File { path, text }
        })
        .collect()
}

/// Runs `A-5` to `A-11`.
pub(super) fn check(root: &Path, files: &[File], findings: &mut Findings) {
    check_lints(root, files, findings);
    check_core(files, findings);
    check_config(files, findings);
    check_greeting(files, findings);
    check_bridge(files, findings);
    check_imports(files, findings);
    check_remote_packages(files, findings);
}

/// `A-5`: every member manifest sets the lint lines, and no formatter or
/// linter config file exists.
fn check_lints(root: &Path, files: &[File], findings: &mut Findings) {
    if let Some(members) = deps::members(root, findings) {
        for member in members {
            let path = format!("{}/Cargo.toml", member.dir);
            let Ok(text) = fs::read_to_string(root.join(&path)) else {
                continue;
            };
            let lints = super::manifest::table(&text, "[lints.rust]");
            for line in LINT_LINES {
                if !lints.contains(&line) {
                    findings.push(5, &path, 0, format!("A-5: [lints.rust] must set {line}"));
                }
            }
        }
    }
    for file in files {
        if CONFIG_FILES.contains(&file.name()) {
            findings.push(
                5,
                &file.path,
                0,
                "A-5: formatter and linter config files are not used".to_string(),
            );
        }
    }
}

/// Records one problem per line of `text` that holds a pattern, for the
/// first pattern in list order.
fn scan_lines(
    rule: u32,
    path: &str,
    text: &str,
    patterns: &[(&str, &str)],
    findings: &mut Findings,
) {
    for (number, line) in text.lines().enumerate() {
        if let Some((pattern, message)) =
            patterns.iter().find(|(pattern, _)| line.contains(pattern))
        {
            let message = message.replace("<teks>", pattern);
            findings.push(rule, path, number + 1, message);
        }
    }
}

/// `A-6`: Core sources do not branch on the OS, and Core files do not reach
/// outside `crates/core`.
fn check_core(files: &[File], findings: &mut Findings) {
    let os = "A-6: Core source must not branch on the OS (<teks>)";
    let outside = "A-6: Core files must not reach outside crates/core (<teks>)";
    let in_source = [
        ("target_os", os),
        ("target_family", os),
        ("cfg(windows)", os),
        ("cfg(unix)", os),
        ("../../", outside),
        ("CARGO_MANIFEST_DIR", outside),
    ];
    for file in files {
        if !file.path.starts_with("crates/core/") || !file.path.ends_with(".rs") {
            continue;
        }
        let Some(text) = &file.text else { continue };
        let patterns = if file.path.starts_with("crates/core/src/") {
            &in_source[..]
        } else {
            &in_source[4..]
        };
        scan_lines(6, &file.path, text, patterns, findings);
    }
}

/// `A-7`: Core and the bridge do not read configuration.
fn check_config(files: &[File], findings: &mut Findings) {
    let message = "A-7: Core and the bridge do not read configuration (<teks>)";
    let patterns = [("shared/config", message), ("public.json", message)];
    for file in files {
        let covered = ["crates/core/src/", "crates/macos-ffi/src/", "bindings/"]
            .iter()
            .any(|folder| file.path.starts_with(folder));
        if let (true, Some(text)) = (covered, &file.text) {
            scan_lines(7, &file.path, text, &patterns, findings);
        }
    }
}

/// `A-8`: the greeting text lives only in `crates/core/src/greeting.rs`.
fn check_greeting(files: &[File], findings: &mut Findings) {
    let message = "A-8: greeting text outside crates/core/src/greeting.rs";
    for file in files {
        let covered = ["crates/", "apps/", "bindings/", "tooling/"]
            .iter()
            .any(|folder| file.path.starts_with(folder));
        let source = file.path.ends_with(".rs") || file.path.ends_with(".swift");
        if !covered || !source || file.path == GREETING_FILE || file.is_test() {
            continue;
        }
        if let Some(text) = file.non_test() {
            scan_lines(8, &file.path, text, &[(GREETING, message)], findings);
        }
    }
}

/// Collapses every run of white space into one space and trims the ends.
fn normalize(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The Swift name of a Rust function: camel case with a lower case start.
fn swift_name(name: &str) -> String {
    let mut parts = name.split('_').filter(|part| !part.is_empty());
    let mut out = parts.next().unwrap_or_default().to_string();
    for part in parts {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
    }
    out
}

/// `A-9`: the bridge only forwards Core functions, and the binding has a
/// public func for each.
fn check_bridge(files: &[File], findings: &mut Findings) {
    let find = |path: &str| files.iter().find(|file| file.path == path);
    let Some(core_lib) = find(CORE_LIB).and_then(|file| file.text.as_deref()) else {
        return;
    };
    let Some(bridge) = find(BRIDGE).and_then(File::non_test) else {
        return;
    };

    let mut exports = Vec::new();
    for (number, line) in core_lib.lines().enumerate() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("pub use ") else {
            continue;
        };
        if rest.contains(['{', '*']) || rest.contains(" as ") {
            findings.push(
                9,
                CORE_LIB,
                number + 1,
                format!("A-9: cannot read re-export {trimmed}"),
            );
            continue;
        }
        let path = rest.trim_end_matches(';');
        exports.push(path.rsplit("::").next().unwrap_or(path).to_string());
    }

    let core_sources: Vec<String> = files
        .iter()
        .filter(|file| file.path.starts_with("crates/core/src/") && file.path.ends_with(".rs"))
        .filter_map(File::non_test)
        .map(normalize)
        .collect();
    let bindings: Vec<&File> = files
        .iter()
        .filter(|file| {
            file.path.starts_with(&format!("{BINDING_DIR}/")) && file.path.ends_with(".swift")
        })
        .collect();
    let binding_path = bindings
        .first()
        .map_or(BINDING_DIR, |file| file.path.as_str());

    let lines: Vec<&str> = bridge.lines().collect();
    for (index, line) in lines.iter().enumerate() {
        let Some(rest) = line.trim_start().strip_prefix("pub fn ") else {
            continue;
        };
        let Some(open) = rest.find('(') else { continue };
        let name = &rest[..open];
        let number = index + 1;

        if !exports.iter().any(|export| export == name) {
            findings.push(
                9,
                BRIDGE,
                number,
                format!("A-9: bridge exports {name}, which desktop-core does not export"),
            );
            continue;
        }

        // The signature runs from `pub fn` up to the opening `{`.
        let mut signature = String::new();
        let mut brace_line = None;
        for (offset, part) in lines[index..].iter().enumerate() {
            if let Some(at) = part.find('{') {
                signature.push_str(&part[..at]);
                brace_line = Some(index + offset);
                break;
            }
            signature.push_str(part);
            signature.push('\n');
        }
        let signature = normalize(&signature);
        let wanted = format!("{signature} {{");
        if brace_line.is_none() || !core_sources.iter().any(|text| text.contains(&wanted)) {
            findings.push(
                9,
                BRIDGE,
                number,
                format!("A-9: signature of {name} differs from desktop-core"),
            );
            continue;
        }

        // The body runs to the first line that is exactly `}`.
        let start = brace_line.map_or(index, |line| line + 1);
        let body: String = lines[start..]
            .iter()
            .take_while(|line| **line != "}")
            .map(|line| line.trim())
            .collect();
        let arguments: Vec<&str> = signature
            .split_once('(')
            .and_then(|(_, rest)| rest.rsplit_once(')'))
            .map(|(params, _)| params)
            .unwrap_or_default()
            .split(',')
            .filter_map(|param| param.split_once(':').map(|(name, _)| name.trim()))
            .collect();
        if body != format!("desktop_core::{name}({})", arguments.join(", ")) {
            findings.push(
                9,
                BRIDGE,
                number,
                format!("A-9: {name} must only forward to desktop_core::{name}"),
            );
            continue;
        }

        let swift = swift_name(name);
        let declared = bindings.iter().any(|file| {
            file.text.as_deref().is_some_and(|text| {
                text.lines()
                    .any(|line| line.starts_with(&format!("public func {swift}(")))
            })
        });
        if !declared {
            findings.push(
                9,
                binding_path,
                0,
                format!("A-9: binding has no public func {swift}, regenerate the binding"),
            );
        }
    }
}

/// `A-10`: only `apps/macos/Interop` imports the bridge module, and nothing
/// in the app imports the C module of the binding.
fn check_imports(files: &[File], findings: &mut Findings) {
    for file in files {
        if !file.path.starts_with("apps/macos/") || !file.path.ends_with(".swift") {
            continue;
        }
        let Some(text) = &file.text else { continue };
        for (number, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.ends_with("import DesktopMacosFFI")
                && !file.path.starts_with("apps/macos/Interop/")
            {
                findings.push(
                    10,
                    &file.path,
                    number + 1,
                    "A-10: only apps/macos/Interop imports DesktopMacosFFI".to_string(),
                );
            } else if trimmed.ends_with("import desktop_macos_ffiFFI") {
                findings.push(
                    10,
                    &file.path,
                    number + 1,
                    "A-10: desktop_macos_ffiFFI is imported only by the generated binding"
                        .to_string(),
                );
            }
        }
    }
}

/// `A-11`: no remote Swift package without a spec.
fn check_remote_packages(files: &[File], findings: &mut Findings) {
    let message = "A-11: remote Swift packages need a spec first";
    for file in files {
        let patterns: &[(&str, &str)] = if file.name() == "Package.swift" {
            &[(".package(url:", message), (".package(id:", message)]
        } else if file.path.ends_with(".pbxproj") {
            &[("XCRemoteSwiftPackageReference", message)]
        } else {
            continue;
        };
        if let Some(text) = &file.text {
            scan_lines(11, &file.path, text, patterns, findings);
        }
    }
}
