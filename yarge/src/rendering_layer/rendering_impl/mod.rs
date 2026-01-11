/// Implements the DirectX API
#[cfg(directx_renderer)]
mod directx;
#[cfg(directx_renderer)]
pub(crate) use directx::RenderingLayerImpl;

/// Implements the Metal API
#[cfg(metal_renderer)]
mod metal;
#[cfg(metal_renderer)]
pub(crate) use metal::RenderingLayerImpl;

/// Implements the OpenGL API
#[cfg(opengl_renderer)]
mod opengl;
#[cfg(opengl_renderer)]
pub(crate) use opengl::OpenglConfig;
#[cfg(opengl_renderer)]
pub(crate) use opengl::RenderingLayerImpl;

/// Implements the WebGPU API
#[cfg(wgpu_renderer)]
mod wgpu;
#[cfg(wgpu_renderer)]
pub(crate) use wgpu::RenderingLayerImpl;

/// Implements the Vulkan API
#[cfg(vulkan_renderer)]
mod vulkan;
#[cfg(vulkan_renderer)]
pub(crate) use vulkan::RenderingLayerImpl;

#[cfg(vulkan_renderer)]
#[allow(unused)]
pub(crate) use vulkan::{VkFeatures11, VkFeatures12, VkFeatures13, VkLayers};
