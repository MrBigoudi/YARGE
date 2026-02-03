use crate::core_layer::application_system::ecs::resource::ResourceManager;
#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use std::collections::VecDeque;

use crate::config::Version;
use crate::core_layer::application_system::events::user_events::UserEventWrapper;
use crate::{
    ECS, Game, config::Config, platform_layer::event::Event,
    rendering_layer::types::RendererBeginFrameOutput,
};
use crate::{PlatformLayerImpl, RenderingLayer, RenderingLayerImpl};

/// The application system
pub(crate) struct ApplicationSystem<'a> {
    /// The name of the application
    pub(crate) name: String,
    /// The version of the application
    pub(crate) version: Version,

    /// The user defined game
    pub(crate) user_game: &'a mut dyn Game,

    /// The ECS
    pub(crate) ecs: ECS,

    /// A queue of user events
    pub(crate) user_events: VecDeque<UserEventWrapper>,
}

impl<'a> ApplicationSystem<'a> {
    /// Initializes the application
    pub(crate) fn init(
        user_game: &'a mut dyn Game,
        config: &Config,
        platform_layer: &mut PlatformLayerImpl,
        rendering_layer: &mut RenderingLayerImpl<'a>,
    ) -> Result<Self, ErrorType> {
        let name = config.application_config.name.clone();
        let version = config.application_config.version.clone();

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
        let user_events = VecDeque::new();

        let mut application = Self {
            name,
            version,
            user_game,
            ecs,
            user_events,
        };
        log_info!(
            "Application: {:?}, version: {:?} initialized",
            application.name,
            application.version
        );

        // Inits the user's game
        match application.user_game.on_start() {
            Ok(mut events) => application.user_events.append(&mut events),
            Err(err) => {
                log_error!("The user game failed to start: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };
        // Must be called in case user generated entities in the 'on_start' event
        if let Err(err) = application.generate_ecs_entities() {
            log_error!(
                "Failed to generate entities in the ECS when initializing the application: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
        // TODO: register engine level FileResourceId

        match application.handle_user_events(platform_layer, rendering_layer) {
            Ok(true) => {
                log_error!("User asked to quit the app on start");
                return Err(ErrorType::WrongArgument(String::from(
                    "`UserEvent::QuitApp' shouldn't be created inside `on_start()'",
                )));
            }
            Err(err) => {
                log_error!(
                    "Failed to handle user events when initializing the application: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
            _ => {}
        }

        // Run ECS systems
        if let Err(err) = application.run_systems() {
            log_error!(
                "Failed to run the ECS systems when initializing the application: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        Ok(application)
    }

    /// Generates ECS entities
    pub(crate) fn generate_ecs_entities(&mut self) -> Result<(), ErrorType> {
        if let Err(err) = self.ecs.spawn_real_entities() {
            log_error!(
                "Failed to spawn entities in the ECS from the application layer: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
        Ok(())
    }

    /// Updates the application
    /// Returns true if the application should quit
    pub(crate) fn update(
        &mut self,
        event: Event,
        platform_layer: &mut PlatformLayerImpl,
        rendering_layer: &mut RenderingLayerImpl<'_>,
    ) -> Result<bool, ErrorType> {
        // Handle application events
        match self.handle_event(event) {
            Ok(mut events) => {
                self.user_events.append(&mut events);
            }
            Err(err) => {
                log_error!(
                    "Failed to handle an event in the application layer: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        // Handle resource loading
        match self.handle_loading_resources(platform_layer, rendering_layer) {
            Ok(mut events) => {
                self.user_events.append(&mut events);
            }
            Err(err) => {
                log_error!(
                    "Failed to handle loading resources in the application layer: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let delta_time = 0.;
        match self.user_game.on_update(delta_time) {
            Ok(mut events) => {
                self.user_events.append(&mut events);
            }
            Err(err) => {
                log_error!(
                    "Failed to update the game in the application layer: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        // Handle user events
        // Begin by generate new entities if needed
        if let Err(err) = self.generate_ecs_entities() {
            log_error!(
                "Failed to generate entities in the ECS when updating the application: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
        let should_quit = match self.handle_user_events(platform_layer, rendering_layer) {
            Ok(should_quit) => should_quit,
            Err(err) => {
                log_error!(
                    "Failed to handle user events in the application layer: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        // Run ECS systems
        if let Err(err) = self.run_systems() {
            log_error!(
                "Failed to run the ECS systems in the application layer: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        Ok(should_quit)
    }

    /// Renders the application
    /// Returns true if the application should quit
    pub(crate) fn render(
        &mut self,
        event: Event,
        platform_layer: &mut PlatformLayerImpl,
        rendering_layer: &mut RenderingLayerImpl<'_>,
    ) -> Result<bool, ErrorType> {
        if event == Event::Expose {
            // Begin by generate new entities if needed
            if let Err(err) = self.generate_ecs_entities() {
                log_error!(
                    "Failed to generate entities in the ECS when updating the application: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }

            // TODO: create rendering packet
            let delta_time = 0.;
            match self.user_game.on_render(delta_time) {
                Ok(mut events) => {
                    self.user_events.append(&mut events);
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

            // Handle user events
            match self.handle_user_events(platform_layer, rendering_layer) {
                Ok(should_quit) => return Ok(should_quit),
                Err(err) => {
                    log_error!(
                        "Failed to handle user events in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
        }
        Ok(false)
    }

    /// One iteration of the infinite running loop
    /// Returns true if the application should quit
    pub(crate) fn loop_iteration(
        &mut self,
        event: Event,
        platform_layer: &mut PlatformLayerImpl,
        rendering_layer: &mut RenderingLayerImpl<'_>,
    ) -> Result<bool, ErrorType> {
        match self.update(event, platform_layer, rendering_layer) {
            Ok(true) => return Ok(true),
            Err(err) => {
                log_error!("Failed to update the application layer: {:?}", err);
                return Err(ErrorType::Unknown);
            }
            _ => {}
        };

        match self.render(event, platform_layer, rendering_layer) {
            Ok(true) => return Ok(true),
            Err(err) => {
                log_error!("Failed to render the application layer: {:?}", err);
                return Err(ErrorType::Unknown);
            }
            _ => {}
        }

        Ok(false)
    }

    /// Shuts down the application
    pub(crate) fn shutdown(&mut self) -> Result<(), ErrorType> {
        // Shuts down the ECS system
        if let Err(err) = self.ecs.shutdown() {
            log_error!(
                "Failed to shut down the ECS system when shutting down the application: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        // Shuts down the user's game
        if let Err(err) = self.user_game.on_shutdown() {
            log_error!("The user game failed to shutdown: {:?}", err);
            return Err(ErrorType::Unknown);
        }

        log_info!("Application system shutted down");

        Ok(())
    }

    pub(crate) fn handle_loading_resources(
        &mut self,
        _platform_layer: &mut PlatformLayerImpl,
        _rendering_layer: &mut RenderingLayerImpl<'_>,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        let mut user_events = VecDeque::new();

        // TODO: find a workaround to avoid copying
        let loading_resources: Vec<_> = self
            .ecs
            .resource_manager
            .loading_resources
            .iter()
            .copied()
            .collect();

        for (type_id, real_id) in &loading_resources {
            match self.ecs.resource_manager.sys_try_get(real_id, type_id) {
                Err(err) => {
                    log_error!(
                        "Failed to try getting a loading resource in the application: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
                Ok(None) => {}
                Ok(Some(handler)) => {
                    let user_resource_id = match ResourceManager::get_user_id(real_id) {
                        Ok(id) => id,
                        Err(err) => {
                            log_error!(
                                "Failed to get the user id when handling a loaded resource in the application: {:?}",
                                err
                            );
                            return Err(ErrorType::Unknown);
                        }
                    };
                    match self
                        .user_game
                        .on_resource_loaded(&user_resource_id, handler)
                    {
                        Err(err) => {
                            log_error!(
                                "The user game failed to handle a loaded resource: {:?}",
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
