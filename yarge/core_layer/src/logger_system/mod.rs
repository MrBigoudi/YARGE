mod logger;
pub use logger::LoggerSystem;

mod helpers;
use helpers::LogLevel;

mod logger_impl;
pub use logger_impl::macros::{debug, error, info, warn};
