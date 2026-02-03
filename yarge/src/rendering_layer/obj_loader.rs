#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{Resource, ResourceLoadingParameters, rendering_layer::mesh::MeshData};

#[derive(Debug, Clone)]
/// Parsed asset data
pub(crate) struct ObjFile {
    /// The meshes in the obj
    pub(crate) meshes: Vec<MeshData>,
    // TODO: add the materials
}
impl Resource for ObjFile {}

impl ResourceLoadingParameters<ObjFile> for std::path::PathBuf {
    fn load_resource(&self) -> Result<ObjFile, ErrorType> {
        todo!()
    }
}
