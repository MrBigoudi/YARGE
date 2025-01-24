use config::Config;
use error::ErrorType;

use crate::window::{DisplayMode, Window};

pub struct PsVitaWindow;

impl Window for PsVitaWindow {
    type WindowType = PsVitaWindow;

    fn init(_config: &Config) -> Result<Self::WindowType, ErrorType> {
        // TODO: Implement ps-vita specific code
        Ok(PsVitaWindow)
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        // TODO: Implement ps-vita specific code
        Ok(())
    }

    fn get_width(&self) -> f32 {
        // TODO: Implement ps-vita specific code
        todo!()
    }

    fn get_height(&self) -> f32 {
        // TODO: Implement ps-vita specific code
        todo!()
    }

    fn get_position(&self) -> maths::Vector2 {
        // TODO: Implement ps-vita specific code
        todo!()
    }

    fn get_dpi_factor(&self) -> f32 {
        // TODO: Implement ps-vita specific code
        todo!()
    }

    fn get_id(&self) -> u8 {
        // TODO: Implement ps-vita specific code
        todo!()
    }

    fn set_display_mode(&mut self, _mode: DisplayMode) -> Result<(), ErrorType> {
        // TODO: Implement ps-vita specific code
        todo!()
    }

    fn set_position(&mut self, _x: f32, _y: f32) -> Result<(), ErrorType> {
        // TODO: Implement ps-vita specific code
        todo!()
    }
}
