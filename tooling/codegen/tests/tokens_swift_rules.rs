//! Rule tests for the Swift emitter of the `tokens` source, spec
//! `macos/0005`, beyond the two cases in `tokens_swift_contract.rs`.
//!
//! They lock the naming rule (dots to camel case, backticks for every one
//! of the 53 Swift keywords and only for them), the hex form of colors, the
//! placeholder note, the mapping of every role and weight with no catch all
//! branch, the fixed shape of `Tokens.swift` for empty and one kind sets,
//! how `plan` carries the file (after the schemas, in
//! `apps/macos/DesignSystem/`, sorted by name), and the `generate` and
//! `check` round trip. The rules live in
//! `docs/specs/macos/0005-tema-bersama-dan-kontrol-native/index.md`.
//! Broken sources are written under `env!("CARGO_TARGET_TMPDIR")`; nothing
//! under `design-system/` or `apps/macos/` is edited.

use std::fs;
use std::path::{Path, PathBuf};

use desktop_codegen::tokens::swift::{PATH, output};
use desktop_codegen::tokens::{self, Color, Rgb, Role, RoleName, Space, TokenSet, Weight};
use desktop_codegen::{check, generate, repo_root};

/// The emitter source, read as text for the `_ =>` check of AC-2.
const EMITTER: &str = include_str!("../src/tokens/swift.rs");

/// The 53 Swift keywords of the spec that need backticks as a name.
const KEYWORDS: [&str; 53] = [
    "as",
    "associatedtype",
    "await",
    "break",
    "case",
    "catch",
    "class",
    "continue",
    "default",
    "defer",
    "deinit",
    "do",
    "else",
    "enum",
    "extension",
    "fallthrough",
    "false",
    "fileprivate",
    "for",
    "func",
    "guard",
    "if",
    "import",
    "in",
    "init",
    "inout",
    "internal",
    "is",
    "let",
    "nil",
    "open",
    "operator",
    "precedencegroup",
    "private",
    "protocol",
    "public",
    "repeat",
    "rethrows",
    "return",
    "self",
    "static",
    "struct",
    "subscript",
    "super",
    "switch",
    "throw",
    "throws",
    "true",
    "try",
    "typealias",
    "var",
    "where",
    "while",
];

/// A color token.
fn color(name: &str, light: Rgb, dark: Rgb, placeholder: bool) -> Color {
    Color {
        name: name.to_string(),
        description: "A color.".to_string(),
        light,
        dark,
        placeholder,
    }
}

/// A spacing token.
fn space(name: &str, value: u32) -> Space {
    Space {
        name: name.to_string(),
        description: "A gap.".to_string(),
        value,
    }
}

/// A token set with only the given parts.
fn set(colors: Vec<Color>, roles: Vec<Role>, spacing: Vec<Space>) -> TokenSet {
    TokenSet {
        placeholders: colors.iter().filter(|color| color.placeholder).count(),
        colors,
        roles,
        spacing,
    }
}

/// The constant names after `public static let` in the order written.
fn constants(text: &str) -> Vec<&str> {
    text.lines()
        .filter_map(|line| line.trim_start().strip_prefix("public static let "))
        .map(|rest| {
            rest.split([' ', ':'])
                .next()
                .expect("a constant has a name")
        })
        .collect()
}

/// Covers AC-2 (naming rule): every one of the 53 keywords is written in
/// backticks, and names that are not keywords are not.
#[test]
fn every_swift_keyword_gets_backticks_and_nothing_else() {
    let colors = KEYWORDS
        .iter()
        .map(|name| color(name, Rgb(0, 0, 0), Rgb(0, 0, 0), false))
        .collect();
    let text = output(&set(colors, vec![], vec![])).contents;
    let expected: Vec<String> = KEYWORDS.iter().map(|name| format!("`{name}`")).collect();
    assert_eq!(constants(&text), expected, "AC-2: 53 keywords in backticks");

    let plain = ["type", "value", "defaults", "selfish", "letter", "deFault"];
    let spacing = ["type", "value", "defaults", "selfish", "letter", "de.fault"]
        .iter()
        .map(|name| space(name, 1))
        .collect();
    let text = output(&set(vec![], vec![], spacing)).contents;
    assert_eq!(constants(&text), plain, "AC-2: other names stay plain");
}

/// Covers AC-2 (naming rule): dots are dropped and the first character of
/// each later part is upper cased, so every name is a plain identifier.
#[test]
fn dotted_names_become_camel_case_identifiers() {
    let cases = [
        ("text.primary", "textPrimary"),
        ("a.b.c", "aBC"),
        ("space.2x", "space2x"),
        ("gap.control.large", "gapControlLarge"),
        ("x1.y2", "x1Y2"),
        ("plain", "plain"),
    ];
    let colors = cases
        .iter()
        .map(|(name, _)| color(name, Rgb(1, 2, 3), Rgb(1, 2, 3), false))
        .collect();
    let spacing = cases.iter().map(|(name, _)| space(name, 4)).collect();
    let text = output(&set(colors, vec![], spacing)).contents;
    let names: Vec<&str> = cases.iter().map(|(_, swift)| *swift).collect();
    let mut expected = names.clone();
    expected.extend(&names);
    assert_eq!(
        constants(&text),
        expected,
        "AC-2: camel case for colors and spacing"
    );
    for name in constants(&text) {
        assert!(
            name.chars().next().is_some_and(|c| c.is_ascii_lowercase())
                && name.chars().all(|c| c.is_ascii_alphanumeric()),
            "AC-2: {name} is a plain identifier"
        );
    }
}

/// Covers AC-2 (O-5): colors are `0xRRGGBB` with upper case hex digits and
/// zero padding, light first, then dark.
#[test]
fn colors_are_upper_case_zero_padded_hex() {
    let text = output(&set(
        vec![
            color("a", Rgb(0xab, 0xcd, 0xef), Rgb(0, 0, 0), false),
            color("b", Rgb(1, 2, 3), Rgb(0xff, 0xff, 0xff), false),
        ],
        vec![],
        vec![],
    ))
    .contents;
    for line in [
        "        public static let a = ThemeColor(light: 0xABCDEF, dark: 0x000000)",
        "        public static let b = ThemeColor(light: 0x010203, dark: 0xFFFFFF)",
    ] {
        assert!(text.lines().any(|l| l == line), "AC-2: {line}");
    }
}

/// Covers AC-2 (K-9): only placeholder colors get ` Placeholder value.` in
/// their doc comment.
#[test]
fn only_placeholder_colors_carry_the_note() {
    let text = output(&set(
        vec![
            color("flagged", Rgb(0, 0, 0), Rgb(0, 0, 0), true),
            color("final", Rgb(0, 0, 0), Rgb(0, 0, 0), false),
        ],
        vec![Role {
            name: RoleName::Body,
            description: "Reading text.".to_string(),
            weight: Weight::Regular,
        }],
        vec![space("gap", 1)],
    ))
    .contents;
    assert_eq!(
        text.matches("Placeholder value").count(),
        1,
        "AC-2: one note"
    );
    assert!(
        text.contains("        /// A color. Placeholder value.\n        public static let flagged"),
        "AC-2: the note is on the flagged color"
    );
}

/// The Swift text style of a role. The `match` has no catch all branch, so
/// a new `RoleName` stops this test from compiling until it is mapped here.
fn expected_style(role: RoleName) -> &'static str {
    match role {
        RoleName::Body => ".body",
    }
}

/// The Swift weight of a weight. The `match` has no catch all branch, so a
/// new `Weight` stops this test from compiling until it is mapped here.
fn expected_weight(weight: Weight) -> &'static str {
    match weight {
        Weight::Regular => ".regular",
        Weight::Semibold => ".semibold",
    }
}

/// Covers AC-2 (K-2): every role and weight is mapped, and the emitter has
/// no `_ =>` branch that would swallow a new kind.
#[test]
fn every_role_and_weight_is_mapped_without_a_catch_all() {
    for role in RoleName::ALL {
        for weight in [Weight::Regular, Weight::Semibold] {
            let text = output(&set(
                vec![],
                vec![Role {
                    name: role,
                    description: "A role.".to_string(),
                    weight,
                }],
                vec![],
            ))
            .contents;
            let line = format!(
                "        public static let {} = TextRole(style: {}, weight: {})",
                role.as_str(),
                expected_style(role),
                expected_weight(weight)
            );
            assert!(text.lines().any(|l| l == line), "AC-2: {line}");
        }
    }
    assert_eq!(
        EMITTER.matches("_ =>").count(),
        0,
        "AC-2: no catch all branch"
    );
}

/// Covers AC-1 (O-1, O-3): the marker is on the first line, the three
/// namespaces always come in order, LF only, exactly one final newline,
/// no double blank line, and the same bytes on every call, also for an
/// empty set and a set with one kind only.
#[test]
fn shape_is_fixed_for_empty_and_one_kind_sets() {
    let sets = [
        set(vec![], vec![], vec![]),
        set(vec![], vec![], vec![space("gap", 12)]),
        set(
            vec![color("a", Rgb(0, 0, 0), Rgb(1, 1, 1), true)],
            vec![],
            vec![],
        ),
    ];
    for (index, tokens) in sets.iter().enumerate() {
        let first = output(tokens);
        assert_eq!(first, output(tokens), "AC-1: deterministic, set {index}");
        assert_eq!(first.path, PATH, "AC-1: path");
        let text = first.contents.as_str();
        let line_one = text.lines().next().unwrap_or_default();
        assert!(
            line_one.starts_with("// ")
                && line_one.contains("Generated by desktop-codegen")
                && line_one.contains("Do not edit"),
            "AC-1: marker on line 1: {line_one}"
        );
        assert!(
            text.ends_with("    }\n}\n") && !text.ends_with("\n\n") && !text.contains('\r'),
            "AC-1: line endings, set {index}"
        );
        assert!(
            !text.contains("\n\n\n"),
            "AC-1: no double blank line, set {index}"
        );
        let colors = text.find("public enum Colors {").expect("Colors");
        let typography = text.find("public enum Typography {").expect("Typography");
        let spacing = text.find("public enum Spacing {").expect("Spacing");
        assert!(
            colors < typography && typography < spacing,
            "AC-1: namespace order"
        );
        let count = tokens.colors.len() + tokens.roles.len() + tokens.spacing.len();
        assert_eq!(constants(text).len(), count, "AC-1: one constant per token");
    }
    let empty = output(&sets[0]).contents;
    for namespace in ["Colors", "Typography", "Spacing"] {
        assert!(
            empty.contains(&format!("    public enum {namespace} {{\n    }}\n")),
            "AC-1: empty {namespace}"
        );
    }
}

/// Reads one committed token file.
fn read(path: &str) -> String {
    fs::read_to_string(repo_root().join(path)).expect("the token file is readable")
}

/// Covers AC-1 (K-1): the committed plan carries `Tokens.swift` in
/// `apps/macos/DesignSystem/` after every schema, built from the parsed
/// tokens, with constants sorted by token name in each namespace.
#[test]
fn plan_carries_tokens_swift_after_the_schemas() {
    let plan = (tokens::SOURCE.plan)(&repo_root()).expect("the committed tokens are valid");
    let swift = plan
        .outputs
        .iter()
        .position(|output| output.path == PATH)
        .expect("the plan holds Tokens.swift");
    assert!(
        PATH.starts_with("apps/macos/DesignSystem/"),
        "AC-1: location"
    );
    assert!(
        plan.outputs[..swift]
            .iter()
            .any(|output| output.path.starts_with("design-system/schemas/"))
            && plan.outputs[swift..]
                .iter()
                .all(|output| !output.path.starts_with("design-system/schemas/")),
        "AC-1: Tokens.swift comes after every schema"
    );
    let set = tokens::parse(
        &read(tokens::COLORS_PATH),
        &read(tokens::TYPOGRAPHY_PATH),
        &read(tokens::SPACING_PATH),
    )
    .expect("the committed tokens are valid");
    assert_eq!(
        plan.outputs[swift],
        output(&set),
        "AC-1: built from the parsed tokens"
    );
    let mut names: Vec<&str> = set.colors.iter().map(|color| color.name.as_str()).collect();
    let sorted = {
        let mut sorted = names.clone();
        sorted.sort_unstable();
        sorted
    };
    assert_eq!(names, sorted, "AC-1: colors sorted by name");
    names = set
        .spacing
        .iter()
        .map(|space| space.name.as_str())
        .collect();
    let mut sorted = names.clone();
    sorted.sort_unstable();
    assert_eq!(names, sorted, "AC-1: spacing sorted by name");
}

/// A fresh `tokens_swift_rules_<test>` root holding the three committed
/// token files, with the first `find` in colors replaced when given.
fn temp_root(test: &str, colors_edit: Option<(&str, &str)>) -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("tokens_swift_rules_{test}"));
    if root.exists() {
        fs::remove_dir_all(&root).expect("the old copy can be removed");
    }
    fs::create_dir_all(root.join("design-system/tokens")).expect("the copy folder can be made");
    for path in [
        tokens::COLORS_PATH,
        tokens::TYPOGRAPHY_PATH,
        tokens::SPACING_PATH,
    ] {
        let mut text = read(path);
        if let (Some((find, replace)), true) = (colors_edit, path == tokens::COLORS_PATH) {
            assert!(text.contains(find), "{find:?} is in colors.json");
            text = text.replacen(find, replace, 1);
        }
        fs::write(root.join(path), text).expect("the token file can be written");
    }
    root
}

/// Covers AC-1: a broken token source makes `check` and `generate` fail
/// and writes no `Tokens.swift`.
#[test]
fn broken_source_writes_no_tokens_swift() {
    let root = temp_root("broken", Some(("\"#FAFAFA\"", "\"#fafafa\"")));
    check(&root, &tokens::SOURCE).expect_err("the source is broken");
    generate(&root, &tokens::SOURCE).expect_err("the source is broken");
    assert!(!root.join(PATH).exists(), "AC-1: no Tokens.swift");
    assert!(!root.join("apps").exists(), "AC-1: no app folder");
    fs::remove_dir_all(&root).expect("the copy can be removed");
}

/// Covers AC-1: in a fresh root `generate` makes `apps/macos/DesignSystem/`
/// and writes the planned file, `check` passes, a hand edit is reported as
/// that one file, and `generate` rewrites only it.
#[test]
fn generate_and_check_round_trip_for_tokens_swift() {
    let root = temp_root("round_trip", None);
    let (written, _) = generate(&root, &tokens::SOURCE).expect("generate succeeds");
    assert!(written.iter().any(|path| path == PATH), "AC-1: {written:?}");
    let planned = output(
        &tokens::parse(
            &read(tokens::COLORS_PATH),
            &read(tokens::TYPOGRAPHY_PATH),
            &read(tokens::SPACING_PATH),
        )
        .expect("valid"),
    );
    let on_disk = fs::read_to_string(root.join(PATH)).expect("written");
    assert_eq!(on_disk, planned.contents, "AC-1: written as planned");
    check(&root, &tokens::SOURCE).expect("AC-1: check passes");

    fs::write(root.join(PATH), on_disk.replace("0x3366CC", "0x000000")).expect("editable");
    let problems = check(&root, &tokens::SOURCE).expect_err("a hand edit is caught");
    assert_eq!(problems.len(), 1, "AC-1: {problems:?}");
    assert_eq!(problems[0].path, PATH);
    assert!(
        problems[0]
            .message
            .starts_with("differs from the generator output"),
        "AC-1: {}",
        problems[0]
    );
    let (written, _) = generate(&root, &tokens::SOURCE).expect("generate succeeds");
    assert_eq!(written, [PATH], "AC-1: only the edited file");
    fs::remove_dir_all(&root).expect("the copy can be removed");
}
