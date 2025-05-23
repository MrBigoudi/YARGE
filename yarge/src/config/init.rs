use std::path::Path;

use crate::error::ErrorType;

use super::{LoggerConfig, WindowConfig};

/// A structure containing all the engine configuration
#[derive(Default)]
pub struct Config {
    /// The window's configuration
    pub window_config: WindowConfig,

    /// The logger's configuration
    pub logger_config: LoggerConfig,
}

impl Config {
    /// Reads the config file to fill the Config struct
    pub fn init(config_file: Option<&Path>) -> Result<Self, ErrorType> {
        match config_file {
            Some(file_path) => todo!(), // TODO: init config from file
            None => Ok(Config::default()),
        }
    }
}
