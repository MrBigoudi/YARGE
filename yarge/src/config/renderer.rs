#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

#[cfg(opengl_renderer)]
use crate::rendering_layer::rendering_impl::OpenglConfig;

#[cfg(vulkan_renderer)]
use crate::rendering_layer::rendering_impl::types::VulkanConfig;

/// The configuration for the renderer
#[derive(Clone)]
pub(crate) struct RendererConfig {
    #[cfg(opengl_renderer)]
    /// The opengl specific parameters
    pub(crate) opengl_parameters: OpenglConfig,

    #[cfg(vulkan_renderer)]
    /// The vulkan specific parameters
    pub(crate) vulkan_parameters: VulkanConfig,
}

/// Default implementation for the renderer config
impl Default for RendererConfig {
    fn default() -> RendererConfig {
        RendererConfig {
            #[cfg(opengl_renderer)]
            opengl_parameters: OpenglConfig::default(),

            #[cfg(vulkan_renderer)]
            vulkan_parameters: VulkanConfig::default(),
        }
    }
}
