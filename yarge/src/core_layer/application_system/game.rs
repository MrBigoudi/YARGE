#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use std::collections::VecDeque;

use crate::{
    core_layer::application_system::events::user_events::UserEventWrapper, gamepad::GamepadButton,
    keyboard::KeyboardKey, mouse::MouseButton,
};

pub struct UnsafeGameCell {
    ptr: *mut dyn Game,
}
impl UnsafeGameCell {
    pub(crate) fn new(game: &mut dyn Game) -> Self {
        let ptr: *mut dyn Game = game;
        Self { ptr }
    }
    #[inline]
    pub(crate) unsafe fn get(&self) -> &dyn Game {
        unsafe { &(*self.ptr) }
    }
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub(crate) unsafe fn get_mut(&self) -> &mut dyn Game {
        unsafe { &mut (*self.ptr) }
    }
}

/// The game trait that can be ovveride by the user
pub trait Game: std::any::Any + 'static {
    /// Runs when the application starts
    /// Default behavior: don't do anything
    fn on_start(&mut self) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }

    /// Runs each frame
    /// Default behavior: don't do anything
    fn on_update(&mut self, _delta_time: f64) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }

    /// Runs each frame
    /// Default behavior: don't do anything
    fn on_render(&self, _delta_time: f64) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }

    /// Runs everytime the user wants to resize the window
    /// Default behavior: don't do anything
    fn on_resize(
        &mut self,
        _new_width: f32,
        _new_height: f32,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }

    /// Runs when the application stops
    /// Default behavior: don't do anything
    fn on_shutdown(&mut self) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }

    /// Runs when a keyboard key is pressed
    /// Default behavior: don't do anything
    fn on_keyboard_key_pressed(
        &mut self,
        _keyboard_key: KeyboardKey,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }

    /// Runs when a keyboard key is released
    /// Default behavior: don't do anything
    fn on_keyboard_key_released(
        &mut self,
        _keyboard_key: KeyboardKey,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }

    /// Runs when a mouse button is pressed
    /// Default behavior: don't do anything
    fn on_mouse_button_pressed(
        &mut self,
        _mouse_button: MouseButton,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }
    /// Runs when a mouse button is released
    /// Default behavior: don't do anything
    fn on_mouse_button_released(
        &mut self,
        _mouse_button: MouseButton,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }
    /// Runs when the mouse is scrolled
    /// Default behavior: don't do anything
    fn on_mouse_scrolled(&mut self, _delta: f32) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }
    /// Runs when the mouse moves
    /// Default behavior: don't do anything
    fn on_mouse_moved(
        &mut self,
        _new_x: u16,
        _new_y: u16,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }
    /// Runs when the mouse moves while a mouse button is being pressed
    /// Default behavior: don't do anything
    fn on_mouse_moved_and_button_pressed(
        &mut self,
        _new_x: u16,
        _new_y: u16,
        _mouse_button: MouseButton,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }
    /// Runs when the mouse enters the window
    /// Default behavior: don't do anything
    fn on_mouse_entered_window(
        &mut self,
        _x: u16,
        _y: u16,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }
    /// Runs when the mouse leaves the window
    /// Default behavior: don't do anything
    fn on_mouse_left_window(
        &mut self,
        _x: u16,
        _y: u16,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }

    /// Runs when a gamepad button is pressed
    /// Default behavior: don't do anything
    fn on_gamepad_button_pressed(
        &mut self,
        _gamepad_button: GamepadButton,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }
    /// Runs when a gamepad button is released
    /// Default behavior: don't do anything
    fn on_gamepad_button_released(
        &mut self,
        _gamepad_button: GamepadButton,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }

    /// Runs when the window is miminized
    /// Default behavior: don't do anything
    fn on_window_minimized(&mut self) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }
    /// Runs when the window is restored
    /// Default behavior: don't do anything
    fn on_window_resotred(&mut self) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }
    /// Runs when the window gains focus
    /// Default behavior: don't do anything
    fn on_window_focused(&mut self) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }
    /// Runs when the window looses focus
    /// Default behavior: don't do anything
    fn on_window_unfocused(&mut self) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }

    /// Runs when the window is closed
    /// Default behavior: don't do anything
    fn on_window_closed(&mut self) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }

    /// Runs when a given resource is done loading
    /// Default behavior: don't do anything
    fn on_resource_loaded(
        &mut self,
        _resource_id: &crate::ResourceId,
        _handler: crate::ResourceHandle,
    ) -> Result<VecDeque<UserEventWrapper>, ErrorType> {
        Ok(VecDeque::new())
    }
}
