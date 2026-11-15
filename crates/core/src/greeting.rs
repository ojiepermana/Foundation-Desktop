//! The greeting shown by every app.
//!
//! Contract: `docs/specs/_root/0002-kontrak-sapaan-bersama/index.md`.

/// Returns the greeting for `name`: the text `Hello, `, then `name` exactly
/// as given, then `!`.
///
/// Every `String` is accepted. Nothing is trimmed, normalised, or cut, and
/// the function never fails or panics. An empty `name` gives `Hello, !`.
///
/// # Examples
///
/// ```
/// assert_eq!(desktop_core::hello("Ayu".to_string()), "Hello, Ayu!");
/// ```
#[must_use]
#[allow(
    clippy::needless_pass_by_value,
    reason = "the shared contract fixes an owned String (spec _root/0002, R-3)"
)]
pub fn hello(name: String) -> String {
    format!("Hello, {name}!")
}

#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn greets_by_name() {
        assert_eq!(hello("Ayu".to_string()), "Hello, Ayu!");
    }

    #[test]
    fn accepts_empty_name() {
        assert_eq!(hello(String::new()), "Hello, !");
    }

    #[test]
    fn keeps_whitespace_as_given() {
        assert_eq!(hello("   ".to_string()), "Hello,    !", "V-3");
        assert_eq!(hello("  Ayu  ".to_string()), "Hello,   Ayu  !", "V-4");
    }

    #[test]
    fn same_input_gives_same_output() {
        assert_eq!(hello("Ayu".to_string()), hello("Ayu".to_string()));
    }

    #[test]
    fn adds_exactly_eight_bytes() {
        for name in ["", "Ayu", "\u{62F}\u{646}\u{64A}\u{627}", "A\0B"] {
            assert_eq!(hello(name.to_string()).len(), name.len() + 8, "{name:?}");
        }
    }
}
