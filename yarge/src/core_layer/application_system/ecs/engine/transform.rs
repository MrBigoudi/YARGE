#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::core_layer::application_system::ecs::component::Component;

use crate::maths::{Matrix4x4, Vector3, to_radians};

/// A simple transform component
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TransformComponent {
    /// The position in object space
    pub(crate) position: Vector3,
    /// The scaling in object space
    pub(crate) scale: Vector3,
    /// The rotation in object space
    /// Using Euler angles (for now)
    /// The angles are in degrees between -360 and 360
    pub(crate) rotation: Vector3,
}
impl Component for TransformComponent {}

impl Default for TransformComponent {
    fn default() -> Self {
        Self {
            position: Vector3::ZEROS,
            scale: Vector3::ONES,
            rotation: Vector3::ZEROS,
        }
    }
}

impl TransformComponent {
    /// Gets the transformation matrix
    pub(crate) fn get_model(&self) -> Matrix4x4 {
        let scaling = Matrix4x4::scale(self.scale.x, self.scale.y, self.scale.z);
        let rotation_x = Matrix4x4::rotation_x(to_radians(self.rotation.x));
        let rotation_y = Matrix4x4::rotation_y(to_radians(self.rotation.y));
        let rotation_z = Matrix4x4::rotation_z(to_radians(self.rotation.z));
        let translation = Matrix4x4::translation(self.position.x, self.position.y, self.position.z);

        translation * rotation_z * rotation_y * rotation_x * scaling
    }
}
