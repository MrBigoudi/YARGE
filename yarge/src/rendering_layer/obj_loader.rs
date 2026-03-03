#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{
    Resource, ResourceLoadingParameters,
    maths::{Vector3, vec2, vec3, vec4},
    renderer_types::PrimitiveTopology,
    rendering_layer::{mesh::MeshData, vertex::VertexData},
};

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
        let (models, materials) = match tobj::load_obj(self, &tobj::GPU_LOAD_OPTIONS) {
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

        log_warn!("Loading {:?}:", self);
        log_warn!("\t# of models: {}", models.len());
        log_warn!("\t# of materials: {}", materials.len());

        let mut meshes = Vec::with_capacity(models.len());

        // Create the data for each mesh in the obj
        for model in models {
            let mesh = &model.mesh;

            if mesh.positions.len() % 3 != 0 {
                log_error!("Unsupported mesh format in `{:?}' obj file", self);
                return Err(ErrorType::NotSupported);
            }

            let nb_vertices = mesh.positions.len() / 3;
            let mut vertices = Vec::with_capacity(nb_vertices);
            for v in 0..nb_vertices {
                let mut vertex = VertexData::default().position(vec3(
                    mesh.positions[3 * v],
                    mesh.positions[3 * v + 1],
                    mesh.positions[3 * v + 2],
                ));
                if !mesh.normals.is_empty() {
                    vertex = vertex.normal(vec3(
                        mesh.normals[3 * v],
                        mesh.normals[3 * v + 1],
                        mesh.normals[3 * v + 2],
                    ));
                }
                if !mesh.vertex_color.is_empty() {
                    vertex = vertex.color(vec4(
                        mesh.vertex_color[3 * v],
                        mesh.vertex_color[3 * v + 1],
                        mesh.vertex_color[3 * v + 2],
                        1.,
                    ));
                }
                if !mesh.texcoords.is_empty() {
                    vertex = vertex.texture_coordinates(vec2(
                        mesh.texcoords[2 * v],
                        mesh.texcoords[2 * v + 1],
                    ));
                }
                vertices.push(vertex);
            }

            let indices: Vec<usize> = mesh.indices.iter().map(|elem| *elem as usize).collect();

            // Compute default normals
            if mesh.normals.is_empty() {
                let nb_triangles = indices.len() / 3;
                for tri in 0..nb_triangles {
                    let p0_idx = indices[3 * tri];
                    let p0 = vertices[p0_idx].position.0;
                    let p1_idx = indices[3 * tri + 1];
                    let p1 = vertices[p1_idx].position.0;
                    let p2_idx = indices[3 * tri + 2];
                    let p2 = vertices[p2_idx].position.0;

                    let edge_1 = (p1 - p0).normalize()?;
                    let edge_2 = (p2 - p0).normalize()?;
                    let normal = Vector3::cross(&edge_1, &edge_2).normalize()?;

                    vertices[p0_idx].normal.0 = normal;
                    vertices[p1_idx].normal.0 = normal;
                    vertices[p2_idx].normal.0 = normal;
                }
            }

            let new_mesh = MeshData {
                vertices,
                indices: Some(indices),
                topology: PrimitiveTopology::TriangleList,
            };

            meshes.push(new_mesh);
        }

        // for (i, m) in materials.iter().enumerate() {
        //     log_warn!("material[{}].name = \'{}\'", i, m.name);
        //     if let Some(ambient) = m.ambient {
        //         log_warn!(
        //             "    material.Ka = ({}, {}, {})",
        //             ambient[0], ambient[1], ambient[2]
        //         );
        //     }
        //     if let Some(diffuse) = m.diffuse {
        //         log_warn!(
        //             "    material.Kd = ({}, {}, {})",
        //             diffuse[0], diffuse[1], diffuse[2]
        //         );
        //     }
        //     if let Some(specular) = m.specular {
        //         log_warn!(
        //             "    material.Ks = ({}, {}, {})",
        //             specular[0], specular[1], specular[2]
        //         );
        //     }
        //     if let Some(shininess) = m.shininess {
        //         log_warn!("    material.Ns = {}", shininess);
        //     }
        //     if let Some(dissolve) = m.dissolve {
        //         log_warn!("    material.d = {}", dissolve);
        //     }
        //     if let Some(ambient_texture) = &m.ambient_texture {
        //         log_warn!("    material.map_Ka = {}", ambient_texture);
        //     }
        //     if let Some(diffuse_texture) = &m.diffuse_texture {
        //         log_warn!("    material.map_Kd = {}", diffuse_texture);
        //     }
        //     if let Some(specular_texture) = &m.specular_texture {
        //         log_warn!("    material.map_Ks = {}", specular_texture);
        //     }
        //     if let Some(shininess_texture) = &m.shininess_texture {
        //         log_warn!("    material.map_Ns = {}", shininess_texture);
        //     }
        //     if let Some(normal_texture) = &m.normal_texture {
        //         log_warn!("    material.map_Bump = {}", normal_texture);
        //     }
        //     if let Some(dissolve_texture) = &m.dissolve_texture {
        //         log_warn!("    material.map_d = {}", dissolve_texture);
        //     }

        //     for (k, v) in &m.unknown_param {
        //         log_warn!("    material.{} = {}", k, v);
        //     }
        // }

        Ok(ObjFile { meshes })
    }
}
