mod logger;
pub use logger::LoggerSystem;

mod helpers;
use helpers::LogLevel;

mod logger_impl;
pub use logger_impl::macros::debug as log_debug;
pub use logger_impl::macros::error as log_error;
pub use logger_impl::macros::info as log_info;
pub use logger_impl::macros::warn as log_warn;
