use crate::renderer_types::FinalTransform;
#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

pub(crate) mod config;
pub(crate) mod extensions;
pub(crate) mod features;
pub(crate) mod formats;
pub(crate) mod layers;
pub(crate) mod usages;

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

impl FinalTransform {
    /// Gets the ash equivalent of a FinalTransform
    pub(crate) fn as_ash(&self) -> ash::vk::SurfaceTransformFlagsKHR {
        match self {
            FinalTransform::Automatic => ash::vk::SurfaceTransformFlagsKHR::INHERIT,
            FinalTransform::Identity => ash::vk::SurfaceTransformFlagsKHR::IDENTITY,
            FinalTransform::Rotate90Clockwise => ash::vk::SurfaceTransformFlagsKHR::ROTATE_90,
            FinalTransform::Rotate90CounterClockwise => {
                ash::vk::SurfaceTransformFlagsKHR::ROTATE_270
            }
            FinalTransform::Rotate180 => ash::vk::SurfaceTransformFlagsKHR::ROTATE_180,
            FinalTransform::HorizontalFlip => ash::vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR,
            FinalTransform::HorizontalFlipRotate90ClockWise => {
                ash::vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_90
            }
            FinalTransform::HorizontalFlipRotate90CounterClockWise => {
                ash::vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_270
            }
            FinalTransform::HorizontalFlipRotate180 => {
                ash::vk::SurfaceTransformFlagsKHR::HORIZONTAL_MIRROR_ROTATE_180
            }
        }
    }
}
