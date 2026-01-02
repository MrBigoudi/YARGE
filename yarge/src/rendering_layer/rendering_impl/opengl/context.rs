#[allow(unused)]
use crate::{config::Config, error::ErrorType, log, log_error, log_info};

/// The opengl context
pub struct OpenglContext {}

impl OpenglContext {
    pub fn new(_config: &Config) -> Result<Self, ErrorType> {
        Ok(Self {})
    }
}
