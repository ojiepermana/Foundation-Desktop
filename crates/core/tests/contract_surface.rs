//! Source and manifest checks for the public contract of `desktop-core`.
//!
//! These tests mirror the grep and manifest checks of AC-2, AC-4, AC-6, and
//! AC-7 in `docs/specs/_root/0002-kontrak-sapaan-bersama/index.md`, so plain
//! `cargo test` catches drift without extra tools or dependencies. The files
//! are read at compile time with `include_str!`. The commands in `verify.md`
//! stay the authoritative proof: `cargo tree` (AC-2), `git grep` (AC-3), and
//! `cargo doc` with `-D warnings` (AC-7).

const MANIFEST: &str = include_str!("../Cargo.toml");
const LIB_RS: &str = include_str!("../src/lib.rs");
const GREETING_RS: &str = include_str!("../src/greeting.rs");

/// Returns the body lines of every manifest table, paired with its header.
/// Blank lines and comments are left out.
fn manifest_tables() -> Vec<(String, Vec<String>)> {
    let mut tables: Vec<(String, Vec<String>)> = vec![(String::new(), Vec::new())];
    for line in MANIFEST.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            tables.push((line.to_string(), Vec::new()));
        } else if let Some((_, body)) = tables.last_mut() {
            body.push(line.to_string());
        }
    }
    tables
}

/// Covers AC-2: no dependency table of any kind (normal, dev, build, or
/// per target) has an entry, so Core keeps zero dependencies, also for tests.
#[test]
fn manifest_declares_no_dependencies_of_any_kind() {
    for (header, body) in manifest_tables() {
        if header.contains("dependencies") {
            assert!(body.is_empty(), "AC-2: {header} must stay empty: {body:?}");
        }
        for line in &body {
            assert!(
                !line.starts_with("dependencies")
                    && !line.starts_with("dev-dependencies")
                    && !line.starts_with("build-dependencies"),
                "AC-2: dotted dependency key under {header}: {line}"
            );
        }
    }
}

/// Covers AC-6 and AC-7: the crate forbids `unsafe` and warns on missing docs.
#[test]
fn manifest_forbids_unsafe_code_and_warns_on_missing_docs() {
    let lints = manifest_tables()
        .into_iter()
        .find(|(header, _)| header == "[lints.rust]")
        .map(|(_, body)| body)
        .expect("AC-6: crates/core/Cargo.toml has a [lints.rust] table");
    assert!(
        lints.iter().any(|line| line == "unsafe_code = \"forbid\""),
        "AC-6: unsafe_code = \"forbid\" in [lints.rust]: {lints:?}"
    );
    assert!(
        lints.iter().any(|line| line == "missing_docs = \"warn\""),
        "AC-7: missing_docs = \"warn\" in [lints.rust]: {lints:?}"
    );
}

/// Covers AC-4: `grep -E '^(pub|mod) ' crates/core/src/lib.rs` gives exactly
/// the private module and the one re export.
#[test]
fn crate_root_exports_only_hello_from_a_private_module() {
    let surface: Vec<&str> = LIB_RS
        .lines()
        .filter(|line| line.starts_with("pub ") || line.starts_with("mod "))
        .collect();
    assert_eq!(
        surface,
        ["mod greeting;", "pub use greeting::hello;"],
        "AC-4: public surface of desktop_core"
    );
}

/// Covers AC-4 (rule R-1): the crate root holds only crate docs, `mod`
/// declarations, and `pub use` lines.
#[test]
fn crate_root_holds_only_docs_mod_and_pub_use() {
    for line in LIB_RS.lines().filter(|line| !line.trim().is_empty()) {
        assert!(
            line.starts_with("//!") || line.starts_with("mod ") || line.starts_with("pub use "),
            "AC-4 (R-1): unexpected line in lib.rs: {line}"
        );
    }
}

/// Covers AC-6: the part of `greeting.rs` above its test module has no
/// `use`, `static`, `extern`, or `unsafe` item, so `hello` touches only its
/// argument.
#[test]
fn greeting_body_has_no_use_static_extern_or_unsafe() {
    let body = GREETING_RS
        .split("#[cfg(test)]")
        .next()
        .expect("split always yields a first part");
    assert!(
        body.contains("pub fn hello("),
        "AC-6: the checked part holds hello"
    );
    for (number, line) in body.lines().enumerate() {
        let item = line.trim_start();
        let item = item.strip_prefix("pub ").unwrap_or(item);
        for keyword in ["use", "static", "extern", "unsafe"] {
            let starts_item = item
                .strip_prefix(keyword)
                .and_then(|rest| rest.chars().next())
                .is_some_and(char::is_whitespace);
            assert!(
                !starts_item,
                "AC-6: greeting.rs line {}: {line}",
                number + 1
            );
        }
    }
}

/// Covers AC-7: the crate docs point to the contract spec and its rules.
#[test]
fn crate_docs_point_to_the_contract_rules() {
    let docs: String = LIB_RS
        .lines()
        .filter(|line| line.starts_with("//!"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        docs.contains("docs/specs/_root/0002-kontrak-sapaan-bersama/index.md"),
        "AC-7: crate docs name the spec path"
    );
    assert!(
        docs.contains("R-1 to R-9"),
        "AC-7: crate docs name rules R-1 to R-9"
    );
}

/// Covers AC-7 (rule R-7): the rustdoc of `hello` carries a code example,
/// which `cargo test --doc` runs.
#[test]
fn hello_rustdoc_carries_a_runnable_example() {
    let before_fn = GREETING_RS
        .split("pub fn hello(")
        .next()
        .expect("split always yields a first part");
    let rustdoc: Vec<&str> = before_fn
        .lines()
        .map(str::trim_start)
        .filter(|line| line.starts_with("///"))
        .collect();
    assert!(
        rustdoc.iter().filter(|line| **line == "/// ```").count() >= 2,
        "AC-7: hello has a fenced example in its rustdoc: {rustdoc:?}"
    );
    assert!(
        rustdoc
            .iter()
            .any(|line| line.contains("desktop_core::hello(")),
        "AC-7: the example calls the public path"
    );
}
