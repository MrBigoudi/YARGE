use std::path::PathBuf;

/// The target to log the file
pub enum LogTarget {
    /// Log to the console
    Console,
    /// Log to the console error
    ErrorConsole,
    /// Log to a file
    File(PathBuf),
}
