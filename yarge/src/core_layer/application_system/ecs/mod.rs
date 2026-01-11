/// A module representing generatioonal indices structures
/// See https://lucassardois.medium.com/generational-indices-guide-8e3c5f7fd594
pub(crate) mod generational;

/// A module representing components in the ECS
pub(crate) mod component;
/// A module representing entities in the ECS
/// See https://austinmorlan.com/posts/entity_component_system/
/// See https://kyren.github.io/2018/09/14/rustconf-talk.html
pub(crate) mod entity;
pub(crate) mod system;

pub(crate) mod engine;

#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

#[allow(clippy::upper_case_acronyms)]
/// An entity component system
pub struct ECS {
    /// The list of entities
    pub(crate) entities: Vec<entity::Entity>,

    /// Full of types like EntityMap<T>
    /// Each collections should be of the same size
    pub(crate) component_manager: component::ComponentManager,

    pub(crate) system_manager: system::SystemManager,
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

        let system_manager = system::SystemManager::init();

        Ok(ECS {
            entities: vec![],
            component_manager,
            system_manager,
        })
    }

    /// Shuts down the ECS
    pub(crate) fn shutdown() -> Result<(), ErrorType> {
        match entity::GLOBAL_ENTITY_GENERATOR.write() {
            Ok(mut generator) => {
                generator.shutdown();
                Ok(())
            }
            Err(err) => {
                log_error!(
                    "Failed to access the global entity generator when shutting down the ECS: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
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

    /// Creates real entities
    pub(crate) fn spawn_real_entities(&mut self) -> Result<(), ErrorType> {
        let nb_new_entities_to_spawn = match entity::GLOBAL_ENTITY_GENERATOR.read() {
            Ok(generator) => {
                generator.entity_to_generate.len()
                // log_warn!("Nb real entities to generate: {:?}", nb_entities);
            }
            Err(err) => {
                log_error!(
                    "Failed to access the global entity generator when spawning real entities in the ECS: {:?}",
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
                Ok(mut new_generated_entities) => {
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
                            if let Err(err) = generator.update_table(&new_generated_entities) {
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
                    self.entities.append(&mut new_generated_entities);
                }
            }
        }

        // log_info!("Spawned: {:?} entities", nb_new_entities_to_spawn);

        Ok(())
    }

    /// Removes an entity
    pub(crate) fn remove_entity(
        &mut self,
        user_entity: &entity::UserEntity,
    ) -> Result<(), ErrorType> {
        let real_entity = match entity::GLOBAL_ENTITY_GENERATOR.read() {
            Ok(generator) => match generator.get_real_entity(user_entity) {
                Ok(entity) => entity,
                Err(err) => {
                    log_error!(
                        "Failed to get the real entity from the entity generator when removing an entity in the ECS: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            },
            Err(err) => {
                log_error!(
                    "Failed to access the global entity generator when removing an entity in the ECS: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        if let Err(err) = self.component_manager.remove_entity(&real_entity) {
            log_error!(
                "Failed to remove an entity in the component manager: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        // Update systems
        self.system_manager.on_removed_entity(&real_entity);

        let indices_to_remove: Vec<usize> = self
            .entities
            .iter()
            .enumerate()
            .filter(|&(_, val)| *val == real_entity)
            .map(|(index, _)| index)
            .collect();
        for index in indices_to_remove.into_iter().rev() {
            let _ = self.entities.drain(index..index + 1);
        }

        Ok(())
    }

    /// Removes entities
    pub(crate) fn remove_entities(
        &mut self,
        user_entities: &[entity::UserEntity],
    ) -> Result<(), ErrorType> {
        if user_entities.is_empty() {
            log_warn!("Trying to remove 0 entities");
            return Ok(());
        }
        let real_entities = match entity::GLOBAL_ENTITY_GENERATOR.read() {
            Ok(generator) => match generator.get_real_entities(user_entities) {
                Ok(entities) => entities,
                Err(err) => {
                    log_error!(
                        "Failed to get the real entities from the entity generator when removing entities in the ECS: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            },
            Err(err) => {
                log_error!(
                    "Failed to access the global entity generator when removing entities in the ECS: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        if let Err(err) = self.component_manager.remove_entities(&real_entities) {
            log_error!(
                "Failed to remove entities in the component manager: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        // Update systems
        self.system_manager.on_removed_entities(&real_entities);

        let indices_to_remove: Vec<usize> = self
            .entities
            .iter()
            .enumerate()
            .filter(|&(_, val)| real_entities.contains(val))
            .map(|(index, _)| index)
            .collect();
        for index in indices_to_remove.into_iter().rev() {
            let _ = self.entities.drain(index..index + 1);
        }

        Ok(())
    }

    /// Registers a new component
    pub(crate) fn register_component(
        &mut self,
        _component_id: &std::any::TypeId,
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
        component_id: &std::any::TypeId,
        remove_fct: &component::RemoveComponentFunction,
    ) -> Result<(), ErrorType> {
        if let Err(err) = remove_fct(&mut self.component_manager) {
            log_error!("Failed to remove a component in the ECS: {:?}", err);
            return Err(ErrorType::Unknown);
        }
        self.system_manager.on_component_removed(component_id);

        Ok(())
    }

    pub(crate) fn add_component_to_entity(
        &mut self,
        _component_id: &std::any::TypeId,
        user_entity: &entity::UserEntity,
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
                    "Failed to access the global entity generator when adding a component to an entity in the ECS: {:?}",
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

        if let Err(err) = self
            .system_manager
            .on_component_changed_for_entity(&self.component_manager, &real_entity)
        {
            log_error!(
                "Failed to handle component changed in the system manager when adding a component to an entity in the ECS: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        Ok(())
    }

    pub(crate) fn remove_component_from_entity(
        &mut self,
        _component_id: &std::any::TypeId,
        user_entity: &entity::UserEntity,
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
                    "Failed to access the global entity generator when removing a component from an entity in the ECS: {:?}",
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

        if let Err(err) = self
            .system_manager
            .on_component_changed_for_entity(&self.component_manager, &real_entity)
        {
            log_error!(
                "Failed to handle component changed in the system manager when removing a component from an entity in the ECS: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        Ok(())
    }

    pub(crate) fn update_component_value_for_entity(
        &mut self,
        _component_id: &std::any::TypeId,
        user_entity: &entity::UserEntity,
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
                    "Failed to access the global entity generator when updating a component for an entity in the ECS: {:?}",
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

    pub(crate) fn register_system(
        &mut self,
        name: std::any::TypeId,
        with: Vec<std::any::TypeId>,
        without: Vec<std::any::TypeId>,
        callback: system::SystemCallback,
        schedule: crate::SystemSchedule,
        condition: system::SystemCallbackConditionFunction,
    ) -> Result<(), ErrorType> {
        // Check if all types are valid component
        if !self.component_manager.is_registered(&name) {
            log_error!(
                "Failed to register a new system in the ECS: the component `{:?}' is not registered",
                name
            );
            return Err(ErrorType::DoesNotExist);
        }
        for type_id in &with {
            if !self.component_manager.is_registered(type_id) {
                log_error!(
                    "Failed to register a new system in the ECS: the component `{:?}' used in the `With' is not registered",
                    type_id
                );
                return Err(ErrorType::DoesNotExist);
            }
        }
        for type_id in &without {
            if !self.component_manager.is_registered(type_id) {
                log_error!(
                    "Failed to register a new system in the ECS: the component `{:?}' used in the `Without' is not registered",
                    type_id
                );
                return Err(ErrorType::DoesNotExist);
            }
        }

        // Create a new system
        let internal = system::SystemInternal::new(name, &with, &without, schedule, condition);
        if let Err(err) = self.system_manager.register_new_system_ref(
            internal,
            callback,
            &self.component_manager,
            &self.entities,
        ) {
            log_error!("Failed to register a new system in the ECS: {:?}", err);
            return Err(ErrorType::Unknown);
        }

        Ok(())
    }

    pub(crate) fn register_system_mut(
        &mut self,
        name: std::any::TypeId,
        with: Vec<std::any::TypeId>,
        without: Vec<std::any::TypeId>,
        callback: system::SystemMutCallback,
        schedule: crate::SystemSchedule,
        condition: system::SystemCallbackConditionFunction,
    ) -> Result<(), ErrorType> {
        // Check if all types are valid component
        if !self.component_manager.is_registered(&name) {
            log_error!(
                "Failed to register a new system in the ECS: the component `{:?}' is not registered",
                name
            );
            return Err(ErrorType::DoesNotExist);
        }
        for type_id in &with {
            if !self.component_manager.is_registered(type_id) {
                log_error!(
                    "Failed to register a new mut system in the ECS: the component `{:?}' used in the `With' is not registered",
                    type_id
                );
                return Err(ErrorType::DoesNotExist);
            }
        }
        for type_id in &without {
            if !self.component_manager.is_registered(type_id) {
                log_error!(
                    "Failed to register a new mut system in the ECS: the component `{:?}' used in the `Without' is not registered",
                    type_id
                );
                return Err(ErrorType::DoesNotExist);
            }
        }

        // Create a new system
        let internal = system::SystemInternal::new(name, &with, &without, schedule, condition);
        if let Err(err) = self.system_manager.register_new_system_mut(
            internal,
            callback,
            &self.component_manager,
            &self.entities,
        ) {
            log_error!("Failed to register a new mut system in the ECS: {:?}", err);
            return Err(ErrorType::Unknown);
        }

        Ok(())
    }
}
