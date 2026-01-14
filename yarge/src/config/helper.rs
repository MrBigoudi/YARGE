#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// A struct representing a version
#[derive(Debug, Clone)]
pub struct Version {
    /// The variant
    pub variant: u8,
    /// The major
    pub major: u8,
    /// The minor
    pub minor: u8,
    /// The patch
    pub patch: u8,
}

impl Default for Version {
    /// Default to version (0.1.0.0)
    fn default() -> Self {
        Self {
            variant: 0u8,
            major: 1u8,
            minor: 0u8,
            patch: 0u8,
        }
    }
}

impl Version {
    /// Simple constructor
    pub fn new(variant: u8, major: u8, minor: u8, patch: u8) -> Self {
        Version {
            variant,
            major,
            minor,
            patch,
        }
    }

    /// Sets the variant using the builder pattern
    pub fn variant(mut self, variant: u8) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the major using the builder pattern
    pub fn major(mut self, major: u8) -> Self {
        self.major = major;
        self
    }

    /// Sets the minor using the builder pattern
    pub fn minor(mut self, minor: u8) -> Self {
        self.minor = minor;
        self
    }

    /// Sets the patch using the builder pattern
    pub fn patch(mut self, patch: u8) -> Self {
        self.patch = patch;
        self
    }

    /// Creates a string from a version
    pub fn as_string(&self) -> String {
        format!(
            "({:?}.{:?}.{:?}.{:?})",
            self.variant, self.major, self.minor, self.patch
        )
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.variant == other.variant
            && self.major == other.major
            && self.minor == other.minor
            && self.patch == other.patch
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.variant.partial_cmp(&other.variant) {
            Some(std::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.major.partial_cmp(&other.major) {
            Some(std::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.minor.partial_cmp(&other.minor) {
            Some(std::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.patch.partial_cmp(&other.patch)
    }
}