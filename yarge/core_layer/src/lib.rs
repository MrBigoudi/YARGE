#![warn(missing_docs)]

//! The core layer of the engine

mod application_system;
pub use application_system::{ApplicationSystem, Game};

mod logger_system;
pub use logger_system::LoggerSystem;

mod entry;
pub use entry::Entry;

mod core;