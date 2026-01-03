use crate::log_info;
#[allow(unused)]
use crate::{
    config::Config,
    core_layer::logger_system::helpers::{LogLevel, LogTarget},
    error::ErrorType,
    log, log_error,
    platform_layer::{PlatformLayer, Window, event::Event},
};

use colored::Colorize;

use super::window::LinuxX11Window;

/// The platform structure for Linux X11
pub struct LinuxX11PlatformLayer {
    window: LinuxX11Window,
}

impl PlatformLayer for LinuxX11PlatformLayer {
    type PlatformLayerType = LinuxX11PlatformLayer;

    fn init(config: &Config) -> Result<Self::PlatformLayerType, ErrorType> {
        let window = match LinuxX11Window::init(config) {
            Ok(window) => window,
            Err(err) => {
                log_error!("Failed to init the X11 linux window: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };
        Ok(LinuxX11PlatformLayer { window })
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        self.window.shutdown()?;
        log_info!("Platform layer shutted down");
        Ok(())
    }

    fn get_window(&mut self, _window_id: u8) -> &mut impl Window {
        &mut self.window
    }

    fn poll_event(&mut self) -> Result<Event, ErrorType> {
        match self.window.poll_event() {
            Ok(event) => Ok(event),
            Err(err) => {
                log_error!(
                    "Failed to poll an event from the X11 linux platform layer: {:?}",
                    err
                );
                Err(ErrorType::Unknown)
            }
        }
    }

    fn get_time_since_unix_epoch() -> Result<u128, ErrorType> {
        let start = std::time::SystemTime::now();
        match start.duration_since(std::time::UNIX_EPOCH) {
            Err(err) => {
                log_error!("Failed to get the linux time {:?}", err);
                Err(ErrorType::Unknown)
            }
            Ok(duration) => Ok(duration.as_millis()),
        }
    }

    fn write(level: &LogLevel, message: &str, target: &LogTarget) -> Result<(), ErrorType> {
        match target {
            LogTarget::Console => print!("[{}]: {}", Self::format_level(level), message),
            LogTarget::ErrorConsole => eprint!("[{:?}]: {:?}", Self::format_level(level), message),
        };
        Ok(())
    }
}

impl LinuxX11PlatformLayer {
    /// Get the correct ANSI color given the logging level
    fn format_level(level: &LogLevel) -> String {
        match level {
            LogLevel::Info => "Info".green().to_string(),
            LogLevel::Debug => "Debug".yellow().to_string(),
            LogLevel::Warn => "Warn".truecolor(255, 165, 0).to_string(),
            LogLevel::Error => "Error".red().to_string(),
        }
    }
}
