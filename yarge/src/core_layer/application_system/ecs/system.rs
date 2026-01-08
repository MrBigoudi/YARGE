use std::marker::PhantomData;

use crate::{Component, error::ErrorType};

#[allow(unused)]
use crate::{log_debug, log_error, log_info, log_warn};

/// An empty system type
pub struct SystemNil;

/// A Cons pair system type
pub struct SystemCons<H, T>(PhantomData<(H, T)>);

pub trait ComponentList {
    fn get_ids() -> Vec<std::any::TypeId>;
}

impl ComponentList for SystemNil {
    fn get_ids() -> Vec<std::any::TypeId> {
        Vec::new()
    }
}

impl<H: Component, T: ComponentList> ComponentList for SystemCons<H, T> {
    fn get_ids() -> Vec<std::any::TypeId> {
        let mut ids = vec![H::get_type_id()];
        ids.extend(T::get_ids());
        ids
    }
}

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

pub type UserSystemCallback<G, T> = fn(&mut G, &T) -> Result<(), ErrorType>;
pub type UserSystemMutCallback<G, T> = fn(&mut G, &mut T) -> Result<(), ErrorType>;

pub struct UserSystemCallbackBuilder;
impl UserSystemCallbackBuilder {
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
                    log_error!("Failed to downcast game `{}`", std::any::type_name::<G>());
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
                    log_error!("Failed to downcast game `{}`", std::any::type_name::<G>());
                    return Err(ErrorType::Unknown);
                }
            };
            callback_mut(game, value)
        })
    }
}

pub(crate) struct SystemRef {
    pub callback: SystemCallback,
    pub name: std::any::TypeId,
    pub with: Vec<std::any::TypeId>,
    pub without: Vec<std::any::TypeId>,
    // TODO: add if condition
    // TODO: add schedule (Once, Everytime)
}

pub(crate) struct SystemMut {
    pub callback: SystemMutCallback,
    pub name: std::any::TypeId,
    pub with: Vec<std::any::TypeId>,
    pub without: Vec<std::any::TypeId>,
    // TODO: add if condition
    // TODO: add schedule (Once, Everytime)
}

pub(crate) struct SystemManager {
    // TODO: replace the list by generational indices list
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
}
