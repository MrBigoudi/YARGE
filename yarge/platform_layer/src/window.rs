use config::Config;
use error::ErrorType;
use maths::Vector2;

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

    /// Gets the window's width
    /// The value is given between 0 and 1
    fn get_width(&self) -> f32;

    /// Gets the window's height
    /// The value is given between 0 and 1
    fn get_height(&self) -> f32;

    /// Gets the window's position
    /// The position is such that
    /// `x` is the left of the window
    /// `y` is the top of the window
    /// The value are given between 0 and 1
    fn get_position(&self) -> Vector2;

    /// Gets the Dots Per Inch (DPI) factor
    fn get_dpi_factor(&self) -> f32;

    /// Gets the ID of the window
    /// This is useful for multiple window handling
    fn get_id(&self) -> u8;

    /// Sets the window's display mode
    /// See [DisplayMode]
    /// Returns a [WrongArgument] error if `mode` is [DisplayMode::Floating]
    /// with values that are not between 0. and 1.
    fn set_display_mode(&mut self, mode: DisplayMode) -> Result<(), ErrorType>;

    /// Moves the window to a given position
    /// `x` is the left of the window
    /// `y` is the top of the window
    /// The values must be given between 0 and 1
    /// Returns a [WrongArgument] error if the given parameters are not between 0. and 1.
    fn set_position(&mut self, x: f32, y: f32) -> Result<(), ErrorType>;
}
