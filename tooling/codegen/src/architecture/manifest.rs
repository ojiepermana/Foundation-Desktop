//! Reads `Cargo.toml` files and `Cargo.lock` as text, line by line.

/// The trimmed body lines of the TOML table `header` (for example
/// `[workspace]`), without blank lines and comments.
pub(super) fn table<'a>(toml: &'a str, header: &str) -> Vec<&'a str> {
    let mut inside = false;
    let mut body = Vec::new();
    for line in toml.lines().map(str::trim) {
        if line.starts_with('[') {
            inside = line == header;
        } else if inside && !line.is_empty() && !line.starts_with('#') {
            body.push(line);
        }
    }
    body
}

/// The value of `key = value` in `lines`, with quotes removed.
pub(super) fn value<'a>(lines: &[&'a str], key: &str) -> Option<&'a str> {
    lines.iter().find_map(|line| {
        let (left, right) = line.split_once('=')?;
        (left.trim() == key).then(|| right.trim().trim_matches('"'))
    })
}

/// The entries of the `members` array of `[workspace]`, which may span
/// several lines.
pub(super) fn members(root_manifest: &str) -> Vec<String> {
    let mut inside = false;
    let mut text = String::new();
    let mut collecting = false;
    for line in root_manifest.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && !collecting {
            inside = trimmed == "[workspace]";
            continue;
        }
        if !inside {
            continue;
        }
        if !collecting {
            let Some((key, rest)) = trimmed.split_once('=') else {
                continue;
            };
            if key.trim() != "members" {
                continue;
            }
            collecting = true;
            text.push_str(rest);
        } else {
            text.push_str(trimmed);
        }
        if text.contains(']') {
            break;
        }
    }
    let inner = text
        .split_once('[')
        .and_then(|(_, rest)| rest.split_once(']'))
        .map_or("", |(inner, _)| inner);
    inner
        .split(',')
        .map(|entry| entry.trim().trim_matches('"').to_string())
        .filter(|entry| !entry.is_empty())
        .collect()
}

/// One `[[package]]` block of `Cargo.lock`.
pub(super) struct Package {
    pub(super) name: String,
    pub(super) version: String,
    pub(super) has_source: bool,
    pub(super) dependencies: Vec<String>,
}

/// The `[[package]]` blocks of `Cargo.lock`, in file order.
pub(super) fn packages(lockfile: &str) -> Vec<Package> {
    let mut found: Vec<Package> = Vec::new();
    let mut current: Option<Package> = None;
    let mut in_dependencies = false;
    for line in lockfile.lines().map(str::trim) {
        if line.starts_with('[') && !in_dependencies {
            found.extend(current.take());
            if line == "[[package]]" {
                current = Some(Package {
                    name: String::new(),
                    version: String::new(),
                    has_source: false,
                    dependencies: Vec::new(),
                });
            }
            continue;
        }
        let Some(package) = current.as_mut() else {
            continue;
        };
        if in_dependencies {
            if line.starts_with(']') {
                in_dependencies = false;
            } else {
                let entry = line.trim_end_matches(',').trim_matches('"');
                if !entry.is_empty() {
                    package.dependencies.push(entry.to_string());
                }
            }
            continue;
        }
        let Some((key, rest)) = line.split_once('=') else {
            continue;
        };
        let rest = rest.trim();
        match key.trim() {
            "name" => package.name = rest.trim_matches('"').to_string(),
            "version" => package.version = rest.trim_matches('"').to_string(),
            "source" => package.has_source = true,
            "dependencies" => {
                let inline = rest.trim_start_matches('[');
                if let Some(inner) = inline.strip_suffix(']') {
                    package.dependencies.extend(
                        inner
                            .split(',')
                            .map(|entry| entry.trim().trim_matches('"'))
                            .filter(|entry| !entry.is_empty())
                            .map(str::to_string),
                    );
                } else {
                    in_dependencies = true;
                }
            }
            _ => {}
        }
    }
    found.extend(current);
    found
}

/// The index of the one package an entry such as `name`, `name version`,
/// or `name version (source)` points at, if exactly one matches.
pub(super) fn resolve(packages: &[Package], entry: &str) -> Option<usize> {
    let mut parts = entry.split(' ');
    let name = parts.next()?;
    let version = parts.next();
    let mut matches = packages.iter().enumerate().filter(|(_, package)| {
        package.name == name && version.is_none_or(|version| package.version == version)
    });
    let (index, _) = matches.next()?;
    matches.next().is_none().then_some(index)
}
