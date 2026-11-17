// On Windows release builds, don't open a console window alongside the app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use findy_core::Platform;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    run(findy_platform::current())
}

/// Generic over `P`: the concrete adapter is already known at compile time
/// (static dispatch, no `dyn`).
fn run<P: Platform>(_platform: P) -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;
    window.set_platform_name(P::NAME.into());
    window.run()
}
