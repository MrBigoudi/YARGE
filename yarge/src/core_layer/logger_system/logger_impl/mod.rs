use crate::{config::Config, error::ErrorType};

use super::{
    LoggerSystem,
    helpers::{LogLevel, LogTarget},
    logger::GLOBAL_LOGGER,
};

pub mod macros;

impl LoggerSystem {
    /// Initiates the logger systems
    pub fn init(config: &Config) -> Result<Self, ErrorType> {
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

    /// Updates the the minimum log level
    #[allow(unused)]
    pub fn update_min_level(&mut self, _new_min_level: LogLevel) {
        todo!("Implement logger level update")
    }

    /// Updates the the log target
    #[allow(unused)]
    pub fn update_target(&mut self, _new_target: LogTarget) {
        todo!("Implement logger target update")
    }

    /// Shuts down the logger
    pub fn shutdown(&mut self) -> Result<(), ErrorType> {
        Ok(())
    }
}
