use findy_core::Platform;

/// macOS adapter.
#[derive(Debug, Default)]
pub struct MacosPlatform;

impl Platform for MacosPlatform {
    const NAME: &'static str = "macOS";
}
