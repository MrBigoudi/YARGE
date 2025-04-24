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

/// A module representing an event
mod event;
pub use event::Event;

/// A module that handles different input devices
mod input;
pub use input::{gamepad, keyboard, mouse};
