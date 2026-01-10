/// Macro for for logging message
/// This macro should not be used on its own but through other macros like error!, warn!, debug! and info!
#[macro_export]
macro_rules! log {
    ($level:expr, $target:expr) => {{
        #[allow(unused)]
        use $crate::{PlatformLayer, PlatformLayerImpl, LogLevel, LogTarget};
        let message = format!("No message (from {}:{})", file!(), line!());
        if let Err(err) = PlatformLayerImpl::write_log($level, &message, $target) {
            let message = format!("Logging failure {:?}, default to error console, original message: {}\n", err, message
            );
            if let Err(err) = PlatformLayerImpl::write_log(&LogLevel::Warn, &message, &LogTarget::ErrorConsole) {
                panic!("Critical logging failure: {:?}, original message: {}", err, message);
            }
        }
    }};
    ($level:expr, $target:expr, $($arg:tt)*) => {{
        #[allow(unused)]
        use $crate::{PlatformLayer, PlatformLayerImpl, LogLevel, LogTarget};
        let message = format!("{} (from {}:{})", format!($($arg)*), file!(), line!());
        if let Err(err) = PlatformLayerImpl::write_log($level, &message, $target) {
            let message = format!("Logging failure {:?}, default to error console, original message: {}\n",
                err, message
            );
            if let Err(err) = PlatformLayerImpl::write_log(&LogLevel::Warn, &message, &LogTarget::ErrorConsole) {
                panic!("Critical logging failure: {:?}, original message: {}", err, message);
            }
        }
    }};
}

/// Macro to log info messages
#[macro_export]
macro_rules! log_info {
    () => {{
        #[allow(unused)]
        use $crate::{LogLevel, LogTarget};
        match $crate::GLOBAL_LOGGER.read() {
            Ok(logger) => {
                let target = &logger.config.target;
                log!(&LogLevel::Info, target)
            },
            Err(err) => {
                let message = format!("Logging failure {:?}, failed to get info from the global logger, default to error console\n",
                    err
                );
                log!(&LogLevel::Error, &LogTarget::ErrorConsole, "{}", message)
            }
        }
    }};
    ($($arg:tt)*) => {{
        #[allow(unused)]
        use $crate::{LogLevel, LogTarget, log};
        match $crate::GLOBAL_LOGGER.read() {
            Ok(logger) => {
                let target = &logger.config.target;
                log!(&LogLevel::Info, target, $($arg)*)
            },
            Err(err) => {
                let message = format!("Logging failure {:?}, failed to get info from the global logger, default to error console, original message {}\n",
                    err, format!($($arg)*)
                );
                log!(&LogLevel::Error, &LogTarget::ErrorConsole, "{}", message)
            }
        }
    }};
}

/// Macro to log debug messages
#[macro_export]
macro_rules! log_debug {
    () => {{
        #[allow(unused)]
        use $crate::{LogLevel, LogTarget, log};
        match $crate::GLOBAL_LOGGER.read() {
            Ok(logger) => {
                let target = &logger.config.target;
                log!(&LogLevel::Debug, target)
            },
            Err(err) => {
                let message = format!("Logging failure {:?}, failed to get info from the global logger, default to error console\n",
                    err
                );
                log!(&LogLevel::Error, &LogTarget::ErrorConsole, "{}", message)
            }
        }
    }};
    ($($arg:tt)*) => {{
        #[allow(unused)]
        use $crate::{LogLevel, LogTarget, log};
        match $crate::GLOBAL_LOGGER.read() {
            Ok(logger) => {
                let target = &logger.config.target;
                log!(&LogLevel::Debug, target, $($arg)*)
            },
            Err(err) => {
                let message = format!("Logging failure {:?}, failed to get info from the global logger, default to error console, original message {}\n",
                    err, format!($($arg)*)
                );
                log!(&LogLevel::Error, &LogTarget::ErrorConsole, "{}", message)
            }
        }
    }};
}

/// Macro to log warn messages
#[macro_export]
macro_rules! log_warn {
    () => {{
        #[allow(unused)]
        use $crate::{LogLevel, LogTarget, log};
        match $crate::GLOBAL_LOGGER.read() {
            Ok(logger) => {
                let target = &logger.config.target;
                log!(&LogLevel::Warn, target)
            },
            Err(err) => {
                let message = format!("Logging failure {:?}, failed to get info from the global logger, default to error console\n",
                    err
                );
                log!(&LogLevel::Error, &LogTarget::ErrorConsole, "{}", message)
            }
        }
    }};
    ($($arg:tt)*) => {{
        #[allow(unused)]
        use $crate::{LogLevel, LogTarget, log};
        match $crate::GLOBAL_LOGGER.read() {
            Ok(logger) => {
                let target = &logger.config.target;
                log!(&LogLevel::Warn, target, $($arg)*)
            },
            Err(err) => {
                let message = format!("Logging failure {:?}, failed to get info from the global logger, default to error console, original message {}\n",
                    err, format!($($arg)*)
                );
                log!(&LogLevel::Error, &LogTarget::ErrorConsole, "{}", message)
            }
        }
    }};
}

/// Macro to log error messages
#[macro_export]
macro_rules! log_error {
    () => {{
        #[allow(unused)]
        use $crate::{LogLevel, LogTarget, log};
        match $crate::GLOBAL_LOGGER.read() {
            Ok(logger) => {
                let target = &logger.config.target;
                $crate::log!(&LogLevel::Error, target)
            },
            Err(err) => {
                let message = format!(
                    "Logging failure {:?}, failed to get info from the global logger, default to error console\n",
                    err
                );
                $crate::log!(&LogLevel::Error, &LogTarget::ErrorConsole, "{}", message)
            }
        }
    }};
    ($($arg:tt)*) => {{
        #[allow(unused)]
        use $crate::{LogLevel, LogTarget, log};
        match $crate::GLOBAL_LOGGER.read() {
            Ok(logger) => {
                let target = &logger.config.target;
                log!(&LogLevel::Error, target, $($arg)*)
            },
            Err(err) => {
                let message = format!("Logging failure {:?}, failed to get info from the global logger, default to error console, original message {}\n",
                    err, format!($($arg)*)
                );
                log!(&LogLevel::Error, &LogTarget::ErrorConsole, "{}", message)
            }
        }
    }};
}
