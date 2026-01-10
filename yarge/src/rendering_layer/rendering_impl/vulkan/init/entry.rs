#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use ash::Entry;

/// Helper function to initiate the vulkan entry
pub fn init_entry() -> Result<Entry, ErrorType> {
    match unsafe { Entry::load() } {
        Ok(entry) => Ok(entry),
        Err(err) => {
            log_error!("Failed to load the vulkan entry in ash: {:?}", err);
            Err(ErrorType::Unknown)
        }
    }
}
