#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{
    Event, Query, With,
    core_layer::application_system::ecs::engine::{
        camera::CameraComponent, is_activated::IsActivatedComponent, mesh::MeshComponent,
        transform::TransformComponent,
    },
};
use std::collections::VecDeque;

#[crate::system]
pub(crate) fn culling_system(
    resource_manager: &mut crate::ResourceManager,
    q1: Query<'_, '_, (&mut CameraComponent, &TransformComponent), With<IsActivatedComponent>>,
    q2: Query<'_, '_, (&MeshComponent, &TransformComponent), With<IsActivatedComponent>>,
) -> Result<VecDeque<Event>, ErrorType> {
    for (camera, transform) in &q1 {
        camera.visible_entities.clear();
        let frustum = camera.get_world_space_frustum(transform);
        'all_entities_loop: for ((mesh, transform), entity) in q2.with_entities() {
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
            let aabb = todo!();
            // if frustum.intersects(aabb, transform) {
            //     camera.visible_entities.push(entity);
            // }
        }
    }
    Ok(VecDeque::new())
}
