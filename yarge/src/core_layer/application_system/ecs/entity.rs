#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A real entity in the ECS system
pub(crate) struct Entity(pub(crate) super::generational::GenerationalKey);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A user defined entity which does not necessarily match the real entity's id
pub struct UserEntity(pub(crate) super::generational::GenerationalKey);

/// A static entity generator
pub(crate) struct EntityGenerator {
    /// A hash map to map user's entity id to real entity ids
    pub(crate) table: HashMap<UserEntity, Entity>,
    /// A list of not yet created entities
    pub(crate) entity_to_generate: Vec<UserEntity>,
    /// The total number of created entities
    pub(crate) nb_entities_total: usize,
    /// The current generatrion of entities
    pub(crate) generation: super::generational::GenerationalGeneration,
}

impl EntityGenerator {
    /// Creates a new generator
    pub(crate) fn init() -> Self {
        Self {
            table: HashMap::new(),
            entity_to_generate: vec![],
            nb_entities_total: 0,
            generation: 0,
        }
    }

    /// Creates empty entities
    pub(crate) fn spawn_empty_entities(&mut self, nb_entities: usize) -> Vec<UserEntity> {
        let mut new_entities = vec![];
        for _ in 0..nb_entities {
            match self.nb_entities_total.checked_add(1) {
                Some(res) => self.nb_entities_total = res,
                None => {
                    self.nb_entities_total = 0;
                    self.generation += 1;
                }
            }
            let new_entity = UserEntity(super::generational::GenerationalKey {
                index: self.nb_entities_total,
                generation: self.generation,
            });
            self.entity_to_generate.push(new_entity);
            new_entities.push(new_entity);
        }
        new_entities
    }

    /// Gets the real entity given the UserEntity
    pub(crate) fn get_real_entity(&self, user_entity: &UserEntity) -> Result<Entity, ErrorType> {
        match self.table.get(user_entity) {
            Some(entity) => Ok(*entity),
            None => {
                log_error!("Failed to retrieve the real entity from user entity");
                Err(ErrorType::DoesNotExist)
            }
        }
    }

    /// Gets the real entities given a list of UserEntity
    pub(crate) fn get_real_entities(
        &self,
        user_entity: &[UserEntity],
    ) -> Result<Vec<Entity>, ErrorType> {
        let mut output = Vec::with_capacity(user_entity.len());
        for entity in user_entity {
            output.push(self.get_real_entity(entity)?);
        }
        Ok(output)
    }

    /// Updates the user entities to real entities table
    pub(crate) fn update_table(&mut self, real_entities: &[Entity]) -> Result<(), ErrorType> {
        if self.entity_to_generate.len() != real_entities.len() {
            log_error!(
                "Failed to update the user entities to real entities table in the entity generator: lengths do not match"
            );
            return Err(ErrorType::WrongArgument(format!(
                "New user entities length: `{:?}', new real entities length: `{:?}'",
                self.entity_to_generate.len(),
                real_entities.len(),
            )));
        }

        for (i, real_entity) in real_entities.iter().enumerate() {
            let user_entity = self.entity_to_generate[i];
            if self.table.insert(user_entity, *real_entity).is_some() {
                log_error!(
                    "Failed to update the user entities to real entities table in the entity generator"
                );
                return Err(ErrorType::WrongArgument(format!(
                    "The user key: `{:?}' already exists so the real entity: `{:?}' can't be matched",
                    user_entity, real_entity
                )));
            }
        }

        self.entity_to_generate.clear();

        Ok(())
    }

    pub(crate) fn shutdown(&mut self) {
        self.table = HashMap::new();
        self.entity_to_generate = vec![];
    }
}

use crate::platform_layer::platform_impl::PlatformLayerRwLock;

/// The global entity generator to interface between user request and real entities
pub(crate) static GLOBAL_ENTITY_GENERATOR: once_cell::sync::Lazy<
    PlatformLayerRwLock<EntityGenerator>,
> = once_cell::sync::Lazy::new(|| PlatformLayerRwLock::new(EntityGenerator::init()));
