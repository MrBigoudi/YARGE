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
    fn add_to_entity(
        &mut self,
        entity: &Entity,
        value: Box<dyn RealComponent>,
    ) -> Result<(), ErrorType>;
    fn remove_from_entity(&mut self, entity: &Entity) -> Result<(), ErrorType>;
    fn updates_for_entity(
        &mut self,
        entity: &Entity,
        value: Box<dyn RealComponent>,
    ) -> Result<(), ErrorType>;
    fn get_value_from_entity(
        &mut self,
        entity: &Entity,
    ) -> Result<Box<dyn RealComponent>, ErrorType>;
    fn remove_entity(&mut self, entity: &Entity) -> Result<(), ErrorType>;
    fn get(&self, entity: &Entity) -> Result<Option<&dyn RealComponent>, ErrorType>;
    fn get_mut(&mut self, entity: &Entity) -> Result<Option<&mut dyn RealComponent>, ErrorType>;
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

    fn add_to_entity(
        &mut self,
        entity: &Entity,
        value: Box<dyn RealComponent>,
    ) -> Result<(), ErrorType> {
        let new_value = match value.into_any().downcast::<T>() {
            Ok(value) => value,
            Err(err) => {
                log_error!(
                    "Failed to downcast a value when adding the `{:?}' component to an entity in a component storage: {:?}",
                    std::any::type_name::<Self>(),
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        match self.get_mut_entry(entity) {
            Ok(entry) => {
                if let super::generational::Entry::Free { .. } = entry {
                    log_error!(
                        "Failed to get a non free entity when adding a component to an entity in a component storage"
                    );
                    return Err(ErrorType::DoesNotExist);
                }

                if let super::generational::Entry::Occupied { value: Some(..) } = entry {
                    log_error!(
                        "Failed to get a non occupied entity when adding a component to an entity in a component storage"
                    );
                    return Err(ErrorType::Unknown);
                }

                *entry = super::generational::Entry::Occupied {
                    value: Some(*new_value),
                };
            }
            Err(err) => {
                log_error!(
                    "Failed to get an entity when adding a component to an entity in a component storage: {:?}",
                    err
                );
                return Err(ErrorType::DoesNotExist);
            }
        }

        Ok(())
    }

    fn remove_from_entity(&mut self, entity: &Entity) -> Result<(), ErrorType> {
        match self.get_mut_entry(entity) {
            Ok(entry) => {
                if let super::generational::Entry::Free { .. } = entry {
                    log_error!(
                        "Failed to get a non free entity when removing a component to an entity in a component storage"
                    );
                    return Err(ErrorType::DoesNotExist);
                }

                if let super::generational::Entry::Occupied { value: None } = entry {
                    log_error!(
                        "Failed to get a non empty entity when removing a component to an entity in a component storage"
                    );
                    return Err(ErrorType::Unknown);
                }

                *entry = super::generational::Entry::Occupied { value: None };
            }
            Err(err) => {
                log_error!(
                    "Failed to get an entity when removing a component to an entity in a component storage: {:?}",
                    err
                );
                return Err(ErrorType::DoesNotExist);
            }
        }

        Ok(())
    }

    fn updates_for_entity(
        &mut self,
        entity: &Entity,
        value: Box<dyn RealComponent>,
    ) -> Result<(), ErrorType> {
        let new_value = match value.into_any().downcast::<T>() {
            Ok(value) => value,
            Err(err) => {
                log_error!(
                    "Failed to downcast a value when updating the `{:?}' component for an entity in a component storage: {:?}",
                    std::any::type_name::<Self>(),
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        match self.get_mut_entry(entity) {
            Ok(entry) => {
                if let super::generational::Entry::Free { .. } = entry {
                    log_error!(
                        "Failed to get a non free entity when updating a component for an entity in a component storage"
                    );
                    return Err(ErrorType::DoesNotExist);
                }

                if let super::generational::Entry::Occupied { value: None } = entry {
                    log_error!(
                        "Failed to get a non empty entity when updating a component for an entity in a component storage"
                    );
                    return Err(ErrorType::Unknown);
                }

                *entry = super::generational::Entry::Occupied {
                    value: Some(*new_value),
                };
            }
            Err(err) => {
                log_error!(
                    "Failed to get an entity when updating a component for an entity in a component storage: {:?}",
                    err
                );
                return Err(ErrorType::DoesNotExist);
            }
        }

        Ok(())
    }

    fn get_value_from_entity(
        &mut self,
        entity: &Entity,
    ) -> Result<Box<dyn RealComponent>, ErrorType> {
        match self.get_entry(entity) {
            Ok(super::generational::Entry::Occupied { value: Some(v) }) => Ok(Box::new(v.clone())),
            Ok(_) => {
                log_error!(
                    "Failed to find a non empty value for an entity when querying the component's value of an entity in a component storage"
                );
                Err(ErrorType::DoesNotExist)
            }
            Err(err) => {
                log_error!(
                    "Failed to get an entity when querying the component's value of an entity in a component storage: {:?}",
                    err
                );
                Err(ErrorType::DoesNotExist)
            }
        }
    }

    fn remove_entity(&mut self, entity: &Entity) -> Result<(), ErrorType> {
        if let Err(err) = self.remove(entity) {
            log_error!(
                "Failed to remove an entity in a component storage: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
        Ok(())
    }

    fn get(&self, entity: &Entity) -> Result<Option<&dyn RealComponent>, ErrorType> {
        match self.get_value(entity) {
            Ok(None) => Ok(None),
            Ok(Some(value)) => {
                let component: &dyn RealComponent = value;
                Ok(Some(component))
            }
            Err(err) => {
                log_error!(
                    "Failed to get the value when getting a storage value: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
    }

    fn get_mut(&mut self, entity: &Entity) -> Result<Option<&mut dyn RealComponent>, ErrorType> {
        match self.get_mut_value(entity) {
            Ok(None) => Ok(None),
            Ok(Some(value)) => {
                let component: &mut dyn RealComponent = value;
                Ok(Some(component))
            }
            Err(err) => {
                log_error!(
                    "Failed to get the value when getting a storage value: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
    }
}

pub trait RealComponent: Send + 'static {
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: Component> RealComponent for T {
    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// A component
pub trait Component: std::any::Any + Clone + Send + Sized + 'static {
    /// Tells if this component is a default component
    /// Should not be used by the user
    const IS_DEFAULT: bool = false;

    /// Registers a component type into a manager
    fn register(manager: &mut ComponentManager) -> Result<(), ErrorType> {
        let type_id = Self::get_type_id();

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

    /// Removes a component type from the manager
    fn remove(manager: &mut ComponentManager) -> Result<(), ErrorType> {
        let type_id = Self::get_type_id();

        if manager.component_storages.remove(&type_id).is_none() {
            log_error!(
                "Failed to remove the `{:?}' component to the ECS: the component doesn't exist",
                std::any::type_name::<Self>()
            );
            return Err(ErrorType::DoesNotExist);
        }

        log_warn!("Component `{:?}' removed", std::any::type_name::<Self>());

        Ok(())
    }

    /// Gets the type id of the component
    fn get_type_id() -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }

    /// Adds a component to an entity
    fn add_to_entity(
        manager: &mut ComponentManager,
        entity: &Entity,
        value: Box<dyn RealComponent>,
    ) -> Result<(), ErrorType> {
        let type_id = Self::get_type_id();

        match manager.component_storages.get_mut(&type_id) {
            Some(storage) => {
                if let Err(err) = storage.add_to_entity(entity, value) {
                    log_error!(
                        "Failed to add the `{:?}' component to an entity: {:?}",
                        std::any::type_name::<Self>(),
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            None => {
                log_error!(
                    "Can't find the `{:?}' component when adding it to an entity: component not yet registered",
                    std::any::type_name::<Self>()
                );
                return Err(ErrorType::DoesNotExist);
            }
        }

        Ok(())
    }

    /// Removes a component from an entity
    fn remove_from_entity(
        manager: &mut ComponentManager,
        entity: &Entity,
    ) -> Result<(), ErrorType> {
        let type_id = Self::get_type_id();

        match manager.component_storages.get_mut(&type_id) {
            Some(storage) => {
                if let Err(err) = storage.remove_from_entity(entity) {
                    log_error!(
                        "Failed to remove the `{:?}' component from an entity: {:?}",
                        std::any::type_name::<Self>(),
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            None => {
                log_error!(
                    "Can't find the `{:?}' component when removing it to an entity: component not yet registered",
                    std::any::type_name::<Self>()
                );
                return Err(ErrorType::DoesNotExist);
            }
        }

        Ok(())
    }

    /// Updates a component for an entity
    fn update_for_entity(
        manager: &mut ComponentManager,
        entity: &Entity,
        value: Box<dyn RealComponent>,
    ) -> Result<(), ErrorType> {
        let type_id = Self::get_type_id();

        match manager.component_storages.get_mut(&type_id) {
            Some(storage) => {
                if let Err(err) = storage.updates_for_entity(entity, value) {
                    log_error!(
                        "Failed to add the `{:?}' component to an entity: {:?}",
                        std::any::type_name::<Self>(),
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            None => {
                log_error!(
                    "Can't find the `{:?}' component when adding it to an entity: component not yet registered",
                    std::any::type_name::<Self>()
                );
                return Err(ErrorType::DoesNotExist);
            }
        }

        Ok(())
    }

    /// Gets the value of a component from an entity
    fn get_value_from_entity(
        manager: &mut ComponentManager,
        entity: &Entity,
    ) -> Result<Box<dyn RealComponent>, ErrorType> {
        let type_id = Self::get_type_id();

        match manager.component_storages.get_mut(&type_id) {
            Some(storage) => match storage.get_value_from_entity(entity) {
                Ok(value) => Ok(value),
                Err(err) => {
                    log_error!(
                        "Failed to add the `{:?}' component to an entity: {:?}",
                        std::any::type_name::<Self>(),
                        err
                    );
                    Err(ErrorType::Unknown)
                }
            },
            None => {
                log_error!(
                    "Can't find the `{:?}' component when adding it to an entity: component not yet registered",
                    std::any::type_name::<Self>()
                );
                Err(ErrorType::DoesNotExist)
            }
        }
    }
}

/// A default component used to query info on all the other components
#[derive(Debug, Clone)]
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
        let default_component_id = DefaultComponent::get_type_id();
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

    /// Removes a given entity from every component
    pub fn remove_entity(&mut self, entity: &Entity) -> Result<(), ErrorType> {
        for (type_id, storages) in &mut self.component_storages {
            if let Err(err) = storages.remove_entity(entity) {
                log_error!(
                    "Failed to remove an entity in the `{:?}' component: {:?}",
                    type_id,
                    err
                );
                return Err(ErrorType::Unknown);
            }
        }
        Ok(())
    }

    /// Removes a list of entities from every component
    pub fn remove_entities(&mut self, entities: &[Entity]) -> Result<(), ErrorType> {
        for entity in entities {
            self.remove_entity(entity)?
        }
        Ok(())
    }

    /// Check if a given component type is well registered
    pub fn is_registered(&self, type_id: &std::any::TypeId) -> bool {
        self.component_storages.contains_key(type_id)
    }

    pub fn has_component_type(
        &self,
        entity: &Entity,
        type_id: &std::any::TypeId,
    ) -> Result<bool, ErrorType> {
        match self.component_storages.get(type_id) {
            None => {
                log_error!("Unexisting component");
                Err(ErrorType::DoesNotExist)
            }
            Some(storage) => match storage.get(entity) {
                Ok(Some(..)) => Ok(true),
                Ok(None) => Ok(false),
                Err(err) => {
                    log_error!("Failed to get value: {:?}", err);
                    Err(ErrorType::Unknown)
                }
            },
        }
    }

    pub fn has_correct_constraints(
        &self,
        entity: &Entity,
        with: &[std::any::TypeId],
        without: &[std::any::TypeId],
    ) -> Result<bool, ErrorType> {
        for type_id in with {
            if !self.has_component_type(entity, type_id)? {
                return Ok(false);
            }
        }

        for type_id in without {
            if self.has_component_type(entity, type_id)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    pub fn get(
        &self,
        type_id: &std::any::TypeId,
        entity: &Entity,
    ) -> Result<&dyn RealComponent, ErrorType> {
        match self.component_storages.get(type_id) {
            None => {
                log_error!("Unexisting component");
                Err(ErrorType::DoesNotExist)
            }
            Some(storage) => match storage.get(entity) {
                Ok(Some(value)) => Ok(value),
                Ok(None) => {
                    log_error!("Failed to find non empty value");
                    Err(ErrorType::DoesNotExist)
                }
                Err(err) => {
                    log_error!("Failed to get value: {:?}", err);
                    Err(ErrorType::Unknown)
                }
            },
        }
    }

    pub fn get_mut(
        &mut self,
        type_id: &std::any::TypeId,
        entity: &Entity,
    ) -> Result<&mut dyn RealComponent, ErrorType> {
        match self.component_storages.get_mut(type_id) {
            None => {
                log_error!("Unexisting component");
                Err(ErrorType::DoesNotExist)
            }
            Some(storage) => match storage.get_mut(entity) {
                Ok(Some(value)) => Ok(value),
                Ok(None) => {
                    log_error!("Failed to find non empty value");
                    Err(ErrorType::DoesNotExist)
                }
                Err(err) => {
                    log_error!("Failed to get value: {:?}", err);
                    Err(ErrorType::Unknown)
                }
            },
        }
    }
}

pub(crate) type RegisterComponentFunction = fn(&mut ComponentManager) -> Result<(), ErrorType>;
pub(crate) type RemoveComponentFunction = fn(&mut ComponentManager) -> Result<(), ErrorType>;
pub(crate) type AddComponentToEntityFunction =
    fn(&mut ComponentManager, &Entity, Box<dyn RealComponent>) -> Result<(), ErrorType>;
pub(crate) type RemoveComponentFromEntityFunction =
    fn(&mut ComponentManager, &Entity) -> Result<(), ErrorType>;
pub(crate) type UpdateComponentForEntityFunction =
    fn(&mut ComponentManager, &Entity, Box<dyn RealComponent>) -> Result<(), ErrorType>;
