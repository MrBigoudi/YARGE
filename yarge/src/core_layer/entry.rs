#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{PlatformLayer, config::Config, core_layer::core::CoreLayer};

/// The entry point of the engine
pub struct Entry;

impl Entry {
    /// The entry point of the engine
    /// Every program using the engine should work simply by calling this function
    pub fn run(
        user_game: &mut dyn crate::Game,
        config_file: Option<&std::path::Path>,
    ) -> Result<(), ErrorType> {
        // Reads the configuration file
        let config = match Config::init(config_file) {
            Ok(config) => {
                println!("Configuration initialized");
                config
            }
            Err(err) => {
                // TODO: add better logging messages when the logging system is not available
                eprintln!("Failed to initialize the config: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };

        // Inits the core layer
        let mut core_layer = match CoreLayer::init(user_game, &config) {
            Ok(core_layer) => {
                log_info!("Core layer initialized");
                core_layer
            }
            Err(err) => {
                // TODO: add better logging messages when the logging system is not available
                eprintln!("Failed to initialize the core layer: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };

        // Runs the application
        'infinite_loop: loop {
            // Handle events
            match core_layer.platform_layer.poll_event() {
                Ok(event) => {
                    let should_quit = match core_layer.application_system.loop_iteration(
                        event,
                        &mut core_layer.platform_layer,
                        &mut core_layer.rendering_layer,
                    ) {
                        Err(err) => {
                            log_error!("Failed to handle an event: {:?}", err);
                            return Err(ErrorType::Unknown);
                        }
                        Ok(event) => event,
                    };
                    if event == crate::platform_layer::event::Event::WindowClosed || should_quit {
                        log_info!("The window is closing");
                        break 'infinite_loop;
                    }
                }
                Err(err) => {
                    // TODO: add logging messages
                    log_error!("Failed to poll an event: {:?}", err);
                    return Err(ErrorType::Unknown);
                }
            };
        }

        // Shuts down the core layer
        if let Err(err) = core_layer.shutdown() {
            // TODO: add better logging messages when the logging system is not available
            eprintln!("Failed to shutdown the core layer: {:?}", err);
            return Err(ErrorType::Unknown);
        }

        // Fck VSCode weird integrated terminal
        println!();

        Ok(())
    }
}
