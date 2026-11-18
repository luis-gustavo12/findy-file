use std::io;
use std::path::{Path, PathBuf};

use crate::FileEntry;

/// Contract implemented by each operating system adapter.
///
/// Operations (listing a directory, moving to the trash, etc.) will be added
/// incrementally.
///
/// The concrete adapter is selected **at compile time** by the
/// `findy-platform` crate. Code that uses this trait must therefore take it
/// through generics (`P: Platform`), never as `dyn Platform`.
pub trait Platform {
    /// Name of the operating system served by this adapter.
    const NAME: &'static str;

    /// The current user's home folder (e.g., `/home/name` on Linux,
    /// `C:\Users\name` on Windows). `None` when it can't be determined.
    fn home_dir(&self) -> Option<PathBuf>;

    /// The items directly inside the folder at `path`, in the order the
    /// operating system returns them (not sorted).
    ///
    /// Fails if the folder itself can't be read (it doesn't exist, access is
    /// denied, etc.).
    fn list_dir(&self, path: &Path) -> io::Result<Vec<FileEntry>>;
}
