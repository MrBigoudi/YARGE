use std::sync::mpsc::Receiver;

use crate::{
    core_layer::file_system::file::{
        FileLoaderSystemInternal, FileResource, FileResourceTypeId, LoadingFile,
    },
    error::ErrorType,
    log_error,
    platform_layer::{PlatformLayer, PlatformLayerImpl},
};

/// A trait representing a resource that can be loaded from a .ron file
pub trait RonFileResource: FileResource + for<'a> serde::Deserialize<'a> {
    /// Begin to load a ron file
    fn start_load_ron(
        path: &std::path::Path,
    ) -> Result<Receiver<std::sync::Arc<dyn FileResource>>, ErrorType> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let cloned_path = std::path::PathBuf::from(path);
        let thread_name = String::from("ron_loader");

        if let Err(err) = std::thread::Builder::new()
            .name(thread_name)
            .spawn(move || {
                let data: String = match PlatformLayerImpl::load_to_string(&cloned_path) {
                    Ok(data) => data,
                    Err(err) => {
                        log_error!("Failed to load the ron file `{:?}': {:?}", &cloned_path, err);
                        panic!();
                    },
                };

                let loaded_data: Self = match ron::from_str(&data) {
                    Ok(loaded) => loaded,
                    Err(err) => {
                        log_error!("Failed to deserialize the ron file {:?}: {:?}", &cloned_path, err);
                        panic!();
                    },
                };

                let boxed_data: std::sync::Arc<dyn FileResource> = std::sync::Arc::new(loaded_data);
                if let Err(err) = sender.send(boxed_data) {
                    log_error!("Failed to send data between threads when loading the ron file `{:?}': {:?}", &cloned_path, err);
                    panic!();
                }
            })
        {
            log_error!("Failed to build a thread when loading the ron file `{:?}': {:?}", &path, err);
            return Err(ErrorType::Unknown);
        }

        Ok(receiver)
    }
}

impl LoadingFile {
    /// Begin to load a ron file
    fn start_load_ron<T: RonFileResource>(path: &std::path::Path) -> Result<Self, ErrorType> {
        let receiver = match T::start_load_ron(path) {
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

impl FileLoaderSystemInternal {
    /// Registers a new resource type in the system
    pub fn register<T: RonFileResource>(
        &mut self,
        id: &FileResourceTypeId,
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

        self.loaders
            .insert(id.clone(), |path| LoadingFile::start_load_ron::<T>(path));

        Ok(())
    }
}
