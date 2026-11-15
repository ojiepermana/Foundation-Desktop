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
