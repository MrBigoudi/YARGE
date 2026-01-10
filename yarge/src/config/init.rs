#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use std::path::Path;

use super::{ApplicationConfig, LoggerConfig, RendererConfig, WindowConfig};

/// A structure containing all the engine configuration
#[derive(Default)]
pub(crate) struct Config {
    /// The window's configuration
    pub(crate) window_config: WindowConfig,

    /// The logger's configuration
    pub(crate) logger_config: LoggerConfig,

    /// The renderer's configuration
    pub(crate) renderer_config: RendererConfig,

    /// The application's configuration
    pub(crate) application_config: ApplicationConfig,
}

impl Config {
    /// Reads the config file to fill the Config struct
    pub(crate) fn init(config_file: Option<&Path>) -> Result<Self, ErrorType> {
        match config_file {
            Some(_file_path) => todo!(), // TODO: init config from file
            None => Ok(Config::default()),
        }
    }
}
