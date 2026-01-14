#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// Helper function to initiate the vulkan allocator
pub(crate) fn init_allocator<'a>() -> Result<Option<ash::vk::AllocationCallbacks<'a>>, ErrorType> {
    log_warn!("Vulkan custom allocator not implemented yet");
    Ok(None)
}
