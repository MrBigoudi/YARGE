use crate::{PlatformLayer, PlatformLayerImpl, platform_layer::window::Window};
#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// A wrapper around a Vulkan swapchain
pub(in crate::rendering_layer::rendering_impl::vulkan) struct VkSwapchain {
    /// The swapchain instance
    pub(in crate::rendering_layer::rendering_impl::vulkan) instance: ash::khr::swapchain::Instance,
    /// The swapchain device
    pub(in crate::rendering_layer::rendering_impl::vulkan) device: ash::khr::swapchain::Device,
    /// The actual swapchain
    pub(in crate::rendering_layer::rendering_impl::vulkan) swapchain: ash::vk::SwapchainKHR,
    /// The swapchain images
    pub(in crate::rendering_layer::rendering_impl::vulkan) images: Vec<ash::vk::Image>,
    /// The swapchain images views
    pub(in crate::rendering_layer::rendering_impl::vulkan) images_views: Vec<ash::vk::ImageView>,
    /// The swapchain images format
    pub(in crate::rendering_layer::rendering_impl::vulkan) images_format: ash::vk::Format,
    /// The swapchain images extent
    pub(in crate::rendering_layer::rendering_impl::vulkan) images_extent: ash::vk::Extent2D,
}

/// Gets all the surface capabilities
fn get_surface_capabilities(
    physical_device: &ash::vk::PhysicalDevice,
    surface_wrapper: &super::surface::VkSurface,
) -> Result<ash::vk::SurfaceCapabilitiesKHR, ErrorType> {
    match unsafe {
        surface_wrapper
            .instance
            .get_physical_device_surface_capabilities(*physical_device, surface_wrapper.surface)
    } {
        Ok(capabilities) => Ok(capabilities),
        Err(err) => {
            log_error!(
                "Failed to get the surface capabilities when initializing the Vulkan swapchain: {:?}",
                err
            );
            Err(ErrorType::VulkanError)
        }
    }
}

/// Gets all the available surface formats
fn get_available_surface_formats(
    physical_device: &ash::vk::PhysicalDevice,
    surface_wrapper: &super::surface::VkSurface,
) -> Result<Vec<ash::vk::SurfaceFormatKHR>, ErrorType> {
    match unsafe {
        surface_wrapper
            .instance
            .get_physical_device_surface_formats(*physical_device, surface_wrapper.surface)
    } {
        Ok(formats) => Ok(formats),
        Err(err) => {
            log_error!(
                "Failed to get the available surface formats when initializing the Vulkan swapchain: {:?}",
                err
            );
            Err(ErrorType::VulkanError)
        }
    }
}

/// Gets all the available present modes
fn get_available_present_modes(
    physical_device: &ash::vk::PhysicalDevice,
    surface_wrapper: &super::surface::VkSurface,
) -> Result<Vec<ash::vk::PresentModeKHR>, ErrorType> {
    match unsafe {
        surface_wrapper
            .instance
            .get_physical_device_surface_present_modes(*physical_device, surface_wrapper.surface)
    } {
        Ok(present_modes) => Ok(present_modes),
        Err(err) => {
            log_error!(
                "Failed to get the available surface present modes when initializing the Vulkan swapchain: {:?}",
                err
            );
            Err(ErrorType::VulkanError)
        }
    }
}

/// Selects the best surface format
fn select_surface_format(
    config: &crate::config::Config,
    available_formats: &[ash::vk::SurfaceFormatKHR],
) -> Result<ash::vk::SurfaceFormatKHR, ErrorType> {
    let mut best_format = None;
    let mut index_in_prefered = None;

    for format in available_formats {
        // Default selection
        if best_format.is_none() {
            best_format = Some(format);
        }

        // Check if user specified a prefered format
        'inner_loop: for (index, prefered) in config
            .renderer_config
            .vulkan_parameters
            .prefered_swapchain_formats
            .iter()
            .enumerate()
        {
            if prefered.as_ash() == format.format {
                // Found best
                if index == 0 {
                    return Ok(*format);
                }
                // Found better
                if index_in_prefered.is_none() || index_in_prefered.unwrap() > index {
                    best_format = Some(format);
                    index_in_prefered = Some(index);
                }
                break 'inner_loop;
            }
        }
    }

    match best_format {
        Some(format) => Ok(*format),
        None => {
            log_error!("Failed to find a suitable swapchain format");
            Err(ErrorType::DoesNotExist)
        }
    }
}

/// Selects the best present mode
/// Default to FIFO
fn select_present_mode(
    config: &crate::config::Config,
    available_present_modes: &[ash::vk::PresentModeKHR],
) -> ash::vk::PresentModeKHR {
    let mut best_mode = ash::vk::PresentModeKHR::FIFO;
    let mut index_in_prefered = None;

    for mode in available_present_modes {
        // Check if user specified a prefered mode
        'inner_loop: for (index, prefered) in config
            .renderer_config
            .vulkan_parameters
            .prefered_swapchain_present_modes
            .iter()
            .enumerate()
        {
            if prefered.as_ash() == *mode {
                // Found best
                if index == 0 {
                    return *mode;
                }
                // Found better
                if index_in_prefered.is_none() || index_in_prefered.unwrap() > index {
                    best_mode = *mode;
                    index_in_prefered = Some(index);
                }
                break 'inner_loop;
            }
        }
    }

    best_mode
}

/// Selects the best extent
fn select_extent(
    platform_layer_impl: &PlatformLayerImpl,
    capabilities: &ash::vk::SurfaceCapabilitiesKHR,
) -> ash::vk::Extent2D {
    if capabilities.current_extent.width != u32::MAX {
        return capabilities.current_extent;
    }

    let width = platform_layer_impl
        .get_window_ref(0)
        .get_framebuffer_width() as u32;
    let height = platform_layer_impl
        .get_window_ref(0)
        .get_framebuffer_height() as u32;

    ash::vk::Extent2D {
        width: width.clamp(
            capabilities.min_image_extent.width,
            capabilities.max_image_extent.width,
        ),
        height: height.clamp(
            capabilities.min_image_extent.height,
            capabilities.max_image_extent.height,
        ),
    }
}

/// Selects the best minimum image count
fn select_min_image_count(
    config: &crate::config::Config,
    capabilities: &ash::vk::SurfaceCapabilitiesKHR,
) -> u32 {
    // At least `min_image_count' images in the swapchain
    let min_image_count = config
        .renderer_config
        .vulkan_parameters
        .prefered_swapchain_min_image_count;
    let min_image_count = std::cmp::max(min_image_count, capabilities.min_image_count + 1);
    // If can't take at least `min_image_count', take the max image count
    if capabilities.max_image_count > 0 && min_image_count < capabilities.max_image_count {
        capabilities.max_image_count
    } else {
        min_image_count
    }
}

/// Selects the best number of image array layers
fn select_image_array_layers(config: &crate::config::Config) -> u32 {
    match config.renderer_config.application_type {
        crate::renderer_types::RenderingApplicationType::Stereoscopic3D => 2u32,
        _ => 1u32,
    }
}

/// Selects the image usage
fn select_image_usage(
    config: &crate::config::Config,
    capabilities: &ash::vk::SurfaceCapabilitiesKHR,
) -> Result<ash::vk::ImageUsageFlags, ErrorType> {
    let mut flags = ash::vk::ImageUsageFlags::empty();
    for usage in &config
        .renderer_config
        .vulkan_parameters
        .swapchain_image_usages
    {
        let flag = usage.as_ash();
        if capabilities.supported_usage_flags.contains(flag) {
            flags |= usage.as_ash();
        } else {
            log_error!(
                "The Vulkan physical device doesn't support the wanted `{:?}' image usage",
                flag
            );
            return Err(ErrorType::NotSupported);
        }
    }

    Ok(flags)
}

/// Selects the final transform before presentation
fn select_pre_transform(
    config: &crate::config::Config,
    capabilities: &ash::vk::SurfaceCapabilitiesKHR,
) -> Result<ash::vk::SurfaceTransformFlagsKHR, ErrorType> {
    let wanted = config.renderer_config.final_transform.as_ash();
    if !capabilities.supported_transforms.contains(wanted) {
        log_error!(
            "The Vulkan physical device doesn't support the wanted `{:?}' pre transform",
            wanted
        );
        Err(ErrorType::NotSupported)
    } else {
        Ok(wanted)
    }
}

/// Initializes the Vulkan swapchain
#[allow(clippy::too_many_arguments)]
pub(in crate::rendering_layer::rendering_impl::vulkan) fn init_swapchain(
    config: &crate::config::Config,
    platform_layer_impl: &PlatformLayerImpl,
    entry: &ash::Entry,
    instance: &ash::Instance,
    physical_device: &ash::vk::PhysicalDevice,
    device_wrapper: &super::device::VkDevice,
    surface_wrapper: &super::surface::VkSurface,
    allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
) -> Result<VkSwapchain, ErrorType> {
    log_info!("Swapchain info:");
    let surface_capabilities = get_surface_capabilities(physical_device, surface_wrapper)?;
    log_info!("\tCapabilities: {:?}", surface_capabilities);
    let available_formats = get_available_surface_formats(physical_device, surface_wrapper)?;
    log_info!("\tAvailable formats:");
    for format in &available_formats {
        log_info!("\t\t- {:?}", format);
    }
    let available_present_modes = get_available_present_modes(physical_device, surface_wrapper)?;
    log_info!("\tAvailable present modes:");
    for mode in &available_present_modes {
        log_info!("\t\t- {:?}", mode);
    }

    log_info!("Swapchain selected:");
    let best_surface_format = match select_surface_format(config, &available_formats) {
        Ok(format) => format,
        Err(err) => {
            log_error!(
                "Failed to select a surface format when initializing the Vulkan swapchain: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };
    log_info!("\t- {:?}", best_surface_format);
    let best_present_mode = select_present_mode(config, &available_present_modes);
    log_info!("\t- {:?}", best_present_mode);
    let best_extent = select_extent(platform_layer_impl, &surface_capabilities);
    log_info!("\t- {:?}", best_extent);
    let min_image_count = select_min_image_count(config, &surface_capabilities);
    log_info!("\t- Image count: {:?}", min_image_count);
    let image_array_layers = select_image_array_layers(config);
    log_info!("\t- Image array layers: {:?}", image_array_layers);
    let image_usage = match select_image_usage(config, &surface_capabilities) {
        Ok(usage) => {
            log_info!("\t- Image usage:");
            for usage in &config
                .renderer_config
                .vulkan_parameters
                .swapchain_image_usages
            {
                log_info!("\t\t- {:?}", usage);
            }
            usage
        }
        Err(err) => {
            log_error!("Failed to select the swapchain image usage: {:?}", err);
            return Err(ErrorType::Unknown);
        }
    };
    let pre_transform = match select_pre_transform(config, &surface_capabilities) {
        Ok(transform) => {
            log_info!("\t- Pre transform: {:?}", &transform);
            transform
        }
        Err(_) => {
            let current_transform = surface_capabilities.current_transform;
            log_warn!("\t- Pre transform default to: {:?}", current_transform);
            current_transform
        }
    };

    let swapchain_info = ash::vk::SwapchainCreateInfoKHR::default()
        .surface(surface_wrapper.surface)
        .image_format(best_surface_format.format)
        .image_color_space(best_surface_format.color_space)
        .present_mode(best_present_mode)
        .image_extent(best_extent)
        .min_image_count(min_image_count)
        .image_array_layers(image_array_layers)
        .image_usage(image_usage)
        .pre_transform(pre_transform)
        // TODO: Change that
        .image_sharing_mode(ash::vk::SharingMode::EXCLUSIVE)
        .composite_alpha(ash::vk::CompositeAlphaFlagsKHR::OPAQUE)
        .clipped(true)
        .old_swapchain(ash::vk::SwapchainKHR::null());

    let swapchain_instance = ash::khr::swapchain::Instance::new(entry, instance);
    let swapchain_device = ash::khr::swapchain::Device::new(instance, &device_wrapper.device);
    let swapchain = match unsafe { swapchain_device.create_swapchain(&swapchain_info, allocator) } {
        Ok(swapchain) => swapchain,
        Err(err) => {
            log_error!("Failed to create the Vulkan swapchain: {:?}", err);
            return Err(ErrorType::VulkanError);
        }
    };

    let images = match unsafe { swapchain_device.get_swapchain_images(swapchain) } {
        Ok(images) => images,
        Err(err) => {
            log_error!(
                "Failed to get the images when initializing the Vulkan swapchain: {:?}",
                err
            );
            return Err(ErrorType::VulkanError);
        }
    };
    let nb_images = images.len();
    let images_views = Vec::with_capacity(nb_images);

    let mut new_swapchain = VkSwapchain {
        instance: swapchain_instance,
        device: swapchain_device,
        swapchain,
        images,
        images_views,
        images_format: best_surface_format.format,
        images_extent: best_extent,
    };

    if let Err(err) = new_swapchain.create_image_views(device_wrapper, allocator) {
        log_error!("Failed to create the image views when initializing the Vulkan swapchain: {:?}", err);
        return Err(ErrorType::Unknown);
    }

    log_info!("Vulkan swapchain initialized");
    Ok(new_swapchain)
}

/// Shuts down the Vulkan swapchain
pub(in crate::rendering_layer::rendering_impl::vulkan) fn shutdown_swapchain(
    device_wrapper: &super::device::VkDevice,
    swapchain_wrapper: &VkSwapchain,
    allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
) {
    for view in &swapchain_wrapper.images_views {
        unsafe { device_wrapper.device.destroy_image_view(*view, allocator) };
    }

    unsafe {
        swapchain_wrapper
            .device
            .destroy_swapchain(swapchain_wrapper.swapchain, allocator)
    };
    log_info!("Vulkan swapchain shutted down");
}


impl VkSwapchain {
    /// Creates the image views
    pub(in crate::rendering_layer::rendering_impl::vulkan) fn create_image_views(&mut self, device_wrapper: &super::device::VkDevice, allocator: Option<&ash::vk::AllocationCallbacks<'_>>) -> Result<(), ErrorType> {
        self.images_views.clear();

        // TODO: Change for different application type (AR / VR)
        let view_info = ash::vk::ImageViewCreateInfo::default()
            .view_type(ash::vk::ImageViewType::TYPE_2D)
            .format(self.images_format)
            .subresource_range(ash::vk::ImageSubresourceRange {
                aspect_mask: ash::vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            })
            .components(ash::vk::ComponentMapping::default()
                .r(ash::vk::ComponentSwizzle::IDENTITY)
                .g(ash::vk::ComponentSwizzle::IDENTITY)
                .b(ash::vk::ComponentSwizzle::IDENTITY)
                .a(ash::vk::ComponentSwizzle::IDENTITY)
            )
        ;

        for image in &self.images {
            let view_info = view_info.image(*image);
            let new_view = match unsafe { device_wrapper.device.create_image_view(&view_info, allocator) } {
                Ok(view) => view,
                Err(err) => {
                    log_error!("Failed to create the Vulkan swapchain image views: {:?}", err);
                    return Err(ErrorType::VulkanError);
                },
            };
            self.images_views.push(new_view);
        }

        Ok(())
    }
}