#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{ResourceId, core_layer::application_system::ecs::component::Component};

pub(crate) struct MeshComponent {
    pub(crate) resource_file: ResourceId,
}
impl Component for MeshComponent {}
