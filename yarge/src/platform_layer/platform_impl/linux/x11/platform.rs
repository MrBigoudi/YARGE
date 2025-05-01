use crate::{
    config::Config,
    error::ErrorType,
    log_error,
    platform_layer::{
        PlatformLayer, Window, event::Event,
    },
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
                // TODO: add error message
                return Err(err);
            }
        };
        Ok(LinuxX11PlatformLayer { window })
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        self.window.shutdown()?;
        Ok(())
    }

    fn get_window(&mut self, _window_id: u8) -> &mut impl Window {
        &mut self.window
    }

    fn poll_event(&mut self) -> Result<Event, ErrorType> {
        match self.window.poll_event(){
            Ok(event) => Ok(event),
            Err(err) => {
                log_error!("Failed to poll an event from the X11 linux platform layer: {:?}", err);
                Err(ErrorType::Unknown)
            },
        }
    }
    
    fn get_time_since_unix_epoch() -> Result<u128, ErrorType> {
        let start = std::time::SystemTime::now();
        match start.duration_since(std::time::UNIX_EPOCH){
            Err(err) => {
                log_error!("Failed to get the linux time {:?}", err);
                return Err(ErrorType::Unknown);
            },
            Ok(duration) => Ok(duration.as_millis())
        }
    }
}
