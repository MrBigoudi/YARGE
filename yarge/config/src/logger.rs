use std::path::PathBuf;

/// The log levels to put in a config file
/// They should match the levels from the [core_layer] crate
#[derive(Debug, Default)]
pub enum ConfigLogLevel {
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
/// They should match the level targets from the [core_layer] crate
#[derive(Debug, Default)]
pub enum ConfigLogTarget {
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

#[derive(Default)]
/// The configuration for the logger
pub struct LoggerConfig {
    /// The minimum log level to be displayed
    pub level: ConfigLogLevel,
    /// The target for the logs
    pub target: ConfigLogTarget,
}
