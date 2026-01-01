/// Implements the Linux platforms
#[cfg(linux_platform)]
mod linux;
#[cfg(linux_platform)]
pub use linux::PlatformLayerImpl;
#[cfg(linux_platform)]
pub use std::sync::RwLock as PlatformLayerRwLock;
