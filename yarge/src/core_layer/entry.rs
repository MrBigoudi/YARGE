use std::path::Path;

use crate::{
    config::Config,
    error::ErrorType,
    log_debug, log_error, log_info, log_warn,
    platform_layer::{Event, PlatformLayer},
};

use super::{Game, core::CoreLayer};

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
        'infinite_loop: loop {
            // Handle events
            match core_layer.platform_layer.poll_event() {
                Ok(event) => {
                    if let Err(err) = core_layer.application_system.loop_iteration(event) {
                        log_error!("Failed to handle an event: {:?}", err);
                        return Err(err);
                    }
                    if event == Event::WindowClosed {
                        log_debug!("The window is closing");
                        break 'infinite_loop
                    }
                }
                Err(err) => {
                    // TODO: add logging messages
                    log_error!("Failed to poll an event: {:?}", err);
                    return Err(err);
                }
            };
        }

        // Shuts down the core layer
        if let Err(err) = core_layer.shutdown() {
            // TODO: add logging messages
            log_error!("Failed to shutdown the core layer");
            return Err(err);
        }

        Ok(())
    }
}
