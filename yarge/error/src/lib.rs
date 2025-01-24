#![warn(missing_docs)]

//! Library containing everything needed to debug the code

mod error;
pub use error::{ErrorLevel, ErrorType};

mod warnings;
pub use warnings::WarningType;
