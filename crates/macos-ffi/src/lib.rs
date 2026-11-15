//! Thin bridge from Swift to the Rust Core.
//!
//! Design: `docs/specs/macos/0001-sapaan-di-jendela-macos/index.md`.
//!
//! Every exported function has the same name and signature as the function in
//! `desktop_core` and only forwards the call. No behaviour lives here.
//!
//! Ownership at the boundary: every value that crosses is copied. Swift copies
//! the argument into a buffer owned by Rust, and Rust takes it over. Rust
//! returns a buffer owned by Rust, and the generated Swift code copies it into
//! a Swift `String` and frees the buffer. Nothing is shared after a call ends.

uniffi::setup_scaffolding!();

/// Returns the greeting for `name`. Forwards to [`desktop_core::hello`].
#[uniffi::export]
#[must_use]
pub fn hello(name: String) -> String {
    desktop_core::hello(name)
}

#[cfg(test)]
mod tests {
    use super::hello;

    #[test]
    fn forwards_to_core() {
        let pinned: fn(String) -> String = hello;
        assert_eq!(pinned("Ayu".to_string()), "Hello, Ayu!");
        assert_eq!(pinned(String::new()), "Hello, !");
    }
}
