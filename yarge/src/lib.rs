#![warn(absolute_paths_not_starting_with_crate)]
#![warn(elided_lifetimes_in_paths)]
#![warn(explicit_outlives_requirements)]
#![warn(keyword_idents)]
#![warn(macro_use_extern_crate)]
#![warn(meta_variable_misuse)]
#![warn(missing_abi)]
// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![warn(non_ascii_idents)]
#![warn(noop_method_call)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unsafe_op_in_unsafe_fn)]
#![warn(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_macro_rules)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![feature(portable_simd)]
#![cfg_attr(bare_metal, no_std)]

//! The yarge library

pub(crate) mod core_layer;
pub(crate) mod platform_layer;
pub(crate) mod rendering_layer;

pub mod config;
pub mod error;
pub mod maths;

pub use core_layer::application_system::game::Game;
pub use core_layer::entry::Entry;

pub use core_layer::application_system::ecs::ECS;
pub use core_layer::application_system::ecs::resource::ResourceHandle;
pub use core_layer::application_system::ecs::resource::UserResource as Resource;
pub use core_layer::application_system::ecs::resource::UserResourceId as ResourceId;
pub use core_layer::application_system::ecs::resource::UserResourceLoadingParameters as ResourceLoadingParameters;

pub use core_layer::application_system::ecs::component::UserComponent as Component;
pub use core_layer::application_system::ecs::entity::UserEntity as Entity;
pub use core_layer::application_system::ecs::system::SystemSchedule;
pub use core_layer::application_system::events::builder as event_builder;
pub use core_layer::application_system::events::user_events::UserEventWrapper as Event;

pub use core_layer::input_system::{gamepad, keyboard, mouse};

pub use core_layer::logger_system::helpers::{LogLevel, LogTarget};
pub use core_layer::logger_system::logger::GLOBAL_LOGGER;

pub use platform_layer::platform::PlatformLayer;
pub use platform_layer::platform_impl::{PlatformLayerImpl, PlatformLayerRwLock};
pub use platform_layer::window::DisplayMode;

pub(crate) use rendering_layer::RenderingLayer;
pub(crate) use rendering_layer::rendering_impl::RenderingLayerImpl;
pub use rendering_layer::{shaders, types as renderer_types};

pub use macros::{Component, Resource};
