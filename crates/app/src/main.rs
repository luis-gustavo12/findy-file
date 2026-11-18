// On Windows release builds, don't open a console window alongside the app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod format;

use std::path::PathBuf;
use std::thread;

use findy_core::Platform;
use format::DateFormat;
use slint::{ModelRc, VecModel};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    run(findy_platform::current())
}

/// Generic over `P`: the concrete adapter is already known at compile time
/// (static dispatch, no `dyn`).
fn run<P: Platform + Send + 'static>(platform: P) -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;

    // Fixed for now; it will come from the user's settings.
    let date_format = DateFormat::Brazilian;

    // Start at the user's home folder; the input stays empty if it's unknown.
    if let Some(home) = platform.home_dir() {
        window.set_current_path(home.to_string_lossy().as_ref().into());
        load_dir(&window, platform, home, date_format);
    }

    window.run()
}

/// Lists `path` on a background thread, so a large folder doesn't freeze the
/// UI, then shows the items in the window.
fn load_dir<P: Platform + Send + 'static>(
    window: &MainWindow,
    platform: P,
    path: PathBuf,
    date_format: DateFormat,
) {
    let window = window.as_weak();

    thread::spawn(move || {
        let entries = match platform.list_dir(&path) {
            Ok(entries) => entries,
            Err(error) => {
                eprintln!("Couldn't list {}: {error}", path.display());
                return;
            }
        };
        let rows: Vec<EntryRow> = entries
            .iter()
            .map(|entry| EntryRow {
                name: entry.name().to_string_lossy().as_ref().into(),
                modified: format::date(entry.modified, date_format).into(),
                kind: format::kind(entry).into(),
                size: format::size(entry.size).into(),
            })
            .collect();

        // Slint models must be created on the UI thread. This only fails if
        // the window was already closed, so there's nothing left to update.
        let _ = window.upgrade_in_event_loop(move |window| {
            window.set_entries(ModelRc::new(VecModel::from(rows)));
        });
    });
}
