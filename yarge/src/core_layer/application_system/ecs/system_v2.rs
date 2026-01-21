#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

pub trait SystemParam {
    type State: 'static;
    type Item<'w, 's>;

    // TODO: add fucntions to update state
    // remove entity, add component to entity, remove component from entity, remove component

    fn init_state(ecs: &crate::ECS) -> Result<Self::State, ErrorType>;
    unsafe fn get_item<'w, 's>(
        state: &'s mut Self::State,
        ecs: &'w super::query::UnsafeECSCell,
    ) -> Result<Self::Item<'w, 's>, ErrorType>;
}

impl SystemParam for () {
    type State = ();
    type Item<'w, 's> = ();

    fn init_state(_: &crate::ECS) -> Result<Self::State, ErrorType> {
        Ok(())
    }

    unsafe fn get_item<'w, 's>(
        _: &'s mut Self::State,
        _: &'w super::query::UnsafeECSCell,
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

    fn init_state(ecs: &crate::ECS) -> Result<Self::State, ErrorType> {
        Ok((A::init_state(ecs)?, B::init_state(ecs)?))
    }

    // TODO: add fucntions to update state
    // remove entity, add component to entity, remove component from entity, remove component

    unsafe fn get_item<'w, 's>(
        state: &'s mut Self::State,
        ecs: &'w super::query::UnsafeECSCell,
    ) -> Result<Self::Item<'w, 's>, ErrorType> {
        let item_a = unsafe { A::get_item(&mut state.0, ecs) }?;
        let item_b = unsafe { B::get_item(&mut state.1, ecs) }?;
        Ok((item_a, item_b))
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
    fn init(&mut self, ecs: &crate::ECS) -> Result<(), ErrorType> {
        self.state = Some(Param::init_state(ecs)?);
        Ok(())
    }
    fn run(&mut self, ecs_ptr: &super::query::UnsafeECSCell) -> Result<(), ErrorType> {
        let state = match &mut self.state {
            None => return Err(ErrorType::Unknown),
            Some(state) => state,
        };
        let param = unsafe { Param::get_item(state, ecs_ptr) }?;
        (self.function)(param)
    }
}

pub trait IntoSystem {
    fn into_system(self) -> Box<dyn SystemTrait>;
}

pub trait SystemTrait {
    // TODO: add fucntions to update state
    // remove entity, add component to entity, remove component from entity, remove component

    fn init(&mut self, ecs: &crate::ECS) -> Result<(), ErrorType>;
    fn run(&mut self, ecs_ptr: &super::query::UnsafeECSCell) -> Result<(), ErrorType>;
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
        ecs_ptr: &super::query::UnsafeECSCell,
    ) -> Result<(), ErrorType> {
        for system in &mut self.systems {
            match (system.internal.condition)(game) {
                Ok(should_run) => {
                    if should_run && system.should_run_this_update() 
                        && let Err(err) = system.system_trait.run(ecs_ptr) {
                            log_error!("Failed to run a system: {:?}", err);
                            return Err(ErrorType::Unknown);
                        
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
        Component, RegisterComponentFunction,
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
        ecs.register_component(&id_1, &register_1).unwrap();
        ecs.register_component(&id_2, &register_2).unwrap();

        // let entities= std::collections::HashSet::new();

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

        let internal_0 = SystemInternalV2::new(
            SystemSchedule::default(),
            UserSystemCallbackBuilder::default_condition(),
        );
        let mut system_0 = test_system_0.into_system();
        system_0.init(&ecs).unwrap();
        ecs.system_manager_2
            .add_system(internal_0, system_0)
            .unwrap();

        let internal_1 = SystemInternalV2::new(
            SystemSchedule::default(),
            UserSystemCallbackBuilder::default_condition(),
        );
        let mut system_1 = test_system_1.into_system();
        system_1.init(&ecs).unwrap();
        ecs.system_manager_2
            .add_system(internal_1, system_1)
            .unwrap();

        let internal_2 = SystemInternalV2::new(
            SystemSchedule::default(),
            UserSystemCallbackBuilder::default_condition(),
        );
        let mut system_2 = test_system_2.into_system();
        system_2.init(&ecs).unwrap();
        ecs.system_manager_2
            .add_system(internal_2, system_2)
            .unwrap();
    }
}
