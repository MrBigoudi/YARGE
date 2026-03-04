#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{
    Entity,
    core_layer::application_system::ecs::{
        component::Component, engine::transform::TransformComponent,
    },
    maths::{Matrix4x4, Vector3, Vector4, mat4x4, to_radians, vec3, vec4},
    rendering_layer::bounding_volumes::Frustum,
};

/// A union for the camera type
#[derive(Clone, Copy)]
pub(crate) enum RealProjection {
    Perspective(PerspectiveProjection),
    Orthographic(OrthographicProjection),
}

/// A camera component
#[derive(Clone)]
pub(crate) struct CameraComponent {
    /// The world up direction
    pub(crate) world_up: Vector3,
    /// The camera projection
    pub(crate) projection: RealProjection,
    /// The entities visible by the camera
    /// This field is rebuilt every-time culling is called on the camera
    /// and should only be used after culling is complete to avoid saving wrong entities
    pub(crate) visible_entities: Vec<Entity>,
}
impl Component for CameraComponent {}

impl CameraComponent {
    /// Gets the camera view matrix
    pub(crate) fn get_view(&self, transform: &TransformComponent) -> Matrix4x4 {
        let camera_position = transform.position;
        let target_position = transform.get_model() * Vector4::NEG_Z;
        let target_position = target_position.from_homogeneous();
        Matrix4x4::look_at(&camera_position, &target_position, &self.world_up)
    }

    /// Gets the camera projection matrix
    pub(crate) fn get_projection(&self) -> Matrix4x4 {
        match self.projection {
            RealProjection::Perspective(perspective) => perspective.projection(),
            RealProjection::Orthographic(orthographic) => orthographic.projection(),
        }
    }

    /// Gets the camera frustum in camera's view space
    pub(crate) fn get_view_space_frustum(&self) -> Frustum {
        match self.projection {
            RealProjection::Perspective(perspective) => perspective.view_frustum(),
            RealProjection::Orthographic(orthographic) => orthographic.view_frustum(),
        }
    }
}

pub(crate) trait CameraProjection {
    /// Creates a projection matrxi
    fn projection(&self) -> Matrix4x4;

    /// Computes the view frustum
    fn view_frustum(&self) -> Frustum;
}

/// A simple perspective camera
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PerspectiveProjection {
    /// The vertical field of view in degrees
    pub(crate) field_of_view: f32,
    /// The aspect ratio
    pub(crate) aspect_ratio: f32,
    /// The far plane
    pub(crate) far_plane: f32,
    /// The near plane
    pub(crate) near_plane: f32,
}

impl CameraProjection for PerspectiveProjection {
    fn projection(&self) -> Matrix4x4 {
        let fov = to_radians(self.field_of_view * 0.5);
        let cot = fov.sin() / fov.cos();
        let diff = self.far_plane - self.near_plane;
        mat4x4(
            &vec4(self.aspect_ratio * cot, 0., 0., 0.),
            &vec4(0., cot, 0., 0.),
            &vec4(
                0.,
                0.,
                -(self.far_plane + self.near_plane) / diff,
                -2. * self.far_plane * self.near_plane / diff,
            ),
            &vec4(0., 0., -1., 0.),
        )
    }

    fn view_frustum(&self) -> Frustum {
        let tan_half_fov = (to_radians(self.field_of_view) * 0.5).tan();
        let near_height = 2. * self.near_plane * tan_half_fov;
        let near_width = near_height * self.aspect_ratio;
        let far_height = 2. * self.far_plane * tan_half_fov;
        let far_width = far_height * self.aspect_ratio;

        let near_bottom_left = vec3(-near_width * 0.5, -near_height * 0.5, -self.near_plane);
        let near_bottom_right = vec3(near_width * 0.5, -near_height * 0.5, -self.near_plane);
        let near_top_right = vec3(near_width * 0.5, near_height * 0.5, -self.near_plane);
        let near_top_left = vec3(-near_width * 0.5, near_height * 0.5, -self.near_plane);

        let far_bottom_left = vec3(-far_width * 0.5, -far_height * 0.5, -self.far_plane);
        let far_bottom_right = vec3(far_width * 0.5, -far_height * 0.5, -self.far_plane);
        let far_top_right = vec3(far_width * 0.5, far_height * 0.5, -self.far_plane);
        let far_top_left = vec3(-far_width * 0.5, far_height * 0.5, -self.far_plane);

        Frustum {
            near_bottom_left,
            near_bottom_right,
            near_top_right,
            near_top_left,
            far_bottom_left,
            far_bottom_right,
            far_top_right,
            far_top_left,
        }
    }
}

/// A simple orthographic camera
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct OrthographicProjection {
    /// The left plane
    pub(crate) left_plane: f32,
    /// The right plane
    pub(crate) right_plane: f32,
    /// The bottom plane
    pub(crate) bottom_plane: f32,
    /// The top plane
    pub(crate) top_plane: f32,
    /// The far plane
    pub(crate) far_plane: f32,
    /// The near plane
    pub(crate) near_plane: f32,
}

impl CameraProjection for OrthographicProjection {
    fn projection(&self) -> Matrix4x4 {
        let width = self.right_plane - self.left_plane;
        let height = self.top_plane - self.bottom_plane;
        let depth = self.far_plane - self.near_plane;

        mat4x4(
            &vec4(
                2. / width,
                0.,
                0.,
                -(self.right_plane + self.left_plane) / width,
            ),
            &vec4(
                0.,
                2. / height,
                0.,
                -(self.top_plane + self.bottom_plane) / height,
            ),
            &vec4(
                0.,
                0.,
                -2. / depth,
                -(self.far_plane + self.near_plane) / depth,
            ),
            &vec4(0., 0., 0., 1.),
        )
    }

    fn view_frustum(&self) -> Frustum {
        let near_bottom_left = vec3(self.left_plane, self.bottom_plane, -self.near_plane);
        let near_bottom_right = vec3(self.right_plane, self.bottom_plane, -self.near_plane);
        let near_top_right = vec3(self.right_plane, self.top_plane, -self.near_plane);
        let near_top_left = vec3(self.left_plane, self.top_plane, -self.near_plane);

        let far_bottom_left = vec3(self.left_plane, self.bottom_plane, -self.far_plane);
        let far_bottom_right = vec3(self.right_plane, self.bottom_plane, -self.far_plane);
        let far_top_right = vec3(self.right_plane, self.top_plane, -self.far_plane);
        let far_top_left = vec3(self.left_plane, self.top_plane, -self.far_plane);

        Frustum {
            near_bottom_left,
            near_bottom_right,
            near_top_right,
            near_top_left,
            far_bottom_left,
            far_bottom_right,
            far_top_right,
            far_top_left,
        }
    }
}
