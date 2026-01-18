#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// Helper function to initiate the Vulkan allocator
pub(in crate::rendering_layer::rendering_impl::vulkan) fn init_allocator<'a>()
-> Result<Option<ash::vk::AllocationCallbacks<'a>>, ErrorType> {
    log_warn!("Vulkan custom allocator not implemented yet");
    Ok(None)
}

/// Shuts down the Vulkan allocator
pub(in crate::rendering_layer::rendering_impl::vulkan) fn shutdown_allocator<'a>(
    _allocator: &'a Option<ash::vk::AllocationCallbacks<'a>>,
) {
    log_info!("Vulkan allocator shutted down");
}
