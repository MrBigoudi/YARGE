#![warn(missing_docs)]
#![feature(portable_simd)]
#![cfg_attr(bare_metal, no_std)]

//! The yarge library

pub mod config;
pub(crate) mod core_layer;

pub mod error;
pub mod maths;
pub(crate) mod platform_layer;
pub(crate) mod rendering_layer;

pub use core_layer::{
    Component, ECS, Entry, FileResource, FileResourceTypeId, GLOBAL_LOGGER, Game, Query,
    RonFileResource, UserEntity as Entity, UserEventBuilder, gamepad, keyboard,
    logger_system::helpers::{LogLevel, LogTarget},
    mouse,
};

pub use platform_layer::{PlatformLayer, PlatformLayerImpl};

pub use macros::{Component, FileResource, RonFileResource};
