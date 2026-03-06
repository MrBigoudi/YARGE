use crate::rendering_layer::rendering_impl::types::textures::VkImageLayoutTransition;
#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use super::context::VulkanContext;
use crate::PlatformLayer;
use crate::rendering_layer::rendering_impl::vulkan::init::commands::VkCommands;
use crate::rendering_layer::types::RendererBeginFrameOutput;
use crate::{PlatformLayerImpl, RenderingLayer, config::Config};

pub(crate) struct VulkanRenderingLayer<'a> {
    pub(in crate::rendering_layer::rendering_impl::vulkan) context: VulkanContext<'a>,
}

impl<'a> RenderingLayer<'a> for VulkanRenderingLayer<'a> {
    type RenderingLayerType = VulkanRenderingLayer<'a>;

    fn init(
        config: &Config,
        platform_layer: &mut PlatformLayerImpl,
    ) -> Result<Self::RenderingLayerType, ErrorType> {
        let context = match VulkanContext::init(config, platform_layer) {
            Ok(context) => context,
            Err(err) => {
                log_error!("Failed to initialize the Vulkan context: {:?}", err);
                return Err(ErrorType::Unknown);
            }
        };
        log_info!("Vulkan renderer initialized");
        Ok(VulkanRenderingLayer { context })
    }

    fn shutdown(&mut self) -> Result<(), ErrorType> {
        if let Err(err) = self.context.shutdown() {
            log_error!("Failed to shutdown the Vulkan context: {:?}", err);
            return Err(ErrorType::Unknown);
        }
        log_info!("Vulkan renderer shutted down");
        Ok(())
    }

    fn begin_frame(&mut self) -> Result<RendererBeginFrameOutput, ErrorType> {
        // TODO: temporary code, to move
        let real_path = std::path::PathBuf::from(format!(
            "{}/assets/shaders/compiled/vk_tuto.spv",
            env!("CARGO_MANIFEST_DIR")
        ));
        let shader_code = PlatformLayerImpl::read_to_bytes(&real_path)?;
        let shader_code: Vec<u32> = shader_code
            .chunks_exact(4)
            .map(|b| {
                u32::from_le_bytes(b.try_into().unwrap_or_else(|err| {
                    log_error!(
                        "Failed to get the code from shader file {:?}: {:?}",
                        &real_path,
                        err
                    );
                    panic!();
                }))
            })
            .collect();

        macro_rules! create_shader_module {
            ($code:expr) => {{
                let module_info = ash::vk::ShaderModuleCreateInfo::default().code($code);
                match unsafe {
                    self.context
                        .device_wrapper
                        .device
                        .create_shader_module(&module_info, self.context.allocator.as_ref())
                } {
                    Ok(module) => Ok(module),
                    Err(err) => {
                        log_error!("Failed to create a Vulkan shader module: {:?}", err);
                        Err(ErrorType::Unknown)
                    }
                }
            }};
        }

        let shader_module = create_shader_module!(&shader_code)?;

        let vert_entry_point = std::ffi::CString::new("vert_main").unwrap();
        let vert_shader_info = ash::vk::PipelineShaderStageCreateInfo::default()
            .stage(ash::vk::ShaderStageFlags::VERTEX)
            .module(shader_module)
            .name(&vert_entry_point);
        let frag_entry_point = std::ffi::CString::new("frag_main").unwrap();
        let frag_shader_info = ash::vk::PipelineShaderStageCreateInfo::default()
            .stage(ash::vk::ShaderStageFlags::FRAGMENT)
            .module(shader_module)
            .name(&frag_entry_point);
        let shader_stages = vec![vert_shader_info, frag_shader_info];

        let dynamic_states = vec![
            ash::vk::DynamicState::VIEWPORT,
            ash::vk::DynamicState::SCISSOR,
        ];
        let dynamic_state_info =
            ash::vk::PipelineDynamicStateCreateInfo::default().dynamic_states(&dynamic_states);
        let vertex_input_info = ash::vk::PipelineVertexInputStateCreateInfo::default();
        let input_assembly_info = ash::vk::PipelineInputAssemblyStateCreateInfo::default()
            .topology(ash::vk::PrimitiveTopology::TRIANGLE_LIST);
        let viewport_state_info = ash::vk::PipelineViewportStateCreateInfo::default()
            .viewport_count(1)
            .scissor_count(1);

        let rasterization_state_info = ash::vk::PipelineRasterizationStateCreateInfo::default()
            .depth_clamp_enable(false)
            .polygon_mode(ash::vk::PolygonMode::FILL) // Change for wireframe mode
            .cull_mode(ash::vk::CullModeFlags::BACK)
            .front_face(ash::vk::FrontFace::CLOCKWISE)
            .line_width(1f32);

        let multisample_state_info = ash::vk::PipelineMultisampleStateCreateInfo::default()
            .rasterization_samples(ash::vk::SampleCountFlags::TYPE_1)
            .sample_shading_enable(false);

        let color_blend_attachment = vec![
            ash::vk::PipelineColorBlendAttachmentState::default()
                .color_write_mask(ash::vk::ColorComponentFlags::RGBA)
                .blend_enable(true)
                .src_color_blend_factor(ash::vk::BlendFactor::SRC_ALPHA)
                .dst_color_blend_factor(ash::vk::BlendFactor::ONE_MINUS_SRC_ALPHA)
                .color_blend_op(ash::vk::BlendOp::ADD)
                .src_alpha_blend_factor(ash::vk::BlendFactor::ONE)
                .dst_alpha_blend_factor(ash::vk::BlendFactor::ZERO)
                .alpha_blend_op(ash::vk::BlendOp::ADD),
        ];

        let color_blend_state_info = ash::vk::PipelineColorBlendStateCreateInfo::default()
            .attachments(&color_blend_attachment);

        let layout_info = ash::vk::PipelineLayoutCreateInfo::default();
        let layout = match unsafe {
            self.context
                .device_wrapper
                .device
                .create_pipeline_layout(&layout_info, None)
        } {
            Ok(layout) => layout,
            Err(err) => {
                log_error!("Failed to create a Vulkan pipeline layout: {:?}", err);
                return Err(ErrorType::VulkanError);
            }
        };

        let formats: Vec<ash::vk::Format> = vec![self.context.swapchain_wrapper.images_format];
        let mut rendering_info =
            ash::vk::PipelineRenderingCreateInfo::default().color_attachment_formats(&formats);

        let graphics_pipeline_info = vec![
            ash::vk::GraphicsPipelineCreateInfo::default()
                .stages(&shader_stages)
                .vertex_input_state(&vertex_input_info)
                .input_assembly_state(&input_assembly_info)
                .dynamic_state(&dynamic_state_info)
                .viewport_state(&viewport_state_info)
                .rasterization_state(&rasterization_state_info)
                .multisample_state(&multisample_state_info)
                .color_blend_state(&color_blend_state_info)
                .layout(layout)
                .render_pass(ash::vk::RenderPass::null()) // left null for Dynamic rendering
                .push_next(&mut rendering_info),
        ];

        let graphics_pipeline = match unsafe {
            self.context
                .device_wrapper
                .device
                .create_graphics_pipelines(
                    ash::vk::PipelineCache::null(),
                    &graphics_pipeline_info,
                    None,
                )
        } {
            Ok(pipelines) => pipelines[0],
            Err(err) => {
                log_error!("Failed to create a Vulkan graphics pipeline: {:?}", err);
                return Err(ErrorType::VulkanError);
            }
        };

        let command_buffer = if self.context.command_pool.buffers.is_empty() {
            match self
                .context
                .command_pool
                .create_command_buffer(&self.context.device_wrapper)
            {
                Ok(buffer) => buffer,
                Err(err) => {
                    log_error!(
                        "Failed to create the unique Vulkan command buffer: {:?}",
                        err
                    );
                    return Err(ErrorType::Unknown);
                }
            }
        } else {
            &self.context.command_pool.buffers[0]
        };
        if let Err(err) = VkCommands::begin_record(&self.context.device_wrapper, command_buffer) {
            log_error!(
                "Failed to begin recording the unique Vulkan command buffer: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        let transition_parameters = VkImageLayoutTransition::default()
            .src_layout(ash::vk::ImageLayout::UNDEFINED)
            .dst_layout(ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .dst_access_mask(ash::vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
            .src_stage_mask(ash::vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
            .dst_stage_mask(ash::vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT);
        VkCommands::transition_image_layout(
            &self.context.device_wrapper,
            command_buffer,
            self.context.swapchain_wrapper.images[self.context.image_index],
            &transition_parameters,
        );

        let clear_color = [0.2f32, 0.4f32, 0.9f32, 1f32];
        let clear_color = ash::vk::ClearValue {
            color: ash::vk::ClearColorValue {
                float32: clear_color,
            },
        };
        let atachment_info = vec![
            ash::vk::RenderingAttachmentInfo::default()
                .image_view(self.context.swapchain_wrapper.images_views[self.context.image_index])
                .image_layout(ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
                .load_op(ash::vk::AttachmentLoadOp::CLEAR)
                .store_op(ash::vk::AttachmentStoreOp::STORE)
                .clear_value(clear_color),
        ];
        let rendering_info = ash::vk::RenderingInfo::default()
            .render_area(ash::vk::Rect2D {
                offset: ash::vk::Offset2D::default(),
                extent: self.context.swapchain_wrapper.images_extent,
            })
            .layer_count(1)
            .color_attachments(&atachment_info);
        VkCommands::begin_rendering(
            &self.context.device_wrapper,
            command_buffer,
            &rendering_info,
        );
        VkCommands::bind_pipeline(
            &self.context.device_wrapper,
            command_buffer,
            ash::vk::PipelineBindPoint::GRAPHICS,
            graphics_pipeline,
        );

        let viewport = ash::vk::Viewport::default()
            .x(0f32)
            .y(0f32)
            .width(self.context.swapchain_wrapper.images_extent.width as f32)
            .height(self.context.swapchain_wrapper.images_extent.height as f32)
            .min_depth(0f32)
            .max_depth(1f32);
        let scissor = ash::vk::Rect2D::default()
            .offset(ash::vk::Offset2D::default())
            .extent(self.context.swapchain_wrapper.images_extent);
        VkCommands::set_viewport(&self.context.device_wrapper, command_buffer, viewport);
        VkCommands::set_scissor(&self.context.device_wrapper, command_buffer, scissor);
        VkCommands::draw(&self.context.device_wrapper, command_buffer, 3, 1, 0, 0);
        VkCommands::end_rendering(&self.context.device_wrapper, command_buffer);

        let transition_parameters = VkImageLayoutTransition::default()
            .src_layout(ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .dst_layout(ash::vk::ImageLayout::PRESENT_SRC_KHR)
            .src_access_mask(ash::vk::AccessFlags2::COLOR_ATTACHMENT_WRITE)
            .src_stage_mask(ash::vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT)
            .dst_stage_mask(ash::vk::PipelineStageFlags2::BOTTOM_OF_PIPE);
        VkCommands::transition_image_layout(
            &self.context.device_wrapper,
            command_buffer,
            self.context.swapchain_wrapper.images[self.context.image_index],
            &transition_parameters,
        );
        if let Err(err) = VkCommands::end_record(&self.context.device_wrapper, command_buffer) {
            log_error!(
                "Failed to end recording the unique Vulkan command buffer: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }

        unsafe {
            self.context
                .device_wrapper
                .device
                .destroy_shader_module(shader_module, self.context.allocator.as_ref());
            self.context
                .device_wrapper
                .device
                .destroy_pipeline_layout(layout, self.context.allocator.as_ref());
            self.context
                .device_wrapper
                .device
                .destroy_pipeline(graphics_pipeline, self.context.allocator.as_ref());
        }

        log_error!("Function is not yet implemented");
        Err(ErrorType::NotImplemented)

        // Ok(RendererBeginFrameOutput::Success)
    }

    fn end_frame(&mut self, _platform_layer: &mut PlatformLayerImpl) -> Result<(), ErrorType> {
        log_error!("Function is not yet implemented");
        // TODO: swap buffer and present to screen
        Err(ErrorType::NotImplemented)
    }
}
