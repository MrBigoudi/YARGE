#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{core_layer::application_system::ecs::component::Component, maths::{Matrix4x4, Vector3, Vector4, vec4}};

/// The possible camera projections
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum CameraProjection {
    /// An orthographic camera
    Orthographic,
    /// A perspective camera
    Perspective,
}

impl CameraProjection {
    /// Creates an orthographic matrix
    pub(crate) fn get_orthographic() -> Matrix4x4 {
        todo!()
    }

    /// Creates a perspective matrix
    pub(crate) fn get_perspective() -> Matrix4x4 {
        todo!()
    }

    /// Creates a projection matrix
    pub(crate) fn get_projection(&self) -> Matrix4x4 {
        match self {
            CameraProjection::Orthographic => Self::get_orthographic(),
            CameraProjection::Perspective => Self::get_perspective(),
        }
    }
}

/// The clip plane
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CameraClipPlane {
    /// The far plane
    pub(crate) far: f32,
    /// The near plane
    pub(crate) near: f32,
}

/// A simple camera component
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CameraComponent {
    /// The projection type
    pub(crate) projection: CameraProjection,
    /// The field of view
    pub(crate) field_of_view: f32,
    /// The clipping planes
    pub(crate) clip_planes: CameraClipPlane,
}

impl Component for CameraComponent{}


impl CameraComponent {
    /// Creates a look-at matrix
    pub(crate) fn look_at(camera_position: &Vector3, target_position: &Vector3, world_up: &Vector3) -> Matrix4x4 {
        let forward = (camera_position-target_position).normalize()
            .expect("Failed to normalize a vector when building a look at matrix")
        ;
        let right = Vector3::cross(world_up, &forward).normalize()
            .expect("Failed to normalize a vector when building a look at matrix")
        ;
        let up = Vector3::cross(&forward, &right);

        let rotation = Matrix4x4::new(
            &vec4(right.x, right.y, right.z, 1f32),
            &vec4(up.x, up.y, up.z, 1f32),
            &vec4(forward.x, forward.y, forward.z, 1f32),
            &Vector4::ONES,
        );

        let translation = Matrix4x4::translation(
            -camera_position.x,
            -camera_position.y,
            -camera_position.z,
        );

        rotation * translation
    }
}
