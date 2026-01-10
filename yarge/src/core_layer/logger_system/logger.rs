#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{PlatformLayerRwLock, config::LoggerConfig};

/// A custom logger
pub(crate) struct LoggerSystem {
    #[allow(unused)]
    /// The actual global logger
    pub(crate) global_logger:
        &'static once_cell::sync::Lazy<PlatformLayerRwLock<LoggerSystemInternal>>,
}

/// The internal logger
pub struct LoggerSystemInternal {
    /// The logger configuration
    pub config: LoggerConfig,
}

/// The global logger to allow static log messages
pub static GLOBAL_LOGGER: once_cell::sync::Lazy<PlatformLayerRwLock<LoggerSystemInternal>> =
    once_cell::sync::Lazy::new(|| {
        PlatformLayerRwLock::new(LoggerSystemInternal {
            config: LoggerConfig::default(),
        })
    });
