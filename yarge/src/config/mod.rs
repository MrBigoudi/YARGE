//! Module defining all the configuration related stuff for the engine

mod window;
pub use window::WindowConfig;

mod logger;
pub use logger::LoggerConfig;

mod renderer;
pub use renderer::RendererConfig;

mod application;
pub use application::ApplicationConfig;

mod init;
pub use init::Config;

mod helper;
pub use helper::Version;
