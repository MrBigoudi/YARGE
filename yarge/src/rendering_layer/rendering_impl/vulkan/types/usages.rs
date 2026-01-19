#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::renderer_types::usages::ImageUsage;

impl ImageUsage {
    /// Gets the ash equivalent of an ImageUsage
    pub(crate) fn as_ash(&self) -> ash::vk::ImageUsageFlags {
        match self {
            ImageUsage::TransferSrc => ash::vk::ImageUsageFlags::TRANSFER_SRC,
            ImageUsage::TransferDst => ash::vk::ImageUsageFlags::TRANSFER_DST,
            ImageUsage::Storage => ash::vk::ImageUsageFlags::STORAGE,
            ImageUsage::Sample => ash::vk::ImageUsageFlags::SAMPLED,
            ImageUsage::ColorAttachment => ash::vk::ImageUsageFlags::COLOR_ATTACHMENT,
            ImageUsage::DepthAttachment => ash::vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            ImageUsage::StencilAttachment => ash::vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
            ImageUsage::TransientAttachment => ash::vk::ImageUsageFlags::TRANSIENT_ATTACHMENT,
        }
    }
}
