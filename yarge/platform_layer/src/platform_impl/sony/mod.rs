/// Implements the ps-vita platform
#[cfg(psvita_platform)]
mod psvita;
#[cfg(psvita_platform)]
pub use psvita::PsVitaPlatformLayer as PlatformLayerImpl;
