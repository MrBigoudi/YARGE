use super::{LogLevel, helpers::LogTarget};

/// A custom logger
/// Based from a [blog post](https://burgers.io/custom-logging-in-rust-using-tracing)
pub struct LoggerSystem {
    /// The minimum log level to be displayed
    /// Any logs with weaker level won't be displayed
    pub min_level: LogLevel,

    /// Where to log
    pub target: LogTarget,
}
