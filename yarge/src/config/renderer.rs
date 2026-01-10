#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

#[cfg(opengl_renderer)]
use crate::rendering_layer::rendering_impl::OpenglConfig;

/// The configuration for the renderer
#[derive(Clone)]
pub struct RendererConfig {
    #[cfg(opengl_renderer)]
    /// The opengl specific parameters
    pub opengl_parameters: OpenglConfig,

    #[cfg(vulkan_renderer)]
    /// The required vulkan layers
    /// Default to only the validation layers
    pub vulkan_required_layers: Vec<crate::rendering_layer::rendering_impl::VkLayers>,
}

/// Default implementation for the renderer config
impl Default for RendererConfig {
    fn default() -> RendererConfig {
        #[cfg(vulkan_renderer)]
        let vulkan_required_layers =
            vec![crate::rendering_layer::rendering_impl::VkLayers::Validation];

        RendererConfig {
            #[cfg(opengl_renderer)]
            opengl_parameters: OpenglConfig::default(),

            #[cfg(vulkan_renderer)]
            vulkan_required_layers,
        }
    }
}
