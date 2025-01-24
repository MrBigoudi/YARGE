use config::Config;
use error::ErrorType;

use crate::{platform::PlatformLayer, window::Window};

use super::window::PsVitaWindow;

/// The platform structure for the ps-vita
pub struct PsVitaPlatformLayer {
    window: PsVitaWindow,
}

impl PlatformLayer for PsVitaPlatformLayer {
    type PlatformLayerType = PsVitaPlatformLayer;

    fn init(config: &Config) -> Result<Self::PlatformLayerType, ErrorType> {
        let window = match PsVitaWindow::init(config) {
            Ok(window) => window,
            Err(err) => {
                // TODO: add error message
                return Err(err);
            }
        };
        Ok(PsVitaPlatformLayer { window })
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        self.window.shutdown()?;
        Ok(())
    }

    fn get_window(&mut self, _window_id: u8) -> &mut impl Window {
        &mut self.window
    }
}
