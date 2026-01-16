#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::rendering_layer::rendering_impl::types::{VkNames, convert_string_to_vknames};

/// Custom enum for Vulkan layers names
#[derive(Debug, Clone)]
#[allow(unused)]
pub(crate) enum VkLayers {
    /// Khronos validation layers
    KhrValidation(Vec<VkValidationLayerSettings>),
}

impl VkLayers {
    /// Gets the layer's name as a String
    pub(crate) fn as_string(&self) -> String {
        match self {
            VkLayers::KhrValidation(_) => String::from("VK_LAYER_KHRONOS_validation"),
        }
    }

    /// Creates a VkNames from the layer
    pub(crate) fn to_vknames(layers: &[Self]) -> Result<VkNames, ErrorType> {
        let layers_string: Vec<String> = layers.iter().map(|layer| layer.as_string()).collect();
        convert_string_to_vknames(&layers_string)
    }

    /// Creates a layer settings create info from the given setting
    pub(crate) fn get_settings(&self) -> Result<Vec<VkLayerSettingWithData>, ErrorType> {
        match self {
            VkLayers::KhrValidation(settings) => {
                let mut owned_settings = Vec::new();
                for setting in settings {
                    owned_settings.extend(setting.as_setting_with_data()?);
                }
                Ok(owned_settings)
            }
        }
    }
}

/// Custom enum for Vulkan validation layers settings
#[derive(Debug, Clone)]
#[allow(unused)]
pub(crate) enum VkValidationLayerSettings {
    /// Checks threads
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#thread-safety
    ThreadSafety,
    /// Gives warnings when using legacy parts of the API
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#legacy-detection
    LegacyDetection,
    /// Checks for resource access conflicts from misused synchronization operations
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#synchronization
    Synchronization(Vec<VkValidationLayerSyncValSettings>),
    /// Enables DebugPrintf to display log messages from shaders
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#debug-printf
    DebugPrintf(Vec<VkValidationLayerPrintfSettings>),
    /// Enables validation that cannot be done by the CPU
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#gpu-assisted-validation
    GpuAssistedValidation(Vec<VkValidationLayerGpuavSettings>),
    /// Outputs warnings related to common misuse of the API which are not prohibited by the specification
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#best-practices
    BestPractices(Vec<VkValidationLayerBestPracticesSettings>),
    /// Displays the application in the validation messages
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#display-application-name
    DisplayApplicationName,
}

/// Custom enum for Vulkan validation layers synchronization settings
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub(crate) enum VkValidationLayerSyncValSettings {
    /// Takes into account memory accesses performed by the shader based on SPIR-V static analysis
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#shader-accesses-heuristic
    ShaderAccessesHeuristic,
    /// Appends a section of key-value properties to the error message
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#extra-properties
    MessageExtraProperties,
}

/// Custom enum for Vulkan validation layers debug printf settings
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub(crate) enum VkValidationLayerPrintfSettings {
    /// Enables redirection of Debug Printf messages from the debug callback to stdout
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#redirect-printf-messages-to-stdout
    ToStdout,
    /// Prints out more information for each message
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#printf-verbose
    Verbose,
}

/// Custom enum for Vulkan validation layers gpu assisted settings
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub(crate) enum VkValidationLayerGpuavSettings {
    /// Tries to prevent crashes
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#safe-mode
    SafeMode,
    /// Enables all possible robustness features for the app at device creation time
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#force-on-robustness-features
    ForceOnRobustnessFeatures,
    /// Selects which shaders to instrument
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#enable-instrumenting-shaders-selectively
    SelectInstrumentedShaders,
}

/// Custom enum for Vulkan validation layers best practices settings
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub(crate) enum VkValidationLayerBestPracticesSettings {
    /// Specific for Arm GPUs
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#arm-specific-best-practices
    Arm,
    /// Specific for Amd GPUs
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#amd-specific-best-practices
    Amd,
    /// Specific for Imagination GPUs
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#img-specific-best-practices
    Img,
    /// Specific for Nvidia GPUs
    /// https://vulkan.lunarg.com/doc/view/latest/windows/khronos_validation_layer.html#nvidia-specific-best-practices
    Nvidia,
}

/// Helper struct to hold owned names data
pub(crate) struct VkLayerSettingWithData {
    /// The name of the layer
    pub layer_name: std::ffi::CString,
    /// The name of the setting
    pub setting_name: std::ffi::CString,
    /// The value of the setting
    pub value: Vec<u8>,
}

impl VkLayerSettingWithData {
    /// Creates a new VkLayerSettingWithData for a boolean setting
    pub(crate) fn new_true(setting_name: &str) -> Result<Self, ErrorType> {
        let layer_name = &VkLayers::KhrValidation(Vec::new()).as_string();
        let layer_name =
            convert_string_to_vknames(&[layer_name.to_string()])?.names_cstrings[0].clone();
        let setting_name =
            convert_string_to_vknames(&[setting_name.to_string()])?.names_cstrings[0].clone();
        let value_vec = vec![true as u8];

        Ok(Self {
            layer_name,
            setting_name,
            value: value_vec,
        })
    }

    /// Creates a Vulkan layer setting from the current data
    pub(crate) fn as_vk_validation_setting_ext<'a>(&'a self) -> ash::vk::LayerSettingEXT<'a> {
        ash::vk::LayerSettingEXT::default()
            .layer_name(&self.layer_name)
            .setting_name(&self.setting_name)
            .values(&self.value)
            .ty(ash::vk::LayerSettingTypeEXT::BOOL32)
    }
}

impl VkValidationLayerSettings {
    /// Creates a list of VkLayerSettingWithData from the given setting
    pub(crate) fn as_setting_with_data(&self) -> Result<Vec<VkLayerSettingWithData>, ErrorType> {
        let mut results = Vec::new();

        match self {
            VkValidationLayerSettings::ThreadSafety => {
                results.push(VkLayerSettingWithData::new_true(&self.as_string())?);
            }
            VkValidationLayerSettings::LegacyDetection => {
                results.push(VkLayerSettingWithData::new_true(&self.as_string())?);
            }
            VkValidationLayerSettings::Synchronization(sub_settings) => {
                // First add all sub-settings
                for sub_setting in sub_settings {
                    results.push(sub_setting.as_setting_with_data()?);
                }
                // Then add the main enable setting
                results.push(VkLayerSettingWithData::new_true(&self.as_string())?);
            }
            VkValidationLayerSettings::DebugPrintf(sub_settings) => {
                // First add all sub-settings
                for sub_setting in sub_settings {
                    results.push(sub_setting.as_setting_with_data()?);
                }
                // Then add the main enable setting
                results.push(VkLayerSettingWithData::new_true(&self.as_string())?);
            }
            VkValidationLayerSettings::GpuAssistedValidation(sub_settings) => {
                // First add all sub-settings
                for sub_setting in sub_settings {
                    results.push(sub_setting.as_setting_with_data()?);
                }
                // Then add the main enable setting
                results.push(VkLayerSettingWithData::new_true(&self.as_string())?);
            }
            VkValidationLayerSettings::BestPractices(sub_settings) => {
                // First add all sub-settings
                for sub_setting in sub_settings {
                    results.push(sub_setting.as_setting_with_data()?);
                }
                // Then add the main enable setting
                results.push(VkLayerSettingWithData::new_true(&self.as_string())?);
            }
            VkValidationLayerSettings::DisplayApplicationName => {
                results.push(VkLayerSettingWithData::new_true(&self.as_string())?);
            }
        }

        Ok(results)
    }

    /// Gets the settings's name as a String
    pub(crate) fn as_string(&self) -> String {
        match self {
            VkValidationLayerSettings::ThreadSafety => String::from("thread_safety"),
            VkValidationLayerSettings::LegacyDetection => String::from("legacy_detection"),
            VkValidationLayerSettings::Synchronization(_) => String::from("validate_sync"),
            VkValidationLayerSettings::DebugPrintf(_) => String::from("printf_enable"),
            VkValidationLayerSettings::GpuAssistedValidation(_) => String::from("gpuav_enable"),
            VkValidationLayerSettings::BestPractices(_) => String::from("validate_best_practices"),
            VkValidationLayerSettings::DisplayApplicationName => {
                String::from("message_format_display_application_name")
            }
        }
    }
}

pub(crate) trait VkValidationLayerSubSetting {
    /// Creates a list of VkLayerSettingWithData from the given setting
    fn as_setting_with_data(&self) -> Result<VkLayerSettingWithData, ErrorType> {
        match VkLayerSettingWithData::new_true(&self.as_string()) {
            Ok(data) => Ok(data),
            Err(err) => {
                log_error!(
                    "Failed to create a VkLayerSettingWithData from a validation layer setting for setting `{:?}': {:?}",
                    self.as_string(),
                    err,
                );
                Err(ErrorType::Unknown)
            }
        }
    }

    /// Gets the settings's name as a String
    fn as_string(&self) -> String;
}

impl VkValidationLayerSubSetting for VkValidationLayerSyncValSettings {
    /// Gets the settings's name as a String
    fn as_string(&self) -> String {
        match self {
            VkValidationLayerSyncValSettings::ShaderAccessesHeuristic => {
                String::from("syncval_shader_accesses_heuristic")
            }
            VkValidationLayerSyncValSettings::MessageExtraProperties => {
                String::from("syncval_message_extra_properties")
            }
        }
    }
}

impl VkValidationLayerSubSetting for VkValidationLayerPrintfSettings {
    /// Gets the settings's name as a String
    fn as_string(&self) -> String {
        match self {
            VkValidationLayerPrintfSettings::ToStdout => String::from("printf_to_stdout"),
            VkValidationLayerPrintfSettings::Verbose => String::from("printf_verbose"),
        }
    }
}

impl VkValidationLayerSubSetting for VkValidationLayerGpuavSettings {
    /// Gets the settings's name as a String
    fn as_string(&self) -> String {
        match self {
            VkValidationLayerGpuavSettings::SafeMode => String::from("gpuav_safe_mode"),
            VkValidationLayerGpuavSettings::ForceOnRobustnessFeatures => {
                String::from("gpuav_force_on_robustness")
            }
            VkValidationLayerGpuavSettings::SelectInstrumentedShaders => {
                String::from("gpuav_select_instrumented_shaders")
            }
        }
    }
}

impl VkValidationLayerSubSetting for VkValidationLayerBestPracticesSettings {
    /// Gets the settings's name as a String
    fn as_string(&self) -> String {
        match self {
            VkValidationLayerBestPracticesSettings::Arm => {
                String::from("validate_best_practices_arm")
            }
            VkValidationLayerBestPracticesSettings::Amd => {
                String::from("validate_best_practices_amd")
            }
            VkValidationLayerBestPracticesSettings::Img => {
                String::from("validate_best_practices_img")
            }
            VkValidationLayerBestPracticesSettings::Nvidia => {
                String::from("validate_best_practices_nvidia")
            }
        }
    }
}
