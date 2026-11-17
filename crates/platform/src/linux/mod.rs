use findy_core::Platform;

/// Linux adapter.
#[derive(Debug, Default)]
pub struct LinuxPlatform;

impl Platform for LinuxPlatform {
    const NAME: &'static str = "Linux";
}
