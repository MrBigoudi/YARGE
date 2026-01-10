#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::config::Config;

/// The opengl context
pub(crate) struct OpenglContext {}

impl OpenglContext {
    pub(crate) fn new(_config: &Config) -> Result<Self, ErrorType> {
        Ok(Self {})
    }
}
