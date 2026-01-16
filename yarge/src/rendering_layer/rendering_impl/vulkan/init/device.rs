#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{config::Config, rendering_layer::rendering_impl::types::{extensions::VkExtension, features::{VkFeatures10, VkFeatures11, VkFeatures12, VkFeatures13}}};

/// A structure representing a Vulkan device and its associated queues
pub(crate) struct VkDevice {
    /// The logical device
    pub(crate) device: ash::Device,
    /// The associated queue families
    pub(crate) queue_families: VkQueueFamilies,
    /// The associated queues
    pub(crate) queues: VkQueues,
}

/// A structrue representing Vulkan queues
pub(crate) struct VkQueues {
    /// The queues supporting graphics commands
    pub(crate) graphics: Vec<ash::vk::Queue>,
    /// The queues supporting compute commands
    pub(crate) compute: Vec<ash::vk::Queue>,
    /// The queues supporting transfer commands
    pub(crate) transfer: Vec<ash::vk::Queue>,
}

impl VkQueues {
    /// Creates the needed queues from the device
    fn new(device: &ash::Device, queue_families: &VkQueueFamilies) -> Result<Self, ErrorType> {
        let graphics: Vec<ash::vk::Queue> = (0..queue_families.graphics.count)
            .map(|queue_index| unsafe { 
                device.get_device_queue(
                    queue_families.graphics.index as u32, 
                    queue_index as u32
                ) 
            })
            .collect()
        ;
        if graphics.is_empty() {
            log_error!("Failed to get at least a graphics queue from the Vulkan device");
            return Err(ErrorType::DoesNotExist);
        }

        let compute: Vec<ash::vk::Queue> = (0..queue_families.compute.count)
            .map(|queue_index| unsafe { 
                device.get_device_queue(
                    queue_families.compute.index as u32, 
                    queue_index as u32
                ) 
            })
            .collect()
        ;
        if compute.is_empty() {
            log_error!("Failed to get at least a compute queue from the Vulkan device");
            return Err(ErrorType::DoesNotExist);
        }

        let transfer: Vec<ash::vk::Queue> = (0..queue_families.transfer.count)
            .map(|queue_index| unsafe { 
                device.get_device_queue(
                    queue_families.transfer.index as u32, 
                    queue_index as u32
                ) 
            })
            .collect()
        ;
        if transfer.is_empty() {
            log_error!("Failed to get at least a transfer queue from the Vulkan device");
            return Err(ErrorType::DoesNotExist);
        }

        Ok(Self {
            graphics,
            compute,
            transfer,
        })
    } 
}


/// A structrue representing Vulkan queue families
#[derive(Debug, Clone, Copy)]
pub(crate) struct VkQueueFamilies {
    /// The queue family supporting graphics commands
    pub(crate) graphics: VkQueueFamily,
    /// The queue family supporting compute commands
    pub(crate) compute: VkQueueFamily,
    /// The queue family supporting transfer commands
    pub(crate) transfer: VkQueueFamily,
}

impl VkQueueFamilies {
    /// Creates the needed queue families from the properties
    fn new(queue_family_properties: &[ash::vk::QueueFamilyProperties]) -> Result<Self, ErrorType> {
        let graphics = match VkQueueFamily::new_graphics(&queue_family_properties) {
            Ok(index) => index,
            Err(err) => {
                log_error!("Failed to create a graphics queue family when initializing the Vulkan queues: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };
        let compute = match VkQueueFamily::new_compute(&queue_family_properties) {
            Ok(index) => index,
            Err(err) => {
                log_error!("Failed to create a compute queue family when initializing the Vulkan queues: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };
        let transfer = match VkQueueFamily::new_transfer(&queue_family_properties) {
            Ok(index) => index,
            Err(err) => {
                log_error!("Failed to creeate a transfer queue family when initializing the Vulkan queues: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };

        log_info!("Device queues: ");
        log_info!("\t- Graphics: {:?}", graphics);
        log_info!("\t- Compute: {:?}", compute);
        log_info!("\t- Transfer: {:?}", transfer);

        Ok(Self {
            graphics,
            compute,
            transfer,
        })
    }

}

/// A structure representing a single queue
#[derive(Debug, Clone, Copy)]
pub(crate) struct VkQueueFamily {
    /// The index in the queue family
    pub(crate) index: usize,
    /// The number of queues in the family
    pub(crate) count: usize,
    /// The priority of the queue
    pub(crate) priority: f32,
}

/// The priority for the graphics queue
const GRAPHICS_QUEUE_PRIORITY: f32 = 0.;
/// The priority for the compute queue
const COMPUTE_QUEUE_PRIORITY: f32 = 0.5;
/// The priority for the transfer queue
const TRANSFER_QUEUE_PRIORITY: f32 = 1.;

impl VkQueueFamily {
    /// Creates a queue capable of handling graphics commands
    /// If a queue is only able to handle graphics commands it will be prefered
    fn new_graphics(queue_family_properties: &[ash::vk::QueueFamilyProperties]) -> Result<Self, ErrorType> {
        let mut graphics_index = None;
        let mut count = 0;
        'outter_loop: for (index, property) in queue_family_properties.iter().enumerate() {
            if property.queue_flags.intersects(ash::vk::QueueFlags::GRAPHICS){
                if graphics_index.is_none() {
                    graphics_index = Some(index);
                    count = property.queue_count as usize;
                } else {
                    if !property.queue_flags.intersects(ash::vk::QueueFlags::COMPUTE){
                        graphics_index = Some(index);
                        count = property.queue_count as usize;
                    }
                    if !property.queue_flags.intersects(ash::vk::QueueFlags::TRANSFER){
                        graphics_index = Some(index);
                        count = property.queue_count as usize;
                    }
                    if !property.queue_flags.intersects(ash::vk::QueueFlags::COMPUTE)
                        && !property.queue_flags.intersects(ash::vk::QueueFlags::TRANSFER) {
                            graphics_index = Some(index);
                            count = property.queue_count as usize;
                            break 'outter_loop;
                        }

                }
            }
        };
        match graphics_index {
            None => {
                log_error!("Failed to find a queue supporting graphics commands");
                Err(ErrorType::DoesNotExist)
            },
            Some(index) => Ok(Self {
                index,
                count,
                priority: GRAPHICS_QUEUE_PRIORITY,
            })
        }
    }


    /// Creates a queue capable of handling compute commands
    /// If a queue is only able to handle compute commands it will be prefered
    fn new_compute(queue_family_properties: &[ash::vk::QueueFamilyProperties]) -> Result<Self, ErrorType> {
        let mut compute_index = None;
        let mut count = 0;
        'outter_loop: for (index, property) in queue_family_properties.iter().enumerate() {
            if property.queue_flags.intersects(ash::vk::QueueFlags::COMPUTE){
                if compute_index.is_none() {
                    compute_index = Some(index);
                    count = property.queue_count as usize;
                } else {
                    if !property.queue_flags.intersects(ash::vk::QueueFlags::GRAPHICS){
                        compute_index = Some(index);
                        count = property.queue_count as usize;
                    }
                    if !property.queue_flags.intersects(ash::vk::QueueFlags::TRANSFER){
                        compute_index = Some(index);
                        count = property.queue_count as usize;
                    }
                    if !property.queue_flags.intersects(ash::vk::QueueFlags::GRAPHICS)
                        && !property.queue_flags.intersects(ash::vk::QueueFlags::TRANSFER) {
                            compute_index = Some(index);
                            count = property.queue_count as usize;
                            break 'outter_loop;
                        }
                }
            }
        };
        match compute_index {
            None => {
                log_error!("Failed to find a queue supporting compute commands");
                Err(ErrorType::DoesNotExist)
            },
            Some(index) => Ok(Self {
                index,
                count,
                priority: COMPUTE_QUEUE_PRIORITY,
            })
        }
    }


    /// Creates a queue capable of handling transfer commands
    /// If a queue is only able to handle transfer commands it will be prefered
    fn new_transfer(queue_family_properties: &[ash::vk::QueueFamilyProperties]) -> Result<Self, ErrorType> {
        let mut transfer_index = None;
        let mut count = 0;
        'outter_loop: for (index, property) in queue_family_properties.iter().enumerate() {
            if property.queue_flags.intersects(ash::vk::QueueFlags::TRANSFER){
                if transfer_index.is_none() {
                    transfer_index = Some(index);
                    count = property.queue_count as usize;
                } else {
                    if !property.queue_flags.intersects(ash::vk::QueueFlags::GRAPHICS){
                        transfer_index = Some(index);
                        count = property.queue_count as usize;
                    }
                    if !property.queue_flags.intersects(ash::vk::QueueFlags::COMPUTE){
                        transfer_index = Some(index);
                        count = property.queue_count as usize;
                    }
                    if !property.queue_flags.intersects(ash::vk::QueueFlags::COMPUTE)
                        && !property.queue_flags.intersects(ash::vk::QueueFlags::GRAPHICS) {
                            transfer_index = Some(index);
                            count = property.queue_count as usize;
                            break 'outter_loop;
                        }
                }
            }
        };
        match transfer_index {
            None => {
                log_error!("Failed to find a queue supporting transfer commands");
                Err(ErrorType::DoesNotExist)
            },
            Some(index) => Ok(Self {
                index,
                count,
                priority: TRANSFER_QUEUE_PRIORITY,
            })
        }
    }
}


/// Initializes the logical device
pub(crate) fn init_device(config: &Config, instance: &ash::Instance, physical_device: &ash::vk::PhysicalDevice, allocator: Option<&ash::vk::AllocationCallbacks<'_>>) -> Result<VkDevice, ErrorType> {
    // Queues
    let queue_family_properties = unsafe { instance.get_physical_device_queue_family_properties(*physical_device) };
    log_info!("queue family properties: {:?}", queue_family_properties);
    let queue_families = match VkQueueFamilies::new(&queue_family_properties) {
        Ok(queue_families) => queue_families,
        Err(err) => {
            log_error!("Failed to create the queue families when initializing Vulkan device: {:?}", err);
            return Err(ErrorType::Unknown);
        }
    };
    let graphics_queue_priorities = vec![queue_families.graphics.priority; queue_families.graphics.count];
    let compute_queue_priorities = vec![queue_families.compute.priority; queue_families.compute.count];
    let transfer_queue_priorities = vec![queue_families.transfer.priority; queue_families.transfer.count];
    let mut device_queue_info = vec![ash::vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_families.graphics.index as u32)
        .queue_priorities(&graphics_queue_priorities)
    ];
    if queue_families.compute.index != queue_families.graphics.index {
        device_queue_info.push(ash::vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_families.compute.index as u32)
            .queue_priorities(&compute_queue_priorities)
        );
    }
    if queue_families.transfer.index != queue_families.graphics.index && queue_families.transfer.index != queue_families.compute.index {
        device_queue_info.push(ash::vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_families.transfer.index as u32)
            .queue_priorities(&transfer_queue_priorities)
        );
    }

    // Extensions
    let required_extensions = match VkExtension::to_vknames(
        &config.renderer_config.vulkan_parameters.required_device_extensions
    ) {
        Err(err) => {
            log_error!("Failed to get the required device extensions names when initializing Vulkan device: {:?}", err);
            return Err(ErrorType::Unknown);
        },
        Ok(extensions) => extensions,
    };


    // Features
    let required_features_1_0 = VkFeatures10::as_ash_features(
        &config.renderer_config.vulkan_parameters.required_physical_device_features_1_0
    );
    let mut required_features_1_1 = VkFeatures11::as_ash_features(
        &config.renderer_config.vulkan_parameters.required_physical_device_features_1_1
    );
    let mut required_features_1_2 = VkFeatures12::as_ash_features(
        &config.renderer_config.vulkan_parameters.required_physical_device_features_1_2
    );
    let mut required_features_1_3 = VkFeatures13::as_ash_features(
        &config.renderer_config.vulkan_parameters.required_physical_device_features_1_3
    );
    // TODO: Update to ash 0.39 and Vulkan 1.4
    // let required_features_1_4 = VkFeatures14::as_ash_features(
    //     &config.renderer_config.vulkan_parameters.required_physical_device_features_1_4
    // );
    let mut required_features = ash::vk::PhysicalDeviceFeatures2::default()
        .features(required_features_1_0)
        .push_next(&mut required_features_1_1)
        .push_next(&mut required_features_1_2)
        .push_next(&mut required_features_1_3)
        // TODO: Update to ash 0.39 and Vulkan 1.4
        // .push_next(&mut required_features_1_4)
    ;

    let device_info = ash::vk::DeviceCreateInfo::default()
        .queue_create_infos(&device_queue_info)
        .enabled_extension_names(&required_extensions.names)
        .push_next(&mut required_features)
    ;

    let device = match unsafe { instance.create_device(*physical_device, &device_info, allocator) }{
        Ok(device) => device,
        Err(err) => {
            log_error!("Failed to create a device: {:?}", err);
            return Err(ErrorType::VulkanError);
        }
    };

    let queues = match VkQueues::new(&device, &queue_families) {
        Ok(queues) => queues,
        Err(err) => {
            log_error!("Failed to get the queues when initializing the Vulkan device: {:?}", err);
            return Err(ErrorType::Unknown);
        }
    };

    log_info!("Vulkan device initialized");
    Ok(VkDevice {
        device,
        queue_families,
        queues,
    })
    
}