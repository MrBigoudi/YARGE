#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::core_layer::application_system::ecs::entity::UserEntity;
use crate::core_layer::application_system::ecs::resource::UserResourceId;

/// An enum representing user fireable events
pub(crate) enum UserEvent {
    /// To close the application
    QuitApp,

    /// To register a new resource
    RegisterCustomResource {
        user_id: UserResourceId,
        resource_type_id: std::any::TypeId,
        loading_function:
            crate::core_layer::application_system::ecs::resource::ResourceLoadingFunction,
    },

    /// To begin loading a new resource
    StartLoadCustomResource {
        user_id: UserResourceId,
        resource_type_id: std::any::TypeId,
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

    /// Registers a new system
    RegisterSystem {
        /// The name of the main component the system will run on
        system: Box<dyn crate::SystemTrait>,
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
        _platform_layer: &mut crate::PlatformLayerImpl,
        _rendering_layer: &mut crate::RenderingLayerImpl<'_>,
    ) -> Result<bool, ErrorType> {
        let mut should_quit = false;
        while let Some(event_builder) = self.user_events.pop_front() {
            match event_builder.event {
                UserEvent::QuitApp => {
                    should_quit = true;
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
                    system,
                    schedule,
                    condition,
                } => {
                    if let Err(err) =
                        self.ecs
                            .register_system(self.user_game, system, schedule, condition)
                    {
                        log_error!(
                            "Failed to register a new system when handling a `RegisterSystem' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                    log_debug!("Added new system");
                }
                UserEvent::RegisterCustomResource {
                    user_id,
                    resource_type_id,
                    loading_function,
                } => {
                    if let Err(err) = self.ecs.register_custom_resource(
                        &user_id,
                        &resource_type_id,
                        loading_function,
                    ) {
                        log_error!(
                            "Failed to register a new resource when handling a `RegisterCustomResource' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                }
                UserEvent::StartLoadCustomResource {
                    user_id,
                    resource_type_id,
                } => {
                    if let Err(err) = self
                        .ecs
                        .try_load_custom_resource(&user_id, &resource_type_id)
                    {
                        log_error!(
                            "Failed to start loading a new resource when handling a `StartLoadCustomResource' event in the application: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                }
            }
        }

        Ok(should_quit)
    }
}
