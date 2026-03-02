#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{Resource, ResourceLoadingParameters, rendering_layer::mesh::MeshData};

#[derive(Debug, Clone)]
/// Parsed asset data
pub struct ObjFile {
    /// The meshes in the obj
    pub(crate) meshes: Vec<MeshData>,
    // TODO: add the materials
}
impl Resource for ObjFile {}

impl ResourceLoadingParameters<ObjFile> for std::path::PathBuf {
    fn load_resource(&self) -> Result<ObjFile, ErrorType> {
        let (models, materials) = match tobj::load_obj(self, &tobj::GPU_LOAD_OPTIONS){
            Ok((models, materials)) => (models, materials),
            Err(err) => {
                log_error!("Failed to load the {:?} obj: {:?}", self, err);
                return Err(ErrorType::IO);
            }
        };

        let materials = match materials {
            Ok(materials) => materials,
            Err(err) => {
                log_error!("Failed to load the {:?} obj materials: {:?}", self, err);
                return Err(ErrorType::IO);
            }
        };

        log_warn!("# of models: {}", models.len());
        log_warn!("# of materials: {}", materials.len());

        for (i, m) in models.iter().enumerate() {
            let mesh = &m.mesh;

            log_warn!("model[{}].name = \'{}\'", i, m.name);
            log_warn!("model[{}].mesh.material_id = {:?}", i, mesh.material_id);

            log_warn!(
                "Size of model[{}].face_arities: {}",
                i,
                mesh.face_arities.len()
            );

            let mut next_face = 0;
            for f in 0..mesh.face_arities.len() {
                let end = next_face + mesh.face_arities[f] as usize;
                let face_indices: Vec<_> = mesh.indices[next_face..end].iter().collect();
                log_warn!("    face[{}] = {:?}", f, face_indices);
                next_face = end;
            }

            // Normals and texture coordinates are also loaded, but not printed in this example
            log_warn!("model[{}].vertices: {}", i, mesh.positions.len() / 3);

            assert!(mesh.positions.len() % 3 == 0);
            for v in 0..mesh.positions.len() / 3 {
                log_warn!(
                    "    v[{}] = ({}, {}, {})",
                    v,
                    mesh.positions[3 * v],
                    mesh.positions[3 * v + 1],
                    mesh.positions[3 * v + 2]
                );
            }
        }

        for (i, m) in materials.iter().enumerate() {
            log_warn!("material[{}].name = \'{}\'", i, m.name);
            if let Some(ambient) = m.ambient {
                log_warn!(
                    "    material.Ka = ({}, {}, {})",
                    ambient[0], ambient[1], ambient[2]
                );
            }
            if let Some(diffuse) = m.diffuse {
                log_warn!(
                    "    material.Kd = ({}, {}, {})",
                    diffuse[0], diffuse[1], diffuse[2]
                );
            }
            if let Some(specular) = m.specular {
                log_warn!(
                    "    material.Ks = ({}, {}, {})",
                    specular[0], specular[1], specular[2]
                );
            }
            if let Some(shininess) = m.shininess {
                log_warn!("    material.Ns = {}", shininess);
            }
            if let Some(dissolve) = m.dissolve {
                log_warn!("    material.d = {}", dissolve);
            }
            if let Some(ambient_texture) = &m.ambient_texture {
                log_warn!("    material.map_Ka = {}", ambient_texture);
            }
            if let Some(diffuse_texture) = &m.diffuse_texture {
                log_warn!("    material.map_Kd = {}", diffuse_texture);
            }
            if let Some(specular_texture) = &m.specular_texture {
                log_warn!("    material.map_Ks = {}", specular_texture);
            }
            if let Some(shininess_texture) = &m.shininess_texture {
                log_warn!("    material.map_Ns = {}", shininess_texture);
            }
            if let Some(normal_texture) = &m.normal_texture {
                log_warn!("    material.map_Bump = {}", normal_texture);
            }
            if let Some(dissolve_texture) = &m.dissolve_texture {
                log_warn!("    material.map_d = {}", dissolve_texture);
            }

            for (k, v) in &m.unknown_param {
                log_warn!("    material.{} = {}", k, v);
            }
        }

        Ok(ObjFile {
            meshes: vec![],
        })
    }
}
