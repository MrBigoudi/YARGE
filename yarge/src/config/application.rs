use crate::config::Version;

/// The configuration for the application
#[derive(Default, Clone)]
pub struct ApplicationConfig {
    /// The application's name
    pub name: String,
    /// The application's version
    pub version: Version,
}
