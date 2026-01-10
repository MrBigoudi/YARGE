#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::core_layer::application_system::ecs::entity::UserEntity;

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
        user_entity: UserEntity,
    },

    /// To remove entities
    RemoveEntities {
        /// The entities to remove
        user_entities: Vec<UserEntity>,
    },

    /// To register a new component
    RegisterCustomComponent {
        /// The id of the component
        component_id: std::any::TypeId,
        /// The function to register the component
        register_fct:
            crate::core_layer::application_system::ecs::component::RegisterComponentFunction,
    },

    /// To remove a component
    /// When removed, all systems linked to this component are destroyed and need to be recreated even if this component is registered again later
    RemoveCustomComponent {
        /// The id of the component
        component_id: std::any::TypeId,
        /// The function to remove the component
        remove_fct:
            crate::core_layer::application_system::ecs::component::RemoveComponentFunction,
    },

    /// Adds a component to an entity
    AddComponentToEntity {
        /// The id of the component
        component_id: std::any::TypeId,
        /// The user entity to which add the component
        user_entity: UserEntity,
        /// The value of the component to add to the entity
        value: Box<dyn crate::core_layer::application_system::ecs::component::RealComponent>,
        /// The function to add a component to an entity
        add_to_entity_fct:
            crate::core_layer::application_system::ecs::component::AddComponentToEntityFunction,
    },

    /// Removes a component from an entity
    RemoveComponentFromEntity {
        /// The id of the component
        component_id: std::any::TypeId,
        /// The user entity to which add the component
        user_entity: UserEntity,
        /// The function to add a component to an entity
        remove_from_entity_fct:
            crate::core_layer::application_system::ecs::component::RemoveComponentFromEntityFunction,
    },

    /// Updates the value of a component for an entity
    UpdateComponentValueForEntity {
        /// The id of the component
        component_id: std::any::TypeId,
        /// The user entity which needs a component update
        user_entity: UserEntity,
        /// The new value of the component for the entity
        value: Box<dyn crate::core_layer::application_system::ecs::component::RealComponent>,
        /// The function to update a component for an entity
        update_for_entity_fct:
            crate::core_layer::application_system::ecs::component::UpdateComponentForEntityFunction,
    },

    // TODO: Add schedule + if condition
    RegisterSystem {
        /// The name of the main component the system will run on
        name: std::any::TypeId,
        /// A list of component the entity must have for this system to run on it
        with: Vec<std::any::TypeId>,
        /// A list of component the entity must not have for this system to run on it
        without: Vec<std::any::TypeId>,
        /// The system function
        callback: crate::core_layer::application_system::ecs::system::SystemCallback,
        /// The rate at which this system will be called
        schedule: crate::SystemSchedule,
        /// The condition function to run or not this system
        condition: crate::core_layer::application_system::ecs::system::SystemCallbackConditionFunction,
    },

    RegisterSystemMut {
        /// The name of the main component the system will run on
        name: std::any::TypeId,
        /// A list of component the entity must have for this system to run on it
        with: Vec<std::any::TypeId>,
        /// A list of component the entity must not have for this system to run on it
        without: Vec<std::any::TypeId>,
        /// The system function
        callback_mut: crate::core_layer::application_system::ecs::system::SystemMutCallback,
        /// The rate at which this system will be called
        schedule: crate::SystemSchedule,
        /// The condition function to run or not this system
        condition: crate::core_layer::application_system::ecs::system::SystemCallbackConditionFunction,
    },
}

/// A public Wrapper for UserEvent
pub struct UserEventWrapper {
    pub(crate) event: UserEvent,
}

impl crate::core_layer::application_system::application::ApplicationSystem<'_> {
    /// User events handling
    /// Returns true if the application should quit
    pub(crate) fn handle_user_events(
        &mut self,
        events: std::collections::VecDeque<UserEventWrapper>,
        _platform_layer: &mut crate::PlatformLayerImpl,
        _rendering_layer: &mut crate::RenderingLayerImpl,
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
                UserEvent::RegisterCustomComponent {
                    component_id,
                    register_fct,
                } => {
                    if let Err(err) = self.ecs.register_component(&component_id, &register_fct) {
                        log_error!(
                            "Failed to register a custom component when handling a `RegisterCustomComponent' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("Custom component registered");
                }
                UserEvent::RemoveCustomComponent {
                    component_id,
                    remove_fct,
                } => {
                    if let Err(err) = self.ecs.remove_component(&component_id, &remove_fct) {
                        log_error!(
                            "Failed to remove a custom component when handling a `RemoveCustomComponent' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("Custom component removed");
                }
                UserEvent::AddComponentToEntity {
                    component_id,
                    user_entity,
                    value,
                    add_to_entity_fct,
                } => {
                    if let Err(err) = self.ecs.add_component_to_entity(
                        &component_id,
                        &user_entity,
                        value,
                        &add_to_entity_fct,
                    ) {
                        log_error!(
                            "Failed to add a component to an entity when handling a `AddComponentToEntity' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("Added component to entity `{:?}'", user_entity);
                }
                UserEvent::RemoveComponentFromEntity {
                    component_id,
                    user_entity,
                    remove_from_entity_fct,
                } => {
                    if let Err(err) = self.ecs.remove_component_from_entity(
                        &component_id,
                        &user_entity,
                        &remove_from_entity_fct,
                    ) {
                        log_error!(
                            "Failed to remove a component from an entity when handling a `RemoveComponentFromEntity' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("Removed component to entity `{:?}'", user_entity);
                }
                UserEvent::UpdateComponentValueForEntity {
                    component_id,
                    user_entity,
                    value,
                    update_for_entity_fct,
                } => {
                    if let Err(err) = self.ecs.update_component_value_for_entity(
                        &component_id,
                        &user_entity,
                        value,
                        &update_for_entity_fct,
                    ) {
                        log_error!(
                            "Failed to update a component for an entity when handling a `UpdateComponentValueForEntity' event in the application: {:?}",
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
                    schedule,
                    condition,
                } => {
                    if let Err(err) = self
                        .ecs
                        .register_system(name, with, without, callback, schedule, condition)
                    {
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
                    schedule,
                    condition,
                } => {
                    if let Err(err) = self.ecs.register_system_mut(
                        name,
                        with,
                        without,
                        callback_mut,
                        schedule,
                        condition,
                    ) {
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
