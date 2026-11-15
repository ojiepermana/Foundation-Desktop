//! Rust Core for the desktop foundation.
//!
//! This crate owns the behaviour shared by the three native apps. It builds
//! and tests without any UI library or OS-specific SDK.
//!
//! # Public contract
//!
//! The rules for this crate's public API live in
//! `docs/specs/_root/0002-kontrak-sapaan-bersama/index.md` (R-1 to R-9).
//! In short:
//!
//! * Only items re-exported from this crate root are public. Modules stay private.
//! * A public item is added or changed only after a `_root` spec records it.
//! * Public functions take and return owned values. No lifetimes, no generic parameters.
//! * Apps and bridges pass input and show output unchanged. They add no behaviour.

mod greeting;

pub use greeting::hello;
