//! Module defining all the configuration related stuff for the engine
#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

mod window;
pub(crate) use window::WindowConfig;

mod logger;
pub(crate) use logger::LoggerConfig;

mod renderer;
pub(crate) use renderer::RendererConfig;

mod application;
pub(crate) use application::ApplicationConfig;

mod init;
pub(crate) use init::Config;

mod helper;
pub use helper::Version;
