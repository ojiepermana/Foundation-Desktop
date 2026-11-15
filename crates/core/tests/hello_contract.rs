//! Contract tests for `desktop_core::hello`.
//!
//! These tests use only the public path of the crate. The expected values
//! come from the contract vectors in
//! `docs/specs/_root/0002-kontrak-sapaan-bersama/index.md`.

use desktop_core::hello;

#[test]
fn ayu_returns_hello_ayu() {
    assert_eq!(hello("Ayu".to_string()), "Hello, Ayu!");
}

#[test]
fn signature_is_pinned() {
    let pinned: fn(String) -> String = hello;
    assert_eq!(pinned("Ayu".to_string()), "Hello, Ayu!");
}

/// Contract vectors V-1 to V-10, as Rust string literals from the spec.
const VECTORS: [(&str, &str, &str); 10] = [
    ("V-1", "Ayu", "Hello, Ayu!"),
    ("V-2", "", "Hello, !"),
    ("V-3", "   ", "Hello,    !"),
    ("V-4", "  Ayu  ", "Hello,   Ayu  !"),
    ("V-5", "Ayu\nBudi\t", "Hello, Ayu\nBudi\t!"),
    (
        "V-6",
        "\u{141}ukasz \u{674E} \u{1F469}\u{1F3FD}\u{200D}\u{1F4BB}",
        "Hello, \u{141}ukasz \u{674E} \u{1F469}\u{1F3FD}\u{200D}\u{1F4BB}!",
    ),
    ("V-7", "Cafe\u{301}", "Hello, Cafe\u{301}!"),
    (
        "V-8",
        "\u{62F}\u{646}\u{64A}\u{627}",
        "Hello, \u{62F}\u{646}\u{64A}\u{627}!",
    ),
    ("V-9", "%s {0} \\n", "Hello, %s {0} \\n!"),
    ("V-10", "A\0B", "Hello, A\0B!"),
];

#[test]
fn vectors_match_contract() {
    for (id, name, expected) in VECTORS {
        assert_eq!(hello(name.to_string()), expected, "{id}");
    }
    assert_ne!(
        hello("Cafe\u{301}".to_string()),
        "Hello, Caf\u{e9}!",
        "V-7: no Unicode normalisation"
    );
}

#[test]
fn long_name_is_not_cut() {
    let name = "a".repeat(1_048_576);
    let greeting = hello(name.clone());
    assert_eq!(greeting.len(), 1_048_584, "V-11 length");
    assert!(greeting.starts_with("Hello, "), "V-11 prefix");
    assert!(greeting.ends_with('!'), "V-11 suffix");
    assert_eq!(&greeting[7..greeting.len() - 1], name, "V-11 middle");
}
