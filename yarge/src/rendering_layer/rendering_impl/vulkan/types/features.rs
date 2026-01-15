#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// Custom enum for Vulkan 1.1 features names
#[derive(Debug, Clone, Copy)]
pub(crate) enum VkFeatures11 {}

/// Custom enum for Vulkan 1.2 features names
#[derive(Debug, Clone, Copy)]
pub(crate) enum VkFeatures12 {}

/// Custom enum for Vulkan 1.3 features names
#[derive(Debug, Clone, Copy)]
pub(crate) enum VkFeatures13 {}
