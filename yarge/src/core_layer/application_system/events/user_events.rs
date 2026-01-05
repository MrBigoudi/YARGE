use std::collections::VecDeque;

use crate::{
    core_layer::{ApplicationSystem, FileLoaderSystem},
    error::ErrorType,
    log_debug, log_error,
    platform_layer::PlatformLayerImpl,
    rendering_layer::RenderingLayerImpl,
};

/// An enum representing user fireable events
pub(crate) enum UserEvent {
    /// To close the application
    QuitApp,

    /// To register a new user defined file resource
    /// Should never be created directly by the user, instead use the UserEvent methods
    RegisterCustomFileResource {
        /// The id of the new resource type
        resource_id: crate::FileResourceTypeId,
        /// The function to load the resource
        loader_fct: crate::core_layer::file_system::file::LoadingFileFunction,
    },

    /// To begin loading a file resource
    StartLoadCustomFileResource {
        /// The id of the resource type to load
        resource_id: crate::FileResourceTypeId,
        /// The path to the resource to load
        path: std::path::PathBuf,
    },

    /// To register a new component
    RegisterCustomComponent {
        /// The function to register the component
        register_fct:
            crate::core_layer::application_system::ecs::component::RegisterComponentFunction,
    },
}

/// A public builder for UserEvent
pub struct UserEventBuilder {
    event: UserEvent,
}

impl UserEventBuilder {
    /// Creates a QuitApp event to close the application
    pub fn quit_app() -> Self {
        Self {
            event: UserEvent::QuitApp,
        }
    }

    /// Creates an event to register a new user defined custom file resource
    pub fn register_custom_ron_file_resource<T: crate::RonFileResource>(
        resource_id: &crate::FileResourceTypeId,
    ) -> Self {
        Self {
            event: UserEvent::RegisterCustomFileResource {
                resource_id: FileLoaderSystem::cast_resource_id(resource_id),
                loader_fct: T::start_load_ron,
            },
        }
    }

    /// Creates an event to begin loading a custom file resource
    pub fn start_load_custom_file_resource(
        resource_id: &crate::FileResourceTypeId,
        path: &std::path::Path,
    ) -> Self {
        Self {
            event: UserEvent::StartLoadCustomFileResource {
                resource_id: FileLoaderSystem::cast_resource_id(resource_id),
                path: std::path::PathBuf::from(path),
            },
        }
    }

    /// Creates an event to register a custom ECS component
    pub fn register_custom_component<T: crate::Component>() -> Self {
        Self {
            event: UserEvent::RegisterCustomComponent {
                register_fct: T::register,
            },
        }
    }
}

impl<'a> ApplicationSystem<'a> {
    /// User events handling
    /// Returns true if the application should quit
    pub(crate) fn handle_user_events(
        &mut self,
        events: VecDeque<UserEventBuilder>,
        _platform_layer: &mut PlatformLayerImpl,
        _rendering_layer: &mut RenderingLayerImpl,
    ) -> Result<bool, ErrorType> {
        let mut should_quit = false;
        for event_builder in &events {
            match &event_builder.event {
                UserEvent::QuitApp => {
                    should_quit = true;
                }
                UserEvent::RegisterCustomFileResource {
                    resource_id,
                    loader_fct,
                } => {
                    if let Err(err) = self.file_loader.register(resource_id, *loader_fct) {
                        log_error!(
                            "Failed to register the custom `{:?}' resource when handling a `RegisterCustomFileResource' event in the application: {:?}",
                            resource_id,
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("Custom resource `{:?}' registered", resource_id);
                }
                UserEvent::StartLoadCustomFileResource { resource_id, path } => {
                    if let Err(err) = self.file_loader.start_load(resource_id, path) {
                        log_error!(
                            "Failed to start loading the custom `{:?}' resource at `{:?}' when handling a `StartLoadCustomFileResource' event in the application: {:?}",
                            resource_id,
                            path,
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!(
                        "Start loading the custom `{:?}' resource at `{:?}'",
                        resource_id,
                        path
                    );
                }
                UserEvent::RegisterCustomComponent { register_fct } => {
                    if let Err(err) = register_fct(&mut self.ecs.components) {
                        log_error!(
                            "Failed to register a custom component when handling a `RegisterCustomComponent' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("Custom component registered")
                }
            }
        }

        Ok(should_quit)
    }
}
