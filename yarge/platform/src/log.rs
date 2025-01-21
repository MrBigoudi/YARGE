use std::path::PathBuf;

/// The target to log the file
pub enum LogTarget {
    /// Logs to the console
    Console,
    /// Logs to the console error
    ErrorConsole,
    /// Logs to a file
    File(PathBuf),
}
