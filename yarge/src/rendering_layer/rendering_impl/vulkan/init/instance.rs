#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use ash::{
    Entry, Instance,
    vk::{self, ApplicationInfo, InstanceCreateInfo},
};

use crate::{
    PlatformLayerImpl,
    config::{Config, Version},
    platform_layer::window::Window,
    rendering_layer::rendering_impl::types::{
        VkNames, extensions::VkInstanceExtensions, layers::VkLayers,
    },
};

/// Helper function to create the application info
fn init_application_info<'a>(
    config: &'a Config,
    entry: &'a Entry,
) -> Result<ApplicationInfo<'a>, ErrorType> {
    // Get user game info
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

    // Get engine info
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

    // Get Vulkan info
    let vk_config = &config.renderer_config.vulkan_parameters;
    let supported_version = match unsafe { entry.try_enumerate_instance_version() } {
        Ok(Some(version)) => Version::new(
            vk::api_version_variant(version) as u8,
            vk::api_version_major(version) as u8,
            vk::api_version_minor(version) as u8,
            vk::api_version_patch(version) as u8,
        ),
        Ok(None) => Version::new(0, 1, 0, 0),
        Err(err) => {
            log_error!(
                "Failed to enumerate the supported Vulkan instance version: {:?}",
                err
            );
            return Err(ErrorType::VulkanError);
        }
    };
    if vk_config.version > supported_version {
        log_error!(
            "Invalid Vulkan version requested: requested version `{:?}', while maximum version supported is `{:?}'",
            vk_config.version.as_string(),
            supported_version.as_string()
        );
        return Err(ErrorType::Unknown);
    };
    let api_version = vk::make_api_version(
        vk_config.version.variant as u32,
        vk_config.version.major as u32,
        vk_config.version.minor as u32,
        vk_config.version.patch as u32,
    );

    // Bulid the application info
    let application_info = ApplicationInfo {
        p_application_name: application_name.as_ptr() as *const std::ffi::c_char,
        p_engine_name: engine_name.as_ptr() as *const std::ffi::c_char,
        ..Default::default()
    }
    .api_version(api_version)
    .application_version(application_version)
    .engine_version(engine_version);

    Ok(application_info)
}

/// Helper function to fetch the required layers
/// Checks if all required layers are available
fn get_required_layers(
    config: &Config,
    entry: &Entry,
) -> Result<(Vec<VkLayers>, VkNames), ErrorType> {
    let vk_config = &config.renderer_config.vulkan_parameters;
    let required_layers: Vec<VkLayers> = [
        vk_config.required_layers.clone(),
        #[cfg(debug_assertions)]
        vk_config.required_layers_debug.clone(),
    ]
    .concat();
    let required_layers_names = match VkLayers::to_vknames(&required_layers) {
        Ok(names) => names,
        Err(err) => {
            log_error!(
                "Failed to convert required Vulkan layers to vknames: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    // Display required layers
    log_info!("Required Vulkan layers:");
    for (layer_index, layer_name) in required_layers_names.names_cstrings.iter().enumerate() {
        log_info!("\t- Layer: {:?}", layer_name);
        let layer_settings = match required_layers[layer_index].get_settings() {
            Ok(settings) => settings,
            Err(err) => {
                log_error!(
                    "Failed to get the settings for the `{:?}' layer: {:?}",
                    layer_name,
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        for setting in layer_settings {
            log_info!("\t\t- Setting: {:?}", setting.setting_name);
        }
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
    log_info!("Available Vulkan layers:");
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

    for required in &required_layers_names.names_cstrings {
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

    Ok((required_layers, required_layers_names))
}

/// Helper function to fetch the required instance extensions
/// Checks if all required extensions are available
fn get_required_extensions(
    config: &Config,
    entry: &Entry,
    platform_layer: &PlatformLayerImpl,
) -> Result<(Vec<VkInstanceExtensions>, VkNames), ErrorType> {
    let mut required_extensions = match platform_layer
        .window
        .vulkan_get_required_instance_extensions()
    {
        Ok(extensions) => extensions,
        Err(err) => {
            log_error!(
                "Failed to get the required instance extension from the platform extension when initializing the Vulkan instance create info: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };
    required_extensions.extend(config.renderer_config.vulkan_parameters.required_instance_extensions.clone());
    #[cfg(debug_assertions)]
    required_extensions.extend(config.renderer_config.vulkan_parameters.required_instance_extensions_debug.clone());

    let required_extensions_names = match VkInstanceExtensions::to_vknames(&required_extensions) {
        Ok(names) => names,
        Err(err) => {
            log_error!(
                "Failed to convert required Vulkan extensions to vknames: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    // Display required extensions
    log_info!("Required Vulkan instance extensions:");
    for extension_name in &required_extensions_names.names_cstrings {
        log_info!("\t- Extension: {:?}", extension_name);
    }

    let available_extensions = unsafe {
        match entry.enumerate_instance_extension_properties(None) {
            Ok(extensions) => extensions,
            Err(err) => {
                log_error!(
                    "Failed to enumerate the available extensions properties when querying the required extensions: {:?}",
                    err
                );
                return Err(ErrorType::VulkanError);
            }
        }
    };

    // Display available extensions
    log_info!("Available Vulkan instance extensions:");
    for extension in &available_extensions {
        match extension.extension_name_as_c_str() {
            Ok(extension_cstr) => match extension_cstr.to_str() {
                Ok(extension_name) => log_info!("\t- Extension: {}", extension_name),
                Err(err) => {
                    log_error!("Invalid UTF-8 in available extension name: {:?}", err);
                    return Err(ErrorType::IO);
                }
            },
            Err(err) => {
                log_error!(
                    "Invalid string format in available extension name: {:?}",
                    err
                );
                return Err(ErrorType::IO);
            }
        }
    }

    for required in &required_extensions_names.names_cstrings {
        // Check the name of the required extension
        let mut is_available = false;
        'inner: for available in &available_extensions {
            let name = match available.extension_name_as_c_str() {
                Ok(name) => name,
                Err(err) => {
                    log_error!(
                        "Failed to fetch the extension name when querying the required extensions: {:?}",
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
                "Failed to find the {:?} extension when querying the required extensions",
                required
            );
            return Err(ErrorType::VulkanError);
        }
    }

    Ok((required_extensions, required_extensions_names))
}

pub(crate) fn init_instance(
    config: &Config,
    entry: &Entry,
    platform_layer: &PlatformLayerImpl,
    allocator: Option<&vk::AllocationCallbacks<'_>>,
) -> Result<Instance, ErrorType> {
    // Create application
    let application_info = match init_application_info(config, entry) {
        Ok(info) => info,
        Err(err) => {
            log_error!(
                "Failed to create the application info when initializing the instance: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    // Create instance extensions
    let (_required_extensions, required_extensions_names) = match get_required_extensions(
        config,
        entry,
        platform_layer,
    ) {
        Ok(extensions) => extensions,
        Err(err) => {
            log_error!(
                "Failed to get the required instance extensions when initializing the instance: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    // Create instance layers
    let (required_layers, required_layers_names) = match get_required_layers(config, entry) {
        Ok(layers) => layers,
        Err(err) => {
            log_error!(
                "Failed to get the required layers when initializing the instance: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };
    let mut layers_settings = Vec::new();
    for layer in required_layers {
        let settings = match layer.get_settings() {
            Ok(settings) => settings,
            Err(err) => {
                log_error!(
                    "Failed to get the `{:?}' layer settings when initializing the Vulkan instance: {:?}",
                    layer.as_string(),
                    err
                );
                return Err(ErrorType::Unknown);
            }
        };
        layers_settings.extend(settings);
    }
    let layers_settings_ext: Vec<vk::LayerSettingEXT<'_>> = layers_settings
        .iter()
        .map(|setting| setting.as_vk_validation_setting_ext())
        .collect();
    let mut layers_settings_info =
        vk::LayerSettingsCreateInfoEXT::default().settings(&layers_settings_ext);

    // Create the instance
    let instance_info = InstanceCreateInfo::default()
        .application_info(&application_info)
        .enabled_extension_names(&required_extensions_names.names)
        .enabled_layer_names(&required_layers_names.names)
        .push_next(&mut layers_settings_info);

    let instance = match unsafe { entry.create_instance(&instance_info, allocator) } {
        Ok(instance) => instance,
        Err(err) => {
            log_error!("Failed to create the Vulkan instance: {:?}", err);
            return Err(ErrorType::VulkanError);
        }
    };

    log_info!("Vulkan instance initialized");
    Ok(instance)
}
