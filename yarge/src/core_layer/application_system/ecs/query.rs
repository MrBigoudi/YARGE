#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::core_layer::application_system::ecs::component::{Component, RealComponent};

pub(crate) struct UnsafeECSCell {
    ptr: *mut crate::ECS,
}

impl UnsafeECSCell {
    pub(crate) fn new(ecs: &mut crate::ECS) -> Self {
        let ptr: *mut crate::ECS = ecs;
        Self {
            ptr,
        }
    }

    #[inline]
    pub(crate) unsafe fn get(&self) -> &crate::ECS {
        unsafe { &(*self.ptr) }
    }

    #[inline]
    pub(crate) unsafe fn get_mut(&self) -> &mut crate::ECS {
        unsafe { &mut (*self.ptr) }
    }
}

pub(crate) trait QueryParam {
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

impl<A: QueryParam, B: QueryParam> QueryParam for (A,B) {
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

pub(crate) unsafe trait QueryFetch<'w>: QueryParam {
    type Item;
    unsafe fn fetch(ecs_ptr: &'w UnsafeECSCell, user_entity: &super::entity::UserEntity,) -> Result<Option<Self::Item>, ErrorType>;
}


unsafe impl<'w, T: Component> QueryFetch<'w> for &T {
    type Item = &'w T;

    unsafe fn fetch(ecs_ptr: &'w UnsafeECSCell, user_entity: &super::entity::UserEntity,) -> Result<Option<Self::Item>, ErrorType> {
        let component_manager = unsafe { &ecs_ptr.get().component_manager };

        let type_id = Self::component_ids()[0];
        let real_entity = match crate::ECS::get_real_entity(&user_entity) {
            Ok(Some(id)) => id,
            Ok(None) => return Ok(None),
            Err(err) => {
                log_error!("Failed to get the real entity equivalent of the `{:?}' entity from a query parameter: {:?}",
                    user_entity, err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let component: &dyn RealComponent = match component_manager.get(&type_id, &real_entity) {
            Ok(component) => component,
            Err(ErrorType::DoesNotExist) => {
                log_warn!("The `{:?}' component for entity `{:?}' doesn't exist",
                    std::any::type_name::<T>(), user_entity,
                );
                return Ok(None);
            }
            Err(err) => {
                log_error!("Failed to fetch a `{:?}' component from a query: {:?}",
                    std::any::type_name::<T>(), err
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
                return Err(ErrorType::Unknown);
            },
        }
    }
}

unsafe impl<'w, T: Component> QueryFetch<'w> for &mut T {
    type Item = &'w mut T;

    unsafe fn fetch(ecs_ptr: &'w UnsafeECSCell, user_entity: &super::entity::UserEntity,) -> Result<Option<Self::Item>, ErrorType> {
        let component_manager = unsafe { &mut ecs_ptr.get_mut().component_manager };
        let type_id = Self::mutable_ids()[0];
        let real_entity = match crate::ECS::get_real_entity(&user_entity) {
            Ok(Some(id)) => id,
            Ok(None) => return Ok(None),
            Err(err) => {
                log_error!("Failed to get the real entity equivalent of the `{:?}' entity from a query parameter: {:?}",
                    user_entity, err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let component: &mut dyn RealComponent = match component_manager.get_mut(&type_id, &real_entity) {
            Ok(component) => component,
            Err(ErrorType::DoesNotExist) => {
                log_warn!("The `{:?}' component for entity `{:?}' doesn't exist",
                    std::any::type_name::<T>(), user_entity,
                );
                return Ok(None);
            }
            Err(err) => {
                log_error!("Failed to fetch a `{:?}' component from a mutable query: {:?}",
                    std::any::type_name::<T>(), err
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
                return Err(ErrorType::Unknown);
            },
        }
    }
}

unsafe impl<'w, A, B> QueryFetch<'w> for (A,B) 
    where 
        A: QueryFetch<'w>, 
        B: QueryFetch<'w>,
    {
    type Item = (A::Item, B::Item);

    unsafe fn fetch(ecs_ptr: &'w UnsafeECSCell, user_entity: &super::entity::UserEntity,) -> Result<Option<Self::Item>, ErrorType> {
        let component_a = match unsafe { A::fetch(ecs_ptr, user_entity) } {
            Ok(Some(component)) => component,
            Ok(None) => return Ok(None),
            Err(err) => {
                log_error!("Failed to fetch a component in a tuple query fetch: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let component_b = match unsafe { B::fetch(ecs_ptr, user_entity) } {
            Ok(Some(component)) => component,
            Ok(None) => return Ok(None),
            Err(err) => {
                log_error!("Failed to fetch a component in a tuple query fetch: {:?}",
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

pub(crate) trait QueryFilterList {
    fn component_ids() -> Vec<std::any::TypeId>;
}
impl<T: Component> QueryFilterList for T {
    fn component_ids() -> Vec<std::any::TypeId> {
        vec![T::get_type_id()]
    }
}
impl<A, B> QueryFilterList for (A,B) 
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

pub(crate) struct With<T: QueryFilterList>(std::marker::PhantomData<T>);
pub(crate) struct Without<T: QueryFilterList>(std::marker::PhantomData<T>);
pub(crate) trait QueryFilter {
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



pub(crate) struct Query<'w, Q, F = ()>
    where
        Q: QueryFetch<'w>,
        F: QueryFilter,
    {
    _marker: std::marker::PhantomData<(Q, F)>,
    pub(crate) ecs_ptr: &'w mut UnsafeECSCell,
    pub(crate) entities: std::collections::HashSet<super::entity::UserEntity>,
}

impl<'w, Q, F> Query<'w, Q, F>
    where
        Q: QueryFetch<'w>,
        F: QueryFilter,
    {

    pub(crate) fn add_entities(
        &mut self,
        entities: &[super::entity::UserEntity],
    ) -> Result<(), ErrorType> {
        for entity in entities {
            if let Err(err) = self.add_entity(entity) {
                log_error!(
                    "Failed to add an entity to a query when trying to add multiple entities in a query: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        }
        Ok(())
    }
    
    pub(crate) fn add_entity(
        &mut self,
        user_entity: &super::entity::UserEntity,
    ) -> Result<(), ErrorType> {
        let component_manager = unsafe { &self.ecs_ptr.get().component_manager };
        let entity = match crate::ECS::get_real_entity(&user_entity) {
            Ok(Some(id)) => id,
            Ok(None) => return Ok(()),
            Err(err) => {
                log_error!("Failed to get the real entity equivalent of the `{:?}' entity when adding an entity in a query: {:?}",
                    user_entity, err
                );
                return Err(ErrorType::Unknown);
            }
        };

        for component_id in &Q::component_ids() {
            match component_manager.has_component_type(&entity, component_id) {
                Ok(false) => return Ok(()),
                Ok(true) => {
                    match component_manager.has_correct_constraints(&entity, &F::with(), &F::without()) {
                        Ok(false) => return Ok(()),
                        Err(err) => {
                            log_error!(
                                "Failed to check if an entry has the correct constraints when adding an entity to a query: {:?}",
                                err
                            );
                            return Err(ErrorType::Unknown);
                        }
                        _ => {},
                    }
                }
                Err(err) => {
                    log_error!(
                        "Failed to check if an entry has the given component type when adding an entity to a query: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
        }

        if !self.entities.insert(*user_entity) {
            log_error!(
                "Failed to insert a new entity in a query, the entity was already present"
            );
            return Err(ErrorType::Duplicate);
        }
        Ok(())
    }

    pub(crate) fn remove_entity(&mut self, entity: &super::entity::UserEntity) -> Result<(), ErrorType> {
        if !self.entities.remove(entity) {
            log_error!("Trying to remove an entity not present in a query");
            return Err(ErrorType::DoesNotExist);
        }
        Ok(())
    }

    pub(crate) fn remove_entity_unchecked(&mut self, entity: &super::entity::UserEntity) {
        if !self.entities.remove(entity) {
            log_warn!("Trying to remove an entity not present in a query");
        }
    }

    pub(crate) fn remove_entities(&mut self, entities: &[super::entity::UserEntity]) -> Result<(), ErrorType> {
        for entity in entities {
            if let Err(err) = self.remove_entity(entity) {
                log_error!(
                    "Failed to remove an entity from a query when trying to remove multiple entities in a query: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        }
        Ok(())
    }

    pub(crate) fn remove_entities_unchecked(&mut self, entities: &[super::entity::UserEntity]) {
        for entity in entities {
            self.remove_entity_unchecked(entity);
        }
    }
}

//////////////////////////////////////////////////////////
///////////////     query tests      ////////////////////
//////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_layer::application_system::ecs::component::{AddComponentToEntityFunction, DefaultComponent, RegisterComponentFunction};

    #[test]
    fn initialization() {
        // Create new components
        struct NewComponent1 {
            value: u32,
        }
        struct NewComponent2;

        impl crate::Component for NewComponent1{}
        impl crate::Component for NewComponent2{}
        // Helpers to play with components
        let id_1 = NewComponent1::get_type_id();
        let add_1: AddComponentToEntityFunction = NewComponent1::add_to_entity;
        let register_1: RegisterComponentFunction = NewComponent1::register;
        let id_2 = NewComponent2::get_type_id();
        let add_2: AddComponentToEntityFunction = NewComponent2::add_to_entity;
        let register_2: RegisterComponentFunction = NewComponent2::register;

        // Init ecs
        let mut ecs = crate::ECS::init().unwrap();
        let mut ecs_ptr = UnsafeECSCell::new(&mut ecs);
        ecs.register_component(&id_1, &register_1).unwrap();
        ecs.register_component(&id_2, &register_2).unwrap();

        // Generate an entity
        let entities = crate::ECS::spawn_empty_entities(2).unwrap();
        ecs.spawn_real_entities().unwrap();
        let entity_0 = &entities[0];
        let entity_1 = &entities[1];

        // Create values for each entity
        let value_1_entity_0 = Box::new(NewComponent1{value: 0});
        let value_2_entity_0 = Box::new(NewComponent2{});
        let value_1_entity_1 = Box::new(NewComponent1{value: 1});
        let _value_2_entity_1 = Box::new(NewComponent2{});

        // Add Component 1 and 2 to entity 0
        ecs.add_component_to_entity(&id_1, entity_0, value_1_entity_0, &add_1).unwrap();
        ecs.add_component_to_entity(&id_2, entity_0, value_2_entity_0, &add_2).unwrap();
        // Add Component 1 to entity 1
        ecs.add_component_to_entity(&id_1, entity_1, value_1_entity_1, &add_1).unwrap();

        // Adds an entity to the query, should add both
        let mut query = Query::<&NewComponent1>{
            _marker: std::marker::PhantomData::default(),
            ecs_ptr: &mut ecs_ptr,
            entities: std::collections::HashSet::new(),
        };
        query.add_entity(&entity_0).unwrap();
        query.add_entity(&entity_1).unwrap();
        assert!(query.entities.contains(&entity_0), "Test 0, entity 0 should have been in the list");
        assert!(query.entities.contains(&entity_1), "Test 0, entity 1 should have been in the list");

        // Adds an entity to the query, should add only entity 0
        let mut query = Query::<&NewComponent1, With::<NewComponent2>>{
            _marker: std::marker::PhantomData::default(),
            ecs_ptr: &mut ecs_ptr,
            entities: std::collections::HashSet::new(),
        };
        query.add_entity(&entity_0).unwrap();
        query.add_entity(&entity_1).unwrap();
        assert!(query.entities.contains(&entity_0), "Test 1, entity 0 should have been in the list");
        assert!(!query.entities.contains(&entity_1), "Test 1, entity 1 should not have been in the list");

        // Adds an entity to the query, should add only entity 1
        let mut query = Query::<&NewComponent1, Without::<NewComponent2>>{
            _marker: std::marker::PhantomData::default(),
            ecs_ptr: &mut ecs_ptr,
            entities: std::collections::HashSet::new(),
        };
        query.add_entity(&entity_0).unwrap();
        query.add_entity(&entity_1).unwrap();
        assert!(!query.entities.contains(&entity_0), "Test 2, entity 0 should not have been in the list");
        assert!(query.entities.contains(&entity_1), "Test 2, entity 1 should have been in the list");

        // Adds an entity to the query, should add only entity 0
        let mut query = Query::<(&NewComponent1, &mut NewComponent2)>{
            _marker: std::marker::PhantomData::default(),
            ecs_ptr: &mut ecs_ptr,
            entities: std::collections::HashSet::new(),
        };
        query.add_entity(&entity_0).unwrap();
        query.add_entity(&entity_1).unwrap();
        assert!(query.entities.contains(&entity_0), "Test 3, entity 0 should have been in the list");
        assert!(!query.entities.contains(&entity_1), "Test 3, entity 1 should not have been in the list");

        // Adds an entity to the query, should add none
        let mut query = Query::<&NewComponent1, Without::<NewComponent1>>{
            _marker: std::marker::PhantomData::default(),
            ecs_ptr: &mut ecs_ptr,
            entities: std::collections::HashSet::new(),
        };
        query.add_entity(&entity_0).unwrap();
        query.add_entity(&entity_1).unwrap();
        assert!(!query.entities.contains(&entity_0), "Test 4, entity 0 should not have been in the list");
        assert!(!query.entities.contains(&entity_1), "Test 4, entity 1 should not have been in the list");

        // Adds an entity to the query, should add only entity 1
        let mut query = Query::<&NewComponent1, Without::<NewComponent2>>{
            _marker: std::marker::PhantomData::default(),
            ecs_ptr: &mut ecs_ptr,
            entities: std::collections::HashSet::new(),
        };
        query.add_entities(&entities).unwrap();
        assert!(!query.entities.contains(&entity_0), "Test 5, entity 0 should not have been in the list");
        assert!(query.entities.contains(&entity_1), "Test 5, entity 1 should have been in the list");
        assert!(query.remove_entity(entity_0).is_err(), "Test 6, removing entity 0 should have failed");
        assert!(query.remove_entity(entity_1).is_ok(), "Test 6, removing entity 1 should not have failed");
        assert!(!query.entities.contains(&entity_0), "Test 7, entity 0 should not have been in the list");
        assert!(!query.entities.contains(&entity_1), "Test 7, entity 1 should not have been in the list");
        query.add_entities(&entities).unwrap();
        assert!(!query.entities.contains(&entity_0), "Test 8, entity 0 should not have been in the list");
        assert!(query.entities.contains(&entity_1), "Test 8, entity 1 should have been in the list");
        query.remove_entities_unchecked(&entities);
        assert!(!query.entities.contains(&entity_0), "Test 9, entity 0 should not have been in the list");
        assert!(!query.entities.contains(&entity_1), "Test 9, entity 1 should not have been in the list");
        assert!(query.remove_entities(&entities).is_err(), "Test 10, removing entities should have failed");
    }

    #[test]
    fn filters() {
        let ids = With::<DefaultComponent>::with();
        assert_eq!(ids, vec![DefaultComponent::get_type_id()]);

        let ids = With::<(DefaultComponent, DefaultComponent)>::with();
        assert_eq!(ids, 
            vec![
                DefaultComponent::get_type_id(),
                DefaultComponent::get_type_id(),
            ]
        );

        let ids = Without::<DefaultComponent>::without();
        assert_eq!(ids, vec![DefaultComponent::get_type_id()]);

        let ids = Without::<(DefaultComponent, (DefaultComponent, DefaultComponent))>::without();
        assert_eq!(ids, 
            vec![
                DefaultComponent::get_type_id(),
                DefaultComponent::get_type_id(),
                DefaultComponent::get_type_id(),
            ]
        );
    }

}