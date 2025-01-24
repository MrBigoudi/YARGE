use std::path::Path;

use config::Config;
use error::ErrorType;

use crate::{Game, core::CoreLayer};

/// The entry point of the engine
pub struct Entry;

impl Entry {
    /// The entry point of the engine
    /// Every program using the engine should work simply by calling this function
    pub fn run(user_game: &mut dyn Game, config_file: Option<&Path>) -> Result<(), ErrorType> {
        // Reads the configuration file
        let config = match Config::init(config_file) {
            Ok(config) => config,
            Err(err) => {
                // TODO: add better logging messages
                eprintln!("Failed to initialize the config");
                return Err(err);
            }
        };

        // Inits the core layer
        let mut core_layer = match CoreLayer::init(user_game, &config) {
            Ok(application_layer) => application_layer,
            Err(err) => {
                // TODO: add logging messages
                eprintln!("Failed to initialize the core layer");
                return Err(err);
            }
        };

        // Runs the application

        // Shuts down the core layer
        if let Err(err) = core_layer.shutdown() {
            // TODO: add logging messages
            eprintln!("Failed to shutdown the core layer");
            return Err(err);
        }

        Ok(())
    }
}
