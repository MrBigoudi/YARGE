#![warn(missing_docs)]
#![feature(portable_simd)]

#![cfg_attr(bare_metal, no_std)]

//! The yarge library

pub mod config;
pub (crate) mod core_layer;

pub mod error;
pub mod maths;
pub mod platform_layer;
pub (crate) mod rendering_layer;

pub use core_layer::Entry;
pub use core_layer::Game;

pub use core_layer::{gamepad, keyboard, mouse, logger_system::helpers::{LogLevel, LogTarget}, GLOBAL_LOGGER};
