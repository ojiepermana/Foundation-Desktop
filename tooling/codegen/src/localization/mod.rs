//! The `localization` source: the shared text catalog at
//! `shared/localization/catalog.json`.
//!
//! [`parse`] reads the catalog into closed Rust types, runs the check order
//! and the rules L-2 to L-10 of spec `_root/0006`
//! (`docs/specs/_root/0006-satu-contoh-lokalisasi/index.md`), and fills
//! every missing translation with the source text (L-9). The generated
//! `shared/localization/catalog.schema.json` is built from the same
//! constants, so the editor and the tool cannot disagree on the shape.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::json::UniqueMap;
use crate::{Output, Plan, Problem, Source};

mod schema;

/// The `localization` source, run as
/// `desktop-codegen localization <check|generate>`.
pub const SOURCE: Source = Source {
    name: "localization",
    plan,
};

/// The shape version the catalog must declare in `schemaVersion`.
pub const SCHEMA_VERSION: u64 = 1;

/// The catalog, from the repo root.
pub const CATALOG_PATH: &str = "shared/localization/catalog.json";

/// The generated schema, from the repo root.
pub const SCHEMA_PATH: &str = "shared/localization/catalog.schema.json";

/// The value `$schema` must have in the catalog.
pub(crate) const SCHEMA_REFERENCE: &str = "./catalog.schema.json";

/// Pattern every key must match (L-3).
pub(crate) const KEY_PATTERN: &str = r"^[a-z][a-z0-9]*(\.[a-z][a-z0-9]*){1,4}$";

/// Pattern every locale name must match (L-4).
pub(crate) const LOCALE_PATTERN: &str = "^[a-z]{2,3}(-[A-Z]{2})?$";

/// Pattern every description must match (T-4 of spec `_root/0003`).
pub(crate) const DESCRIPTION_PATTERN: &str = "^[A-Za-z0-9 ,.()-]{1,120}$";

/// The longest description, matching [`DESCRIPTION_PATTERN`].
const MAX_DESCRIPTION: usize = 120;

/// The fewest and most segments of a key, matching [`KEY_PATTERN`].
const KEY_SEGMENTS: std::ops::RangeInclusive<usize> = 2..=5;

/// Where the text of one locale came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Origin {
    /// The entry has its own text for the locale.
    Own,
    /// The text is missing and the source locale text stands in (L-9).
    Fallback,
}

/// The text of one entry in one locale, after fallback.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text {
    /// The locale name, for example `id`.
    pub locale: String,
    /// The text shown in that locale.
    pub value: String,
    /// Whether the text is the entry's own or a fallback.
    pub origin: Origin,
}

/// One key of the catalog with a text for every listed locale.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// The stable key, for example `greeting.submit`.
    pub key: String,
    /// The role of the text on screen, for translators.
    pub description: String,
    /// Placeholder names of the source text, each once, in order of first
    /// appearance.
    pub placeholders: Vec<String>,
    /// One text per locale, in the order of [`Catalog::locales`].
    pub texts: Vec<Text>,
}

/// One translation that is missing from the catalog (L-8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Missing {
    /// The key whose text is missing.
    pub key: String,
    /// The locale the text is missing for.
    pub locale: String,
}

/// The whole catalog, checked and filled with fallbacks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Catalog {
    /// The source locale, the default language of every app.
    pub source_locale: String,
    /// The supported locales, in file order.
    pub locales: Vec<String>,
    /// Every entry, sorted by key in byte order.
    pub entries: Vec<Entry>,
    /// Every missing translation, by key in byte order, then by locale
    /// order.
    pub missing: Vec<Missing>,
}

/// `catalog.json` as written.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CatalogFile {
    #[serde(rename = "$schema")]
    schema: String,
    #[serde(rename = "schemaVersion")]
    schema_version: u64,
    #[serde(rename = "sourceLocale")]
    source_locale: String,
    locales: Vec<String>,
    entries: UniqueMap<EntryFile>,
}

/// One entry of `entries` as written.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EntryFile {
    description: String,
    text: UniqueMap<String>,
}

/// Reads the catalog under `root` and plans the schema output.
fn plan(root: &Path) -> Result<Plan, Vec<Problem>> {
    let text = fs::read_to_string(root.join(CATALOG_PATH))
        .map_err(|error| vec![Problem::new(CATALOG_PATH, format!("cannot read: {error}"))])?;
    let catalog = parse(&text)?;
    Ok(Plan {
        outputs: vec![Output {
            path: SCHEMA_PATH.to_string(),
            contents: schema::contents(),
        }],
        summary: summary(&catalog),
    })
}

/// The summary of a catalog, for example
/// `1 keys, 3 locales, 2 missing translations (greeting.submit id; greeting.submit ms)`.
fn summary(catalog: &Catalog) -> String {
    let mut line = format!(
        "{} keys, {} locales, {} missing translations",
        catalog.entries.len(),
        catalog.locales.len(),
        catalog.missing.len()
    );
    if !catalog.missing.is_empty() {
        let pairs: Vec<String> = catalog
            .missing
            .iter()
            .map(|missing| format!("{} {}", missing.key, missing.locale))
            .collect();
        line.push_str(&format!(" ({})", pairs.join("; ")));
    }
    line
}

/// Parses and checks the text of `catalog.json`.
///
/// Runs the check order of spec `_root/0006`. Every problem uses
/// [`CATALOG_PATH`] as its path. Without problems, returns the catalog with
/// every missing translation filled from the source locale.
pub fn parse(catalog: &str) -> Result<Catalog, Vec<Problem>> {
    let mut out = Problems::default();

    // Step 1: the closed shape (L-2).
    let file: CatalogFile = match serde_json::from_str(catalog) {
        Ok(file) => file,
        Err(error) => {
            out.push(error.to_string());
            return Err(out.0);
        }
    };

    // Step 2: `$schema` and `schemaVersion` (L-10).
    if file.schema != SCHEMA_REFERENCE {
        out.push(format!(
            "$schema: expected {SCHEMA_REFERENCE}, found {}",
            file.schema
        ));
    }
    if file.schema_version != SCHEMA_VERSION {
        out.push(format!(
            "schemaVersion: expected {SCHEMA_VERSION}, found {}",
            file.schema_version
        ));
    }
    if !out.0.is_empty() {
        return Err(out.0);
    }

    // Step 3: locale names (L-4, L-5).
    check_locales(&file, &mut out);
    if !out.0.is_empty() {
        return Err(out.0);
    }

    // Step 4: every entry, by key in byte order.
    let mut entries = Vec::new();
    for (key, entry) in &file.entries.0 {
        if let Some(entry) = check_entry(&file, key, entry, &mut out) {
            entries.push(entry);
        }
    }
    if !out.0.is_empty() {
        return Err(out.0);
    }

    let missing = entries
        .iter()
        .flat_map(|entry| {
            entry
                .texts
                .iter()
                .filter(|text| text.origin == Origin::Fallback)
                .map(|text| Missing {
                    key: entry.key.clone(),
                    locale: text.locale.clone(),
                })
        })
        .collect();
    Ok(Catalog {
        source_locale: file.source_locale,
        locales: file.locales,
        entries,
        missing,
    })
}

/// The problems of the catalog, all under [`CATALOG_PATH`].
#[derive(Default)]
struct Problems(Vec<Problem>);

impl Problems {
    fn push(&mut self, message: String) {
        self.0.push(Problem::new(CATALOG_PATH, message));
    }
}

/// Step 3: each listed locale (pattern, then repetition), an empty list,
/// then `sourceLocale` (pattern, then membership).
fn check_locales(file: &CatalogFile, out: &mut Problems) {
    let mut seen: Vec<&str> = Vec::new();
    let mut repeated: Vec<&str> = Vec::new();
    for locale in &file.locales {
        if !is_valid_locale(locale) {
            out.push(format!(
                "locales.{locale}: locale must match {LOCALE_PATTERN}"
            ));
        } else if seen.contains(&locale.as_str()) {
            if !repeated.contains(&locale.as_str()) {
                repeated.push(locale);
                out.push(format!("locales: {locale} is listed twice"));
            }
        } else {
            seen.push(locale);
        }
    }
    if file.locales.is_empty() {
        out.push("locales: needs at least one locale".to_string());
    }
    let source = &file.source_locale;
    if !is_valid_locale(source) {
        out.push(format!("sourceLocale: locale must match {LOCALE_PATTERN}"));
    } else if !file.locales.contains(source) {
        out.push(format!("sourceLocale: {source} is not listed in locales"));
    }
}

/// Step 4 for one entry. Returns the resolved entry when it has no problem.
fn check_entry(
    file: &CatalogFile,
    key: &str,
    entry: &EntryFile,
    out: &mut Problems,
) -> Option<Entry> {
    let before = out.0.len();
    let place = format!("entries.{key}");

    // 4.1: the key pattern (L-3).
    if !is_valid_key(key) {
        out.push(format!("{place}: key must match {KEY_PATTERN}"));
        return None;
    }

    // 4.2: the description pattern.
    if !is_valid_description(&entry.description) {
        out.push(format!(
            "{place}.description: must match {DESCRIPTION_PATTERN}"
        ));
    }

    // 4.3: texts for locales that are not listed, by locale in byte order.
    let texts = &entry.text.0;
    for locale in texts.keys() {
        if !file.locales.contains(locale) {
            out.push(format!(
                "{place}.text.{locale}: locale is not listed in locales"
            ));
        }
    }

    // 4.4: the source text must exist (L-5).
    let source = file.source_locale.as_str();
    let Some(source_text) = texts.get(source) else {
        out.push(format!("{place}.text: missing source text for {source}"));
        return None;
    };

    // 4.5: each listed text, in locale order: empty, control, or braces.
    let mut placeholders: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for locale in &file.locales {
        let Some(value) = texts.get(locale) else {
            continue;
        };
        match text_problem(value) {
            Ok(names) => {
                placeholders.insert(locale, names);
            }
            Err(reason) => out.push(format!("{place}.text.{locale}: {reason}")),
        }
    }

    // 4.6: each translation has the same placeholder set as the source.
    if let Some(expected) = placeholders.get(source) {
        let expected_set = name_list(expected);
        for locale in &file.locales {
            if locale == source {
                continue;
            }
            let Some(found) = placeholders.get(locale.as_str()) else {
                continue;
            };
            let found_set = name_list(found);
            if found_set != expected_set {
                out.push(format!(
                    "{place}.text.{locale}: placeholders differ from {source}: expected {expected_set}, found {found_set}"
                ));
            }
        }
    }

    if out.0.len() != before {
        return None;
    }
    let source_placeholders = placeholders.get(source).cloned().unwrap_or_default();
    let resolved = file
        .locales
        .iter()
        .map(|locale| match texts.get(locale) {
            Some(value) => Text {
                locale: locale.clone(),
                value: value.clone(),
                origin: Origin::Own,
            },
            None => Text {
                locale: locale.clone(),
                value: source_text.clone(),
                origin: Origin::Fallback,
            },
        })
        .collect();
    Some(Entry {
        key: key.to_string(),
        description: entry.description.clone(),
        placeholders: source_placeholders,
        texts: resolved,
    })
}

/// Checks one text (L-6, L-7). Returns its placeholder names, each once, in
/// order of first appearance, or the reason it is rejected.
fn text_problem(value: &str) -> Result<Vec<String>, &'static str> {
    if value.chars().all(|c| c == ' ') {
        return Err("text is empty");
    }
    if value.chars().any(|c| c <= '\u{1f}' || c == '\u{7f}') {
        return Err("text contains a control character");
    }
    placeholder_names(value).ok_or("braces must form {name} placeholders")
}

/// The placeholder names in `value`, each once, in order of first
/// appearance, or `None` when a brace does not form a `{name}` placeholder.
fn placeholder_names(value: &str) -> Option<Vec<String>> {
    let mut names: Vec<String> = Vec::new();
    let mut rest = value;
    while let Some(at) = rest.find(['{', '}']) {
        if rest[at..].starts_with('}') {
            return None;
        }
        let after = &rest[at + 1..];
        let end = after.find('}')?;
        let name = &after[..end];
        if !is_valid_placeholder(name) {
            return None;
        }
        if !names.iter().any(|seen| seen == name) {
            names.push(name.to_string());
        }
        rest = &after[end + 1..];
    }
    Some(names)
}

/// The names sorted in byte order and joined with `, `, or `none`.
fn name_list(names: &[String]) -> String {
    let mut sorted: Vec<&str> = names.iter().map(String::as_str).collect();
    sorted.sort_unstable();
    if sorted.is_empty() {
        "none".to_string()
    } else {
        sorted.join(", ")
    }
}

/// Whether `name` matches `^[a-z][a-z0-9_]*$`.
fn is_valid_placeholder(name: &str) -> bool {
    let mut chars = name.chars();
    chars.next().is_some_and(|c| c.is_ascii_lowercase())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// Whether `key` matches [`KEY_PATTERN`].
fn is_valid_key(key: &str) -> bool {
    let segments: Vec<&str> = key.split('.').collect();
    KEY_SEGMENTS.contains(&segments.len())
        && segments.iter().all(|segment| {
            let mut chars = segment.chars();
            chars.next().is_some_and(|c| c.is_ascii_lowercase())
                && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        })
}

/// Whether `locale` matches [`LOCALE_PATTERN`].
fn is_valid_locale(locale: &str) -> bool {
    let (language, region) = match locale.split_once('-') {
        Some((language, region)) => (language, Some(region)),
        None => (locale, None),
    };
    (2..=3).contains(&language.len())
        && language.chars().all(|c| c.is_ascii_lowercase())
        && region.is_none_or(|region| {
            region.len() == 2 && region.chars().all(|c| c.is_ascii_uppercase())
        })
}

/// Whether `description` matches [`DESCRIPTION_PATTERN`].
fn is_valid_description(description: &str) -> bool {
    let length = description.chars().count();
    (1..=MAX_DESCRIPTION).contains(&length)
        && description
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || " ,.()-".contains(c))
}
