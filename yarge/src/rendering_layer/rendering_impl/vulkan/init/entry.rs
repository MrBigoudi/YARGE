#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// Helper function to initiate the Vulkan entry
pub(in crate::rendering_layer::rendering_impl::vulkan) fn init_entry()
-> Result<ash::Entry, ErrorType> {
    match unsafe { ash::Entry::load() } {
        Ok(entry) => {
            log_info!("Vulkan entry initialized");
            Ok(entry)
        }
        Err(err) => {
            log_error!("Failed to load the Vulkan entry in ash: {:?}", err);
            Err(ErrorType::Unknown)
        }
    }
}

/// Shuts down the Vulkan entry
pub(in crate::rendering_layer::rendering_impl::vulkan) fn shutdown_entry(_entry: &ash::Entry) {
    log_info!("Vulkan entry shutted down");
}
