#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use ash::{Entry, Instance};

use crate::{PlatformLayerImpl, config::Config, rendering_layer::rendering_impl::vulkan::init::{allocator::init_allocator, debug_messenger::{VkDebugMessenger, init_debug_messenger}, entry::init_entry, instance::init_instance}};

/// The Vulkan context
pub(crate) struct VulkanContext<'a> {
    /// The entry
    pub(crate) entry: Entry,
    /// The allocation callback
    pub(crate) allocator: Option<ash::vk::AllocationCallbacks<'a>>,
    /// The instance
    pub(crate) instance: Instance,
    /// The debug messenger
    pub(crate) debug_messenger: Option<VkDebugMessenger>,
}

impl VulkanContext<'_> {
    pub(crate) fn init(
        config: &Config,
        platform_layer: &PlatformLayerImpl,
    ) -> Result<Self, ErrorType> {
        let entry = match init_entry() {
            Ok(entry) => entry,
            Err(err) => {
                log_error!(
                    "Failed to initialize the entry in the Vulkan context: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let allocator = match init_allocator() {
            Ok(allocator) => allocator,
            Err(err) => {
                log_error!(
                    "Failed to initialize the allocation callback in the Vulkan context: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let instance = match init_instance(config, &entry, platform_layer, allocator.as_ref()) {
            Ok(instance) => instance,
            Err(err) => {
                log_error!(
                    "Failed to initialize the instance in the Vulkan context: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let debug_messenger = match init_debug_messenger(&entry, allocator.as_ref(), &instance){
            Ok(messenger) => messenger,
            Err(err) => {
                log_error!("Failed to initialize the debug messenger in the Vulkan context: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };

        log_info!("Vulkan context initialized");
        Ok(Self {
            entry,
            allocator,
            instance,
            debug_messenger,
        })
    }
}
