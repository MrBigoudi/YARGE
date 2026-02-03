#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{ResourceId, rendering_layer::mesh::MeshData};

pub(crate) struct MeshComponent {
    pub(crate) data: Option<MeshData>,
    pub(crate) resource_file: ResourceId,
}
