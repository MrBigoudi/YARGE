#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::renderer_types::{FinalTransform, RenderingApplicationType};

#[cfg(opengl_renderer)]
use crate::rendering_layer::rendering_impl::OpenglConfig;

#[cfg(vulkan_renderer)]
use crate::rendering_layer::rendering_impl::types::config::VulkanConfig;

/// The configuration for the renderer
#[derive(Clone)]
pub(crate) struct RendererConfig {
    #[cfg(opengl_renderer)]
    /// The opengl specific parameters
    pub(crate) opengl_parameters: OpenglConfig,

    #[cfg(vulkan_renderer)]
    /// The Vulkan specific parameters
    pub(crate) vulkan_parameters: VulkanConfig,

    /// The application type
    pub(crate) application_type: RenderingApplicationType,
    /// The final transform applied to the image before presentation
    pub(crate) final_transform: FinalTransform,
}

/// Default implementation for the renderer config
impl Default for RendererConfig {
    fn default() -> RendererConfig {
        RendererConfig {
            #[cfg(opengl_renderer)]
            opengl_parameters: OpenglConfig::default(),

            #[cfg(vulkan_renderer)]
            vulkan_parameters: VulkanConfig::default(),

            application_type: RenderingApplicationType::default(),
            final_transform: FinalTransform::default(),
        }
    }
}
