#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{
    Event, Query, With,
    core_layer::application_system::ecs::engine::{
        camera::CameraComponent, is_activated::IsActivatedComponent, mesh::MeshComponent,
        transform::TransformComponent,
    },
    rendering_layer::bounding_volumes::AABB,
};
use std::collections::VecDeque;

#[crate::system]
pub(crate) fn culling_system(
    resource_manager: &mut crate::ResourceManager,
    q1: Query<'_, '_, (&mut CameraComponent, &TransformComponent), With<IsActivatedComponent>>,
    q2: Query<'_, '_, (&MeshComponent, &TransformComponent), With<IsActivatedComponent>>,
) -> Result<VecDeque<Event>, ErrorType> {
    for (camera, camera_transform) in &q1 {
        camera.visible_entities.clear();
        let frustum = camera.get_view_space_frustum();
        'all_entities_loop: for ((mesh, mesh_transform), entity) in q2.with_entities() {
            // TODO: add other types of mesh file loader
            // TODO: add indices in meshComponent to point to the correct mesh in the ObjFile or store meshes and materials as resources
            let obj_file = match resource_manager.try_get::<crate::ObjFile>(&mesh.resource_file) {
                Ok(None) => continue 'all_entities_loop,
                Ok(Some(resource)) => resource.clone(),
                Err(err) => {
                    log_error!(
                        "Failed to load a mesh resource in the engine culling system: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            };
            // TODO: handle multiple meshes in obj file
            let aabb = match AABB::from_mesh(&obj_file.meshes[0]) {
                Ok(mut aabb) => {
                    aabb.as_world(&mesh_transform.get_model());
                    aabb.as_view(&camera.get_view(camera_transform));
                    aabb
                }
                Err(err) => {
                    log_error!(
                        "Failed to build an AABB in the engine culling system: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            };
            if frustum.intersects(&aabb) {
                camera.visible_entities.push(entity);
            }
        }
    }
    Ok(VecDeque::new())
}
