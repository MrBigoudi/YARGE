use crate::{config::Config, error::ErrorType, log_debug, log_error, platform_layer::Event};

use super::Game;

/// The application system
pub struct ApplicationSystem<'a> {
    user_game: &'a mut dyn Game,
}

impl<'a> ApplicationSystem<'a> {
    /// Initializes the application
    pub fn init(user_game: &'a mut dyn Game, _config: &Config) -> Result<Self, ErrorType> {
        // Inits the user's game
        if let Err(err) = user_game.on_start() {
            log_error!("The user game failed to start");
            return Err(err);
        }

        Ok(ApplicationSystem { user_game })
    }

    /// Event handling
    fn handle_event(&mut self, event: Event) -> Result<(), ErrorType> {
        match event {
            crate::platform_layer::Event::KeyboardKeyPressed(keyboard_key) => {
                if let Err(err) = self.user_game.on_keyboard_key_pressed(keyboard_key) {
                    log_debug!(
                        "Failed to handle a keyboard key press in the application layer: {:?}",
                        err
                    );
                    return Err(err);
                }
            }
            crate::platform_layer::Event::KeyboardKeyReleased(keyboard_key) => {
                if let Err(err) = self.user_game.on_keyboard_key_released(keyboard_key) {
                    log_debug!(
                        "Failed to handle a keyboard key press in the application layer: {:?}",
                        err
                    );
                    return Err(err);
                }
            }
            crate::platform_layer::Event::MouseButtonPressed(mouse_button) => {
                if let Err(err) = self.user_game.on_mouse_button_pressed(mouse_button) {
                    log_debug!(
                        "Failed to handle a mouse button pressed in the application layer: {:?}",
                        err
                    );
                    return Err(err);
                }
            }
            crate::platform_layer::Event::MouseButtonReleased(mouse_button) => {
                if let Err(err) = self.user_game.on_mouse_button_released(mouse_button) {
                    log_debug!(
                        "Failed to handle a mouse button released in the application layer: {:?}",
                        err
                    );
                    return Err(err);
                }
            }
            crate::platform_layer::Event::MouseScrolled(delta) => {
                if let Err(err) = self.user_game.on_mouse_scrolled(delta) {
                    log_debug!(
                        "Failed to handle a mouse scrolled in the application layer: {:?}",
                        err
                    );
                    return Err(err);
                }
            }
            crate::platform_layer::Event::MouseMoved(new_x, new_y) => {
                if let Err(err) = self.user_game.on_mouse_moved(new_x, new_y) {
                    log_debug!(
                        "Failed to handle a mouse moved in the application layer: {:?}",
                        err
                    );
                    return Err(err);
                }
            }
            crate::platform_layer::Event::GamepadButtonPressed(gamepad_button) => {
                if let Err(err) = self.user_game.on_gamepad_button_pressed(gamepad_button) {
                    log_debug!(
                        "Failed to handle a gamepad button pressed in the application layer: {:?}",
                        err
                    );
                    return Err(err);
                }
            }
            crate::platform_layer::Event::GamepadButtonReleased(gamepad_button) => {
                if let Err(err) = self.user_game.on_gamepad_button_released(gamepad_button) {
                    log_debug!(
                        "Failed to handle a gamepad button released in the application layer: {:?}",
                        err
                    );
                    return Err(err);
                }
            }
            crate::platform_layer::Event::WindowResized(new_width, new_height) => {
                if let Err(err) = self.user_game.on_resize(new_width, new_height) {
                    log_debug!(
                        "Failed to handle a window resized in the application layer: {:?}",
                        err
                    );
                    return Err(err);
                }
            }
            _ => {}
        };
        Ok(())
    }

    /// One iteration of the infinite running loop
    pub fn loop_iteration(&mut self, event: Event) -> Result<(), ErrorType> {
        if let Err(err) = self.handle_event(event) {
            log_error!("Failed to handle an event in the application layer");
            return Err(err);
        };

        // TODO: delta time
        let delta_time = 0.;
        if let Err(err) = self.user_game.on_update(delta_time) {
            log_error!("Failed to update the game in the application layer");
            return Err(err);
        };

        if let Err(err) = self.user_game.on_render(delta_time) {
            log_error!("Failed to update the game in the application layer");
            return Err(err);
        };

        Ok(())
    }

    /// Shuts down the application
    pub fn shutdown(&mut self) -> Result<(), ErrorType> {
        // Shuts down the user's game
        if let Err(err) = self.user_game.on_shutdown() {
            log_error!("The user game failed to shutdown");
            return Err(err);
        }

        Ok(())
    }
}
