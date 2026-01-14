#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::core_layer::application_system::ecs::component::Component;

use crate::maths::Vector3;

/// A simple transform component
pub(crate) struct TransformComponent {
    /// The position in object space
    pub(crate) position: Vector3,
    /// The scaling in object space
    pub(crate) scale: Vector3,
    /// The rotation in object space
    /// Using Euler angles (for now)
    pub(crate) rotation: Vector3,
}
impl Component for TransformComponent {}
