//! Contains the implementation for a Mouse device

/// The state of a mouse button
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MouseButtonState {
    /// When a mouse button is being pressed
    Pressed,
    /// When a mouse button is being released
    #[default]
    Released,
}

/// The different mouse buttons
/// By default, a mouse can only have 3 buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    /// The left button
    Left,
    /// The right button
    Right,
    /// The middle button
    /// This button is often the mouse wheel
    Middle,
}
