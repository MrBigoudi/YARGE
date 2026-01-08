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

    /// To remove a single entity
    RemoveEntity {
        /// The entity to remove
        user_entity: crate::core_layer::UserEntity,
    },

    /// To remove entities
    RemoveEntities {
        /// The entities to remove
        user_entities: Vec<crate::core_layer::UserEntity>,
    },

    /// To register a new component
    RegisterCustomComponent {
        /// The function to register the component
        register_fct:
            crate::core_layer::application_system::ecs::component::RegisterComponentFunction,
    },

    /// To remove a component
    RemoveCustomComponent {
        /// The function to remove the component
        remove_fct:
            crate::core_layer::application_system::ecs::component::RemoveComponentFunction,
    },

    /// Adds a component to an entity
    AddComponentToEntity {
        /// The user entity to which add the component
        user_entity: crate::core_layer::UserEntity,
        /// The value of the component to add to the entity
        value: Box<dyn crate::core_layer::application_system::ecs::component::RealComponent>,
        /// The function to add a component to an entity
        add_to_entity_fct:
            crate::core_layer::application_system::ecs::component::AddComponentToEntityFunction,
    },

    /// Removes a component from an entity
    RemoveComponentFromEntity {
        /// The user entity to which add the component
        user_entity: crate::core_layer::UserEntity,
        /// The function to add a component to an entity
        remove_from_entity_fct:
            crate::core_layer::application_system::ecs::component::RemoveComponentFromEntityFunction,
    },

    /// Updates the value of a component for an entity
    UpdateComponentForEntity {
        /// The user entity which needs a component update
        user_entity: crate::core_layer::UserEntity,
        /// The new value of the component for the entity
        value: Box<dyn crate::core_layer::application_system::ecs::component::RealComponent>,
        /// The function to update a component for an entity
        update_for_entity_fct:
            crate::core_layer::application_system::ecs::component::UpdateComponentForEntityFunction,
    },

    // TODO: Add schedule + if condition
    RegisterSystem {
        name: std::any::TypeId,
        with: Vec<std::any::TypeId>,
        without: Vec<std::any::TypeId>,
        callback: crate::core_layer::application_system::ecs::system::SystemCallback,
    },

    RegisterSystemMut {
        name: std::any::TypeId,
        with: Vec<std::any::TypeId>,
        without: Vec<std::any::TypeId>,
        callback_mut: crate::core_layer::application_system::ecs::system::SystemMutCallback,
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

    /// Creates an event to remove a single entity from the ECS
    pub fn remove_entity(user_entity: crate::core_layer::UserEntity) -> Self {
        Self {
            event: UserEvent::RemoveEntity { user_entity },
        }
    }

    /// Creates an event to remove entities from the ECS
    pub fn remove_entities(user_entities: Vec<crate::core_layer::UserEntity>) -> Self {
        Self {
            event: UserEvent::RemoveEntities { user_entities },
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

    /// Creates an event to remove a custom ECS component
    pub fn remove_custom_component<T: crate::Component>() -> Self {
        Self {
            event: UserEvent::RemoveCustomComponent {
                remove_fct: T::remove,
            },
        }
    }

    /// Creates an event to add a component to an entity
    pub fn add_component_to_entity<T: crate::Component>(
        entity: crate::core_layer::UserEntity,
        value: T,
    ) -> Self {
        Self {
            event: UserEvent::AddComponentToEntity {
                user_entity: entity,
                value: Box::new(value),
                add_to_entity_fct: T::add_to_entity,
            },
        }
    }

    /// Creates an event to remove a component from an entity
    pub fn remove_component_from_entity<T: crate::Component>(
        entity: crate::core_layer::UserEntity,
    ) -> Self {
        Self {
            event: UserEvent::RemoveComponentFromEntity {
                user_entity: entity,
                remove_from_entity_fct: T::remove_from_entity,
            },
        }
    }

    /// Creates an event to update a component for an entity
    pub fn update_component_for_entity<T: crate::Component>(
        entity: crate::core_layer::UserEntity,
        value: T,
    ) -> Self {
        Self {
            event: UserEvent::UpdateComponentForEntity {
                user_entity: entity,
                value: Box::new(value),
                update_for_entity_fct: T::updates_for_entity,
            },
        }
    }

    /// Creates an event to register a new system in the ECS
    pub fn register_system<G, T, With, Without>(
        callback: crate::core_layer::application_system::ecs::system::UserSystemCallback<G, T>,
    ) -> Self
    where
        G: crate::Game + 'static,
        T: crate::Component + 'static,
        With: crate::core_layer::application_system::ecs::system::ComponentList,
        Without: crate::core_layer::application_system::ecs::system::ComponentList,
    {
        Self {
            event: UserEvent::RegisterSystem {
                name: T::get_type_id(),
                with: With::get_ids(),
                without: Without::get_ids(),
                callback:
                    crate::core_layer::application_system::ecs::system::UserSystemCallbackBuilder::system::<G, T>(callback),
            }
        }
    }

    /// Creates an event to register a new mutable system in the ECS
    pub fn register_system_mut<G, T, With, Without>(
        callback: crate::core_layer::application_system::ecs::system::UserSystemMutCallback<G, T>,
    ) -> Self
    where
        G: crate::Game + 'static,
        T: crate::Component + 'static,
        With: crate::core_layer::application_system::ecs::system::ComponentList,
        Without: crate::core_layer::application_system::ecs::system::ComponentList,
    {
        Self {
            event: UserEvent::RegisterSystemMut {
                name: T::get_type_id(),
                with: With::get_ids(),
                without: Without::get_ids(),
                callback_mut:
                    crate::core_layer::application_system::ecs::system::UserSystemCallbackBuilder::system_mut::<G, T>(callback),
            }
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
        // TODO: check if need to create new entities
        if let Err(err) = self.ecs.spawn_real_entities() {
            log_error!(
                "Failed to spawn entities in the ECS from the application layer: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        let mut should_quit = false;
        for event_builder in events {
            match event_builder.event {
                UserEvent::QuitApp => {
                    should_quit = true;
                }
                UserEvent::RegisterCustomFileResource {
                    resource_id,
                    loader_fct,
                } => {
                    if let Err(err) = self.file_loader.register(&resource_id, loader_fct) {
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
                    if let Err(err) = self.file_loader.start_load(&resource_id, &path) {
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
                UserEvent::RemoveEntity { user_entity } => {
                    if let Err(err) = self.ecs.remove_entity(&user_entity) {
                        log_error!(
                            "Failed to remove an entity when handling a `RemoveEntity' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("One entity removed");
                }
                UserEvent::RemoveEntities { user_entities } => {
                    if let Err(err) = self.ecs.remove_entities(&user_entities) {
                        log_error!(
                            "Failed to remove entities when handling a `RemoveEntities' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("`{:?}' entities removed", user_entities.len());
                }
                UserEvent::RegisterCustomComponent { register_fct } => {
                    if let Err(err) = self.ecs.register_component(&register_fct) {
                        log_error!(
                            "Failed to register a custom component when handling a `RegisterCustomComponent' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("Custom component registered");
                }
                UserEvent::RemoveCustomComponent { remove_fct } => {
                    if let Err(err) = self.ecs.remove_component(&remove_fct) {
                        log_error!(
                            "Failed to remove a custom component when handling a `RemoveCustomComponent' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("Custom component removed");
                }
                UserEvent::AddComponentToEntity {
                    user_entity,
                    value,
                    add_to_entity_fct,
                } => {
                    if let Err(err) =
                        self.ecs
                            .add_component_to_entity(&user_entity, value, &add_to_entity_fct)
                    {
                        log_error!(
                            "Failed to add a component to an entity when handling a `AddComponentToEntity' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("Added component to entity `{:?}'", user_entity);
                }
                UserEvent::RemoveComponentFromEntity {
                    user_entity,
                    remove_from_entity_fct,
                } => {
                    if let Err(err) = self
                        .ecs
                        .remove_component_from_entity(&user_entity, &remove_from_entity_fct)
                    {
                        log_error!(
                            "Failed to remove a component from an entity when handling a `RemoveComponentFromEntity' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("Removed component to entity `{:?}'", user_entity);
                }
                UserEvent::UpdateComponentForEntity {
                    user_entity,
                    value,
                    update_for_entity_fct,
                } => {
                    if let Err(err) = self.ecs.update_component_for_entity(
                        &user_entity,
                        value,
                        &update_for_entity_fct,
                    ) {
                        log_error!(
                            "Failed to update a component for an entity when handling a `UpdateComponentForEntity' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("Updated component for entity `{:?}'", user_entity);
                }

                UserEvent::RegisterSystem {
                    name,
                    with,
                    without,
                    callback,
                } => {
                    if let Err(err) = self.ecs.register_system(name, with, without, callback) {
                        log_error!(
                            "Failed to register a new system when handling a `RegisterSystem' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("Added new system");
                }
                UserEvent::RegisterSystemMut {
                    name,
                    with,
                    without,
                    callback_mut,
                } => {
                    if let Err(err) =
                        self.ecs
                            .register_system_mut(name, with, without, callback_mut)
                    {
                        log_error!(
                            "Failed to register a new system when handling a `RegisterSystemMut' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("Added new system mut");
                }
            }
        }

        if let Err(err) = self.run_systems() {
            log_error!("Failed to run the user systems: {:?}", err);
            return Err(ErrorType::Unknown);
        }

        Ok(should_quit)
    }
}
