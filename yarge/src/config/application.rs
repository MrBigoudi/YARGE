/// The application's version
#[derive(Default, Clone)]
pub struct ApplicationVersion {
    /// The variant
    pub variant: u32,
    /// The major
    pub major: u32,
    /// The minor
    pub minor: u32,
    /// The patch
    pub patch: u32,
}

/// The configuration for the application
#[derive(Default, Clone)]
pub struct ApplicationConfig {
    /// The application's name
    pub name: String,
    /// The application's version
    pub version: ApplicationVersion,
}
