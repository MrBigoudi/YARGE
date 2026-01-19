#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{
    PlatformLayerImpl,
    config::Config,
    rendering_layer::rendering_impl::vulkan::init::{
        allocator, debug_messenger, device, entry, instance, physical_device, surface, swapchain,
    },
};

/// The Vulkan context
pub(in crate::rendering_layer::rendering_impl::vulkan) struct VulkanContext<'a> {
    /// The entry
    pub(in crate::rendering_layer::rendering_impl::vulkan) entry: ash::Entry,
    /// The allocation callback
    pub(in crate::rendering_layer::rendering_impl::vulkan) allocator:
        Option<ash::vk::AllocationCallbacks<'a>>,
    /// The instance
    pub(in crate::rendering_layer::rendering_impl::vulkan) instance: ash::Instance,
    /// The debug messenger
    pub(in crate::rendering_layer::rendering_impl::vulkan) debug_messenger:
        Option<debug_messenger::VkDebugMessenger>,
    /// The window surface
    pub(in crate::rendering_layer::rendering_impl::vulkan) surface_wrapper: surface::VkSurface,
    /// The physical device
    pub(in crate::rendering_layer::rendering_impl::vulkan) physical_device: ash::vk::PhysicalDevice,
    /// The logical device
    pub(in crate::rendering_layer::rendering_impl::vulkan) device_wrapper: device::VkDevice,
    /// The swapchain
    pub(in crate::rendering_layer::rendering_impl::vulkan) swapchain_wrapper:
        swapchain::VkSwapchain,
}

impl VulkanContext<'_> {
    pub(in crate::rendering_layer::rendering_impl::vulkan) fn init(
        config: &Config,
        platform_layer: &PlatformLayerImpl,
    ) -> Result<Self, ErrorType> {
        let entry = match entry::init_entry() {
            Ok(entry) => entry,
            Err(err) => {
                log_error!(
                    "Failed to initialize the entry in the Vulkan context: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let allocator = match allocator::init_allocator() {
            Ok(allocator) => allocator,
            Err(err) => {
                log_error!(
                    "Failed to initialize the allocation callback in the Vulkan context: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let instance =
            match instance::init_instance(config, &entry, platform_layer, allocator.as_ref()) {
                Ok(instance) => instance,
                Err(err) => {
                    log_error!(
                        "Failed to initialize the instance in the Vulkan context: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            };

        let debug_messenger =
            match debug_messenger::init_debug_messenger(&entry, allocator.as_ref(), &instance) {
                Ok(messenger) => messenger,
                Err(err) => {
                    log_error!(
                        "Failed to initialize the debug messenger in the Vulkan context: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            };

        let surface_wrapper =
            match surface::init_surface(platform_layer, &entry, &instance, allocator.as_ref()) {
                Ok(surface) => surface,
                Err(err) => {
                    log_error!(
                        "Failed to initialize the surface in the Vulkan context: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            };

        let physical_device = match physical_device::init_physical_device(config, &instance) {
            Ok(device) => device,
            Err(err) => {
                log_error!(
                    "Failed to initialize the physical device in the Vulkan context: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let device_wrapper = match device::init_device(
            config,
            &instance,
            &surface_wrapper,
            &physical_device,
            allocator.as_ref(),
        ) {
            Ok(device) => device,
            Err(err) => {
                log_error!(
                    "Failed to initialize the device in the Vulkan context: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };

        let swapchain_wrapper = match swapchain::init_swapchain(
            config,
            platform_layer,
            &entry,
            &instance,
            &physical_device,
            &device_wrapper,
            &surface_wrapper,
            allocator.as_ref(),
        ) {
            Ok(swapchain) => swapchain,
            Err(err) => {
                log_error!(
                    "Failed to initialize the swapchain in the Vulkan context: {:?}",
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
            debug_messenger,
            surface_wrapper,
            physical_device,
            device_wrapper,
            swapchain_wrapper,
        })
    }

    pub(in crate::rendering_layer::rendering_impl::vulkan) fn shutdown(
        &mut self,
    ) -> Result<(), ErrorType> {
        let allocator = self.allocator.as_ref();
        swapchain::shutdown_swapchain(&self.swapchain_wrapper, allocator);

        device::shutdown_device(&self.device_wrapper, allocator);

        physical_device::shutdown_device(&self.physical_device, allocator);

        surface::shutdown_surface(&self.surface_wrapper, allocator);

        debug_messenger::shutdown_debug_messenger(&self.debug_messenger, allocator);
        self.debug_messenger = None;

        instance::shutdown_instance(&self.instance, allocator);

        allocator::shutdown_allocator(&self.allocator);
        self.allocator = None;

        entry::shutdown_entry(&self.entry);

        Ok(())
    }
}
