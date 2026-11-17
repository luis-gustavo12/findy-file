//! findy-file domain: models, rules and the contract every operating system
//! must fulfill.
//!
//! This crate depends on neither a GUI framework nor any OS-specific API.
//! No `#[cfg(target_os = ...)]` here.

mod platform;

pub use platform::Platform;
