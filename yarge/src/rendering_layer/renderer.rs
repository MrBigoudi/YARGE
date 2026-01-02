use crate::{config::Config, error::ErrorType, platform_layer::PlatformLayerImpl};

/// Abstract trait for the renderer backend specific code
pub trait RendereringLayer {
    /// The type of the struct implementing the trait
    /// This would often be `Self`
    type RendereringLayerType;

    /// Initializes the renderer backend
    fn init(
        config: &Config,
        platform_layer: &mut PlatformLayerImpl,
    ) -> Result<Self::RendereringLayerType, ErrorType>;

    /// Shuts down the renderer backend
    fn shutdown(&mut self) -> Result<(), ErrorType>;

    /// Prepares a frame for rendering
    /// TODO: add frame data as parameter
    fn begin_frame(&mut self) -> Result<(), ErrorType>;

    /// Ends a frame just before rendering
    /// TODO: add frame data as parameter
    fn end_frame(&mut self, platform_layer: &mut PlatformLayerImpl) -> Result<(), ErrorType>;
}
