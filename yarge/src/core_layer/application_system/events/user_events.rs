use std::collections::VecDeque;

use crate::{
    core_layer::ApplicationSystem, error::ErrorType, platform_layer::PlatformLayerImpl,
    rendering_layer::RenderingLayerImpl,
};

/// An enum representing user fireable events
#[derive(Debug, Clone, PartialEq)]
pub enum UserEvent {
    /// To close the application
    QuitApp,
}

impl<'a> ApplicationSystem<'a> {
    /// User events handling
    /// Returns true if the application should quit
    pub(crate) fn handle_user_events(
        &mut self,
        events: VecDeque<UserEvent>,
        _platform_layer: &mut PlatformLayerImpl,
        _rendering_layer: &mut RenderingLayerImpl,
    ) -> Result<bool, ErrorType> {
        let mut should_quit = false;
        for event in &events {
            match event {
                UserEvent::QuitApp => {
                    should_quit = true;
                }
            }
        }

        Ok(should_quit)
    }
}
