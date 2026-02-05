#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::ResourceId;

pub(crate) struct MeshComponent {
    pub(crate) resource_file: ResourceId,
}
