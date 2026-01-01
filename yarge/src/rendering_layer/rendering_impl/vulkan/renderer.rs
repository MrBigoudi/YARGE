#[allow(unused)]
use crate::{
    config::Config, error::ErrorType, log, log_debug, log_error,
    rendering_layer::renderer::RendereringLayer,
};

use super::context::VulkanContext;

pub struct VulkanRenderingLayer {
    #[allow(unused)]
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
        Ok(VulkanRenderingLayer { context })
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        log_debug!("Vulkan renderer shutted down");
        Ok(())
    }
}
