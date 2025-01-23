use config::Config;
use error::ErrorType;

use crate::window::Window;

/// Abstract trait for the platform specific code
pub trait PlatformLayer {
    /// The type of the struct implementing the trait
    /// This would often be `Self`
    type PlatformLayerType;

    /// Initializes the platform
    fn init(config: &Config) -> Result<Self::PlatformLayerType, ErrorType>;

    /// Shuts down the platform
    fn shutdown(&mut self) -> Result<(), ErrorType>;

    /// Accessor to a window
    fn get_window(&mut self, window_id: u8) -> &mut impl Window;
}
