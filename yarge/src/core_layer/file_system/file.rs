#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use std::{
    collections::HashMap,
    sync::mpsc::{Receiver, TryRecvError},
};

use downcast_rs::impl_downcast;

/// A trait representing a file resource
pub trait FileResource: downcast_rs::DowncastSync + Send + Sync + 'static {}
impl_downcast!(sync FileResource);

/// A file that is being loaded
pub(crate) struct LoadingFile {
    pub(crate) receiver: Receiver<std::sync::Arc<dyn FileResource>>,
}

/// A type to abstract a loading function
pub(crate) type LoadingFileFunction =
    fn(&std::path::Path) -> Result<Receiver<std::sync::Arc<dyn FileResource>>, ErrorType>;

impl LoadingFile {
    /// Begin to load a file
    fn new(path: &std::path::Path, loading_fct: &LoadingFileFunction) -> Result<Self, ErrorType> {
        let receiver = match loading_fct(path) {
            Ok(receiver) => receiver,
            Err(err) => {
                log_error!(
                    "Failed to start to load the ron file `{:?}': {:?}",
                    path,
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        Ok(Self { receiver })
    }
}

/// A file that is done loading
pub(crate) struct LoadedFile {
    pub(crate) data: std::sync::Arc<dyn FileResource>,
}

/// The id of a resource type
pub type FileResourceTypeId = String;

/// The internal file loader system
pub(crate) struct FileLoaderSystem {
    /// The loading functions
    pub(crate) loaders: HashMap<FileResourceTypeId, LoadingFileFunction>,
    /// Files that are currently being loaded
    pub(crate) loading_files: HashMap<std::path::PathBuf, LoadingFile>,
    /// Files that are done being loaded and are now accessible
    pub(crate) loaded_files: HashMap<std::path::PathBuf, LoadedFile>,
}

impl FileLoaderSystem {
    /// Casts a user defined resource id into its safe version
    pub(crate) fn cast_resource_id(id: &FileResourceTypeId) -> FileResourceTypeId {
        String::from("user.") + id
    }

    /// A simple constructor
    pub(crate) fn init() -> Self {
        Self {
            loaders: HashMap::new(),
            loading_files: HashMap::new(),
            loaded_files: HashMap::new(),
        }
    }

    /// Registers a new resource type in the system
    pub(crate) fn register(
        &mut self,
        id: &FileResourceTypeId,
        loading_fct: LoadingFileFunction,
    ) -> Result<(), ErrorType> {
        if self.loaders.contains_key(id) {
            log_error!(
                "Failed to register a new file resource type: resource `{:?}' is already registered",
                id
            );
            return Err(ErrorType::WrongArgument(String::from(
                "Can't add twice the same resoure id",
            )));
        }

        let _ = self.loaders.insert(id.clone(), loading_fct);

        Ok(())
    }

    /// Begin to load a file
    pub(crate) fn start_load(
        &mut self,
        id: &FileResourceTypeId,
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

        match self.loaders.get(id) {
            None => {
                log_error!(
                    "Can't load a `{:?}' resource without registering it beforehand",
                    id
                );
                return Err(ErrorType::DoesNotExist);
            }
            Some(loader_function) => {
                let loading_file = match LoadingFile::new(path, loader_function) {
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
                let _ = self
                    .loading_files
                    .insert(std::path::PathBuf::from(path), loading_file);
            }
        }

        Ok(())
    }

    /// Check if the given file is done loading
    pub(crate) fn end_load(
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
                    if self.loading_files.remove(path).is_none() {
                        log_error!("File `{:?}' was not loading", path);
                        return Err(ErrorType::DoesNotExist);
                    }
                    if self
                        .loaded_files
                        .insert(std::path::PathBuf::from(path), LoadedFile { data })
                        .is_some()
                    {
                        log_error!("File `{:?}' has already been loaded", path);
                        return Err(ErrorType::Duplicate);
                    }
                    Ok(Some(arc))
                }
                Err(TryRecvError::Empty) => {
                    // log_info!("The `{:?}' file is not done loading yet", path);
                    Ok(None)
                }
                Err(err) => {
                    log_error!("Failed to load the `{:?}' file: {:?}", path, err);
                    Err(ErrorType::Unknown)
                }
            },
        }
    }

    /// Gets the path of all currently loading files
    pub(crate) fn get_loading_file_paths(&self) -> Vec<std::path::PathBuf> {
        let mut keys = vec![];
        for key in self.loading_files.keys() {
            keys.push(std::path::PathBuf::from(key));
        }
        keys
    }
}
