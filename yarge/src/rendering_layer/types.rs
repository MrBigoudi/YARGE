#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

#[allow(non_camel_case_types, unused)]
/// Different image formats
#[derive(Debug, Clone)]
pub enum ImageFormat {
    R8_UNORM,
    R8_SFLOAT,
    R8G8_UNORM,
    R8G8_SFLOAT,
    R8G8B8_UNORM,
    R8G8B8_SFLOAT,
    R8G8B8A8_UNORM,
    R8G8B8A8_SFLOAT,

    R16_UNORM,
    R16_SFLOAT,
    R16G16_UNORM,
    R16G16_SFLOAT,
    R16G16B16_UNORM,
    R16G16B16_SFLOAT,
    R16G16B16A16_UNORM,
    R16G16B16A16_SFLOAT,

    R32_UNORM,
    R32_SFLOAT,
    R32G32_UNORM,
    R32G32_SFLOAT,
    R32G32B32_UNORM,
    R32G32B32_SFLOAT,
    R32G32B32A32_UNORM,
    R32G32B32A32_SFLOAT,
}

impl ImageFormat {
    /// Gets the size in number of bits of a single channel
    pub fn get_channel_size(&self) -> u8 {
        match self {
            ImageFormat::R8_UNORM => 8u8,
            ImageFormat::R8_SFLOAT => 8u8,
            ImageFormat::R8G8_UNORM => 8u8,
            ImageFormat::R8G8_SFLOAT => 8u8,
            ImageFormat::R8G8B8_UNORM => 8u8,
            ImageFormat::R8G8B8_SFLOAT => 8u8,
            ImageFormat::R8G8B8A8_UNORM => 8u8,
            ImageFormat::R8G8B8A8_SFLOAT => 8u8,
            ImageFormat::R16_UNORM => 16u8,
            ImageFormat::R16_SFLOAT => 16u8,
            ImageFormat::R16G16_UNORM => 16u8,
            ImageFormat::R16G16_SFLOAT => 16u8,
            ImageFormat::R16G16B16_UNORM => 16u8,
            ImageFormat::R16G16B16_SFLOAT => 16u8,
            ImageFormat::R16G16B16A16_UNORM => 16u8,
            ImageFormat::R16G16B16A16_SFLOAT => 16u8,
            ImageFormat::R32_UNORM => 32u8,
            ImageFormat::R32_SFLOAT => 32u8,
            ImageFormat::R32G32_UNORM => 32u8,
            ImageFormat::R32G32_SFLOAT => 32u8,
            ImageFormat::R32G32B32_UNORM => 32u8,
            ImageFormat::R32G32B32_SFLOAT => 32u8,
            ImageFormat::R32G32B32A32_UNORM => 32u8,
            ImageFormat::R32G32B32A32_SFLOAT => 32u8,
        }
    }
}

#[allow(unused)]
/// The possible output for the begin frame
pub(crate) enum RendererBeginFrameOutput {
    /// Can pursue to the end frame
    Success,

    /// Should not present the frame
    Failure,
}
