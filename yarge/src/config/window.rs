use crate::maths::Vector2;

/// The configuration for the initial window
/// The window's position is such that:
/// [0.,0.] is the top left corner of the monitor
/// [1.,0.] is the top right corner of the monitor
/// [0.,1.] is the bottom left corner of the monitor
/// [1.,1.] is the bottom right corner of the monitor
pub struct WindowConfig {
    /// The window's title
    pub title: String,

    /// The window's position
    /// The position is such that
    /// `x` is the left of the window
    /// `y` is the top of the window
    /// The values are given between 0 and 1
    pub position: Vector2,

    /// The window's width
    /// The width must be between 0. and 1.
    /// 1 meaning the monitor's width
    pub width: f32,

    /// The window's height
    /// The height must be between 0. and 1.
    /// 1 meaning the monitor's height
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
