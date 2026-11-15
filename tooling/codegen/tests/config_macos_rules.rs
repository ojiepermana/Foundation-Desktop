//! Rule tests for the macOS output of the `config` source, spec
//! `macos/0004`, beyond the eleven cases in `config_macos_contract.rs`.
//!
//! They lock the edges of `M-1` and `M-2` (rejected and accepted right at
//! the line, one problem per rule), the bundle identifier derivation, the
//! fixed shape of `Shared.xcconfig`, how `plan` carries it (after the
//! schema, with the workspace version, only once the `R` rules pass), and
//! the `generate` and `check` round trip. The rules live in
//! `docs/specs/macos/0004-konfigurasi-publik/index.md`. Inputs are copies
//! under `env!("CARGO_TARGET_TMPDIR")`; `shared/config/` and `apps/macos/`
//! are never edited. The output rules O-1 to O-6 for every planned output
//! stay in `config_rules.rs`.

use std::fs;
use std::path::{Path, PathBuf};

use desktop_codegen::config::macos::{
    BUNDLE_IDENTIFIER_SUFFIX, SHARED_XCCONFIG_PATH, shared_xcconfig,
};
use desktop_codegen::config::{self, CONFIG_DIR, PUBLIC_PATH, README_PATH, SCHEMA_PATH};
use desktop_codegen::{Output, Problem, check, generate, repo_root};

const ROOT_MANIFEST: &str = include_str!("../../../Cargo.toml");

/// The product name line of the committed `public.json`.
const PRODUCT_LINE: &str = "\"product_name\": \"Desktop Foundation\"";

/// `(path, rule ID)` of each problem, in order.
fn ids(problems: &[Problem]) -> Vec<(String, String)> {
    problems
        .iter()
        .map(|problem| {
            let id = problem.message.split(':').next().unwrap_or_default();
            (problem.path.clone(), id.to_string())
        })
        .collect()
}

/// Expected `(path, rule ID)` pairs.
fn expect(pairs: &[(&str, &str)]) -> Vec<(String, String)> {
    pairs
        .iter()
        .map(|(path, id)| ((*path).to_string(), (*id).to_string()))
        .collect()
}

/// The output for valid inputs, or a failure with every problem.
fn accepted(name: &str, base: &str, version: &str) -> Output {
    shared_xcconfig(name, base, version)
        .unwrap_or_else(|problems| panic!("{name:?} {base:?} {version:?}: {problems:?}"))
}

/// The value of `key` in the xcconfig text.
fn setting<'a>(text: &'a str, key: &str) -> Option<&'a str> {
    text.lines()
        .find_map(|line| line.strip_prefix(key)?.strip_prefix(" = "))
}

/// The workspace version in `[workspace.package]` of the root manifest.
fn workspace_version() -> &'static str {
    let mut inside = false;
    for line in ROOT_MANIFEST.lines().map(str::trim) {
        if line.starts_with('[') {
            inside = line == "[workspace.package]";
        } else if inside && let Some(value) = line.strip_prefix("version = ") {
            return value.trim_matches('"');
        }
    }
    panic!("the root manifest has a workspace version")
}

/// Covers AC-5 (`M-1`): every form the rule names is rejected with one
/// `M-1` problem on `shared/config/public.json`.
#[test]
fn m1_forms_are_rejected() {
    for name in [
        "//",
        "a//b",
        "$",
        "a$b",
        "$(PRODUCT_NAME)",
        ";",
        "A;",
        "\\",
        "A\\",
    ] {
        let problems = shared_xcconfig(name, "com.example.app", "1.0.0").expect_err("rejected");
        assert_eq!(
            ids(&problems),
            expect(&[(PUBLIC_PATH, "M-1")]),
            "AC-5: {name:?}"
        );
    }
}

/// Covers AC-5 (`M-1`): forms next to the banned ones are carried as is: a
/// single slash, `;` or `\` before other text, quotes, `#`, `=`, `%`, and
/// braces.
#[test]
fn m1_neighbours_are_carried_as_is() {
    for name in [
        "a/b",
        "a / b",
        "a;b",
        "a\\b",
        "a\\;b",
        "\"quoted\"",
        "#1",
        "a = b",
        "100%",
        "{x}",
        "A",
    ] {
        let output = accepted(name, "com.example.app", "1.0.0");
        assert_eq!(
            setting(&output.contents, "INFOPLIST_KEY_CFBundleDisplayName"),
            Some(name),
            "AC-5: {name:?} is carried as is"
        );
    }
}

/// Covers AC-5: each failing rule is reported once, however many of its
/// forms the input breaks, and both rules are reported together.
#[test]
fn one_problem_per_failing_rule() {
    let problems = shared_xcconfig("A // $x;", "com.example.app", "1.0").expect_err("rejected");
    assert_eq!(
        ids(&problems),
        expect(&[(PUBLIC_PATH, "M-1"), ("Cargo.toml", "M-2")]),
        "AC-5: one M-1 and one M-2"
    );
}

/// Covers AC-5 (`M-2`): only three ASCII decimal numbers separated by dots.
#[test]
fn m2_version_edges() {
    for version in [
        "1.0.0.0",
        "1",
        "",
        "1..0",
        ".1.0",
        "1.0.",
        "v1.0.0",
        "1.0.0 ",
        " 1.0.0",
        "1.0.x",
        "1.0.0-rc",
        "1.-1.0",
        "\u{ff11}.0.0",
    ] {
        let problems = shared_xcconfig("Name", "com.example.app", version).expect_err("rejected");
        assert_eq!(
            ids(&problems),
            expect(&[("Cargo.toml", "M-2")]),
            "AC-5: {version:?}"
        );
    }
    for version in ["0.0.0", "10.20.30", "999999.0.1"] {
        let output = accepted("Name", "com.example.app", version);
        assert_eq!(
            setting(&output.contents, "MARKETING_VERSION"),
            Some(version),
            "AC-5: {version:?}"
        );
    }
}

/// Covers AC-2 (`K-6`): the bundle identifier is the base as given, then
/// `.macos`, on one line.
#[test]
fn bundle_identifier_is_the_base_plus_macos() {
    assert_eq!(BUNDLE_IDENTIFIER_SUFFIX, ".macos", "AC-2: suffix");
    for (base, identifier) in [
        ("a.b", "a.b.macos"),
        (
            "com.example.desktop-foundation",
            "com.example.desktop-foundation.macos",
        ),
        ("A1.b-2", "A1.b-2.macos"),
    ] {
        let output = accepted("Name", base, "1.0.0");
        let lines: Vec<&str> = output
            .contents
            .lines()
            .filter(|line| line.starts_with("PRODUCT_BUNDLE_IDENTIFIER"))
            .collect();
        assert_eq!(
            lines,
            [format!("PRODUCT_BUNDLE_IDENTIFIER = {identifier}")],
            "AC-2: {base}"
        );
    }
}

/// Covers AC-1 and AC-9 ("Isi file"): two fixed comment lines carry the
/// marker, then exactly three `KEY = value` lines sorted by key, LF line
/// endings, one final newline, the fixed path, and the same bytes on every
/// call.
#[test]
fn xcconfig_shape_is_fixed() {
    for (name, base, version) in [
        (
            "Desktop Foundation",
            "com.example.desktop-foundation",
            "0.1.0",
        ),
        (
            "\u{dc}n\u{ef} \u{540d}\u{524d}",
            "Com.Example.App",
            "12.34.56",
        ),
    ] {
        let first = accepted(name, base, version);
        assert_eq!(first, accepted(name, base, version), "AC-1: deterministic");
        assert_eq!(first.path, SHARED_XCCONFIG_PATH, "AC-1: path");
        let text = first.contents.as_str();
        assert!(
            text.ends_with('\n') && !text.ends_with("\n\n") && !text.contains('\r'),
            "AC-1: line endings"
        );
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 5, "AC-9: two comments and three settings");
        let head = lines[..2].join("\n");
        assert!(
            lines[..2].iter().all(|line| line.starts_with("// ")),
            "AC-1: comments"
        );
        assert!(
            head.contains("Generated by desktop-codegen") && head.contains("Do not edit"),
            "AC-1: marker in the first two lines"
        );
        let keys: Vec<&str> = lines[2..]
            .iter()
            .map(|line| line.split_once(" = ").expect("KEY = value").0)
            .collect();
        assert_eq!(
            keys,
            [
                "INFOPLIST_KEY_CFBundleDisplayName",
                "MARKETING_VERSION",
                "PRODUCT_BUNDLE_IDENTIFIER"
            ],
            "AC-9: three keys in order"
        );
    }
}

/// Covers AC-1 and AC-2: the committed plan carries the xcconfig after the
/// schema, built from `public.json` and the workspace version.
#[test]
fn plan_carries_the_xcconfig_after_the_schema() {
    let plan = (config::SOURCE.plan)(&repo_root()).expect("the committed config is valid");
    let position = |path: &str| plan.outputs.iter().position(|output| output.path == path);
    let schema = position(SCHEMA_PATH).expect("the plan holds the schema");
    let xcconfig = position(SHARED_XCCONFIG_PATH).expect("the plan holds the xcconfig");
    assert!(
        schema < xcconfig,
        "AC-1: the xcconfig comes after the schema"
    );

    let public: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(repo_root().join(PUBLIC_PATH)).expect("read"))
            .expect("public.json is JSON");
    let build = &public["build"];
    let expected = accepted(
        build["product_name"].as_str().expect("a name"),
        build["app_identifier"].as_str().expect("an identifier"),
        workspace_version(),
    );
    assert_eq!(
        plan.outputs[xcconfig], expected,
        "AC-2: values from the sources"
    );
}

/// A fresh `config_macos_rules_<test>` root with copies of the three files
/// in `shared/config/`, without any output when `with_outputs` is false.
fn fixture(test: &str, with_outputs: bool) -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("config_macos_rules_{test}"));
    if root.exists() {
        fs::remove_dir_all(&root).expect("the old fixture can be removed");
    }
    fs::create_dir_all(root.join(CONFIG_DIR)).expect("the fixture folder can be made");
    let mut files = vec![PUBLIC_PATH, README_PATH];
    if with_outputs {
        files.extend([SCHEMA_PATH, SHARED_XCCONFIG_PATH]);
        fs::create_dir_all(root.join("apps/macos/Config")).expect("the config folder can be made");
    }
    for path in files {
        fs::copy(repo_root().join(path), root.join(path)).expect("the committed file is copied");
    }
    root
}

/// Sets `build.product_name` in the fixture to `name`, as a JSON string.
fn set_name(root: &Path, name: &str) {
    let path = root.join(PUBLIC_PATH);
    let text = fs::read_to_string(&path).expect("readable");
    assert!(
        text.contains(PRODUCT_LINE),
        "the committed name line is present"
    );
    let literal = serde_json::to_string(name).expect("a string serializes");
    fs::write(
        &path,
        text.replacen(PRODUCT_LINE, &format!("\"product_name\": {literal}"), 1),
    )
    .expect("writable");
}

/// Covers AC-5: a name that passes the `R` rules but breaks `M-1` fails
/// `plan` with the `M-1` problem, and `check` and `generate` write nothing.
#[test]
fn m1_failure_through_plan_writes_nothing() {
    let root = fixture("m1_through_plan", false);
    set_name(&root, "Name;");
    let problems = (config::SOURCE.plan)(&root).expect_err("M-1 fails plan");
    assert_eq!(
        ids(&problems),
        expect(&[(PUBLIC_PATH, "M-1")]),
        "AC-5: plan"
    );
    check(&root, &config::SOURCE).expect_err("check fails");
    generate(&root, &config::SOURCE).expect_err("generate fails");
    assert!(!root.join(SCHEMA_PATH).exists(), "AC-5: no schema written");
    assert!(!root.join("apps").exists(), "AC-5: no xcconfig written");
    fs::remove_dir_all(&root).expect("the fixture can be removed");
}

/// Covers AC-5 (order of `plan`): the xcconfig is built only once the `R`
/// rules pass, so a name that breaks both `R-2` and `M-1` gives only `R-2`.
#[test]
fn r_rule_failure_skips_the_macos_rules() {
    let root = fixture("r_before_m", false);
    set_name(&root, " $x");
    let problems = (config::SOURCE.plan)(&root).expect_err("R-2 fails plan");
    assert_eq!(
        ids(&problems),
        expect(&[(PUBLIC_PATH, "R-2")]),
        "AC-5: only R-2"
    );
    fs::remove_dir_all(&root).expect("the fixture can be removed");
}

/// Covers AC-1: in a fresh root `generate` makes `apps/macos/Config/` and
/// writes the planned xcconfig, `check` passes, a hand edit is reported as
/// that one file, and `generate` rewrites only it.
#[test]
fn generate_and_check_round_trip_for_the_xcconfig() {
    let root = fixture("round_trip", false);
    let (written, _) = generate(&root, &config::SOURCE).expect("generate succeeds");
    assert!(
        written.iter().any(|path| path == SHARED_XCCONFIG_PATH),
        "AC-1: {written:?}"
    );
    let plan = (config::SOURCE.plan)(&root).expect("the fixture is valid");
    let planned = plan
        .outputs
        .iter()
        .find(|output| output.path == SHARED_XCCONFIG_PATH)
        .expect("the plan holds the xcconfig");
    let on_disk = fs::read_to_string(root.join(SHARED_XCCONFIG_PATH)).expect("written");
    assert_eq!(on_disk, planned.contents, "AC-1: written as planned");
    check(&root, &config::SOURCE).expect("AC-1: check passes");

    fs::write(
        root.join(SHARED_XCCONFIG_PATH),
        on_disk.replace("0.1.0", "9.9.9"),
    )
    .expect("the output can be edited");
    let problems = check(&root, &config::SOURCE).expect_err("a hand edit is caught");
    assert_eq!(problems.len(), 1, "AC-1: {problems:?}");
    assert_eq!(problems[0].path, SHARED_XCCONFIG_PATH);
    assert!(
        problems[0]
            .message
            .starts_with("differs from the generator output"),
        "AC-1: {}",
        problems[0]
    );
    let (written, _) = generate(&root, &config::SOURCE).expect("generate succeeds");
    assert_eq!(
        written,
        [SHARED_XCCONFIG_PATH],
        "AC-1: only the edited file"
    );
    fs::remove_dir_all(&root).expect("the fixture can be removed");
}
