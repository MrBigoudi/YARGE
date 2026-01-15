#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

unsafe extern "system" fn pfn_user_callback(
    message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    message_types: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT<'_>,
    _p_user_data: *mut std::ffi::c_void,
) -> ash::vk::Bool32 {
    let callback_data = unsafe { *p_callback_data };

    let message = if callback_data.p_message.is_null() {
        "<no message>"
    } else {
        unsafe { &std::ffi::CStr::from_ptr(callback_data.p_message).to_string_lossy() }
    };

    // Optional: message ID
    let message_id = if callback_data.p_message_id_name.is_null() {
        "<unknown>"
    } else {
        unsafe { &std::ffi::CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy() }
    };

    // Optional: pretty-print message types
    let types = format!("{:?}", message_types);

    match message_severity {
        ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => {
            log_error!("[VULKAN][{}][{}] {}", message_id, types, message);
        }
        ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => {
            log_warn!("[VULKAN][{}][{}] {}", message_id, types, message);
        }
        ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO => {
            log_info!("[VULKAN][{}][{}] {}", message_id, types, message);
        }
        _ => {
            log_debug!("[VULKAN][{}][{}] {}", message_id, types, message);
        }
    }

    ash::vk::FALSE
}

/// The debug messenger
pub(crate) struct VkDebugMessenger {
    /// The debug utils instance
    pub(crate) instance: ash::ext::debug_utils::Instance,
    /// The debug messenger
    pub(crate) messenger: ash::vk::DebugUtilsMessengerEXT,
}


/// Helper function to initiate the Vulkan debug messenger
pub(crate) fn init_debug_messenger(entry: &ash::Entry, allocator: Option<&ash::vk::AllocationCallbacks<'_>>, instance: &ash::Instance) -> Result<Option<VkDebugMessenger>, ErrorType> {
    #[cfg(not(debug_assertions))]
    {
        log_warn!("The Vulkan debug messenger will not be set without rust debug assertions enabled");
        Ok(None)
    }
    #[cfg(debug_assertions)]
    {
        let severity_flags = ash::vk::DebugUtilsMessageSeverityFlagsEXT::empty()
            | ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
            | ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
            | ash::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
            | ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO
        ;
        let message_type_flags = ash::vk::DebugUtilsMessageTypeFlagsEXT::empty()
            | ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
            | ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
        ;
        let debug_messenger_info = ash::vk::DebugUtilsMessengerCreateInfoEXT::default()
            .message_severity(severity_flags)
            .message_type(message_type_flags)
            .pfn_user_callback(Some(pfn_user_callback))
        ;

        let debug_instance = ash::ext::debug_utils::Instance::new(entry, instance);
        let debug_messenger = match unsafe { debug_instance.create_debug_utils_messenger(
            &debug_messenger_info,
            allocator,
        ) }{
            Ok(messenger) => messenger,
            Err(err) => {
                log_error!("Failed to create the Vulkan debug utils messenger: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };

        log_info!("Vulkan debug messenger initialized");
        Ok(Some(VkDebugMessenger {
            instance: debug_instance,
            messenger: debug_messenger,
        }))
    }
}
