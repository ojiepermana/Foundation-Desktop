//! Rules `C-1` to `C-3`: the macOS text rules of spec `macos/0008`
//! (`docs/specs/macos/0008-aturan-pengembangan-dan-pemeriksaan-arsitektur/index.md`).
//!
//! The rules read the Swift files of the macOS app as text, with the same
//! walk, UTF-8 reading, and test file definition as the `A` rules. They
//! match their patterns without regular expressions: one problem per line,
//! for the first pattern in list order.

use std::path::Path;

use super::Findings;
use super::code::{File, walk};

/// Folders whose Swift files `C-1` and `C-2` check.
const VIEW_FOLDERS: [&str; 2] = ["apps/macos/Presentation/", "apps/macos/App/"];

/// The only folder outside the tests that may import `DesignSystem` (`C-3`).
const DESIGN_SYSTEM_USER: &str = "apps/macos/Presentation/";

/// Exact style patterns of `C-1`, after the four patterns with a number.
const STYLE_TEXTS: [&str; 12] = [
    "Color(",
    "Color.",
    "NSColor",
    "Font.",
    ".font(.",
    "fontWeight",
    ".bold(",
    ".foregroundStyle(.",
    ".foregroundColor",
    ".tint(.",
    ".background(.",
    "size:",
];

/// Exact control patterns of `C-2`, in order.
const CONTROL_TEXTS: [&str; 14] = [
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

/// Runs `C-1` to `C-3`.
pub(super) fn check(root: &Path, findings: &mut Findings) {
    let files = walk(root);
    for file in &files {
        if !file.path.ends_with(".swift") || !file.path.starts_with("apps/macos/") {
            continue;
        }
        let Some(text) = &file.text else { continue };
        let is_view = VIEW_FOLDERS
            .iter()
            .any(|folder| file.path.starts_with(folder));
        for (number, line) in text.lines().enumerate() {
            let number = number + 1;
            if is_view {
                if let Some(found) = style_pattern(line) {
                    findings.push_rule(
                        'C',
                        1,
                        &file.path,
                        number,
                        format!("C-1: style literal {found} outside the generated tokens"),
                    );
                }
                if let Some(found) = CONTROL_TEXTS.iter().find(|text| line.contains(*text)) {
                    findings.push_rule(
                        'C',
                        2,
                        &file.path,
                        number,
                        format!("C-2: native control override {found}"),
                    );
                }
            }
            check_import(file, line, number, findings);
        }
    }
}

/// `C-3`: an import of `DesignSystem` outside `Presentation` and the tests.
fn check_import(file: &File, line: &str, number: usize, findings: &mut Findings) {
    let imports = line.trim().ends_with("import DesignSystem");
    if imports && !file.path.starts_with(DESIGN_SYSTEM_USER) && !file.is_test() {
        findings.push_rule(
            'C',
            3,
            &file.path,
            number,
            "C-3: only apps/macos/Presentation imports DesignSystem".to_string(),
        );
    }
}

/// The first style pattern of `C-1` in `line`, as it appears in messages.
fn style_pattern(line: &str) -> Option<&'static str> {
    if followed_by(line, "spacing: ", starts_with_digit) {
        return Some("spacing:");
    }
    if followed_by(line, "padding(", starts_with_digit) {
        return Some("padding(");
    }
    if followed_by(line, "padding(.", edge_inset_then_number) {
        return Some("padding(.");
    }
    if followed_by(line, "minLength:", |rest| {
        starts_with_digit(rest.trim_start_matches(' '))
    }) {
        return Some("minLength:");
    }
    STYLE_TEXTS.iter().copied().find(|text| line.contains(text))
}

/// Whether some occurrence of `prefix` in `line` is followed by text that
/// `rest_matches` accepts.
fn followed_by(line: &str, prefix: &str, rest_matches: impl Fn(&str) -> bool) -> bool {
    line.match_indices(prefix)
        .any(|(at, _)| rest_matches(&line[at + prefix.len()..]))
}

/// Whether `text` starts with an ASCII digit.
fn starts_with_digit(text: &str) -> bool {
    text.starts_with(|c: char| c.is_ascii_digit())
}

/// One or more lower case ASCII letters, `,`, any spaces, then a digit,
/// as in `horizontal, 4`.
fn edge_inset_then_number(rest: &str) -> bool {
    let letters = rest
        .find(|c: char| !c.is_ascii_lowercase())
        .unwrap_or(rest.len());
    letters > 0
        && rest[letters..]
            .strip_prefix(',')
            .is_some_and(|after| starts_with_digit(after.trim_start_matches(' ')))
}
