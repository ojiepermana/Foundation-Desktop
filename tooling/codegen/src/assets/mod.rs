//! The `assets` source: shared brand assets under `shared/assets/`.
//!
//! [`SOURCE`] checks the folder (A-1), the manifest `brand.json` (A-2), the
//! source files derived from it (A-3), and the content of every SVG source
//! (S-1 to S-7) of spec `_root/0005`
//! (`docs/specs/_root/0005-aset-merek-bersama/index.md`). It plans the
//! generated `shared/assets/brand.schema.json`, built from the same
//! constants the check uses, and the macOS icon document of spec
//! `macos/0007` (`apps/macos/Resources/AppIcon.icon`), whose stale files
//! rule I-1 rejects. The tool never writes binary files.

use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::json::UniqueMap;
use crate::{Output, Plan, Problem, Source};

pub mod macos;
mod schema;
mod svg;

pub use svg::check_svg;

/// The `assets` source, run as `desktop-codegen assets <check|generate>`.
pub const SOURCE: Source = Source {
    name: "assets",
    plan,
};

/// The manifest shape version `brand.json` must declare in `schemaVersion`.
pub const SCHEMA_VERSION: u64 = 1;

/// The shared assets folder, from the repo root.
pub const DIR: &str = "shared/assets";

/// The folder rules, from the repo root.
pub const README_PATH: &str = "shared/assets/README.md";

/// The manifest, from the repo root.
pub const MANIFEST_PATH: &str = "shared/assets/brand.json";

/// The generated manifest schema, from the repo root.
pub const SCHEMA_PATH: &str = "shared/assets/brand.schema.json";

/// The folder of source files, from the repo root.
pub const SOURCE_DIR: &str = "shared/assets/source";

/// The asset every manifest must list.
pub const REQUIRED_ASSET: &str = "app-icon";

/// The exact `viewBox` of the `app-icon` source.
pub const APP_ICON_VIEW_BOX: &str = "0 0 1024 1024";

/// The largest source file, in bytes.
pub const MAX_SOURCE_BYTES: usize = 262_144;

/// The SVG elements a source may use, in the order of the spec.
pub const ALLOWED_ELEMENTS: [&str; 19] = [
    "svg",
    "g",
    "path",
    "rect",
    "circle",
    "ellipse",
    "line",
    "polyline",
    "polygon",
    "defs",
    "linearGradient",
    "radialGradient",
    "stop",
    "clipPath",
    "mask",
    "use",
    "symbol",
    "title",
    "desc",
];

/// The value `$schema` must have in `brand.json`.
pub(crate) const SCHEMA_REFERENCE: &str = "./brand.schema.json";

/// Pattern every asset id must match.
pub(crate) const ID_PATTERN: &str = "^[a-z][a-z0-9]*(-[a-z0-9]+)*$";

/// Suffixes an asset id may not end with, because they name variants.
const VARIANT_SUFFIXES: [&str; 2] = ["-light", "-dark"];

/// System files that are ignored in the folder and in `source/`.
const IGNORED_ENTRIES: [&str; 3] = [".DS_Store", "Thumbs.db", "desktop.ini"];

/// How many source files an asset has, one per appearance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Variants {
    /// One source file for both light and dark appearance (`any`).
    Any,
    /// One source file per appearance (`light-dark`).
    LightDark,
}

impl Variants {
    /// Every value, in the order the schema lists them.
    pub(crate) const ALL: [Variants; 2] = [Variants::Any, Variants::LightDark];

    /// The value written in `brand.json`.
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Variants::Any => "any",
            Variants::LightDark => "light-dark",
        }
    }
}

/// One brand asset listed in the manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    /// The asset id, for example `app-icon`.
    pub id: String,
    /// Whether the asset has one source file or one per appearance.
    pub variants: Variants,
    /// `true` while the source files are temporary markers.
    pub placeholder: bool,
}

impl Asset {
    /// The source paths derived from the id, from the repo root: one for
    /// [`Variants::Any`], light then dark for [`Variants::LightDark`].
    pub fn source_paths(&self) -> Vec<String> {
        match self.variants {
            Variants::Any => vec![format!("{SOURCE_DIR}/{}.svg", self.id)],
            Variants::LightDark => vec![
                format!("{SOURCE_DIR}/{}-light.svg", self.id),
                format!("{SOURCE_DIR}/{}-dark.svg", self.id),
            ],
        }
    }
}

/// The manifest after parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Brand {
    /// Every asset, sorted by id in byte order.
    pub assets: Vec<Asset>,
    /// How many assets are still marked as placeholders.
    pub placeholders: usize,
}

/// `brand.json` as written.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ManifestFile {
    #[serde(rename = "$schema")]
    schema: String,
    #[serde(rename = "schemaVersion")]
    schema_version: u64,
    assets: UniqueMap<AssetFile>,
}

/// One entry of `assets` as written.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AssetFile {
    variants: Variants,
    #[serde(default)]
    placeholder: bool,
}

/// Parses and checks the text of `brand.json` (A-2).
///
/// Collects every problem. A text that does not fit the closed type gives
/// exactly one problem, the `serde_json` message.
pub fn parse(manifest: &str) -> Result<Brand, Vec<Problem>> {
    let mut problems = Vec::new();
    let mut push = |message: String| problems.push(Problem::new(MANIFEST_PATH, message));
    let file: ManifestFile = match serde_json::from_str(manifest) {
        Ok(file) => file,
        Err(error) => return Err(vec![Problem::new(MANIFEST_PATH, format!("A-2: {error}"))]),
    };
    if file.schema != SCHEMA_REFERENCE {
        push(format!(
            "A-2: $schema: expected {SCHEMA_REFERENCE}, found {}",
            file.schema
        ));
    }
    if file.schema_version != SCHEMA_VERSION {
        push(format!(
            "A-2: schemaVersion: expected {SCHEMA_VERSION}, found {}",
            file.schema_version
        ));
    }
    let entries = &file.assets.0;
    if !entries.contains_key(REQUIRED_ASSET) {
        push(format!(
            "A-2: assets: missing required asset {REQUIRED_ASSET}"
        ));
    }
    for id in entries.keys() {
        if !is_valid_id(id) {
            push(format!("A-2: assets.{id}: id must match {ID_PATTERN}"));
        } else if VARIANT_SUFFIXES.iter().any(|suffix| id.ends_with(suffix)) {
            push(format!(
                "A-2: assets.{id}: id must not end with -light or -dark"
            ));
        }
    }
    if !problems.is_empty() {
        return Err(problems);
    }
    let assets: Vec<Asset> = entries
        .iter()
        .map(|(id, entry)| Asset {
            id: id.clone(),
            variants: entry.variants,
            placeholder: entry.placeholder,
        })
        .collect();
    let placeholders = assets.iter().filter(|asset| asset.placeholder).count();
    Ok(Brand {
        assets,
        placeholders,
    })
}

/// Whether `id` matches [`ID_PATTERN`].
fn is_valid_id(id: &str) -> bool {
    let mut parts = id.split('-');
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

/// What one folder entry is.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    File,
    Folder,
    Other,
}

/// The entries of a folder, sorted by name, without ignored system files.
fn entries(folder: &Path) -> std::io::Result<Vec<(String, Kind)>> {
    let mut found: Vec<(String, Kind)> = fs::read_dir(folder)?
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let name = entry.file_name().to_string_lossy().into_owned();
            if IGNORED_ENTRIES.contains(&name.as_str()) {
                return None;
            }
            let kind = match entry.file_type() {
                Ok(kind) if kind.is_file() => Kind::File,
                Ok(kind) if kind.is_dir() => Kind::Folder,
                _ => Kind::Other,
            };
            Some((name, kind))
        })
        .collect();
    found.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(found)
}

/// Checks `shared/assets/` under `root` and plans the schema output.
fn plan(root: &Path) -> Result<Plan, Vec<Problem>> {
    if !root.join(DIR).is_dir() {
        return Err(vec![Problem::new(DIR, "A-1: missing folder")]);
    }
    let mut problems = Vec::new();
    let (manifest_ok, source_ok) = check_folder(root, &mut problems);

    let brand = if manifest_ok {
        read_manifest(root, &mut problems)
    } else {
        None
    };
    if let (Some(brand), true) = (&brand, source_ok) {
        check_sources(root, brand, &mut problems);
    }

    match brand {
        Some(brand) if problems.is_empty() => {
            let files: usize = brand
                .assets
                .iter()
                .map(|asset| asset.source_paths().len())
                .sum();
            let asset = brand
                .assets
                .iter()
                .find(|asset| asset.id == REQUIRED_ASSET)
                .expect("A-2 guarantees app-icon");
            let mut contents = Vec::new();
            for path in asset.source_paths() {
                match fs::read_to_string(root.join(&path)) {
                    Ok(text) => contents.push(text),
                    Err(error) => {
                        problems.push(Problem::new(&path, format!("S-1: cannot read: {error}")));
                    }
                }
            }
            if !problems.is_empty() {
                return Err(problems);
            }
            let icon = macos::outputs(asset, &contents);
            check_icon_folder(root, &icon, &mut problems);
            if !problems.is_empty() {
                return Err(problems);
            }
            let mut outputs = vec![Output {
                path: SCHEMA_PATH.to_string(),
                contents: schema::contents(),
            }];
            outputs.extend(icon);
            Ok(Plan {
                outputs,
                summary: format!(
                    "{} assets, {files} source files, {} placeholder values",
                    brand.assets.len(),
                    brand.placeholders
                ),
            })
        }
        _ => Err(problems),
    }
}

/// I-1: when the macOS icon document exists, it holds only `icon.json` and
/// an `Assets/` folder whose files are all produced by the generator.
/// Missing files are left to `check` and `generate`.
fn check_icon_folder(root: &Path, icon: &[Output], problems: &mut Vec<Problem>) {
    let stale = "I-1: not produced by the generator, remove it";
    if !root.join(macos::ICON_DIR).exists() {
        return;
    }
    let found = match entries(&root.join(macos::ICON_DIR)) {
        Ok(found) => found,
        Err(error) => {
            problems.push(Problem::new(
                macos::ICON_DIR,
                format!("I-1: cannot read the folder: {error}"),
            ));
            return;
        }
    };
    for (name, kind) in found {
        let path = format!("{}/{name}", macos::ICON_DIR);
        match (name.as_str(), kind) {
            ("icon.json", Kind::File) => {}
            ("Assets", Kind::Folder) => match entries(&root.join(macos::ICON_ASSETS_DIR)) {
                Ok(layers) => {
                    for (layer, kind) in layers {
                        let path = format!("{}/{layer}", macos::ICON_ASSETS_DIR);
                        let produced = icon.iter().any(|output| output.path == path);
                        if kind != Kind::File || !produced {
                            problems.push(Problem::new(&path, stale));
                        }
                    }
                }
                Err(error) => problems.push(Problem::new(
                    macos::ICON_ASSETS_DIR,
                    format!("I-1: cannot read the folder: {error}"),
                )),
            },
            _ => problems.push(Problem::new(&path, stale)),
        }
    }
}

/// A-1: the folder holds `README.md`, `brand.json`, and `source/`, may hold
/// the generated schema, and holds nothing else except ignored entries.
/// Returns whether `brand.json` and `source/` are usable.
fn check_folder(root: &Path, problems: &mut Vec<Problem>) -> (bool, bool) {
    let required = [
        ("README.md", Kind::File),
        ("brand.json", Kind::File),
        ("source", Kind::Folder),
    ];
    let found = match entries(&root.join(DIR)) {
        Ok(found) => found,
        Err(error) => {
            problems.push(Problem::new(
                DIR,
                format!("A-1: cannot read the folder: {error}"),
            ));
            return (false, false);
        }
    };
    for (name, kind) in &found {
        let path = format!("{DIR}/{name}");
        match required.iter().find(|(wanted, _)| wanted == name) {
            Some((_, wanted)) if wanted == kind => {}
            Some((_, Kind::Folder)) => {
                problems.push(Problem::new(&path, "A-1: expected a folder"));
            }
            Some(_) => problems.push(Problem::new(&path, "A-1: expected a file")),
            None if name == "brand.schema.json" && *kind == Kind::File => {}
            None => problems.push(Problem::new(&path, "A-1: unexpected entry")),
        }
    }
    let mut usable = |name: &str, wanted: Kind| {
        let entry = found.iter().find(|(seen, _)| seen == name);
        if entry.is_none() {
            let message = match wanted {
                Kind::Folder => "A-1: missing required folder",
                _ => "A-1: missing required file",
            };
            problems.push(Problem::new(&format!("{DIR}/{name}"), message));
        }
        entry.is_some_and(|(_, kind)| *kind == wanted)
    };
    let _readme = usable("README.md", Kind::File);
    let manifest = usable("brand.json", Kind::File);
    let source = usable("source", Kind::Folder);
    (manifest, source)
}

/// Reads `brand.json` and parses it (A-2).
fn read_manifest(root: &Path, problems: &mut Vec<Problem>) -> Option<Brand> {
    let text = fs::read(root.join(MANIFEST_PATH))
        .map_err(|error| error.to_string())
        .and_then(|bytes| String::from_utf8(bytes).map_err(|error| error.to_string()));
    let text = match text {
        Ok(text) => text,
        Err(error) => {
            problems.push(Problem::new(
                MANIFEST_PATH,
                format!("A-2: cannot read: {error}"),
            ));
            return None;
        }
    };
    match parse(&text) {
        Ok(brand) => Some(brand),
        Err(found) => {
            problems.extend(found);
            None
        }
    }
}

/// A-3 and S-1 to S-7: `source/` holds exactly the derived source files,
/// and every source that exists is a valid SVG.
fn check_sources(root: &Path, brand: &Brand, problems: &mut Vec<Problem>) {
    let derived: Vec<(String, &str)> = brand
        .assets
        .iter()
        .flat_map(|asset| {
            asset
                .source_paths()
                .into_iter()
                .map(move |path| (path, asset.id.as_str()))
        })
        .collect();
    let found = match entries(&root.join(SOURCE_DIR)) {
        Ok(found) => found,
        Err(error) => {
            problems.push(Problem::new(
                SOURCE_DIR,
                format!("A-3: cannot read the folder: {error}"),
            ));
            return;
        }
    };
    for (name, kind) in &found {
        let path = format!("{SOURCE_DIR}/{name}");
        let is_derived = derived.iter().any(|(wanted, _)| *wanted == path);
        if !is_derived || *kind != Kind::File {
            problems.push(Problem::new(&path, "A-3: unexpected entry"));
        }
    }
    for (path, id) in &derived {
        let present = found
            .iter()
            .any(|(name, kind)| format!("{SOURCE_DIR}/{name}") == *path && *kind == Kind::File);
        if !present {
            if !found
                .iter()
                .any(|(name, _)| format!("{SOURCE_DIR}/{name}") == *path)
            {
                problems.push(Problem::new(path, "A-3: missing source file"));
            }
            continue;
        }
        match fs::read(root.join(path)) {
            Ok(bytes) => problems.extend(check_svg(path, id, &bytes)),
            Err(error) => problems.push(Problem::new(path, format!("S-1: cannot read: {error}"))),
        }
    }
}
