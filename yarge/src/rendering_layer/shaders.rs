#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// The different shader types
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ShaderStage {
    /// The compute stage
    Compute,
    /// The vertex stage
    Vertex,
    /// The fragment stage
    Fragment,
    // TODO: add other stages
}

/// The shader input location number
pub(crate) struct ShaderLocation(pub(crate) usize);
