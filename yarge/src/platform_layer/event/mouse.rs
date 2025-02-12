/// The state of a mouse button
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum MouseButtonState {
    /// When a mouse button is being pressed
    Pressed,
    /// When a mouse button is being released
    #[default]
    Released,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}