mod renderer;

pub use renderer::OpenglRenderingLayer as RenderingLayerImpl;

mod context;
mod types;

pub use types::OpenglConfig;
