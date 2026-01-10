#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use ash::{
    Entry, Instance,
    vk::{self, ApplicationInfo, InstanceCreateInfo},
};

use crate::{
    config::Config,
    rendering_layer::rendering_impl::{VkLayers, vulkan::types::VkNames},
};

/// Helper function to create the application info
fn init_application_info(config: &'_ Config) -> Result<ApplicationInfo<'_>, ErrorType> {
    let application_name = match std::ffi::CString::new(config.application_config.name.clone()) {
        Ok(name) => name,
        Err(err) => {
            log_error!(
                "Failed to get the application name when initializing the application info: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    let application_version = vk::make_api_version(
        config.application_config.version.variant as u32,
        config.application_config.version.major as u32,
        config.application_config.version.minor as u32,
        config.application_config.version.patch as u32,
    );

    let engine_name = match std::ffi::CString::new(env!("CARGO_PKG_NAME")) {
        Ok(name) => name,
        Err(err) => {
            log_error!(
                "Failed to get the engine name when initializing the application info: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    let engine_version = env!("CARGO_PKG_VERSION");
    let engine_version: Vec<u32> = engine_version
        .split('.')
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    let (major, minor, patch) = match engine_version.as_slice() {
        [maj, min, pat, ..] => (*maj, *min, *pat),
        [maj, min] => (*maj, *min, 0),
        [maj] => (*maj, 0, 0),
        _ => (0, 0, 0),
    };
    let engine_version = vk::make_api_version(0, major, minor, patch);

    Ok(ApplicationInfo {
        p_application_name: application_name.as_ptr() as *const std::ffi::c_char,
        p_engine_name: engine_name.as_ptr() as *const std::ffi::c_char,
        ..Default::default()
    }
    .api_version(vk::API_VERSION_1_3)
    .application_version(application_version)
    .engine_version(engine_version))
}

/// Helper function to fetch the required layers
/// Checks if all required layers are available
fn get_required_layers(config: &Config, entry: &Entry) -> Result<VkNames, ErrorType> {
    let required_layers = match VkLayers::to_vknames(&config.renderer_config.vulkan_required_layers)
    {
        Ok(names) => names,
        Err(err) => {
            log_error!(
                "Failed to convert required vulkan layers to vknames: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    // Display required layers
    log_info!("Required layers:");
    for layer_name in &required_layers.names_cstrings {
        log_info!("\t- Layer: {:?}", layer_name);
    }

    let available_layers = unsafe {
        match entry.enumerate_instance_layer_properties() {
            Ok(layers) => layers,
            Err(err) => {
                log_error!(
                    "Failed to enumerate the available layers properties when querying the required layers: {:?}",
                    err
                );
                return Err(ErrorType::VulkanError);
            }
        }
    };

    // Display available layers
    log_info!("Available layers:");
    for layer in &available_layers {
        match layer.layer_name_as_c_str() {
            Ok(layer_cstr) => match layer_cstr.to_str() {
                Ok(layer_name) => log_info!("\t- Layer: {}", layer_name),
                Err(err) => {
                    log_error!("Invalid UTF-8 in available layer name: {:?}", err);
                    return Err(ErrorType::IO);
                }
            },
            Err(err) => {
                log_error!("Invalid string format in available layer name: {:?}", err);
                return Err(ErrorType::IO);
            }
        }
    }

    for required in &required_layers.names_cstrings {
        // Check the name of the required layer
        let mut is_available = false;
        'inner: for available in &available_layers {
            let name = match available.layer_name_as_c_str() {
                Ok(name) => name,
                Err(err) => {
                    log_error!(
                        "Failed to fetch the layer name when querying the required layers: {:?}",
                        err
                    );
                    return Err(ErrorType::VulkanError);
                }
            };
            if name == required {
                is_available = true;
                break 'inner;
            }
        }
        if !is_available {
            log_error!(
                "Failed to find the {:?} layer when querying the required layers",
                required
            );
            return Err(ErrorType::VulkanError);
        }
    }

    Ok(required_layers)
}

/// Helper function to initialize the instance create info
fn init_instance_create_info<'a>(
    app_info: &'a ApplicationInfo,
) -> Result<InstanceCreateInfo<'a>, ErrorType> {
    Ok(InstanceCreateInfo::default().application_info(app_info))
}

pub fn init_instance(config: &Config, entry: &Entry) -> Result<Instance, ErrorType> {
    let application_info = match init_application_info(config) {
        Ok(info) => info,
        Err(err) => {
            log_error!(
                "Failed to create the application info when initializing the instance: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    let _required_layers = match get_required_layers(config, entry) {
        Ok(layers) => layers,
        Err(err) => {
            log_error!(
                "Failed to get the required layers when initializing the instance: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    let _instance_info = init_instance_create_info(&application_info);

    Err(ErrorType::Unknown)
}
