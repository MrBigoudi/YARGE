//! Rendering layer: abstraction layer that interacts directly
//! with the underlying rendering API

/// A module that abstracts a renderer
mod renderer;
pub use renderer::RenderingLayer;

/// The concrete implementation
pub mod rendering_impl;
pub use rendering_impl::RenderingLayerImpl;

/// A module with different types declaration
pub mod types;