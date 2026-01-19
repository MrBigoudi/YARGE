#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::renderer_types::{formats::ImageFormat, present::PresentMode};

impl ImageFormat {
    /// Gets the ash equivalent of an ImageFormat
    pub(crate) fn as_ash(&self) -> ash::vk::Format {
        match self {
            // -------- 8-bit UNORM --------
            ImageFormat::R8_UNORM => ash::vk::Format::R8_UNORM,
            ImageFormat::R8G8_UNORM => ash::vk::Format::R8G8_UNORM,
            ImageFormat::R8G8B8_UNORM => ash::vk::Format::R8G8B8_UNORM,
            ImageFormat::R8G8B8A8_UNORM => ash::vk::Format::R8G8B8A8_UNORM,
            ImageFormat::B8G8R8_UNORM => ash::vk::Format::B8G8R8_UNORM,
            ImageFormat::B8G8R8A8_UNORM => ash::vk::Format::B8G8R8A8_UNORM,

            // -------- 8-bit SRGB --------
            ImageFormat::R8G8B8_SRGB => ash::vk::Format::R8G8B8_SRGB,
            ImageFormat::R8G8B8A8_SRGB => ash::vk::Format::R8G8B8A8_SRGB,
            ImageFormat::B8G8R8_SRGB => ash::vk::Format::B8G8R8_SRGB,
            ImageFormat::B8G8R8A8_SRGB => ash::vk::Format::B8G8R8A8_SRGB,

            // -------- 16-bit UNORM --------
            ImageFormat::R16_UNORM => ash::vk::Format::R16_UNORM,
            ImageFormat::R16G16_UNORM => ash::vk::Format::R16G16_UNORM,
            ImageFormat::R16G16B16_UNORM => ash::vk::Format::R16G16B16_UNORM,
            ImageFormat::R16G16B16A16_UNORM => ash::vk::Format::R16G16B16A16_UNORM,

            // -------- 16-bit SFLOAT --------
            ImageFormat::R16_SFLOAT => ash::vk::Format::R16_SFLOAT,
            ImageFormat::R16G16_SFLOAT => ash::vk::Format::R16G16_SFLOAT,
            ImageFormat::R16G16B16_SFLOAT => ash::vk::Format::R16G16B16_SFLOAT,
            ImageFormat::R16G16B16A16_SFLOAT => ash::vk::Format::R16G16B16A16_SFLOAT,

            // -------- 32-bit UNORM (UINT in Vulkan) --------
            ImageFormat::R32_UNORM => ash::vk::Format::R32_UINT,
            ImageFormat::R32G32_UNORM => ash::vk::Format::R32G32_UINT,
            ImageFormat::R32G32B32_UNORM => ash::vk::Format::R32G32B32_UINT,
            ImageFormat::R32G32B32A32_UNORM => ash::vk::Format::R32G32B32A32_UINT,

            // -------- 32-bit SFLOAT --------
            ImageFormat::R32_SFLOAT => ash::vk::Format::R32_SFLOAT,
            ImageFormat::R32G32_SFLOAT => ash::vk::Format::R32G32_SFLOAT,
            ImageFormat::R32G32B32_SFLOAT => ash::vk::Format::R32G32B32_SFLOAT,
            ImageFormat::R32G32B32A32_SFLOAT => ash::vk::Format::R32G32B32A32_SFLOAT,
        }
    }
}

impl PresentMode {
    /// Gets the ash equivalent of an PresentMode
    pub(crate) fn as_ash(&self) -> ash::vk::PresentModeKHR {
        match self {
            PresentMode::Immediate => ash::vk::PresentModeKHR::IMMEDIATE,
            PresentMode::Vsync => ash::vk::PresentModeKHR::FIFO,
            PresentMode::TripleBuffering => ash::vk::PresentModeKHR::MAILBOX,
        }
    }
}
