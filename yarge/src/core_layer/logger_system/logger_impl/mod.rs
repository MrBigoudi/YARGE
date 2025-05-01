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
        if event.metadata().level() < &self.config.min_level.as_tracing_level() {
            return;
        }

        match &self.config.target {
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

        let logger = LoggerSystem {
            config: config.logger_config.clone(),
        };

        // Sets up how `tracing-subscriber` will deal with tracing data
        tracing_subscriber::registry().with(logger).init();

        Ok(LoggerSystem {
            config: config.logger_config.clone(),
        })
    }

    /// Updates the the minimum log level
    pub fn update_min_level(&mut self, new_min_level: LogLevel) {
        let mut new_config = self.config.clone();
        new_config.min_level = new_min_level;
        let logger = LoggerSystem {
            config: new_config,
        };
        tracing_subscriber::registry().with(logger).init();
        self.config.min_level = new_min_level;
    }

    /// Updates the the log target
    pub fn update_target(&mut self, new_target: LogTarget) {
        let mut new_config = self.config.clone();
        new_config.target = new_target.clone();
        let logger = LoggerSystem {
            config: new_config,
        };
        tracing_subscriber::registry().with(logger).init();
        self.config.target = new_target;
    }

    /// Shuts down the logger
    pub fn shutdown(&mut self) -> Result<(), ErrorType> {
        Ok(())
    }
}
