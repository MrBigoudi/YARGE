#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::core_layer::{
    application_system::application::ApplicationSystem, logger_system::logger::LoggerSystem,
};

use crate::{PlatformLayer, PlatformLayerImpl, RenderingLayer, RenderingLayerImpl, config::Config};

/// The core layer
pub(crate) struct CoreLayer<'a> {
    pub(crate) platform_layer: PlatformLayerImpl,
    pub(crate) rendering_layer: RenderingLayerImpl<'a>,
    pub(crate) logger_system: LoggerSystem,
    pub(crate) application_system: ApplicationSystem<'a>,
}

impl<'a> CoreLayer<'a> {
    /// Initializes the application
    pub(crate) fn init(
        user_game: &'a mut dyn crate::Game,
        config: &Config,
    ) -> Result<Self, ErrorType> {
        // Inits the logger system
        let logger_system = match LoggerSystem::init(config) {
            Err(err) => {
                // TODO: add logging messages
                eprintln!("Failed to initialize the logger system: {:?}", err);
                return Err(ErrorType::Unknown);
            }
            Ok(logger_system) => {
                log_info!("Logger system initialized");
                logger_system
            }
        };

        // Inits the platform layer
        let mut platform_layer = match PlatformLayerImpl::init(config) {
            Ok(platform_layer) => {
                log_info!("Platform layer initialized");
                platform_layer
            }
            Err(err) => {
                log_error!("Failed to initialize the platform layer: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };

        // Inits the rendering layer
        let mut rendering_layer = match RenderingLayerImpl::init(config, &mut platform_layer) {
            Ok(rendering_layer) => {
                log_info!("Rendering layer initialized");
                rendering_layer
            }
            Err(err) => {
                log_error!("Failed to initialize the rendering layer: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };

        // Inits the application system
        let application_system = match ApplicationSystem::init(
            user_game,
            config,
            &mut platform_layer,
            &mut rendering_layer,
        ) {
            Err(err) => {
                log_error!("Failed to initialize the application system: {:?}", err);
                return Err(ErrorType::Unknown);
            }
            Ok(application_system) => {
                log_info!("Application system initialized");
                application_system
            }
        };

        Ok(CoreLayer {
            platform_layer,
            rendering_layer,
            logger_system,
            application_system,
        })
    }

    /// Shuts down the application
    pub(crate) fn shutdown(&mut self) -> Result<(), ErrorType> {
        // Shuts down the rendering layer
        if let Err(err) = self.rendering_layer.shutdown() {
            log_error!("Failed to shutdown the rendering layer: {:?}", err);
            return Err(ErrorType::Unknown);
        }

        // Shuts down the application system
        if let Err(err) = self.application_system.shutdown() {
            log_error!("Failed to shutdown the application system: {:?}", err);
            return Err(ErrorType::Unknown);
        }

        // Shuts down the platform layer
        if let Err(err) = self.platform_layer.shutdown() {
            log_error!("Failed to shutdown the platform layer: {:?}", err);
            return Err(ErrorType::Unknown);
        }

        // Shuts down the logger system
        if let Err(err) = self.logger_system.shutdown() {
            // TODO: add better logging messages
            eprintln!("Failed to shutdown the logger system: {:?}", err);
            return Err(ErrorType::Unknown);
        }

        println!("Core layer shutted down");
        Ok(())
    }
}
