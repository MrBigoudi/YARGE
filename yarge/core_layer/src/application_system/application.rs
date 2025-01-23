use config::Config;
use error::ErrorType;

use super::Game;

/// The application system
pub struct ApplicationSystem<'a> {
    user_game: &'a mut dyn Game,
}

impl<'a> ApplicationSystem<'a> {
    /// Initializes the application
    pub fn init(user_game: &'a mut dyn Game, _config: &Config) -> Result<Self, ErrorType> {
        // Inits the user's game
        if let Err(err) = user_game.on_start() {
            // TODO: add logging messages
            return Err(err);
        }

        Ok(ApplicationSystem {
            user_game,
        })
    }

    /// Shuts down the application
    pub fn shutdown(&mut self) -> Result<(), ErrorType> {
        // Shuts down the user's game
        if let Err(err) = self.user_game.on_shutdown() {
            // TODO: add logging messages
            return Err(err);
        }

        Ok(())
    }

    
}
