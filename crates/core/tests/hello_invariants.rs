//! Invariant tests for `desktop_core::hello`, beyond the literal vectors in
//! `hello_contract.rs`.
//!
//! Each test names the acceptance criteria (AC) and contract vectors (V) of
//! `docs/specs/_root/0002-kontrak-sapaan-bersama/index.md` that it covers.
//! Expected values are literals, or prefix, suffix, length, and middle
//! checks. They are never rebuilt with the greeting pattern (AC-3).

use std::sync::Barrier;
use std::thread;

use desktop_core::hello;

/// The fixed text before the name. Used only for prefix checks.
const PREFIX: &str = "Hello, ";

/// Bytes the greeting adds around the name: `PREFIX` plus `!`.
const ADDED_BYTES: usize = 8;

/// The `name` of every contract vector, V-1 to V-11.
fn vector_names() -> Vec<(&'static str, String)> {
    vec![
        ("V-1", "Ayu".to_string()),
        ("V-2", String::new()),
        ("V-3", "   ".to_string()),
        ("V-4", "  Ayu  ".to_string()),
        ("V-5", "Ayu\nBudi\t".to_string()),
        (
            "V-6",
            "\u{141}ukasz \u{674E} \u{1F469}\u{1F3FD}\u{200D}\u{1F4BB}".to_string(),
        ),
        ("V-7", "Cafe\u{301}".to_string()),
        ("V-8", "\u{62F}\u{646}\u{64A}\u{627}".to_string()),
        ("V-9", "%s {0} \\n".to_string()),
        ("V-10", "A\0B".to_string()),
        ("V-11", "a".repeat(1_048_576)),
    ]
}

/// Asserts the byte for byte invariant: `PREFIX`, then `name` unchanged,
/// then `!`, and nothing else.
fn assert_wraps_name(id: &str, name: &str, greeting: &str) {
    assert_eq!(
        greeting.len(),
        name.len() + ADDED_BYTES,
        "{id}: length is name plus 8 bytes (AC-5)"
    );
    assert!(greeting.starts_with(PREFIX), "{id}: prefix (AC-5)");
    assert!(greeting.ends_with('!'), "{id}: suffix (AC-5)");
    assert_eq!(
        &greeting[PREFIX.len()..greeting.len() - 1],
        name,
        "{id}: middle is the name unchanged (AC-5)"
    );
}

/// Covers AC-5 for V-1 to V-11: the key invariants hold for every vector,
/// not only the four names in the unit test.
#[test]
fn every_vector_is_prefix_then_name_unchanged_then_bang() {
    for (id, name) in vector_names() {
        let greeting = hello(name.clone());
        assert_wraps_name(id, &name, &greeting);
    }
}

/// Covers AC-5: inputs outside the vector table are still used as given.
/// A name that already looks like a greeting is not collapsed, and control,
/// formatting, and invisible characters stay in place.
#[test]
fn unusual_names_are_kept_exactly_as_given() {
    let cases = [
        ("already a greeting", "Hello, Ayu!", "Hello, Hello, Ayu!!"),
        ("only a bang", "!", "Hello, !!"),
        ("carriage return", "Ayu\r\n", "Hello, Ayu\r\n!"),
        ("byte order mark", "\u{FEFF}Ayu", "Hello, \u{FEFF}Ayu!"),
        (
            "terminal escape",
            "\u{1B}[31mAyu\u{1B}[0m",
            "Hello, \u{1B}[31mAyu\u{1B}[0m!",
        ),
        ("delete control", "\u{7F}", "Hello, \u{7F}!"),
        ("lone combining mark", "\u{301}", "Hello, \u{301}!"),
        (
            "right to left override",
            "\u{202E}Ayu",
            "Hello, \u{202E}Ayu!",
        ),
        ("zero width joiner only", "\u{200D}", "Hello, \u{200D}!"),
        ("highest scalar value", "\u{10FFFF}", "Hello, \u{10FFFF}!"),
        ("only NUL", "\0", "Hello, \0!"),
    ];
    for (label, name, expected) in cases {
        assert_eq!(hello(name.to_string()), expected, "AC-5: {label}");
    }
}

/// Covers AC-3 and AC-5 (V-9): braces in the name, including the name of
/// the placeholder itself, stay plain text. Checked by prefix, suffix, and
/// middle so this file never holds the greeting pattern.
#[test]
fn braces_in_name_stay_plain_text() {
    for name in ["{name}", "{}", "{{", "}}", "{name:?}", "{0}{1}"] {
        let greeting = hello(name.to_string());
        assert_wraps_name(name, name, &greeting);
    }
}

/// Covers AC-5: a long name of multi byte characters is not cut, and the
/// cut points of the prefix and suffix fall on character boundaries.
#[test]
fn long_multi_byte_name_is_not_cut() {
    let name = "\u{1F469}\u{1F3FD}\u{200D}\u{1F4BB}".repeat(65_536);
    let greeting = hello(name.clone());
    assert_wraps_name("long emoji name", &name, &greeting);
    assert_eq!(
        greeting.chars().count(),
        name.chars().count() + ADDED_BYTES,
        "AC-5: character count is name plus 8"
    );
}

/// Covers AC-1 and AC-6: called from many threads at the same time, `hello`
/// gives the same result as on the main thread, and V-1 is still
/// `Hello, Ayu!` on every thread.
#[test]
fn same_result_from_many_threads_at_once() {
    const THREADS: usize = 8;
    let names: Vec<String> = vector_names()
        .into_iter()
        .filter(|(id, _)| *id != "V-11")
        .map(|(_, name)| name)
        .collect();
    let on_main: Vec<String> = names.iter().cloned().map(hello).collect();
    let start = Barrier::new(THREADS);

    thread::scope(|scope| {
        let workers: Vec<_> = (0..THREADS)
            .map(|_| {
                scope.spawn(|| {
                    start.wait();
                    let ayu = hello("Ayu".to_string());
                    let all: Vec<String> = names.iter().cloned().map(hello).collect();
                    (ayu, all)
                })
            })
            .collect();
        for worker in workers {
            let (ayu, all) = worker.join().expect("AC-6: hello never panics");
            assert_eq!(ayu, "Hello, Ayu!", "AC-1 on another thread");
            assert_eq!(all, on_main, "AC-6: same input gives same output");
        }
    });
}
