//! The core layer of the engine

pub(crate) mod application_system;
pub use application_system::{
    ApplicationSystem, Component, ECS, Game, SystemSchedule, UserEntity, UserEventWrapper,
};

pub(crate) mod logger_system;
pub use logger_system::GLOBAL_LOGGER;
pub use logger_system::LoggerSystem;

pub(crate) mod entry;
pub use entry::Entry;

/// A module that handles different input devices
pub(crate) mod input_system;
pub use input_system::{gamepad, keyboard, mouse};

pub(crate) mod core;

pub(crate) mod file_system;
pub use file_system::{FileLoaderSystem, FileResource, FileResourceTypeId, RonFileResource};
