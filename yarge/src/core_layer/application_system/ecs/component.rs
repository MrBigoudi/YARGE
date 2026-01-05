use super::entity::Entity;
use crate::{error::ErrorType, log_error};

pub type ComponentMap<T> = super::generational::GenerationalVec<T>;

/// A component
pub trait Component: Send + Sized + 'static {
    /// Tries to register a component into a manager
    fn register(manager: &mut ComponentManager) -> Result<(), ErrorType> {
        if manager.components.contains::<ComponentMap<Self>>() {
            log_error!(
                "Failed to add the `{:?}' component to the ECS: the component already exists",
                std::any::type_name::<Self>()
            );
            return Err(ErrorType::WrongArgument(String::from(
                "Can't register a component multiple times",
            )));
        }
        manager.components.insert(ComponentMap::<Self>::new());
        Ok(())
    }

    /// Tries to add a component to an entity
    fn add_to_entity(manager: &mut ComponentManager, entity: Entity) -> Result<(), ErrorType> {
        todo!()
    }

    /// Tries to remove a component to an entity
    fn remove_from_entity(manager: &mut ComponentManager, entity: Entity) -> Result<(), ErrorType> {
        todo!()
    }
}

pub struct ComponentManager {
    components: anymap::AnyMap,
}

impl ComponentManager {
    /// Initialize the component manager
    pub fn init() -> Self {
        Self {
            components: anymap::AnyMap::new(),
        }
    }
}

pub(crate) type RegisterComponentFunction = fn(&mut ComponentManager) -> Result<(), ErrorType>;
