#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use std::sync::mpsc::Receiver;

use crate::{PlatformLayer, PlatformLayerImpl, core_layer::file_system::file::FileResource};

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
