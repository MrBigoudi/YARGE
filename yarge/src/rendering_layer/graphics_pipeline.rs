#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

// fn CullingSystem(
//     q1: Query<(&CameraComponent, &TransformComponent), With<IsActivated>>,
//     q2: Query<(&MeshComponent, &TransformComponent, &mut CameraVisibilityComponent)>,
// ) -> Result<(), ErrorType>
// {
//     for ((camera, transform), entity) in &q1.with_entities() {
//         let frustum = camera.get_frustum(transform);
//         for (mesh, transform, mut visibility) in &mut q2 {
//             if frustum.intersects(mesh.get_aabb(transform)) {
//                 visibility[entity] = true;
//             }
//         }
//     }
//     Ok(())
// }
