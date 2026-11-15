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
