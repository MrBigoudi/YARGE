#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// Different image usages
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ImageUsage {
    /// To copy from this image
    TransferSrc,
    /// To copy into this image
    TransferDst,

    /// To store data in this image
    Storage,
    /// To read data from the image
    Sample,

    /// To draw into this image
    ColorAttachment,
    /// To use the image as a depth buffer
    DepthAttachment,
    /// To use the image as a stencil buffer
    StencilAttachment,

    /// To create a temporary image used only during rendering
    /// Usefull for MSAA
    TransientAttachment,
}
