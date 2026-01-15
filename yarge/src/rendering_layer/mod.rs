//! Rendering layer: abstraction layer that interacts directly
//! with the underlying rendering API

/// A module that abstracts a renderer
mod renderer;
pub(crate) use renderer::RenderingLayer;

/// The concrete implementation
pub(crate) mod rendering_impl;

/// A module with different types declaration
pub mod types;

/// A module to handle shaders and shaders operations
pub mod shaders;

/// A module to handle graphics pipelines
pub(crate) mod graphics_pipeline;

pub(crate) mod buffer;
pub(crate) mod mesh;
pub(crate) mod vertex;
