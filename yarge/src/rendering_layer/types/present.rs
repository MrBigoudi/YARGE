#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// Different present modes
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum PresentMode {
    /// Images submitted by the application are transferred to the screen right away which may result in tearing
    Immediate,
    /// The display takes an image from a queue when the display is refreshed, and the program inserts rendered images at the back of the queue
    /// If the queue is full, then the program has to wait
    Vsync,
    /// Similar to Vsync but instead of blocking the application when the queue is full, the images that are already queued are replaced with the newer ones
    TripleBuffering,
}
