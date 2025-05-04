use crate::{log_debug, log, log_error, config::Config, error::ErrorType, rendering_layer::renderer::RendereringLayer};

use super::context::VulkanContext;

pub struct VulkanRenderingLayer {
    pub context: VulkanContext,
}

impl RendereringLayer for VulkanRenderingLayer {
    type RendereringLayerType = VulkanRenderingLayer;

    fn init(config: &Config) -> Result<Self::RendereringLayerType, ErrorType> {
        let context = match VulkanContext::new(config) {
            Ok(context) => context,
            Err(err) => {
                log_error!("Failed to initialize the vulkan context: {:?}", err);
                return Err(err);
            }
        };
        log_debug!("Vulkan renderer initialized");
        Ok(VulkanRenderingLayer {
            context,
        })
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        log_debug!("Vulkan renderer shutted down");
        Ok(())
    }
}