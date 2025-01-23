use error::ErrorType;

/// The game trait that can be ovveride by the user
pub trait Game {
    /// Runs when the application starts
    /// Default behavior: don't do anything
    fn on_start(&mut self) -> Result<(), ErrorType> {
        // TODO: add debug message
        Ok(())
    }

    /// Runs each frame
    /// Default behavior: don't do anything
    fn on_update(&mut self, _delta_time: f64) -> Result<(), ErrorType> {
        // TODO: add debug message
        Ok(())
    }

    /// Runs each frame
    /// Default behavior: don't do anything
    fn on_render(&self, _delta_time: f64) -> Result<(), ErrorType> {
        // TODO: add debug message
        Ok(())
    }

    /// Runs everytime the user wants to resize the window
    /// Default behavior: don't do anything
    fn on_resize(&mut self, _new_width: f32, _new_height: f32) -> Result<(), ErrorType> {
        // TODO: add debug message
        Ok(())
    }

    /// Runs when the application stops
    /// Default behavior: don't do anything
    fn on_shutdown(&mut self) -> Result<(), ErrorType> {
        // TODO: add debug message
        Ok(())
    }
}
