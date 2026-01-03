#[allow(unused)]
use crate::{config::Config, error::ErrorType, log, log_debug, log_error, platform_layer::Event};
use crate::{
    platform_layer::PlatformLayerImpl,
    rendering_layer::{RenderingLayer, RenderingLayerImpl, types::RendererBeginFrameOutput},
};

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
            log_error!("The user game failed to start: {:?}", err);
            return Err(ErrorType::Unknown);
        }

        Ok(ApplicationSystem { user_game })
    }

    /// Event handling
    fn handle_event(&mut self, event: Event) -> Result<(), ErrorType> {
        match event {
            crate::platform_layer::Event::KeyboardKeyPressed(keyboard_key) => {
                if let Err(err) = self.user_game.on_keyboard_key_pressed(keyboard_key) {
                    log_debug!(
                        "Failed to handle a keyboard key pressed event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::KeyboardKeyReleased(keyboard_key) => {
                if let Err(err) = self.user_game.on_keyboard_key_released(keyboard_key) {
                    log_debug!(
                        "Failed to handle a keyboard key pressed event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::MouseButtonPressed(mouse_button) => {
                if let Err(err) = self.user_game.on_mouse_button_pressed(mouse_button) {
                    log_debug!(
                        "Failed to handle a mouse button pressed event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::MouseButtonReleased(mouse_button) => {
                if let Err(err) = self.user_game.on_mouse_button_released(mouse_button) {
                    log_debug!(
                        "Failed to handle a mouse button released event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::MouseScrolled(delta) => {
                if let Err(err) = self.user_game.on_mouse_scrolled(delta) {
                    log_debug!(
                        "Failed to handle a mouse scrolled event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::MouseMoved(new_x, new_y) => {
                if let Err(err) = self.user_game.on_mouse_moved(new_x, new_y) {
                    log_debug!(
                        "Failed to handle a mouse moved event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::MouseMovedAndButton(new_x, new_y, mouse_button) => {
                if let Err(err) =
                    self.user_game
                        .on_mouse_moved_and_button_pressed(new_x, new_y, mouse_button)
                {
                    log_debug!(
                        "Failed to handle a mouse moved while button pressed event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::MouseEnteredWindow(x, y) => {
                if let Err(err) = self.user_game.on_mouse_entered_window(x, y) {
                    log_debug!(
                        "Failed to handle a mouse entered the window event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::MouseLeftWindow(x, y) => {
                if let Err(err) = self.user_game.on_mouse_left_window(x, y) {
                    log_debug!(
                        "Failed to handle a mouse left the window event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::GamepadButtonPressed(gamepad_button) => {
                if let Err(err) = self.user_game.on_gamepad_button_pressed(gamepad_button) {
                    log_debug!(
                        "Failed to handle a gamepad button pressed event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::GamepadButtonReleased(gamepad_button) => {
                if let Err(err) = self.user_game.on_gamepad_button_released(gamepad_button) {
                    log_debug!(
                        "Failed to handle a gamepad button released event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::WindowResized(new_width, new_height) => {
                if let Err(err) = self.user_game.on_resize(new_width, new_height) {
                    log_debug!(
                        "Failed to handle a window resized event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::WindowMinimized => {
                if let Err(err) = self.user_game.on_window_minimized() {
                    log_debug!(
                        "Failed to handle a window minimized event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::WindowRestored => {
                if let Err(err) = self.user_game.on_window_resotred() {
                    log_debug!(
                        "Failed to handle a window restored event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::WindowFocused => {
                if let Err(err) = self.user_game.on_window_focused() {
                    log_debug!(
                        "Failed to handle a window focused event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::WindowUnfocused => {
                if let Err(err) = self.user_game.on_window_unfocused() {
                    log_debug!(
                        "Failed to handle a window unfocued event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            crate::platform_layer::Event::WindowClosed => {
                if let Err(err) = self.user_game.on_window_closed() {
                    log_debug!(
                        "Failed to handle a window closed event in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
            _ => {} //TODO: handle other events
        };
        Ok(())
    }

    /// One iteration of the infinite running loop
    pub fn loop_iteration(
        &mut self,
        event: Event,
        platform_layer: &mut PlatformLayerImpl,
        rendering_layer: &mut RenderingLayerImpl,
    ) -> Result<Option<Event>, ErrorType> {
        if let Err(err) = self.handle_event(event) {
            log_error!(
                "Failed to handle an event in the application layer: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        };

        // TODO: create rendering packet
        let delta_time = 0.;
        let user_event = match self.user_game.on_update(delta_time) {
            Ok(Some(Event::WindowClosed)) => Some(Event::WindowClosed),
            Err(err) => {
                log_error!(
                    "Failed to update the game in the application layer: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
            _ => None,
        };

        if event == Event::Expose {
            if let Err(err) = self.user_game.on_render(delta_time) {
                log_error!(
                    "Failed to render the game in the application layer: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
            match rendering_layer.begin_frame() {
                Err(err) => {
                    log_error!(
                        "Failed to begin a frame in the application layer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                },
                Ok(RendererBeginFrameOutput::Success) => {
                    if let Err(err) = rendering_layer.end_frame(platform_layer) {
                        log_error!("Failed to end a frame in the application layer: {:?}", err);
                        return Err(ErrorType::Unknown);
                    }
                },
                _ => {
                },
            }
        }

        Ok(user_event)
    }

    /// Shuts down the application
    pub fn shutdown(&mut self) -> Result<(), ErrorType> {
        // Shuts down the user's game
        if let Err(err) = self.user_game.on_shutdown() {
            log_error!("The user game failed to shutdown: {:?}", err);
            return Err(ErrorType::Unknown);
        }

        Ok(())
    }
}
