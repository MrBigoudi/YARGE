#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use std::{
    collections::{HashMap, HashSet},
    sync::mpsc::Receiver,
};

pub(crate) type ResourceLoadingFunction =
    std::sync::Arc<dyn Fn() -> Result<ResourceHandle, ErrorType> + Send + Sync>;

pub(crate) struct ResourceLoadingBuilder;
impl ResourceLoadingBuilder {
    pub(crate) fn loader<P, R>(params: &P) -> ResourceLoadingFunction
    where
        P: ResourceLoadingParameters<R>,
        R: Resource,
    {
        let params = params.clone();
        std::sync::Arc::new(move || {
            let loaded_data = match params.load_resource() {
                Ok(data) => data,
                Err(err) => {
                    log_error!("Failed to load the `{:?}' resource: {:?}", params, err);
                    return Err(ErrorType::Unknown);
                }
            };
            let handler = std::sync::Arc::new(loaded_data);
            Ok(ResourceHandle(handler))
        })
    }
}

pub(crate) trait ResourceLoadingParameters<R: Resource>:
    std::hash::Hash + std::any::Any + Send + Sync + std::fmt::Debug + Clone + 'static
{
    fn load_resource(&self) -> Result<R, ErrorType>;
}
pub trait Resource: std::any::Any + Send + Sync + 'static {}

/// A user defined resource
pub trait UserResource: std::any::Any + Send + Sync + Clone + 'static {}
impl<T: UserResource> Resource for T {}
pub trait UserResourceLoadingParameters<R: UserResource>:
    std::hash::Hash + std::any::Any + Send + Sync + std::fmt::Debug + Clone + 'static
{
    fn load_resource(&self) -> Result<R, ErrorType>;
}

impl<T, R> ResourceLoadingParameters<R> for T
where
    T: UserResourceLoadingParameters<R>,
    R: UserResource,
{
    fn load_resource(&self) -> Result<R, ErrorType> {
        UserResourceLoadingParameters::load_resource(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ResourceId(super::generational::GenerationalKey);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserResourceId(super::generational::GenerationalKey);
#[derive(Clone)]
pub struct ResourceHandle(std::sync::Arc<dyn Resource>);

impl ResourceHandle {
    pub fn get_clone<R: Resource + Clone + 'static>(&self) -> Result<R, ErrorType> {
        match std::sync::Arc::downcast::<R>(self.0.clone()) {
            Ok(new) => Ok((*new).clone()),
            Err(err) => {
                log_error!(
                    "Failed to downcast a resource handler to the real resource: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
    }
}

pub(crate) struct LoadingResource {
    pub(crate) receiver: Receiver<ResourceHandle>,
}

impl LoadingResource {
    pub(crate) fn new(
        resource_id: &UserResourceId,
        loading_function: &ResourceLoadingFunction,
    ) -> Result<Self, ErrorType> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let thread_name = format!(
            "load_resource_id_{}_gen_{}",
            resource_id.0.index, resource_id.0.generation
        );

        let loading_function = std::sync::Arc::clone(loading_function);
        if let Err(err) = std::thread::Builder::new()
            .name(thread_name)
            .spawn(move || {
                let data = match loading_function() {
                    Ok(data) => data,
                    Err(err) => {
                        log_error!("Failed to load a resource: {:?}", err);
                        return Err(ErrorType::Unknown);
                    }
                };

                if let Err(err) = sender.send(data) {
                    log_error!(
                        "Failed to send data between threads when loading a resource: {:?}",
                        err
                    );
                    panic!();
                };

                Ok(())
            })
        {
            log_error!(
                "Failed to build a thread when loading a resource: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        Ok(Self { receiver })
    }
}

pub(crate) struct LoadedResource {
    pub(crate) handler: ResourceHandle,
}

pub(crate) enum RealResource {
    Loading(LoadingResource),
    Loaded(LoadedResource),
}
pub(crate) struct ResourcesStorage(super::generational::GenerationalVec<RealResource>);

pub(crate) struct ResourceIdGenerator {
    pub(crate) table: HashMap<UserResourceId, ResourceId>,
    pub(crate) inv_table: HashMap<ResourceId, UserResourceId>,
    pub(crate) nb_ids_total: usize,
    pub(crate) generation: super::generational::GenerationalGeneration,
}

impl ResourceIdGenerator {
    /// Shuts down the ID generator
    pub(crate) fn shutdown(&mut self) {
        self.table = HashMap::new();
        self.inv_table = HashMap::new();
    }

    /// Creates a new generator
    pub(crate) fn init() -> Self {
        Self {
            table: HashMap::new(),
            inv_table: HashMap::new(),
            nb_ids_total: 0,
            generation: 0,
        }
    }

    pub(crate) fn generate_id(&mut self) -> UserResourceId {
        match self.nb_ids_total.checked_add(1) {
            Some(res) => self.nb_ids_total = res,
            None => {
                self.nb_ids_total = 0;
                self.generation += 1;
            }
        }
        UserResourceId(super::generational::GenerationalKey {
            index: self.nb_ids_total,
            generation: self.generation,
        })
    }

    pub(crate) fn get_real_id(&self, id: &UserResourceId) -> Result<ResourceId, ErrorType> {
        match self.table.get(id) {
            Some(real_id) => Ok(*real_id),
            None => {
                log_error!("Failed to retrieve the real resource id from user resource id");
                Err(ErrorType::DoesNotExist)
            }
        }
    }

    pub(crate) fn get_user_id(&self, id: &ResourceId) -> Result<UserResourceId, ErrorType> {
        match self.inv_table.get(id) {
            Some(user_id) => Ok(*user_id),
            None => {
                log_error!("Failed to retrieve the user resource id from real resource id");
                Err(ErrorType::DoesNotExist)
            }
        }
    }

    pub(crate) fn get_real_ids(
        &self,
        ids: &[UserResourceId],
    ) -> Result<Vec<ResourceId>, ErrorType> {
        let mut output = Vec::with_capacity(ids.len());
        for id in ids {
            output.push(self.get_real_id(id)?);
        }
        Ok(output)
    }

    pub(crate) fn insert(
        &mut self,
        id: &UserResourceId,
        real_id: &ResourceId,
    ) -> Result<(), ErrorType> {
        if self.table.insert(*id, *real_id).is_some() {
            log_error!("Failed to add a new key in the resource id table");
            return Err(ErrorType::Duplicate);
        }
        Ok(())
    }

    pub(crate) fn insert_inv(
        &mut self,
        id: &ResourceId,
        user_id: &UserResourceId,
    ) -> Result<(), ErrorType> {
        if self.inv_table.insert(*id, *user_id).is_some() {
            log_error!("Failed to add a new key in the inverse resource id table");
            return Err(ErrorType::Duplicate);
        }
        Ok(())
    }
}

use crate::platform_layer::platform_impl::PlatformLayerRwLock;

/// The global entity generator to interface between user request and real resources
pub(crate) static GLOBAL_RESOURCE_ID_GENERATOR: once_cell::sync::Lazy<
    PlatformLayerRwLock<ResourceIdGenerator>,
> = once_cell::sync::Lazy::new(|| PlatformLayerRwLock::new(ResourceIdGenerator::init()));

pub(crate) struct ResourceManager {
    pub(crate) loading_functions: HashMap<ResourceId, ResourceLoadingFunction>,
    pub(crate) resources: ResourcesStorage,
    pub(crate) real_resources: HashMap<std::any::TypeId, HashSet<ResourceId>>,
    pub(crate) loading_resources: HashSet<(std::any::TypeId, ResourceId)>,
}

impl ResourceManager {
    pub(crate) fn shutdown(&mut self) -> Result<(), ErrorType> {
        match GLOBAL_RESOURCE_ID_GENERATOR.write() {
            Ok(mut generator) => {
                generator.shutdown();
            }
            Err(err) => {
                log_error!(
                    "Failed to access the global resource id generator when shutting down the resource manager: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        }
        log_info!("Resource manager shutted down");
        Ok(())
    }

    pub(crate) fn init() -> Self {
        let res = Self {
            loading_functions: HashMap::new(),
            resources: ResourcesStorage(
                super::generational::GenerationalVec::<RealResource>::init_empty(),
            ),
            real_resources: HashMap::new(),
            loading_resources: HashSet::new(),
        };
        log_info!("Resource manager initialized");
        res
    }

    /// Creates a new resource
    /// This method is for the User
    pub(crate) fn generate_id() -> Result<UserResourceId, ErrorType> {
        match GLOBAL_RESOURCE_ID_GENERATOR.write() {
            Ok(mut generator) => Ok(generator.generate_id()),
            Err(err) => {
                log_error!(
                    "Failed to access the global resource id generator when generating resource id: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
    }

    pub(crate) fn get_user_id(real_id: &ResourceId) -> Result<UserResourceId, ErrorType> {
        match GLOBAL_RESOURCE_ID_GENERATOR.read() {
            Ok(generator) => generator.get_user_id(real_id),
            Err(err) => {
                log_error!(
                    "Failed to access the global resource id generator when getting user id from real id: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
    }

    pub(crate) fn get_real_id(user_id: &UserResourceId) -> Result<ResourceId, ErrorType> {
        match GLOBAL_RESOURCE_ID_GENERATOR.read() {
            Ok(generator) => generator.get_real_id(user_id),
            Err(err) => {
                log_error!(
                    "Failed to access the global resource id generator when getting real id from user id: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
    }

    pub(crate) fn add(
        &mut self,
        user_id: &UserResourceId,
        resource_type_id: &std::any::TypeId,
        loading_function: ResourceLoadingFunction,
    ) -> Result<(), ErrorType> {
        // Generate real id
        let real_id = ResourceId(match self.resources.0.insert_empty_entries(1, true) {
            Ok(Some(id)) if id.len() == 1 => id[0],
            _ => {
                log_error!(
                    "Failed to generate a real id when adding a new resource in the manager"
                );
                return Err(ErrorType::Unknown);
            }
        });

        // Store real id in generator
        match GLOBAL_RESOURCE_ID_GENERATOR.write() {
            Ok(mut generator) => {
                if let Err(err) = generator.insert(user_id, &real_id) {
                    log_error!(
                        "Failed to add a user id to real id entry in the resource id generator: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
                if let Err(err) = generator.insert_inv(&real_id, user_id) {
                    log_error!(
                        "Failed to add a real id to user id entry in the resource id generator: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            Err(err) => {
                log_error!(
                    "Failed to access the global resource id generator when generating resource id: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        }
        if !self.real_resources.contains_key(resource_type_id) {
            let _ = self
                .real_resources
                .insert(*resource_type_id, HashSet::new());
        }
        let set = self.real_resources.get_mut(resource_type_id).unwrap();
        if !set.insert(real_id) {
            log_error!("Failed to add a real id when adding a new resource in the manager");
            return Err(ErrorType::Duplicate);
        }
        // Generate new loading function
        if self
            .loading_functions
            .insert(real_id, loading_function)
            .is_some()
        {
            log_error!(
                "Failed to add a loading function when adding a new resource in the manager"
            );
            return Err(ErrorType::Duplicate);
        }

        Ok(())
    }

    pub(crate) fn get_all_ids(
        &self,
        resource_type_id: &std::any::TypeId,
    ) -> Result<&HashSet<ResourceId>, ErrorType> {
        if !self.real_resources.contains_key(resource_type_id) {
            log_error!(
                "Failed to get all resource ids of the `{:?}' resource type from the resource manager",
                resource_type_id
            );
            return Err(ErrorType::DoesNotExist);
        }
        let set = self.real_resources.get(resource_type_id).unwrap();
        Ok(set)
    }

    pub(crate) fn try_get(
        &mut self,
        id: &ResourceId,
        resource_type_id: &std::any::TypeId,
    ) -> Result<Option<ResourceHandle>, ErrorType> {
        let set = self.get_all_ids(resource_type_id)?;
        if !set.contains(id) {
            log_error!(
                "Unknown real resource id found when trying to access resource in the resource manager"
            );
            return Err(ErrorType::DoesNotExist);
        }
        match self.resources.0.get_mut_entry(&id.0) {
            Ok(entry) => {
                match entry {
                    super::generational::Entry::Free { .. } => {
                        log_error!(
                            "Invalid real resource id found when trying to access resource in the resource manager"
                        );
                        Err(ErrorType::DoesNotExist)
                    }
                    super::generational::Entry::Occupied { value } => match value {
                        // if empty: starts to load and return None
                        None => {
                            let loading_function = match self.loading_functions.get(id) {
                                None => {
                                    log_error!(
                                        "Failed to find a loading function when trying to access resource in the resource manager"
                                    );
                                    return Err(ErrorType::DoesNotExist);
                                }
                                Some(fct) => fct,
                            };
                            let user_id = match Self::get_user_id(id) {
                                Ok(id) => id,
                                Err(err) => {
                                    log_error!("Failed to get user id from resource id: {:?}", err);
                                    return Err(ErrorType::Unknown);
                                }
                            };
                            let loading_resource =
                                match LoadingResource::new(&user_id, loading_function) {
                                    Ok(loading) => loading,
                                    Err(err) => {
                                        log_error!("Failed to start loading a resource: {:?}", err);
                                        return Err(ErrorType::Unknown);
                                    }
                                };
                            // TODO: add check
                            let _ = self.loading_resources.insert((*resource_type_id, *id));
                            *value = Some(RealResource::Loading(loading_resource));
                            Ok(None)
                        }
                        // if loading: return None
                        Some(RealResource::Loading(loading_resource)) => {
                            match loading_resource.receiver.try_recv() {
                                Ok(data) => {
                                    // TODO: add check
                                    let _ =
                                        self.loading_resources.remove(&(*resource_type_id, *id));
                                    let handler = ResourceHandle::clone(&data);
                                    *value = Some(RealResource::Loaded(LoadedResource {
                                        handler: data,
                                    }));
                                    Ok(Some(handler))
                                }
                                Err(std::sync::mpsc::TryRecvError::Empty) => {
                                    // log_info!("The `{:?}' file is not done loading yet", path);
                                    Ok(None)
                                }
                                Err(err) => {
                                    log_error!(
                                        "Failed to load a resource in the resource manager: {:?}",
                                        err
                                    );
                                    Err(ErrorType::Unknown)
                                }
                            }
                        }
                        // if loaded: return handle
                        Some(RealResource::Loaded(LoadedResource { handler })) => {
                            Ok(Some(ResourceHandle::clone(handler)))
                        }
                    },
                }
            }
            Err(err) => {
                log_error!(
                    "Failed to get an entry when trying to access resource in the resource manager: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
    }

    pub(crate) fn get(
        &mut self,
        id: &ResourceId,
        resource_type_id: &std::any::TypeId,
    ) -> Result<ResourceHandle, ErrorType> {
        loop {
            match self.try_get(id, resource_type_id) {
                Err(err) => {
                    log_error!(
                        "Failed to get the `{:?}' resource from the resource manager: {:?}",
                        id,
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
                Ok(Some(handler)) => return Ok(handler),
                Ok(None) => continue,
            }
        }
    }
}
