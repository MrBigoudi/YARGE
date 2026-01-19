#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{
    config::Version,
    renderer_types::{formats::ImageFormat, present::PresentMode, usages::ImageUsage},
    rendering_layer::rendering_impl::types::{
        extensions::{VkDeviceExtensions, VkInstanceExtensions},
        features::{
            VkFeatures10, VkFeatures11, VkFeatures12, VkFeatures13, VkFeatures14, VkFeaturesExt,
        },
        layers::VkLayers,
    },
};

#[derive(Debug, Clone)]
/// The config for a Vulkan context
pub(crate) struct VulkanConfig {
    /// The Vulkan version
    pub(crate) version: Version,
    /// The required layers in normal mode
    pub(crate) required_layers: Vec<VkLayers>,
    /// The required layers in debug mode
    pub(crate) required_layers_debug: Vec<VkLayers>,
    /// The required instance extensions in normal mode
    pub(crate) required_instance_extensions: Vec<VkInstanceExtensions>,
    /// The required instanceextensions in debug mode
    pub(crate) required_instance_extensions_debug: Vec<VkInstanceExtensions>,
    /// The required physical device core 1.0 features
    pub(crate) required_physical_device_features_1_0: Vec<VkFeatures10>,
    /// The required physical device 1.1 features
    pub(crate) required_physical_device_features_1_1: Vec<VkFeatures11>,
    /// The required physical device 1.2 features
    pub(crate) required_physical_device_features_1_2: Vec<VkFeatures12>,
    /// The required physical device 1.3 features
    pub(crate) required_physical_device_features_1_3: Vec<VkFeatures13>,
    /// The required physical device 1.4 features
    pub(crate) required_physical_device_features_1_4: Vec<VkFeatures14>,
    /// The required extension features
    pub(crate) required_device_features_ext: Vec<VkFeaturesExt>,
    /// The required device extensions in normal mode
    pub(crate) required_device_extensions: Vec<VkDeviceExtensions>,
    /// The prefered swapchain formats in order of preference
    pub(crate) prefered_swapchain_formats: Vec<ImageFormat>,
    /// The prefered swapchain present modes in order of preference
    pub(crate) prefered_swapchain_present_modes: Vec<PresentMode>,
    /// The prefered minimum number of images in the swapchain
    pub(crate) prefered_swapchain_min_image_count: u32,
    /// The image usage flags for the swapchain
    pub(crate) swapchain_image_usages: Vec<ImageUsage>,
}

impl Default for VulkanConfig {
    fn default() -> Self {
        let version = Version::default().major(1).minor(4);
        let required_layers = Vec::new();
        let required_layers_debug = vec![VkLayers::KhrValidation(vec![
            super::layers::VkValidationLayerSettings::ThreadSafety,
            super::layers::VkValidationLayerSettings::DisplayApplicationName,
            super::layers::VkValidationLayerSettings::BestPractices(vec![]),
            super::layers::VkValidationLayerSettings::Synchronization(vec![
                super::layers::VkValidationLayerSyncValSettings::ShaderAccessesHeuristic,
                super::layers::VkValidationLayerSyncValSettings::MessageExtraProperties,
            ]),
            super::layers::VkValidationLayerSettings::DebugPrintf(vec![
                super::layers::VkValidationLayerPrintfSettings::Verbose,
            ]),
            // TODO: Enable when checking for GPU validation errors aget all CoreCheck errors are solved
            // super::layers::VkValidationLayerSettings::GpuAssistedValidation(vec![
            //     super::layers::VkValidationLayerGpuavSettings::SafeMode,
            //     super::layers::VkValidationLayerGpuavSettings::ForceOnRobustnessFeatures,
            // ]),
        ])];

        let required_instance_extensions = vec![VkInstanceExtensions::KhrSurface];
        let required_instance_extensions_debug = vec![VkInstanceExtensions::ExtDebugUtils];

        let required_physical_device_features_1_0 = vec![
            VkFeatures10::GeometryShader,
            VkFeatures10::TessellationShader,
            VkFeatures10::DepthClamp,
            VkFeatures10::FillModeNonSolid,
            VkFeatures10::WideLines,
            VkFeatures10::MultiViewport,
            VkFeatures10::VertexPipelineStoresAndAtomics,
            VkFeatures10::FragmentStoresAndAtomics,
            VkFeatures10::ShaderUniformBufferArrayDynamicIndexing,
            VkFeatures10::ShaderSampledImageArrayDynamicIndexing,
            VkFeatures10::ShaderStorageBufferArrayDynamicIndexing,
            VkFeatures10::ShaderStorageImageArrayDynamicIndexing,
            // VkFeatures10::ShaderFloat64,
        ];
        let required_physical_device_features_1_1 = vec![
            VkFeatures11::Multiview,
            VkFeatures11::MultiviewGeometryShader,
            VkFeatures11::MultiviewTessellationShader,
        ];
        let required_physical_device_features_1_2 = vec![
            VkFeatures12::ShaderBufferInt64Atomics,
            // VkFeatures12::ShaderSharedInt64Atomics,
            // VkFeatures12::ShaderInputAttachmentArrayDynamicIndexing,
            VkFeatures12::ShaderUniformTexelBufferArrayDynamicIndexing,
            VkFeatures12::ShaderStorageTexelBufferArrayDynamicIndexing,
        ];
        let required_physical_device_features_1_3 = vec![VkFeatures13::DynamicRendering];
        let required_physical_device_features_1_4 = vec![VkFeatures14::HostImageCopy];
        let required_device_features_ext = vec![
            VkFeaturesExt::ExtendedDynamicState,
            VkFeaturesExt::ExtendedDynamicState2,
            VkFeaturesExt::ExtendedDynamicState2LogicOp,
            VkFeaturesExt::ExtendedDynamicState2PatchControlPoints,
        ];

        let required_device_extensions = vec![
            VkDeviceExtensions::KhrSwapchain,
            VkDeviceExtensions::KhrSpirV14,
            VkDeviceExtensions::KhrSynchronization2,
            VkDeviceExtensions::KhrCreateRenderpass2,
        ];

        let prefered_swapchain_formats =
            vec![ImageFormat::R8G8B8A8_SRGB, ImageFormat::B8G8R8A8_SRGB];

        let prefered_swapchain_present_modes =
            vec![PresentMode::TripleBuffering, PresentMode::Vsync];

        let prefered_swapchain_min_image_count = 3u32;

        let swapchain_image_usages = vec![
            ImageUsage::ColorAttachment,
            ImageUsage::TransferDst, // To copy to swapchain images for indirect rendering
            ImageUsage::TransferSrc, // For screens shots
            ImageUsage::Sample,      // For UI compositing
        ];

        Self {
            version,
            required_layers,
            required_layers_debug,
            required_instance_extensions,
            required_instance_extensions_debug,
            required_physical_device_features_1_0,
            required_physical_device_features_1_1,
            required_physical_device_features_1_2,
            required_physical_device_features_1_3,
            required_physical_device_features_1_4,
            required_device_features_ext,
            required_device_extensions,
            prefered_swapchain_formats,
            prefered_swapchain_present_modes,
            prefered_swapchain_min_image_count,
            swapchain_image_usages,
        }
    }
}
