use crate::{
    config::Config,
    core_layer::logger_system::helpers::{LogLevel, LogTarget},
    error::ErrorType,
};

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

    // Static method that are platform dependant
    /// Get the time in milliseconds ellapsed since the Unix epochs
    fn get_time_since_unix_epoch() -> Result<u128, ErrorType>;

    /// Write a logging message
    fn write(level: &LogLevel, message: &str, target: &LogTarget) -> Result<(), ErrorType>;
}
