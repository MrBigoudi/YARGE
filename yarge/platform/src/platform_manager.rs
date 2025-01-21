use error::ErrorType;

/// Abstract trait for the platform specific code
pub trait PlatformManager {
    /// Initiate the internal structure of the platform
    fn init() -> Result<ErrorType, impl PlatformManager>;

    /// Shutdown the platform
    fn shutdown(&mut self) -> Result<(), ErrorType>;
}
