//! The core layer of the engine

mod application_system;
pub use application_system::{ApplicationSystem, Game};

pub(crate) mod logger_system;
pub use logger_system::GLOBAL_LOGGER;
pub use logger_system::LoggerSystem;

mod entry;
pub use entry::Entry;

/// A module that handles different input devices
mod input_system;
pub use input_system::{gamepad, keyboard, mouse};

mod core;
