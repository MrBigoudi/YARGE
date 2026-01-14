#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::config::Version;

/// Custrom structure for Vulkan names
pub(crate) struct VkNames {
    pub(crate) names_cstrings: Vec<std::ffi::CString>, // owns storage
    pub(crate) names: Vec<*const std::os::raw::c_char>, // pointers into cstrings
}

/// Creates Vulkan names from rust string
fn convert_string_to_vknames(names_string: &[String]) -> Result<VkNames, ErrorType> {
    let names_cstrings: Vec<std::ffi::CString> = names_string
        .iter()
        .map(|ext| {
            std::ffi::CString::new(ext.as_str()).map_err(|err| {
                log_error!(
                    "Failed to convert the string `{:?}` to a Vulkan name: {:?}",
                    ext,
                    err
                );
                ErrorType::IO
            })
        })
        .collect::<Result<_, _>>()?; // ← early return happens here

    let names: Vec<*const std::os::raw::c_char> =
        names_cstrings.iter().map(|ext| ext.as_ptr()).collect();

    Ok(VkNames {
        names_cstrings,
        names,
    })
}

/// Custom enum for Vulkan 1.1 features names
#[derive(Debug, Clone, Copy)]
pub(crate) enum VkFeatures11 {}

/// Custom enum for Vulkan 1.2 features names
#[derive(Debug, Clone, Copy)]
pub(crate) enum VkFeatures12 {}

/// Custom enum for Vulkan 1.3 features names
#[derive(Debug, Clone, Copy)]
pub(crate) enum VkFeatures13 {}

/// Custom enum for Vulkan layers names
#[derive(Debug, Clone, Copy)]
pub(crate) enum VkLayers {
    /// Khronos validation layers
    KhrValidation,
}

impl VkLayers {
    pub(crate) fn as_string(&self) -> String {
        match self {
            VkLayers::KhrValidation => String::from("VK_LAYER_KHRONOS_validation"),
        }
    }

    pub(crate) fn to_vknames(layers: &[Self]) -> Result<VkNames, ErrorType> {
        let layers_string: Vec<String> = layers.iter().map(|layer| layer.as_string()).collect();
        convert_string_to_vknames(&layers_string)
    }
}

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
        }
    }

    pub(crate) fn to_vknames(extensions: &[Self]) -> Result<VkNames, ErrorType> {
        let extensions_string: Vec<String> = extensions.iter().map(|layer| layer.as_string()).collect();
        convert_string_to_vknames(&extensions_string)
    }
}


#[derive(Debug, Clone)]
/// The config for a Vulkan context
pub(crate) struct VulkanConfig {
    /// The Vulkan version
    pub(crate) version: Version,
    /// The layers in normal mode
    pub(crate) required_layers: Vec<VkLayers>,
    /// The layers in debug mode
    pub(crate) required_layers_debug: Vec<VkLayers>,

}

impl Default for VulkanConfig {
    fn default() -> Self {
        let version = Version::default().major(1).minor(4);
        let required_layers = Vec::new();
        let required_layers_debug = vec![VkLayers::KhrValidation];

        Self {
            version,
            required_layers,
            required_layers_debug,
        }
    }
}
