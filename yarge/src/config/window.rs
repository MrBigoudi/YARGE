use crate::maths::Vector2;

/// The configuration for the initial window
#[derive(Clone)]
pub struct WindowConfig {
    /// The window's title
    pub title: String,

    /// The window's position
    /// The positions must be between [0., 0.] (left, top) and [1., 1.] (right, bottom)
    pub position: Vector2,

    /// The window's width
    /// The width must be between 0. (0) and 1. (screen width)
    pub width: f32,

    /// The window's height
    /// The height must be between 0. (0) and 1. (screen height)
    pub height: f32,

    /// The window's border width
    /// The width is in pixels
    pub border_width: u16,
}

impl Default for WindowConfig {
    /// The default window is the size of the screen
    fn default() -> Self {
        Self {
            title: String::from("NewWindow"),
            position: Vector2::ZEROS,
            width: 1.,
            height: 1.,
            border_width: 4,
        }
    }
}
