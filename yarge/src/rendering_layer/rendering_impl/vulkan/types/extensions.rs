#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::rendering_layer::rendering_impl::types::{VkNames, convert_string_to_vknames};

/// Custom trait for Vulkan extensions
pub(crate) trait VkExtension: Sized {
    /// Converts an Extension into a String
    fn as_string(&self) -> String;

    /// Converts an extension in VkNames
    /// [VkNames]
    fn to_vknames(extensions: &[Self]) -> Result<VkNames, ErrorType> {
        let extensions_string: Vec<String> =
            extensions.iter().map(|layer| layer.as_string()).collect();
        convert_string_to_vknames(&extensions_string)
    }
}

/// Custom enum for Vulkan Instance Extensions
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub(crate) enum VkInstanceExtensions {
    /// Khronos surface instance extension
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_surface.html
    KhrSurface,
    /// Khronos android surface instance extension
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_android_surface.html
    KhrAndroidSurface,
    /// Khronos instance extension
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_wayland_surface.html
    KhrWaylandSurface,
    /// Khronos instance extension
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_win32_surface.html
    KhrWin32Surface,
    /// Khronos instance extension
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_xcb_surface.html
    KhrXcbSurface,
    /// Khronos instance extension
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_xlib_surface.html
    KhrXlibSurface,
    /// Extension to setup a debug messenger callback
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VK_EXT_debug_utils.html
    ExtDebugUtils,
}

impl VkExtension for VkInstanceExtensions {
    fn as_string(&self) -> String {
        match self {
            VkInstanceExtensions::KhrSurface => String::from("VK_KHR_surface"),
            VkInstanceExtensions::KhrAndroidSurface => String::from("VK_KHR_android_surface"),
            VkInstanceExtensions::KhrWaylandSurface => String::from("VK_KHR_wayland_surface"),
            VkInstanceExtensions::KhrWin32Surface => String::from("VK_KHR_win32_surface"),
            VkInstanceExtensions::KhrXcbSurface => String::from("VK_KHR_xcb_surface"),
            VkInstanceExtensions::KhrXlibSurface => String::from("VK_KHR_xlib_surface"),
            VkInstanceExtensions::ExtDebugUtils => String::from("VK_EXT_debug_utils"),
        }
    }
}

/// Custom enum for Vulkan Device Extensions
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub(crate) enum VkDeviceExtensions {
    /// Enables the use of SwapchainKHR objects which provide the ability to present rendering results to a surface
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_swapchain.html
    KhrSwapchain,

    /// Allows the use of SPIR-V 1.4
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_spirv_1_4.html
    KhrSpirV14,

    /// Simplifies the core synchronization API
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_synchronization2.html
    KhrSynchronization2,

    /// Adds new commands to create render passes
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VK_KHR_create_renderpass2.html
    KhrCreateRenderpass2,
}

impl VkExtension for VkDeviceExtensions {
    fn as_string(&self) -> String {
        match self {
            VkDeviceExtensions::KhrSwapchain => String::from("VK_KHR_swapchain"),
            VkDeviceExtensions::KhrSpirV14 => String::from("VK_KHR_spirv_1_4"),
            VkDeviceExtensions::KhrSynchronization2 => String::from("VK_KHR_synchronization2"),
            VkDeviceExtensions::KhrCreateRenderpass2 => String::from("VK_KHR_create_renderpass2"),
        }
    }
}
