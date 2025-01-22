use error::ErrorType;

use crate::{platform::PlatformLayer, window::Window};

use super::window::LinuxX11Window;

/// The platform structure for Linux X11
pub struct LinuxX11PlatformLayer {
    window: LinuxX11Window,
}

impl PlatformLayer for LinuxX11PlatformLayer {
    fn init() -> Result<impl PlatformLayer, ErrorType> {
        Ok(LinuxX11PlatformLayer {
            window: LinuxX11Window,
        })
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        todo!()
    }

    fn get_window(&mut self, _window_id: u8) -> &mut impl Window {
        &mut self.window
    }
}
