//! The core layer of the engine

mod application_system;
pub use application_system::{ApplicationSystem, Game};

mod logger_system;
pub use logger_system::LoggerSystem;
pub use logger_system::helpers;
pub use logger_system::{log_debug, log_error, log_info, log_warn};

mod entry;
pub use entry::Entry;

/// A module that handles different input devices
mod input_system;
pub use input_system::{gamepad, keyboard, mouse};

mod core;
