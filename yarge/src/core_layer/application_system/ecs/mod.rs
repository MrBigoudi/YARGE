/// A module representing generatioonal indices structures
/// See https://lucassardois.medium.com/generational-indices-guide-8e3c5f7fd594
pub mod generational;

/// A module representing components in the ECS
pub mod component;
/// A module representing entities in the ECS
/// See https://austinmorlan.com/posts/entity_component_system/
/// See https://kyren.github.io/2018/09/14/rustconf-talk.html
pub mod entity;
pub mod query;
pub mod system;

pub use component::Component;
pub use entity::UserEntity;
pub use query::Query;

use crate::error::ErrorType;

#[allow(unused)]
use crate::{log_error, log_info, log_warn};

#[allow(clippy::upper_case_acronyms)]
/// An entity component system
pub struct ECS {
    /// Full of types like EntityMap<T>
    /// Each collections should be of the same size
    pub(crate) component_manager: component::ComponentManager,
}

impl ECS {
    /// Initialized the ECS
    pub(crate) fn init() -> Result<Self, ErrorType> {
        let component_manager = match component::ComponentManager::init() {
            Ok(manager) => manager,
            Err(err) => {
                log_error!(
                    "Failed to initialize the ComponentManager when initializing the ECS system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        Ok(ECS { component_manager })
    }

    /// Creates empty entities
    /// This method is for the User
    pub fn spawn_empty_entities(nb_entities: usize) -> Result<Vec<entity::UserEntity>, ErrorType> {
        match entity::GLOBAL_ENTITY_GENERATOR.write() {
            Ok(mut generator) => Ok(generator.spawn_empty_entities(nb_entities)),
            Err(err) => {
                log_error!(
                    "Failed to access the global entity generator when spawning user entities in the ECS: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
    }

    /// Creates a new query
    pub fn generate_queries(nb_entities: usize) -> Result<Vec<Query>, ErrorType> {
        match query::GLOBAL_QUERY_GENERATOR.write() {
            Ok(mut generator) => Ok(generator.generate_queries(nb_entities)),
            Err(err) => {
                log_error!(
                    "Failed to access the global query generator when generating new queries in the ECS: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
    }

    /// Creates real entities
    pub(crate) fn spawn_real_entities(&mut self) -> Result<(), ErrorType> {
        let nb_new_entities_to_spawn = match entity::GLOBAL_ENTITY_GENERATOR.read() {
            Ok(generator) => {
                generator.entity_to_generate.len()
                // log_warn!("Nb real entities to generate: {:?}", nb_entities);
            }
            Err(err) => {
                log_error!(
                    "Failed to access the global the entity generator when spawning real entities in the ECS: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        if nb_new_entities_to_spawn > 0 {
            match self
                .component_manager
                .spawn_empty_entities(nb_new_entities_to_spawn)
            {
                Err(err) => {
                    log_error!(
                        "Failed to spawn entities in the component manager: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
                Ok(new_generated_entities) => {
                    // Sync entities with the GLOBAL_ENTITY_GENERATOR
                    if new_generated_entities.len() != nb_new_entities_to_spawn {
                        log_error!(
                            "Wrong number of generated entities when spawning real entities in the ECS"
                        );
                        return Err(ErrorType::WrongArgument(format!(
                            "Expected: {:?}, got: {:?}",
                            nb_new_entities_to_spawn,
                            new_generated_entities.len()
                        )));
                    }
                    match entity::GLOBAL_ENTITY_GENERATOR.write() {
                        Ok(mut generator) => {
                            if let Err(err) = generator.update_table(new_generated_entities) {
                                log_error!(
                                    "Failed to update the global entity generator table: {:?}",
                                    err
                                );
                                return Err(ErrorType::Unknown);
                            }
                        }
                        Err(err) => {
                            log_error!(
                                "Failed to access the global entity generator when updating the user entities to real entities table in the ECS: {:?}",
                                err
                            );
                            return Err(ErrorType::Unknown);
                        }
                    }
                }
            }
        }

        // log_info!("Spawned: {:?} entities", nb_new_entities_to_spawn);

        Ok(())
    }

    /// Register a new component
    pub(crate) fn register_component(
        &mut self,
        register_fct: &component::RegisterComponentFunction,
    ) -> Result<(), ErrorType> {
        if let Err(err) = register_fct(&mut self.component_manager) {
            log_error!("Failed to register a new component in the ECS: {:?}", err);
            return Err(ErrorType::Unknown);
        }

        Ok(())
    }

    /// Removes a component
    pub(crate) fn remove_component(
        &mut self,
        remove_fct: &component::RemoveComponentFunction,
    ) -> Result<(), ErrorType> {
        if let Err(err) = remove_fct(&mut self.component_manager) {
            log_error!("Failed to remove a component in the ECS: {:?}", err);
            return Err(ErrorType::Unknown);
        }

        Ok(())
    }

    pub(crate) fn add_component_to_entity(
        &mut self,
        user_entity: &UserEntity,
        value: Box<dyn component::RealComponent>,
        add_to_entity_fct: &component::AddComponentToEntityFunction,
    ) -> Result<(), ErrorType> {
        let real_entity = match entity::GLOBAL_ENTITY_GENERATOR.read() {
            Ok(generator) => match generator.get_real_entity(user_entity) {
                Ok(entity) => entity,
                Err(err) => {
                    log_error!(
                        "Failed to get the real entity from the global entity generator when adding a component to an entity in the ECS: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            },
            Err(err) => {
                log_error!(
                    "Failed to access the global the entity generator when adding a component to an entity in the ECS: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        if let Err(err) = add_to_entity_fct(&mut self.component_manager, &real_entity, value) {
            log_error!(
                "Failed to add a component to an entity in the ECS: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        Ok(())
    }

    pub(crate) fn remove_component_from_entity(
        &mut self,
        user_entity: &UserEntity,
        remove_from_entity: &component::RemoveComponentFromEntityFunction,
    ) -> Result<(), ErrorType> {
        let real_entity = match entity::GLOBAL_ENTITY_GENERATOR.read() {
            Ok(generator) => match generator.get_real_entity(user_entity) {
                Ok(entity) => entity,
                Err(err) => {
                    log_error!(
                        "Failed to get the real entity from the global entity generator when removing a component from an entity in the ECS: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            },
            Err(err) => {
                log_error!(
                    "Failed to access the global the entity generator when removing a component from an entity in the ECS: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        if let Err(err) = remove_from_entity(&mut self.component_manager, &real_entity) {
            log_error!(
                "Failed to remove a component to an entity in the ECS: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        Ok(())
    }

    pub(crate) fn update_component_for_entity(
        &mut self,
        user_entity: &UserEntity,
        value: Box<dyn component::RealComponent>,
        update_for_entity_fct: &component::UpdateComponentForEntityFunction,
    ) -> Result<(), ErrorType> {
        let real_entity = match entity::GLOBAL_ENTITY_GENERATOR.read() {
            Ok(generator) => match generator.get_real_entity(user_entity) {
                Ok(entity) => entity,
                Err(err) => {
                    log_error!(
                        "Failed to get the real entity from the global entity generator when updating a component for an entity in the ECS: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            },
            Err(err) => {
                log_error!(
                    "Failed to access the global the entity generator when updating a component for an entity in the ECS: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        if let Err(err) = update_for_entity_fct(&mut self.component_manager, &real_entity, value) {
            log_error!(
                "Failed to update a component for an entity in the ECS: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        Ok(())
    }

    pub(crate) fn get_component_value_from_entity(
        &mut self,
        user_entity: &UserEntity,
        get_value_for_entity_fct: &component::GetComponentValueForEntityFunction,
    ) -> Result<Box<dyn component::RealComponent>, ErrorType> {
        let real_entity = match entity::GLOBAL_ENTITY_GENERATOR.read() {
            Ok(generator) => match generator.get_real_entity(user_entity) {
                Ok(entity) => entity,
                Err(err) => {
                    log_error!(
                        "Failed to get the real entity from the global entity generator when querying a component value from an entity in the ECS: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            },
            Err(err) => {
                log_error!(
                    "Failed to access the global the entity generator when querying a component value from an entity in the ECS: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        match get_value_for_entity_fct(&mut self.component_manager, &real_entity) {
            Err(err) => {
                log_error!(
                    "Failed to query a component value from an entity in the ECS: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
            Ok(value) => Ok(value),
        }
    }
}
