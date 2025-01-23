/// The log levels to put in a config file
/// They should match the levels from the [core_layer] crate
#[derive(Debug, Default)]
pub enum ConfigLogLevel {
    /// To be used when displaying information
    #[default]
    Info,
    /// Only visible on debug mode
    Debug,
    /// Non fatal error messages
    Warn,
    /// Fatal error messages
    Fatal,
}

#[derive(Default)]
pub struct LoggerConfig {
    pub level: ConfigLogLevel,
}