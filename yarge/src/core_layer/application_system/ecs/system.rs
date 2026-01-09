use std::collections::HashSet;

use crate::error::ErrorType;

#[allow(unused)]
use crate::{log_debug, log_error, log_info, log_warn};

pub type SystemCallback = Box<
    dyn Fn(&mut dyn crate::Game, &dyn super::component::RealComponent) -> Result<(), ErrorType>
        + Send
        + Sync,
>;
pub type SystemMutCallback = Box<
    dyn Fn(&mut dyn crate::Game, &mut dyn super::component::RealComponent) -> Result<(), ErrorType>
        + Send
        + Sync,
>;
pub type SystemCallbackConditionFunction =
    Box<dyn Fn(&mut dyn crate::Game) -> Result<bool, ErrorType> + Send + Sync>;

pub type UserSystemCallback<G, T> = fn(&mut G, &T) -> Result<(), ErrorType>;
pub type UserSystemMutCallback<G, T> = fn(&mut G, &mut T) -> Result<(), ErrorType>;
pub type UserSystemCallbackConditionFunction<G> = fn(&mut G) -> bool;

pub struct UserSystemCallbackBuilder;
impl UserSystemCallbackBuilder {
    pub fn default_condition() -> SystemCallbackConditionFunction {
        Box::new(|_| Ok(true))
    }

    pub fn condition<G>(
        condition: UserSystemCallbackConditionFunction<G>,
    ) -> SystemCallbackConditionFunction
    where
        G: crate::Game + 'static,
    {
        Box::new(move |game| {
            let game = match (game as &mut dyn std::any::Any).downcast_mut::<G>() {
                Some(g) => g,
                None => {
                    log_error!("Failed to downcast game `{:?}`", std::any::type_name::<G>());
                    return Err(ErrorType::Unknown);
                }
            };
            Ok(condition(game))
        })
    }

    pub fn system<G, T>(callback: UserSystemCallback<G, T>) -> SystemCallback
    where
        G: crate::Game + 'static,
        T: crate::Component + 'static,
    {
        Box::new(move |game, value| {
            let value = match value.as_any().downcast_ref::<T>() {
                Some(v) => v,
                None => {
                    log_error!(
                        "Failed to downcast component `{:?}`",
                        std::any::type_name::<T>(),
                    );
                    return Err(ErrorType::Unknown);
                }
            };
            let game = match (game as &mut dyn std::any::Any).downcast_mut::<G>() {
                Some(g) => g,
                None => {
                    log_error!("Failed to downcast game `{:?}`", std::any::type_name::<G>());
                    return Err(ErrorType::Unknown);
                }
            };
            callback(game, value)
        })
    }

    pub fn system_mut<G, T>(callback_mut: UserSystemMutCallback<G, T>) -> SystemMutCallback
    where
        G: crate::Game + 'static,
        T: crate::Component + 'static,
    {
        Box::new(move |game, value| {
            let value = match value.as_any_mut().downcast_mut::<T>() {
                Some(v) => v,
                None => {
                    log_error!(
                        "Failed to downcast component `{:?}`",
                        std::any::type_name::<T>(),
                    );
                    return Err(ErrorType::Unknown);
                }
            };
            let game = match (game as &mut dyn std::any::Any).downcast_mut::<G>() {
                Some(g) => g,
                None => {
                    log_error!("Failed to downcast game `{:?}`", std::any::type_name::<G>());
                    return Err(ErrorType::Unknown);
                }
            };
            callback_mut(game, value)
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

pub(crate) struct SystemInternal {
    pub entities: HashSet<crate::Entity>,
    pub name: std::any::TypeId,
    pub with: Vec<std::any::TypeId>,
    pub without: Vec<std::any::TypeId>,
    pub schedule: SystemSchedule,
    updates_counter: usize,
    pub condition: SystemCallbackConditionFunction,
}

impl SystemInternal {
    pub fn new(
        name: std::any::TypeId,
        with: &[std::any::TypeId],
        without: &[std::any::TypeId],
        schedule: SystemSchedule,
        condition: SystemCallbackConditionFunction,
    ) -> Self {
        Self {
            entities: HashSet::new(),
            name,
            with: with.to_vec(),
            without: without.to_vec(),
            schedule,
            updates_counter: 0,
            condition,
        }
    }

    pub fn should_run_this_update(&mut self) -> bool {
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

    pub fn add_entity(
        &mut self,
        component_manager: &super::component::ComponentManager,
        entity: &crate::Entity,
    ) -> Result<(), ErrorType> {
        match component_manager.has_component_type(entity, &self.name) {
            Ok(false) => {}
            Ok(true) => {
                match component_manager.has_correct_constraints(entity, &self.with, &self.without) {
                    Ok(false) => {}
                    Ok(true) => {
                        if !self.entities.insert(*entity) {
                            log_error!(
                                "Failed to insert a new entity in a system, the entity was already present"
                            );
                            return Err(ErrorType::Duplicate);
                        }
                    }
                    Err(err) => {
                        log_error!(
                            "Failed to check if an entry has the correct constraints when adding an entity to a system: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                }
            }
            Err(err) => {
                log_error!(
                    "Failed to check if an entry has the given component type when adding an entity to a system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        }
        Ok(())
    }

    pub fn add_entities(
        &mut self,
        component_manager: &super::component::ComponentManager,
        entities: &[crate::Entity],
    ) -> Result<(), ErrorType> {
        for entity in entities {
            if let Err(err) = self.add_entity(component_manager, entity) {
                log_error!(
                    "Failed to add an entity to a system when trying to add multiple entities in a system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        }
        Ok(())
    }

    #[allow(unused)]
    pub fn remove_entity(&mut self, entity: &crate::Entity) -> Result<(), ErrorType> {
        if !self.entities.remove(entity) {
            log_error!("Trying to remove an entity not present in a system");
            return Err(ErrorType::DoesNotExist);
        }
        Ok(())
    }

    #[allow(unused)]
    pub fn remove_entity_unchecked(&mut self, entity: &crate::Entity) {
        self.entities.remove(entity);
    }

    #[allow(unused)]
    pub fn remove_entities(&mut self, entities: &[crate::Entity]) -> Result<(), ErrorType> {
        for entity in entities {
            if let Err(err) = self.remove_entity(entity) {
                log_error!(
                    "Failed to remove an entity from a system when trying to remove multiple entities in a system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        }
        Ok(())
    }

    #[allow(unused)]
    pub fn remove_entities_unchecked(&mut self, entities: &[crate::Entity]) {
        for entity in entities {
            self.remove_entity_unchecked(entity);
        }
    }

    pub fn on_component_changed_for_entity(
        &mut self,
        component_manager: &super::component::ComponentManager,
        entity: &crate::Entity,
    ) -> Result<(), ErrorType> {
        let does_fulfill_requirements = match component_manager
            .has_component_type(entity, &self.name)
        {
            Ok(false) => false,
            Ok(true) => {
                match component_manager.has_correct_constraints(entity, &self.with, &self.without) {
                    Ok(res) => res,
                    Err(err) => {
                        log_error!(
                            "Failed to check if an entry has the correct constraints when checking a component change in a system: {:?}",
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                }
            }
            Err(err) => {
                log_error!(
                    "Failed to check if an entry has the given component type when checking a component change in a system: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        match self.entities.contains(entity) {
            true => {
                // Check if need to be removed
                if !does_fulfill_requirements {
                    self.remove_entity_unchecked(entity);
                }
            }
            false => {
                // Check if need to be added
                if does_fulfill_requirements {
                    self.entities.insert(*entity);
                }
            }
        }
        Ok(())
    }

    pub fn does_need_component(&self, component_querried: &std::any::TypeId) -> bool {
        self.name == *component_querried
            || self.with.contains(component_querried)
            || self.without.contains(component_querried)
    }
}

pub(crate) struct SystemRef {
    pub callback: SystemCallback,
    pub internal: SystemInternal,
}

pub(crate) struct SystemMut {
    pub internal: SystemInternal,
    pub callback: SystemMutCallback,
}

impl SystemRef {
    pub fn new(internal: SystemInternal, callback: SystemCallback) -> Self {
        Self { internal, callback }
    }
}

impl SystemMut {
    pub fn new(internal: SystemInternal, callback: SystemMutCallback) -> Self {
        Self { internal, callback }
    }
}

pub(crate) struct SystemManager {
    pub systems_ref: Vec<SystemRef>,
    pub systems_mut: Vec<SystemMut>,
}

impl SystemManager {
    pub fn init() -> Self {
        Self {
            systems_ref: vec![],
            systems_mut: vec![],
        }
    }

    pub fn register_new_system_ref(
        &mut self,
        internal: SystemInternal,
        callback: SystemCallback,
        component_manager: &super::component::ComponentManager,
        existing_entities: &[crate::Entity],
    ) -> Result<(), ErrorType> {
        let mut new_system = SystemRef::new(internal, callback);
        if let Err(err) = new_system
            .internal
            .add_entities(component_manager, existing_entities)
        {
            log_error!(
                "Failed to add the entities when registering a new system: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
        self.systems_ref.push(new_system);
        Ok(())
    }

    pub fn register_new_system_mut(
        &mut self,
        internal: SystemInternal,
        callback: SystemMutCallback,
        component_manager: &super::component::ComponentManager,
        existing_entities: &[crate::Entity],
    ) -> Result<(), ErrorType> {
        let mut new_system = SystemMut::new(internal, callback);
        if let Err(err) = new_system
            .internal
            .add_entities(component_manager, existing_entities)
        {
            log_error!(
                "Failed to add the entities when registering a new mut system: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
        self.systems_mut.push(new_system);
        Ok(())
    }

    #[allow(unused)]
    pub fn remove_entity_unchecked(&mut self, entity: &crate::Entity) {
        for system in &mut self.systems_ref {
            system.internal.remove_entity_unchecked(entity);
        }
        for system in &mut self.systems_mut {
            system.internal.remove_entity_unchecked(entity);
        }
    }

    #[allow(unused)]
    pub fn remove_entities_unchecked(&mut self, entities: &[crate::Entity]) {
        for system in &mut self.systems_ref {
            system.internal.remove_entities_unchecked(entities);
        }
        for system in &mut self.systems_mut {
            system.internal.remove_entities_unchecked(entities);
        }
    }

    #[allow(unused)]
    pub fn remove_entity(&mut self, entity: &crate::Entity) -> Result<(), ErrorType> {
        for system in &mut self.systems_ref {
            system.internal.remove_entity(entity)?;
        }
        for system in &mut self.systems_mut {
            system.internal.remove_entity(entity)?;
        }
        Ok(())
    }

    #[allow(unused)]
    pub fn remove_entities(&mut self, entities: &[crate::Entity]) -> Result<(), ErrorType> {
        for system in &mut self.systems_ref {
            system.internal.remove_entities(entities)?;
        }
        for system in &mut self.systems_mut {
            system.internal.remove_entities(entities)?;
        }
        Ok(())
    }

    pub fn on_removed_entity(&mut self, entity: &crate::Entity) {
        self.remove_entity_unchecked(entity);
    }
    pub fn on_removed_entities(&mut self, entities: &[crate::Entity]) {
        self.remove_entities_unchecked(entities);
    }

    pub fn on_component_changed_for_entity(
        &mut self,
        component_manager: &super::component::ComponentManager,
        entity: &crate::Entity,
    ) -> Result<(), ErrorType> {
        for system in &mut self.systems_ref {
            if let Err(err) = system
                .internal
                .on_component_changed_for_entity(component_manager, entity)
            {
                log_error!(
                    "Failed to handle a component change for an entity in the system manager: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        }
        for system in &mut self.systems_mut {
            if let Err(err) = system
                .internal
                .on_component_changed_for_entity(component_manager, entity)
            {
                log_error!(
                    "Failed to handle a component change for an entity in the system manager: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        }
        Ok(())
    }

    pub fn on_component_removed(&mut self, removed_component: &std::any::TypeId) {
        let indices_to_remove: Vec<usize> = self
            .systems_ref
            .iter()
            .enumerate()
            .filter(|&(_, val)| val.internal.does_need_component(removed_component))
            .map(|(index, _)| index)
            .collect();
        for index in indices_to_remove.into_iter().rev() {
            self.systems_ref.drain(index..index + 1);
        }

        let indices_to_remove: Vec<usize> = self
            .systems_mut
            .iter()
            .enumerate()
            .filter(|&(_, val)| val.internal.does_need_component(removed_component))
            .map(|(index, _)| index)
            .collect();
        for index in indices_to_remove.into_iter().rev() {
            self.systems_mut.drain(index..index + 1);
        }
    }

    pub fn clean_dead_systems(&mut self) {
        let indices_to_remove: Vec<usize> = self
            .systems_ref
            .iter()
            .enumerate()
            .filter(|&(_, val)| val.internal.schedule == SystemSchedule::Never)
            .map(|(index, _)| index)
            .collect();
        for index in indices_to_remove.into_iter().rev() {
            self.systems_ref.drain(index..index + 1);
        }

        let indices_to_remove: Vec<usize> = self
            .systems_mut
            .iter()
            .enumerate()
            .filter(|&(_, val)| val.internal.schedule == SystemSchedule::Never)
            .map(|(index, _)| index)
            .collect();
        for index in indices_to_remove.into_iter().rev() {
            self.systems_mut.drain(index..index + 1);
        }
    }

    pub fn run_all(
        &mut self,
        component_manager: &mut super::component::ComponentManager,
        game: &mut dyn crate::Game,
    ) -> Result<(), ErrorType> {
        // By ref first
        // let mut systems_to_remove = Vec::with_capacity(self.systems_ref.len());
        for system in &mut self.systems_ref {
            match (system.internal.condition)(game) {
                Ok(should_run) => {
                    if should_run && system.internal.should_run_this_update() {
                        for entity in &system.internal.entities {
                            let value = match component_manager.get(&system.internal.name, entity) {
                                Ok(value) => value,
                                Err(err) => {
                                    log_error!(
                                        "Failed to get a component value when running systems: {:?}",
                                        err
                                    );
                                    return Err(ErrorType::Unknown);
                                }
                            };
                            if let Err(err) = (system.callback)(game, value) {
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

        // By mut then
        for system in &mut self.systems_mut {
            match (system.internal.condition)(game) {
                Ok(should_run) => {
                    if should_run && system.internal.should_run_this_update() {
                        for entity in &system.internal.entities {
                            let value = match component_manager
                                .get_mut(&system.internal.name, entity)
                            {
                                Ok(value) => value,
                                Err(err) => {
                                    log_error!(
                                        "Failed to get a component value when running systems: {:?}",
                                        err
                                    );
                                    return Err(ErrorType::Unknown);
                                }
                            };
                            if let Err(err) = (system.callback)(game, value) {
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

        // Clean empty systems
        self.clean_dead_systems();

        Ok(())
    }
}

impl crate::core_layer::ApplicationSystem<'_> {
    pub fn run_systems(&mut self) -> Result<(), ErrorType> {
        if let Err(err) = self
            .ecs
            .system_manager
            .run_all(&mut self.ecs.component_manager, self.user_game)
        {
            log_error!("Failed to run the systems in the application: {:?}", err);
            return Err(ErrorType::Unknown);
        }
        Ok(())
    }
}
