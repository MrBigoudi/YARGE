use crate::{config::LoggerConfig, platform_layer::PlatformLayerRwLock};

use once_cell::sync::Lazy;

/// A custom logger
pub struct LoggerSystem {
    /// The actual global logger
    pub (crate) global_logger: &'static Lazy<PlatformLayerRwLock<LoggerSystemInternal>>,
}

/// The internal logger
pub struct LoggerSystemInternal {
    /// The logger configuration
    pub config: LoggerConfig, 
}


/// The global logger to allow static log messages
pub static GLOBAL_LOGGER: Lazy<PlatformLayerRwLock<LoggerSystemInternal>> = Lazy::new(|| { 
    PlatformLayerRwLock::new(LoggerSystemInternal {
        config: LoggerConfig::default()
    })
});