//! Unified format for events

use keyboard::Key;
use mouse::MouseButton;

use crate::maths::Vector2;

pub mod keyboard;
pub mod mouse;
pub mod window;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    /// Shuts the application down on the next frame
    ApplicationQuit,
    /// Keyboard key pressed
    KeyPressed { key: Key },
    /// Keyboard key released
    KeyReleased { key: Key },
    /// Mouse button pressed
    MouseButtonPressed { button: MouseButton },
    /// Mouse button released
    MouseButtonReleased { button: MouseButton },
    /// Mouse moved
    /// The new position should be between 0. and 1.
    MouseMoved { new_position: Vector2 },
    /// Mouse wheel moved
    /// The delta should be between -1. and 1.
    MouseWheel { z_delta: f32 },
    /// Resized/resolution changed
    /// The values must be between 0. and 1.
    Resized { new_width: f32, new_height: f32 },
}