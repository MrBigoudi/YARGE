#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use std::collections::VecDeque;

use crate::{
    core_layer::application_system::application::ApplicationSystem, platform_layer::event::Event,
};

impl ApplicationSystem<'_> {
    /// Event handling
    pub(crate) fn handle_event(
        &mut self,
        event: Event,
    ) -> Result<VecDeque<super::user_events::UserEventWrapper>, ErrorType> {
        match event {
            Event::KeyboardKeyPressed(keyboard_key) => {
                match self.user_game.on_keyboard_key_pressed(keyboard_key) {
                    Ok(events) => Ok(events),
                    Err(err) => {
                        log_error!(
                            "Failed to handle a keyboard key pressed event in the application layer: {:?}",
                            err
                        );
                        Err(ErrorType::Unknown)
                    }
                }
            }
            Event::KeyboardKeyReleased(keyboard_key) => {
                match self.user_game.on_keyboard_key_released(keyboard_key) {
                    Ok(events) => Ok(events),
                    Err(err) => {
                        log_error!(
                            "Failed to handle a keyboard key pressed event in the application layer: {:?}",
                            err
                        );
                        Err(ErrorType::Unknown)
                    }
                }
            }
            Event::MouseButtonPressed(mouse_button) => {
                match self.user_game.on_mouse_button_pressed(mouse_button) {
                    Ok(events) => Ok(events),
                    Err(err) => {
                        log_error!(
                            "Failed to handle a mouse button pressed event in the application layer: {:?}",
                            err
                        );
                        Err(ErrorType::Unknown)
                    }
                }
            }
            Event::MouseButtonReleased(mouse_button) => {
                match self.user_game.on_mouse_button_released(mouse_button) {
                    Ok(events) => Ok(events),
                    Err(err) => {
                        log_error!(
                            "Failed to handle a mouse button released event in the application layer: {:?}",
                            err
                        );
                        Err(ErrorType::Unknown)
                    }
                }
            }
            Event::MouseScrolled(delta) => match self.user_game.on_mouse_scrolled(delta) {
                Ok(events) => Ok(events),
                Err(err) => {
                    log_error!(
                        "Failed to handle a mouse scrolled event in the application layer: {:?}",
                        err
                    );
                    Err(ErrorType::Unknown)
                }
            },
            Event::MouseMoved(new_x, new_y) => match self.user_game.on_mouse_moved(new_x, new_y) {
                Ok(events) => Ok(events),
                Err(err) => {
                    log_error!(
                        "Failed to handle a mouse moved event in the application layer: {:?}",
                        err
                    );
                    Err(ErrorType::Unknown)
                }
            },
            Event::MouseMovedAndButton(new_x, new_y, mouse_button) => {
                match self
                    .user_game
                    .on_mouse_moved_and_button_pressed(new_x, new_y, mouse_button)
                {
                    Ok(events) => Ok(events),
                    Err(err) => {
                        log_error!(
                            "Failed to handle a mouse moved while button pressed event in the application layer: {:?}",
                            err
                        );
                        Err(ErrorType::Unknown)
                    }
                }
            }
            Event::MouseEnteredWindow(x, y) => match self.user_game.on_mouse_entered_window(x, y) {
                Ok(events) => Ok(events),
                Err(err) => {
                    log_error!(
                        "Failed to handle a mouse entered the window event in the application layer: {:?}",
                        err
                    );
                    Err(ErrorType::Unknown)
                }
            },
            Event::MouseLeftWindow(x, y) => match self.user_game.on_mouse_left_window(x, y) {
                Ok(events) => Ok(events),
                Err(err) => {
                    log_error!(
                        "Failed to handle a mouse left the window event in the application layer: {:?}",
                        err
                    );
                    Err(ErrorType::Unknown)
                }
            },
            Event::GamepadButtonPressed(gamepad_button) => {
                match self.user_game.on_gamepad_button_pressed(gamepad_button) {
                    Ok(events) => Ok(events),
                    Err(err) => {
                        log_error!(
                            "Failed to handle a gamepad button pressed event in the application layer: {:?}",
                            err
                        );
                        Err(ErrorType::Unknown)
                    }
                }
            }
            Event::GamepadButtonReleased(gamepad_button) => {
                match self.user_game.on_gamepad_button_released(gamepad_button) {
                    Ok(events) => Ok(events),
                    Err(err) => {
                        log_error!(
                            "Failed to handle a gamepad button released event in the application layer: {:?}",
                            err
                        );
                        Err(ErrorType::Unknown)
                    }
                }
            }
            Event::WindowResized(new_width, new_height) => {
                match self.user_game.on_resize(new_width, new_height) {
                    Ok(events) => Ok(events),
                    Err(err) => {
                        log_error!(
                            "Failed to handle a window resized event in the application layer: {:?}",
                            err
                        );
                        Err(ErrorType::Unknown)
                    }
                }
            }
            Event::WindowMinimized => match self.user_game.on_window_minimized() {
                Ok(events) => Ok(events),
                Err(err) => {
                    log_error!(
                        "Failed to handle a window minimized event in the application layer: {:?}",
                        err
                    );
                    Err(ErrorType::Unknown)
                }
            },
            Event::WindowRestored => match self.user_game.on_window_resotred() {
                Ok(events) => Ok(events),
                Err(err) => {
                    log_error!(
                        "Failed to handle a window restored event in the application layer: {:?}",
                        err
                    );
                    Err(ErrorType::Unknown)
                }
            },
            Event::WindowFocused => match self.user_game.on_window_focused() {
                Ok(events) => Ok(events),
                Err(err) => {
                    log_error!(
                        "Failed to handle a window focused event in the application layer: {:?}",
                        err
                    );
                    Err(ErrorType::Unknown)
                }
            },
            Event::WindowUnfocused => match self.user_game.on_window_unfocused() {
                Ok(events) => Ok(events),
                Err(err) => {
                    log_error!(
                        "Failed to handle a window unfocued event in the application layer: {:?}",
                        err
                    );
                    Err(ErrorType::Unknown)
                }
            },
            Event::WindowClosed => match self.user_game.on_window_closed() {
                Ok(events) => Ok(events),
                Err(err) => {
                    log_error!(
                        "Failed to handle a window closed event in the application layer: {:?}",
                        err
                    );
                    Err(ErrorType::Unknown)
                }
            },
            _ => Ok(VecDeque::new()), //TODO: handle other events
        }
    }
}
