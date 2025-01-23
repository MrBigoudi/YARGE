use config::{ConfigLogLevel, Config};
use error::ErrorType;
use tracing_subscriber::{Layer,prelude::*};

/// The possible log levels
pub enum LogLevel {
    /// To be used when displaying information
    Info,
    /// Only visible on debug mode
    Debug,
    /// Non fatal error messages
    Warn,
    /// Fatal error messages
    Fatal,
}

impl LogLevel {
    pub fn from_config(config: &ConfigLogLevel) -> Self {
        match config {
            ConfigLogLevel::Info => LogLevel::Info,
            ConfigLogLevel::Debug => LogLevel::Debug,
            ConfigLogLevel::Warn => LogLevel::Warn,
            ConfigLogLevel::Fatal => LogLevel::Fatal,
        }
    }
}

/// A custom logger
/// Based from a [blog post](https://burgers.io/custom-logging-in-rust-using-tracing)
pub struct LoggerSystem {
    /// The minimum log level to be displayed
    /// Any logs with weaker level won't be displayed
    pub min_level: LogLevel,
}

impl<S> Layer<S> for LoggerSystem where S: tracing::Subscriber {
    fn on_event(&self, event: &tracing::Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        // TODO: remove temporary code
        println!("Got event!");
        println!("  level={:?}", event.metadata().level());
        println!("  target={:?}", event.metadata().target());
        println!("  name={:?}", event.metadata().name());
        for field in event.fields() {
            println!("  field={}", field.name());
        }
    }
}


impl LoggerSystem {
    /// Initiate the logger systems
    pub fn init(config: &Config) -> Result<Self, ErrorType> {
        let min_level = LogLevel::from_config(&config.logger_config.level);
        let logger = LoggerSystem{
            min_level,
        };

        // Sets up how `tracing-subscriber` will deal with tracing data
        tracing_subscriber::registry().with(logger).init();

        tracing::info!(a_bool = true, answer = 42, message = "first example");
        todo!();
        Ok(logger)
    }

    pub fn shutdown(&mut self) -> Result<(), ErrorType> {
        todo!()
    }
}