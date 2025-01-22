#![warn(missing_docs)]

//! Platform layer: abstraction layer that interacts directly
//! with the underlying hardware and operating system

/// A module that abstracts a platform layer
mod platform;
pub use platform::PlatformLayer;

/// A module that abstracts a window manager
mod window;
pub use window::{DisplayMode, Window};

/// The concrete implementations
mod platform_impl;
pub use platform_impl::PlatformLayerImpl;
