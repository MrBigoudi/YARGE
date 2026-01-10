//! Platform layer: abstraction layer that interacts directly
//! with the underlying hardware and operating system

/// A module that abstracts a platform layer
pub(crate) mod platform;

/// A module that abstracts a window manager
pub(crate) mod window;

/// The concrete implementations
pub(crate) mod platform_impl;

/// A module representing an event
pub(crate) mod event;
