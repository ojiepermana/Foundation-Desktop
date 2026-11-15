//! Rules `A-12` and `A-13`: relative Markdown links and the references in
//! every `AGENTS.md`, read from the files in the repo as text.

use std::path::Path;

use super::Findings;
use super::code::File;
use super::deps;

/// Markdown under this folder is not checked by `A-12`.
const COORDINATION: &str = "docs/scope/coordination/";

/// Spec folders a reference such as `_root/0001` may name (A-13).
const SPEC_FOLDERS: [&str; 4] = ["_root", "macos", "windows", "linux"];

/// Path candidates that are named as bans, so they need not exist (A-13).
const BANNED_PATHS: [&str; 2] = [".cargo/config.toml", ".cargo/config"];

/// Runs `A-12` and `A-13`.
pub(super) fn check(root: &Path, files: &[File], findings: &mut Findings) {
    for file in files {
        if !file.path.ends_with(".md") || file.path.starts_with(COORDINATION) {
            continue;
        }
        if let Some(text) = &file.text {
            check_links(root, &file.path, text, findings);
        }
    }
    // Package names need every member manifest. When one cannot be read,
    // its name is unknown and the package part of A-13 is skipped.
    let packages: Option<Vec<String>> = deps::read_members(root, findings)
        .filter(|(_, complete)| *complete)
        .map(|(members, _)| members.into_iter().map(|member| member.package).collect());
    for file in files {
        if file.name() != "AGENTS.md" {
            continue;
        }
        if let Some(text) = &file.text {
            check_agents(root, &file.path, text, packages.as_deref(), findings);
        }
    }
}

/// The folder of a repo path, with a trailing `/`, or empty at the root.
fn folder_of(path: &str) -> &str {
    path.rfind('/').map_or("", |at| &path[..=at])
}

/// Whether a line opens or closes a fenced code block.
fn is_fence(line: &str) -> bool {
    line.trim().starts_with("```")
}

/// The line with the text between each pair of backticks removed.
fn without_code_spans(line: &str) -> String {
    let mut out = String::new();
    let mut rest = line;
    while let Some(open) = rest.find('`') {
        let after = &rest[open + 1..];
        let Some(close) = after.find('`') else {
            break;
        };
        out.push_str(&rest[..open]);
        rest = &after[close + 1..];
    }
    out.push_str(rest);
    out
}

/// Whether a link starts with a URI scheme such as `https:`.
fn has_scheme(link: &str) -> bool {
    let mut chars = link.chars();
    if !chars.next().is_some_and(|c| c.is_ascii_alphabetic()) {
        return false;
    }
    for c in chars {
        if c == ':' {
            return true;
        }
        if !(c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-')) {
            return false;
        }
    }
    false
}

/// Decodes `%XX` escapes. Invalid escapes are kept as they are.
fn percent_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%'
            && let Some(hex) = text.get(index + 1..index + 3)
            && let Ok(byte) = u8::from_str_radix(hex, 16)
        {
            out.push(byte);
            index += 3;
            continue;
        }
        out.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// `A-12`: every relative link points at a file or folder that exists.
fn check_links(root: &Path, path: &str, text: &str, findings: &mut Findings) {
    let folder = folder_of(path);
    let mut in_code = false;
    for (number, line) in text.lines().enumerate() {
        if is_fence(line) {
            in_code = !in_code;
            continue;
        }
        if in_code {
            continue;
        }
        let line = without_code_spans(line);
        let mut rest = line.as_str();
        while let Some(start) = rest.find("](") {
            let after = &rest[start + 2..];
            let Some(end) = after.find(')') else {
                break;
            };
            let link = &after[..end];
            rest = &after[end + 1..];
            if link.is_empty()
                || link.contains(' ')
                || link.starts_with(['#', '<', '/'])
                || has_scheme(link)
            {
                continue;
            }
            let target = percent_decode(link.split('#').next().unwrap_or(link));
            if target.is_empty() || !root.join(folder).join(&target).exists() {
                findings.push(
                    12,
                    path,
                    number + 1,
                    format!("A-12: broken link {link} on line {}", number + 1),
                );
            }
        }
    }
}

/// The spec references such as `` `_root/0001` `` in a line.
fn spec_references(line: &str) -> Vec<(&'static str, &str)> {
    let mut found = Vec::new();
    for folder in SPEC_FOLDERS {
        let opener = format!("`{folder}/");
        let mut rest = line;
        while let Some(at) = rest.find(&opener) {
            let after = &rest[at + opener.len()..];
            let digits = after
                .get(..4)
                .filter(|digits| digits.chars().all(|c| c.is_ascii_digit()));
            if let Some(digits) = digits
                && after[4..].starts_with('`')
            {
                found.push((folder, digits));
            }
            rest = after;
        }
    }
    found
}

/// The package names that follow ` -p ` in a line.
fn package_names(line: &str) -> Vec<&str> {
    let mut found = Vec::new();
    let mut rest = line;
    while let Some(at) = rest.find(" -p ") {
        let after = &rest[at + 4..];
        let end = after
            .find(|c: char| !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'))
            .unwrap_or(after.len());
        if end > 0 {
            found.push(&after[..end]);
        }
        rest = after;
    }
    found
}

/// The texts between pairs of backticks in a line.
fn code_spans(line: &str) -> Vec<&str> {
    let mut found = Vec::new();
    let mut rest = line;
    while let Some(open) = rest.find('`') {
        let after = &rest[open + 1..];
        let Some(close) = after.find('`') else {
            break;
        };
        found.push(&after[..close]);
        rest = &after[close + 1..];
    }
    found
}

/// Whether a candidate looks like a file path worth checking.
fn is_path_candidate(candidate: &str) -> bool {
    !candidate.is_empty()
        && candidate
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '/' | '-'))
        && candidate.contains('/')
        && !candidate.starts_with('/')
        && candidate
            .rsplit('/')
            .next()
            .is_some_and(|last| last.contains('.'))
        && !BANNED_PATHS.contains(&candidate)
}

/// `A-13`: spec references, package names, and paths in one `AGENTS.md`.
fn check_agents(
    root: &Path,
    path: &str,
    text: &str,
    packages: Option<&[String]>,
    findings: &mut Findings,
) {
    let folder = folder_of(path);
    let mut in_code = false;
    for (number, line) in text.lines().enumerate() {
        let number = number + 1;
        if is_fence(line) {
            in_code = !in_code;
            continue;
        }
        for (spec_folder, digits) in spec_references(line) {
            let prefix = format!("{digits}-");
            let exists = std::fs::read_dir(root.join("docs/specs").join(spec_folder))
                .map(|entries| {
                    entries
                        .filter_map(Result::ok)
                        .any(|entry| entry.file_name().to_string_lossy().starts_with(&prefix))
                })
                .unwrap_or(false);
            if !exists {
                findings.push(
                    13,
                    path,
                    number,
                    format!("A-13: spec {spec_folder}/{digits} does not exist"),
                );
            }
        }
        if let Some(packages) = packages {
            for name in package_names(line) {
                if !packages.iter().any(|package| package == name) {
                    findings.push(
                        13,
                        path,
                        number,
                        format!("A-13: package {name} is not a workspace member"),
                    );
                }
            }
        }
        let candidates: Vec<&str> = if in_code {
            line.split_whitespace().collect()
        } else {
            code_spans(line)
        };
        for candidate in candidates {
            if !is_path_candidate(candidate) {
                continue;
            }
            if !root.join(folder).join(candidate).exists() && !root.join(candidate).exists() {
                findings.push(
                    13,
                    path,
                    number,
                    format!("A-13: path {candidate} does not exist"),
                );
            }
        }
    }
}
