use crate::{
    config::Config,
    error::ErrorType,
    platform_layer::{
        PlatformLayer, Window, event::Event, keyboard::KeyboardKey, mouse::MouseButton,
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
        // TODO: implement X11 specific code
        static mut COUNTER: u32 = 0;
        unsafe { COUNTER += 1 };
        unsafe {
            match COUNTER {
                1 => Ok(Event::KeyboardKeyPressed(KeyboardKey::A)),
                2 => Ok(Event::KeyboardKeyReleased(KeyboardKey::B)),
                3 => Ok(Event::MouseButtonPressed(MouseButton::Left)),
                4 => Ok(Event::MouseButtonReleased(MouseButton::Right)),
                _ => Ok(Event::WindowClosed),
            }
        }
    }
}
