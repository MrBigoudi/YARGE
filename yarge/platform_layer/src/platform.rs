use error::ErrorType;

use crate::window::Window;

/// Abstract trait for the platform specific code
pub trait PlatformLayer {
    /// Initializes the platform
    fn init() -> Result<impl PlatformLayer, ErrorType>;

    /// Shuts down the platform
    fn shutdown(&mut self) -> Result<(), ErrorType>;

    /// Accessor to a window
    fn get_window(&mut self, window_id: u8) -> &mut impl Window;
}
