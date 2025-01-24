/// Implements the Linux platforms
#[cfg(linux_platform)]
mod linux;
#[cfg(linux_platform)]
pub use linux::PlatformLayerImpl;

/// Implements the Sony platforms
#[cfg(sony_platform)]
mod sony;
#[cfg(sony_platform)]
pub use sony::PlatformLayerImpl;