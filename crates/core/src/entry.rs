//! The file entry: what findy-file knows about one item in a folder.

use std::ffi::OsStr;
use std::path::PathBuf;
use std::time::SystemTime;

/// One item in the file system: a file, a directory, a link or something else.
///
/// Filled in by the platform adapter. It holds only data that is cheap to read
/// while listing a folder, so that opening folders with thousands of items
/// stays fast.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    /// Full path of the entry.
    pub path: PathBuf,
    /// What kind of item this is.
    pub kind: EntryKind,
    /// Size in bytes. `None` for directories (their size requires walking
    /// their contents) and for kinds without a meaningful size.
    pub size: Option<u64>,
    /// When the entry was created. `None` when the file system doesn't
    /// record it (common on Linux).
    pub created: Option<SystemTime>,
    /// When the entry's contents were last modified.
    pub modified: Option<SystemTime>,
    /// When the entry was last read. Often disabled or imprecise on modern
    /// systems, so treat it as a hint.
    pub accessed: Option<SystemTime>,
    /// Flags such as hidden and read-only.
    pub attributes: Attributes,
    /// Name of the user account that owns the entry. `None` when unavailable.
    pub owner: Option<String>,
}

/// What kind of item a [`FileEntry`] is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Directory,
    /// A symbolic link (or a junction on Windows). `target` is the path stored
    /// in the link; it may not exist (broken link).
    Symlink {
        target: PathBuf,
    },
    /// Anything else: devices, sockets, named pipes, etc.
    Other,
}

/// Flags of a [`FileEntry`].
///
/// The platform adapter decides what each flag means on its system — for
/// example, `hidden` is a leading dot on Linux and the hidden attribute on
/// Windows.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Attributes {
    /// Hidden from normal listings.
    pub hidden: bool,
    /// Cannot be modified.
    pub read_only: bool,
    /// Marked as an operating system file (Windows only; `false` elsewhere).
    pub system: bool,
}

impl FileEntry {
    /// The entry's name: the last component of its path.
    ///
    /// For a root path (`/` or `C:\`) the whole path is returned.
    pub fn name(&self) -> &OsStr {
        self.path.file_name().unwrap_or(self.path.as_os_str())
    }

    /// The extension, without the dot. Only files have one: a directory named
    /// `photos.2024` has no extension.
    pub fn extension(&self) -> Option<&OsStr> {
        match self.kind {
            EntryKind::File => self.path.extension(),
            _ => None,
        }
    }

    pub fn is_dir(&self) -> bool {
        self.kind == EntryKind::Directory
    }
}

#[cfg(test)]
mod tests {
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
    fn name_is_last_path_component() {
        let file = entry("/home/user/report.pdf", EntryKind::File);
        assert_eq!(file.name(), "report.pdf");
    }

    #[test]
    fn name_of_root_is_whole_path() {
        let root = entry("/", EntryKind::Directory);
        assert_eq!(root.name(), "/");
    }

    #[test]
    fn file_extension_is_last_suffix() {
        let file = entry("/tmp/archive.tar.gz", EntryKind::File);
        assert_eq!(file.extension(), Some(OsStr::new("gz")));
    }

    #[test]
    fn dotfile_has_no_extension() {
        let file = entry("/home/user/.bashrc", EntryKind::File);
        assert_eq!(file.extension(), None);
    }

    #[test]
    fn directory_has_no_extension() {
        let dir = entry("/home/user/photos.2024", EntryKind::Directory);
        assert_eq!(dir.extension(), None);
        assert!(dir.is_dir());
    }

    #[test]
    fn symlink_is_not_a_directory() {
        let link = entry(
            "/home/user/docs",
            EntryKind::Symlink {
                target: PathBuf::from("/mnt/docs"),
            },
        );
        assert!(!link.is_dir());
    }
}
