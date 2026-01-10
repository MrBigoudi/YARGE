#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::rendering_layer::types::RendererBeginFrameOutput;

use crate::{PlatformLayerImpl, RenderingLayer, config::Config};

use super::context::VulkanContext;

pub(crate) struct VulkanRenderingLayer {
    pub(crate) context: VulkanContext,
}

impl RenderingLayer for VulkanRenderingLayer {
    type RenderingLayerType = VulkanRenderingLayer;

    fn init(
        config: &Config,
        _platform_layer: &mut PlatformLayerImpl,
    ) -> Result<Self::RenderingLayerType, ErrorType> {
        let context = match VulkanContext::new(config) {
            Ok(context) => context,
            Err(err) => {
                log_error!("Failed to initialize the vulkan context: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };
        log_debug!("Vulkan renderer initialized");
        Ok(VulkanRenderingLayer { context })
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        log_debug!("Vulkan renderer shutted down");
        Ok(())
    }

    fn begin_frame(&mut self) -> Result<RendererBeginFrameOutput, ErrorType> {
        Err(ErrorType::NotImplemented)
    }

    fn end_frame(&mut self, _platform_layer: &mut PlatformLayerImpl) -> Result<(), ErrorType> {
        Err(ErrorType::NotImplemented)
    }
}
