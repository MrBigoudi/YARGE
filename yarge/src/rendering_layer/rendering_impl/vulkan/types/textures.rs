#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

pub(crate) struct VkImageLayoutTransition {
    pub(crate) src_layout: ash::vk::ImageLayout,
    pub(crate) dst_layout: ash::vk::ImageLayout,
    pub(crate) src_access_mask: ash::vk::AccessFlags2,
    pub(crate) dst_access_mask: ash::vk::AccessFlags2,
    pub(crate) src_stage_mask: ash::vk::PipelineStageFlags2,
    pub(crate) dst_stage_mask: ash::vk::PipelineStageFlags2,
    pub(crate) subresource_range: ash::vk::ImageSubresourceRange,
}

impl Default for VkImageLayoutTransition {
    fn default() -> Self {
        Self {
            src_layout: Default::default(),
            dst_layout: Default::default(),
            src_access_mask: Default::default(),
            dst_access_mask: Default::default(),
            src_stage_mask: Default::default(),
            dst_stage_mask: Default::default(),
            subresource_range: ash::vk::ImageSubresourceRange::default()
                .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1),
        }
    }
}

impl VkImageLayoutTransition {
    pub(crate) fn src_layout(mut self, layout: ash::vk::ImageLayout) -> Self {
        self.src_layout = layout;
        self
    }
    pub(crate) fn dst_layout(mut self, layout: ash::vk::ImageLayout) -> Self {
        self.dst_layout = layout;
        self
    }
    pub(crate) fn src_access_mask(mut self, mask: ash::vk::AccessFlags2) -> Self {
        self.src_access_mask = mask;
        self
    }
    pub(crate) fn dst_access_mask(mut self, mask: ash::vk::AccessFlags2) -> Self {
        self.dst_access_mask = mask;
        self
    }
    pub(crate) fn src_stage_mask(mut self, mask: ash::vk::PipelineStageFlags2) -> Self {
        self.src_stage_mask = mask;
        self
    }
    pub(crate) fn dst_stage_mask(mut self, mask: ash::vk::PipelineStageFlags2) -> Self {
        self.dst_stage_mask = mask;
        self
    }
    pub(crate) fn subresource_range(mut self, range: ash::vk::ImageSubresourceRange) -> Self {
        self.subresource_range = range;
        self
    }
}
