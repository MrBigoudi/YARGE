#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{maths::Vector3, renderer_types::PrimitiveTopology, rendering_layer::buffer::BufferId};

pub(crate) struct MeshData {
    /// A list of object space vertex coordinates
    pub(crate) vertices: Vec<Vector3>,
    /// A list of indices if the mesh reuses indices
    pub(crate) indices: Option<Vec<usize>>,
    /// The topology of the primitives in the mesh
    pub(crate) topology: PrimitiveTopology,
    /// The GPU data
    pub(crate) gpu_data: MeshGpuData,
}

pub(crate) struct MeshGpuData {
    /// The VertexBuffer for the positions
    pub(crate) positions_buffer: Option<BufferId>,
    /// The VertexBuffer for the colors
    pub(crate) colors_buffer: Option<BufferId>,
    /// The VertexBuffer for the normals
    pub(crate) normals_buffer: Option<BufferId>,
    /// The VertexBuffer for the texture coordinates
    pub(crate) texture_coordinates_buffer: Option<BufferId>,
}
