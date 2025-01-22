/// Implements the Linux X11 platform
#[cfg(x11_platform)]
mod x11;
#[cfg(x11_platform)]
pub use x11::LinuxX11PlatformLayer as PlatformLayerImpl;
