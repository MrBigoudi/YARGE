#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// Custom enum for Vulkan core 1.0 features names
/// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceFeatures.html
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub(crate) enum VkFeatures10 {
    /// Enables robust bounds checking for buffer accesses in shaders
    RobustBufferAccess,
    /// Allows uint32 draw indices instead of being limited to uint16
    FullDrawIndexUint32,
    /// Enables cube map arrays as image views
    ImageCubeArray,
    /// Allows independent blend state per color attachment
    IndependentBlend,
    /// Enables geometry shader stage support
    GeometryShader,
    /// Enables tessellation control and evaluation shader stages
    TessellationShader,
    /// Allows per-sample shading and programmable sample rates
    SampleRateShading,
    /// Enables dual-source color blending
    DualSrcBlend,
    /// Allows logical operations in color blending
    LogicOp,
    /// Enables issuing multiple indirect draw commands from a single buffer
    MultiDrawIndirect,
    /// Allows indirect draws to specify the first instance value
    DrawIndirectFirstInstance,
    /// Allows depth values outside the [0,1] range to be clamped
    DepthClamp,
    /// Enables depth bias clamping to a specified range
    DepthBiasClamp,
    /// Allows non-solid polygon fill modes such as wireframe or points
    FillModeNonSolid,
    /// Enables depth bounds testing
    DepthBounds,
    /// Allows line widths greater than 1.0
    WideLines,
    /// Allows point sizes greater than 1.0
    LargePoints,
    /// Enables alpha-to-one color blending
    AlphaToOne,
    /// Allows rendering to multiple viewports in a single draw
    MultiViewport,
    /// Enables anisotropic texture filtering
    SamplerAnisotropy,
    /// Enables ETC2 texture compression formats
    TextureCompressionEtc2,
    /// Enables ASTC LDR texture compression formats
    TextureCompressionAstcLdr,
    /// Enables BC (block-compressed) texture compression formats
    TextureCompressionBc,
    /// Enables precise occlusion query results
    OcclusionQueryPrecise,
    /// Enables pipeline statistics queries
    PipelineStatisticsQuery,
    /// Allows vertex shader stores and atomic operations
    VertexPipelineStoresAndAtomics,
    /// Allows fragment shader stores and atomic operations
    FragmentStoresAndAtomics,
    /// Allows point size control in tessellation and geometry shaders
    ShaderTessellationAndGeometryPointSize,
    /// Enables extended image gather operations in shaders
    ShaderImageGatherExtended,
    /// Allows storage images to use extended format types
    ShaderStorageImageExtendedFormats,
    /// Enables multisampled storage images in shaders
    ShaderStorageImageMultisample,
    /// Allows reading storage images without a defined format
    ShaderStorageImageReadWithoutFormat,
    /// Allows writing storage images without a defined format
    ShaderStorageImageWriteWithoutFormat,
    /// Allows dynamic indexing of uniform buffer arrays in shaders
    ShaderUniformBufferArrayDynamicIndexing,
    /// Allows dynamic indexing of sampled image arrays in shaders
    ShaderSampledImageArrayDynamicIndexing,
    /// Allows dynamic indexing of storage buffer arrays in shaders
    ShaderStorageBufferArrayDynamicIndexing,
    /// Allows dynamic indexing of storage image arrays in shaders
    ShaderStorageImageArrayDynamicIndexing,
    /// Enables use of clip distances in shaders
    ShaderClipDistance,
    /// Enables use of cull distances in shaders
    ShaderCullDistance,
    /// Enables 64-bit floating point operations in shaders
    ShaderFloat64,
    /// Enables 64-bit integer operations in shaders
    ShaderInt64,
    /// Enables 16-bit integer operations in shaders
    ShaderInt16,
    /// Enables sparse residency support in shaders
    ShaderResourceResidency,
    /// Enables minimum level-of-detail control for shader resources
    ShaderResourceMinLod,
    /// Enables sparse memory binding for resources
    SparseBinding,
    /// Enables sparse residency for buffer resources
    SparseResidencyBuffer,
    /// Enables sparse residency for 2D image resources
    SparseResidencyImage2D,
    /// Enables sparse residency for 3D image resources
    SparseResidencyImage3D,
    /// Enables sparse residency for 2-sample multisampled images
    SparseResidency2Samples,
    /// Enables sparse residency for 4-sample multisampled images
    SparseResidency4Samples,
    /// Enables sparse residency for 8-sample multisampled images
    SparseResidency8Samples,
    /// Enables sparse residency for 16-sample multisampled images
    SparseResidency16Samples,
    /// Enables aliased sparse residency between resources
    SparseResidencyAliased,
    /// Allows variable multisample rates per framebuffer
    VariableMultisampleRate,
    /// Allows queries to be inherited by secondary command buffers
    InheritedQueries,
}

impl VkFeatures10 {
    /// Converts as Ash features
    pub(crate) fn as_ash_features(features: &[Self]) -> ash::vk::PhysicalDeviceFeatures {
        let mut ash_features = ash::vk::PhysicalDeviceFeatures::default();
        for feature in features {
            ash_features = match feature {
                VkFeatures10::RobustBufferAccess => ash_features.robust_buffer_access(true),
                VkFeatures10::FullDrawIndexUint32 => ash_features.full_draw_index_uint32(true),
                VkFeatures10::ImageCubeArray => ash_features.image_cube_array(true),
                VkFeatures10::IndependentBlend => ash_features.independent_blend(true),
                VkFeatures10::GeometryShader => ash_features.geometry_shader(true),
                VkFeatures10::TessellationShader => ash_features.tessellation_shader(true),
                VkFeatures10::SampleRateShading => ash_features.sample_rate_shading(true),
                VkFeatures10::DualSrcBlend => ash_features.dual_src_blend(true),
                VkFeatures10::LogicOp => ash_features.logic_op(true),
                VkFeatures10::MultiDrawIndirect => ash_features.multi_draw_indirect(true),
                VkFeatures10::DrawIndirectFirstInstance => {
                    ash_features.draw_indirect_first_instance(true)
                }
                VkFeatures10::DepthClamp => ash_features.depth_clamp(true),
                VkFeatures10::DepthBiasClamp => ash_features.depth_bias_clamp(true),
                VkFeatures10::FillModeNonSolid => ash_features.fill_mode_non_solid(true),
                VkFeatures10::DepthBounds => ash_features.depth_bounds(true),
                VkFeatures10::WideLines => ash_features.wide_lines(true),
                VkFeatures10::LargePoints => ash_features.large_points(true),
                VkFeatures10::AlphaToOne => ash_features.alpha_to_one(true),
                VkFeatures10::MultiViewport => ash_features.multi_viewport(true),
                VkFeatures10::SamplerAnisotropy => ash_features.sampler_anisotropy(true),
                VkFeatures10::TextureCompressionEtc2 => ash_features.texture_compression_etc2(true),
                VkFeatures10::TextureCompressionAstcLdr => {
                    ash_features.texture_compression_astc_ldr(true)
                }
                VkFeatures10::TextureCompressionBc => ash_features.texture_compression_bc(true),
                VkFeatures10::OcclusionQueryPrecise => ash_features.occlusion_query_precise(true),
                VkFeatures10::PipelineStatisticsQuery => {
                    ash_features.pipeline_statistics_query(true)
                }
                VkFeatures10::VertexPipelineStoresAndAtomics => {
                    ash_features.vertex_pipeline_stores_and_atomics(true)
                }
                VkFeatures10::FragmentStoresAndAtomics => {
                    ash_features.fragment_stores_and_atomics(true)
                }
                VkFeatures10::ShaderTessellationAndGeometryPointSize => {
                    ash_features.shader_tessellation_and_geometry_point_size(true)
                }
                VkFeatures10::ShaderImageGatherExtended => {
                    ash_features.shader_image_gather_extended(true)
                }
                VkFeatures10::ShaderStorageImageExtendedFormats => {
                    ash_features.shader_storage_image_extended_formats(true)
                }
                VkFeatures10::ShaderStorageImageMultisample => {
                    ash_features.shader_storage_image_multisample(true)
                }
                VkFeatures10::ShaderStorageImageReadWithoutFormat => {
                    ash_features.shader_storage_image_read_without_format(true)
                }
                VkFeatures10::ShaderStorageImageWriteWithoutFormat => {
                    ash_features.shader_storage_image_write_without_format(true)
                }
                VkFeatures10::ShaderUniformBufferArrayDynamicIndexing => {
                    ash_features.shader_uniform_buffer_array_dynamic_indexing(true)
                }
                VkFeatures10::ShaderSampledImageArrayDynamicIndexing => {
                    ash_features.shader_sampled_image_array_dynamic_indexing(true)
                }
                VkFeatures10::ShaderStorageBufferArrayDynamicIndexing => {
                    ash_features.shader_storage_buffer_array_dynamic_indexing(true)
                }
                VkFeatures10::ShaderStorageImageArrayDynamicIndexing => {
                    ash_features.shader_storage_image_array_dynamic_indexing(true)
                }
                VkFeatures10::ShaderClipDistance => ash_features.shader_clip_distance(true),
                VkFeatures10::ShaderCullDistance => ash_features.shader_cull_distance(true),
                VkFeatures10::ShaderFloat64 => ash_features.shader_float64(true),
                VkFeatures10::ShaderInt64 => ash_features.shader_int64(true),
                VkFeatures10::ShaderInt16 => ash_features.shader_int16(true),
                VkFeatures10::ShaderResourceResidency => {
                    ash_features.shader_resource_residency(true)
                }
                VkFeatures10::ShaderResourceMinLod => ash_features.shader_resource_min_lod(true),
                VkFeatures10::SparseBinding => ash_features.sparse_binding(true),
                VkFeatures10::SparseResidencyBuffer => ash_features.sparse_residency_buffer(true),
                VkFeatures10::SparseResidencyImage2D => {
                    ash_features.sparse_residency_image2_d(true)
                }
                VkFeatures10::SparseResidencyImage3D => {
                    ash_features.sparse_residency_image3_d(true)
                }
                VkFeatures10::SparseResidency2Samples => {
                    ash_features.sparse_residency2_samples(true)
                }
                VkFeatures10::SparseResidency4Samples => {
                    ash_features.sparse_residency4_samples(true)
                }
                VkFeatures10::SparseResidency8Samples => {
                    ash_features.sparse_residency8_samples(true)
                }
                VkFeatures10::SparseResidency16Samples => {
                    ash_features.sparse_residency16_samples(true)
                }
                VkFeatures10::SparseResidencyAliased => ash_features.sparse_residency_aliased(true),
                VkFeatures10::VariableMultisampleRate => {
                    ash_features.variable_multisample_rate(true)
                }
                VkFeatures10::InheritedQueries => ash_features.inherited_queries(true),
            }
        }

        ash_features
    }

    /// Returns true if the features is enabled
    pub(crate) fn is_enabled(&self, device_features: &ash::vk::PhysicalDeviceFeatures) -> bool {
        match self {
            VkFeatures10::RobustBufferAccess => device_features.robust_buffer_access != 0,
            VkFeatures10::FullDrawIndexUint32 => device_features.full_draw_index_uint32 != 0,
            VkFeatures10::ImageCubeArray => device_features.image_cube_array != 0,
            VkFeatures10::IndependentBlend => device_features.independent_blend != 0,
            VkFeatures10::GeometryShader => device_features.geometry_shader != 0,
            VkFeatures10::TessellationShader => device_features.tessellation_shader != 0,
            VkFeatures10::SampleRateShading => device_features.sample_rate_shading != 0,
            VkFeatures10::DualSrcBlend => device_features.dual_src_blend != 0,
            VkFeatures10::LogicOp => device_features.logic_op != 0,
            VkFeatures10::MultiDrawIndirect => device_features.multi_draw_indirect != 0,
            VkFeatures10::DrawIndirectFirstInstance => {
                device_features.draw_indirect_first_instance != 0
            }
            VkFeatures10::DepthClamp => device_features.depth_clamp != 0,
            VkFeatures10::DepthBiasClamp => device_features.depth_bias_clamp != 0,
            VkFeatures10::FillModeNonSolid => device_features.fill_mode_non_solid != 0,
            VkFeatures10::DepthBounds => device_features.depth_bounds != 0,
            VkFeatures10::WideLines => device_features.wide_lines != 0,
            VkFeatures10::LargePoints => device_features.large_points != 0,
            VkFeatures10::AlphaToOne => device_features.alpha_to_one != 0,
            VkFeatures10::MultiViewport => device_features.multi_viewport != 0,
            VkFeatures10::SamplerAnisotropy => device_features.sampler_anisotropy != 0,
            VkFeatures10::TextureCompressionEtc2 => device_features.texture_compression_etc2 != 0,
            VkFeatures10::TextureCompressionAstcLdr => {
                device_features.texture_compression_astc_ldr != 0
            }
            VkFeatures10::TextureCompressionBc => device_features.texture_compression_bc != 0,
            VkFeatures10::OcclusionQueryPrecise => device_features.occlusion_query_precise != 0,
            VkFeatures10::PipelineStatisticsQuery => device_features.pipeline_statistics_query != 0,
            VkFeatures10::VertexPipelineStoresAndAtomics => {
                device_features.vertex_pipeline_stores_and_atomics != 0
            }
            VkFeatures10::FragmentStoresAndAtomics => {
                device_features.fragment_stores_and_atomics != 0
            }
            VkFeatures10::ShaderTessellationAndGeometryPointSize => {
                device_features.shader_tessellation_and_geometry_point_size != 0
            }
            VkFeatures10::ShaderImageGatherExtended => {
                device_features.shader_image_gather_extended != 0
            }
            VkFeatures10::ShaderStorageImageExtendedFormats => {
                device_features.shader_storage_image_extended_formats != 0
            }
            VkFeatures10::ShaderStorageImageMultisample => {
                device_features.shader_storage_image_multisample != 0
            }
            VkFeatures10::ShaderStorageImageReadWithoutFormat => {
                device_features.shader_storage_image_read_without_format != 0
            }
            VkFeatures10::ShaderStorageImageWriteWithoutFormat => {
                device_features.shader_storage_image_write_without_format != 0
            }
            VkFeatures10::ShaderUniformBufferArrayDynamicIndexing => {
                device_features.shader_uniform_buffer_array_dynamic_indexing != 0
            }
            VkFeatures10::ShaderSampledImageArrayDynamicIndexing => {
                device_features.shader_sampled_image_array_dynamic_indexing != 0
            }
            VkFeatures10::ShaderStorageBufferArrayDynamicIndexing => {
                device_features.shader_storage_buffer_array_dynamic_indexing != 0
            }
            VkFeatures10::ShaderStorageImageArrayDynamicIndexing => {
                device_features.shader_storage_image_array_dynamic_indexing != 0
            }
            VkFeatures10::ShaderClipDistance => device_features.shader_clip_distance != 0,
            VkFeatures10::ShaderCullDistance => device_features.shader_cull_distance != 0,
            VkFeatures10::ShaderFloat64 => device_features.shader_float64 != 0,
            VkFeatures10::ShaderInt64 => device_features.shader_int64 != 0,
            VkFeatures10::ShaderInt16 => device_features.shader_int16 != 0,
            VkFeatures10::ShaderResourceResidency => device_features.shader_resource_residency != 0,
            VkFeatures10::ShaderResourceMinLod => device_features.shader_resource_min_lod != 0,
            VkFeatures10::SparseBinding => device_features.sparse_binding != 0,
            VkFeatures10::SparseResidencyBuffer => device_features.sparse_residency_buffer != 0,
            VkFeatures10::SparseResidencyImage2D => device_features.sparse_residency_image2_d != 0,
            VkFeatures10::SparseResidencyImage3D => device_features.sparse_residency_image3_d != 0,
            VkFeatures10::SparseResidency2Samples => device_features.sparse_residency2_samples != 0,
            VkFeatures10::SparseResidency4Samples => device_features.sparse_residency4_samples != 0,
            VkFeatures10::SparseResidency8Samples => device_features.sparse_residency8_samples != 0,
            VkFeatures10::SparseResidency16Samples => {
                device_features.sparse_residency16_samples != 0
            }
            VkFeatures10::SparseResidencyAliased => device_features.sparse_residency_aliased != 0,
            VkFeatures10::VariableMultisampleRate => device_features.variable_multisample_rate != 0,
            VkFeatures10::InheritedQueries => device_features.inherited_queries != 0,
        }
    }
}

/// Custom enum for Vulkan 1.1 features names
/// https://docs.vulkan.org/spec/latest/chapters/features.html#VkPhysicalDeviceVulkan11Features
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub(crate) enum VkFeatures11 {
    /// Allows protected memory allocations and resources
    ProtectedMemory,
    /// Enables multiview rendering
    Multiview,
    /// Enables multiview geometry shaders
    MultiviewGeometryShader,
    /// Enables multiview tessellation shaders
    MultiviewTessellationShader,
    /// Allows variable pointer storage class in shaders
    VariablePointersStorageBuffer,
    /// Allows variable pointers in shaders
    VariablePointers,
    /// Enables 16-bit integer operations in storage buffers
    StorageBuffer16BitAccess,
    /// Enables 16-bit integer operations in uniform buffers
    UniformAndStorageBuffer16BitAccess,
    /// Enables 16-bit integer operations for push constants
    StoragePushConstant16,
    /// Enables 16-bit input/output for shaders
    StorageInputOutput16,
    /// Enables sampler Y′CBCR conversion
    SamplerYcbcrConversion,
    /// Enables maintenance4-style device feature inheritance
    ShaderDrawParameters,
}

impl VkFeatures11 {
    /// Converts as Ash features
    pub(crate) fn as_ash_features<'a>(
        features: &'a [Self],
    ) -> ash::vk::PhysicalDeviceVulkan11Features<'a> {
        let mut ash_features = ash::vk::PhysicalDeviceVulkan11Features::default();
        for feature in features {
            ash_features = match feature {
                VkFeatures11::ProtectedMemory => ash_features.protected_memory(true),
                VkFeatures11::Multiview => ash_features.multiview(true),
                VkFeatures11::MultiviewGeometryShader => {
                    ash_features.multiview_geometry_shader(true)
                }
                VkFeatures11::MultiviewTessellationShader => {
                    ash_features.multiview_tessellation_shader(true)
                }
                VkFeatures11::VariablePointersStorageBuffer => {
                    ash_features.variable_pointers_storage_buffer(true)
                }
                VkFeatures11::VariablePointers => ash_features.variable_pointers(true),
                VkFeatures11::StorageBuffer16BitAccess => {
                    ash_features.storage_buffer16_bit_access(true)
                }
                VkFeatures11::UniformAndStorageBuffer16BitAccess => {
                    ash_features.uniform_and_storage_buffer16_bit_access(true)
                }
                VkFeatures11::StoragePushConstant16 => ash_features.storage_push_constant16(true),
                VkFeatures11::StorageInputOutput16 => ash_features.storage_input_output16(true),
                VkFeatures11::SamplerYcbcrConversion => ash_features.sampler_ycbcr_conversion(true),
                VkFeatures11::ShaderDrawParameters => ash_features.shader_draw_parameters(true),
            }
        }

        ash_features
    }

    /// Returns true if the feature is enabled
    pub(crate) fn is_enabled(
        &self,
        features: &ash::vk::PhysicalDeviceVulkan11Features<'_>,
    ) -> bool {
        match self {
            VkFeatures11::ProtectedMemory => features.protected_memory != 0,
            VkFeatures11::Multiview => features.multiview != 0,
            VkFeatures11::MultiviewGeometryShader => features.multiview_geometry_shader != 0,
            VkFeatures11::MultiviewTessellationShader => {
                features.multiview_tessellation_shader != 0
            }
            VkFeatures11::VariablePointersStorageBuffer => {
                features.variable_pointers_storage_buffer != 0
            }
            VkFeatures11::VariablePointers => features.variable_pointers != 0,
            VkFeatures11::StorageBuffer16BitAccess => features.storage_buffer16_bit_access != 0,
            VkFeatures11::UniformAndStorageBuffer16BitAccess => {
                features.uniform_and_storage_buffer16_bit_access != 0
            }
            VkFeatures11::StoragePushConstant16 => features.storage_push_constant16 != 0,
            VkFeatures11::StorageInputOutput16 => features.storage_input_output16 != 0,
            VkFeatures11::SamplerYcbcrConversion => features.sampler_ycbcr_conversion != 0,
            VkFeatures11::ShaderDrawParameters => features.shader_draw_parameters != 0,
        }
    }
}

/// Custom enum for Vulkan 1.2 features names
/// https://docs.vulkan.org/spec/latest/chapters/features.html#VkPhysicalDeviceVulkan12Features
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub(crate) enum VkFeatures12 {
    /// Enables sampler mirror clamp-to-edge addressing mode
    SamplerMirrorClampToEdge,
    /// Enables drawIndirectCount and drawIndexedIndirectCount
    DrawIndirectCount,
    /// Enables storage buffers with non-uniform indexing
    StorageBuffer8BitAccess,
    /// Enables uniform buffers with non-uniform indexing
    UniformAndStorageBuffer8BitAccess,
    /// Enables 8-bit input/output for shaders
    StoragePushConstant8,
    /// Enables shader buffer int64 operations
    ShaderBufferInt64Atomics,
    /// Enables shader shared int64 operations
    ShaderSharedInt64Atomics,
    /// Enables shader float16 operations
    ShaderFloat16,
    /// Enables shader int8 operations
    ShaderInt8,
    /// Enables descriptor indexing
    DescriptorIndexing,
    /// Allows shaders to index sampled images non-uniformly
    ShaderInputAttachmentArrayDynamicIndexing,
    /// Allows shaders to index uniform texel buffers dynamically
    ShaderUniformTexelBufferArrayDynamicIndexing,
    /// Allows shaders to index storage texel buffers dynamically
    ShaderStorageTexelBufferArrayDynamicIndexing,
    /// Allows shaders to index uniform buffers dynamically
    ShaderUniformBufferArrayNonUniformIndexing,
    /// Allows shaders to index sampled images non-uniformly
    ShaderSampledImageArrayNonUniformIndexing,
    /// Allows shaders to index storage buffers non-uniformly
    ShaderStorageBufferArrayNonUniformIndexing,
    /// Allows shaders to index storage images non-uniformly
    ShaderStorageImageArrayNonUniformIndexing,
    /// Allows descriptor binding updates after bind
    DescriptorBindingUniformBufferUpdateAfterBind,
    /// Allows sampled image descriptor binding updates after bind
    DescriptorBindingSampledImageUpdateAfterBind,
    /// Allows storage image descriptor binding updates after bind
    DescriptorBindingStorageImageUpdateAfterBind,
    /// Allows storage buffer descriptor binding updates after bind
    DescriptorBindingStorageBufferUpdateAfterBind,
    /// Allows uniform texel buffer descriptor binding updates after bind
    DescriptorBindingUniformTexelBufferUpdateAfterBind,
    /// Allows storage texel buffer descriptor binding updates after bind
    DescriptorBindingStorageTexelBufferUpdateAfterBind,
    /// Allows partially bound descriptor arrays
    DescriptorBindingPartiallyBound,
    /// Allows variable-sized descriptor arrays
    DescriptorBindingVariableDescriptorCount,
    /// Allows descriptors to be updated while pending
    DescriptorBindingUpdateUnusedWhilePending,
    /// Enables runtime-sized descriptor arrays
    RuntimeDescriptorArray,
    /// Enables scalar block layout for buffer layouts
    ScalarBlockLayout,
    /// Enables imageless framebuffers
    ImagelessFramebuffer,
    /// Enables uniform buffer standard layout
    UniformBufferStandardLayout,
    /// Enables subgroup extended types
    ShaderSubgroupExtendedTypes,
    /// Enables separate depth/stencil layouts
    SeparateDepthStencilLayouts,
    /// Enables host query reset
    HostQueryReset,
    /// Enables timeline semaphores
    TimelineSemaphore,
    /// Enables buffer device addresses
    BufferDeviceAddress,
    /// Enables buffer capture/replay addresses
    BufferDeviceAddressCaptureReplay,
    /// Enables multi-device buffer addresses
    BufferDeviceAddressMultiDevice,
    /// Enables Vulkan memory model
    VulkanMemoryModel,
    /// Enables Vulkan memory model with device scope
    VulkanMemoryModelDeviceScope,
    /// Enables atomic operations with relaxed ordering
    VulkanMemoryModelAvailabilityVisibilityChains,
    /// Enables shader output viewport index
    ShaderOutputViewportIndex,
    /// Enables shader output layer
    ShaderOutputLayer,
    /// Enables subgroup broadcast dynamic ID
    SubgroupBroadcastDynamicId,
}

impl VkFeatures12 {
    pub(crate) fn as_ash_features<'a>(
        features: &'a [Self],
    ) -> ash::vk::PhysicalDeviceVulkan12Features<'a> {
        let mut ash_features = ash::vk::PhysicalDeviceVulkan12Features::default();
        for feature in features {
            ash_features = match feature {
                VkFeatures12::SamplerMirrorClampToEdge => {
                    ash_features.sampler_mirror_clamp_to_edge(true)
                }
                VkFeatures12::DrawIndirectCount => ash_features.draw_indirect_count(true),
                VkFeatures12::StorageBuffer8BitAccess => {
                    ash_features.storage_buffer8_bit_access(true)
                }
                VkFeatures12::UniformAndStorageBuffer8BitAccess => {
                    ash_features.uniform_and_storage_buffer8_bit_access(true)
                }
                VkFeatures12::StoragePushConstant8 => ash_features.storage_push_constant8(true),
                VkFeatures12::ShaderBufferInt64Atomics => {
                    ash_features.shader_buffer_int64_atomics(true)
                }
                VkFeatures12::ShaderSharedInt64Atomics => {
                    ash_features.shader_shared_int64_atomics(true)
                }
                VkFeatures12::ShaderFloat16 => ash_features.shader_float16(true),
                VkFeatures12::ShaderInt8 => ash_features.shader_int8(true),
                VkFeatures12::DescriptorIndexing => ash_features.descriptor_indexing(true),
                VkFeatures12::ShaderInputAttachmentArrayDynamicIndexing => {
                    ash_features.shader_input_attachment_array_dynamic_indexing(true)
                }
                VkFeatures12::ShaderUniformTexelBufferArrayDynamicIndexing => {
                    ash_features.shader_uniform_texel_buffer_array_dynamic_indexing(true)
                }
                VkFeatures12::ShaderStorageTexelBufferArrayDynamicIndexing => {
                    ash_features.shader_storage_texel_buffer_array_dynamic_indexing(true)
                }
                VkFeatures12::ShaderUniformBufferArrayNonUniformIndexing => {
                    ash_features.shader_uniform_buffer_array_non_uniform_indexing(true)
                }
                VkFeatures12::ShaderSampledImageArrayNonUniformIndexing => {
                    ash_features.shader_sampled_image_array_non_uniform_indexing(true)
                }
                VkFeatures12::ShaderStorageBufferArrayNonUniformIndexing => {
                    ash_features.shader_storage_buffer_array_non_uniform_indexing(true)
                }
                VkFeatures12::ShaderStorageImageArrayNonUniformIndexing => {
                    ash_features.shader_storage_image_array_non_uniform_indexing(true)
                }
                VkFeatures12::DescriptorBindingUniformBufferUpdateAfterBind => {
                    ash_features.descriptor_binding_uniform_buffer_update_after_bind(true)
                }
                VkFeatures12::DescriptorBindingSampledImageUpdateAfterBind => {
                    ash_features.descriptor_binding_sampled_image_update_after_bind(true)
                }
                VkFeatures12::DescriptorBindingStorageImageUpdateAfterBind => {
                    ash_features.descriptor_binding_storage_image_update_after_bind(true)
                }
                VkFeatures12::DescriptorBindingStorageBufferUpdateAfterBind => {
                    ash_features.descriptor_binding_storage_buffer_update_after_bind(true)
                }
                VkFeatures12::DescriptorBindingUniformTexelBufferUpdateAfterBind => {
                    ash_features.descriptor_binding_uniform_texel_buffer_update_after_bind(true)
                }
                VkFeatures12::DescriptorBindingStorageTexelBufferUpdateAfterBind => {
                    ash_features.descriptor_binding_storage_texel_buffer_update_after_bind(true)
                }
                VkFeatures12::DescriptorBindingPartiallyBound => {
                    ash_features.descriptor_binding_partially_bound(true)
                }
                VkFeatures12::DescriptorBindingVariableDescriptorCount => {
                    ash_features.descriptor_binding_variable_descriptor_count(true)
                }
                VkFeatures12::DescriptorBindingUpdateUnusedWhilePending => {
                    ash_features.descriptor_binding_update_unused_while_pending(true)
                }
                VkFeatures12::RuntimeDescriptorArray => ash_features.runtime_descriptor_array(true),
                VkFeatures12::ScalarBlockLayout => ash_features.scalar_block_layout(true),
                VkFeatures12::ImagelessFramebuffer => ash_features.imageless_framebuffer(true),
                VkFeatures12::UniformBufferStandardLayout => {
                    ash_features.uniform_buffer_standard_layout(true)
                }
                VkFeatures12::ShaderSubgroupExtendedTypes => {
                    ash_features.shader_subgroup_extended_types(true)
                }
                VkFeatures12::SeparateDepthStencilLayouts => {
                    ash_features.separate_depth_stencil_layouts(true)
                }
                VkFeatures12::HostQueryReset => ash_features.host_query_reset(true),
                VkFeatures12::TimelineSemaphore => ash_features.timeline_semaphore(true),
                VkFeatures12::BufferDeviceAddress => ash_features.buffer_device_address(true),
                VkFeatures12::BufferDeviceAddressCaptureReplay => {
                    ash_features.buffer_device_address_capture_replay(true)
                }
                VkFeatures12::BufferDeviceAddressMultiDevice => {
                    ash_features.buffer_device_address_multi_device(true)
                }
                VkFeatures12::VulkanMemoryModel => ash_features.vulkan_memory_model(true),
                VkFeatures12::VulkanMemoryModelDeviceScope => {
                    ash_features.vulkan_memory_model_device_scope(true)
                }
                VkFeatures12::VulkanMemoryModelAvailabilityVisibilityChains => {
                    ash_features.vulkan_memory_model_availability_visibility_chains(true)
                }
                VkFeatures12::ShaderOutputViewportIndex => {
                    ash_features.shader_output_viewport_index(true)
                }
                VkFeatures12::ShaderOutputLayer => ash_features.shader_output_layer(true),
                VkFeatures12::SubgroupBroadcastDynamicId => {
                    ash_features.subgroup_broadcast_dynamic_id(true)
                }
            }
        }

        ash_features
    }

    /// Returns true if the feature is enabled
    pub(crate) fn is_enabled(
        &self,
        features: &ash::vk::PhysicalDeviceVulkan12Features<'_>,
    ) -> bool {
        match self {
            VkFeatures12::SamplerMirrorClampToEdge => features.sampler_mirror_clamp_to_edge != 0,
            VkFeatures12::DrawIndirectCount => features.draw_indirect_count != 0,
            VkFeatures12::StorageBuffer8BitAccess => features.storage_buffer8_bit_access != 0,
            VkFeatures12::UniformAndStorageBuffer8BitAccess => {
                features.uniform_and_storage_buffer8_bit_access != 0
            }
            VkFeatures12::StoragePushConstant8 => features.storage_push_constant8 != 0,
            VkFeatures12::ShaderBufferInt64Atomics => features.shader_buffer_int64_atomics != 0,
            VkFeatures12::ShaderSharedInt64Atomics => features.shader_shared_int64_atomics != 0,
            VkFeatures12::ShaderFloat16 => features.shader_float16 != 0,
            VkFeatures12::ShaderInt8 => features.shader_int8 != 0,
            VkFeatures12::DescriptorIndexing => features.descriptor_indexing != 0,
            VkFeatures12::ShaderInputAttachmentArrayDynamicIndexing => {
                features.shader_input_attachment_array_dynamic_indexing != 0
            }
            VkFeatures12::ShaderUniformTexelBufferArrayDynamicIndexing => {
                features.shader_uniform_texel_buffer_array_dynamic_indexing != 0
            }
            VkFeatures12::ShaderStorageTexelBufferArrayDynamicIndexing => {
                features.shader_storage_texel_buffer_array_dynamic_indexing != 0
            }
            VkFeatures12::ShaderUniformBufferArrayNonUniformIndexing => {
                features.shader_uniform_buffer_array_non_uniform_indexing != 0
            }
            VkFeatures12::ShaderSampledImageArrayNonUniformIndexing => {
                features.shader_sampled_image_array_non_uniform_indexing != 0
            }
            VkFeatures12::ShaderStorageBufferArrayNonUniformIndexing => {
                features.shader_storage_buffer_array_non_uniform_indexing != 0
            }
            VkFeatures12::ShaderStorageImageArrayNonUniformIndexing => {
                features.shader_storage_image_array_non_uniform_indexing != 0
            }
            VkFeatures12::DescriptorBindingUniformBufferUpdateAfterBind => {
                features.descriptor_binding_uniform_buffer_update_after_bind != 0
            }
            VkFeatures12::DescriptorBindingSampledImageUpdateAfterBind => {
                features.descriptor_binding_sampled_image_update_after_bind != 0
            }
            VkFeatures12::DescriptorBindingStorageImageUpdateAfterBind => {
                features.descriptor_binding_storage_image_update_after_bind != 0
            }
            VkFeatures12::DescriptorBindingStorageBufferUpdateAfterBind => {
                features.descriptor_binding_storage_buffer_update_after_bind != 0
            }
            VkFeatures12::DescriptorBindingUniformTexelBufferUpdateAfterBind => {
                features.descriptor_binding_uniform_texel_buffer_update_after_bind != 0
            }
            VkFeatures12::DescriptorBindingStorageTexelBufferUpdateAfterBind => {
                features.descriptor_binding_storage_texel_buffer_update_after_bind != 0
            }
            VkFeatures12::DescriptorBindingPartiallyBound => {
                features.descriptor_binding_partially_bound != 0
            }
            VkFeatures12::DescriptorBindingVariableDescriptorCount => {
                features.descriptor_binding_variable_descriptor_count != 0
            }
            VkFeatures12::DescriptorBindingUpdateUnusedWhilePending => {
                features.descriptor_binding_update_unused_while_pending != 0
            }
            VkFeatures12::RuntimeDescriptorArray => features.runtime_descriptor_array != 0,
            VkFeatures12::ScalarBlockLayout => features.scalar_block_layout != 0,
            VkFeatures12::ImagelessFramebuffer => features.imageless_framebuffer != 0,
            VkFeatures12::UniformBufferStandardLayout => {
                features.uniform_buffer_standard_layout != 0
            }
            VkFeatures12::ShaderSubgroupExtendedTypes => {
                features.shader_subgroup_extended_types != 0
            }
            VkFeatures12::SeparateDepthStencilLayouts => {
                features.separate_depth_stencil_layouts != 0
            }
            VkFeatures12::HostQueryReset => features.host_query_reset != 0,
            VkFeatures12::TimelineSemaphore => features.timeline_semaphore != 0,
            VkFeatures12::BufferDeviceAddress => features.buffer_device_address != 0,
            VkFeatures12::BufferDeviceAddressCaptureReplay => {
                features.buffer_device_address_capture_replay != 0
            }
            VkFeatures12::BufferDeviceAddressMultiDevice => {
                features.buffer_device_address_multi_device != 0
            }
            VkFeatures12::VulkanMemoryModel => features.vulkan_memory_model != 0,
            VkFeatures12::VulkanMemoryModelDeviceScope => {
                features.vulkan_memory_model_device_scope != 0
            }
            VkFeatures12::VulkanMemoryModelAvailabilityVisibilityChains => {
                features.vulkan_memory_model_availability_visibility_chains != 0
            }
            VkFeatures12::ShaderOutputViewportIndex => features.shader_output_viewport_index != 0,
            VkFeatures12::ShaderOutputLayer => features.shader_output_layer != 0,
            VkFeatures12::SubgroupBroadcastDynamicId => features.subgroup_broadcast_dynamic_id != 0,
        }
    }
}

/// Custom enum for Vulkan 1.3 features names
/// https://docs.vulkan.org/spec/latest/chapters/features.html#VkPhysicalDeviceVulkan13Features
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub(crate) enum VkFeatures13 {
    /// Enables robust image access bounds checking for shader image loads
    RobustImageAccess,
    /// Enables support for inline uniform blocks
    InlineUniformBlock,
    /// Allows updating inline uniform block descriptors after bind
    DescriptorBindingInlineUniformBlockUpdateAfterBind,
    /// Enables control over pipeline creation cache behaviour
    PipelineCreationCacheControl,
    /// Enables private data slots for objects
    PrivateData,
    /// Allows shaders to demote to helper invocations
    ShaderDemoteToHelperInvocation,
    /// Allows early termination in fragment shaders
    ShaderTerminateInvocation,
    /// Enables subgroup size control
    SubgroupSizeControl,
    /// Enables full subgroup compute support
    ComputeFullSubgroups,
    /// Enables Vulkan synchronization2 enhanced sync APIs
    Synchronization2,
    /// Enables ASTC HDR texture compression formats
    TextureCompressionAstcHdr,
    /// Enables zero initialization of workgroup memory in shaders
    ShaderZeroInitializeWorkgroupMemory,
    /// Enables dynamic rendering without render passes
    DynamicRendering,
    /// Enables integer dot product operations in shaders
    ShaderIntegerDotProduct,
    /// Enables maintenance4 style functionality
    Maintenance4,
}

impl VkFeatures13 {
    /// Converts as Ash features
    pub(crate) fn as_ash_features<'a>(
        features: &'a [Self],
    ) -> ash::vk::PhysicalDeviceVulkan13Features<'a> {
        let mut ash_features = ash::vk::PhysicalDeviceVulkan13Features::default();
        for feature in features {
            ash_features = match feature {
                VkFeatures13::RobustImageAccess => ash_features.robust_image_access(true),
                VkFeatures13::InlineUniformBlock => ash_features.inline_uniform_block(true),
                VkFeatures13::DescriptorBindingInlineUniformBlockUpdateAfterBind => {
                    ash_features.descriptor_binding_inline_uniform_block_update_after_bind(true)
                }
                VkFeatures13::PipelineCreationCacheControl => {
                    ash_features.pipeline_creation_cache_control(true)
                }
                VkFeatures13::PrivateData => ash_features.private_data(true),
                VkFeatures13::ShaderDemoteToHelperInvocation => {
                    ash_features.shader_demote_to_helper_invocation(true)
                }
                VkFeatures13::ShaderTerminateInvocation => {
                    ash_features.shader_terminate_invocation(true)
                }
                VkFeatures13::SubgroupSizeControl => ash_features.subgroup_size_control(true),
                VkFeatures13::ComputeFullSubgroups => ash_features.compute_full_subgroups(true),
                VkFeatures13::Synchronization2 => ash_features.synchronization2(true),
                VkFeatures13::TextureCompressionAstcHdr => {
                    ash_features.texture_compression_astc_hdr(true)
                }
                VkFeatures13::ShaderZeroInitializeWorkgroupMemory => {
                    ash_features.shader_zero_initialize_workgroup_memory(true)
                }
                VkFeatures13::DynamicRendering => ash_features.dynamic_rendering(true),
                VkFeatures13::ShaderIntegerDotProduct => {
                    ash_features.shader_integer_dot_product(true)
                }
                VkFeatures13::Maintenance4 => ash_features.maintenance4(true),
            }
        }

        ash_features
    }

    /// Returns true if the feature is enabled
    pub(crate) fn is_enabled(
        &self,
        features: &ash::vk::PhysicalDeviceVulkan13Features<'_>,
    ) -> bool {
        match self {
            VkFeatures13::RobustImageAccess => features.robust_image_access != 0,
            VkFeatures13::InlineUniformBlock => features.inline_uniform_block != 0,
            VkFeatures13::DescriptorBindingInlineUniformBlockUpdateAfterBind => {
                features.descriptor_binding_inline_uniform_block_update_after_bind != 0
            }
            VkFeatures13::PipelineCreationCacheControl => {
                features.pipeline_creation_cache_control != 0
            }
            VkFeatures13::PrivateData => features.private_data != 0,
            VkFeatures13::ShaderDemoteToHelperInvocation => {
                features.shader_demote_to_helper_invocation != 0
            }
            VkFeatures13::ShaderTerminateInvocation => features.shader_terminate_invocation != 0,
            VkFeatures13::SubgroupSizeControl => features.subgroup_size_control != 0,
            VkFeatures13::ComputeFullSubgroups => features.compute_full_subgroups != 0,
            VkFeatures13::Synchronization2 => features.synchronization2 != 0,
            VkFeatures13::TextureCompressionAstcHdr => features.texture_compression_astc_hdr != 0,
            VkFeatures13::ShaderZeroInitializeWorkgroupMemory => {
                features.shader_zero_initialize_workgroup_memory != 0
            }
            VkFeatures13::DynamicRendering => features.dynamic_rendering != 0,
            VkFeatures13::ShaderIntegerDotProduct => features.shader_integer_dot_product != 0,
            VkFeatures13::Maintenance4 => features.maintenance4 != 0,
        }
    }
}

/// Custom enum for Vulkan 1.4 features names
/// https://docs.vulkan.org/spec/latest/chapters/features.html#VkPhysicalDeviceVulkan14Features
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub(crate) enum VkFeatures14 {
    /// Support querying global queue priorities
    GlobalPriorityQuery,
    /// Shader subgroup rotate operations support
    ShaderSubgroupRotate,
    /// Clustered subgroup rotate operations support
    ShaderSubgroupRotateClustered,
    /// Supports additional float controls in shaders
    ShaderFloatControls2,
    /// Shader expect/assume operations support
    ShaderExpectAssume,
    /// Rectangular line rasterization support
    RectangularLines,
    /// Bresenham line rasterization support
    BresenhamLines,
    /// Smooth line rasterization support
    SmoothLines,
    /// Rectangular stippled line rasterization
    StippledRectangularLines,
    /// Bresenham stippled line rasterization
    StippledBresenhamLines,
    /// Smooth stippled line rasterization
    StippledSmoothLines,
    /// Vertex attribute instance rate divisor support
    VertexAttributeInstanceRateDivisor,
    /// Zero divisor vertex attribute instance rate support
    VertexAttributeInstanceRateZeroDivisor,
    /// Support for 8-bit index type
    IndexTypeUint8,
    /// Dynamic rendering local read support
    DynamicRenderingLocalRead,
    /// Maintenance5 support
    Maintenance5,
    /// Maintenance6 support
    Maintenance6,
    /// Pipeline protected access support
    PipelineProtectedAccess,
    /// Pipeline robustness control support
    PipelineRobustness,
    /// Host image copy support
    HostImageCopy,
    /// Push descriptor support
    PushDescriptor,
}

// TODO: for future ash update
// impl VkFeatures14 {
//     /// Converts as Ash features
// pub(crate) fn as_ash_features<'a>(features: &'a[Self]) -> ash::vk::PhysicalDeviceVulkan12Features<'a> {
//     let mut ash_features = ash::vk::PhysicalDeviceVulkan12Features::default();
//     for feature in features {
//         ash_features = match feature {
//         }
//     }

//     ash_features
// }

//     /// Returns true if the feature is enabled
//     pub(crate) fn is_enabled(
//         &self,
//         features: &ash::vk::PhysicalDeviceVulkan14Features<'_>,
//     ) -> bool {
//         match self {
//             VkFeatures14::GlobalPriorityQuery => features.global_priority_query != 0,
//             VkFeatures14::ShaderSubgroupRotate => features.shader_subgroup_rotate != 0,
//             VkFeatures14::ShaderSubgroupRotateClustered => {
//                 features.shader_subgroup_rotate_clustered != 0
//             }
//             VkFeatures14::ShaderFloatControls2 => features.shader_float_controls2 != 0,
//             VkFeatures14::ShaderExpectAssume => features.shader_expect_assume != 0,
//             VkFeatures14::RectangularLines => features.rectangular_lines != 0,
//             VkFeatures14::BresenhamLines => features.bresenham_lines != 0,
//             VkFeatures14::SmoothLines => features.smooth_lines != 0,
//             VkFeatures14::StippledRectangularLines => {
//                 features.stippled_rectangular_lines != 0
//             }
//             VkFeatures14::StippledBresenhamLines => {
//                 features.stippled_bresenham_lines != 0
//             }
//             VkFeatures14::StippledSmoothLines => features.stippled_smooth_lines != 0,
//             VkFeatures14::VertexAttributeInstanceRateDivisor => {
//                 features.vertex_attribute_instance_rate_divisor != 0
//             }
//             VkFeatures14::VertexAttributeInstanceRateZeroDivisor => {
//                 features.vertex_attribute_instance_rate_zero_divisor != 0
//             }
//             VkFeatures14::IndexTypeUint8 => features.index_type_uint8 != 0,
//             VkFeatures14::DynamicRenderingLocalRead => {
//                 features.dynamic_rendering_local_read != 0
//             }
//             VkFeatures14::Maintenance5 => features.maintenance5 != 0,
//             VkFeatures14::Maintenance6 => features.maintenance6 != 0,
//             VkFeatures14::PipelineProtectedAccess => {
//                 features.pipeline_protected_access != 0
//             }
//             VkFeatures14::PipelineRobustness => features.pipeline_robustness != 0,
//             VkFeatures14::HostImageCopy => features.host_image_copy != 0,
//             VkFeatures14::PushDescriptor => features.push_descriptor != 0,
//         }
//     }
// }

/// Custom enum for Vulkan features names coming from extensions
#[derive(Debug, Clone, Copy)]
#[allow(unused)]
pub(crate) enum VkFeaturesExt {
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicStateFeaturesEXT.html
    ExtendedDynamicState,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html
    ExtendedDynamicState2,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html
    ExtendedDynamicState2LogicOp,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState2FeaturesEXT.html
    ExtendedDynamicState2PatchControlPoints,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3TessellationDomainOrigin,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3DepthClampEnable,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3PolygonMode,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3RasterizationSamples,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3SampleMask,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3AlphaToCoverageEnable,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3AlphaToOneEnable,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3LogicOpEnable,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3ColorBlendEnable,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3ColorBlendEquation,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3ColorWriteMask,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3RasterizationStream,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3ConservativeRasterizationMode,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3ExtraPrimitiveOverestimationSize,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3DepthClipEnable,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3SampleLocationsEnable,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3ColorBlendAdvanced,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3ProvokingVertexMode,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3LineRasterizationMode,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3LineStippleEnable,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3DepthClipNegativeOneToOne,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3ViewportWScalingEnable,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3ViewportSwizzle,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3CoverageToColorEnable,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3CoverageToColorLocation,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3CoverageModulationMode,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3CoverageModulationTableEnable,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3CoverageModulationTable,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3CoverageReductionMode,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3RepresentativeFragmentTestEnable,
    /// https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceExtendedDynamicState3FeaturesEXT.html
    ExtendedDynamicState3ShadingRateImageEnable,
}

impl VkFeaturesExt {
    /// Converts as Ash features
    pub(crate) fn as_extended_dynamic_state<'a>(
        features: &'a [Self],
    ) -> ash::vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT<'a> {
        let mut ash_features = ash::vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT::default();
        for feature in features {
            ash_features = match feature {
                VkFeaturesExt::ExtendedDynamicState => ash_features.extended_dynamic_state(true),
                _ => ash_features,
            }
        }
        ash_features
    }

    /// Returns true if the feature is enabled
    pub(crate) fn is_enabled_extended_dynamic_state(
        &self,
        features: &ash::vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT<'_>,
    ) -> bool {
        match self {
            VkFeaturesExt::ExtendedDynamicState => features.extended_dynamic_state != 0,
            _ => true,
        }
    }
}

impl VkFeaturesExt {
    /// Converts as Ash features
    pub(crate) fn as_extended_dynamic_state2<'a>(
        features: &'a [Self],
    ) -> ash::vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT<'a> {
        let mut ash_features = ash::vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT::default();
        for feature in features {
            ash_features = match feature {
                VkFeaturesExt::ExtendedDynamicState2 => ash_features.extended_dynamic_state2(true),
                VkFeaturesExt::ExtendedDynamicState2LogicOp => {
                    ash_features.extended_dynamic_state2_logic_op(true)
                }
                VkFeaturesExt::ExtendedDynamicState2PatchControlPoints => {
                    ash_features.extended_dynamic_state2_patch_control_points(true)
                }
                _ => ash_features,
            }
        }
        ash_features
    }

    /// Returns true if the feature is enabled
    pub(crate) fn is_enabled_extended_dynamic_state2(
        &self,
        features: &ash::vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT<'_>,
    ) -> bool {
        match self {
            VkFeaturesExt::ExtendedDynamicState2 => features.extended_dynamic_state2 != 0,
            VkFeaturesExt::ExtendedDynamicState2LogicOp => {
                features.extended_dynamic_state2_logic_op != 0
            }
            VkFeaturesExt::ExtendedDynamicState2PatchControlPoints => {
                features.extended_dynamic_state2_patch_control_points != 0
            }
            _ => true,
        }
    }
}

impl VkFeaturesExt {
    /// Converts as Ash features
    pub(crate) fn as_extended_dynamic_state3<'a>(
        features: &'a [Self],
    ) -> ash::vk::PhysicalDeviceExtendedDynamicState3FeaturesEXT<'a> {
        let mut ash_features = ash::vk::PhysicalDeviceExtendedDynamicState3FeaturesEXT::default();
        for feature in features {
            ash_features = match feature {
                VkFeaturesExt::ExtendedDynamicState3TessellationDomainOrigin => {
                    ash_features.extended_dynamic_state3_tessellation_domain_origin(true)
                }
                VkFeaturesExt::ExtendedDynamicState3DepthClampEnable => {
                    ash_features.extended_dynamic_state3_depth_clamp_enable(true)
                }
                VkFeaturesExt::ExtendedDynamicState3PolygonMode => {
                    ash_features.extended_dynamic_state3_polygon_mode(true)
                }
                VkFeaturesExt::ExtendedDynamicState3RasterizationSamples => {
                    ash_features.extended_dynamic_state3_rasterization_samples(true)
                }
                VkFeaturesExt::ExtendedDynamicState3SampleMask => {
                    ash_features.extended_dynamic_state3_sample_mask(true)
                }
                VkFeaturesExt::ExtendedDynamicState3AlphaToCoverageEnable => {
                    ash_features.extended_dynamic_state3_alpha_to_coverage_enable(true)
                }
                VkFeaturesExt::ExtendedDynamicState3AlphaToOneEnable => {
                    ash_features.extended_dynamic_state3_alpha_to_one_enable(true)
                }
                VkFeaturesExt::ExtendedDynamicState3LogicOpEnable => {
                    ash_features.extended_dynamic_state3_logic_op_enable(true)
                }
                VkFeaturesExt::ExtendedDynamicState3ColorBlendEnable => {
                    ash_features.extended_dynamic_state3_color_blend_enable(true)
                }
                VkFeaturesExt::ExtendedDynamicState3ColorBlendEquation => {
                    ash_features.extended_dynamic_state3_color_blend_equation(true)
                }
                VkFeaturesExt::ExtendedDynamicState3ColorWriteMask => {
                    ash_features.extended_dynamic_state3_color_write_mask(true)
                }
                VkFeaturesExt::ExtendedDynamicState3RasterizationStream => {
                    ash_features.extended_dynamic_state3_rasterization_stream(true)
                }
                VkFeaturesExt::ExtendedDynamicState3ConservativeRasterizationMode => {
                    ash_features.extended_dynamic_state3_conservative_rasterization_mode(true)
                }
                VkFeaturesExt::ExtendedDynamicState3ExtraPrimitiveOverestimationSize => {
                    ash_features.extended_dynamic_state3_extra_primitive_overestimation_size(true)
                }
                VkFeaturesExt::ExtendedDynamicState3DepthClipEnable => {
                    ash_features.extended_dynamic_state3_depth_clip_enable(true)
                }
                VkFeaturesExt::ExtendedDynamicState3SampleLocationsEnable => {
                    ash_features.extended_dynamic_state3_sample_locations_enable(true)
                }
                VkFeaturesExt::ExtendedDynamicState3ColorBlendAdvanced => {
                    ash_features.extended_dynamic_state3_color_blend_advanced(true)
                }
                VkFeaturesExt::ExtendedDynamicState3ProvokingVertexMode => {
                    ash_features.extended_dynamic_state3_provoking_vertex_mode(true)
                }
                VkFeaturesExt::ExtendedDynamicState3LineRasterizationMode => {
                    ash_features.extended_dynamic_state3_line_rasterization_mode(true)
                }
                VkFeaturesExt::ExtendedDynamicState3LineStippleEnable => {
                    ash_features.extended_dynamic_state3_line_stipple_enable(true)
                }
                VkFeaturesExt::ExtendedDynamicState3DepthClipNegativeOneToOne => {
                    ash_features.extended_dynamic_state3_depth_clip_negative_one_to_one(true)
                }
                VkFeaturesExt::ExtendedDynamicState3ViewportWScalingEnable => {
                    ash_features.extended_dynamic_state3_viewport_w_scaling_enable(true)
                }
                VkFeaturesExt::ExtendedDynamicState3ViewportSwizzle => {
                    ash_features.extended_dynamic_state3_viewport_swizzle(true)
                }
                VkFeaturesExt::ExtendedDynamicState3CoverageToColorEnable => {
                    ash_features.extended_dynamic_state3_coverage_to_color_enable(true)
                }
                VkFeaturesExt::ExtendedDynamicState3CoverageToColorLocation => {
                    ash_features.extended_dynamic_state3_coverage_to_color_location(true)
                }
                VkFeaturesExt::ExtendedDynamicState3CoverageModulationMode => {
                    ash_features.extended_dynamic_state3_coverage_modulation_mode(true)
                }
                VkFeaturesExt::ExtendedDynamicState3CoverageModulationTableEnable => {
                    ash_features.extended_dynamic_state3_coverage_modulation_table_enable(true)
                }
                VkFeaturesExt::ExtendedDynamicState3CoverageModulationTable => {
                    ash_features.extended_dynamic_state3_coverage_modulation_table(true)
                }
                VkFeaturesExt::ExtendedDynamicState3CoverageReductionMode => {
                    ash_features.extended_dynamic_state3_coverage_reduction_mode(true)
                }
                VkFeaturesExt::ExtendedDynamicState3RepresentativeFragmentTestEnable => {
                    ash_features.extended_dynamic_state3_representative_fragment_test_enable(true)
                }
                VkFeaturesExt::ExtendedDynamicState3ShadingRateImageEnable => {
                    ash_features.extended_dynamic_state3_shading_rate_image_enable(true)
                }
                _ => ash_features,
            }
        }
        ash_features
    }

    /// Returns true if the feature is enabled
    pub(crate) fn is_enabled_extended_dynamic_state3(
        &self,
        features: &ash::vk::PhysicalDeviceExtendedDynamicState3FeaturesEXT<'_>,
    ) -> bool {
        match self {
            VkFeaturesExt::ExtendedDynamicState3TessellationDomainOrigin => {
                features.extended_dynamic_state3_tessellation_domain_origin != 0
            }
            VkFeaturesExt::ExtendedDynamicState3DepthClampEnable => {
                features.extended_dynamic_state3_depth_clamp_enable != 0
            }
            VkFeaturesExt::ExtendedDynamicState3PolygonMode => {
                features.extended_dynamic_state3_polygon_mode != 0
            }
            VkFeaturesExt::ExtendedDynamicState3RasterizationSamples => {
                features.extended_dynamic_state3_rasterization_samples != 0
            }
            VkFeaturesExt::ExtendedDynamicState3SampleMask => {
                features.extended_dynamic_state3_sample_mask != 0
            }
            VkFeaturesExt::ExtendedDynamicState3AlphaToCoverageEnable => {
                features.extended_dynamic_state3_alpha_to_coverage_enable != 0
            }
            VkFeaturesExt::ExtendedDynamicState3AlphaToOneEnable => {
                features.extended_dynamic_state3_alpha_to_one_enable != 0
            }
            VkFeaturesExt::ExtendedDynamicState3LogicOpEnable => {
                features.extended_dynamic_state3_logic_op_enable != 0
            }
            VkFeaturesExt::ExtendedDynamicState3ColorBlendEnable => {
                features.extended_dynamic_state3_color_blend_enable != 0
            }
            VkFeaturesExt::ExtendedDynamicState3ColorBlendEquation => {
                features.extended_dynamic_state3_color_blend_equation != 0
            }
            VkFeaturesExt::ExtendedDynamicState3ColorWriteMask => {
                features.extended_dynamic_state3_color_write_mask != 0
            }
            VkFeaturesExt::ExtendedDynamicState3RasterizationStream => {
                features.extended_dynamic_state3_rasterization_stream != 0
            }
            VkFeaturesExt::ExtendedDynamicState3ConservativeRasterizationMode => {
                features.extended_dynamic_state3_conservative_rasterization_mode != 0
            }
            VkFeaturesExt::ExtendedDynamicState3ExtraPrimitiveOverestimationSize => {
                features.extended_dynamic_state3_extra_primitive_overestimation_size != 0
            }
            VkFeaturesExt::ExtendedDynamicState3DepthClipEnable => {
                features.extended_dynamic_state3_depth_clip_enable != 0
            }
            VkFeaturesExt::ExtendedDynamicState3SampleLocationsEnable => {
                features.extended_dynamic_state3_sample_locations_enable != 0
            }
            VkFeaturesExt::ExtendedDynamicState3ColorBlendAdvanced => {
                features.extended_dynamic_state3_color_blend_advanced != 0
            }
            VkFeaturesExt::ExtendedDynamicState3ProvokingVertexMode => {
                features.extended_dynamic_state3_provoking_vertex_mode != 0
            }
            VkFeaturesExt::ExtendedDynamicState3LineRasterizationMode => {
                features.extended_dynamic_state3_line_rasterization_mode != 0
            }
            VkFeaturesExt::ExtendedDynamicState3LineStippleEnable => {
                features.extended_dynamic_state3_line_stipple_enable != 0
            }
            VkFeaturesExt::ExtendedDynamicState3DepthClipNegativeOneToOne => {
                features.extended_dynamic_state3_depth_clip_negative_one_to_one != 0
            }
            VkFeaturesExt::ExtendedDynamicState3ViewportWScalingEnable => {
                features.extended_dynamic_state3_viewport_w_scaling_enable != 0
            }
            VkFeaturesExt::ExtendedDynamicState3ViewportSwizzle => {
                features.extended_dynamic_state3_viewport_swizzle != 0
            }
            VkFeaturesExt::ExtendedDynamicState3CoverageToColorEnable => {
                features.extended_dynamic_state3_coverage_to_color_enable != 0
            }
            VkFeaturesExt::ExtendedDynamicState3CoverageToColorLocation => {
                features.extended_dynamic_state3_coverage_to_color_location != 0
            }
            VkFeaturesExt::ExtendedDynamicState3CoverageModulationMode => {
                features.extended_dynamic_state3_coverage_modulation_mode != 0
            }
            VkFeaturesExt::ExtendedDynamicState3CoverageModulationTableEnable => {
                features.extended_dynamic_state3_coverage_modulation_table_enable != 0
            }
            VkFeaturesExt::ExtendedDynamicState3CoverageModulationTable => {
                features.extended_dynamic_state3_coverage_modulation_table != 0
            }
            VkFeaturesExt::ExtendedDynamicState3CoverageReductionMode => {
                features.extended_dynamic_state3_coverage_reduction_mode != 0
            }
            VkFeaturesExt::ExtendedDynamicState3RepresentativeFragmentTestEnable => {
                features.extended_dynamic_state3_representative_fragment_test_enable != 0
            }
            VkFeaturesExt::ExtendedDynamicState3ShadingRateImageEnable => {
                features.extended_dynamic_state3_shading_rate_image_enable != 0
            }
            _ => true,
        }
    }
}
