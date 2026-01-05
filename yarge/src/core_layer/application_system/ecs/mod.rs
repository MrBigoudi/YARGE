/// A module representing generatioonal indices structures
/// See https://lucassardois.medium.com/generational-indices-guide-8e3c5f7fd594
pub mod generational;

pub mod component;
/// A module representing entities in the ECS
/// See https://austinmorlan.com/posts/entity_component_system/
/// See https://kyren.github.io/2018/09/14/rustconf-talk.html
pub mod entity;
pub mod system;

pub use component::Component;

use crate::error::ErrorType;

#[allow(clippy::upper_case_acronyms)]
/// An entity component system
pub struct ECS {
    /// Full of types like EntityMap<T>
    /// Each collections should be of the same size
    pub components: component::ComponentManager,

    /// The length of the components lists
    pub len: usize,

    /// The next free head in the components lists
    pub free_head: generational::GenerationalIndex,
}

impl ECS {
    /// Initialized the ECS
    pub fn init() -> Self {
        ECS {
            components: component::ComponentManager::init(),
            len: 0,
            free_head: 0,
        }
    }
}

// TODO: Add a static Entity generator which always contains the next entity's ID
// It should get the ID from the `free_head` inside the generational indices
// TODO: Find a way to keep the lists synced: i.e. always of the same size
