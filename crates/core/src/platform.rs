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
}
