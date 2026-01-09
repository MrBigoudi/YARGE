use crate::{
    Component, FileResourceTypeId, RonFileResource, SystemSchedule, UserEventWrapper,
    core_layer::{
        FileLoaderSystem, UserEntity,
        application_system::{
            ecs::{
                component::{
                    AddComponentToEntityFunction, RealComponent, RegisterComponentFunction,
                    RemoveComponentFromEntityFunction, RemoveComponentFunction,
                    UpdateComponentForEntityFunction,
                },
                system::{
                    SystemCallback, SystemCallbackConditionFunction, SystemMutCallback,
                    UserSystemCallback, UserSystemCallbackBuilder,
                    UserSystemCallbackConditionFunction, UserSystemMutCallback,
                },
            },
            events::user_events::UserEvent,
        },
        file_system::file::LoadingFileFunction,
    },
};

#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

pub struct QuitAppEventBuilder;
impl QuitAppEventBuilder {
    pub fn build() -> Result<UserEventWrapper, ErrorType> {
        Ok(UserEventWrapper {
            event: UserEvent::QuitApp,
        })
    }
}

#[derive(Default)]
pub struct RegisterCustomFileResourceEventBuilder {
    /// The id of the new resource type
    resource_id: Option<FileResourceTypeId>,
    /// The function to load the resource
    loader_fct: Option<LoadingFileFunction>,
}
impl RegisterCustomFileResourceEventBuilder {
    pub fn resource_id(mut self, resource_id: &FileResourceTypeId) -> Self {
        self.resource_id = Some(FileLoaderSystem::cast_resource_id(resource_id));
        self
    }
    pub fn resource_type<T: RonFileResource>(mut self) -> Self {
        self.loader_fct = Some(T::start_load_ron);
        self
    }
    pub fn build(self) -> Result<UserEventWrapper, ErrorType> {
        if self.loader_fct.is_none() {
            log_error!("Can't build a `RegisterCustomFileResource' event without a resource type");
            return Err(ErrorType::DoesNotExist);
        }
        if self.resource_id.is_none() {
            log_error!("Can't build a `RegisterCustomFileResource' event without a resource id");
            return Err(ErrorType::DoesNotExist);
        }
        Ok(UserEventWrapper {
            event: UserEvent::RegisterCustomFileResource {
                resource_id: self.resource_id.unwrap(),
                loader_fct: self.loader_fct.unwrap(),
            },
        })
    }
}

#[derive(Default)]
pub struct StartLoadCustomFileResourceEventBuilder {
    /// The id of the resource type to load
    resource_id: Option<FileResourceTypeId>,
    /// The path to the resource to load
    path: Option<std::path::PathBuf>,
}
impl StartLoadCustomFileResourceEventBuilder {
    pub fn resource_id(mut self, resource_id: &FileResourceTypeId) -> Self {
        self.resource_id = Some(FileLoaderSystem::cast_resource_id(resource_id));
        self
    }
    pub fn path(mut self, path: &std::path::Path) -> Self {
        self.path = Some(std::path::PathBuf::from(path));
        self
    }
    pub fn build(self) -> Result<UserEventWrapper, ErrorType> {
        if self.resource_id.is_none() {
            log_error!("Can't build a `StartLoadCustomFileResource' event without a resource id");
            return Err(ErrorType::DoesNotExist);
        }
        if self.path.is_none() {
            log_error!("Can't build a `StartLoadCustomFileResource' event without a path");
            return Err(ErrorType::DoesNotExist);
        }
        Ok(UserEventWrapper {
            event: UserEvent::StartLoadCustomFileResource {
                resource_id: self.resource_id.unwrap(),
                path: self.path.unwrap(),
            },
        })
    }
}

#[derive(Default)]
pub struct RemoveEntitiesEventBuilder {
    /// The entities to remove
    user_entities: Vec<UserEntity>,
}
impl RemoveEntitiesEventBuilder {
    pub fn add_entity(mut self, user_entity: &UserEntity) -> Self {
        self.user_entities.push(*user_entity);
        self
    }
    pub fn add_entities(mut self, user_entities: &[UserEntity]) -> Self {
        self.user_entities.append(&mut user_entities.to_vec());
        self
    }
    pub fn build(self) -> Result<UserEventWrapper, ErrorType> {
        if self.user_entities.is_empty() {
            log_error!("Can't build a `RemoveEntities' event without user entities");
            return Err(ErrorType::DoesNotExist);
        }
        if self.user_entities.len() == 1 {
            Ok(UserEventWrapper {
                event: UserEvent::RemoveEntity {
                    user_entity: self.user_entities[0],
                },
            })
        } else {
            Ok(UserEventWrapper {
                event: UserEvent::RemoveEntities {
                    user_entities: self.user_entities,
                },
            })
        }
    }
}

#[derive(Default)]
pub struct RegisterCustomComponentEventBuilder {
    /// The id of the component
    component_id: Option<std::any::TypeId>,
    /// The function to register the component
    register_fct: Option<RegisterComponentFunction>,
}
impl RegisterCustomComponentEventBuilder {
    pub fn component_type<T: Component>(mut self) -> Self {
        self.component_id = Some(T::get_type_id());
        self.register_fct = Some(T::register);
        self
    }
    pub fn build(self) -> Result<UserEventWrapper, ErrorType> {
        if self.component_id.is_none() {
            log_error!("Can't build a `RegisterCustomComponent' event without a component type");
            return Err(ErrorType::DoesNotExist);
        }

        Ok(UserEventWrapper {
            event: UserEvent::RegisterCustomComponent {
                component_id: self.component_id.unwrap(),
                register_fct: self.register_fct.unwrap(),
            },
        })
    }
}

#[derive(Default)]
pub struct RemoveCustomComponentEventBuilder {
    /// The id of the component
    component_id: Option<std::any::TypeId>,
    /// The function to remove the component
    remove_fct: Option<RemoveComponentFunction>,
}
impl RemoveCustomComponentEventBuilder {
    pub fn component_type<T: Component>(mut self) -> Self {
        self.component_id = Some(T::get_type_id());
        self.remove_fct = Some(T::remove);
        self
    }
    pub fn build(self) -> Result<UserEventWrapper, ErrorType> {
        if self.component_id.is_none() {
            log_error!("Can't build a `RemoveCustomComponent' event without a component type");
            return Err(ErrorType::DoesNotExist);
        }
        Ok(UserEventWrapper {
            event: UserEvent::RemoveCustomComponent {
                component_id: self.component_id.unwrap(),
                remove_fct: self.remove_fct.unwrap(),
            },
        })
    }
}

#[derive(Default)]
pub struct AddComponentToEntityEventBuilder {
    /// The id of the component
    component_id: Option<std::any::TypeId>,
    /// The function to add a component to an entity
    add_to_entity_fct: Option<AddComponentToEntityFunction>,
    /// The user entity to which add the component
    user_entity: Option<UserEntity>,
    /// The value of the component to add to the entity
    value: Option<Box<dyn RealComponent>>,
}
impl AddComponentToEntityEventBuilder {
    pub fn component_type<T: Component>(mut self) -> Self {
        self.component_id = Some(T::get_type_id());
        self.add_to_entity_fct = Some(T::add_to_entity);
        self
    }
    pub fn entity(mut self, user_entity: &UserEntity) -> Self {
        self.user_entity = Some(*user_entity);
        self
    }
    pub fn value<T: Component>(mut self, value: T) -> Self {
        self.value = Some(Box::new(value));
        self
    }
    pub fn build(self) -> Result<UserEventWrapper, ErrorType> {
        if self.component_id.is_none() {
            log_error!("Can't build a `AddComponentToEntity' event without a component type");
            return Err(ErrorType::DoesNotExist);
        }
        if self.user_entity.is_none() {
            log_error!("Can't build an `AddComponentToEntity` event without a user entity");
            return Err(ErrorType::DoesNotExist);
        }
        if self.value.is_none() {
            log_error!("Can't build an `AddComponentToEntity` event without a value");
            return Err(ErrorType::DoesNotExist);
        }
        Ok(UserEventWrapper {
            event: UserEvent::AddComponentToEntity {
                component_id: self.component_id.unwrap(),
                user_entity: self.user_entity.unwrap(),
                value: self.value.unwrap(),
                add_to_entity_fct: self.add_to_entity_fct.unwrap(),
            },
        })
    }
}

#[derive(Default)]
pub struct RemoveComponentFromEntityEventBuilder {
    /// The id of the component
    component_id: Option<std::any::TypeId>,
    /// The function to add a component to an entity
    remove_from_entity_fct: Option<RemoveComponentFromEntityFunction>,
    /// The user entity to which add the component
    user_entity: Option<UserEntity>,
}
impl RemoveComponentFromEntityEventBuilder {
    pub fn component_type<T: Component>(mut self) -> Self {
        self.component_id = Some(T::get_type_id());
        self.remove_from_entity_fct = Some(T::remove_from_entity);
        self
    }
    pub fn entity(mut self, user_entity: &UserEntity) -> Self {
        self.user_entity = Some(*user_entity);
        self
    }
    pub fn build(self) -> Result<UserEventWrapper, ErrorType> {
        if self.component_id.is_none() {
            log_error!("Can't build a `RemoveComponentFromEntity' event without a component type");
            return Err(ErrorType::DoesNotExist);
        }
        if self.user_entity.is_none() {
            log_error!("Can't build a `RemoveComponentFromEntity` event without a user entity");
            return Err(ErrorType::DoesNotExist);
        }
        Ok(UserEventWrapper {
            event: UserEvent::RemoveComponentFromEntity {
                component_id: self.component_id.unwrap(),
                user_entity: self.user_entity.unwrap(),
                remove_from_entity_fct: self.remove_from_entity_fct.unwrap(),
            },
        })
    }
}

#[derive(Default)]
pub struct UpdateComponentValueForEntityEventBuilder {
    /// The id of the component
    component_id: Option<std::any::TypeId>,
    /// The function to update a component for an entity
    update_for_entity_fct: Option<UpdateComponentForEntityFunction>,
    /// The user entity which needs a component update
    user_entity: Option<UserEntity>,
    /// The new value of the component for the entity
    value: Option<Box<dyn RealComponent>>,
}
impl UpdateComponentValueForEntityEventBuilder {
    pub fn component_type<T: Component>(mut self) -> Self {
        self.component_id = Some(T::get_type_id());
        self.update_for_entity_fct = Some(T::update_for_entity);
        self
    }
    pub fn entity(mut self, user_entity: &UserEntity) -> Self {
        self.user_entity = Some(*user_entity);
        self
    }
    pub fn value<T: Component>(mut self, value: T) -> Self {
        self.value = Some(Box::new(value));
        self
    }
    pub fn build(self) -> Result<UserEventWrapper, ErrorType> {
        if self.component_id.is_none() {
            log_error!(
                "Can't build a `UpdateComponentValueForEntity' event without a component type"
            );
            return Err(ErrorType::DoesNotExist);
        }
        if self.user_entity.is_none() {
            log_error!("Can't build a `UpdateComponentValueForEntity` event without a user entity");
            return Err(ErrorType::DoesNotExist);
        }
        if self.value.is_none() {
            log_error!("Can't build a `UpdateComponentValueForEntity` event without a value");
            return Err(ErrorType::DoesNotExist);
        }
        Ok(UserEventWrapper {
            event: UserEvent::UpdateComponentValueForEntity {
                component_id: self.component_id.unwrap(),
                user_entity: self.user_entity.unwrap(),
                value: self.value.unwrap(),
                update_for_entity_fct: self.update_for_entity_fct.unwrap(),
            },
        })
    }
}

pub struct RegisterSystemEventBuilder {
    /// The name of the main component the system will run on
    component_id: Option<std::any::TypeId>,
    /// A list of component the entity must have for this system to run on it
    with: Vec<std::any::TypeId>,
    /// A list of component the entity must not have for this system to run on it
    without: Vec<std::any::TypeId>,
    /// The rate at which this system will be called
    schedule: SystemSchedule,
    /// The condition function to run or not this system
    condition: SystemCallbackConditionFunction,
    /// The system function
    callback: Option<SystemCallback>,
    /// The system function as mutable
    callback_mut: Option<SystemMutCallback>,
}
impl Default for RegisterSystemEventBuilder {
    fn default() -> Self {
        Self {
            component_id: None,
            with: vec![],
            without: vec![],
            schedule: SystemSchedule::default(),
            condition: UserSystemCallbackBuilder::default_condition(),
            callback: None,
            callback_mut: None,
        }
    }
}
impl RegisterSystemEventBuilder {
    pub fn component_type<T: Component>(mut self) -> Self {
        self.component_id = Some(T::get_type_id());
        self
    }
    pub fn callback<G, T>(mut self, callback: UserSystemCallback<G, T>) -> Self
    where
        G: crate::Game + 'static,
        T: crate::Component + 'static,
    {
        self.callback = Some(UserSystemCallbackBuilder::system::<G, T>(callback));
        if self.callback_mut.is_some() {
            log_warn!("Mutable system callback removed from the builder");
        }
        self
    }
    pub fn callback_mut<G, T>(mut self, callback: UserSystemMutCallback<G, T>) -> Self
    where
        G: crate::Game + 'static,
        T: crate::Component + 'static,
    {
        self.callback_mut = Some(UserSystemCallbackBuilder::system_mut::<G, T>(callback));
        if self.callback.is_some() {
            log_warn!("Reference system callback removed from the builder");
        }
        self
    }
    pub fn with<T: Component>(mut self) -> Self {
        self.with.push(T::get_type_id());
        self
    }
    pub fn without<T: Component>(mut self) -> Self {
        self.without.push(T::get_type_id());
        self
    }
    pub fn schedule(mut self, schedule: &SystemSchedule) -> Self {
        self.schedule = schedule.clone();
        self
    }
    pub fn condition<G: crate::Game>(
        mut self,
        condition: UserSystemCallbackConditionFunction<G>,
    ) -> Self {
        self.condition = UserSystemCallbackBuilder::condition::<G>(condition);
        self
    }
    pub fn build(self) -> Result<UserEventWrapper, ErrorType> {
        if self.component_id.is_none() {
            log_error!("Can't build a `RegisterSystem' event without a component type");
            return Err(ErrorType::DoesNotExist);
        }
        if self.callback.is_none() && self.callback_mut.is_none() {
            log_error!("Can't build a `RegisterSystem` event without a callback");
            return Err(ErrorType::DoesNotExist);
        }
        if self.callback.is_some() {
            Ok(UserEventWrapper {
                event: UserEvent::RegisterSystem {
                    name: self.component_id.unwrap(),
                    with: self.with,
                    without: self.without,
                    callback: self.callback.unwrap(),
                    schedule: self.schedule,
                    condition: self.condition,
                },
            })
        } else {
            Ok(UserEventWrapper {
                event: UserEvent::RegisterSystemMut {
                    name: self.component_id.unwrap(),
                    with: self.with,
                    without: self.without,
                    callback_mut: self.callback_mut.unwrap(),
                    schedule: self.schedule,
                    condition: self.condition,
                },
            })
        }
    }
}
