use config::Config;
use error::ErrorType;

use crate::{platform::PlatformLayer, window::Window};

use super::window::LinuxX11Window;

/// The platform structure for Linux X11
pub struct LinuxX11PlatformLayer {
    window: LinuxX11Window,
}

impl PlatformLayer for LinuxX11PlatformLayer {
    type PlatformLayerType = LinuxX11PlatformLayer;

    fn init(config: &Config) -> Result<Self::PlatformLayerType, ErrorType> {
        let window = match LinuxX11Window::init(config){
            Ok(window) => window,
            Err(err) => {
                // TODO: add error message
                return Err(err);
            }
        };
        Ok(LinuxX11PlatformLayer {
            window,
        })
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        self.window.shutdown()?;
        Ok(())
    }

    fn get_window(&mut self, _window_id: u8) -> &mut impl Window {
        &mut self.window
    }
}
