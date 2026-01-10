#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// The possible log levels
#[derive(Debug, Default, Clone, Copy)]
pub enum LogLevel {
    /// To be used when displaying information
    #[default]
    Info,
    /// Only visible on debug mode
    Debug,
    /// Non fatal error messages
    Warn,
    /// Fatal error messages
    Error,
}

/// The location of the loggers
#[derive(Debug, Default, Clone)]
pub enum LogTarget {
    /// To log to the console
    #[default]
    Console,
    /// To log to the error console
    ErrorConsole,
    // /// To log to a markdown file
    // Markdown(PathBuf),
    // /// To log to a json file
    // Json(PathBuf),
}
