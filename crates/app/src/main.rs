// On Windows release builds, don't open a console window alongside the app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::Path;

use findy_core::{EntryKind, FileEntry, Platform};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    run(findy_platform::current())
}

/// Generic over `P`: the concrete adapter is already known at compile time
/// (static dispatch, no `dyn`).
fn run<P: Platform>(platform: P) -> Result<(), slint::PlatformError> {
    let window = MainWindow::new()?;
    window.set_platform_name(P::NAME.into());

    // Start at the user's home folder; the input stays empty if it's unknown.
    if let Some(home) = platform.home_dir() {
        window.set_current_path(home.to_string_lossy().as_ref().into());
        print_dir(&platform, &home);
    }

    window.run()
}

/// Temporary: prints the folder contents to the console until the UI shows
/// them.
fn print_dir<P: Platform>(platform: &P, path: &Path) {
    let entries = match platform.list_dir(path) {
        Ok(entries) => entries,
        Err(error) => {
            eprintln!("Couldn't list {}: {error}", path.display());
            return;
        }
    };

    println!("{} ({} items)", path.display(), entries.len());
    for entry in &entries {
        println!("{}", describe(entry));
    }
}

/// One line per entry: kind, size, flags (hidden, read-only, system) and name.
fn describe(entry: &FileEntry) -> String {
    let kind = match entry.kind {
        EntryKind::File => "file",
        EntryKind::Directory => "dir",
        EntryKind::Symlink { .. } => "link",
        EntryKind::Other => "other",
    };
    let size = entry
        .size
        .map(|bytes| bytes.to_string())
        .unwrap_or_default();
    let flag = |on: bool, letter: char| if on { letter } else { '-' };
    let attributes = &entry.attributes;
    let flags: String = [
        flag(attributes.hidden, 'H'),
        flag(attributes.read_only, 'R'),
        flag(attributes.system, 'S'),
    ]
    .iter()
    .collect();

    let mut line = format!(
        "{kind:<5} {size:>12} {flags} {}",
        entry.name().to_string_lossy()
    );
    if let EntryKind::Symlink { target } = &entry.kind {
        line.push_str(&format!(" -> {}", target.display()));
    }
    line
}
