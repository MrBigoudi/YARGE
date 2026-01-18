#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{config::Version, rendering_layer::types::ImageFormat};

#[derive(Debug, Clone)]
/// The config for an OpenGl context
pub(crate) struct OpenglConfig {
    /// The opengl version
    pub(crate) version: Version,
    /// The framebuffer format
    pub(crate) framebuffer_format: ImageFormat,
    /// The depthbuffer format
    pub(crate) depthbuffer_format: Option<ImageFormat>,
    /// The stencilbuffer format
    pub(crate) stencilbuffer_format: Option<ImageFormat>,
}

impl Default for OpenglConfig {
    fn default() -> Self {
        let version = Version::default().major(4).minor(6);
        let framebuffer_format = ImageFormat::R8G8B8A8_SFLOAT;
        let depthbuffer_format = Some(ImageFormat::R8G8B8_SFLOAT);
        let stencilbuffer_format = Some(ImageFormat::R8_SFLOAT);

        Self {
            version,
            framebuffer_format,
            depthbuffer_format,
            stencilbuffer_format,
        }
    }
}
