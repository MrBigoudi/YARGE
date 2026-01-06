use super::entity::Entity;

#[allow(unused)]
use crate::{error::ErrorType, log_error, log_info, log_warn};

pub type ComponentMap<T> = super::generational::GenerationalVec<T>;

pub trait ComponentStorage {
    fn type_name(&self) -> &'static str;
    fn len(&self) -> usize;
    fn insert_empty_entities(
        &mut self,
        nb_entities: usize,
    ) -> Result<Option<Vec<Entity>>, ErrorType>;
}

impl<T: Component> ComponentStorage for ComponentMap<T> {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn insert_empty_entities(
        &mut self,
        nb_entities: usize,
    ) -> Result<Option<Vec<Entity>>, ErrorType> {
        self.insert_empty_entries(nb_entities, T::IS_DEFAULT)
    }
}

/// A component
pub trait Component: Send + Sized + 'static {
    /// Tells if this component is a default component
    /// Should not be used by the user
    const IS_DEFAULT: bool = false;

    /// Tries to register a component type into a manager
    fn register(manager: &mut ComponentManager) -> Result<(), ErrorType> {
        let type_id = std::any::TypeId::of::<Self>();
        if manager.component_storages.contains_key(&type_id) {
            log_error!(
                "Failed to add the `{:?}' component to the ECS: the component already exists",
                std::any::type_name::<Self>()
            );
            return Err(ErrorType::WrongArgument(String::from(
                "Can't register a component multiple times",
            )));
        }

        // Creates the real data in the hashmap
        match ComponentMap::<Self>::init_filled_with_empty_entries(manager.length) {
            Ok(new_map) => {
                manager
                    .component_storages
                    .insert(type_id, Box::new(new_map));
            }
            Err(err) => {
                log_error!(
                    "Failed to initiate a new ComponentMap when registering a new component: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        }

        // log_warn!("Component `{:?}' registered", std::any::type_name::<Self>());

        Ok(())
    }

    /// Tries to add a component to an entity
    fn add_to_entity(
        self,
        manager: &mut ComponentManager,
        entity: Entity,
    ) -> Result<(), ErrorType> {
        // TODO:
        todo!()
    }

    /// Tries to remove a component to an entity
    fn remove_from_entity(manager: &mut ComponentManager, entity: Entity) -> Result<(), ErrorType> {
        // TODO:
        todo!()
    }
}

/// A default component used to query info on all the other components
pub struct DefaultComponent;
impl Component for DefaultComponent {
    const IS_DEFAULT: bool = true;
}

/// A struct to manage components
pub struct ComponentManager {
    /// The real components storages
    pub component_storages: std::collections::HashMap<std::any::TypeId, Box<dyn ComponentStorage>>,
    /// The common length for all components storages
    pub length: usize,
}

impl ComponentManager {
    /// Initializes the component manager
    pub fn init() -> Result<Self, ErrorType> {
        let mut new_manager = Self {
            component_storages: std::collections::HashMap::new(),
            length: 0,
        };
        if let Err(err) = DefaultComponent::register(&mut new_manager) {
            log_error!(
                "Failed to register the DefaultComponent when initializing the ComponentManager: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        Ok(new_manager)
    }

    /// Checks the common length of each component maps
    pub fn check_length(&mut self) -> Result<(), ErrorType> {
        let default_component_id = std::any::TypeId::of::<DefaultComponent>();
        self.length = match self.component_storages.get(&default_component_id) {
            Some(default_component) => default_component.len(),
            None => {
                log_error!(
                    "Failed to find the DefaultComponent in the component manager when checking the lengths"
                );
                return Err(ErrorType::DoesNotExist);
            }
        };

        for component_storage in self.component_storages.values() {
            let found_length = component_storage.len();
            if found_length != self.length {
                log_error!(
                    "The component storage `{:?}' doesn't have the correct length",
                    component_storage.type_name()
                );
                return Err(ErrorType::WrongArgument(format!(
                    "Found `{:?}', expecting: `{:?}'",
                    found_length, self.length
                )));
            }
        }

        Ok(())
    }

    /// Adds empty entities to every components
    pub fn spawn_empty_entities(&mut self, nb_entities: usize) -> Result<Vec<Entity>, ErrorType> {
        let mut new_entities = vec![];
        for component_storage in self.component_storages.values_mut() {
            match component_storage.insert_empty_entities(nb_entities) {
                Err(err) => {
                    log_error!(
                        "Failed to insert empty entities into the `{:?}' component: {:?}",
                        component_storage.type_name(),
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
                Ok(Some(entities)) => new_entities = entities,
                _ => {}
            }
        }

        if let Err(err) = self.check_length() {
            log_error!(
                "Failed to check the length of components maps when spawning empty entities in the components: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        Ok(new_entities)
    }
}

pub(crate) type RegisterComponentFunction = fn(&mut ComponentManager) -> Result<(), ErrorType>;
