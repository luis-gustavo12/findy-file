use findy_core::Platform;

/// Windows adapter.
#[derive(Debug, Default)]
pub struct WindowsPlatform;

impl Platform for WindowsPlatform {
    const NAME: &'static str = "Windows";
}
