//! Text checks on one SVG source, rules S-1 to S-7 of spec `_root/0005`.
//!
//! The checks read the file as text and never parse XML. They work on the
//! text after the declaration and comments are removed, including the
//! content of `title` and `desc`.

use super::{ALLOWED_ELEMENTS, APP_ICON_VIEW_BOX, MAX_SOURCE_BYTES, REQUIRED_ASSET};
use crate::Problem;

/// The namespace the `svg` element must declare.
const SVG_NAMESPACE: &str = r#"xmlns="http://www.w3.org/2000/svg""#;

/// The entities a source may use.
const ENTITIES: [&str; 5] = ["&amp;", "&lt;", "&gt;", "&quot;", "&apos;"];

/// Checks one source file for the asset `asset_id` (S-1 to S-7).
///
/// Every problem uses `path`. An empty list means the source is valid.
pub fn check_svg(path: &str, asset_id: &str, bytes: &[u8]) -> Vec<Problem> {
    let mut problems = Vec::new();
    let mut push = |message: String| problems.push(Problem::new(path, message));

    // S-1: size, UTF-8 without BOM, no carriage return.
    if bytes.len() > MAX_SOURCE_BYTES {
        push(format!(
            "S-1: the file has {} bytes, at most {MAX_SOURCE_BYTES} are allowed",
            bytes.len()
        ));
        return problems;
    }
    let Ok(text) = std::str::from_utf8(bytes) else {
        push("S-1: the file is not UTF-8 text".to_string());
        return problems;
    };
    if text.starts_with('\u{feff}') {
        push("S-1: the file starts with a byte order mark".to_string());
        return problems;
    }
    if text.contains('\r') {
        push("S-1: the file contains a carriage return, use LF line endings".to_string());
        return problems;
    }

    // S-2: declaration, comments, then no `<!` and no `<?`.
    let body = match strip_declaration_and_comments(text) {
        Ok(body) => body,
        Err(message) => {
            push(message.to_string());
            return problems;
        }
    };
    if body.contains("<!") {
        push("S-2: DOCTYPE, ENTITY, and CDATA are not allowed".to_string());
    }
    if body.contains("<?") {
        push("S-2: processing instructions are not allowed".to_string());
    }

    check_elements(&body, &mut push);
    check_root(&body, asset_id, &mut push);
    check_events(&body, &mut push);
    check_references(&body, &mut push);
    check_entities(&body, &mut push);
    problems
}

/// S-2: removes the `<?xml ...?>` declaration at byte 0, then every
/// comment, each up to its first `-->`.
fn strip_declaration_and_comments(text: &str) -> Result<String, &'static str> {
    let mut rest = text;
    if let Some(after) = rest.strip_prefix("<?xml")
        && after.starts_with(is_space)
    {
        let end = after
            .find("?>")
            .ok_or("S-2: the XML declaration is not closed")?;
        rest = &after[end + 2..];
    }
    let mut body = String::with_capacity(rest.len());
    while let Some(start) = rest.find("<!--") {
        body.push_str(&rest[..start]);
        let after = &rest[start + 4..];
        let end = after.find("-->").ok_or("S-2: a comment is not closed")?;
        rest = &after[end + 3..];
    }
    body.push_str(rest);
    Ok(body)
}

/// S-3: every `<` opens or closes an allowed element.
fn check_elements(body: &str, push: &mut impl FnMut(String)) {
    for (at, _) in body.match_indices('<') {
        let after = &body[at + 1..];
        if after.starts_with(['!', '?']) {
            continue;
        }
        let after = after.strip_prefix('/').unwrap_or(after);
        let name = letters(after);
        let next = &after[name.len()..];
        let ends_well =
            next.starts_with(is_space) || next.starts_with('>') || next.starts_with("/>");
        if !ALLOWED_ELEMENTS.contains(&name) || !ends_well {
            push(format!(
                "S-3: element at byte {at} is not one of the allowed elements"
            ));
        }
    }
}

/// S-4: the first element is `svg` with the namespace and a `viewBox`.
fn check_root(body: &str, asset_id: &str, push: &mut impl FnMut(String)) {
    let first = body
        .match_indices('<')
        .map(|(at, _)| &body[at + 1..])
        .find(|after| !after.starts_with(['/', '!', '?']));
    let Some(tag) = first else {
        push("S-4: the file has no svg element".to_string());
        return;
    };
    let name = letters(tag);
    if name != "svg" {
        push(format!("S-4: the first element is {name:?}, expected svg"));
        return;
    }
    let tag = tag.find('>').map_or(tag, |end| &tag[..end]);
    if !tag.contains(SVG_NAMESPACE) {
        push(format!("S-4: the svg element needs {SVG_NAMESPACE}"));
    }
    let view_box = tag
        .find("viewBox=\"")
        .map(|start| &tag[start + 9..])
        .and_then(|rest| rest.find('"').map(|end| &rest[..end]));
    match view_box {
        Some(value) if is_valid_view_box(value) => {
            if asset_id == REQUIRED_ASSET && value != APP_ICON_VIEW_BOX {
                push(format!(
                    "S-4: the {REQUIRED_ASSET} viewBox must be \"{APP_ICON_VIEW_BOX}\", found \"{value}\""
                ));
            }
        }
        _ => push("S-4: the svg element needs viewBox=\"0 0 <width> <height>\"".to_string()),
    }
}

/// Whether `value` is `0 0 <width> <height>` with positive whole numbers
/// without leading zeros.
fn is_valid_view_box(value: &str) -> bool {
    let positive = |number: &str| {
        !number.is_empty() && !number.starts_with('0') && number.chars().all(|c| c.is_ascii_digit())
    };
    let parts: Vec<&str> = value.split(' ').collect();
    matches!(parts.as_slice(), ["0", "0", width, height] if positive(width) && positive(height))
}

/// S-5: no event attribute such as `onload=`.
fn check_events(body: &str, push: &mut impl FnMut(String)) {
    let lower = body.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    for (at, _) in lower.match_indices("on") {
        let before_ok = at == 0 || {
            let before = bytes[at - 1];
            !(before.is_ascii_alphanumeric() || matches!(before, b'_' | b'.' | b':' | b'-'))
        };
        if !before_ok {
            continue;
        }
        let rest = &lower[at + 2..];
        let name = letters(rest);
        if name.is_empty() {
            continue;
        }
        if rest[name.len()..]
            .trim_start_matches(is_space)
            .starts_with('=')
        {
            push(format!("S-5: event attribute on{name} is not allowed"));
        }
    }
}

/// S-6: `href` and `url(` point at an id in the same file, and there is no
/// `xml:base`.
fn check_references(body: &str, push: &mut impl FnMut(String)) {
    let lower = body.to_ascii_lowercase();
    for (at, _) in lower.match_indices("href") {
        let rest = lower[at + 4..].trim_start_matches(is_space);
        let local = rest
            .strip_prefix('=')
            .map(|rest| rest.trim_start_matches(is_space))
            .and_then(|rest| rest.strip_prefix(['"', '\'']))
            .is_some_and(|rest| rest.starts_with('#'));
        if !local {
            push(format!(
                "S-6: href at byte {at} must point at an id in the same file"
            ));
        }
    }
    for (at, _) in lower.match_indices("url(") {
        let rest = lower[at + 4..].trim_start_matches(is_space);
        let rest = rest.strip_prefix(['"', '\'']).unwrap_or(rest);
        if !rest.starts_with('#') {
            push(format!(
                "S-6: url( at byte {at} must point at an id in the same file"
            ));
        }
    }
    if lower.contains("xml:base") {
        push("S-6: xml:base is not allowed".to_string());
    }
}

/// S-7: every `&` opens one of the five XML entities.
fn check_entities(body: &str, push: &mut impl FnMut(String)) {
    for (at, _) in body.match_indices('&') {
        if !ENTITIES.iter().any(|entity| body[at..].starts_with(entity)) {
            push(format!(
                "S-7: the entity at byte {at} is not one of &amp; &lt; &gt; &quot; &apos;"
            ));
        }
    }
}

/// The longest run of ASCII letters at the start of `text`.
fn letters(text: &str) -> &str {
    let end = text
        .find(|c: char| !c.is_ascii_alphabetic())
        .unwrap_or(text.len());
    &text[..end]
}

/// Whether `c` is white space in the sense of the spec: space, tab, or new
/// line.
fn is_space(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n')
}
