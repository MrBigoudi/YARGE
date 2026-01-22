#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

pub trait SystemParam {
    type State: 'static;
    type Item<'w, 's>;

    // TODO: add fucntions to update state
    // remove entity, add component to entity, remove component from entity, remove component

    fn init_state(game: &dyn crate::Game, ecs: &crate::ECS) -> Result<Self::State, ErrorType>;
    unsafe fn get_item<'w, 's>(
        state: &'s mut Self::State,
        game_ptr: &'w crate::UnsafeGameCell,
        ecs_ptr: &'w crate::UnsafeECSCell,
    ) -> Result<Self::Item<'w, 's>, ErrorType>;
}

impl SystemParam for () {
    type State = ();
    type Item<'w, 's> = ();

    fn init_state(_game: &dyn crate::Game, _ecs: &crate::ECS) -> Result<Self::State, ErrorType> {
        Ok(())
    }

    unsafe fn get_item<'w, 's>(
        _state: &'s mut Self::State,
        _game_ptr: &'w crate::UnsafeGameCell,
        _ecs_ptr: &'w crate::UnsafeECSCell,
    ) -> Result<Self::Item<'w, 's>, ErrorType> {
        Ok(())
    }
}

impl<A, B> SystemParam for (A, B)
where
    A: SystemParam,
    B: SystemParam,
{
    type State = (A::State, B::State);
    type Item<'w, 's> = (A::Item<'w, 's>, B::Item<'w, 's>);

    fn init_state(game: &dyn crate::Game, ecs: &crate::ECS) -> Result<Self::State, ErrorType> {
        Ok((A::init_state(game, ecs)?, B::init_state(game, ecs)?))
    }

    // TODO: add fucntions to update state
    // remove entity, add component to entity, remove component from entity, remove component

    unsafe fn get_item<'w, 's>(
        state: &'s mut Self::State,
        game_ptr: &'w crate::UnsafeGameCell,
        ecs_ptr: &'w crate::UnsafeECSCell,
    ) -> Result<Self::Item<'w, 's>, ErrorType> {
        let item_a = unsafe { A::get_item(&mut state.0, game_ptr, ecs_ptr) }?;
        let item_b = unsafe { B::get_item(&mut state.1, game_ptr, ecs_ptr) }?;
        Ok((item_a, item_b))
    }
}

// TODO: impl SystemParam for game
impl<T> SystemParam for &mut T
where
    T: crate::Game + 'static,
{
    type State = ();
    type Item<'w, 's> = &'w mut T;

    fn init_state(_game: &dyn crate::Game, _ecs: &crate::ECS) -> Result<Self::State, ErrorType> {
        Ok(())
    }

    unsafe fn get_item<'w, 's>(
        _state: &'s mut Self::State,
        game_ptr: &'w crate::UnsafeGameCell,
        _ecs_ptr: &'w crate::UnsafeECSCell,
    ) -> Result<Self::Item<'w, 's>, ErrorType> {
        let any: &mut (dyn std::any::Any + 'static) = unsafe { game_ptr.get_mut() };
        let game = match any.downcast_mut::<T>() {
            Some(game) => game,
            None => {
                log_error!(
                    "Failed to downcast the mutable `{:?}' user game when using it as a system parameter",
                    std::any::type_name::<T>(),
                );
                return Err(ErrorType::Unknown);
            }
        };
        Ok(game)
    }
}
impl<T> SystemParam for &T
where
    T: crate::Game + 'static,
{
    type State = ();
    type Item<'w, 's> = &'w T;

    fn init_state(_game: &dyn crate::Game, _ecs: &crate::ECS) -> Result<Self::State, ErrorType> {
        Ok(())
    }

    unsafe fn get_item<'w, 's>(
        _state: &'s mut Self::State,
        game_ptr: &'w crate::UnsafeGameCell,
        _ecs_ptr: &'w crate::UnsafeECSCell,
    ) -> Result<Self::Item<'w, 's>, ErrorType> {
        let any: &(dyn std::any::Any + 'static) = unsafe { game_ptr.get() };
        let game = match any.downcast_ref::<T>() {
            Some(game) => game,
            None => {
                log_error!(
                    "Failed to downcast the `{:?}' user game when using it as a system parameter",
                    std::any::type_name::<T>(),
                );
                return Err(ErrorType::Unknown);
            }
        };
        Ok(game)
    }
}

pub struct SystemFuncWrapper<Func, Param>
where
    Func: for<'w, 's> FnMut(Param::Item<'w, 's>) -> Result<(), ErrorType> + 'static,
    Param: SystemParam,
{
    pub(crate) function: Func,
    pub(crate) state: Option<Param::State>,
    _marker: std::marker::PhantomData<Param>,
}
impl<Func, Param> SystemTrait for SystemFuncWrapper<Func, Param>
where
    Func: for<'w, 's> FnMut(Param::Item<'w, 's>) -> Result<(), ErrorType> + 'static,
    Param: SystemParam,
{
    fn init(&mut self, game: &dyn crate::Game, ecs: &crate::ECS) -> Result<(), ErrorType> {
        self.state = Some(Param::init_state(game, ecs)?);
        Ok(())
    }
    fn run(
        &mut self,
        game_ptr: &crate::UnsafeGameCell,
        ecs_ptr: &crate::UnsafeECSCell,
    ) -> Result<(), ErrorType> {
        let state = match &mut self.state {
            None => return Err(ErrorType::Unknown),
            Some(state) => state,
        };
        let param = unsafe { Param::get_item(state, game_ptr, ecs_ptr) }?;
        (self.function)(param)
    }
}

pub trait IntoSystem {
    fn into_system(self) -> Box<dyn SystemTrait>;
}

pub trait SystemTrait {
    // TODO: add fucntions to update state
    // remove entity, add component to entity, remove component from entity, remove component

    fn init(&mut self, game: &dyn crate::Game, ecs: &crate::ECS) -> Result<(), ErrorType>;
    fn run(
        &mut self,
        game_ptr: &crate::UnsafeGameCell,
        ecs_ptr: &crate::UnsafeECSCell,
    ) -> Result<(), ErrorType>;
}

pub(crate) struct SystemInternalV2 {
    pub(crate) schedule: super::system::SystemSchedule,
    updates_counter: usize,
    pub(crate) condition: super::system::SystemCallbackConditionFunction,
}

impl SystemInternalV2 {
    pub(crate) fn new(
        schedule: super::system::SystemSchedule,
        condition: super::system::SystemCallbackConditionFunction,
    ) -> Self {
        Self {
            schedule,
            updates_counter: 0,
            condition,
        }
    }

    pub(crate) fn should_run_this_update(&mut self) -> bool {
        match self.schedule {
            super::system::SystemSchedule::Never => false,
            super::system::SystemSchedule::SingleCall => {
                self.schedule = super::system::SystemSchedule::Never;
                true
            }
            super::system::SystemSchedule::Always => true,
            super::system::SystemSchedule::ForXUpdates(nb_frames_remaining) => {
                if nb_frames_remaining == 1 {
                    self.schedule = super::system::SystemSchedule::Never;
                } else {
                    self.schedule =
                        super::system::SystemSchedule::ForXUpdates(nb_frames_remaining - 1);
                }
                true
            }
            super::system::SystemSchedule::EveryXUpdates(nb_frames_to_wait) => {
                if self.updates_counter.is_multiple_of(nb_frames_to_wait) {
                    self.updates_counter = 1; // go back to 1 so we always avoid usize overflow
                    true
                } else {
                    self.updates_counter += 1;
                    false
                }
            }
        }
    }
}

pub(crate) struct SystemWrapper {
    system_trait: Box<dyn SystemTrait>,
    pub(crate) internal: SystemInternalV2,
}

impl SystemWrapper {
    pub(crate) fn should_run_this_update(&mut self) -> bool {
        self.internal.should_run_this_update()
    }
}

// The System Manager
pub(crate) struct SystemManagerV2 {
    systems: Vec<SystemWrapper>,
}

impl SystemManagerV2 {
    pub(crate) fn init() -> Self {
        log_info!("System manager V2 initialized");
        Self {
            systems: Vec::new(),
        }
    }

    /// Add a system without using generics in the function signature
    /// The generic constraint is on the impl block instead
    pub(crate) fn add_system(
        &mut self,
        internal: SystemInternalV2,
        system: Box<dyn SystemTrait>,
    ) -> Result<(), ErrorType> {
        let wrapper = SystemWrapper {
            system_trait: system,
            internal,
        };

        self.systems.push(wrapper);
        Ok(())
    }

    pub(crate) fn clean_dead_systems(&mut self) {
        let indices_to_remove: Vec<usize> = self
            .systems
            .iter()
            .enumerate()
            .filter(|&(_, val)| val.internal.schedule == super::system::SystemSchedule::Never)
            .map(|(index, _)| index)
            .collect();
        for index in indices_to_remove.into_iter().rev() {
            let _ = self.systems.drain(index..index + 1);
        }
    }

    /// Run all systems
    pub(crate) fn run_all(
        &mut self,
        game: &mut dyn crate::Game,
        ecs_ptr: &crate::UnsafeECSCell,
    ) -> Result<(), ErrorType> {
        for system in &mut self.systems {
            match (system.internal.condition)(game) {
                Ok(should_run) => {
                    if should_run && system.should_run_this_update() {
                        let game_ptr = crate::UnsafeGameCell::new(game);
                        if let Err(err) = system.system_trait.run(&game_ptr, ecs_ptr) {
                            log_error!("Failed to run a system: {:?}", err);
                            return Err(ErrorType::Unknown);
                        }
                    }
                }
                Err(err) => {
                    log_error!(
                        "Failed to check the condition when running systems: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
        }

        self.clean_dead_systems();

        Ok(())
    }
}

//////////////////////////////////////////////////////////
///////////////     systems tests      ////////////////////
//////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::super::query::{Query, With, Without};
    use super::super::system::{SystemSchedule, UserSystemCallbackBuilder};
    use super::*;
    use crate::core_layer::application_system::ecs::component::{
        AddComponentToEntityFunction, Component, RegisterComponentFunction,
    };

    // Create new components
    struct NewComponent1 {
        value: u32,
    }
    struct NewComponent2;

    impl crate::Component for NewComponent1 {}
    impl crate::Component for NewComponent2 {}

    struct TestGame {
        test: u32,
    }
    impl crate::Game for TestGame {}

    macro_rules! default_system_internal {
        () => {
            SystemInternalV2::new(
                SystemSchedule::default(),
                UserSystemCallbackBuilder::default_condition(),
            )
        };
    }

    #[test]
    fn initialization() {
        // Init Game
        let game = TestGame { test: 0u32 };
        // Init ecs
        let mut ecs = crate::ECS::init().unwrap();
        let register_1: RegisterComponentFunction = NewComponent1::register;
        let register_2: RegisterComponentFunction = NewComponent2::register;
        ecs.register_component(&NewComponent1::get_type_id(), &register_1)
            .unwrap();
        ecs.register_component(&NewComponent2::get_type_id(), &register_2)
            .unwrap();

        #[macros::system]
        fn test_system_0() -> Result<(), ErrorType> {
            log_info!("Inside test system with 0 params");
            Ok(())
        }
        #[macros::system]
        fn test_system_1(
            _query: Query<'_, '_, &NewComponent1, With<NewComponent2>>,
        ) -> Result<(), ErrorType> {
            log_info!("Inside test system with 1 params");
            Ok(())
        }
        #[macros::system]
        fn test_system_2(
            _q1: Query<'_, '_, &NewComponent1, Without<NewComponent2>>,
            _q2: Query<'_, '_, &NewComponent1, Without<NewComponent2>>,
        ) -> Result<(), ErrorType> {
            log_info!("Inside test system with 2 params");
            Ok(())
        }

        let internal_0 = default_system_internal!();
        let mut system_0 = test_system_0.into_system();
        system_0.init(&game, &ecs).unwrap();
        ecs.system_manager_2
            .add_system(internal_0, system_0)
            .unwrap();

        let internal_1 = default_system_internal!();
        let mut system_1 = test_system_1.into_system();
        system_1.init(&game, &ecs).unwrap();
        ecs.system_manager_2
            .add_system(internal_1, system_1)
            .unwrap();

        let internal_2 = default_system_internal!();
        let mut system_2 = test_system_2.into_system();
        system_2.init(&game, &ecs).unwrap();
        ecs.system_manager_2
            .add_system(internal_2, system_2)
            .unwrap();
    }

    #[test]
    fn systems_with_game() {
        // Init Game
        let game = TestGame { test: 0u32 };
        // Init ecs
        let mut ecs = crate::ECS::init().unwrap();

        #[macros::system]
        fn test_system(_game: &mut TestGame) -> Result<(), ErrorType> {
            Ok(())
        }
        #[macros::system]
        fn test_system_query(
            _game: &TestGame,
            _query: Query<'_, '_, &NewComponent1>,
        ) -> Result<(), ErrorType> {
            Ok(())
        }

        let internal = default_system_internal!();
        let mut system = test_system.into_system();
        system.init(&game, &ecs).unwrap();
        ecs.system_manager_2.add_system(internal, system).unwrap();

        let internal = default_system_internal!();
        let mut system = test_system_query.into_system();
        system.init(&game, &ecs).unwrap();
        ecs.system_manager_2.add_system(internal, system).unwrap();
    }

    #[test]
    fn systems_as_function() {
        // Init ecs
        let mut ecs = crate::ECS::init().unwrap();

        #[macros::system]
        fn test_system() -> Result<(), ErrorType> {
            Ok(())
        }
        #[macros::system]
        fn test_system_err(_query: Query<'_, '_, &NewComponent1>) -> Result<(), ErrorType> {
            Err(ErrorType::Unknown)
        }
        #[macros::system]
        fn test_system_ok(
            _q1: Query<'_, '_, &NewComponent1>,
            _q2: Query<'_, '_, &NewComponent2>,
        ) -> Result<(), ErrorType> {
            Ok(())
        }
        let ecs_ptr = crate::UnsafeECSCell::new(&mut ecs);
        let entities = std::collections::HashSet::new();
        let query = Query::<&NewComponent1> {
            _marker: std::marker::PhantomData,
            ecs_ptr: &ecs_ptr,
            entities: &entities,
        };
        let q1 = Query::<&NewComponent1> {
            _marker: std::marker::PhantomData,
            ecs_ptr: &ecs_ptr,
            entities: &entities,
        };
        let q2 = Query::<&NewComponent2> {
            _marker: std::marker::PhantomData,
            ecs_ptr: &ecs_ptr,
            entities: &entities,
        };
        assert!(test_system().is_ok());
        assert!(test_system_err(query).is_err());
        assert!(test_system_ok(q1, q2).is_ok());
    }

    #[test]
    fn systems_with_game_running() {
        // Init Game
        let mut game = TestGame { test: 0u32 };
        // Init ecs
        let mut ecs = crate::ECS::init().unwrap();

        #[macros::system]
        fn test_system(game: &mut TestGame) -> Result<(), ErrorType> {
            game.test += 1u32;
            Ok(())
        }

        let internal = default_system_internal!();
        let mut system = test_system.into_system();
        system.init(&game, &ecs).unwrap();
        ecs.system_manager_2.add_system(internal, system).unwrap();
        assert_eq!(game.test, 0u32);

        let ecs_ptr = crate::UnsafeECSCell::new(&mut ecs);
        ecs.system_manager_2.run_all(&mut game, &ecs_ptr).unwrap();
        assert_eq!(game.test, 1u32);

        let ecs_ptr = crate::UnsafeECSCell::new(&mut ecs);
        ecs.system_manager_2.run_all(&mut game, &ecs_ptr).unwrap();
        assert_eq!(game.test, 2u32);

        assert!(test_system(&mut game).is_ok());
        assert_eq!(game.test, 3u32);
    }

    #[test]
    fn systems_with_queries_running() {
        // Helpers
        let id_1 = NewComponent1::get_type_id();
        let id_2 = NewComponent2::get_type_id();
        let register_1: RegisterComponentFunction = NewComponent1::register;
        let register_2: RegisterComponentFunction = NewComponent2::register;
        let add_to_entity_1: AddComponentToEntityFunction = NewComponent1::add_to_entity;
        let add_to_entity_2: AddComponentToEntityFunction = NewComponent2::add_to_entity;

        // Init Game
        let mut game = TestGame { test: 0u32 };
        // Init ecs
        let mut ecs = crate::ECS::init().unwrap();
        ecs.register_component(&id_1, &register_1).unwrap();
        ecs.register_component(&id_2, &register_2).unwrap();

        // Generate 3 entities
        let entities = crate::ECS::spawn_empty_entities(3).unwrap();
        ecs.spawn_real_entities().unwrap();
        let real_entity_0 = crate::ECS::get_real_entity(&entities[0]).unwrap().unwrap();
        let real_entity_1 = crate::ECS::get_real_entity(&entities[1]).unwrap().unwrap();

        // Add new component 1 and new component 2 to entity 0
        ecs.add_component_to_entity(
            &id_1,
            &entities[0],
            Box::new(NewComponent1 { value: 0u32 }),
            &add_to_entity_1,
        )
        .unwrap();
        ecs.add_component_to_entity(
            &id_2,
            &entities[0],
            Box::new(NewComponent2),
            &add_to_entity_2,
        )
        .unwrap();

        // Add new component 1 to entity 1
        ecs.add_component_to_entity(
            &id_1,
            &entities[1],
            Box::new(NewComponent1 { value: 1u32 }),
            &add_to_entity_1,
        )
        .unwrap();

        // Add new component 2 to entity 2
        ecs.add_component_to_entity(
            &id_2,
            &entities[2],
            Box::new(NewComponent2),
            &add_to_entity_2,
        )
        .unwrap();

        #[macros::system]
        fn test_system_simple(query: Query<'_, '_, &mut NewComponent1>) -> Result<(), ErrorType> {
            for component_1 in &query {
                component_1.value += 1u32;
            }
            Ok(())
        }

        #[macros::system]
        fn test_system_fitler_with(
            query: Query<'_, '_, &mut NewComponent1, With<NewComponent2>>,
        ) -> Result<(), ErrorType> {
            for component_1 in &query {
                component_1.value += 1u32;
            }
            Ok(())
        }

        #[macros::system]
        fn test_system_fitler_without(
            query: Query<'_, '_, &mut NewComponent1, Without<NewComponent2>>,
        ) -> Result<(), ErrorType> {
            for component_1 in &query {
                component_1.value += 1u32;
            }
            Ok(())
        }

        #[macros::system]
        fn test_system(
            game: &mut TestGame,
            query: Query<'_, '_, (&NewComponent1, &NewComponent2)>,
        ) -> Result<(), ErrorType> {
            for ((_component_1, _component_2), entity) in query.with_entities() {
                game.test += entity.0.index as u32;
            }
            Ok(())
        }

        // Add system 0 to ECS
        let internal = default_system_internal!();
        let mut system = test_system_simple.into_system();
        system.init(&game, &ecs).unwrap();
        ecs.system_manager_2.add_system(internal, system).unwrap();

        // Check initial values
        let value_entity_0 = ecs
            .component_manager
            .get(&id_1, &real_entity_0)
            .unwrap()
            .as_any()
            .downcast_ref::<NewComponent1>()
            .unwrap();
        assert_eq!(value_entity_0.value, 0u32);
        let value_entity_1 = ecs
            .component_manager
            .get(&id_1, &real_entity_1)
            .unwrap()
            .as_any()
            .downcast_ref::<NewComponent1>()
            .unwrap();
        assert_eq!(value_entity_1.value, 1u32);

        // Run the system 0
        let ecs_ptr = crate::UnsafeECSCell::new(&mut ecs);
        ecs.system_manager_2.run_all(&mut game, &ecs_ptr).unwrap();

        // Check new values after running system 0
        let value_entity_0 = ecs
            .component_manager
            .get(&id_1, &real_entity_0)
            .unwrap()
            .as_any()
            .downcast_ref::<NewComponent1>()
            .unwrap();
        assert_eq!(value_entity_0.value, 1u32);
        let value_entity_1 = ecs
            .component_manager
            .get(&id_1, &real_entity_1)
            .unwrap()
            .as_any()
            .downcast_ref::<NewComponent1>()
            .unwrap();
        assert_eq!(value_entity_1.value, 2u32);

        // Add system 1 to ECS
        let internal = default_system_internal!();
        let mut system = test_system_fitler_with.into_system();
        system.init(&game, &ecs).unwrap();
        ecs.system_manager_2.add_system(internal, system).unwrap();

        // Run the systems 0 and 1
        let ecs_ptr = crate::UnsafeECSCell::new(&mut ecs);
        ecs.system_manager_2.run_all(&mut game, &ecs_ptr).unwrap();

        // Check new values after running systems 0 and 1
        let value_entity_0 = ecs
            .component_manager
            .get(&id_1, &real_entity_0)
            .unwrap()
            .as_any()
            .downcast_ref::<NewComponent1>()
            .unwrap();
        assert_eq!(value_entity_0.value, 3u32);
        let value_entity_1 = ecs
            .component_manager
            .get(&id_1, &real_entity_1)
            .unwrap()
            .as_any()
            .downcast_ref::<NewComponent1>()
            .unwrap();
        assert_eq!(value_entity_1.value, 3u32);

        // Add system 2 to ECS
        let internal = default_system_internal!();
        let mut system = test_system_fitler_without.into_system();
        system.init(&game, &ecs).unwrap();
        ecs.system_manager_2.add_system(internal, system).unwrap();

        // Run the systems 0, 1 and 2
        let ecs_ptr = crate::UnsafeECSCell::new(&mut ecs);
        ecs.system_manager_2.run_all(&mut game, &ecs_ptr).unwrap();

        // Check new values after running systems 0, 1 and 2
        let value_entity_0 = ecs
            .component_manager
            .get(&id_1, &real_entity_0)
            .unwrap()
            .as_any()
            .downcast_ref::<NewComponent1>()
            .unwrap();
        assert_eq!(value_entity_0.value, 5u32);
        let value_entity_1 = ecs
            .component_manager
            .get(&id_1, &real_entity_1)
            .unwrap()
            .as_any()
            .downcast_ref::<NewComponent1>()
            .unwrap();
        assert_eq!(value_entity_1.value, 5u32);

        // Add system 3 to ECS
        let internal = default_system_internal!();
        let mut system = test_system.into_system();
        system.init(&game, &ecs).unwrap();
        ecs.system_manager_2.add_system(internal, system).unwrap();

        assert_eq!(game.test, 0u32);
        // Run the systems 0, 1, 2 and 3
        let ecs_ptr = crate::UnsafeECSCell::new(&mut ecs);
        ecs.system_manager_2.run_all(&mut game, &ecs_ptr).unwrap();
        assert_eq!(game.test, 1u32);
    }
}
