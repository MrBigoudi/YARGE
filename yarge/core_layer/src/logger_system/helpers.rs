use std::path::PathBuf;

use config::{ConfigLogLevel, ConfigLogTarget};

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

impl LogLevel {
    pub fn from_config(config: &ConfigLogLevel) -> Self {
        match config {
            ConfigLogLevel::Info => LogLevel::Info,
            ConfigLogLevel::Debug => LogLevel::Debug,
            ConfigLogLevel::Warn => LogLevel::Warn,
            ConfigLogLevel::Error => LogLevel::Error,
        }
    }
}

/// The location of the loggers
#[derive(Debug, Default, Clone)]
pub enum LogTarget {
    /// To log to the console
    #[default]
    Console,
    /// To log to the error console
    ErrorConsole,
    /// To log to a markdown file
    Markdown(PathBuf),
    /// To log to a json file
    Json(PathBuf),
}

impl LogTarget {
    pub fn from_config(config: &ConfigLogTarget) -> Self {
        match config {
            ConfigLogTarget::Console => LogTarget::Console,
            ConfigLogTarget::ErrorConsole => LogTarget::ErrorConsole,
            ConfigLogTarget::Markdown(path_buf) => LogTarget::Markdown(path_buf.to_path_buf()),
            ConfigLogTarget::Json(path_buf) => LogTarget::Json(path_buf.to_path_buf()),
        }
    }
}
