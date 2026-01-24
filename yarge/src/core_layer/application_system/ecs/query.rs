#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::core_layer::application_system::ecs::component::{
    Component, ComponentId, RealComponent,
};

/// A trait representing a Query Parameter
#[allow(private_interfaces)]
pub trait QueryParam {
    /// Components this query reads
    fn component_ids() -> Vec<ComponentId>;
    /// Components this query writes
    /// The ids are also inside component_ids
    fn mutable_ids() -> Vec<ComponentId>;
}

#[allow(private_interfaces)]
impl<T: Component> QueryParam for &T {
    fn component_ids() -> Vec<ComponentId> {
        vec![T::get_type_id()]
    }

    fn mutable_ids() -> Vec<ComponentId> {
        vec![]
    }
}

#[allow(private_interfaces)]
impl<T: Component> QueryParam for &mut T {
    fn component_ids() -> Vec<ComponentId> {
        vec![T::get_type_id()]
    }

    fn mutable_ids() -> Vec<ComponentId> {
        vec![T::get_type_id()]
    }
}

/// A macro to generate impls for tuples with 2 to 16 elements
macro_rules! derive_query_params_for_tuples {
    ($($T:ident),*) => {
        #[allow(private_interfaces)]
        impl<$($T: QueryParam),*> QueryParam for ($($T,)*) {
            fn component_ids() -> Vec<ComponentId> {
                let mut ids = Vec::new();
                $(
                    ids.extend($T::component_ids());
                )*
                ids
            }

            fn mutable_ids() -> Vec<ComponentId> {
                let mut ids = Vec::new();
                $(
                    ids.extend($T::mutable_ids());
                )*
                ids
            }
        }
    };
}

variadics_please::all_tuples!(derive_query_params_for_tuples, 2, 16, T);

/// A trait representing an actual query
/// # Safety
///
/// Should verify the ecs_ptr access rights
///
/// ...
pub unsafe trait QueryFetch<'w>: QueryParam {
    type Item;

    /// Fetches a Component value from a query
    /// # Safety
    ///
    /// Should verify the ecs_ptr access rights
    ///
    /// ...
    #[allow(private_interfaces)]
    unsafe fn fetch(
        ecs_ptr: &'w crate::UnsafeECSCell,
        user_entity: &super::entity::UserEntity,
        real_entity: &super::entity::Entity,
    ) -> Result<Option<Self::Item>, ErrorType>;
}

unsafe impl<'w, T: Component> QueryFetch<'w> for &T {
    type Item = &'w T;

    #[allow(private_interfaces)]
    unsafe fn fetch(
        ecs_ptr: &'w crate::UnsafeECSCell,
        user_entity: &super::entity::UserEntity,
        real_entity: &super::entity::Entity,
    ) -> Result<Option<Self::Item>, ErrorType> {
        let component_manager = unsafe { &ecs_ptr.get().component_manager };
        let type_id = Self::component_ids()[0];
        let component: &dyn RealComponent = match component_manager.get(&type_id, real_entity) {
            Ok(component) => component,
            Err(ErrorType::DoesNotExist) => {
                log_warn!(
                    "The `{:?}' component for user entity `{:?}' doesn't exist",
                    std::any::type_name::<T>(),
                    user_entity,
                );
                return Ok(None);
            }
            Err(err) => {
                log_error!(
                    "Failed to fetch a `{:?}' component from a query: {:?}",
                    std::any::type_name::<T>(),
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        match component.as_any().downcast_ref::<T>() {
            Some(component) => Ok(Some(component)),
            None => {
                log_error!(
                    "Failed to downcast a value when fetching the `{:?}' component of an entity in a query",
                    std::any::type_name::<T>(),
                );
                Err(ErrorType::Unknown)
            }
        }
    }
}

unsafe impl<'w, T: Component> QueryFetch<'w> for &mut T {
    type Item = &'w mut T;

    #[allow(private_interfaces)]
    unsafe fn fetch(
        ecs_ptr: &'w crate::UnsafeECSCell,
        user_entity: &super::entity::UserEntity,
        real_entity: &super::entity::Entity,
    ) -> Result<Option<Self::Item>, ErrorType> {
        let component_manager = unsafe { &mut ecs_ptr.get_mut().component_manager };
        let type_id = Self::mutable_ids()[0];
        let component: &mut dyn RealComponent =
            match component_manager.get_mut(&type_id, real_entity) {
                Ok(component) => component,
                Err(ErrorType::DoesNotExist) => {
                    log_warn!(
                        "The `{:?}' component for user entity `{:?}' doesn't exist",
                        std::any::type_name::<T>(),
                        user_entity,
                    );
                    return Ok(None);
                }
                Err(err) => {
                    log_error!(
                        "Failed to fetch a `{:?}' component from a mutable query: {:?}",
                        std::any::type_name::<T>(),
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            };
        match component.as_any_mut().downcast_mut::<T>() {
            Some(component) => Ok(Some(component)),
            None => {
                log_error!(
                    "Failed to downcast a value when fetching the `{:?}' component of an entity in a mutable query",
                    std::any::type_name::<T>(),
                );
                Err(ErrorType::Unknown)
            }
        }
    }
}

/// A macro to generate impls for tuples with 2 to 16 elements
macro_rules! derive_query_fetch_for_tuples {
    ($($T:ident),*) => {
        unsafe impl<'w, $($T: QueryFetch<'w>),*> QueryFetch<'w> for ($($T,)*) {
            type Item = ($($T::Item,)*);

            #[allow(private_interfaces)]
            unsafe fn fetch(
                ecs_ptr: &'w crate::UnsafeECSCell,
                user_entity: &super::entity::UserEntity,
                real_entity: &super::entity::Entity,
            ) -> Result<Option<Self::Item>, ErrorType> {

                Ok(Some((
                    $(
                        match unsafe { $T::fetch(ecs_ptr, user_entity, real_entity) } {
                            Ok(Some(component)) => component,
                            Ok(None) => return Ok(None),
                            Err(err) => {
                                log_error!(
                                    "Failed to fetch a component in a tuple query fetch: {:?}",
                                    err
                                );
                                return Err(ErrorType::Unknown);
                            }
                        }
                    ),*
                )))
            }
        }
    };
}
variadics_please::all_tuples!(derive_query_fetch_for_tuples, 2, 16, T);

#[allow(private_interfaces)]
pub trait QueryFilterList {
    fn component_ids() -> Vec<ComponentId>;
}
#[allow(private_interfaces)]
impl<T: Component> QueryFilterList for T {
    fn component_ids() -> Vec<ComponentId> {
        vec![T::get_type_id()]
    }
}

/// A macro to generate impls for tuples with 2 to 16 elements
macro_rules! derive_query_filter_for_tuples {
    ($($T:ident),*) => {
        impl<$($T: QueryFilterList),*> QueryFilterList for ($($T,)*) {
            #[allow(private_interfaces)]
            fn component_ids() -> Vec<ComponentId> {
                let mut ids = Vec::new();
                $(
                    ids.extend($T::component_ids());
                )*
                ids
            }
        }
    };
}
variadics_please::all_tuples!(derive_query_filter_for_tuples, 2, 16, T);

pub struct With<T: QueryFilterList>(std::marker::PhantomData<T>);
pub struct Without<T: QueryFilterList>(std::marker::PhantomData<T>);

#[allow(private_interfaces)]
pub trait QueryFilter {
    fn with() -> Vec<ComponentId>;
    fn without() -> Vec<ComponentId>;
}

#[allow(private_interfaces)]
impl QueryFilter for () {
    fn with() -> Vec<ComponentId> {
        vec![]
    }
    fn without() -> Vec<ComponentId> {
        vec![]
    }
}

#[allow(private_interfaces)]
impl<T: QueryFilterList> QueryFilter for With<T> {
    fn with() -> Vec<ComponentId> {
        T::component_ids()
    }
    fn without() -> Vec<ComponentId> {
        vec![]
    }
}

#[allow(private_interfaces)]
impl<T: QueryFilterList> QueryFilter for Without<T> {
    fn with() -> Vec<ComponentId> {
        vec![]
    }
    fn without() -> Vec<ComponentId> {
        T::component_ids()
    }
}

#[allow(private_interfaces)]
impl<In: QueryFilterList, Out: QueryFilterList> QueryFilter for (With<In>, Without<Out>) {
    fn with() -> Vec<ComponentId> {
        In::component_ids()
    }
    fn without() -> Vec<ComponentId> {
        Out::component_ids()
    }
}

#[allow(private_interfaces)]
impl<Out: QueryFilterList, In: QueryFilterList> QueryFilter for (Without<Out>, With<In>) {
    fn with() -> Vec<ComponentId> {
        In::component_ids()
    }
    fn without() -> Vec<ComponentId> {
        Out::component_ids()
    }
}

/// A system query
/// The query is built everytime the system is called so the inner values of the structs are build everytime
pub struct Query<'w, 's, Q, F = ()>
where
    Q: QueryFetch<'w>,
    F: QueryFilter,
{
    /// A phantom marker for the types
    pub(crate) _marker: std::marker::PhantomData<(Q, F)>,
    /// An unsafe pointer to the ECS created at runtim
    pub(crate) ecs_ptr: &'w crate::UnsafeECSCell,
    /// A pointer to the entities associated with this query
    pub(crate) entities:
        &'s std::collections::HashSet<(super::entity::UserEntity, super::entity::Entity)>,
}

/// The saved state of the query
pub struct QueryState {
    /// The components with filter
    pub(crate) with: Vec<ComponentId>,
    /// The components without filter
    pub(crate) without: Vec<ComponentId>,
    /// The entities used by the query
    pub(crate) entities:
        std::collections::HashSet<(super::entity::UserEntity, super::entity::Entity)>,
}

impl QueryState {
    /// Checks if the given query state needs this component to work
    pub(crate) fn need_component<Q>(&self, component_id: &ComponentId) -> bool
    where
        Q: QueryParam,
    {
        let component_ids = Q::component_ids();
        component_ids.contains(component_id)
            || self.with.contains(component_id)
            || self.without.contains(component_id)
    }
}

impl<Q, F> super::system::SystemParam for Query<'_, '_, Q, F>
where
    Q: for<'a> QueryFetch<'a>,
    F: QueryFilter,
{
    type State = QueryState;
    type Item<'w, 's> = Query<'w, 's, Q, F>;

    /// Called when an entity is removed from the ECS
    #[allow(private_interfaces)]
    fn on_entity_removed(
        state: &mut Self::State,
        real_entity: &super::entity::Entity,
        user_entity: &super::entity::UserEntity,
    ) -> Result<bool, ErrorType> {
        let _was_present = state.entities.remove(&(*user_entity, *real_entity));
        Ok(false)
    }

    /// Called when a component is added to an entity
    #[allow(private_interfaces)]
    fn on_component_added_to_entity(
        state: &mut Self::State,
        component_manager: &super::component::ComponentManager,
        real_entity: &super::entity::Entity,
        user_entity: &super::entity::UserEntity,
        component_id: &ComponentId,
    ) -> Result<bool, ErrorType> {
        if state.need_component::<Q>(component_id) {
            match component_manager.has_correct_constraints(
                real_entity,
                &state.with,
                &state.without,
            ) {
                Ok(true) => {
                    let _ = state.entities.insert((*user_entity, *real_entity));
                }
                Err(err) => {
                    log_error!(
                        "Failed to check if an entry has the correct constraints when adding a component to an entity in a query state: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
                _ => {}
            }
        }
        Ok(false)
    }

    /// Called when a component is removed from an entity
    #[allow(private_interfaces)]
    fn on_component_removed_from_entity(
        state: &mut Self::State,
        component_manager: &super::component::ComponentManager,
        real_entity: &super::entity::Entity,
        user_entity: &super::entity::UserEntity,
        component_id: &ComponentId,
    ) -> Result<bool, ErrorType> {
        if state.need_component::<Q>(component_id) {
            match component_manager.has_correct_constraints(
                real_entity,
                &state.with,
                &state.without,
            ) {
                Ok(false) => {
                    let _ = state.entities.remove(&(*user_entity, *real_entity));
                }
                Err(err) => {
                    log_error!(
                        "Failed to check if an entry has the correct constraints when removing a component from an entity in a query state: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
                _ => {}
            }
        }
        Ok(false)
    }

    /// Called when a component is removed from the ECS
    #[allow(private_interfaces)]
    fn on_component_removed(
        state: &mut Self::State,
        component_id: &ComponentId,
    ) -> Result<bool, ErrorType> {
        if state.need_component::<Q>(component_id) {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn init_state(_game: &dyn crate::Game, ecs: &crate::ECS) -> Result<Self::State, ErrorType> {
        let component_ids = Q::component_ids();
        let with = F::with();
        let without = F::without();
        let mut entities = std::collections::HashSet::new();

        let component_manager = &ecs.component_manager;
        for entity in &ecs.entities {
            let mut should_add_entity = true;
            'inner_loop: for component_id in &component_ids {
                match component_manager.has_component_type(entity, component_id) {
                    Ok(false) => {
                        should_add_entity = false;
                        break 'inner_loop;
                    }
                    Ok(true) => {
                        match component_manager.has_correct_constraints(entity, &with, &without) {
                            Ok(false) => {
                                should_add_entity = false;
                                break 'inner_loop;
                            }
                            Err(err) => {
                                log_error!(
                                    "Failed to check if an entry has the correct constraints when initializing a query state: {:?}",
                                    err
                                );
                                return Err(ErrorType::Unknown);
                            }
                            _ => {}
                        }
                    }
                    Err(err) => {
                        log_error!(
                            "Failed to check if an entry has the given component type when initializing a query: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                }
            }

            // Only add entities if all constraints are valid
            if should_add_entity {
                let user_entity = match crate::ECS::get_user_entity(entity) {
                    Ok(Some(entity)) => entity,
                    _ => {
                        log_error!(
                            "Failed to find a user entity associated with the real entity when initializing the query state"
                        );
                        return Err(ErrorType::DoesNotExist);
                    }
                };
                if !entities.insert((user_entity, *entity)) {
                    log_error!(
                        "Failed to insert a new entity in a query parameter, the entity was already present"
                    );
                    return Err(ErrorType::Duplicate);
                }
            }
        }

        // log_debug!("Query entities: {:?}\n\n", &entities);
        // log_debug!("Query ids: {:?}", &component_ids);
        // log_debug!("Query filters with: {:?}", &with);
        // log_debug!("Query filters without: {:?}", &without);

        Ok(Self::State {
            with,
            without,
            entities,
        })
    }

    unsafe fn get_item<'w, 's>(
        state: &'s mut Self::State,
        _game_ptr: &'w crate::UnsafeGameCell,
        ecs_ptr: &'w crate::UnsafeECSCell,
    ) -> Result<Self::Item<'w, 's>, ErrorType> {
        Ok(Query {
            _marker: std::marker::PhantomData,
            ecs_ptr,
            entities: &state.entities,
        })
    }
}

//////////////////////////////////////////////////////////
///////////////     query iterator     ///////////////////
//////////////////////////////////////////////////////////
pub struct QueryIter<'w, 's, Q>
where
    Q: QueryFetch<'w>,
{
    ecs_ptr: &'w crate::UnsafeECSCell,
    entities:
        std::collections::hash_set::Iter<'s, (super::entity::UserEntity, super::entity::Entity)>,
    _marker: std::marker::PhantomData<Q>,
}
pub struct QueryIterWithEntities<'w, 's, Q>
where
    Q: QueryFetch<'w>,
{
    ecs_ptr: &'w crate::UnsafeECSCell,
    entities:
        std::collections::hash_set::Iter<'s, (super::entity::UserEntity, super::entity::Entity)>,
    _marker: std::marker::PhantomData<Q>,
}

impl<'w, Q> Iterator for QueryIter<'w, '_, Q>
where
    Q: QueryFetch<'w>,
{
    type Item = Q::Item;

    fn next(&mut self) -> Option<Self::Item> {
        for (user_entity, real_entity) in &mut self.entities {
            match unsafe { Q::fetch(self.ecs_ptr, user_entity, real_entity) } {
                Ok(Some(item)) => return Some(item),
                Ok(None) => continue,
                Err(err) => {
                    log_warn!(
                        "Failed to get the next entry in a QueryIter iterator: {:?}",
                        err
                    );
                    return None;
                }
            }
        }
        None
    }
}
impl<'w, Q> Iterator for QueryIterWithEntities<'w, '_, Q>
where
    Q: QueryFetch<'w>,
{
    type Item = (Q::Item, super::entity::UserEntity);

    fn next(&mut self) -> Option<Self::Item> {
        for (user_entity, real_entity) in &mut self.entities {
            match unsafe { Q::fetch(self.ecs_ptr, user_entity, real_entity) } {
                Ok(Some(item)) => return Some((item, *user_entity)),
                Ok(None) => continue,
                Err(err) => {
                    log_warn!(
                        "Failed to get the next entry in a QueryIterWithEntities iterator: {:?}",
                        err
                    );
                    return None;
                }
            }
        }
        None
    }
}

impl<'w, 's, Q, F> Query<'w, 's, Q, F>
where
    Q: QueryFetch<'w>,
    F: QueryFilter,
{
    pub fn iter(&self) -> QueryIter<'w, 's, Q> {
        QueryIter {
            ecs_ptr: self.ecs_ptr,
            entities: self.entities.iter(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> QueryIter<'w, 's, Q> {
        QueryIter {
            ecs_ptr: self.ecs_ptr,
            entities: self.entities.iter(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_entities(&self) -> QueryIterWithEntities<'w, 's, Q> {
        QueryIterWithEntities {
            ecs_ptr: self.ecs_ptr,
            entities: self.entities.iter(),
            _marker: std::marker::PhantomData,
        }
    }
}

impl<'w, 's, Q, F> IntoIterator for &'s Query<'w, 's, Q, F>
where
    Q: QueryFetch<'w>,
    F: QueryFilter,
{
    type Item = Q::Item;
    type IntoIter = QueryIter<'w, 's, Q>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'w, 's, Q, F> IntoIterator for &'s mut Query<'w, 's, Q, F>
where
    Q: QueryFetch<'w>,
    F: QueryFilter,
{
    type Item = Q::Item;
    type IntoIter = QueryIter<'w, 's, Q>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

//////////////////////////////////////////////////////////
///////////////     query tests      /////////////////////
//////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_layer::application_system::ecs::component::{
        DefaultComponent, RegisterComponentFunction,
    };

    #[test]
    fn initialization() {
        // Create new components
        struct NewComponent1 {
            value: u32,
        }
        struct NewComponent2;

        impl crate::Component for NewComponent1 {}
        impl crate::Component for NewComponent2 {}
        // Helpers to play with components
        let id_1 = NewComponent1::get_type_id();
        let register_1: RegisterComponentFunction = NewComponent1::register;
        let id_2 = NewComponent2::get_type_id();
        let register_2: RegisterComponentFunction = NewComponent2::register;

        // Init ecs
        let mut ecs = crate::ECS::init().unwrap();
        let ecs_ptr = crate::UnsafeECSCell::new(&mut ecs);
        ecs.register_component(&id_1, &register_1).unwrap();
        ecs.register_component(&id_2, &register_2).unwrap();

        let entities = std::collections::HashSet::new();

        let mut _query = Query::<&NewComponent1> {
            _marker: std::marker::PhantomData,
            ecs_ptr: &ecs_ptr,
            entities: &entities,
        };

        let mut _query = Query::<&NewComponent1, With<NewComponent2>> {
            _marker: std::marker::PhantomData,
            ecs_ptr: &ecs_ptr,
            entities: &entities,
        };

        let mut _query = Query::<&NewComponent1, Without<NewComponent2>> {
            _marker: std::marker::PhantomData,
            ecs_ptr: &ecs_ptr,
            entities: &entities,
        };

        let mut _query = Query::<(&NewComponent1, &mut NewComponent2)> {
            _marker: std::marker::PhantomData,
            ecs_ptr: &ecs_ptr,
            entities: &entities,
        };

        let mut _query = Query::<&NewComponent1, Without<NewComponent1>> {
            _marker: std::marker::PhantomData,
            ecs_ptr: &ecs_ptr,
            entities: &entities,
        };

        let mut _query2 = Query::<&NewComponent1, Without<NewComponent2>> {
            _marker: std::marker::PhantomData,
            ecs_ptr: &ecs_ptr,
            entities: &entities,
        };
    }

    #[test]
    fn filters() {
        let ids = With::<DefaultComponent>::with();
        assert_eq!(ids, vec![DefaultComponent::get_type_id()]);

        let ids = With::<(DefaultComponent, DefaultComponent)>::with();
        assert_eq!(
            ids,
            vec![
                DefaultComponent::get_type_id(),
                DefaultComponent::get_type_id(),
            ]
        );

        let ids = Without::<DefaultComponent>::without();
        assert_eq!(ids, vec![DefaultComponent::get_type_id()]);

        let ids = Without::<(DefaultComponent, DefaultComponent, DefaultComponent)>::without();
        assert_eq!(
            ids,
            vec![
                DefaultComponent::get_type_id(),
                DefaultComponent::get_type_id(),
                DefaultComponent::get_type_id(),
            ]
        );
    }
}
