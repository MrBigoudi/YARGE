/// Implements the Linux platforms
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::PlatformLayerImpl;
