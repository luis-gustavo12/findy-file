//! Text shown in the columns of the file list.

use std::time::SystemTime;

use chrono::{DateTime, Local};
use findy_core::{EntryKind, FileEntry};

/// How dates are shown. Meant to become a user setting: new formats are added
/// as new variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateFormat {
    /// Day/month/year, e.g. `17/11/2026 21:20:05`.
    Brazilian,
}

impl DateFormat {
    /// The `chrono` formatting pattern for this format.
    fn pattern(self) -> &'static str {
        match self {
            DateFormat::Brazilian => "%d/%m/%Y %H:%M:%S",
        }
    }
}

/// A date in the local time zone, written in `format`. Empty when unknown.
pub fn date(time: Option<SystemTime>, format: DateFormat) -> String {
    time.map(|time| {
        DateTime::<Local>::from(time)
            .format(format.pattern())
            .to_string()
    })
    .unwrap_or_default()
}

/// What kind of item this is, e.g. `Folder` or `JSON file`.
pub fn kind(entry: &FileEntry) -> String {
    match &entry.kind {
        EntryKind::Directory => "Folder".to_string(),
        EntryKind::Symlink { .. } => "Link".to_string(),
        EntryKind::Other => "Other".to_string(),
        EntryKind::File => match entry.extension() {
            Some(extension) => format!("{} file", extension.to_string_lossy().to_uppercase()),
            None => "File".to_string(),
        },
    }
}

/// A size with binary units (1 KB = 1024 bytes), e.g. `512 B` or `1.5 MB`.
/// Empty when there is no size (folders).
pub fn size(bytes: Option<u64>) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    let Some(bytes) = bytes else {
        return String::new();
    };
    if bytes < 1024 {
        return format!("{bytes} B");
    }

    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    format!("{value:.1} {}", UNITS[unit])
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use chrono::NaiveDate;
    use findy_core::Attributes;

    use super::*;

    fn entry(path: &str, kind: EntryKind) -> FileEntry {
        FileEntry {
            path: PathBuf::from(path),
            kind,
            size: None,
            created: None,
            modified: None,
            accessed: None,
            attributes: Attributes::default(),
            owner: None,
        }
    }

    #[test]
    fn unknown_date_is_empty() {
        assert_eq!(date(None, DateFormat::Brazilian), "");
    }

    #[test]
    fn date_formats() {
        let datetime = NaiveDate::from_ymd_opt(2026, 11, 7)
            .and_then(|day| day.and_hms_opt(21, 5, 9))
            .expect("valid date");
        let render = |format: DateFormat| datetime.format(format.pattern()).to_string();

        assert_eq!(render(DateFormat::Brazilian), "07/11/2026 21:05:09");
    }

    #[test]
    fn date_uses_the_given_format() {
        // The exact value depends on the local time zone; check the shape.
        let text = date(Some(SystemTime::UNIX_EPOCH), DateFormat::Brazilian);
        assert_eq!(text.len(), "17/11/2026 21:20:05".len());
        assert_eq!(&text[2..3], "/");
    }

    #[test]
    fn kind_of_folder_and_link() {
        assert_eq!(kind(&entry("/home/docs", EntryKind::Directory)), "Folder");
        let link = EntryKind::Symlink {
            target: PathBuf::from("/mnt"),
        };
        assert_eq!(kind(&entry("/home/mnt", link)), "Link");
    }

    #[test]
    fn kind_of_file_uses_extension() {
        assert_eq!(kind(&entry("/a/data.json", EntryKind::File)), "JSON file");
        assert_eq!(kind(&entry("/a/Makefile", EntryKind::File)), "File");
        assert_eq!(kind(&entry("/a/.bashrc", EntryKind::File)), "File");
    }

    #[test]
    fn size_formats() {
        assert_eq!(size(None), "");
        assert_eq!(size(Some(0)), "0 B");
        assert_eq!(size(Some(1023)), "1023 B");
        assert_eq!(size(Some(1024)), "1.0 KB");
        assert_eq!(size(Some(1536)), "1.5 KB");
        assert_eq!(size(Some(3_096_626_626)), "2.9 GB");
    }
}
