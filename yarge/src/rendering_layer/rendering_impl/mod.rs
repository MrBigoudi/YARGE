/// Implements the DirectX API
#[cfg(directx_renderer)]
mod directx;
#[cfg(directx_renderer)]
pub use directx::RendererBackendImpl;

/// Implements the Metal API
#[cfg(metal_renderer)]
mod metal;
#[cfg(metal_renderer)]
pub use metal::RendererBackendImpl;

/// Implements the OpenGL API
#[cfg(opengl_renderer)]
mod opengl;
#[cfg(opengl_renderer)]
pub use opengl::RendererBackendImpl;

/// Implements the WebGPU API
#[cfg(wgpu_renderer)]
mod wgpu;
#[cfg(wgpu_renderer)]
pub use wgpu::RendererBackendImpl;

/// Implements the Vulkan API
#[cfg(vulkan_renderer)]
mod vulkan;
#[cfg(vulkan_renderer)]
pub use vulkan::RendererBackendImpl;