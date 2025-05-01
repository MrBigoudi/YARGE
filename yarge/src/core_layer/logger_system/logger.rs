use crate::config::LoggerConfig;

/// A custom logger
/// Based from a [blog post](https://burgers.io/custom-logging-in-rust-using-tracing)
pub struct LoggerSystem {
    /// The logger configuration
    pub config: LoggerConfig, 
}
