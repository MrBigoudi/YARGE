#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::rendering_layer::rendering_impl::types::textures::VkImageLayoutTransition;

/// A structure representing a Vulkan command pool and its associated buffers
pub(in crate::rendering_layer::rendering_impl::vulkan) struct VkCommandPool {
    /// The command pool
    pub(in crate::rendering_layer::rendering_impl::vulkan) pool: ash::vk::CommandPool,
    /// The associated command buffers
    pub(in crate::rendering_layer::rendering_impl::vulkan) buffers: Vec<ash::vk::CommandBuffer>,

    // TODO: remove the temporary sync structures
    pub(in crate::rendering_layer::rendering_impl::vulkan) present_complete_semaphore: ash::vk::Semaphore,
    pub(in crate::rendering_layer::rendering_impl::vulkan) render_finished_semaphore: ash::vk::Semaphore,
    pub(in crate::rendering_layer::rendering_impl::vulkan) draw_fence: ash::vk::Fence,
}

/// Helper function to initiate the Vulkan command pool
pub(in crate::rendering_layer::rendering_impl::vulkan) fn init_command_pool(
    device_wrapper: &super::device::VkDevice,
    allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
) -> Result<VkCommandPool, ErrorType> {
    let pool_info = ash::vk::CommandPoolCreateInfo::default()
        .flags(ash::vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
        .queue_family_index(device_wrapper.queue_families.graphics.index as u32);

    match unsafe {
        device_wrapper
            .device
            .create_command_pool(&pool_info, allocator)
    } {
        Ok(pool) => {
            log_info!("Vulkan command pool initialized");

            // TODO: remove the temporary sync structures
            let semaphore_info = ash::vk::SemaphoreCreateInfo::default();
            let fence_info = ash::vk::FenceCreateInfo::default()
                .flags(ash::vk::FenceCreateFlags::SIGNALED)
            ;
            let present_complete_semaphore = match unsafe {device_wrapper.device.create_semaphore(&semaphore_info, allocator)}{
                Ok(semaphore) => semaphore,
                Err(err) => {
                    log_error!("Failed to create the Vulkan present complete semaphore: {:?}", err);
                    return Err(ErrorType::VulkanError);
                }
            };
            let render_finished_semaphore = match unsafe {device_wrapper.device.create_semaphore(&semaphore_info, allocator)}{
                Ok(semaphore) => semaphore,
                Err(err) => {
                    log_error!("Failed to create the Vulkan render finished semaphore: {:?}", err);
                    return Err(ErrorType::VulkanError);
                }
            };
            let draw_fence = match unsafe {device_wrapper.device.create_fence(&fence_info, allocator)}{
                Ok(fence) => fence,
                Err(err) => {
                    log_error!("Failed to create the Vulkan draw fence: {:?}", err);
                    return Err(ErrorType::VulkanError);
                }
            };

            Ok(VkCommandPool {
                pool,
                buffers: vec![],
                present_complete_semaphore,
                render_finished_semaphore,
                draw_fence,
            })
        }
        Err(err) => {
            log_error!("Failed to initialize the Vulkan command pool: {:?}", err);
            Err(ErrorType::VulkanError)
        }
    }
}

/// Shuts down the Vulkan command pool
pub(in crate::rendering_layer::rendering_impl::vulkan) fn shutdown_command_pool(
    device_wrapper: &super::device::VkDevice,
    command_pool: &mut VkCommandPool,
    allocator: Option<&ash::vk::AllocationCallbacks<'_>>,
) {
    unsafe {
        device_wrapper.device.destroy_semaphore(command_pool.present_complete_semaphore, allocator);
        device_wrapper.device.destroy_semaphore(command_pool.render_finished_semaphore, allocator);
        device_wrapper.device.destroy_fence(command_pool.draw_fence, allocator);

        device_wrapper
            .device
            .destroy_command_pool(command_pool.pool, allocator);
    }
    command_pool.buffers = vec![];
    log_info!("Vulkan command pool shutted down");
}

impl VkCommandPool {
    /// Allocates a single command buffer
    pub(in crate::rendering_layer::rendering_impl::vulkan) fn create_command_buffer(
        &mut self,
        device_wrapper: &super::device::VkDevice,
    ) -> Result<&ash::vk::CommandBuffer, ErrorType> {
        let allocate_info = ash::vk::CommandBufferAllocateInfo::default()
            .command_pool(self.pool)
            .level(ash::vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        match unsafe {
            device_wrapper
                .device
                .allocate_command_buffers(&allocate_info)
        } {
            Ok(buffers) => {
                let nb_new_buffers = buffers.len();
                if nb_new_buffers != 1 {
                    log_error!(
                        "Wrong number of Vulkan command buffers created: expected 1 got {:?}",
                        nb_new_buffers
                    );
                    return Err(ErrorType::VulkanError);
                }
                self.buffers.push(buffers[0]);
                Ok(self.buffers.last().unwrap())
            }
            Err(err) => {
                log_error!("Failed to create a Vulkan command buffer: {:?}", err);
                Err(ErrorType::VulkanError)
            }
        }
    }
}

pub(in crate::rendering_layer::rendering_impl::vulkan) struct VkCommands;

impl VkCommands {
    /// Begins recording a command buffer
    #[inline(always)]
    pub(in crate::rendering_layer::rendering_impl::vulkan) fn begin_record(
        device_wrapper: &super::device::VkDevice,
        command_buffer: &ash::vk::CommandBuffer,
    ) -> Result<(), ErrorType> {
        let begin_info = ash::vk::CommandBufferBeginInfo::default();
        match unsafe {
            device_wrapper
                .device
                .begin_command_buffer(*command_buffer, &begin_info)
        } {
            Ok(()) => Ok(()),
            Err(err) => {
                log_error!(
                    "Failed to begin the record of a Vulkan command buffer: {:?}",
                    err
                );
                Err(ErrorType::VulkanError)
            }
        }
    }

    /// Ends recording a command buffer
    #[inline(always)]
    pub(in crate::rendering_layer::rendering_impl::vulkan) fn end_record(
        device_wrapper: &super::device::VkDevice,
        command_buffer: &ash::vk::CommandBuffer,
    ) -> Result<(), ErrorType> {
        match unsafe { device_wrapper.device.end_command_buffer(*command_buffer) } {
            Ok(()) => Ok(()),
            Err(err) => {
                log_error!(
                    "Failed to end the record of a Vulkan command buffer: {:?}",
                    err
                );
                Err(ErrorType::VulkanError)
            }
        }
    }

    /// Records a transition image layout command in the given command buffer for the given image
    #[inline(always)]
    pub(in crate::rendering_layer::rendering_impl::vulkan) fn transition_image_layout(
        device_wrapper: &super::device::VkDevice,
        command_buffer: &ash::vk::CommandBuffer,
        image: ash::vk::Image,
        transition_parameters: &VkImageLayoutTransition,
    ) {
        let barriers = vec![
            ash::vk::ImageMemoryBarrier2::default()
                .src_stage_mask(transition_parameters.src_stage_mask)
                .dst_stage_mask(transition_parameters.dst_stage_mask)
                .src_access_mask(transition_parameters.src_access_mask)
                .dst_access_mask(transition_parameters.dst_access_mask)
                .old_layout(transition_parameters.src_layout)
                .new_layout(transition_parameters.dst_layout)
                .src_queue_family_index(ash::vk::QUEUE_FAMILY_IGNORED)
                .dst_queue_family_index(ash::vk::QUEUE_FAMILY_IGNORED)
                .image(image)
                .subresource_range(transition_parameters.subresource_range),
        ];

        let dependency_info = ash::vk::DependencyInfo::default().image_memory_barriers(&barriers);

        unsafe {
            device_wrapper
                .device
                .cmd_pipeline_barrier2(*command_buffer, &dependency_info);
        }
    }

    /// Records a begin rendering command in the given command buffer
    #[inline(always)]
    pub(in crate::rendering_layer::rendering_impl::vulkan) fn begin_rendering(
        device_wrapper: &super::device::VkDevice,
        command_buffer: &ash::vk::CommandBuffer,
        rendering_info: &ash::vk::RenderingInfo<'_>,
    ) {
        unsafe {
            device_wrapper
                .device
                .cmd_begin_rendering(*command_buffer, rendering_info);
        }
    }

    /// Records a end rendering command in the given command buffer
    #[inline(always)]
    pub(in crate::rendering_layer::rendering_impl::vulkan) fn end_rendering(
        device_wrapper: &super::device::VkDevice,
        command_buffer: &ash::vk::CommandBuffer,
    ) {
        unsafe {
            device_wrapper.device.cmd_end_rendering(*command_buffer);
        }
    }

    /// Records a bind pipeline command
    #[inline(always)]
    pub(in crate::rendering_layer::rendering_impl::vulkan) fn bind_pipeline(
        device_wrapper: &super::device::VkDevice,
        command_buffer: &ash::vk::CommandBuffer,
        bind_poind: ash::vk::PipelineBindPoint,
        pipeline: ash::vk::Pipeline,
    ) {
        unsafe {
            device_wrapper
                .device
                .cmd_bind_pipeline(*command_buffer, bind_poind, pipeline);
        }
    }

    /// Records a set viewport command
    #[inline(always)]
    pub(in crate::rendering_layer::rendering_impl::vulkan) fn set_viewport(
        device_wrapper: &super::device::VkDevice,
        command_buffer: &ash::vk::CommandBuffer,
        viewport: ash::vk::Viewport,
    ) {
        let viewports = vec![viewport];
        unsafe {
            device_wrapper
                .device
                .cmd_set_viewport(*command_buffer, 0, &viewports)
        }
    }

    /// Records a set scissor command
    #[inline(always)]
    pub(in crate::rendering_layer::rendering_impl::vulkan) fn set_scissor(
        device_wrapper: &super::device::VkDevice,
        command_buffer: &ash::vk::CommandBuffer,
        scissor: ash::vk::Rect2D,
    ) {
        let scissors = vec![scissor];
        unsafe {
            device_wrapper
                .device
                .cmd_set_scissor(*command_buffer, 0, &scissors)
        }
    }

    /// Records a draw command
    #[inline(always)]
    pub(in crate::rendering_layer::rendering_impl::vulkan) fn draw(
        device_wrapper: &super::device::VkDevice,
        command_buffer: &ash::vk::CommandBuffer,
        vertex_count: usize,
        first_vertex: usize,
        instance_count: usize,
        first_instance: usize,
    ) {
        unsafe {
            device_wrapper.device.cmd_draw(
                *command_buffer,
                vertex_count as u32,
                instance_count as u32,
                first_vertex as u32,
                first_instance as u32,
            )
        }
    }
}
