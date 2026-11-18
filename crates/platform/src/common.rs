//! Code shared by every adapter. Nothing OS-specific lives here: each adapter
//! passes in the parts that differ between systems.

use std::ffi::OsStr;
use std::fs::{self, Metadata};
use std::io;
use std::path::{Path, PathBuf};

use findy_core::{Attributes, EntryKind, FileEntry};

/// Lists the folder at `path`, calling `attributes` to read the OS-specific
/// flags (hidden, read-only, system) of each item.
///
/// Items whose information can't be read (for example, deleted while
/// listing) are skipped, so one bad item doesn't hide the whole folder.
pub(crate) fn list_dir(
    path: &Path,
    attributes: impl Fn(&OsStr, &Metadata) -> Attributes,
) -> io::Result<Vec<FileEntry>> {
    let mut entries = Vec::new();

    for dir_entry in fs::read_dir(path)? {
        let Ok(dir_entry) = dir_entry else {
            continue;
        };
        // Doesn't follow symbolic links: describes the link itself.
        let Ok(metadata) = dir_entry.metadata() else {
            continue;
        };

        let attributes = attributes(&dir_entry.file_name(), &metadata);
        entries.push(to_entry(dir_entry.path(), &metadata, attributes));
    }

    Ok(entries)
}

fn to_entry(path: PathBuf, metadata: &Metadata, attributes: Attributes) -> FileEntry {
    let file_type = metadata.file_type();
    let kind = if file_type.is_symlink() {
        // An empty target means the link couldn't be read.
        EntryKind::Symlink {
            target: fs::read_link(&path).unwrap_or_default(),
        }
    } else if file_type.is_dir() {
        EntryKind::Directory
    } else if file_type.is_file() {
        EntryKind::File
    } else {
        EntryKind::Other
    };

    let size = (kind == EntryKind::File).then_some(metadata.len());

    FileEntry {
        path,
        kind,
        size,
        created: metadata.created().ok(),
        modified: metadata.modified().ok(),
        accessed: metadata.accessed().ok(),
        attributes,
        // Reading the owner needs OS-specific APIs; not implemented yet.
        owner: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_attributes(_name: &OsStr, _metadata: &Metadata) -> Attributes {
        Attributes::default()
    }

    /// Lists `path` sorted by name, since the OS order isn't guaranteed.
    fn list_sorted(
        path: &Path,
        attributes: impl Fn(&OsStr, &Metadata) -> Attributes,
    ) -> Vec<FileEntry> {
        let mut entries = list_dir(path, attributes).expect("listing a temp dir");
        entries.sort_by(|a, b| a.name().cmp(b.name()));
        entries
    }

    #[test]
    fn lists_files_and_directories() {
        let dir = tempfile::tempdir().expect("creating a temp dir");
        fs::write(dir.path().join("notes.txt"), "abc").expect("writing a file");
        fs::create_dir(dir.path().join("photos")).expect("creating a dir");

        let entries = list_sorted(dir.path(), no_attributes);

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name(), "notes.txt");
        assert_eq!(entries[0].kind, EntryKind::File);
        assert_eq!(entries[0].size, Some(3));
        assert!(entries[0].modified.is_some());
        assert_eq!(entries[1].name(), "photos");
        assert_eq!(entries[1].kind, EntryKind::Directory);
        assert_eq!(entries[1].size, None);
    }

    #[test]
    fn empty_directory_has_no_entries() {
        let dir = tempfile::tempdir().expect("creating a temp dir");
        assert!(list_sorted(dir.path(), no_attributes).is_empty());
    }

    #[test]
    fn applies_attributes_from_the_adapter() {
        let dir = tempfile::tempdir().expect("creating a temp dir");
        fs::write(dir.path().join(".secret"), "").expect("writing a file");
        fs::write(dir.path().join("visible"), "").expect("writing a file");

        let entries = list_sorted(dir.path(), |name, _metadata| Attributes {
            hidden: name == ".secret",
            ..Attributes::default()
        });

        assert!(entries[0].attributes.hidden);
        assert!(!entries[1].attributes.hidden);
    }

    #[test]
    fn missing_directory_is_an_error() {
        let dir = tempfile::tempdir().expect("creating a temp dir");
        let missing = dir.path().join("does-not-exist");
        assert!(list_dir(&missing, no_attributes).is_err());
    }
}
