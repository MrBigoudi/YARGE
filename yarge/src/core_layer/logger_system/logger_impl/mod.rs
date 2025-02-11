use tracing_subscriber::{Layer, prelude::*};
use visitor_console::LoggerConsoleVisitor;

use crate::{config::Config, error::ErrorType};

use super::{
    LoggerSystem,
    helpers::{LogLevel, LogTarget},
};

pub mod macros;

mod visitor_console;

impl LogLevel {
    fn as_tracing_level(&self) -> tracing::Level {
        match self {
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}

impl<S> Layer<S> for LoggerSystem
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        // Do not display logs that are under the minimum level
        if event.metadata().level() < &self.min_level.as_tracing_level() {
            return;
        }

        match &self.target {
            LogTarget::Console => {
                let mut visitor = LoggerConsoleVisitor;
                event.record(&mut visitor);
            }
            LogTarget::ErrorConsole => todo!(),
            LogTarget::Markdown(_) => todo!(),
            LogTarget::Json(_) => todo!(),
        }
    }
}

impl LoggerSystem {
    /// Initiates the logger systems
    pub fn init(config: &Config) -> Result<Self, ErrorType> {
        let min_level = LogLevel::from_config(&config.logger_config.level);
        let target = LogTarget::from_config(&config.logger_config.target);

        let logger = LoggerSystem {
            min_level,
            target: target.clone(),
        };

        // Sets up how `tracing-subscriber` will deal with tracing data
        tracing_subscriber::registry().with(logger).init();

        Ok(LoggerSystem { min_level, target })
    }

    /// Updates the the minimum log level
    pub fn update_min_level(&mut self, new_min_level: LogLevel) {
        let logger = LoggerSystem {
            min_level: new_min_level,
            target: self.target.clone(),
        };
        tracing_subscriber::registry().with(logger).init();
        self.min_level = new_min_level;
    }

    /// Updates the the log target
    pub fn update_target(&mut self, new_target: LogTarget) {
        let logger = LoggerSystem {
            min_level: self.min_level,
            target: new_target.clone(),
        };
        tracing_subscriber::registry().with(logger).init();
        self.target = new_target;
    }

    /// Shuts down the logger
    pub fn shutdown(&mut self) -> Result<(), ErrorType> {
        Ok(())
    }
}
