use std::io;
use std::os::windows::fs::MetadataExt;
use std::path::{Path, PathBuf};

use findy_core::{Attributes, FileEntry, Platform};

use crate::common;

// File attribute flags, from the Windows SDK (winnt.h).
const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;
const FILE_ATTRIBUTE_SYSTEM: u32 = 0x4;

/// Windows adapter.
#[derive(Debug, Default)]
pub struct WindowsPlatform;

impl Platform for WindowsPlatform {
    const NAME: &'static str = "Windows";

    fn home_dir(&self) -> Option<PathBuf> {
        // Reads the user profile folder, e.g. C:\Users\name.
        std::env::home_dir()
    }

    fn list_dir(&self, path: &Path) -> io::Result<Vec<FileEntry>> {
        common::list_dir(path, |_name, metadata| {
            let flags = metadata.file_attributes();
            Attributes {
                hidden: flags & FILE_ATTRIBUTE_HIDDEN != 0,
                read_only: metadata.permissions().readonly(),
                system: flags & FILE_ATTRIBUTE_SYSTEM != 0,
            }
        })
    }
}
