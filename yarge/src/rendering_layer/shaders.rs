#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// The different shader types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShaderStage {
    /// The compute stage
    Compute,
    /// The vertex stage
    Vertex,
    /// The fragment stage
    Fragment,
    // TODO: add other stages
}
