use error::ErrorType;
use platform_layer::{PlatformLayer, PlatformLayerImpl};

use super::Game;

pub struct ApplicationLayer<'a> {
    platform_layer: PlatformLayerImpl,
    user_game: &'a mut dyn Game,
}

impl<'a> ApplicationLayer<'a> {
    /// Initializes the application
    fn init(user_game: &'a mut dyn Game) -> Result<Self, ErrorType> {
        // Inits the platform layer
        let platform_layer = match PlatformLayerImpl::init() {
            Ok(platform_layer) => platform_layer,
            Err(err) => {
                // TODO: add logging messages
                return Err(err);
            }
        };

        // Inits the user's game
        if let Err(err) = user_game.on_start() {
            // TODO: add logging messages
            return Err(err);
        }

        Ok(ApplicationLayer {
            platform_layer,
            user_game,
        })
    }

    /// Shuts down the application
    fn shutdown(&mut self) -> Result<(), ErrorType> {
        // Shuts down the user's game
        if let Err(err) = self.user_game.on_shutdown() {
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

    pub fn run(user_game: &'a mut dyn Game /*TODO: add config*/) -> Result<(), ErrorType> {
        // Inits the application layer
        let mut application_layer = match Self::init(user_game) {
            Ok(application_layer) => application_layer,
            Err(err) => {
                // TODO: add logging messages
                return Err(err);
            }
        };

        // Runs the application

        // Shuts down the application layer
        if let Err(err) = application_layer.shutdown() {
            // TODO: add logging messages
            return Err(err);
        }

        Ok(())
    }
}
