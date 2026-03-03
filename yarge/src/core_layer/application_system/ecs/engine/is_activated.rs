#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::core_layer::application_system::ecs::component::Component;

pub(crate) struct IsActivatedComponent {}
impl Component for IsActivatedComponent {}
