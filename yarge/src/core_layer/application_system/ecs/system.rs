#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::core_layer::application_system::events::user_events::UserEventWrapper;
use std::collections::VecDeque;

pub(crate) type SystemCallbackConditionFunction =
    Box<dyn Fn(&mut dyn crate::Game) -> Result<bool, ErrorType> + Send + Sync>;
pub(crate) type UserSystemCallbackConditionFunction<G> = fn(&mut G) -> bool;

pub(crate) struct UserSystemConditionBuilder;
impl UserSystemConditionBuilder {
    pub(crate) fn default_condition() -> SystemCallbackConditionFunction {
        Box::new(|_| Ok(true))
    }

    pub(crate) fn condition<G>(
        condition: UserSystemCallbackConditionFunction<G>,
    ) -> SystemCallbackConditionFunction
    where
        G: crate::Game + 'static,
    {
        Box::new(move |game| {
            let game: &mut dyn std::any::Any = game;
            let game = match game.downcast_mut::<G>() {
                Some(g) => g,
                None => {
                    log_error!("Failed to downcast game `{:?}`", std::any::type_name::<G>());
                    return Err(ErrorType::Unknown);
                }
            };
            Ok(condition(game))
        })
    }
}

/// The schedule for the system calls
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum SystemSchedule {
    /// The system will never be called
    Never,
    /// The system will be called a single time
    SingleCall,
    /// The system will be called at every update
    #[default]
    Always,
    /// The system will be called at every updates for the first X updates
    ForXUpdates(usize),
    /// The system will be called once every X updates
    EveryXUpdates(usize),
}

pub trait SystemParam {
    type State: 'static;
    type Item<'w, 's>;

    /// Called when an entity is removed from the ECS
    /// Returns true if the system needs to be destroyed
    #[allow(private_interfaces)]
    fn on_entity_removed(
        _state: &mut Self::State,
        _real_entity: &super::entity::Entity,
        _user_entity: &super::entity::UserEntity,
    ) -> Result<bool, ErrorType> {
        Ok(false)
    }
    /// Called when a component is added to an entity
    /// Returns true if the system needs to be destroyed
    #[allow(private_interfaces)]
    fn on_component_added_to_entity(
        _state: &mut Self::State,
        _component_manager: &super::component::ComponentManager,
        _real_entity: &super::entity::Entity,
        _user_entity: &super::entity::UserEntity,
        _component_id: &super::component::ComponentId,
    ) -> Result<bool, ErrorType> {
        Ok(false)
    }
    /// Called when a component is removed from an entity
    /// Returns true if the system needs to be destroyed
    #[allow(private_interfaces)]
    fn on_component_removed_from_entity(
        _state: &mut Self::State,
        _component_manager: &super::component::ComponentManager,
        _real_entity: &super::entity::Entity,
        _user_entity: &super::entity::UserEntity,
        _component_id: &super::component::ComponentId,
    ) -> Result<bool, ErrorType> {
        Ok(false)
    }
    /// Called when a component is removed from the ECS
    /// Returns true if the system needs to be destroyed
    #[allow(private_interfaces)]
    fn on_component_removed(
        _state: &mut Self::State,
        _component_id: &super::component::ComponentId,
    ) -> Result<bool, ErrorType> {
        Ok(false)
    }

    /// Initializes the state of the system
    /// Call when registering a new system in the ECS
    fn init_state(game: &dyn crate::Game, ecs: &crate::ECS) -> Result<Self::State, ErrorType>;

    /// Gets the actual Item of the system, (for example, the component from a Query)
    /// # Safety
    ///
    /// Should verify the ecs_ptr access rights
    ///
    /// ...
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

    #[allow(private_interfaces)]
    fn on_entity_removed(
        state: &mut Self::State,
        real_entity: &super::entity::Entity,
        user_entity: &super::entity::UserEntity,
    ) -> Result<bool, ErrorType> {
        let should_be_destroyed_a = match A::on_entity_removed(
            &mut state.0,
            real_entity,
            user_entity,
        ) {
            Ok(should_be_destroyed) => should_be_destroyed,
            Err(err) => {
                log_error!(
                    "Failed to handle a remove entity event in the first element of a tuple system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        let should_be_destroyed_b = match B::on_entity_removed(
            &mut state.1,
            real_entity,
            user_entity,
        ) {
            Ok(should_be_destroyed) => should_be_destroyed,
            Err(err) => {
                log_error!(
                    "Failed to handle a remove entity event in the second element of a tuple system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        Ok(should_be_destroyed_a || should_be_destroyed_b)
    }

    #[allow(private_interfaces)]
    fn on_component_added_to_entity(
        state: &mut Self::State,
        component_manager: &super::component::ComponentManager,
        real_entity: &super::entity::Entity,
        user_entity: &super::entity::UserEntity,
        component_id: &super::component::ComponentId,
    ) -> Result<bool, ErrorType> {
        let should_be_destroyed_a = match A::on_component_added_to_entity(
            &mut state.0,
            component_manager,
            real_entity,
            user_entity,
            component_id,
        ) {
            Ok(should_be_destroyed) => should_be_destroyed,
            Err(err) => {
                log_error!(
                    "Failed to handle a component added to an entity event in the first element of a tuple system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        let should_be_destroyed_b = match B::on_component_added_to_entity(
            &mut state.1,
            component_manager,
            real_entity,
            user_entity,
            component_id,
        ) {
            Ok(should_be_destroyed) => should_be_destroyed,
            Err(err) => {
                log_error!(
                    "Failed to handle a component added to an entity event in the second element of a tuple system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        Ok(should_be_destroyed_a || should_be_destroyed_b)
    }

    #[allow(private_interfaces)]
    fn on_component_removed_from_entity(
        state: &mut Self::State,
        component_manager: &super::component::ComponentManager,
        real_entity: &super::entity::Entity,
        user_entity: &super::entity::UserEntity,
        component_id: &super::component::ComponentId,
    ) -> Result<bool, ErrorType> {
        let should_be_destroyed_a = match A::on_component_removed_from_entity(
            &mut state.0,
            component_manager,
            real_entity,
            user_entity,
            component_id,
        ) {
            Ok(should_be_destroyed) => should_be_destroyed,
            Err(err) => {
                log_error!(
                    "Failed to handle a component removed from an entity event in the first element of a tuple system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        let should_be_destroyed_b = match B::on_component_removed_from_entity(
            &mut state.1,
            component_manager,
            real_entity,
            user_entity,
            component_id,
        ) {
            Ok(should_be_destroyed) => should_be_destroyed,
            Err(err) => {
                log_error!(
                    "Failed to handle a component removed from an entity event in the second element of a tuple system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        Ok(should_be_destroyed_a || should_be_destroyed_b)
    }

    #[allow(private_interfaces)]
    fn on_component_removed(
        state: &mut Self::State,
        component_id: &super::component::ComponentId,
    ) -> Result<bool, ErrorType> {
        let should_be_destroyed_a = match A::on_component_removed(&mut state.0, component_id) {
            Ok(should_be_destroyed) => should_be_destroyed,
            Err(err) => {
                log_error!(
                    "Failed to handle a component removed event in the first element of a tuple system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        let should_be_destroyed_b = match B::on_component_removed(&mut state.1, component_id) {
            Ok(should_be_destroyed) => should_be_destroyed,
            Err(err) => {
                log_error!(
                    "Failed to handle a component removed event in the second element of a tuple system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        Ok(should_be_destroyed_a || should_be_destroyed_b)
    }

    fn init_state(game: &dyn crate::Game, ecs: &crate::ECS) -> Result<Self::State, ErrorType> {
        let state_a = match A::init_state(game, ecs) {
            Ok(state) => state,
            Err(err) => {
                log_error!(
                    "Failed to init the state of the first element of a tuple system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        let state_b = match B::init_state(game, ecs) {
            Ok(state) => state,
            Err(err) => {
                log_error!(
                    "Failed to init the state of the second element of a tuple system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        Ok((state_a, state_b))
    }

    unsafe fn get_item<'w, 's>(
        state: &'s mut Self::State,
        game_ptr: &'w crate::UnsafeGameCell,
        ecs_ptr: &'w crate::UnsafeECSCell,
    ) -> Result<Self::Item<'w, 's>, ErrorType> {
        let item_a = match unsafe { A::get_item(&mut state.0, game_ptr, ecs_ptr) } {
            Ok(item) => item,
            Err(err) => {
                log_error!(
                    "Failed to get the item of the first element of a tuple system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        let item_b = match unsafe { B::get_item(&mut state.1, game_ptr, ecs_ptr) } {
            Ok(item) => item,
            Err(err) => {
                log_error!(
                    "Failed to get the item of the second element of a tuple system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        Ok((item_a, item_b))
    }
}

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

/// A wrapper around the real system closure
pub struct SystemFuncWrapper<Func, Param>
where
    Func: for<'w, 's> FnMut(Param::Item<'w, 's>) -> Result<VecDeque<UserEventWrapper>, ErrorType>
        + 'static,
    Param: SystemParam,
{
    /// The system function
    pub function: Func,
    /// The system state
    pub state: Option<Param::State>,
    /// A marker for the type
    pub _marker: std::marker::PhantomData<Param>,
}

impl<Func, Param> SystemTrait for SystemFuncWrapper<Func, Param>
where
    Func: for<'w, 's> FnMut(Param::Item<'w, 's>) -> Result<VecDeque<UserEventWrapper>, ErrorType>
        + 'static,
    Param: SystemParam,
{
    fn init(&mut self, game: &dyn crate::Game, ecs: &crate::ECS) -> Result<(), ErrorType> {
        self.state = match Param::init_state(game, ecs) {
            Ok(state) => Some(state),
            Err(err) => {
                log_error!(
                    "Failed to initialize the stait of a system function wrapper: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        Ok(())
    }

    fn run(
        &mut self,
        game_ptr: &crate::UnsafeGameCell,
        ecs_ptr: &crate::UnsafeECSCell,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        let state = match &mut self.state {
            None => {
                log_error!(
                    "The state of a system function wrapper is not initialized when running: maybe you forgot to call .init()"
                );
                return Err(ErrorType::DoesNotExist);
            }
            Some(state) => state,
        };
        let param = match unsafe { Param::get_item(state, game_ptr, ecs_ptr) } {
            Ok(param) => param,
            Err(err) => {
                log_error!(
                    "Failed to get the item of a system function wrapper: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        (self.function)(param)
    }

    #[allow(private_interfaces)]
    fn on_entity_removed(
        &mut self,
        real_entity: &super::entity::Entity,
        user_entity: &super::entity::UserEntity,
    ) -> Result<bool, ErrorType> {
        let state = match &mut self.state {
            None => {
                log_error!(
                    "The state of a system function wrapper is not initialized on entity removed: maybe you forgot to call .init()"
                );
                return Err(ErrorType::DoesNotExist);
            }
            Some(state) => state,
        };
        match Param::on_entity_removed(state, real_entity, user_entity) {
            Ok(should_be_destroyed) => Ok(should_be_destroyed),
            Err(err) => {
                log_error!(
                    "Failed to handle an on entity removed event in a system function wrapper: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
    }

    #[allow(private_interfaces)]
    fn on_component_added_to_entity(
        &mut self,
        component_manager: &super::component::ComponentManager,
        real_entity: &super::entity::Entity,
        user_entity: &super::entity::UserEntity,
        component_id: &super::component::ComponentId,
    ) -> Result<bool, ErrorType> {
        let state = match &mut self.state {
            None => {
                log_error!(
                    "The state of a system function wrapper is not initialized on component added to entity: maybe you forgot to call .init()"
                );
                return Err(ErrorType::DoesNotExist);
            }
            Some(state) => state,
        };
        match Param::on_component_added_to_entity(
            state,
            component_manager,
            real_entity,
            user_entity,
            component_id,
        ) {
            Ok(should_be_destroyed) => Ok(should_be_destroyed),
            Err(err) => {
                log_error!(
                    "Failed to handle an on component added to entity event in a system function wrapper: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
    }

    #[allow(private_interfaces)]
    fn on_component_removed_from_entity(
        &mut self,
        component_manager: &super::component::ComponentManager,
        real_entity: &super::entity::Entity,
        user_entity: &super::entity::UserEntity,
        component_id: &super::component::ComponentId,
    ) -> Result<bool, ErrorType> {
        let state = match &mut self.state {
            None => {
                log_error!(
                    "The state of a system function wrapper is not initialized on component removed from entity: maybe you forgot to call .init()"
                );
                return Err(ErrorType::DoesNotExist);
            }
            Some(state) => state,
        };
        match Param::on_component_removed_from_entity(
            state,
            component_manager,
            real_entity,
            user_entity,
            component_id,
        ) {
            Ok(should_be_destroyed) => Ok(should_be_destroyed),
            Err(err) => {
                log_error!(
                    "Failed to handle an on component removed from entity event in a system function wrapper: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
    }

    #[allow(private_interfaces)]
    fn on_component_removed(
        &mut self,
        component_id: &super::component::ComponentId,
    ) -> Result<bool, ErrorType> {
        let state = match &mut self.state {
            None => {
                log_error!(
                    "The state of a system function wrapper is not initialized on component removed: maybe you forgot to call .init()"
                );
                return Err(ErrorType::DoesNotExist);
            }
            Some(state) => state,
        };
        match Param::on_component_removed(state, component_id) {
            Ok(should_be_destroyed) => Ok(should_be_destroyed),
            Err(err) => {
                log_error!(
                    "Failed to handle an on component removed event in a system function wrapper: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
    }
}

pub trait SystemTrait {
    /// Initializes the system
    fn init(&mut self, game: &dyn crate::Game, ecs: &crate::ECS) -> Result<(), ErrorType>;
    /// Runs the system
    fn run(
        &mut self,
        game_ptr: &crate::UnsafeGameCell,
        ecs_ptr: &crate::UnsafeECSCell,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType>;

    /// Called when an entity is removed from the ECS
    /// Returns true if the system needs to be destroyed
    #[allow(private_interfaces)]
    fn on_entity_removed(
        &mut self,
        real_entity: &super::entity::Entity,
        user_entity: &super::entity::UserEntity,
    ) -> Result<bool, ErrorType>;

    /// Called when a component is added to an entity
    /// Returns true if the system needs to be destroyed
    #[allow(private_interfaces)]
    fn on_component_added_to_entity(
        &mut self,
        component_manager: &super::component::ComponentManager,
        real_entity: &super::entity::Entity,
        user_entity: &super::entity::UserEntity,
        component_id: &super::component::ComponentId,
    ) -> Result<bool, ErrorType>;

    /// Called when a component is removed from an entity
    /// Returns true if the system needs to be destroyed
    #[allow(private_interfaces)]
    fn on_component_removed_from_entity(
        &mut self,
        component_manager: &super::component::ComponentManager,
        real_entity: &super::entity::Entity,
        user_entity: &super::entity::UserEntity,
        component_id: &super::component::ComponentId,
    ) -> Result<bool, ErrorType>;

    /// Called when a component is removed from the ECS
    /// Returns true if the system needs to be destroyed
    #[allow(private_interfaces)]
    fn on_component_removed(
        &mut self,
        component_id: &super::component::ComponentId,
    ) -> Result<bool, ErrorType>;
}

pub trait IntoSystem {
    fn as_system(&self) -> Box<dyn SystemTrait>;
}

pub(crate) struct SystemInternal {
    pub(crate) schedule: SystemSchedule,
    updates_counter: usize,
    pub(crate) condition: SystemCallbackConditionFunction,
}

impl SystemInternal {
    pub(crate) fn new(
        schedule: SystemSchedule,
        condition: SystemCallbackConditionFunction,
    ) -> Self {
        Self {
            schedule,
            updates_counter: 0,
            condition,
        }
    }

    /// Checks if the system should run this update taking into account it's schedule and condition
    pub(crate) fn should_run_this_update(&mut self) -> bool {
        match self.schedule {
            SystemSchedule::Never => false,
            SystemSchedule::SingleCall => {
                self.schedule = SystemSchedule::Never;
                true
            }
            SystemSchedule::Always => true,
            SystemSchedule::ForXUpdates(nb_frames_remaining) => {
                if nb_frames_remaining == 1 {
                    self.schedule = SystemSchedule::Never;
                } else {
                    self.schedule = SystemSchedule::ForXUpdates(nb_frames_remaining - 1);
                }
                true
            }
            SystemSchedule::EveryXUpdates(nb_frames_to_wait) => {
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

/// A wrapper around a system
pub(crate) struct SystemWrapper {
    /// The actual system with its inner state
    system_trait: Box<dyn SystemTrait>,
    /// The parameters of the system
    pub(crate) internal: SystemInternal,
}

impl SystemWrapper {
    /// Checks if the system should run this update taking into account it's internals
    pub(crate) fn should_run_this_update(&mut self) -> bool {
        self.internal.should_run_this_update()
    }
}

/// The System Manager
pub(crate) struct SystemManager {
    /// A list of systems
    systems: Vec<SystemWrapper>,
}

impl SystemManager {
    /// Initializes the system manager with no systems
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
        internal: SystemInternal,
        system: Box<dyn SystemTrait>,
    ) -> Result<(), ErrorType> {
        let wrapper = SystemWrapper {
            system_trait: system,
            internal,
        };

        self.systems.push(wrapper);
        Ok(())
    }

    /// Remove systems that will never run
    /// A system schedule can't be updated
    pub(crate) fn clean_dead_systems(&mut self) {
        let indices_to_remove: Vec<usize> = self
            .systems
            .iter()
            .enumerate()
            .filter(|&(_, val)| val.internal.schedule == SystemSchedule::Never)
            .map(|(index, _)| index)
            .collect();
        for index in indices_to_remove.into_iter().rev() {
            let _ = self.systems.drain(index..index + 1);
        }
    }

    /// Run all the systems and clean the dead ones
    pub(crate) fn run_all(
        &mut self,
        game: &mut dyn crate::Game,
        ecs_ptr: &crate::UnsafeECSCell,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        let mut user_events = VecDeque::new();
        for system in &mut self.systems {
            match (system.internal.condition)(game) {
                Ok(should_run) => {
                    if should_run && system.should_run_this_update() {
                        let game_ptr = crate::UnsafeGameCell::new(game);
                        match system.system_trait.run(&game_ptr, ecs_ptr) {
                            Ok(mut events) => user_events.append(&mut events),
                            Err(err) => {
                                log_error!("Failed to run a system: {:?}", err);
                                return Err(ErrorType::Unknown);
                            }
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

        Ok(user_events)
    }

    /// Called when an entity is removed from the ECS
    pub(crate) fn on_entity_removed(
        &mut self,
        real_entity: &super::entity::Entity,
        user_entity: &super::entity::UserEntity,
    ) -> Result<(), ErrorType> {
        let mut indices_to_remove: Vec<usize> = Vec::with_capacity(self.systems.len());

        for (index, system) in self.systems.iter_mut().enumerate() {
            match system
                .system_trait
                .on_entity_removed(real_entity, user_entity)
            {
                Ok(true) => indices_to_remove.push(index),
                Err(err) => {
                    log_error!(
                        "Failed to handle an on entity removed event in the system manager: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
                _ => {}
            }
        }

        for index in indices_to_remove.into_iter().rev() {
            let _ = self.systems.drain(index..index + 1);
        }
        Ok(())
    }

    /// Called when a component is added to an entity
    pub(crate) fn on_component_added_to_entity(
        &mut self,
        component_manager: &super::component::ComponentManager,
        real_entity: &super::entity::Entity,
        user_entity: &super::entity::UserEntity,
        component_id: &super::component::ComponentId,
    ) -> Result<(), ErrorType> {
        let mut indices_to_remove: Vec<usize> = Vec::with_capacity(self.systems.len());

        for (index, system) in self.systems.iter_mut().enumerate() {
            match system.system_trait.on_component_added_to_entity(
                component_manager,
                real_entity,
                user_entity,
                component_id,
            ) {
                Ok(true) => indices_to_remove.push(index),
                Err(err) => {
                    log_error!(
                        "Failed to handle an on component added to entity event in the system manager: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
                _ => {}
            }
        }

        for index in indices_to_remove.into_iter().rev() {
            let _ = self.systems.drain(index..index + 1);
        }
        Ok(())
    }

    /// Called when a component is removed from an entity
    pub(crate) fn on_component_removed_from_entity(
        &mut self,
        component_manager: &super::component::ComponentManager,
        real_entity: &super::entity::Entity,
        user_entity: &super::entity::UserEntity,
        component_id: &super::component::ComponentId,
    ) -> Result<(), ErrorType> {
        let mut indices_to_remove: Vec<usize> = Vec::with_capacity(self.systems.len());

        for (index, system) in self.systems.iter_mut().enumerate() {
            match system.system_trait.on_component_removed_from_entity(
                component_manager,
                real_entity,
                user_entity,
                component_id,
            ) {
                Ok(true) => indices_to_remove.push(index),
                Err(err) => {
                    log_error!(
                        "Failed to handle an on component removed from entity event in the system manager: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
                _ => {}
            }
        }

        for index in indices_to_remove.into_iter().rev() {
            let _ = self.systems.drain(index..index + 1);
        }
        Ok(())
    }

    /// Called when a component is removed from the ECS
    pub(crate) fn on_component_removed(
        &mut self,
        component_id: &super::component::ComponentId,
    ) -> Result<(), ErrorType> {
        let mut indices_to_remove: Vec<usize> = Vec::with_capacity(self.systems.len());

        for (index, system) in self.systems.iter_mut().enumerate() {
            match system.system_trait.on_component_removed(component_id) {
                Ok(true) => indices_to_remove.push(index),
                Err(err) => {
                    log_error!(
                        "Failed to handle an on component removed event in the system manager: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
                _ => {}
            }
        }

        for index in indices_to_remove.into_iter().rev() {
            let _ = self.systems.drain(index..index + 1);
        }
        Ok(())
    }
}

impl crate::core_layer::application_system::application::ApplicationSystem<'_> {
    pub(crate) fn run_systems(&mut self) -> Result<(), ErrorType> {
        let ecs_ptr = crate::UnsafeECSCell::new(&mut self.ecs);
        if let Err(err) = self.ecs.system_manager.run_all(self.user_game, &ecs_ptr) {
            log_error!("Failed to run the systems in the application: {:?}", err);
            return Err(ErrorType::Unknown);
        }
        Ok(())
    }
}

//////////////////////////////////////////////////////////
///////////////     systems tests      ////////////////////
//////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::super::query::{Query, With, Without};
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
            SystemInternal::new(
                SystemSchedule::default(),
                UserSystemConditionBuilder::default_condition(),
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
        fn test_system_0() -> Result<VecDeque<UserEventWrapper>, ErrorType> {
            log_info!("Inside test system with 0 params");
            Ok(VecDeque::new())
        }
        #[macros::system]
        fn test_system_1(
            _query: Query<'_, '_, &NewComponent1, With<NewComponent2>>,
        ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
            log_info!("Inside test system with 1 params");
            Ok(VecDeque::new())
        }
        #[macros::system]
        fn test_system_2(
            _q1: Query<'_, '_, &NewComponent1, Without<NewComponent2>>,
            _q2: Query<'_, '_, &NewComponent1, Without<NewComponent2>>,
        ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
            log_info!("Inside test system with 2 params");
            Ok(VecDeque::new())
        }

        let internal_0 = default_system_internal!();
        let mut system_0 = test_system_0.as_system();
        system_0.init(&game, &ecs).unwrap();
        ecs.system_manager.add_system(internal_0, system_0).unwrap();

        let internal_1 = default_system_internal!();
        let mut system_1 = test_system_1.as_system();
        system_1.init(&game, &ecs).unwrap();
        ecs.system_manager.add_system(internal_1, system_1).unwrap();

        let internal_2 = default_system_internal!();
        let mut system_2 = test_system_2.as_system();
        system_2.init(&game, &ecs).unwrap();
        ecs.system_manager.add_system(internal_2, system_2).unwrap();
    }

    #[test]
    fn systems_with_game() {
        // Init Game
        let game = TestGame { test: 0u32 };
        // Init ecs
        let mut ecs = crate::ECS::init().unwrap();

        #[macros::system]
        fn test_system(_game: &mut TestGame) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
            Ok(VecDeque::new())
        }
        #[macros::system]
        fn test_system_query(
            _game: &TestGame,
            _query: Query<'_, '_, &NewComponent1>,
        ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
            Ok(VecDeque::new())
        }

        let internal = default_system_internal!();
        let mut system = test_system.as_system();
        system.init(&game, &ecs).unwrap();
        ecs.system_manager.add_system(internal, system).unwrap();

        let internal = default_system_internal!();
        let mut system = test_system_query.as_system();
        system.init(&game, &ecs).unwrap();
        ecs.system_manager.add_system(internal, system).unwrap();
    }

    #[test]
    fn systems_as_function() {
        // Init ecs
        let mut ecs = crate::ECS::init().unwrap();

        #[macros::system]
        fn test_system() -> Result<VecDeque<UserEventWrapper>, ErrorType> {
            Ok(VecDeque::new())
        }
        #[macros::system]
        fn test_system_err(
            _query: Query<'_, '_, &NewComponent1>,
        ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
            Err(ErrorType::Unknown)
        }
        #[macros::system]
        fn test_system_ok(
            _q1: Query<'_, '_, &NewComponent1>,
            _q2: Query<'_, '_, &NewComponent2>,
        ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
            Ok(VecDeque::new())
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
        fn test_system(game: &mut TestGame) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
            game.test += 1u32;
            Ok(VecDeque::new())
        }

        let internal = default_system_internal!();
        let mut system = test_system.as_system();
        system.init(&game, &ecs).unwrap();
        ecs.system_manager.add_system(internal, system).unwrap();
        assert_eq!(game.test, 0u32);

        let ecs_ptr = crate::UnsafeECSCell::new(&mut ecs);
        let _ = ecs.system_manager.run_all(&mut game, &ecs_ptr).unwrap();
        assert_eq!(game.test, 1u32);

        let ecs_ptr = crate::UnsafeECSCell::new(&mut ecs);
        let _ = ecs.system_manager.run_all(&mut game, &ecs_ptr).unwrap();
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
        fn test_system_simple(
            query: Query<'_, '_, &mut NewComponent1>,
        ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
            for component_1 in &query {
                component_1.value += 1u32;
            }
            Ok(VecDeque::new())
        }

        #[macros::system]
        fn test_system_fitler_with(
            query: Query<'_, '_, &mut NewComponent1, With<NewComponent2>>,
        ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
            for component_1 in &query {
                component_1.value += 1u32;
            }
            Ok(VecDeque::new())
        }

        #[macros::system]
        fn test_system_fitler_without(
            query: Query<'_, '_, &mut NewComponent1, Without<NewComponent2>>,
        ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
            for component_1 in &query {
                component_1.value += 1u32;
            }
            Ok(VecDeque::new())
        }

        #[macros::system]
        fn test_system(
            game: &mut TestGame,
            query: Query<'_, '_, (&NewComponent1, &NewComponent2)>,
        ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
            for ((_component_1, _component_2), entity) in query.with_entities() {
                game.test += entity.0.index as u32;
            }
            Ok(VecDeque::new())
        }

        // Add system 0 to ECS
        let internal = default_system_internal!();
        let mut system = test_system_simple.as_system();
        system.init(&game, &ecs).unwrap();
        ecs.system_manager.add_system(internal, system).unwrap();

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
        let _ = ecs.system_manager.run_all(&mut game, &ecs_ptr).unwrap();

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
        let mut system = test_system_fitler_with.as_system();
        system.init(&game, &ecs).unwrap();
        ecs.system_manager.add_system(internal, system).unwrap();

        // Run the systems 0 and 1
        let ecs_ptr = crate::UnsafeECSCell::new(&mut ecs);
        let _ = ecs.system_manager.run_all(&mut game, &ecs_ptr).unwrap();

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
        let mut system = test_system_fitler_without.as_system();
        system.init(&game, &ecs).unwrap();
        ecs.system_manager.add_system(internal, system).unwrap();

        // Run the systems 0, 1 and 2
        let ecs_ptr = crate::UnsafeECSCell::new(&mut ecs);
        let _ = ecs.system_manager.run_all(&mut game, &ecs_ptr).unwrap();

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
        let mut system = test_system.as_system();
        system.init(&game, &ecs).unwrap();
        ecs.system_manager.add_system(internal, system).unwrap();

        assert_eq!(game.test, 0u32);
        // Run the systems 0, 1, 2 and 3
        let ecs_ptr = crate::UnsafeECSCell::new(&mut ecs);
        let _ = ecs.system_manager.run_all(&mut game, &ecs_ptr).unwrap();
        assert_eq!(game.test, 1u32);
    }
}
