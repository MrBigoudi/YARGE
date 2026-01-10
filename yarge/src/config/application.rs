#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::config::Version;

/// The configuration for the application
#[derive(Default, Clone)]
pub struct ApplicationConfig {
    /// The application's name
    pub name: String,
    /// The application's version
    pub version: Version,
}
