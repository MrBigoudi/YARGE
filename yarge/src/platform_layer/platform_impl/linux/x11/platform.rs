use crate::{
    log_error,
    config::Config,
    error::ErrorType,
    platform_layer::{PlatformLayer, Window},
};

use super::window::LinuxX11Window;

/// The platform structure for Linux X11
pub struct LinuxX11PlatformLayer {
    window: LinuxX11Window,
}

impl PlatformLayer for LinuxX11PlatformLayer {
    type PlatformLayerType = LinuxX11PlatformLayer;

    fn init(config: &Config) -> Result<Self::PlatformLayerType, ErrorType> {
        let window = match LinuxX11Window::init(config) {
            Ok(window) => window,
            Err(err) => {
                log_error!("Failed to initialize the xcb window: {:?}", err);
                return Err(ErrorType::InitializationFailure);
            }
        };
        Ok(LinuxX11PlatformLayer { window })
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        if let Err(err) = self.window.shutdown() {
            log_error!("Failed to shut down the xcb window: {:?}", err);
            return Err(ErrorType::ShutDownFailure);
        }
        Ok(())
    }

    fn get_window(&mut self, _window_id: u8) -> &mut impl Window {
        &mut self.window
    }
}
