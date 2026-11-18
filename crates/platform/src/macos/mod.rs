use std::io;
use std::path::{Path, PathBuf};

use findy_core::{Attributes, FileEntry, Platform};

use crate::common;

/// macOS adapter.
#[derive(Debug, Default)]
pub struct MacosPlatform;

impl Platform for MacosPlatform {
    const NAME: &'static str = "macOS";

    fn home_dir(&self) -> Option<PathBuf> {
        // Reads $HOME, e.g. /Users/name.
        std::env::home_dir()
    }

    fn list_dir(&self, path: &Path) -> io::Result<Vec<FileEntry>> {
        common::list_dir(path, |name, metadata| Attributes {
            // A name starting with a dot means hidden. Finder's own "hidden"
            // flag (UF_HIDDEN) isn't read yet.
            hidden: name.as_encoded_bytes().starts_with(b"."),
            read_only: metadata.permissions().readonly(),
            system: false,
        })
    }
}
