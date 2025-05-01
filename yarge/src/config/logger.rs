use crate::core_layer::helpers::{LogLevel, LogTarget};

#[derive(Default, Clone)]
/// The configuration for the logger
pub struct LoggerConfig {
    /// Any logs with weaker level won't be displayed
    pub min_level: LogLevel,

    /// Where to log
    pub target: LogTarget,
}
