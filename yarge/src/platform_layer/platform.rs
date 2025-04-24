use crate::{config::Config, error::ErrorType};

use super::{Window, event::Event};

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

    /// Poll the next event
    fn poll_event(&mut self) -> Result<Event, ErrorType>;
}
