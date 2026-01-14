#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use std::collections::HashMap;
use crate::core_layer::application_system::ecs::generational::{GenerationalGeneration, GenerationalKey};


/// A GPU buffer is just an ID
/// The underlying buffer generation and buffer management is left to the rendering API
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct BufferId(GenerationalKey);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct UserBufferId(GenerationalKey);

pub(crate) struct BufferIdGenerator {
    pub(crate) table: HashMap<UserBufferId, BufferId>,
    pub(crate) inv_table: HashMap<BufferId, UserBufferId>,
    pub(crate) nb_ids_total: usize,
    pub(crate) generation: GenerationalGeneration,
}

impl BufferIdGenerator {
    /// Creates a new generator
    pub(crate) fn init() -> Self {
        Self {
            table: HashMap::new(),
            inv_table: HashMap::new(),
            nb_ids_total: 0,
            generation: 0,
        }
    }

    pub(crate) fn generate_id(&mut self) -> UserBufferId {
        match self.nb_ids_total.checked_add(1) {
            Some(res) => self.nb_ids_total = res,
            None => {
                self.nb_ids_total = 0;
                self.generation += 1;
            }
        }
        UserBufferId(GenerationalKey {
            index: self.nb_ids_total,
            generation: self.generation,
        })
    }

    pub(crate) fn get_real_id(&self, id: &UserBufferId) -> Result<BufferId, ErrorType> {
        match self.table.get(id) {
            Some(real_id) => Ok(*real_id),
            None => {
                log_error!("Failed to retrieve the real buffer id from user buffer id");
                Err(ErrorType::DoesNotExist)
            }
        }
    }

    pub(crate) fn get_user_id(&self, id: &BufferId) -> Result<UserBufferId, ErrorType> {
        match self.inv_table.get(id) {
            Some(user_id) => Ok(*user_id),
            None => {
                log_error!("Failed to retrieve the user buffer id from real buffer id");
                Err(ErrorType::DoesNotExist)
            }
        }
    }

    pub(crate) fn get_real_ids(
        &self,
        ids: &[UserBufferId],
    ) -> Result<Vec<BufferId>, ErrorType> {
        let mut output = Vec::with_capacity(ids.len());
        for id in ids {
            output.push(self.get_real_id(id)?);
        }
        Ok(output)
    }

    pub(crate) fn insert(
        &mut self,
        id: &UserBufferId,
        real_id: &BufferId,
    ) -> Result<(), ErrorType> {
        if self.table.insert(*id, *real_id).is_some() {
            log_error!("Failed to add a new key in the buffer id table");
            return Err(ErrorType::Duplicate);
        }
        Ok(())
    }

    pub(crate) fn insert_inv(
        &mut self,
        id: &BufferId,
        user_id: &UserBufferId,
    ) -> Result<(), ErrorType> {
        if self.inv_table.insert(*id, *user_id).is_some() {
            log_error!("Failed to add a new key in the inverse buffer id table");
            return Err(ErrorType::Duplicate);
        }
        Ok(())
    }
}