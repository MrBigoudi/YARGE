use std::{
    collections::HashMap,
    path::PathBuf,
    sync::mpsc::{Receiver, TryRecvError},
};

use crate::{
    core_layer::file_system::ron::RonFileResource, error::ErrorType, log_error, log_info, log_warn,
};

/// A trait representing a file resource
pub trait FileResource: downcast_rs::DowncastSync + Send + Sync + 'static {}
impl_downcast!(sync FileResource);

/// A file that is being loaded
pub struct LoadingFile {
    pub(crate) receiver: Receiver<std::sync::Arc<dyn FileResource>>,
}

/// A file that is done loading
pub struct LoadedFile {
    pub(crate) data: std::sync::Arc<dyn FileResource>,
}

/// The id of a resource type
pub type FileResourceTypeId = String;
/// A type for loader function signature
type FileLoaderFunction = fn(&std::path::Path) -> Result<LoadingFile, ErrorType>;

/// The internal file loader system
pub struct FileLoaderSystemInternal {
    /// The loading functions
    pub(crate) loaders: HashMap<FileResourceTypeId, FileLoaderFunction>,
    /// Files that are currently being loaded
    pub(crate) loading_files: HashMap<PathBuf, LoadingFile>,
    /// Files that are done being loaded and are now accessible
    pub(crate) loaded_files: HashMap<PathBuf, LoadedFile>,
}

impl FileLoaderSystemInternal {
    /// A simple constructor
    pub fn new() -> Self {
        Self {
            loaders: HashMap::new(),
            loading_files: HashMap::new(),
            loaded_files: HashMap::new(),
        }
    }

    /// Begin to load a file
    pub fn start_load(
        &mut self,
        id: FileResourceTypeId,
        path: &std::path::Path,
    ) -> Result<(), ErrorType> {
        if self.loading_files.contains_key(path) {
            log_warn!("The `{:?}' file is already starting to load", path);
            return Ok(());
        }

        if self.loaded_files.contains_key(path) {
            log_warn!("The `{:?}' file is already loaded", path);
            return Ok(());
        }

        match self.loaders.get(&id) {
            None => {
                log_error!(
                    "Can't load a `{:?}' resource without registering it beforehand",
                    id
                );
                return Err(ErrorType::DoesNotExist);
            }
            Some(loader_function) => {
                let loading_file = match loader_function(path) {
                    Ok(file) => file,
                    Err(err) => {
                        log_error!(
                            "Failed to load a `{:?}' resource at `{:?}': {:?}",
                            id,
                            path,
                            err
                        );
                        return Err(ErrorType::Unknown);
                    }
                };
                self.loading_files
                    .insert(std::path::PathBuf::from(path), loading_file);
            }
        }

        Ok(())
    }

    /// Check if the given file is done loading
    pub fn end_load(
        &mut self,
        path: &std::path::Path,
    ) -> Result<Option<std::sync::Arc<dyn FileResource>>, ErrorType> {
        if let Some(loaded_file) = self.loaded_files.get(path) {
            return Ok(Some(std::sync::Arc::clone(&loaded_file.data)));
        }

        match self.loading_files.get(path) {
            None => {
                log_error!("File `{:?}' has never started to load", path);
                Err(ErrorType::DoesNotExist)
            }
            Some(loading_file) => match loading_file.receiver.try_recv() {
                Ok(data) => {
                    let arc = std::sync::Arc::clone(&data);
                    self.loading_files.remove(path);
                    self.loaded_files
                        .insert(std::path::PathBuf::from(path), LoadedFile { data });
                    Ok(Some(arc))
                }
                Err(TryRecvError::Empty) => {
                    log_info!("The `{:?}' file has not done loading yet", path);
                    Ok(None)
                }
                Err(err) => {
                    log_error!("Failed to load the `{:?}' file: {:?}", path, err);
                    Err(ErrorType::Unknown)
                }
            },
        }
    }
}

use downcast_rs::impl_downcast;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub(crate) static GLOBAL_FILE_LOADER: Lazy<Mutex<FileLoaderSystemInternal>> =
    Lazy::new(|| Mutex::new(FileLoaderSystemInternal::new()));

/// Static loader system that can be used by both the user and the engine
/// Before loading a custom resource, it must be registered to this system
pub struct FileLoaderSystem;

impl FileLoaderSystem {
    /// Casts a user defined resource id into its safe version
    fn cast_resource_id(id: &FileResourceTypeId) -> FileResourceTypeId {
        String::from("user.") + id
    }

    /// Registers a new resource type in the system
    pub fn register<T: RonFileResource>(id: &FileResourceTypeId) -> Result<(), ErrorType> {
        let correct_id = Self::cast_resource_id(id);
        match GLOBAL_FILE_LOADER.lock() {
            Ok(mut mutex) => mutex.register::<T>(&correct_id),
            Err(err) => {
                log_error!("Failed to register a file resource: {:?}", err);
                Err(ErrorType::Unknown)
            }
        }
    }

    /// Begin to load a file
    pub fn start_load(id: &FileResourceTypeId, path: &std::path::Path) -> Result<(), ErrorType> {
        let correct_id = Self::cast_resource_id(id);
        match GLOBAL_FILE_LOADER.lock() {
            Ok(mut mutex) => mutex.start_load(correct_id, path),
            Err(err) => {
                log_error!("Failed to start loading a file resource: {:?}", err);
                Err(ErrorType::Unknown)
            }
        }
    }

    /// Check if the given file is done loading
    pub fn end_load<T: FileResource>(
        path: &std::path::Path,
    ) -> Result<Option<std::sync::Arc<T>>, ErrorType> {
        match GLOBAL_FILE_LOADER.lock() {
            Ok(mut mutex) => {
                if let Some(arc) = mutex.end_load(path)? {
                    match std::sync::Arc::downcast::<T>(arc) {
                        Ok(arc) => Ok(Some(arc)),
                        Err(err) => {
                            log_error!(
                                "Failed to downcast an Arc when loading the `{:?}' file: {:?}",
                                path,
                                err
                            );
                            Err(ErrorType::Unknown)
                        }
                    }
                } else {
                    Ok(None)
                }
            }
            Err(err) => {
                log_error!("Failed to end loading a file resource: {:?}", err);
                Err(ErrorType::Unknown)
            }
        }
    }
}
