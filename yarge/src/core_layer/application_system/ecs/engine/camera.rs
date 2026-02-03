#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{
    core_layer::application_system::ecs::component::Component,
    maths::{Matrix4x4, Vector3, mat4x4, to_radians, vec3, vec4},
};

/// A simple perspective camera component
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct PerspectiveCameraComponent {
    /// The vertical field of view in degrees
    pub(crate) field_of_view: f32,
    /// The aspect ratio
    pub(crate) aspect_ratio: f32,
    /// The far plane
    pub(crate) far_plane: f32,
    /// The near plane
    pub(crate) near_plane: f32,
}
impl Component for PerspectiveCameraComponent {}

/// A camera frustum view
pub(crate) struct CameraViewFrustum {
    /// The near plane bottom left corner
    pub(crate) near_bottom_left: Vector3,
    /// The near plane bottom right corner
    pub(crate) near_bottom_right: Vector3,
    /// The near plane top right corner
    pub(crate) near_top_right: Vector3,
    /// The near plane top left corner
    pub(crate) near_top_left: Vector3,

    /// The far plane bottom left corner
    pub(crate) far_bottom_left: Vector3,
    /// The far plane bottom right corner
    pub(crate) far_bottom_right: Vector3,
    /// The far plane top right corner
    pub(crate) far_top_right: Vector3,
    /// The far plane top left corner
    pub(crate) far_top_left: Vector3,
}

pub(crate) trait Camera {
    /// Creates a view matrix
    fn view(camera_position: &Vector3, target_position: &Vector3, world_up: &Vector3) -> Matrix4x4 {
        Matrix4x4::look_at(camera_position, target_position, world_up)
    }

    /// Creates a projection matrxi
    fn projection(&self) -> Matrix4x4;

    /// Computes the view frustum
    fn view_frustum(&self) -> CameraViewFrustum;
}

impl Camera for PerspectiveCameraComponent {
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

    fn view_frustum(&self) -> CameraViewFrustum {
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

        CameraViewFrustum {
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

/// A simple orthographic camera component
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct OrthographicCameraComponent {
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
impl Component for OrthographicCameraComponent {}

impl Camera for OrthographicCameraComponent {
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

    fn view_frustum(&self) -> CameraViewFrustum {
        let near_bottom_left = vec3(self.left_plane, self.bottom_plane, -self.near_plane);
        let near_bottom_right = vec3(self.right_plane, self.bottom_plane, -self.near_plane);
        let near_top_right = vec3(self.right_plane, self.top_plane, -self.near_plane);
        let near_top_left = vec3(self.left_plane, self.top_plane, -self.near_plane);

        let far_bottom_left = vec3(self.left_plane, self.bottom_plane, -self.far_plane);
        let far_bottom_right = vec3(self.right_plane, self.bottom_plane, -self.far_plane);
        let far_top_right = vec3(self.right_plane, self.top_plane, -self.far_plane);
        let far_top_left = vec3(self.left_plane, self.top_plane, -self.far_plane);

        CameraViewFrustum {
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
