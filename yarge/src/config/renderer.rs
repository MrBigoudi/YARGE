/// The configuration for the renderer
#[derive(Clone)]
pub struct RendererConfig {
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
            #[cfg(vulkan_renderer)]
            vulkan_required_layers,
        }
    }
}
