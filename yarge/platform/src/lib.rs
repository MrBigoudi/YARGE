#![warn(missing_docs)]

//! Platform layer: abstraction layer that interacts directly
//! with the underlying hardware and operating system

/// A module that abstracts a platform
pub mod platform_manager;

/// A module that abstracts a window
pub mod window;

/// A module that abstracts logs
pub mod log;
