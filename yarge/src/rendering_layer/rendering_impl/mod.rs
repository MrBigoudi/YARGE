/// Implements the DirectX API
#[cfg(directx_renderer)]
mod directx;
#[cfg(directx_renderer)]
pub use directx::RenderingLayerImpl;

/// Implements the Metal API
#[cfg(metal_renderer)]
mod metal;
#[cfg(metal_renderer)]
pub use metal::RenderingLayerImpl;

/// Implements the OpenGL API
#[cfg(opengl_renderer)]
mod opengl;
#[cfg(opengl_renderer)]
pub use opengl::OpenglConfig;
#[cfg(opengl_renderer)]
pub use opengl::RenderingLayerImpl;

/// Implements the WebGPU API
#[cfg(wgpu_renderer)]
mod wgpu;
#[cfg(wgpu_renderer)]
pub use wgpu::RenderingLayerImpl;

/// Implements the Vulkan API
#[cfg(vulkan_renderer)]
mod vulkan;
#[cfg(vulkan_renderer)]
pub use vulkan::RenderingLayerImpl;
#[allow(unused)]
#[cfg(vulkan_renderer)]
pub use vulkan::{VkFeatures11, VkFeatures12, VkFeatures13, VkLayers};

pub mod types;
