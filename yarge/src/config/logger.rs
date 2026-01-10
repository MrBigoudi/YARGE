#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::core_layer::logger_system::helpers::{LogLevel, LogTarget};

/// The configuration for the logger
#[derive(Default, Clone)]
pub struct LoggerConfig {
    /// Any logs with weaker level won't be displayed
    pub min_level: LogLevel,

    /// Where to log
    pub target: LogTarget,
}
