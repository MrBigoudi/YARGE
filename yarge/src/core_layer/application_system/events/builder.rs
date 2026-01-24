#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{
    SystemSchedule,
    core_layer::application_system::{
        ecs::{
            component::{
                AddComponentToEntityFunction, Component, ComponentId, RealComponent,
                RegisterComponentFunction, RemoveComponentFromEntityFunction,
                RemoveComponentFunction, UpdateComponentForEntityFunction,
            },
            entity::UserEntity,
            resource::{
                ResourceLoadingBuilder, ResourceLoadingFunction, ResourceManager, ResourceTypeId,
                UserResource, UserResourceId, UserResourceLoadingParameters,
            },
            system::{
                SystemCallbackConditionFunction, UserSystemCallbackConditionFunction,
                UserSystemConditionBuilder,
            },
        },
        events::user_events::{UserEvent, UserEventWrapper},
    },
};

pub struct QuitAppEventBuilder;
impl QuitAppEventBuilder {
    pub fn build() -> Result<UserEventWrapper, ErrorType> {
        Ok(UserEventWrapper {
            event: UserEvent::QuitApp,
        })
    }
}

#[derive(Default)]
pub struct RegisterCustomResourceEventBuilder {
    /// The type of the resource
    resource_type_id: Option<ResourceTypeId>,
    /// The function to load the resource
    loader_fct: Option<ResourceLoadingFunction>,
}
impl RegisterCustomResourceEventBuilder {
    pub fn loading_parameters<P, R>(mut self, params: &P) -> Self
    where
        P: UserResourceLoadingParameters<R>,
        R: UserResource,
    {
        self.resource_type_id = Some(ResourceTypeId(std::any::TypeId::of::<R>()));
        self.loader_fct = Some(ResourceLoadingBuilder::loader::<P, R>(params));
        self
    }
    pub fn build(self) -> Result<(UserEventWrapper, UserResourceId), ErrorType> {
        let user_id = match ResourceManager::generate_id() {
            Ok(id) => id,
            Err(err) => {
                log_error!(
                    "Failed to generate a new id for a resource when building a `RegisterCustomResource' event: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        if self.loader_fct.is_none() {
            log_error!("Can't build a `RegisterCustomResource' event without a loading parameter");
            return Err(ErrorType::DoesNotExist);
        }

        let new_event = UserEventWrapper {
            event: UserEvent::RegisterCustomResource {
                user_id,
                resource_type_id: self.resource_type_id.unwrap(),
                loading_function: self.loader_fct.unwrap(),
            },
        };
        Ok((new_event, user_id))
    }
}

#[derive(Default)]
pub struct StartLoadCustomResourceEventBuilder {
    /// The id of the new resource type
    user_id: Option<UserResourceId>,
    /// The type of the resource
    resource_type_id: Option<ResourceTypeId>,
}
impl StartLoadCustomResourceEventBuilder {
    pub fn resource_id<R: UserResource>(mut self, id: &UserResourceId) -> Self {
        self.user_id = Some(*id);
        self.resource_type_id = Some(ResourceTypeId(std::any::TypeId::of::<R>()));
        self
    }
    pub fn build(self) -> Result<UserEventWrapper, ErrorType> {
        if self.user_id.is_none() {
            log_error!("Can't build a `StartLoadCustomResource' event without a resource id");
            return Err(ErrorType::DoesNotExist);
        }
        Ok(UserEventWrapper {
            event: UserEvent::StartLoadCustomResource {
                user_id: self.user_id.unwrap(),
                resource_type_id: self.resource_type_id.unwrap(),
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
    component_id: Option<ComponentId>,
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
    component_id: Option<ComponentId>,
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
    component_id: Option<ComponentId>,
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
    component_id: Option<ComponentId>,
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
    component_id: Option<ComponentId>,
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
    /// The new system to add
    system: Option<Box<dyn crate::SystemTrait>>,
    /// The rate at which this system will be called
    schedule: SystemSchedule,
    /// The condition function to run or not this system
    condition: SystemCallbackConditionFunction,
}
impl Default for RegisterSystemEventBuilder {
    fn default() -> Self {
        Self {
            system: None,
            schedule: SystemSchedule::default(),
            condition: UserSystemConditionBuilder::default_condition(),
        }
    }
}
impl RegisterSystemEventBuilder {
    pub fn system(mut self, system: &dyn crate::IntoSystem) -> Self {
        self.system = Some(system.as_system());
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
        self.condition = UserSystemConditionBuilder::condition::<G>(condition);
        self
    }
    pub fn build(self) -> Result<UserEventWrapper, ErrorType> {
        if self.system.is_none() {
            log_error!("Can't build a `RegisterSystem' event without a system");
            return Err(ErrorType::DoesNotExist);
        }
        Ok(UserEventWrapper {
            event: UserEvent::RegisterSystem {
                system: self.system.unwrap(),
                schedule: self.schedule,
                condition: self.condition,
            },
        })
    }
}
