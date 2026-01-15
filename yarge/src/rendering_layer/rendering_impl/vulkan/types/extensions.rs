#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::rendering_layer::rendering_impl::types::{VkNames, convert_string_to_vknames};

/// Custom enum for Vulkan Extensions
#[derive(Debug, Clone, Copy)]
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

impl VkInstanceExtensions {
    pub(crate) fn as_string(&self) -> String {
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

    pub(crate) fn to_vknames(extensions: &[Self]) -> Result<VkNames, ErrorType> {
        let extensions_string: Vec<String> =
            extensions.iter().map(|layer| layer.as_string()).collect();
        convert_string_to_vknames(&extensions_string)
    }
}
