#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// Different image formats
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ImageFormat {
    // -------- 8-bit UNORM --------
    R8_UNORM,
    R8G8_UNORM,
    R8G8B8_UNORM,
    R8G8B8A8_UNORM,

    B8G8R8_UNORM,
    B8G8R8A8_UNORM,

    // -------- 8-bit SRGB --------
    R8G8B8_SRGB,
    R8G8B8A8_SRGB,
    B8G8R8_SRGB,
    B8G8R8A8_SRGB,

    // -------- 16-bit UNORM --------
    R16_UNORM,
    R16G16_UNORM,
    R16G16B16_UNORM,
    R16G16B16A16_UNORM,

    // -------- 16-bit SFLOAT --------
    R16_SFLOAT,
    R16G16_SFLOAT,
    R16G16B16_SFLOAT,
    R16G16B16A16_SFLOAT,

    // -------- 32-bit UNORM (actually UINT in Vulkan) --------
    R32_UNORM,
    R32G32_UNORM,
    R32G32B32_UNORM,
    R32G32B32A32_UNORM,

    // -------- 32-bit SFLOAT --------
    R32_SFLOAT,
    R32G32_SFLOAT,
    R32G32B32_SFLOAT,
    R32G32B32A32_SFLOAT,
}

impl ImageFormat {
    /// Size of a single channel in bits
    pub(crate) fn get_channel_size(&self) -> u8 {
        match self {
            // 8-bit
            ImageFormat::R8_UNORM
            | ImageFormat::R8G8_UNORM
            | ImageFormat::R8G8B8_UNORM
            | ImageFormat::R8G8B8A8_UNORM
            | ImageFormat::B8G8R8_UNORM
            | ImageFormat::B8G8R8A8_UNORM
            | ImageFormat::R8G8B8_SRGB
            | ImageFormat::R8G8B8A8_SRGB
            | ImageFormat::B8G8R8_SRGB
            | ImageFormat::B8G8R8A8_SRGB => 8u8,

            // 16-bit
            ImageFormat::R16_UNORM
            | ImageFormat::R16G16_UNORM
            | ImageFormat::R16G16B16_UNORM
            | ImageFormat::R16G16B16A16_UNORM
            | ImageFormat::R16_SFLOAT
            | ImageFormat::R16G16_SFLOAT
            | ImageFormat::R16G16B16_SFLOAT
            | ImageFormat::R16G16B16A16_SFLOAT => 16u8,

            // 32-bit
            ImageFormat::R32_UNORM
            | ImageFormat::R32G32_UNORM
            | ImageFormat::R32G32B32_UNORM
            | ImageFormat::R32G32B32A32_UNORM
            | ImageFormat::R32_SFLOAT
            | ImageFormat::R32G32_SFLOAT
            | ImageFormat::R32G32B32_SFLOAT
            | ImageFormat::R32G32B32A32_SFLOAT => 32u8,
        }
    }
}
