#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{PlatformLayerImpl, config::Config, rendering_layer::types::RendererBeginFrameOutput};

/// Abstract trait for the renderer backend specific code
pub(crate) trait RenderingLayer<'a> {
    /// The type of the struct implementing the trait
    /// This would often be `Self`
    type RenderingLayerType;

    /// Initializes the renderer backend
    fn init(
        config: &Config,
        platform_layer: &mut PlatformLayerImpl,
    ) -> Result<Self::RenderingLayerType, ErrorType>;

    /// Shuts down the renderer backend
    fn shutdown(&mut self) -> Result<(), ErrorType>;

    /// Prepares a frame for rendering
    /// Returns true if the
    /// TODO: add frame data as parameter
    fn begin_frame(&mut self) -> Result<RendererBeginFrameOutput, ErrorType>;

    /// Ends a frame just before rendering
    /// TODO: add frame data as parameter
    fn end_frame(&mut self, platform_layer: &mut PlatformLayerImpl) -> Result<(), ErrorType>;
}
