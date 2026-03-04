#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{
    maths::{Matrix4x4, Vector3, vec3, vec4},
    rendering_layer::mesh::MeshData,
};

/// A plane
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Plane {
    /// A 3D point
    pub(crate) point: Vector3,
    /// A normal
    pub(crate) normal: Vector3,
}

impl Plane {
    /// Checks if a point is on the normal side of the plane
    pub(crate) fn half_space_test(&self, point: &Vector3) -> bool {
        Vector3::dot(&(point - self.point), &self.normal) > 0f32
    }
}

/// A structure representing a frustum bounding volume
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Frustum {
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

/// A structure representing a frustum's planes
/// The planes' normals are facing outward (directed outside the frustum)
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct FrustumPlanes {
    /// The left plane
    pub(crate) left: Plane,
    /// The right plane
    pub(crate) right: Plane,
    /// The top plane
    pub(crate) top: Plane,
    /// The bottom plane
    pub(crate) bottom: Plane,
    /// The near plane
    pub(crate) near: Plane,
    /// The far plane
    pub(crate) far: Plane,
}

impl Frustum {
    /// Gets the frustum in world space
    pub(crate) fn as_world(&mut self, model: &Matrix4x4) {
        macro_rules! transform {
            ($var:expr) => {{
                let new_var = vec4($var.x, $var.y, $var.z, 1.);
                $var = (model * new_var).from_homogeneous();
            }};
        }

        transform!(self.near_bottom_left);
        transform!(self.near_bottom_right);
        transform!(self.near_top_left);
        transform!(self.near_top_right);
        transform!(self.far_bottom_left);
        transform!(self.far_bottom_right);
        transform!(self.far_top_left);
        transform!(self.far_top_right);
    }

    /// Gets the frustum planes normal
    pub(crate) fn get_planes(&self) -> FrustumPlanes {
        let x_edge = (self.near_top_right - self.near_top_left)
            .normalize()
            .unwrap();
        let y_edge = (self.near_top_left - self.near_bottom_left)
            .normalize()
            .unwrap();
        let z_edge = (self.near_top_left - self.far_top_left)
            .normalize()
            .unwrap();

        let left_normal = Vector3::cross(&z_edge, &y_edge);
        let right_normal = -left_normal;
        let top_normal = Vector3::cross(&z_edge, &x_edge);
        let bottom_normal = -top_normal;
        let near_normal = Vector3::cross(&x_edge, &y_edge);
        let far_normal = -near_normal;

        FrustumPlanes {
            left: Plane {
                point: self.near_bottom_left,
                normal: left_normal,
            },
            right: Plane {
                point: self.far_top_right,
                normal: right_normal,
            },
            top: Plane {
                point: self.far_top_right,
                normal: top_normal,
            },
            bottom: Plane {
                point: self.near_bottom_left,
                normal: bottom_normal,
            },
            near: Plane {
                point: self.near_bottom_left,
                normal: near_normal,
            },
            far: Plane {
                point: self.far_top_right,
                normal: far_normal,
            },
        }
    }

    /// Checks if a point is inside the frustum
    /// Assumes the point and the frustum are in the same space
    pub(crate) fn contains(&self, point: &Vector3) -> bool {
        let planes = self.get_planes();
        !planes.left.half_space_test(point)
            && !planes.right.half_space_test(point)
            && !planes.top.half_space_test(point)
            && !planes.bottom.half_space_test(point)
            && !planes.near.half_space_test(point)
            && !planes.far.half_space_test(point)
    }

    /// Checks if a frustum intersects an AABB
    /// Assumes the AABB and the frustum are in the same space
    pub(crate) fn intersects(&self, aabb: &AABB) -> bool {
        let points = aabb.get_points();
        for point in &points {
            if self.contains(point) {
                return true;
            }
        }
        false
    }
}

/// A structure representing an Axis-Aligned Bounding Box
#[allow(clippy::upper_case_acronyms)]
pub(crate) struct AABB {
    pub(crate) mins: Vector3,
    pub(crate) maxs: Vector3,
}

impl AABB {
    /// Initializes an AABB from a mesh
    /// Fails if the mesh has no vertices
    pub(crate) fn from_mesh(mesh: &MeshData) -> Result<Self, ErrorType> {
        // Sanity check
        if mesh.vertices.is_empty() {
            log_error!("Can't create an AABB from an empty mesh");
            return Err(ErrorType::DoesNotExist);
        }

        let mut mins = Vector3::INFINITY;
        let mut maxs = Vector3::NEG_INFINITY;

        for vertex in &mesh.vertices {
            let position = vertex.position.0;
            mins.x = mins.x.min(position.x);
            mins.y = mins.y.min(position.y);
            mins.z = mins.z.min(position.z);
            maxs.x = maxs.x.max(position.x);
            maxs.y = maxs.y.max(position.y);
            maxs.z = maxs.z.max(position.z);
        }

        Ok(AABB { mins, maxs })
    }

    /// Gets the AABB in world space
    pub(crate) fn as_world(&mut self, model: &Matrix4x4) {
        macro_rules! transform {
            ($var:expr) => {{
                let new_var = vec4($var.x, $var.y, $var.z, 1.);
                $var = (model * new_var).from_homogeneous();
            }};
        }

        transform!(self.mins);
        transform!(self.maxs);
    }

    /// Gets the AABB in view space
    pub(crate) fn as_view(&mut self, view: &Matrix4x4) {
        macro_rules! transform {
            ($var:expr) => {{
                let new_var = vec4($var.x, $var.y, $var.z, 1.);
                $var = (view * new_var).from_homogeneous();
            }};
        }

        transform!(self.mins);
        transform!(self.maxs);
    }

    /// Gets all 8 points of the AABB
    pub(crate) fn get_points(&self) -> [Vector3; 8] {
        [
            vec3(self.mins.x, self.mins.y, self.mins.z),
            vec3(self.mins.x, self.mins.y, self.maxs.z),
            vec3(self.mins.x, self.maxs.y, self.mins.z),
            vec3(self.mins.x, self.maxs.y, self.maxs.z),
            vec3(self.maxs.x, self.mins.y, self.mins.z),
            vec3(self.maxs.x, self.mins.y, self.maxs.z),
            vec3(self.maxs.x, self.maxs.y, self.mins.z),
            vec3(self.maxs.x, self.maxs.y, self.maxs.z),
        ]
    }
}
