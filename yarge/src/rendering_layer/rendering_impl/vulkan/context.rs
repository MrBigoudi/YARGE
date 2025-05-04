use ash::{Instance, Entry};

use crate::{config::Config, log, log_error, error::ErrorType};

use super::init::{init_entry, init_instance};

/// The vulkan context
pub struct VulkanContext {
    pub entry: Entry,
    pub instance: Instance,
}

impl VulkanContext {
    pub fn new(config: &Config) -> Result<Self, ErrorType> {
        let entry = match init_entry() {
            Ok(entry) => entry,
            Err(err) => {
                log_error!("Failed to initialize the entry in the vulkan context: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };

        let instance = match init_instance(config) {
            Ok(instance) => instance,
            Err(err) => {
                log_error!("Failed to initialize the instance in the vulkan context: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };

        Ok(VulkanContext {
            entry,
            instance,
        })
    }
}