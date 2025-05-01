use crate::{config::Config, error::ErrorType, maths::Vector2};

use super::Event;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Tells how the window should be displayed
pub enum DisplayMode {
    /// Fullscreen mode
    Fullscreen,

    /// Minimized mode
    Minimized,

    /// Floating mode
    /// Can optionally give the window's x and y positions
    /// as well as the window's width and height
    /// # Examples
    /// ```
    /// use platform_layer::DisplayMode;
    ///
    /// // To center the window
    /// let width=0.5;  // width in [0.,1.]
    /// let height=0.5; // height in [0.,1.]
    /// let x=0.5-(width/2.); // x in [0.,1.]
    /// let y=0.5-(height/2.); // y in [0.,1.]
    /// let mode = DisplayMode::Floating(Some((x, y, width, height)));
    /// ```
    Floating(Option<(f32, f32, f32, f32)>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Common properties for any platform specific window implementations
pub struct WindowCommonProperties {
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

    /// The current display mode
    pub display_mode: DisplayMode,
}

/// Abstract trait for a window
/// The window's position is such that:
/// [0.,0.] is the top left corner of the monitor
/// [1.,0.] is the top right corner of the monitor
/// [0.,1.] is the bottom left corner of the monitor
/// [1.,1.] is the bottom right corner of the monitor
pub trait Window {
    /// The type of the struct implementing the trait
    /// This would often be `Self`
    type WindowType;

    /// Initializes the window
    fn init(config: &Config) -> Result<Self::WindowType, ErrorType>;

    /// Shuts down the window
    fn shutdown(&mut self) -> Result<(), ErrorType>;

    /// Gets the window's properties
    fn get_properties(&self) -> WindowCommonProperties;

    /// Poll the next event
    fn poll_event(&mut self) -> Result<Event, ErrorType>;
}
