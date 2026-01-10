#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{PlatformLayer, config::Config};

use super::logger::LoggerSystem;

use crate::{GLOBAL_LOGGER, LogLevel, LogTarget};

mod macros;

impl LoggerSystem {
    /// Initializes the logger systems
    pub(crate) fn init(config: &Config) -> Result<Self, ErrorType> {
        let mut logger = match GLOBAL_LOGGER.write() {
            Ok(logger) => logger,
            Err(err) => {
                // TODO: better logging messages
                eprintln!("Failed to modify the global logger: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };
        logger.config = config.logger_config.clone();
        Ok(LoggerSystem {
            global_logger: &GLOBAL_LOGGER,
        })
    }

    #[allow(unused)]
    /// Updates the the minimum log level
    pub(crate) fn update_min_level(&mut self, _new_min_level: LogLevel) {
        todo!("Implement logger level update")
    }

    #[allow(unused)]
    /// Updates the the log target
    pub(crate) fn update_target(&mut self, _new_target: LogTarget) {
        todo!("Implement logger target update")
    }

    /// Shuts down the logger
    pub(crate) fn shutdown(&mut self) -> Result<(), ErrorType> {
        if let Err(err) = crate::PlatformLayerImpl::flush_log() {
            log_error!(
                "Failed to flush the logger from the platform layer when shutting down the logger system: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
        println!("Logger shutted down");
        Ok(())
    }
}
