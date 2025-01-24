#![warn(missing_docs)]

//! Crust defining all the configuration related stuff for the engine

mod window;
pub use window::WindowConfig;

mod logger;
pub use logger::{ConfigLogLevel, ConfigLogTarget, LoggerConfig};

mod config;
pub use config::Config;
