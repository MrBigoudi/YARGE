use error::ErrorType;

use crate::window::{DisplayMode, Window};

pub struct LinuxX11Window;

impl Window for LinuxX11Window {
    type WindowType = LinuxX11Window;

    fn init() -> Result<Self::WindowType, ErrorType> {
        Ok(LinuxX11Window)
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        todo!()
    }

    fn get_width(&self) -> f32 {
        todo!()
    }

    fn get_height(&self) -> f32 {
        todo!()
    }

    fn get_position(&self) -> maths::Vector2 {
        todo!()
    }

    fn get_dpi_factor(&self) -> f32 {
        todo!()
    }

    fn get_id(&self) -> u8 {
        todo!()
    }

    fn set_display_mode(&mut self, _mode: DisplayMode) -> Result<(), ErrorType> {
        todo!()
    }

    fn set_position(&mut self, _x: f32, _y: f32) -> Result<(), ErrorType> {
        todo!()
    }
}
