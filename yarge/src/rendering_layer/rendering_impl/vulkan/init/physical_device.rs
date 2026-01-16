#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{
    config::{Config, Version},
    rendering_layer::rendering_impl::types::extensions::{VkDeviceExtensions, VkExtension},
};

/// A comparison strategy to rate lists in physical device scores
#[derive(Debug, Clone)]
enum VkPhysicalDeviceScoreComparisonStrategy {
    /// Lexicographical comparison (first difference wins)
    #[allow(unused)]
    Lexicographical,
    /// Counts how many elements are greater than the other and returns Ordering based on majority
    #[allow(unused)]
    MostWins,
}

impl VkPhysicalDeviceScoreComparisonStrategy {
    /// Counts how many elements are greater than the other
    /// Returns Ordering based on majority
    fn compare_by_majority(a: &[u32], b: &[u32]) -> std::cmp::Ordering {
        let mut a_wins = 0;
        let mut b_wins = 0;

        for (&va, &vb) in a.iter().zip(b.iter()) {
            if va > vb {
                a_wins += 1;
            } else if vb > va {
                b_wins += 1;
            }
        }

        a_wins.cmp(&b_wins)
    }

    /// Lexicographical comparison (first difference wins)
    fn compare_lexicographically(a: &[u32], b: &[u32]) -> std::cmp::Ordering {
        for (&va, &vb) in a.iter().zip(b.iter()) {
            match va.cmp(&vb) {
                std::cmp::Ordering::Equal => {}
                ord => return ord,
            }
        }
        std::cmp::Ordering::Equal
    }

    /// Equality helper
    fn equal_lists(a: &[u32], b: &[u32]) -> bool {
        a.len() == b.len() && a.iter().zip(b.iter()).all(|(a, b)| a == b)
    }

    /// Use the correct comparison method based on the strategy
    fn comp_lists(&self, a: &[u32], b: &[u32]) -> std::cmp::Ordering {
        match self {
            VkPhysicalDeviceScoreComparisonStrategy::Lexicographical => {
                Self::compare_lexicographically(a, b)
            }
            VkPhysicalDeviceScoreComparisonStrategy::MostWins => Self::compare_by_majority(a, b),
        }
    }
}

/// A structure to rate physical device
#[derive(Debug, Clone)]
struct VkPhysicalDeviceScore {
    /// The comparison strategy
    comparison_strategy: VkPhysicalDeviceScoreComparisonStrategy,
    /// The API version
    version: Version,
    /// The GPU type
    device_type: ash::vk::PhysicalDeviceType,
    /// The queue properties
    queues: Vec<u32>,
    /// The limits
    limits: Vec<u32>,
    /// The 1.1 properties
    properties_1_1: Vec<u32>,
    /// The 1.2 properties
    properties_1_2: Vec<u32>,
    /// The 1.3 properties
    properties_1_3: Vec<u32>,
    // /// The 1.4 properties
    // properties_1_4: Vec<u32>,
}

impl VkPhysicalDeviceScore {
    /// Gets the queues as an ordered list of u32
    fn get_queues(queues: &Vec<ash::vk::QueueFamilyProperties>) -> Vec<u32> {
        vec![
            queues.len() as u32,
            queues.iter().any(|q|q.queue_flags.intersects(ash::vk::QueueFlags::GRAPHICS)) as u32,
            queues.iter().any(|q|q.queue_flags.intersects(ash::vk::QueueFlags::COMPUTE)) as u32,
            queues.iter().any(|q|q.queue_flags.intersects(ash::vk::QueueFlags::TRANSFER)) as u32,
            queues.iter().any(|q|q.queue_flags.intersects(ash::vk::QueueFlags::SPARSE_BINDING)) as u32,
            queues.iter().filter(|q|q.queue_flags.intersects(ash::vk::QueueFlags::GRAPHICS)).collect::<Vec<_>>().len() as u32,
            queues.iter().filter(|q|q.queue_flags.intersects(ash::vk::QueueFlags::COMPUTE)).collect::<Vec<_>>().len() as u32,
            queues.iter().filter(|q|q.queue_flags.intersects(ash::vk::QueueFlags::TRANSFER)).collect::<Vec<_>>().len() as u32,
            queues.iter().filter(|q|q.queue_flags.intersects(ash::vk::QueueFlags::SPARSE_BINDING)).collect::<Vec<_>>().len() as u32,
        ]  
    }

    /// Gets the limits as an ordered list of u32
    fn get_limits(limits: &ash::vk::PhysicalDeviceLimits) -> Vec<u32> {
        vec![
            limits.max_image_dimension1_d,
            limits.max_image_dimension2_d,
            limits.max_image_dimension3_d,
            limits.max_image_dimension_cube,
            limits.max_image_array_layers,
            limits.max_texel_buffer_elements,
            limits.max_uniform_buffer_range,
            limits.max_storage_buffer_range,
            limits.max_push_constants_size,
            limits.max_memory_allocation_count,
            limits.max_sampler_allocation_count,
            limits.buffer_image_granularity as u32,
            limits.sparse_address_space_size as u32,
            limits.max_bound_descriptor_sets,
            limits.max_per_stage_descriptor_samplers,
            limits.max_per_stage_descriptor_uniform_buffers,
            limits.max_per_stage_descriptor_storage_buffers,
            limits.max_per_stage_descriptor_sampled_images,
            limits.max_per_stage_descriptor_storage_images,
            limits.max_per_stage_descriptor_input_attachments,
            limits.max_per_stage_resources,
            limits.max_descriptor_set_samplers,
            limits.max_descriptor_set_uniform_buffers,
            limits.max_descriptor_set_uniform_buffers_dynamic,
            limits.max_descriptor_set_storage_buffers,
            limits.max_descriptor_set_storage_buffers_dynamic,
            limits.max_descriptor_set_sampled_images,
            limits.max_descriptor_set_storage_images,
            limits.max_descriptor_set_input_attachments,
            limits.max_vertex_input_attributes,
            limits.max_vertex_input_bindings,
            limits.max_vertex_input_attribute_offset,
            limits.max_vertex_input_binding_stride,
            limits.max_vertex_output_components,
            limits.max_tessellation_generation_level,
            limits.max_tessellation_patch_size,
            limits.max_tessellation_control_per_vertex_input_components,
            limits.max_tessellation_control_per_vertex_output_components,
            limits.max_tessellation_control_per_patch_output_components,
            limits.max_tessellation_control_total_output_components,
            limits.max_tessellation_evaluation_input_components,
            limits.max_tessellation_evaluation_output_components,
            limits.max_geometry_shader_invocations,
            limits.max_geometry_input_components,
            limits.max_geometry_output_components,
            limits.max_geometry_output_vertices,
            limits.max_geometry_total_output_components,
            limits.max_fragment_input_components,
            limits.max_fragment_output_attachments,
            limits.max_fragment_dual_src_attachments,
            limits.max_fragment_combined_output_resources,
            limits.max_compute_shared_memory_size,
            limits.max_compute_work_group_count[0],
            limits.max_compute_work_group_count[1],
            limits.max_compute_work_group_count[2],
            limits.max_compute_work_group_invocations,
            limits.max_compute_work_group_size[0],
            limits.max_compute_work_group_size[1],
            limits.max_compute_work_group_size[2],
            limits.sub_pixel_precision_bits,
            limits.sub_texel_precision_bits,
            limits.mipmap_precision_bits,
            limits.max_draw_indexed_index_value,
            limits.max_draw_indirect_count,
            limits.max_sampler_lod_bias as u32,
            limits.max_sampler_anisotropy as u32,
            limits.max_viewports,
            limits.max_viewport_dimensions[0],
            limits.max_viewport_dimensions[1],
            limits.viewport_bounds_range[0] as u32,
            limits.viewport_bounds_range[1] as u32,
            limits.viewport_sub_pixel_bits,
            limits.min_memory_map_alignment as u32,
            limits.min_texel_buffer_offset_alignment as u32,
            limits.min_uniform_buffer_offset_alignment as u32,
            limits.min_storage_buffer_offset_alignment as u32,
            limits.min_texel_offset as u32,
            limits.max_texel_offset,
            limits.min_texel_gather_offset as u32,
            limits.max_texel_gather_offset,
            limits.min_interpolation_offset as u32,
            limits.max_interpolation_offset as u32,
            limits.sub_pixel_interpolation_offset_bits,
            limits.max_framebuffer_width,
            limits.max_framebuffer_height,
            limits.max_framebuffer_layers,
            limits
                .framebuffer_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_1) as u32,
            limits
                .framebuffer_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_2) as u32,
            limits
                .framebuffer_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_4) as u32,
            limits
                .framebuffer_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_8) as u32,
            limits
                .framebuffer_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_16) as u32,
            limits
                .framebuffer_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_32) as u32,
            limits
                .framebuffer_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_64) as u32,
            limits
                .framebuffer_depth_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_1) as u32,
            limits
                .framebuffer_depth_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_2) as u32,
            limits
                .framebuffer_depth_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_4) as u32,
            limits
                .framebuffer_depth_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_8) as u32,
            limits
                .framebuffer_depth_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_16) as u32,
            limits
                .framebuffer_depth_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_32) as u32,
            limits
                .framebuffer_depth_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_64) as u32,
            limits
                .framebuffer_stencil_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_1) as u32,
            limits
                .framebuffer_stencil_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_2) as u32,
            limits
                .framebuffer_stencil_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_4) as u32,
            limits
                .framebuffer_stencil_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_8) as u32,
            limits
                .framebuffer_stencil_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_16) as u32,
            limits
                .framebuffer_stencil_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_32) as u32,
            limits
                .framebuffer_stencil_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_64) as u32,
            limits
                .framebuffer_no_attachments_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_1) as u32,
            limits
                .framebuffer_no_attachments_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_2) as u32,
            limits
                .framebuffer_no_attachments_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_4) as u32,
            limits
                .framebuffer_no_attachments_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_8) as u32,
            limits
                .framebuffer_no_attachments_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_16) as u32,
            limits
                .framebuffer_no_attachments_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_32) as u32,
            limits
                .framebuffer_no_attachments_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_64) as u32,
            limits.max_color_attachments,
            limits
                .sampled_image_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_1) as u32,
            limits
                .sampled_image_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_2) as u32,
            limits
                .sampled_image_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_4) as u32,
            limits
                .sampled_image_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_8) as u32,
            limits
                .sampled_image_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_16) as u32,
            limits
                .sampled_image_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_32) as u32,
            limits
                .sampled_image_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_64) as u32,
            limits
                .sampled_image_integer_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_1) as u32,
            limits
                .sampled_image_integer_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_2) as u32,
            limits
                .sampled_image_integer_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_4) as u32,
            limits
                .sampled_image_integer_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_8) as u32,
            limits
                .sampled_image_integer_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_16) as u32,
            limits
                .sampled_image_integer_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_32) as u32,
            limits
                .sampled_image_integer_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_64) as u32,
            limits
                .sampled_image_depth_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_1) as u32,
            limits
                .sampled_image_depth_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_2) as u32,
            limits
                .sampled_image_depth_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_4) as u32,
            limits
                .sampled_image_depth_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_8) as u32,
            limits
                .sampled_image_depth_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_16) as u32,
            limits
                .sampled_image_depth_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_32) as u32,
            limits
                .sampled_image_depth_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_64) as u32,
            limits
                .sampled_image_stencil_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_1) as u32,
            limits
                .sampled_image_stencil_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_2) as u32,
            limits
                .sampled_image_stencil_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_4) as u32,
            limits
                .sampled_image_stencil_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_8) as u32,
            limits
                .sampled_image_stencil_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_16) as u32,
            limits
                .sampled_image_stencil_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_32) as u32,
            limits
                .sampled_image_stencil_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_64) as u32,
            limits
                .storage_image_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_1) as u32,
            limits
                .storage_image_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_2) as u32,
            limits
                .storage_image_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_4) as u32,
            limits
                .storage_image_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_8) as u32,
            limits
                .storage_image_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_16) as u32,
            limits
                .storage_image_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_32) as u32,
            limits
                .storage_image_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_64) as u32,
            limits.max_sample_mask_words,
            limits.timestamp_compute_and_graphics,
            limits.timestamp_period as u32,
            limits.max_clip_distances,
            limits.max_cull_distances,
            limits.max_combined_clip_and_cull_distances,
            limits.discrete_queue_priorities,
            limits.point_size_range[0] as u32,
            limits.point_size_range[1] as u32,
            limits.line_width_range[0] as u32,
            limits.line_width_range[1] as u32,
            limits.point_size_granularity as u32,
            limits.line_width_granularity as u32,
            limits.strict_lines,
            limits.standard_sample_locations,
            limits.optimal_buffer_copy_offset_alignment as u32,
            limits.optimal_buffer_copy_row_pitch_alignment as u32,
            limits.non_coherent_atom_size as u32,
        ]
    }

    /// Gets 1.1 properties as an ordered list of u32
    fn get_properties_1_1(properties: &ash::vk::PhysicalDeviceVulkan11Properties<'_>) -> Vec<u32> {
        vec![
            properties.subgroup_size,
            properties
                .subgroup_supported_stages
                .intersects(ash::vk::ShaderStageFlags::VERTEX) as u32,
            properties
                .subgroup_supported_stages
                .intersects(ash::vk::ShaderStageFlags::FRAGMENT) as u32,
            properties
                .subgroup_supported_stages
                .intersects(ash::vk::ShaderStageFlags::COMPUTE) as u32,
            properties
                .subgroup_supported_stages
                .intersects(ash::vk::ShaderStageFlags::GEOMETRY) as u32,
            properties
                .subgroup_supported_stages
                .intersects(ash::vk::ShaderStageFlags::TESSELLATION_CONTROL) as u32,
            properties
                .subgroup_supported_stages
                .intersects(ash::vk::ShaderStageFlags::TESSELLATION_EVALUATION) as u32,
            properties
                .subgroup_supported_operations
                .intersects(ash::vk::SubgroupFeatureFlags::BASIC) as u32,
            properties
                .subgroup_supported_operations
                .intersects(ash::vk::SubgroupFeatureFlags::VOTE) as u32,
            properties
                .subgroup_supported_operations
                .intersects(ash::vk::SubgroupFeatureFlags::ARITHMETIC) as u32,
            properties
                .subgroup_supported_operations
                .intersects(ash::vk::SubgroupFeatureFlags::BALLOT) as u32,
            properties
                .subgroup_supported_operations
                .intersects(ash::vk::SubgroupFeatureFlags::SHUFFLE) as u32,
            properties
                .subgroup_supported_operations
                .intersects(ash::vk::SubgroupFeatureFlags::SHUFFLE_RELATIVE) as u32,
            properties
                .subgroup_supported_operations
                .intersects(ash::vk::SubgroupFeatureFlags::CLUSTERED) as u32,
            properties
                .subgroup_supported_operations
                .intersects(ash::vk::SubgroupFeatureFlags::QUAD) as u32,
            properties.subgroup_quad_operations_in_all_stages,
            properties.max_multiview_view_count,
            properties.max_multiview_instance_index,
            properties.protected_no_fault,
            properties.max_per_set_descriptors,
            properties.max_memory_allocation_size as u32,
        ]
    }

    /// Gets 1.2 properties as an ordered list of u32
    fn get_properties_1_2(properties: &ash::vk::PhysicalDeviceVulkan12Properties<'_>) -> Vec<u32> {
        vec![
            properties.conformance_version.major as u32,
            properties.conformance_version.minor as u32,
            properties.conformance_version.patch as u32,
            properties.shader_signed_zero_inf_nan_preserve_float16,
            properties.shader_signed_zero_inf_nan_preserve_float32,
            properties.shader_signed_zero_inf_nan_preserve_float64,
            properties.shader_denorm_preserve_float16,
            properties.shader_denorm_preserve_float32,
            properties.shader_denorm_preserve_float64,
            properties.shader_denorm_flush_to_zero_float16,
            properties.shader_denorm_flush_to_zero_float32,
            properties.shader_denorm_flush_to_zero_float64,
            properties.shader_rounding_mode_rte_float16,
            properties.shader_rounding_mode_rte_float32,
            properties.shader_rounding_mode_rte_float64,
            properties.shader_rounding_mode_rtz_float16,
            properties.shader_rounding_mode_rtz_float32,
            properties.shader_rounding_mode_rtz_float64,
            properties.max_update_after_bind_descriptors_in_all_pools,
            properties.shader_uniform_buffer_array_non_uniform_indexing_native,
            properties.shader_sampled_image_array_non_uniform_indexing_native,
            properties.shader_storage_buffer_array_non_uniform_indexing_native,
            properties.shader_storage_image_array_non_uniform_indexing_native,
            properties.shader_input_attachment_array_non_uniform_indexing_native,
            properties.robust_buffer_access_update_after_bind,
            properties.quad_divergent_implicit_lod,
            properties.max_per_stage_descriptor_update_after_bind_samplers,
            properties.max_per_stage_descriptor_update_after_bind_uniform_buffers,
            properties.max_per_stage_descriptor_update_after_bind_storage_buffers,
            properties.max_per_stage_descriptor_update_after_bind_sampled_images,
            properties.max_per_stage_descriptor_update_after_bind_storage_images,
            properties.max_per_stage_descriptor_update_after_bind_input_attachments,
            properties.max_per_stage_update_after_bind_resources,
            properties.max_descriptor_set_update_after_bind_samplers,
            properties.max_descriptor_set_update_after_bind_uniform_buffers,
            properties.max_descriptor_set_update_after_bind_uniform_buffers_dynamic,
            properties.max_descriptor_set_update_after_bind_storage_buffers,
            properties.max_descriptor_set_update_after_bind_storage_buffers_dynamic,
            properties.max_descriptor_set_update_after_bind_sampled_images,
            properties.max_descriptor_set_update_after_bind_storage_images,
            properties.max_descriptor_set_update_after_bind_input_attachments,
            properties
                .supported_depth_resolve_modes
                .intersects(ash::vk::ResolveModeFlags::NONE) as u32,
            properties
                .supported_depth_resolve_modes
                .intersects(ash::vk::ResolveModeFlags::SAMPLE_ZERO) as u32,
            properties
                .supported_depth_resolve_modes
                .intersects(ash::vk::ResolveModeFlags::AVERAGE) as u32,
            properties
                .supported_depth_resolve_modes
                .intersects(ash::vk::ResolveModeFlags::MIN) as u32,
            properties
                .supported_depth_resolve_modes
                .intersects(ash::vk::ResolveModeFlags::MAX) as u32,
            properties
                .supported_stencil_resolve_modes
                .intersects(ash::vk::ResolveModeFlags::NONE) as u32,
            properties
                .supported_stencil_resolve_modes
                .intersects(ash::vk::ResolveModeFlags::SAMPLE_ZERO) as u32,
            properties
                .supported_stencil_resolve_modes
                .intersects(ash::vk::ResolveModeFlags::AVERAGE) as u32,
            properties
                .supported_stencil_resolve_modes
                .intersects(ash::vk::ResolveModeFlags::MIN) as u32,
            properties
                .supported_stencil_resolve_modes
                .intersects(ash::vk::ResolveModeFlags::MAX) as u32,
            properties.independent_resolve_none,
            properties.independent_resolve,
            properties.filter_minmax_single_component_formats,
            properties.filter_minmax_image_component_mapping,
            properties.max_timeline_semaphore_value_difference as u32,
            properties
                .framebuffer_integer_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_1) as u32,
            properties
                .framebuffer_integer_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_2) as u32,
            properties
                .framebuffer_integer_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_4) as u32,
            properties
                .framebuffer_integer_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_8) as u32,
            properties
                .framebuffer_integer_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_16) as u32,
            properties
                .framebuffer_integer_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_32) as u32,
            properties
                .framebuffer_integer_color_sample_counts
                .intersects(ash::vk::SampleCountFlags::TYPE_64) as u32,
        ]
    }

    /// Gets 1.3 properties as an ordered list of u32
    fn get_properties_1_3(properties: &ash::vk::PhysicalDeviceVulkan13Properties<'_>) -> Vec<u32> {
        vec![
            properties.min_subgroup_size,
            properties.max_subgroup_size,
            properties.max_compute_workgroup_subgroups,
            properties.required_subgroup_size_stages.intersects(ash::vk::ShaderStageFlags::VERTEX) as u32,
            properties.required_subgroup_size_stages.intersects(ash::vk::ShaderStageFlags::FRAGMENT) as u32,
            properties.required_subgroup_size_stages.intersects(ash::vk::ShaderStageFlags::COMPUTE) as u32,
            properties.required_subgroup_size_stages.intersects(ash::vk::ShaderStageFlags::GEOMETRY) as u32,
            properties.required_subgroup_size_stages.intersects(ash::vk::ShaderStageFlags::TESSELLATION_CONTROL) as u32,
            properties.required_subgroup_size_stages.intersects(ash::vk::ShaderStageFlags::TESSELLATION_EVALUATION) as u32,
            properties.max_inline_uniform_block_size,
            properties.max_per_stage_descriptor_inline_uniform_blocks,
            properties.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks,
            properties.max_descriptor_set_inline_uniform_blocks,
            properties.max_descriptor_set_update_after_bind_inline_uniform_blocks,
            properties.max_inline_uniform_total_size,
            properties.integer_dot_product8_bit_unsigned_accelerated,
            properties.integer_dot_product8_bit_signed_accelerated,
            properties.integer_dot_product8_bit_mixed_signedness_accelerated,
            properties.integer_dot_product4x8_bit_packed_unsigned_accelerated,
            properties.integer_dot_product4x8_bit_packed_signed_accelerated,
            properties.integer_dot_product4x8_bit_packed_mixed_signedness_accelerated,
            properties.integer_dot_product16_bit_unsigned_accelerated,
            properties.integer_dot_product16_bit_signed_accelerated,
            properties.integer_dot_product16_bit_mixed_signedness_accelerated,
            properties.integer_dot_product32_bit_unsigned_accelerated,
            properties.integer_dot_product32_bit_signed_accelerated,
            properties.integer_dot_product32_bit_mixed_signedness_accelerated,
            properties.integer_dot_product64_bit_unsigned_accelerated,
            properties.integer_dot_product64_bit_signed_accelerated,
            properties.integer_dot_product64_bit_mixed_signedness_accelerated,
            properties.integer_dot_product_accumulating_saturating8_bit_unsigned_accelerated,
            properties.integer_dot_product_accumulating_saturating8_bit_signed_accelerated,
            properties.integer_dot_product_accumulating_saturating8_bit_mixed_signedness_accelerated,
            properties.integer_dot_product_accumulating_saturating4x8_bit_packed_unsigned_accelerated,
            properties.integer_dot_product_accumulating_saturating4x8_bit_packed_signed_accelerated,
            properties.integer_dot_product_accumulating_saturating4x8_bit_packed_mixed_signedness_accelerated,
            properties.integer_dot_product_accumulating_saturating16_bit_unsigned_accelerated,
            properties.integer_dot_product_accumulating_saturating16_bit_signed_accelerated,
            properties.integer_dot_product_accumulating_saturating16_bit_mixed_signedness_accelerated,
            properties.integer_dot_product_accumulating_saturating32_bit_unsigned_accelerated,
            properties.integer_dot_product_accumulating_saturating32_bit_signed_accelerated,
            properties.integer_dot_product_accumulating_saturating32_bit_mixed_signedness_accelerated,
            properties.integer_dot_product_accumulating_saturating64_bit_unsigned_accelerated,
            properties.integer_dot_product_accumulating_saturating64_bit_signed_accelerated,
            properties.integer_dot_product_accumulating_saturating64_bit_mixed_signedness_accelerated,
            properties.storage_texel_buffer_offset_alignment_bytes as u32,
            properties.storage_texel_buffer_offset_single_texel_alignment,
            properties.uniform_texel_buffer_offset_alignment_bytes as u32,
            properties.uniform_texel_buffer_offset_single_texel_alignment,
            properties.max_buffer_size as u32,
        ]
    }

    // /// Gets 1.4 properties as an ordered list of u32
    // /// TODO: Update to ash 0.39 and Vulkan 1.4
    // fn get_properties_14(properties: &ash::vk::PhysicalDeviceVulkan14Properties<'_>) -> Vec<u32> {
    //     vec![
    //     ]
    // }

    /// Creates a new physical device score structure for a device
    fn new(
        instance: &ash::Instance,
        device: &ash::vk::PhysicalDevice,
        properties: &ash::vk::PhysicalDeviceProperties,
    ) -> Result<Self, ErrorType> {
        let supported_version = properties.api_version;
        let version = Version::new(
            ash::vk::api_version_variant(supported_version) as u8,
            ash::vk::api_version_major(supported_version) as u8,
            ash::vk::api_version_minor(supported_version) as u8,
            ash::vk::api_version_patch(supported_version) as u8,
        );
        let device_type = properties.device_type;
        let limits = Self::get_limits(&properties.limits);

        let mut properties_1_1 = ash::vk::PhysicalDeviceVulkan11Properties::default();
        let mut properties_1_2 = ash::vk::PhysicalDeviceVulkan12Properties::default();
        let mut properties_1_3 = ash::vk::PhysicalDeviceVulkan13Properties::default();
        // TODO: Update to ash 0.39 and Vulkan 1.4
        // let mut available_properties_1_4 = ash::vk::PhysicalDeviceVulkan14Properties::default();

        let mut properties_2 = ash::vk::PhysicalDeviceProperties2::default();
        if version >= Version::new(0, 1, 1, 0) {
            properties_2 = properties_2.push_next(&mut properties_1_1);
        }
        if version >= Version::new(0, 1, 2, 0) {
            properties_2 = properties_2.push_next(&mut properties_1_2);
        }
        if version >= Version::new(0, 1, 3, 0) {
            properties_2 = properties_2.push_next(&mut properties_1_3);
        }
        // // TODO: Update to ash 0.39 and Vulkan 1.4
        // if version >= Version::new(0, 1, 4, 0) {
        //     properties_2 = properties_2.push_next(&mut properties_1_4);
        // }

        unsafe {
            instance.get_physical_device_properties2(*device, &mut properties_2);
        }

        let properties_1_1 = Self::get_properties_1_1(&properties_1_1);
        let properties_1_2 = Self::get_properties_1_2(&properties_1_2);
        let properties_1_3 = Self::get_properties_1_3(&properties_1_3);
        // // TODO: Update to ash 0.39 and Vulkan 1.4
        // let properties_1_4 = Self::get_properties_1_4(&properties_1_4);

        let queues = unsafe { instance.get_physical_device_queue_family_properties(*device) };
        let queues = Self::get_queues(&queues);

        // let comparison_strategy = VkPhysicalDeviceScoreComparisonStrategy::Lexicographical;
        let comparison_strategy = VkPhysicalDeviceScoreComparisonStrategy::MostWins;

        Ok(Self {
            comparison_strategy,
            version,
            device_type,
            queues,
            limits,
            properties_1_1,
            properties_1_2,
            properties_1_3,
        })
    }
}

impl PartialEq for VkPhysicalDeviceScore {
    fn eq(&self, other: &Self) -> bool {
        let same_version = self.version == other.version;
        let same_device_type = self.device_type == other.device_type;
        let same_queues =
            VkPhysicalDeviceScoreComparisonStrategy::equal_lists(&self.queues, &other.queues);
        let same_limits =
            VkPhysicalDeviceScoreComparisonStrategy::equal_lists(&self.limits, &other.limits);
        let same_properties_1_1 = VkPhysicalDeviceScoreComparisonStrategy::equal_lists(
            &self.properties_1_1,
            &other.properties_1_1,
        );
        let same_properties_1_2 = VkPhysicalDeviceScoreComparisonStrategy::equal_lists(
            &self.properties_1_2,
            &other.properties_1_2,
        );
        let same_properties_1_3 = VkPhysicalDeviceScoreComparisonStrategy::equal_lists(
            &self.properties_1_3,
            &other.properties_1_3,
        );

        same_version
            && same_device_type
            && same_queues
            && same_limits
            && same_properties_1_1
            && same_properties_1_2
            && same_properties_1_3
    }
}

impl PartialOrd for VkPhysicalDeviceScore {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering;
        // 1️⃣ API Version
        match self.version.partial_cmp(&other.version) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        // 2️⃣ Device type (discrete > integrated > virtual > CPU)
        let device_type_score = |dt: ash::vk::PhysicalDeviceType| -> i32 {
            match dt {
                ash::vk::PhysicalDeviceType::DISCRETE_GPU => 4,
                ash::vk::PhysicalDeviceType::INTEGRATED_GPU => 3,
                ash::vk::PhysicalDeviceType::VIRTUAL_GPU => 2,
                ash::vk::PhysicalDeviceType::CPU => 1,
                _ => 0,
            }
        };
        match device_type_score(self.device_type).cmp(&device_type_score(other.device_type)) {
            Ordering::Equal => {}
            ord => return Some(ord),
        }
        // 3 Queues
        match self
            .comparison_strategy
            .comp_lists(&self.queues, &other.queues)
        {
            Ordering::Equal => {}
            ord => return Some(ord),
        }
        // 4 Limits
        match self
            .comparison_strategy
            .comp_lists(&self.limits, &other.limits)
        {
            Ordering::Equal => {}
            ord => return Some(ord),
        }
        // 5 Vulkan 1.1 properties
        match self
            .comparison_strategy
            .comp_lists(&self.properties_1_1, &other.properties_1_1)
        {
            Ordering::Equal => {}
            ord => return Some(ord),
        }
        // 6 Vulkan 1.2 properties
        match self
            .comparison_strategy
            .comp_lists(&self.properties_1_2, &other.properties_1_2)
        {
            Ordering::Equal => {}
            ord => return Some(ord),
        }
        // 7 Vulkan 1.3 properties
        match self
            .comparison_strategy
            .comp_lists(&self.properties_1_3, &other.properties_1_3)
        {
            Ordering::Equal => {}
            ord => return Some(ord),
        }
        // // TODO: Update to ash 0.39 and Vulkan 1.4
        // // 8 Vulkan 1.4 properties
        // match self.comparison_strategy.comp_lists(&self.properties_1_4, &other.properties_1_4) {
        //     Ordering::Equal => {}
        //     ord => return Some(ord),
        // }

        Some(Ordering::Equal)
    }
}

/// Checks if all required features are satisfied for the given physical device
/// Returns true if all required features are satisfied
/// TODO: Update to ash 0.39 and Vulkan 1.4
fn check_physical_device_features(
    config: &Config,
    instance: &ash::Instance,
    device: &ash::vk::PhysicalDevice,
    device_name: &std::ffi::CStr,
) -> Result<bool, ErrorType> {
    log_info!("Checking features issues for device: {:?}", device_name);
    let mut is_valid = true;
    let api_version = &config.renderer_config.vulkan_parameters.version;

    let mut available_features_1_1 = ash::vk::PhysicalDeviceVulkan11Features::default();
    let mut available_features_1_2 = ash::vk::PhysicalDeviceVulkan12Features::default();
    let mut available_features_1_3 = ash::vk::PhysicalDeviceVulkan13Features::default();
    // TODO: Update to ash 0.39 and Vulkan 1.4
    // let mut available_features_1_4 = ash::vk::PhysicalDeviceVulkan14Features::default();

    let mut features_2 = ash::vk::PhysicalDeviceFeatures2::default()
        .push_next(&mut available_features_1_1)
        .push_next(&mut available_features_1_2)
        .push_next(&mut available_features_1_3)
        // TODO: Update to ash 0.39 and Vulkan 1.4
        // .push_next(&mut available_features_1_4)
    ;

    unsafe {
        instance.get_physical_device_features2(*device, &mut features_2);
    }

    let required_features_1_0 = &config
        .renderer_config
        .vulkan_parameters
        .required_physical_device_features_1_0;
    let available_features_1_0 = unsafe { instance.get_physical_device_features(*device) };
    log_info!("\t- Missing required 1.0 features:");
    let mut missing_1_0_features = false;
    for required in required_features_1_0 {
        if !required.is_enabled(&available_features_1_0) {
            log_info!("\t\t- {:?}", &required);
            is_valid = false;
            missing_1_0_features = true;
        }
    }
    if !missing_1_0_features {
        log_info!("\t\t- None");
    }

    let requires_features_1_1 = *api_version >= Version::new(0, 1, 1, 0);
    let required_features_1_1 = &config
        .renderer_config
        .vulkan_parameters
        .required_physical_device_features_1_1;
    if !requires_features_1_1 && !required_features_1_1.is_empty() {
        log_error!(
            "Can't request 1.1 features when using `{:?}' api",
            api_version
        );
        return Err(ErrorType::BadRequest);
    }
    log_info!("\t- Missing required 1.1 features:");
    let mut missing_1_1_features = false;
    for required in required_features_1_1 {
        if !required.is_enabled(&available_features_1_1) {
            log_info!("\t\t- {:?}", &required);
            is_valid = false;
            missing_1_1_features = true;
        }
    }
    if !missing_1_1_features {
        log_info!("\t\t- None");
    }

    let requires_features_1_2 = *api_version >= Version::new(0, 1, 2, 0);
    let required_features_1_2 = &config
        .renderer_config
        .vulkan_parameters
        .required_physical_device_features_1_2;
    if !requires_features_1_2 && !required_features_1_1.is_empty() {
        log_error!(
            "Can't request 1.2 features when using `{:?}' api",
            api_version
        );
        return Err(ErrorType::BadRequest);
    }
    log_info!("\t- Missing required 1.2 features:");
    let mut missing_1_2_features = false;
    for required in required_features_1_2 {
        if !required.is_enabled(&available_features_1_2) {
            log_info!("\t\t- {:?}", &required);
            is_valid = false;
            missing_1_2_features = true;
        }
    }
    if !missing_1_2_features {
        log_info!("\t\t- None");
    }

    let requires_features_1_3 = *api_version >= Version::new(0, 1, 3, 0);
    let required_features_1_3 = &config
        .renderer_config
        .vulkan_parameters
        .required_physical_device_features_1_3;
    if !requires_features_1_3 && !required_features_1_3.is_empty() {
        log_error!(
            "Can't request 1.3 features when using `{:?}' api",
            api_version
        );
        return Err(ErrorType::BadRequest);
    }
    log_info!("\t- Missing required 1.3 features:");
    let mut missing_1_3_features = false;
    for required in required_features_1_3 {
        if !required.is_enabled(&available_features_1_3) {
            log_info!("\t\t- {:?}", &required);
            is_valid = false;
            missing_1_3_features = true;
        }
    }
    if !missing_1_3_features {
        log_info!("\t\t- None");
    }

    let requires_features_1_4 = *api_version >= Version::new(0, 1, 4, 0);
    let required_features_1_4 = &config
        .renderer_config
        .vulkan_parameters
        .required_physical_device_features_1_4;
    if !requires_features_1_4 && !required_features_1_1.is_empty() {
        log_error!(
            "Can't request 1.4 features when using `{:?}' api",
            api_version
        );
        return Err(ErrorType::BadRequest);
    }
    log_info!("\t- Missing required 1.4 features:");
    let mut missing_1_4_features = false;
    for required in required_features_1_4 {
        log_warn!(
            "\t\t- {:?}, Vulkan 1.4 is not yet supported in ash; skipping the required features",
            &required
        );
        missing_1_4_features = true;
        // TODO: Update to ash 0.39 and Vulkan 1.4
        // if !required.is_enabled(&available_features_1_4) {
        //     log_info!("\t\t- {:?}", &required);
        //     is_valid = false;
        // }
    }
    if !missing_1_4_features {
        log_info!("\t\t- None");
    }

    Ok(is_valid)
}

/// Checks if all required extensions are satisfied for the given physical device
/// Returns true if all required extensions are satisfied
fn check_physical_device_extensions(
    config: &Config,
    instance: &ash::Instance,
    device: &ash::vk::PhysicalDevice,
    device_name: &std::ffi::CStr,
) -> Result<bool, ErrorType> {
    log_info!("Checking extensions issues for device: {:?}", device_name);
    let mut is_valid = true;
    let required_extensions = &config
        .renderer_config
        .vulkan_parameters
        .required_device_extensions;
    let required_extensions_names = match VkDeviceExtensions::to_vknames(required_extensions) {
        Ok(names) => names,
        Err(err) => {
            log_error!(
                "Failed to convert required Vulkan extensions to vknames: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    let available_extensions =
        match unsafe { instance.enumerate_device_extension_properties(*device) } {
            Ok(extensions) => extensions,
            Err(err) => {
                log_error!(
                    "Failed to enumerate the device extension properties: {:?}",
                    err
                );
                return Err(ErrorType::VulkanError);
            }
        };

    // // Display available device extensions
    // for extension in &available_extensions {
    //     match extension.extension_name_as_c_str() {
    //         Ok(extension_cstr) => match extension_cstr.to_str() {
    //             Ok(extension_name) => log_info!("\t- Extension: {:?}", extension_name),
    //             Err(err) => {
    //                 log_error!("Invalid UTF-8 in available extension name: {:?}", err);
    //                 return Err(ErrorType::IO);
    //             }
    //         },
    //         Err(err) => {
    //             log_error!(
    //                 "Invalid string format in available extension name: {:?}",
    //                 err
    //             );
    //             return Err(ErrorType::IO);
    //         }
    //     }
    // }

    log_info!("\t- Missing required extensions:");
    let mut missing_extension = false;
    for required in &required_extensions_names.names_cstrings {
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
            is_valid = false;
            missing_extension = true;
            log_info!("\t\t- {:?}", required);
        }
    }
    if !missing_extension {
        log_info!("\t\t- None");
    }

    Ok(is_valid)
}

/// Checks if all device properties are satisfied for the given physical device
/// Returns true if all required properties are satisfied
fn check_physical_device_properties(
    config: &Config,
    properties: &ash::vk::PhysicalDeviceProperties,
    device_name: &std::ffi::CStr,
) -> Result<bool, ErrorType> {
    log_info!("Checking properties issues for device: {:?}", device_name);
    let mut is_valid = true;

    // Check API version
    let supported_version = properties.api_version;
    let supported_version = Version::new(
        ash::vk::api_version_variant(supported_version) as u8,
        ash::vk::api_version_major(supported_version) as u8,
        ash::vk::api_version_minor(supported_version) as u8,
        ash::vk::api_version_patch(supported_version) as u8,
    );
    let needed_version = &config.renderer_config.vulkan_parameters.version;
    if *needed_version > supported_version {
        log_info!(
            "\t- Wrong supported version: max `{:?}', needed `{:?}'",
            supported_version,
            needed_version
        );
        is_valid = false;
    }

    Ok(is_valid)
}

/// Rates a physical device
/// Returns None if the physical device is not compatible
fn rate_physical_device(
    config: &Config,
    instance: &ash::Instance,
    device: &ash::vk::PhysicalDevice,
) -> Result<Option<VkPhysicalDeviceScore>, ErrorType> {
    let properties = unsafe { instance.get_physical_device_properties(*device) };

    let device_name = match properties.device_name_as_c_str() {
        Ok(name) => name,
        Err(err) => {
            log_error!(
                "Failed to fetch the device name when rating physical devices: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    let are_properties_checked = match check_physical_device_properties(
        config,
        &properties,
        device_name,
    ) {
        Ok(are_checked) => are_checked,
        Err(err) => {
            log_error!(
                "Failed to check the physical device properties for device `{:?}' when rating physical devices: {:?}",
                device_name,
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    let are_features_checked = match check_physical_device_features(
        config,
        instance,
        device,
        device_name,
    ) {
        Ok(are_checked) => are_checked,
        Err(err) => {
            log_error!(
                "Failed to check the physical device features for device `{:?}' when rating physical devices: {:?}",
                device_name,
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    let are_extensions_checked = match check_physical_device_extensions(
        config,
        instance,
        device,
        device_name,
    ) {
        Ok(are_checked) => are_checked,
        Err(err) => {
            log_error!(
                "Failed to check the physical device extensions for device `{:?}' when rating physical devices: {:?}",
                device_name,
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    let is_valid = are_properties_checked && are_features_checked && are_extensions_checked;

    if !is_valid {
        return Ok(None);
    }

    match VkPhysicalDeviceScore::new(instance, device, &properties) {
        Ok(score) => Ok(Some(score)),
        Err(err) => {
            log_error!("Failed to get the score of a physical device: {:?}", err);
            Err(ErrorType::Unknown)
        }
    }
}

/// Picks the best physical device from a list
fn pick_best_physical_device<'a>(
    config: &Config,
    instance: &ash::Instance,
    devices: &'a [ash::vk::PhysicalDevice],
) -> Result<&'a ash::vk::PhysicalDevice, ErrorType> {
    log_info!("Required Vulkan physical device features:");
    log_info!("\t- Features 1.0:");
    for feature in &config
        .renderer_config
        .vulkan_parameters
        .required_physical_device_features_1_0
    {
        log_info!("\t\t- {:?}", feature);
    }
    log_info!("\t- Features 1.1:");
    for feature in &config
        .renderer_config
        .vulkan_parameters
        .required_physical_device_features_1_1
    {
        log_info!("\t\t- {:?}", feature);
    }
    log_info!("\t- Features 1.2:");
    for feature in &config
        .renderer_config
        .vulkan_parameters
        .required_physical_device_features_1_2
    {
        log_info!("\t\t- {:?}", feature);
    }
    log_info!("\t- Features 1.3:");
    for feature in &config
        .renderer_config
        .vulkan_parameters
        .required_physical_device_features_1_3
    {
        log_info!("\t\t- {:?}", feature);
    }
    log_info!("\t- Features 1.4:");
    for feature in &config
        .renderer_config
        .vulkan_parameters
        .required_physical_device_features_1_4
    {
        log_info!("\t\t- {:?}", feature);
    }

    log_info!("Required Vulkan physical device extensions:");
    for extension in &config
        .renderer_config
        .vulkan_parameters
        .required_device_extensions
    {
        log_info!("\t- {:?}", extension)
    }

    let mut best_device = None;
    let mut best_device_score = None;
    for device in devices {
        match rate_physical_device(config, instance, device) {
            Ok(None) => {}
            Ok(Some(score)) => {
                if best_device.is_none() || score > best_device_score.clone().unwrap() {
                    best_device = Some(device);
                    best_device_score = Some(score);
                }
            }
            Err(err) => {
                log_error!(
                    "Failed to rate a physical device when picking the best Vulkan physical device: {:?}",
                    err
                );
                return Err(ErrorType::Unknown);
            }
        }
    }

    match best_device {
        Some(device) => Ok(device),
        None => {
            log_error!(
                "Failed to find a suitable physical device when picking the best Vulkan physical device"
            );
            Err(ErrorType::DoesNotExist)
        }
    }
}

/// Initializes a physical device
pub(crate) fn init_physical_device(
    config: &Config,
    instance: &ash::Instance,
) -> Result<ash::vk::PhysicalDevice, ErrorType> {
    let available_devices = match unsafe { instance.enumerate_physical_devices() } {
        Ok(devices) => devices,
        Err(err) => {
            log_error!(
                "Failed to enumerate the physical devices when initializing Vulkan physical device: {:?}",
                err
            );
            return Err(ErrorType::VulkanError);
        }
    };

    if available_devices.is_empty() {
        log_error!("Failed to find a physical device when initializing Vulkan physical device");
        return Err(ErrorType::DoesNotExist);
    }

    log_info!(
        "Found {:?} physical devices available",
        available_devices.len()
    );
    let physical_device = match pick_best_physical_device(config, instance, &available_devices) {
        Ok(device) => device,
        Err(err) => {
            log_error!(
                "Failed to pick a physical device when initializing Vulkan physical device: {:?}",
                err
            );
            return Err(ErrorType::Unknown);
        }
    };

    log_info!("Vulkan physical device initialized");
    Ok(*physical_device)
}
