#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::config::Version;

/// The configuration for the application
#[derive(Clone)]
pub(crate) struct ApplicationConfig {
    /// The application's name
    pub(crate) name: String,
    /// The application's version
    pub(crate) version: Version,
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        Self { 
            name: String::from("NewApp"), 
            version: Version::new(0, 0, 1, 0), 
        }
    }
}
