use error::{ErrorLevel, ErrorType};

use crate::log::LogTarget;

/// Abstract trait for the platform specific code
pub trait PlatformManager {
    /// Initiate the internal structure of the platform
    fn init() -> Result<ErrorType, impl PlatformManager>;

    /// Shutdown the platform
    fn shutdown(&mut self) -> Result<(), ErrorType>;

    /// Log message
    fn log(message: &str, error_type: &ErrorType, error_level: &ErrorLevel, log_target: &LogTarget);
}
