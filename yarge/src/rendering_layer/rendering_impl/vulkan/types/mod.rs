#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

pub(crate) mod features;
pub(crate) mod extensions;
pub(crate) mod layers;

use crate::{config::Version, rendering_layer::rendering_impl::types::{extensions::VkInstanceExtensions, layers::VkLayers}};

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

#[derive(Debug, Clone)]
/// The config for a Vulkan context
pub(crate) struct VulkanConfig {
    /// The Vulkan version
    pub(crate) version: Version,
    /// The required layers in normal mode
    pub(crate) required_layers: Vec<VkLayers>,
    /// The required layers in debug mode
    pub(crate) required_layers_debug: Vec<VkLayers>,
    /// The required instance extensions in normal mode
    pub(crate) required_instance_extensions: Vec<VkInstanceExtensions>,
    /// The required instanceextensions in debug mode
    pub(crate) required_instance_extensions_debug: Vec<VkInstanceExtensions>,
}

impl Default for VulkanConfig {
    fn default() -> Self {
        let version = Version::default().major(1).minor(4);
        let required_layers = Vec::new();
        let required_layers_debug = vec![VkLayers::KhrValidation(vec![
            layers::VkValidationLayerSettings::ThreadSafety,
            layers::VkValidationLayerSettings::DisplayApplicationName,
            layers::VkValidationLayerSettings::BestPractices(vec![]),
            layers::VkValidationLayerSettings::Synchronization(vec![
                layers::VkValidationLayerSyncValSettings::ShaderAccessesHeuristic,
                layers::VkValidationLayerSyncValSettings::MessageExtraProperties,
            ]),
            layers::VkValidationLayerSettings::DebugPrintf(vec![
                layers::VkValidationLayerPrintfSettings::ToStdout,
                layers::VkValidationLayerPrintfSettings::Verbose,
            ]),
            layers::VkValidationLayerSettings::GpuAssistedValidation(vec![
                layers::VkValidationLayerGpuavSettings::SafeMode,
                layers::VkValidationLayerGpuavSettings::ForceOnRobustnessFeatures,
            ]),
        ])];

        let required_instance_extensions = vec![VkInstanceExtensions::KhrSurface];
        let required_instance_extensions_debug = vec![VkInstanceExtensions::ExtDebugUtils];

        Self {
            version,
            required_layers,
            required_layers_debug,
            required_instance_extensions,
            required_instance_extensions_debug,
        }
    }
}
