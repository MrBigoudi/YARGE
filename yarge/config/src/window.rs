use maths::Vector2;

/// The configuration for the initial window
pub struct WindowConfig {
    /// The window's title
    pub title: String,

    /// The window's position
    /// The position must be between 0. and 1.
    pub position: Vector2,

    /// The window's width
    /// The width must be between 0. and 1.
    pub width: f32,

    /// The window's height
    /// The height must be between 0. and 1.
    pub height: f32,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: String::from("NewWindow"),
            position: Vector2::ZEROS,
            width: 1.,
            height: 1.,
        }
    }
}
