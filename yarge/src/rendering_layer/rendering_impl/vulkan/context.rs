#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use ash::{Entry, Instance};

use crate::config::Config;

use super::init::{init_entry, init_instance};

/// The vulkan context
pub struct VulkanContext {
    pub entry: Entry,

    pub instance: Instance,
}

impl VulkanContext {
    pub fn new(config: &Config) -> Result<Self, ErrorType> {
        let entry = match init_entry() {
            Ok(entry) => {
                log_info!("Vulkan context entry initialized");
                entry
            }
            Err(err) => {
                log_error!(
                    "Failed to initialize the entry in the vulkan context: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let instance = match init_instance(config, &entry) {
            Ok(instance) => {
                log_info!("Vulkan context instance initialized");
                instance
            }
            Err(err) => {
                log_error!(
                    "Failed to initialize the instance in the vulkan context: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        Ok(Self { entry, instance })
    }
}
