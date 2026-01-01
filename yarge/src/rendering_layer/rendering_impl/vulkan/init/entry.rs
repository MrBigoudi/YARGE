use ash::Entry;

#[allow(unused)]
use crate::{error::ErrorType, log, log_error};

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
