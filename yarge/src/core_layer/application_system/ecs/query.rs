#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::core_layer::application_system::ecs::component::{Component, RealComponent};

pub struct UnsafeECSCell {
    ptr: *mut crate::ECS,
}

impl UnsafeECSCell {
    pub(crate) fn new(ecs: &mut crate::ECS) -> Self {
        let ptr: *mut crate::ECS = ecs;
        Self { ptr }
    }

    #[inline]
    pub(crate) unsafe fn get(&self) -> &crate::ECS {
        unsafe { &(*self.ptr) }
    }

    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub(crate) unsafe fn get_mut(&self) -> &mut crate::ECS {
        unsafe { &mut (*self.ptr) }
    }
}

pub trait QueryParam {
    /// Components this query reads
    fn component_ids() -> Vec<std::any::TypeId>;
    /// Components this query writes
    fn mutable_ids() -> Vec<std::any::TypeId>;
}

impl<T: Component> QueryParam for &T {
    fn component_ids() -> Vec<std::any::TypeId> {
        vec![T::get_type_id()]
    }

    fn mutable_ids() -> Vec<std::any::TypeId> {
        vec![]
    }
}

impl<T: Component> QueryParam for &mut T {
    fn component_ids() -> Vec<std::any::TypeId> {
        vec![T::get_type_id()]
    }

    fn mutable_ids() -> Vec<std::any::TypeId> {
        vec![T::get_type_id()]
    }
}

impl<A: QueryParam, B: QueryParam> QueryParam for (A, B) {
    fn component_ids() -> Vec<std::any::TypeId> {
        let mut ids = A::component_ids();
        ids.extend(B::component_ids());
        ids
    }

    fn mutable_ids() -> Vec<std::any::TypeId> {
        let mut ids = A::mutable_ids();
        ids.extend(B::mutable_ids());
        ids
    }
}

pub unsafe trait QueryFetch<'w>: QueryParam {
    type Item;
    unsafe fn fetch(
        ecs_ptr: &'w UnsafeECSCell,
        user_entity: &super::entity::UserEntity,
    ) -> Result<Option<Self::Item>, ErrorType>;
}

unsafe impl<'w, T: Component> QueryFetch<'w> for &T {
    type Item = &'w T;

    unsafe fn fetch(
        ecs_ptr: &'w UnsafeECSCell,
        user_entity: &super::entity::UserEntity,
    ) -> Result<Option<Self::Item>, ErrorType> {
        let component_manager = unsafe { &ecs_ptr.get().component_manager };

        let type_id = Self::component_ids()[0];
        let real_entity = match crate::ECS::get_real_entity(user_entity) {
            Ok(Some(id)) => id,
            Ok(None) => return Ok(None),
            Err(err) => {
                log_error!(
                    "Failed to get the real entity equivalent of the `{:?}' entity from a query parameter: {:?}",
                    user_entity,
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let component: &dyn RealComponent = match component_manager.get(&type_id, &real_entity) {
            Ok(component) => component,
            Err(ErrorType::DoesNotExist) => {
                log_warn!(
                    "The `{:?}' component for entity `{:?}' doesn't exist",
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

    unsafe fn fetch(
        ecs_ptr: &'w UnsafeECSCell,
        user_entity: &super::entity::UserEntity,
    ) -> Result<Option<Self::Item>, ErrorType> {
        let component_manager = unsafe { &mut ecs_ptr.get_mut().component_manager };
        let type_id = Self::mutable_ids()[0];
        let real_entity = match crate::ECS::get_real_entity(user_entity) {
            Ok(Some(id)) => id,
            Ok(None) => return Ok(None),
            Err(err) => {
                log_error!(
                    "Failed to get the real entity equivalent of the `{:?}' entity from a query parameter: {:?}",
                    user_entity,
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let component: &mut dyn RealComponent =
            match component_manager.get_mut(&type_id, &real_entity) {
                Ok(component) => component,
                Err(ErrorType::DoesNotExist) => {
                    log_warn!(
                        "The `{:?}' component for entity `{:?}' doesn't exist",
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

unsafe impl<'w, A, B> QueryFetch<'w> for (A, B)
where
    A: QueryFetch<'w>,
    B: QueryFetch<'w>,
{
    type Item = (A::Item, B::Item);

    unsafe fn fetch(
        ecs_ptr: &'w UnsafeECSCell,
        user_entity: &super::entity::UserEntity,
    ) -> Result<Option<Self::Item>, ErrorType> {
        let component_a = match unsafe { A::fetch(ecs_ptr, user_entity) } {
            Ok(Some(component)) => component,
            Ok(None) => return Ok(None),
            Err(err) => {
                log_error!(
                    "Failed to fetch a component in a tuple query fetch: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let component_b = match unsafe { B::fetch(ecs_ptr, user_entity) } {
            Ok(Some(component)) => component,
            Ok(None) => return Ok(None),
            Err(err) => {
                log_error!(
                    "Failed to fetch a component in a tuple query fetch: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        Ok(Some((component_a, component_b)))
    }
}

// fn CullingSystem(
//     q1: Query<(&CameraComponent, &TransformComponent), With<IsActivated>>,
//     q2: Query<(&MeshComponent, &TransformComponent, &mut CameraVisibilityComponent)>,
//     ) -> Result<(), ErrorType> {
//         for ((camera, transform), entity) in &q1.with_entities() {
//             let frustum = camera.get_frustum(transform);
//             for (mesh, transform, mut visibility) in &mut q2 {
//                 if frustum.intersects(mesh.get_aabb(transform)) {
//                     visibility[entity] = true;
//                 }
//             }
//         }
//         Ok(())
//     }

pub trait QueryFilterList {
    fn component_ids() -> Vec<std::any::TypeId>;
}
impl<T: Component> QueryFilterList for T {
    fn component_ids() -> Vec<std::any::TypeId> {
        vec![T::get_type_id()]
    }
}
impl<A, B> QueryFilterList for (A, B)
where
    A: QueryFilterList,
    B: QueryFilterList,
{
    fn component_ids() -> Vec<std::any::TypeId> {
        let mut ids = A::component_ids();
        ids.extend(B::component_ids());
        ids
    }
}

pub struct With<T: QueryFilterList>(std::marker::PhantomData<T>);
pub struct Without<T: QueryFilterList>(std::marker::PhantomData<T>);
pub trait QueryFilter {
    fn with() -> Vec<std::any::TypeId>;
    fn without() -> Vec<std::any::TypeId>;
}
impl QueryFilter for () {
    fn with() -> Vec<std::any::TypeId> {
        vec![]
    }
    fn without() -> Vec<std::any::TypeId> {
        vec![]
    }
}

impl<T: QueryFilterList> QueryFilter for With<T> {
    fn with() -> Vec<std::any::TypeId> {
        T::component_ids()
    }
    fn without() -> Vec<std::any::TypeId> {
        vec![]
    }
}
impl<T: QueryFilterList> QueryFilter for Without<T> {
    fn with() -> Vec<std::any::TypeId> {
        vec![]
    }
    fn without() -> Vec<std::any::TypeId> {
        T::component_ids()
    }
}
impl<In: QueryFilterList, Out: QueryFilterList> QueryFilter for (With<In>, Without<Out>) {
    fn with() -> Vec<std::any::TypeId> {
        In::component_ids()
    }
    fn without() -> Vec<std::any::TypeId> {
        Out::component_ids()
    }
}
impl<Out: QueryFilterList, In: QueryFilterList> QueryFilter for (Without<Out>, With<In>) {
    fn with() -> Vec<std::any::TypeId> {
        In::component_ids()
    }
    fn without() -> Vec<std::any::TypeId> {
        Out::component_ids()
    }
}

pub struct Query<'w, 's, Q, F = ()>
where
    Q: QueryFetch<'w>,
    F: QueryFilter,
{
    _marker: std::marker::PhantomData<(Q, F)>,
    pub(crate) ecs_ptr: &'w UnsafeECSCell,
    pub(crate) entities: &'s std::collections::HashSet<super::entity::Entity>,
}

pub struct QueryState {
    with: Vec<std::any::TypeId>,
    without: Vec<std::any::TypeId>,
    entities: std::collections::HashSet<super::entity::Entity>,
}

impl<Q, F> super::system_v2::SystemParam for Query<'_, '_, Q, F>
where
    Q: for<'a> QueryFetch<'a>,
    F: QueryFilter,
{
    type State = QueryState;
    type Item<'w, 's> = Query<'w, 's, Q, F>;

    // TODO: add fucntions to update state
    // remove entity, add component to entity, remove component from entity, remove component

    fn init_state(ecs: &crate::ECS) -> Result<Self::State, ErrorType> {
        let with = F::with();
        let without = F::without();
        let mut entities = std::collections::HashSet::new();

        let component_manager = &ecs.component_manager;
        for entity in &ecs.entities {
            'inner_loop: for component_id in &Q::component_ids() {
                match component_manager.has_component_type(entity, component_id) {
                    Ok(false) => break 'inner_loop,
                    Ok(true) => {
                        match component_manager.has_correct_constraints(
                            entity,
                            &F::with(),
                            &F::without(),
                        ) {
                            Ok(false) => break 'inner_loop,
                            Err(err) => {
                                log_error!(
                                    "Failed to check if an entry has the correct constraints when adding an entity to a query parameter: {:?}",
                                    err
                                );
                                return Err(ErrorType::Unknown);
                            }
                            _ => {}
                        }
                    }
                    Err(err) => {
                        log_error!(
                            "Failed to check if an entry has the given component type when adding an entity to a query parameter: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                }
            }

            if !entities.insert(*entity) {
                log_error!(
                    "Failed to insert a new entity in a query parameter, the entity was already present"
                );
                return Err(ErrorType::Duplicate);
            }
        }

        Ok(Self::State {
            with,
            without,
            entities,
        })
    }

    unsafe fn get_item<'w, 's>(
        state: &'s mut Self::State,
        ecs: &'w UnsafeECSCell,
    ) -> Result<Self::Item<'w, 's>, ErrorType> {
        Ok(Query {
            _marker: std::marker::PhantomData,
            ecs_ptr: ecs,
            entities: &state.entities,
        })
    }
}

//////////////////////////////////////////////////////////
///////////////     query tests      ////////////////////
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
        let mut ecs_ptr = UnsafeECSCell::new(&mut ecs);
        ecs.register_component(&id_1, &register_1).unwrap();
        ecs.register_component(&id_2, &register_2).unwrap();

        let entities = std::collections::HashSet::new();

        let mut _query = Query::<&NewComponent1> {
            _marker: std::marker::PhantomData::default(),
            ecs_ptr: &mut ecs_ptr,
            entities: &entities,
        };

        let mut _query = Query::<&NewComponent1, With<NewComponent2>> {
            _marker: std::marker::PhantomData::default(),
            ecs_ptr: &mut ecs_ptr,
            entities: &entities,
        };

        let mut _query = Query::<&NewComponent1, Without<NewComponent2>> {
            _marker: std::marker::PhantomData::default(),
            ecs_ptr: &mut ecs_ptr,
            entities: &entities,
        };

        let mut _query = Query::<(&NewComponent1, &mut NewComponent2)> {
            _marker: std::marker::PhantomData::default(),
            ecs_ptr: &mut ecs_ptr,
            entities: &entities,
        };

        let mut _query = Query::<&NewComponent1, Without<NewComponent1>> {
            _marker: std::marker::PhantomData::default(),
            ecs_ptr: &mut ecs_ptr,
            entities: &entities,
        };

        let mut _query2 = Query::<&NewComponent1, Without<NewComponent2>> {
            _marker: std::marker::PhantomData::default(),
            ecs_ptr: &mut ecs_ptr,
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

        let ids = Without::<(DefaultComponent, (DefaultComponent, DefaultComponent))>::without();
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
