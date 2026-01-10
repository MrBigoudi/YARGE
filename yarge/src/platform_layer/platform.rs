#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{LogLevel, LogTarget, config::Config};

use super::{event::Event, window::Window};

#[allow(private_interfaces)]
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
    fn write_log(level: &LogLevel, message: &str, target: &LogTarget) -> Result<(), ErrorType>;
    /// Flush the logging output
    fn flush_log() -> Result<(), ErrorType>;

    /// Load a file into a string
    fn load_to_string(path: &std::path::Path) -> Result<String, ErrorType> {
        // Default implementation
        // TODO: add implementation to the platform layer
        match std::fs::read_to_string(path) {
            Ok(content) => Ok(content),
            Err(err) => {
                log_error!("Failed to load the file `{:?}': {:?}", path, err);
                Err(ErrorType::IO)
            }
        }
    }
}
