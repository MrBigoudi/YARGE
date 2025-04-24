use crate::{
    error::ErrorType,
    platform_layer::{gamepad::GamepadButton, keyboard::KeyboardKey, mouse::MouseButton},
};

/// The game trait that can be ovveride by the user
pub trait Game {
    /// Runs when the application starts
    /// Default behavior: don't do anything
    fn on_start(&mut self) -> Result<(), ErrorType> {
        // TODO: add debug message
        Ok(())
    }

    /// Runs each frame
    /// Default behavior: don't do anything
    fn on_update(&mut self, _delta_time: f64) -> Result<(), ErrorType> {
        // TODO: add debug message
        Ok(())
    }

    /// Runs each frame
    /// Default behavior: don't do anything
    fn on_render(&self, _delta_time: f64) -> Result<(), ErrorType> {
        // TODO: add debug message
        Ok(())
    }

    /// Runs everytime the user wants to resize the window
    /// Default behavior: don't do anything
    fn on_resize(&mut self, _new_width: f32, _new_height: f32) -> Result<(), ErrorType> {
        // TODO: add debug message
        Ok(())
    }

    /// Runs when the application stops
    /// Default behavior: don't do anything
    fn on_shutdown(&mut self) -> Result<(), ErrorType> {
        // TODO: add debug message
        Ok(())
    }

    /// Runs when a keyboard key is pressed
    /// Default behavior: don't do anything
    fn on_keyboard_key_pressed(&mut self, _keyboard_key: KeyboardKey) -> Result<(), ErrorType> {
        Ok(())
    }

    /// Runs when a keyboard key is released
    /// Default behavior: don't do anything
    fn on_keyboard_key_released(&mut self, _keyboard_key: KeyboardKey) -> Result<(), ErrorType> {
        Ok(())
    }

    /// Runs when a mouse button is pressed
    /// Default behavior: don't do anything
    fn on_mouse_button_pressed(&mut self, _mouse_button: MouseButton) -> Result<(), ErrorType> {
        Ok(())
    }
    /// Runs when a mouse button is released
    /// Default behavior: don't do anything
    fn on_mouse_button_released(&mut self, _mouse_button: MouseButton) -> Result<(), ErrorType> {
        Ok(())
    }
    /// Runs when the mouse is scrolled
    /// Default behavior: don't do anything
    fn on_mouse_scrolled(&mut self, _delta: f32) -> Result<(), ErrorType> {
        Ok(())
    }
    /// Runs when the mouse moved
    /// Default behavior: don't do anything
    fn on_mouse_moved(&mut self, _new_x: u16, _new_y: u16) -> Result<(), ErrorType> {
        Ok(())
    }
    /// Runs when a gamepad button is pressed
    /// Default behavior: don't do anything
    fn on_gamepad_button_pressed(
        &mut self,
        _gamepad_button: GamepadButton,
    ) -> Result<(), ErrorType> {
        Ok(())
    }
    /// Runs when a gamepad button is released
    /// Default behavior: don't do anything
    fn on_gamepad_button_released(
        &mut self,
        _gamepad_button: GamepadButton,
    ) -> Result<(), ErrorType> {
        Ok(())
    }
}
