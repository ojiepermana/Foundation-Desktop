//! Contract tests for the `config` source of spec `_root/0004`.
//!
//! These tests use only the public paths of `desktop_codegen` and the
//! standard library. Each test numbered `tNN_` is one row of the tables
//! "Kasus yang ditolak" (T-1 to T-27) and "Kasus yang diterima" (T-28 to
//! T-32) in `docs/specs/_root/0004-konfigurasi-publik/index.md`. Every case
//! starts from a fresh copy of the three committed files under a temporary
//! root and never edits `shared/config/` itself.

use std::fs;
use std::path::{Path, PathBuf};

use desktop_codegen::config::{self, CONFIG_DIR, PUBLIC_PATH, README_PATH, SCHEMA_PATH};
use desktop_codegen::{check, repo_root};

/// The product name line of the committed `public.json`.
const PRODUCT_LINE: &str = "\"product_name\": \"Desktop Foundation\"";

/// The identifier line of the committed `public.json`.
const IDENTIFIER_LINE: &str = "\"app_identifier\": \"com.example.desktop-foundation\"";

/// Makes `config_contract_<name>` under the target temp folder, holding a
/// copy of the three committed files, and returns its root.
fn fixture(name: &str) -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("config_contract_{name}"));
    if root.exists() {
        fs::remove_dir_all(&root).expect("the old fixture can be removed");
    }
    fs::create_dir_all(root.join(CONFIG_DIR)).expect("the fixture folder can be made");
    for path in [PUBLIC_PATH, README_PATH, SCHEMA_PATH] {
        fs::copy(repo_root().join(path), root.join(path))
            .expect("the committed file can be copied");
    }
    root
}

/// Replaces the first `find` with `replace` in the fixture file at `path`.
fn edit(root: &Path, path: &str, find: &str, replace: &str) {
    let file = root.join(path);
    let text = fs::read_to_string(&file).expect("the fixture file is readable");
    assert!(text.contains(find), "{find:?} is not in {path}");
    fs::write(&file, text.replacen(find, replace, 1)).expect("the fixture file can be written");
}

/// Writes `contents` to the fixture file at `path`.
fn write(root: &Path, path: &str, contents: &str) {
    fs::write(root.join(path), contents).expect("the fixture file can be written");
}

/// Asserts that `plan` fails and one problem message starts with `rule`.
fn assert_rejected(root: &Path, rule: &str) {
    let problems = (config::SOURCE.plan)(root).expect_err("plan must fail");
    assert!(
        problems
            .iter()
            .any(|problem| problem.message.starts_with(rule)),
        "no problem starts with {rule}: {problems:?}"
    );
}

/// Asserts that `plan` succeeds.
fn assert_accepted(root: &Path) {
    if let Err(problems) = (config::SOURCE.plan)(root) {
        panic!("plan must succeed: {problems:?}");
    }
}

#[test]
fn committed_outputs_match_the_source() {
    let line = check(&repo_root(), &config::SOURCE).unwrap_or_else(|problems| {
        let lines: Vec<String> = problems.iter().map(ToString::to_string).collect();
        panic!("config check failed:\n{}", lines.join("\n"))
    });
    assert!(
        line.starts_with("config: ok, schema version 1, 2 build values"),
        "unexpected line: {line}"
    );
}

#[test]
fn t01_rejects_an_extra_local_json() {
    let root = fixture("t01");
    write(&root, "shared/config/local.json", "{}");
    assert_rejected(&root, "R-1");
}

#[test]
fn t02_rejects_an_extra_env_file() {
    let root = fixture("t02");
    write(&root, "shared/config/.env", "TOKEN=x\n");
    assert_rejected(&root, "R-1");
}

#[test]
fn t03_rejects_a_subfolder() {
    let root = fixture("t03");
    fs::create_dir_all(root.join("shared/config/signing")).expect("the subfolder can be made");
    assert_rejected(&root, "R-1");
}

#[test]
fn t04_rejects_a_missing_public_json() {
    let root = fixture("t04");
    fs::remove_file(root.join(PUBLIC_PATH)).expect("the file can be removed");
    assert_rejected(&root, "R-1");
}

#[test]
fn t05_rejects_a_missing_readme() {
    let root = fixture("t05");
    fs::remove_file(root.join(README_PATH)).expect("the file can be removed");
    assert_rejected(&root, "R-1");
}

#[test]
fn t06_rejects_text_that_is_not_json() {
    let root = fixture("t06");
    write(&root, PUBLIC_PATH, "{");
    assert_rejected(&root, "R-2");
}

#[test]
fn t07_rejects_an_array() {
    let root = fixture("t07");
    write(&root, PUBLIC_PATH, "[]");
    assert_rejected(&root, "R-2");
}

#[test]
fn t08_rejects_a_runtime_group() {
    let root = fixture("t08");
    edit(
        &root,
        PUBLIC_PATH,
        "\"schemaVersion\": 1,",
        "\"schemaVersion\": 1,\n  \"runtime\": {},",
    );
    assert_rejected(&root, "R-2");
}

#[test]
fn t09_rejects_an_unknown_build_key() {
    let root = fixture("t09");
    edit(
        &root,
        PUBLIC_PATH,
        IDENTIFIER_LINE,
        "\"app_identifier\": \"com.example.desktop-foundation\",\n    \"api_url\": \"https://example.com\"",
    );
    assert_rejected(&root, "R-2");
}

#[test]
fn t10_rejects_a_missing_product_name() {
    let root = fixture("t10");
    edit(
        &root,
        PUBLIC_PATH,
        "\"product_name\": \"Desktop Foundation\",\n    ",
        "",
    );
    assert_rejected(&root, "R-2");
}

#[test]
fn t11_rejects_another_schema_path() {
    let root = fixture("t11");
    edit(
        &root,
        PUBLIC_PATH,
        "./public.schema.json",
        "./other.schema.json",
    );
    assert_rejected(&root, "R-2");
}

#[test]
fn t12_rejects_an_empty_product_name() {
    let root = fixture("t12");
    edit(&root, PUBLIC_PATH, PRODUCT_LINE, "\"product_name\": \"\"");
    assert_rejected(&root, "R-2");
}

#[test]
fn t13_rejects_a_leading_space_in_the_product_name() {
    let root = fixture("t13");
    edit(
        &root,
        PUBLIC_PATH,
        PRODUCT_LINE,
        "\"product_name\": \" Desktop Foundation\"",
    );
    assert_rejected(&root, "R-2");
}

#[test]
fn t14_rejects_a_newline_in_the_product_name() {
    let root = fixture("t14");
    edit(
        &root,
        PUBLIC_PATH,
        PRODUCT_LINE,
        "\"product_name\": \"Desktop\\nFoundation\"",
    );
    assert_rejected(&root, "R-2");
}

#[test]
fn t15_rejects_a_line_separator_in_the_product_name() {
    let root = fixture("t15");
    edit(
        &root,
        PUBLIC_PATH,
        PRODUCT_LINE,
        "\"product_name\": \"Desktop\\u2028Foundation\"",
    );
    assert_rejected(&root, "R-2");
}

#[test]
fn t16_rejects_a_leading_byte_order_mark_in_the_product_name() {
    let root = fixture("t16");
    edit(
        &root,
        PUBLIC_PATH,
        PRODUCT_LINE,
        "\"product_name\": \"\\ufeffDesktop Foundation\"",
    );
    assert_rejected(&root, "R-2");
}

#[test]
fn t17_rejects_a_number_as_the_product_name() {
    let root = fixture("t17");
    edit(&root, PUBLIC_PATH, PRODUCT_LINE, "\"product_name\": 1");
    assert_rejected(&root, "R-2");
}

#[test]
fn t18_rejects_a_duplicate_product_name() {
    let root = fixture("t18");
    edit(
        &root,
        PUBLIC_PATH,
        PRODUCT_LINE,
        "\"product_name\": \"Desktop Foundation\",\n    \"product_name\": \"Desktop Foundation\"",
    );
    assert_rejected(&root, "R-2");
}

#[test]
fn t19_rejects_a_single_segment_identifier() {
    let root = fixture("t19");
    edit(
        &root,
        PUBLIC_PATH,
        IDENTIFIER_LINE,
        "\"app_identifier\": \"desktop\"",
    );
    assert_rejected(&root, "R-2");
}

#[test]
fn t20_rejects_a_segment_starting_with_a_digit() {
    let root = fixture("t20");
    edit(
        &root,
        PUBLIC_PATH,
        IDENTIFIER_LINE,
        "\"app_identifier\": \"com.1example.app\"",
    );
    assert_rejected(&root, "R-2");
}

#[test]
fn t21_rejects_an_underscore_in_the_identifier() {
    let root = fixture("t21");
    edit(
        &root,
        PUBLIC_PATH,
        IDENTIFIER_LINE,
        "\"app_identifier\": \"com.example.app_x\"",
    );
    assert_rejected(&root, "R-2");
}

#[test]
fn t22_rejects_a_segment_ending_with_a_hyphen() {
    let root = fixture("t22");
    edit(
        &root,
        PUBLIC_PATH,
        IDENTIFIER_LINE,
        "\"app_identifier\": \"com.example.app-\"",
    );
    assert_rejected(&root, "R-2");
}

#[test]
fn t23_rejects_an_identifier_of_101_characters() {
    let root = fixture("t23");
    let identifier = format!("com.{}", "a".repeat(97));
    assert_eq!(identifier.len(), 101);
    edit(
        &root,
        PUBLIC_PATH,
        IDENTIFIER_LINE,
        &format!("\"app_identifier\": \"{identifier}\""),
    );
    assert_rejected(&root, "R-2");
}

#[test]
fn t24_rejects_a_string_schema_version() {
    let root = fixture("t24");
    edit(
        &root,
        PUBLIC_PATH,
        "\"schemaVersion\": 1",
        "\"schemaVersion\": \"1\"",
    );
    assert_rejected(&root, "R-2");
}

#[test]
fn t25_rejects_schema_version_2_in_public_json() {
    let root = fixture("t25");
    edit(
        &root,
        PUBLIC_PATH,
        "\"schemaVersion\": 1",
        "\"schemaVersion\": 2",
    );
    assert_rejected(&root, "R-3");
}

#[test]
fn t26_rejects_schema_version_2_in_the_readme() {
    let root = fixture("t26");
    edit(
        &root,
        README_PATH,
        "| 1 | 2026-11-15 |",
        "| 2 | 2026-11-15 |",
    );
    assert_rejected(&root, "R-3");
}

#[test]
fn t27_rejects_schema_version_2_in_both_places() {
    let root = fixture("t27");
    edit(
        &root,
        PUBLIC_PATH,
        "\"schemaVersion\": 1",
        "\"schemaVersion\": 2",
    );
    edit(
        &root,
        README_PATH,
        "| 1 | 2026-11-15 |",
        "| 2 | 2026-11-15 |",
    );
    assert_rejected(&root, "R-3");
}

#[test]
fn t28_accepts_the_committed_files() {
    let root = fixture("t28");
    assert_accepted(&root);
}

#[test]
fn t29_accepts_unicode_names_and_mixed_case_identifiers() {
    let root = fixture("t29");
    edit(
        &root,
        PUBLIC_PATH,
        PRODUCT_LINE,
        "\"product_name\": \"Ünï 名前\"",
    );
    edit(
        &root,
        PUBLIC_PATH,
        IDENTIFIER_LINE,
        "\"app_identifier\": \"Com.Example.App\"",
    );
    assert_accepted(&root);
}

#[test]
fn t30_accepts_a_ds_store_entry() {
    let root = fixture("t30");
    write(&root, "shared/config/.DS_Store", "");
    assert_accepted(&root);
}

#[test]
fn t31_accepts_an_identifier_of_100_characters() {
    let root = fixture("t31");
    let identifier = format!("com.{}", "a".repeat(96));
    assert_eq!(identifier.len(), 100);
    edit(
        &root,
        PUBLIC_PATH,
        IDENTIFIER_LINE,
        &format!("\"app_identifier\": \"{identifier}\""),
    );
    assert_accepted(&root);
}

#[test]
fn t32_accepts_a_folder_without_the_generated_schema() {
    let root = fixture("t32");
    fs::remove_file(root.join(SCHEMA_PATH)).expect("the file can be removed");
    assert_accepted(&root);
}
