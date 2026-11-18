use std::io;
use std::path::{Path, PathBuf};

use findy_core::{Attributes, FileEntry, Platform};

use crate::common;

/// Linux adapter.
#[derive(Debug, Default)]
pub struct LinuxPlatform;

impl Platform for LinuxPlatform {
    const NAME: &'static str = "Linux";

    fn home_dir(&self) -> Option<PathBuf> {
        // Reads $HOME, e.g. /home/name.
        std::env::home_dir()
    }

    fn list_dir(&self, path: &Path) -> io::Result<Vec<FileEntry>> {
        common::list_dir(path, |name, metadata| Attributes {
            // On Linux, a name starting with a dot means hidden.
            hidden: name.as_encoded_bytes().starts_with(b"."),
            read_only: metadata.permissions().readonly(),
            system: false,
        })
    }
}
