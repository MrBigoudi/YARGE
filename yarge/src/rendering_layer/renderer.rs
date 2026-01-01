use crate::{config::Config, error::ErrorType};

/// Abstract trait for the renderer backend specific code
pub trait RendereringLayer {
    /// The type of the struct implementing the trait
    /// This would often be `Self`
    type RendereringLayerType;

    /// Initializes the renderer backend
    fn init(config: &Config) -> Result<Self::RendereringLayerType, ErrorType>;

    /// Shuts down the renderer backend
    fn shutdown(&mut self) -> Result<(), ErrorType>;
}
