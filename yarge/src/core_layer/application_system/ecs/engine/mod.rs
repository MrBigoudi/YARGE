//! Engine defined components and systems

#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::core_layer::application_system::ecs::component::{Component, ComponentId};
use std::collections::VecDeque;

pub(crate) mod camera;
pub(crate) mod is_activated;
pub(crate) mod mesh;
pub(crate) mod transform;

/// Engine defined components ids
pub(crate) struct EngineComponents {
    /// The id of the activated component
    pub(crate) is_activated: ComponentId,
    /// The id of the transform component
    pub(crate) transform: ComponentId,
    /// The id of the mesh component
    pub(crate) mesh: ComponentId,
    /// The id of the camera component
    pub(crate) camera: ComponentId,
}

/// A macro to generate register engine component events
macro_rules! create_register_component_event {
    ($T:ty, $name:literal) => {
        match crate::event_builder::RegisterCustomComponentEventBuilder::default()
            .component_type::<$T>()
            .build()
        {
            Ok(event) => event,
            Err(err) => {
                log_error!(
                    "Failed to create the \"register engine level `{:?}' component\" event: {:?}",
                    $name,
                    err
                );
                return Err(ErrorType::Unknown);
            }
        }
    };
}

impl EngineComponents {
    pub(crate) fn init() -> Result<(Self, VecDeque<crate::Event>), ErrorType> {
        let mut events = VecDeque::new();

        // IsActivatedComponent
        events.push_back(create_register_component_event!(
            is_activated::IsActivatedComponent,
            "is_activated"
        ));
        let is_activated = is_activated::IsActivatedComponent::get_type_id();

        // TransformComponent
        events.push_back(create_register_component_event!(
            transform::TransformComponent,
            "transform"
        ));
        let transform = transform::TransformComponent::get_type_id();

        // MeshComponent
        events.push_back(create_register_component_event!(
            mesh::MeshComponent,
            "mesh"
        ));
        let mesh = mesh::MeshComponent::get_type_id();

        // CameraComponent
        events.push_back(create_register_component_event!(
            camera::CameraComponent,
            "camera"
        ));
        let camera = camera::CameraComponent::get_type_id();

        let engine_components = EngineComponents {
            is_activated,
            transform,
            mesh,
            camera,
        };

        Ok((engine_components, events))
    }
}
