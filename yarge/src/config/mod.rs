//! Module defining all the configuration related stuff for the engine
#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

mod window;
pub use window::WindowConfig;

mod logger;
pub use logger::LoggerConfig;

mod renderer;
pub use renderer::RendererConfig;

mod application;
pub use application::ApplicationConfig;

mod init;
pub(crate) use init::Config;

mod helper;
pub use helper::Version;
