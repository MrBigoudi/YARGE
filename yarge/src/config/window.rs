#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::maths::Vector2;

/// The configuration for the initial window
/// The window's position is such that:
/// [0.,0.] is the top left corner of the monitor
/// [1.,0.] is the top right corner of the monitor
/// [0.,1.] is the bottom left corner of the monitor
/// [1.,1.] is the bottom right corner of the monitor
pub(crate) struct WindowConfig {
    /// The window's title
    pub(crate) title: String,

    /// The window's position
    /// The position is such that
    /// `x` is the left of the window
    /// `y` is the top of the window
    /// The positions must be between [0., 0.] (left, top) and [1., 1.] (right, bottom)
    pub(crate) position: Vector2,

    /// The window's width
    /// The width must be between 0. (0) and 1. (screen width)
    pub(crate) width: f32,

    /// The window's height
    /// The height must be between 0. (0) and 1. (screen height)
    pub(crate) height: f32,

    /// The window's border width
    /// The width is in pixels
    pub(crate) border_width: u16,
}

impl Default for WindowConfig {
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
