//! Operating system adapters.
//!
//! The implementation in use is selected **at compile time** via
//! `cfg(target_os)`. There is no runtime OS detection: each binary contains
//! only the adapter for the system it was compiled for.
//!
//! This is the only crate in the workspace where `cfg(target_os)` may appear.

mod common;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::WindowsPlatform as CurrentPlatform;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::LinuxPlatform as CurrentPlatform;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::MacosPlatform as CurrentPlatform;

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
compile_error!("findy-file only supports Windows, Linux and macOS.");

/// Returns the adapter for the operating system the program was compiled for.
pub fn current() -> CurrentPlatform {
    // The adapters have no fields today, but `default()` keeps working once
    // one of them gains state.
    #[allow(clippy::default_constructed_unit_structs)]
    CurrentPlatform::default()
}
