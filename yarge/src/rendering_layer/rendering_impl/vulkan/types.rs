use crate::{error::ErrorType, log_error};

/// Custrom structure for vulkan names
pub struct VkNames {
    #[allow(dead_code)]
    pub names_cstrings: Vec<std::ffi::CString>, // owns storage
    pub names: Vec<*const std::os::raw::c_char>, // pointers into cstrings
}

/// Creates vulkan names from rust string
fn convert_string_to_vknames(names_string: &[String]) -> Result<VkNames, ErrorType> {
    let names_cstrings: Vec<std::ffi::CString> = names_string
        .iter()
        .map(|ext| {
            std::ffi::CString::new(ext.as_str()).map_err(|err| {
                log_error!(
                    "Failed to convert the string `{:?}` to a vulkan name: {:?}",
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

/// Custom enum for vulkan layers names
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum VkLayers {
    Validation,
}

/// Custom enum for vulkan 1.1 features names
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum VkFeatures11 {}

/// Custom enum for vulkan 1.2 features names
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum VkFeatures12 {}

/// Custom enum for vulkan 1.3 features names
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum VkFeatures13 {}

impl VkLayers {
    pub fn as_string(&self) -> String {
        match self {
            VkLayers::Validation => String::from("VK_LAYER_KHRONOS_validation"),
        }
    }

    pub fn to_vknames(layers: &[Self]) -> Result<VkNames, ErrorType> {
        let layers_string: Vec<String> = layers.iter().map(|layer| layer.as_string()).collect();
        convert_string_to_vknames(&layers_string)
    }
}
