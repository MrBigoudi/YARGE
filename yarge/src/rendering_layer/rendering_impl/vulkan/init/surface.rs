#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{PlatformLayer, PlatformLayerImpl, platform_layer::window::Window};

/// A wrapper around a surface
pub(crate) struct VkSurface {
    /// The surface instance
    pub(crate) instance: ash::khr::surface::Instance,
    /// The actual surface
    pub(crate) surface: ash::vk::SurfaceKHR,
}

/// Initializes the surface
pub(crate) fn init_surface(
    platform_layer: &PlatformLayerImpl,
    entry: &ash::Entry,
    instance: &ash::Instance,
    allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
) -> Result<VkSurface, ErrorType> {
    let surface_instance = ash::khr::surface::Instance::new(entry, instance);

    let surface = match platform_layer
        .get_window_ref(0u8)
        .vulkan_get_surface(entry, instance, allocator)
    {
        Ok(surface) => surface,
        Err(err) => {
            log_error!(
                "Failed to initialize the Vulkan surface from the platform layer: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    log_info!("Vulkan surface initialized");
    Ok(VkSurface {
        instance: surface_instance,
        surface,
    })
}
