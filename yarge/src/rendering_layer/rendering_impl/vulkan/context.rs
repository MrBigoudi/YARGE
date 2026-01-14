#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use ash::{Entry, Instance};

use crate::{PlatformLayerImpl, config::Config};

use super::init::{init_entry, init_allocator, init_instance};

/// The vulkan context
pub(crate) struct VulkanContext<'a> {
    /// The vulkan entry
    pub(crate) entry: Entry,
    /// The vulkan allocation callback
    pub(crate) allocator: Option<ash::vk::AllocationCallbacks<'a>>,
    /// The vulkan instance
    pub(crate) instance: Instance,
}

impl VulkanContext<'_> {
    pub(crate) fn init(config: &Config, platform_layer: &PlatformLayerImpl) -> Result<Self, ErrorType> {
        let entry = match init_entry() {
            Ok(entry) => entry,
            Err(err) => {
                log_error!(
                    "Failed to initialize the entry in the vulkan context: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let allocator = match init_allocator() {
            Ok(allocator) => allocator,
            Err(err) => {
                log_error!(
                    "Failed to initialize the allocation callback in the vulkan context: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let instance = match init_instance(config, &entry, platform_layer, &allocator) {
            Ok(instance) => instance,
            Err(err) => {
                log_error!(
                    "Failed to initialize the instance in the vulkan context: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        log_info!("Vulkan context initialized");
        Ok(Self { 
            entry,
            allocator,
            instance,
        })
    }
}
