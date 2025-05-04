use ash::{vk::{self, ApplicationInfo, InstanceCreateInfo}, Instance};

use crate::{config::Config, error::ErrorType, log, log_error};

/// Helper function to create the application info
fn init_application_info(config: &Config) -> Result<ApplicationInfo, ErrorType> {
    let application_name = match std::ffi::CString::new(config.application_config.name.clone()) {
        Ok(name) => name,
        Err(err) => {
            log_error!("Failed to get the application name when initializing the application info: {:?}", err);
            return Err(ErrorType::Unknown);
        }
    };

    let application_version = vk::make_api_version(
        config.application_config.version.variant,
        config.application_config.version.major,
        config.application_config.version.minor,
        config.application_config.version.patch,
    );

    let engine_name = match std::ffi::CString::new(env!("CARGO_PKG_NAME")) {
        Ok(name) => name,
        Err(err) => {
            log_error!("Failed to get the engine name when initializing the application info: {:?}", err);
            return Err(ErrorType::Unknown);
        }
    };

    let engine_version = env!("CARGO_PKG_VERSION");
    let engine_version: Vec<u32> = engine_version
        .split('.')
        .map(|s| s.parse().unwrap_or(0))
        .collect()
    ;
    let (major, minor, patch) = match engine_version.as_slice() {
        [maj, min, pat, ..] => (*maj, *min, *pat),
        [maj, min] => (*maj, *min, 0),
        [maj] => (*maj, 0, 0),
        _ => (0, 0, 0),
    };
    let engine_version = vk::make_api_version(0, major, minor, patch);

    Ok(ApplicationInfo{
            p_application_name: application_name.as_ptr() as * const std::ffi::c_char,
            p_engine_name: engine_name.as_ptr() as * const std::ffi::c_char,
            ..Default::default()
        }
        .api_version(vk::API_VERSION_1_3)
        .application_version(application_version)
        .engine_version(engine_version)
    )
}

/// Helper function to initialize the instance create info
fn init_instance_create_info<'a>(app_info: &'a ApplicationInfo) -> Result<InstanceCreateInfo<'a>, ErrorType> {
    Ok(InstanceCreateInfo::default()
        .application_info(app_info)
    )
}

pub fn init_instance(config: &Config) -> Result<Instance, ErrorType> {
    let application_info = match init_application_info(config){
        Ok(info) => info,
        Err(err) => {
            log_error!("Failed to create the application info when initializing the instance: {:?}", err);
            return Err(err);
        }
    };

    Err(ErrorType::Unknown)

}