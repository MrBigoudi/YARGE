use std::collections::VecDeque;

use crate::{
    ECS,
    core_layer::FileLoaderSystem,
    log_info,
    platform_layer::PlatformLayerImpl,
    rendering_layer::{RenderingLayer, RenderingLayerImpl, types::RendererBeginFrameOutput},
};
#[allow(unused)]
use crate::{config::Config, error::ErrorType, log, log_debug, log_error, platform_layer::Event};

use super::Game;

/// The application system
pub struct ApplicationSystem<'a> {
    /// The user defined game
    pub user_game: &'a mut dyn Game,

    /// The file loader system
    pub file_loader: FileLoaderSystem,

    /// The ECS
    pub ecs: ECS,
}

impl<'a> ApplicationSystem<'a> {
    /// Initializes the application
    pub fn init(
        user_game: &'a mut dyn Game,
        _config: &Config,
        platform_layer: &mut PlatformLayerImpl,
        rendering_layer: &mut RenderingLayerImpl,
    ) -> Result<Self, ErrorType> {
        // Inits the file loader system
        let file_loader = FileLoaderSystem::init();
        log_info!("File loader system initialized");

        // Inits the ECS system
        let ecs = match ECS::init() {
            Ok(ecs) => ecs,
            Err(err) => {
                log_error!(
                    "Failed to initialize the ECS system when initializing the application: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        log_info!("ECS initialized");

        let mut application = Self {
            user_game,
            file_loader,
            ecs,
        };

        // Inits the user's game
        let user_events = match application.user_game.on_start() {
            Ok(events) => events,
            Err(err) => {
                log_error!("The user game failed to start: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };

        // TODO: register engine level FileResourceId

        match application.handle_user_events(user_events, platform_layer, rendering_layer) {
            Ok(false) => Ok(application),
            Ok(true) => {
                log_error!("User asked to quit the app on start");
                Err(ErrorType::WrongArgument(String::from(
                    "`UserEvent::QuitApp' can't be return by `on_start()'",
                )))
            }
            Err(err) => {
                log_error!(
                    "Failed to handle user events when starting the application: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
    }

    /// One iteration of the infinite running loop
    /// Returns true if the application should quit
    pub fn loop_iteration(
        &mut self,
        event: Event,
        platform_layer: &mut PlatformLayerImpl,
        rendering_layer: &mut RenderingLayerImpl,
    ) -> Result<bool, ErrorType> {
        let mut user_events = VecDeque::new();

        // Handle application events
        match self.handle_event(event) {
            Ok(mut events) => {
                user_events.append(&mut events);
            }
            Err(err) => {
                log_error!(
                    "Failed to handle an event in the application layer: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        // Handle file loading
        match self.handle_loading_files(platform_layer, rendering_layer) {
            Ok(mut events) => {
                user_events.append(&mut events);
            }
            Err(err) => {
                log_error!(
                    "Failed to handle loading files in the application layer: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        // TODO: create rendering packet
        let delta_time = 0.;
        match self.user_game.on_update(delta_time) {
            Ok(mut events) => {
                user_events.append(&mut events);
            }
            Err(err) => {
                log_error!(
                    "Failed to update the game in the application layer: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        // Draw a frame
        if event == Event::Expose {
            match self.user_game.on_render(delta_time) {
                Ok(mut events) => {
                    user_events.append(&mut events);
                }
                Err(err) => {
                    log_error!(
                        "Failed to render the game in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            };
            match rendering_layer.begin_frame() {
                Err(err) => {
                    log_error!(
                        "Failed to begin a frame in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
                Ok(RendererBeginFrameOutput::Success) => {
                    if let Err(err) = rendering_layer.end_frame(platform_layer) {
                        log_error!("Failed to end a frame in the application layer: {:?}", err);
                        return Err(ErrorType::Unknown);
                    }
                }
                _ => {}
            }
        }

        // Handle user events
        let should_quit =
            match self.handle_user_events(user_events, platform_layer, rendering_layer) {
                Ok(should_quit) => should_quit,
                Err(err) => {
                    log_error!(
                        "Failed to handle user events in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            };

        Ok(should_quit)
    }

    /// Shuts down the application
    pub fn shutdown(&mut self) -> Result<(), ErrorType> {
        // Shuts down the user's game
        if let Err(err) = self.user_game.on_shutdown() {
            log_error!("The user game failed to shutdown: {:?}", err);
            return Err(ErrorType::Unknown);
        }

        log_info!("Application system shutted down");

        Ok(())
    }

    /// Check if any file have loaded
    pub fn handle_loading_files(
        &mut self,
        _platform_layer: &mut PlatformLayerImpl,
        _rendering_layer: &mut RenderingLayerImpl,
    ) -> Result<VecDeque<crate::UserEventBuilder>, ErrorType> {
        let mut user_events = VecDeque::new();
        for path in &self.file_loader.get_loading_file_paths() {
            match self.file_loader.end_load(path) {
                Err(err) => {
                    log_error!("Failed to end loading a file at `{:?}': {:?}", path, err);
                    return Err(ErrorType::Unknown);
                }
                Ok(None) => {}
                Ok(Some(arc)) => {
                    match self.user_game.on_file_loaded(path, arc) {
                        Err(err) => {
                            log_error!(
                                "The user game failed to handle loaded file ath `{:?}': {:?}",
                                path,
                                err
                            );
                            return Err(ErrorType::Unknown);
                        }
                        Ok(mut events) => {
                            user_events.append(&mut events);
                        }
                    };
                }
            }
        }
        Ok(user_events)
    }
}
