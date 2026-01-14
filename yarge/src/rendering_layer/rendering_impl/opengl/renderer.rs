#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{
    PlatformLayer, PlatformLayerImpl, RenderingLayer,
    rendering_layer::types::RendererBeginFrameOutput,
};
use crate::{config::Config, platform_layer::window::Window};

use super::context::OpenglContext;

pub(crate) struct OpenglRenderingLayer<'a> {
    pub(crate) context: OpenglContext<'a>,
}

impl<'a> RenderingLayer<'a> for OpenglRenderingLayer<'a> {
    type RenderingLayerType = OpenglRenderingLayer<'a>;

    fn init(
        config: &Config,
        platform_layer: &mut PlatformLayerImpl,
    ) -> Result<Self::RenderingLayerType, ErrorType> {
        let context = match OpenglContext::new(config) {
            Ok(context) => context,
            Err(err) => {
                log_error!("Failed to initialize the OpenGL context: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };

        if let Err(err) = platform_layer.get_window(0).opengl_make_context_current() {
            log_error!(
                "Failed to make the context current when initializing OpenGL: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        };

        if let Err(err) = platform_layer.get_window(0).opengl_load_functions() {
            log_error!(
                "Failed to load functions when initializing OpenGL: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        };

        log_info!("OpenGL renderer initialized");
        Ok(OpenglRenderingLayer { context })
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        log_info!("OpenGL renderer shutted down");
        Ok(())
    }

    fn begin_frame(&mut self) -> Result<RendererBeginFrameOutput, ErrorType> {
        unsafe { gl::ClearColor(0.2, 0.4, 0.9, 1.0) };
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
        Ok(RendererBeginFrameOutput::Success)
    }

    fn end_frame(&mut self, platform_layer: &mut PlatformLayerImpl) -> Result<(), ErrorType> {
        if let Err(err) = platform_layer.get_window(0).opengl_swap_buffers() {
            log_error!(
                "Failed to swap the buffers when ending an OpenGL frame: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        Ok(())
    }
}
