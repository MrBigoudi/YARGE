#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

pub(crate) mod config;
pub(crate) mod extensions;
pub(crate) mod features;
pub(crate) mod layers;

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
