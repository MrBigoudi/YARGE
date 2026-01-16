#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::rendering_layer::types::RendererBeginFrameOutput;

use crate::{PlatformLayerImpl, RenderingLayer, config::Config};

use super::context::VulkanContext;

pub(crate) struct VulkanRenderingLayer<'a> {
    pub(crate) context: VulkanContext<'a>,
}

impl<'a> RenderingLayer<'a> for VulkanRenderingLayer<'a> {
    type RenderingLayerType = VulkanRenderingLayer<'a>;

    fn init(
        config: &Config,
        platform_layer: &mut PlatformLayerImpl,
    ) -> Result<Self::RenderingLayerType, ErrorType> {
        let context = match VulkanContext::init(config, platform_layer) {
            Ok(context) => context,
            Err(err) => {
                log_error!("Failed to initialize the Vulkan context: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };
        log_info!("Vulkan renderer initialized");
        Ok(VulkanRenderingLayer { context })
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        log_debug!("Vulkan renderer shutted down");
        Ok(())
    }

    fn begin_frame(&mut self) -> Result<RendererBeginFrameOutput, ErrorType> {
        log_error!("Function is not yet implemented");
        Err(ErrorType::NotImplemented)
    }

    fn end_frame(&mut self, _platform_layer: &mut PlatformLayerImpl) -> Result<(), ErrorType> {
        log_error!("Function is not yet implemented");
        Err(ErrorType::NotImplemented)
    }
}
