use crate::{
    config::Config,
    error::ErrorType,
    maths::Vector2,
    platform_layer::{DisplayMode, Window},
};

pub struct LinuxX11Window {}

impl Window for LinuxX11Window {
    type WindowType = LinuxX11Window;

    fn init(_config: &Config) -> Result<Self::WindowType, ErrorType> {
        // TODO: Implement Linux X11 specific code
        Ok(LinuxX11Window {})
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        // TODO: Implement Linux X11 specific code
        Ok(())
    }

    fn get_width(&self) -> f32 {
        // TODO: Implement Linux X11 specific code
        todo!()
    }

    fn get_height(&self) -> f32 {
        // TODO: Implement Linux X11 specific code
        todo!()
    }

    fn get_position(&self) -> Vector2 {
        // TODO: Implement Linux X11 specific code
        todo!()
    }

    fn get_dpi_factor(&self) -> f32 {
        // TODO: Implement Linux X11 specific code
        todo!()
    }

    fn get_id(&self) -> u8 {
        // TODO: Implement Linux X11 specific code
        todo!()
    }

    fn set_display_mode(&mut self, _mode: DisplayMode) -> Result<(), ErrorType> {
        // TODO: Implement Linux X11 specific code
        todo!()
    }

    fn set_position(&mut self, _x: f32, _y: f32) -> Result<(), ErrorType> {
        // TODO: Implement Linux X11 specific code
        todo!()
    }
}
