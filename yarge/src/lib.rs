#![warn(missing_docs)]
#![feature(portable_simd)]

//! The yarge library

pub mod config;
pub(crate) mod core_layer;
pub mod error;
pub mod maths;
pub(crate) mod platform_layer;

pub use core_layer::Entry;
pub use core_layer::Game;
pub use core_layer::log_debug;
pub use core_layer::log_error;
pub use core_layer::log_info;
pub use core_layer::log_warn;

pub use core_layer::{gamepad, keyboard, mouse};
