//! The `tokens` source: color, typography, and spacing tokens under
//! `design-system/tokens/`.
//!
//! [`parse`] reads the three files into closed Rust types and checks the
//! rules T-1 to T-9 of spec `_root/0003`. The JSON schemas under
//! `design-system/schemas/` are built from the same constants, so the
//! editor and the tool cannot disagree on the shape.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde::Deserialize;
use serde::de::DeserializeOwned;

use crate::json::UniqueMap;
use crate::{Plan, Problem, Source};

mod schema;
pub mod swift;

/// The `tokens` source, run as `desktop-codegen tokens <check|generate>`.
pub const SOURCE: Source = Source {
    name: "tokens",
    plan,
};

/// The shape version every token file must declare in `schemaVersion`.
pub const SCHEMA_VERSION: u64 = 1;

/// Color tokens, from the repo root.
pub const COLORS_PATH: &str = "design-system/tokens/colors.json";

/// Typography tokens, from the repo root.
pub const TYPOGRAPHY_PATH: &str = "design-system/tokens/typography.json";

/// Spacing tokens, from the repo root.
pub const SPACING_PATH: &str = "design-system/tokens/spacing.json";

/// The largest spacing value, in logical units.
pub const MAX_SPACE: u64 = 128;

/// The lowest contrast allowed between a color and the color it sits on
/// (the WCAG level AA threshold for normal text).
pub const MIN_CONTRAST: f64 = 4.5;

/// Pattern every token name must match.
pub(crate) const NAME_PATTERN: &str = r"^[a-z][a-z0-9]*(\.[a-z0-9]+)*$";

/// Pattern every base color value must match.
pub(crate) const COLOR_PATTERN: &str = "^#[0-9A-F]{6}$";

/// Pattern every description must match.
pub(crate) const DESCRIPTION_PATTERN: &str = "^[A-Za-z0-9 ,.()-]{1,120}$";

/// The longest description, matching [`DESCRIPTION_PATTERN`].
const MAX_DESCRIPTION: usize = 120;

/// An sRGB color with 8 bits per channel and no alpha.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb(pub u8, pub u8, pub u8);

/// A semantic color resolved to its light and dark values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Color {
    /// The semantic name, for example `text.primary`.
    pub name: String,
    /// What the color is used for on screen.
    pub description: String,
    /// The value in the light appearance.
    pub light: Rgb,
    /// The value in the dark appearance.
    pub dark: Rgb,
    /// `true` when either base value is still a placeholder.
    pub placeholder: bool,
}

/// A font weight from the closed set the tokens allow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Weight {
    /// The regular weight of the system font.
    Regular,
    /// The semibold weight of the system font.
    Semibold,
}

impl Weight {
    /// Every weight, in the order the schema lists them.
    pub(crate) const ALL: [Weight; 2] = [Weight::Regular, Weight::Semibold];

    /// The name used in the token file.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Weight::Regular => "regular",
            Weight::Semibold => "semibold",
        }
    }
}

/// A typography role from the closed set every OS maps to a native style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoleName {
    /// Default reading text.
    Body,
}

impl RoleName {
    /// Every role, in the order the schema lists them.
    pub const ALL: [RoleName; 1] = [RoleName::Body];

    /// The name used in the token file.
    pub fn as_str(self) -> &'static str {
        match self {
            RoleName::Body => "body",
        }
    }
}

/// A typography role with its weight.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Role {
    /// Which role this is.
    pub name: RoleName,
    /// What the role is used for.
    pub description: String,
    /// The font weight of the role.
    pub weight: Weight,
}

/// A semantic spacing value resolved to its number.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Space {
    /// The semantic name, for example `gap.control`.
    pub name: String,
    /// Where the spacing is used.
    pub description: String,
    /// The value in logical units.
    pub value: u32,
}

/// Every semantic token, resolved and sorted by name in byte order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenSet {
    /// Semantic colors.
    pub colors: Vec<Color>,
    /// Typography roles.
    pub roles: Vec<Role>,
    /// Semantic spacing values.
    pub spacing: Vec<Space>,
    /// How many base colors are still marked as placeholders.
    pub placeholders: usize,
}

/// `colors.json` as written.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ColorsFile {
    #[serde(rename = "$schema")]
    schema: String,
    #[serde(rename = "schemaVersion")]
    schema_version: u64,
    base: UniqueMap<BaseColor>,
    semantic: UniqueMap<SemanticColor>,
}

/// One entry of `base` in `colors.json`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BaseColor {
    value: String,
    #[serde(default)]
    placeholder: bool,
}

/// One entry of `semantic` in `colors.json`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SemanticColor {
    description: String,
    light: String,
    dark: String,
    #[serde(default)]
    on: Option<String>,
}

/// `typography.json` as written.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TypographyFile {
    #[serde(rename = "$schema")]
    schema: String,
    #[serde(rename = "schemaVersion")]
    schema_version: u64,
    roles: UniqueMap<RoleEntry>,
}

/// One entry of `roles` in `typography.json`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RoleEntry {
    description: String,
    weight: Weight,
}

/// `spacing.json` as written.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SpacingFile {
    #[serde(rename = "$schema")]
    schema: String,
    #[serde(rename = "schemaVersion")]
    schema_version: u64,
    base: UniqueMap<BaseSpace>,
    semantic: UniqueMap<SemanticSpace>,
}

/// One entry of `base` in `spacing.json`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BaseSpace {
    value: u64,
}

/// One entry of `semantic` in `spacing.json`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SemanticSpace {
    description: String,
    #[serde(rename = "ref")]
    reference: String,
}

/// Reads the three token files under `root` and plans the schema outputs
/// and the Swift tokens of spec `macos/0005`.
fn plan(root: &Path) -> Result<Plan, Vec<Problem>> {
    let mut problems = Vec::new();
    let mut read = |path: &str| match fs::read_to_string(root.join(path)) {
        Ok(text) => text,
        Err(error) => {
            problems.push(Problem::new(path, format!("cannot read: {error}")));
            String::new()
        }
    };
    let colors = read(COLORS_PATH);
    let typography = read(TYPOGRAPHY_PATH);
    let spacing = read(SPACING_PATH);
    if !problems.is_empty() {
        return Err(problems);
    }
    let tokens = parse(&colors, &typography, &spacing)?;
    let summary = format!(
        "{} semantic tokens, {} placeholder values",
        tokens.colors.len() + tokens.roles.len() + tokens.spacing.len(),
        tokens.placeholders
    );
    let mut outputs = schema::outputs();
    outputs.push(swift::output(&tokens));
    Ok(Plan { outputs, summary })
}

/// Parses and checks the text of the three token files.
///
/// Collects every problem, ordered by file (colors, typography, spacing).
/// A file that does not fit its type gives exactly one problem, the
/// `serde_json` message, and the other rules are skipped for that file.
pub fn parse(colors: &str, typography: &str, spacing: &str) -> Result<TokenSet, Vec<Problem>> {
    let mut problems = Vec::new();
    let colors = read_file::<ColorsFile>(COLORS_PATH, colors, &mut problems)
        .and_then(|file| check_colors(file, &mut problems));
    let roles = read_file::<TypographyFile>(TYPOGRAPHY_PATH, typography, &mut problems)
        .and_then(|file| check_typography(file, &mut problems));
    let spacing = read_file::<SpacingFile>(SPACING_PATH, spacing, &mut problems)
        .and_then(|file| check_spacing(file, &mut problems));
    match (colors, roles, spacing) {
        (Some((colors, placeholders)), Some(roles), Some(spacing)) if problems.is_empty() => {
            Ok(TokenSet {
                colors,
                roles,
                spacing,
                placeholders,
            })
        }
        _ => Err(problems),
    }
}

/// Reads `text` into `T`, or records the `serde_json` message (T-1).
fn read_file<T: DeserializeOwned>(
    path: &str,
    text: &str,
    problems: &mut Vec<Problem>,
) -> Option<T> {
    match serde_json::from_str(text) {
        Ok(file) => Some(file),
        Err(error) => {
            problems.push(Problem::new(path, error.to_string()));
            None
        }
    }
}

/// Collects the problems of one file, all under the same path.
struct FileProblems<'a> {
    path: &'static str,
    problems: &'a mut Vec<Problem>,
    found: bool,
}

impl<'a> FileProblems<'a> {
    fn new(path: &'static str, problems: &'a mut Vec<Problem>) -> Self {
        Self {
            path,
            problems,
            found: false,
        }
    }

    fn push(&mut self, message: String) {
        self.found = true;
        self.problems.push(Problem::new(self.path, message));
    }

    /// T-2: the `$schema` and `schemaVersion` header.
    fn header(&mut self, schema: &str, version: u64, schema_file: &str) {
        let expected = format!("../schemas/{schema_file}");
        if schema != expected {
            self.push(format!("$schema: expected {expected}, found {schema}"));
        }
        if version != SCHEMA_VERSION {
            self.push(format!(
                "schemaVersion: expected {SCHEMA_VERSION}, found {version}"
            ));
        }
    }

    /// T-3: every name matches the pattern, and no two collide once the
    /// dots are removed.
    fn names<'n>(&mut self, group: &str, names: impl Iterator<Item = &'n String>) {
        let mut seen: BTreeMap<String, &str> = BTreeMap::new();
        for name in names {
            if !is_valid_name(name) {
                self.push(format!("{group}.{name}: name must match {NAME_PATTERN}"));
            }
            let stripped = name.replace('.', "");
            match seen.get(&stripped) {
                Some(first) => self.push(format!(
                    "{group}: names {first} and {name} collide once the dots are removed"
                )),
                None => {
                    seen.insert(stripped, name);
                }
            }
        }
    }

    /// T-4: a description uses safe characters and fits the length.
    fn description(&mut self, place: &str, description: &str) {
        if !is_valid_description(description) {
            self.push(format!(
                "{place}.description: must match {DESCRIPTION_PATTERN}"
            ));
        }
    }

    /// T-6: a semantic token points at a base token of the same file.
    fn reference<T>(&mut self, base: &BTreeMap<String, T>, name: &str, key: &str, target: &str) {
        if !base.contains_key(target) {
            self.push(format!(
                "semantic.{name}.{key}: unknown base token {target}"
            ));
        }
    }

    /// T-7: every base token is used by at least one semantic token.
    fn unused<'n>(&mut self, base: impl Iterator<Item = &'n String>, used: &BTreeSet<&str>) {
        for name in base {
            if !used.contains(name.as_str()) {
                self.push(format!("base.{name}: not used by any semantic token"));
            }
        }
    }

    /// T-9: a group has at least one token.
    fn not_empty(&mut self, empty: bool) {
        if empty {
            self.push("semantic: needs at least one token".to_string());
        }
    }
}

/// Checks `colors.json` and resolves its semantic colors.
fn check_colors(file: ColorsFile, problems: &mut Vec<Problem>) -> Option<(Vec<Color>, usize)> {
    let mut out = FileProblems::new(COLORS_PATH, problems);
    out.header(&file.schema, file.schema_version, "colors.schema.json");
    let base = &file.base.0;
    let semantic = &file.semantic.0;
    out.names("base", base.keys());
    out.names("semantic", semantic.keys());

    let mut values: BTreeMap<&str, Rgb> = BTreeMap::new();
    for (name, entry) in base {
        match parse_rgb(&entry.value) {
            Some(rgb) => {
                values.insert(name, rgb);
            }
            None => out.push(format!(
                "base.{name}.value: expected #RRGGBB with uppercase hex digits, found {}",
                entry.value
            )),
        }
    }

    let mut used = BTreeSet::new();
    for (name, color) in semantic {
        out.description(&format!("semantic.{name}"), &color.description);
        out.reference(base, name, "light", &color.light);
        out.reference(base, name, "dark", &color.dark);
        used.insert(color.light.as_str());
        used.insert(color.dark.as_str());
    }
    out.unused(base.keys(), &used);

    let resolve = |color: &SemanticColor| {
        Some((
            *values.get(color.light.as_str())?,
            *values.get(color.dark.as_str())?,
        ))
    };
    for (name, color) in semantic {
        let Some(on) = &color.on else { continue };
        if on == name {
            out.push(format!("semantic.{name}.on: a color cannot sit on itself"));
            continue;
        }
        let Some(behind) = semantic.get(on) else {
            out.push(format!("semantic.{name}.on: unknown semantic color {on}"));
            continue;
        };
        let (Some(front), Some(back)) = (resolve(color), resolve(behind)) else {
            continue;
        };
        for (appearance, a, b) in [("light", front.0, back.0), ("dark", front.1, back.1)] {
            let ratio = contrast(a, b);
            if ratio < MIN_CONTRAST {
                out.push(format!(
                    "semantic.{name}: contrast on {on} is {} in {appearance}, needs {MIN_CONTRAST:.2} or more",
                    truncated(ratio)
                ));
            }
        }
    }
    out.not_empty(semantic.is_empty());

    if out.found {
        return None;
    }
    let colors = semantic
        .iter()
        .filter_map(|(name, color)| {
            let (light, dark) = resolve(color)?;
            let flagged = |key: &str| base.get(key).is_some_and(|entry| entry.placeholder);
            Some(Color {
                name: name.clone(),
                description: color.description.clone(),
                light,
                dark,
                placeholder: flagged(&color.light) || flagged(&color.dark),
            })
        })
        .collect();
    let placeholders = base.values().filter(|entry| entry.placeholder).count();
    Some((colors, placeholders))
}

/// Checks `typography.json` and resolves its roles.
fn check_typography(file: TypographyFile, problems: &mut Vec<Problem>) -> Option<Vec<Role>> {
    let mut out = FileProblems::new(TYPOGRAPHY_PATH, problems);
    out.header(&file.schema, file.schema_version, "typography.schema.json");
    let entries = &file.roles.0;
    if entries.is_empty() {
        out.push("roles: needs at least one role".to_string());
    }
    let mut roles = Vec::new();
    for (name, entry) in entries {
        out.description(&format!("roles.{name}"), &entry.description);
        match RoleName::ALL.into_iter().find(|role| role.as_str() == name) {
            Some(role) => roles.push(Role {
                name: role,
                description: entry.description.clone(),
                weight: entry.weight,
            }),
            None => {
                let known: Vec<&str> = RoleName::ALL.iter().map(|role| role.as_str()).collect();
                out.push(format!(
                    "roles.{name}: unknown role, expected one of: {}",
                    known.join(", ")
                ));
            }
        }
    }
    (!out.found).then_some(roles)
}

/// Checks `spacing.json` and resolves its semantic values.
fn check_spacing(file: SpacingFile, problems: &mut Vec<Problem>) -> Option<Vec<Space>> {
    let mut out = FileProblems::new(SPACING_PATH, problems);
    out.header(&file.schema, file.schema_version, "spacing.schema.json");
    let base = &file.base.0;
    let semantic = &file.semantic.0;
    out.names("base", base.keys());
    out.names("semantic", semantic.keys());

    let mut values: BTreeMap<&str, u32> = BTreeMap::new();
    for (name, entry) in base {
        match u32::try_from(entry.value) {
            Ok(value) if entry.value <= MAX_SPACE => {
                values.insert(name, value);
            }
            _ => out.push(format!(
                "base.{name}.value: expected a whole number from 0 to {MAX_SPACE}, found {}",
                entry.value
            )),
        }
    }

    let mut used = BTreeSet::new();
    for (name, space) in semantic {
        out.description(&format!("semantic.{name}"), &space.description);
        out.reference(base, name, "ref", &space.reference);
        used.insert(space.reference.as_str());
    }
    out.unused(base.keys(), &used);
    out.not_empty(semantic.is_empty());

    if out.found {
        return None;
    }
    Some(
        semantic
            .iter()
            .filter_map(|(name, space)| {
                Some(Space {
                    name: name.clone(),
                    description: space.description.clone(),
                    value: *values.get(space.reference.as_str())?,
                })
            })
            .collect(),
    )
}

/// Whether `name` matches [`NAME_PATTERN`].
fn is_valid_name(name: &str) -> bool {
    let mut parts = name.split('.');
    let first_ok = parts.next().is_some_and(|first| {
        let mut chars = first.chars();
        chars.next().is_some_and(|c| c.is_ascii_lowercase())
            && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
    });
    first_ok
        && parts.all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
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

/// Reads `#RRGGBB` with uppercase hex digits, matching [`COLOR_PATTERN`].
fn parse_rgb(value: &str) -> Option<Rgb> {
    let hex = value.strip_prefix('#')?;
    if hex.len() != 6
        || !hex
            .chars()
            .all(|c| c.is_ascii_digit() || ('A'..='F').contains(&c))
    {
        return None;
    }
    let channel = |at: usize| u8::from_str_radix(hex.get(at..at + 2)?, 16).ok();
    Some(Rgb(channel(0)?, channel(2)?, channel(4)?))
}

/// The WCAG relative luminance of a color.
fn luminance(color: Rgb) -> f64 {
    let linear = |v: u8| {
        let c = f64::from(v) / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * linear(color.0) + 0.7152 * linear(color.1) + 0.0722 * linear(color.2)
}

/// The WCAG contrast ratio between two colors.
fn contrast(a: Rgb, b: Rgb) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

/// A ratio cut down (not rounded) to two decimals, for messages only.
fn truncated(ratio: f64) -> String {
    format!("{:.2}", (ratio * 100.0).floor() / 100.0)
}
