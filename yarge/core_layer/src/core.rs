use config::Config;
use error::ErrorType;
use platform_layer::{PlatformLayer, PlatformLayerImpl};

use crate::{ApplicationSystem, Game, LoggerSystem};

/// The core layer
pub struct CoreLayer<'a> {
    platform_layer: PlatformLayerImpl,
    logger_system: LoggerSystem,
    application_system: ApplicationSystem<'a>,
}

impl<'a> CoreLayer<'a> {
    /// Initializes the application
    pub fn init(user_game: &'a mut dyn Game, config: &Config) -> Result<Self, ErrorType> {
        // Inits the platform layer
        let platform_layer = match PlatformLayerImpl::init(config) {
            Ok(platform_layer) => platform_layer,
            Err(err) => {
                // TODO: add logging messages
                return Err(err);
            }
        };

        // Inits the logger system
        let logger_system = match LoggerSystem::init(config) {
            Err(err) => {
                // TODO: add logging messages
                return Err(err);
            },
            Ok(logger_system) => logger_system
        };

        // Inits the application system
        let application_system = match ApplicationSystem::init(user_game, config) {
            Err(err) => {
                // TODO: add logging messages
                return Err(err);
            },
            Ok(application_system) => application_system
        };

        Ok(CoreLayer {
            platform_layer,
            logger_system,
            application_system,
        })
    }

    /// Shuts down the application
    pub fn shutdown(&mut self) -> Result<(), ErrorType> {
        // Shuts down the application system
        if let Err(err) = self.application_system.shutdown() {
            // TODO: add logging messages
            return Err(err);
        }

        // Shuts down the logger system
        if let Err(err) = self.logger_system.shutdown() {
            // TODO: add logging messages
            return Err(err);
        }

        // Shuts down the platform layer
        if let Err(err) = self.platform_layer.shutdown() {
            // TODO: add logging messages
            return Err(err);
        }

        Ok(())
    }
}