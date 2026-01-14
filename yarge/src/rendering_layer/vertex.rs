use std::mem::offset_of;

#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::{maths::{Vector2, Vector3, Vector4}, renderer_types::ImageFormat, shaders::ShaderLocation};

pub(crate) const VERTEX_ATTRIBUTE_BINDING_PACKED: usize = 0;
pub(crate) const VERTEX_ATTRIBUTE_OFFSET_MULTI_ARRAY: usize = 0;

/// An attribute in the VertexData
/// [VertexData]
pub(crate) trait VertexDataAttribute {
    /// The format of the attribute
    const FORMAT: ImageFormat;
    /// The position of the attribute in the VertexData struct
    const POSITION: usize;
    /// The offset of the attribute in the VertexData struct
    const OFFSET: usize;
    /// Gets the description of the given attribute when using a single packed attributes array
    fn get_description_packed(&self) -> VertexAttributeDescription {
        VertexAttributeDescription {
            location: ShaderLocation(Self::POSITION),
            binding: VERTEX_ATTRIBUTE_BINDING_PACKED,
            format: Self::FORMAT,
            offset: Self::OFFSET,
        }
    }
    /// Gets the description of the given attribute when using multiple attributes arrays
    fn get_description_multiple_buffers(&self) -> VertexAttributeDescription {
        VertexAttributeDescription {
            location: ShaderLocation(Self::POSITION),
            binding: Self::POSITION,
            format: Self::FORMAT,
            offset: VERTEX_ATTRIBUTE_OFFSET_MULTI_ARRAY,
        }
    }
}

/// The object space position
/// [VertexData]
pub(crate) struct VertexPosition(Vector3);
impl VertexDataAttribute for VertexPosition {
    const FORMAT: ImageFormat = ImageFormat::R32G32B32_SFLOAT;
    const POSITION: usize = 0;
    const OFFSET: usize = offset_of!(VertexData, position);
}

/// The vertex color and opacity
/// [VertexData]
pub(crate) struct VertexColor(Vector4);
impl VertexDataAttribute for VertexColor {
    const FORMAT: ImageFormat = ImageFormat::R32G32B32A32_SFLOAT;
    const POSITION: usize = 1;
    const OFFSET: usize = offset_of!(VertexData, color);
}

/// The vertex normal
/// [VertexData]
pub(crate) struct VertexNormal(Vector3);
impl VertexDataAttribute for VertexNormal {
    const FORMAT: ImageFormat = ImageFormat::R32G32B32_SFLOAT;
    const POSITION: usize = 2;
    const OFFSET: usize = offset_of!(VertexData, normal);
}

/// The vertex texture coordinates
/// [VertexData]
pub(crate) struct VertexTexCoords(Vector2);
impl VertexDataAttribute for VertexTexCoords {
    const FORMAT: ImageFormat = ImageFormat::R32G32_SFLOAT;
    const POSITION: usize = 3;
    const OFFSET: usize = offset_of!(VertexData, tex_coordinates);
}


/// A structure representing the data contained in a single vertex
pub(crate) struct VertexData {
    /// The object space position
    pub(crate) position: VertexPosition,
    /// The vertex color and opacity
    pub(crate) color: VertexColor,
    /// The vertex normal
    pub(crate) normal: VertexNormal,
    /// The vertex texture coordinates
    pub(crate) tex_coordinates: VertexTexCoords,
}

/// The rate at which vertex attributes are pulled from buffers
pub(crate) enum VertexInputRate {
    /// Function of the vertex index
    Vertex,
    /// Function of the instance index
    Instance,
}

/// A vertex input binding description
pub(crate) struct VertexBindingDescription {
    /// The binding number that this structure describes
    /// Usually 0 for the position, 1 for the color, 2 for the normal and 3 for the texture coordinates
    pub(crate) binding: usize,
    /// The byte stride between consecutive elements within the buffer
    pub(crate) stride: usize,
    /// The input rate
    pub(crate) input_rate: VertexInputRate,
}

pub(crate) struct VertexAttributeDescription {
    /// The shader input location number for this attribute
    pub(crate) location: ShaderLocation,
    /// The binding number which this attribute takes its data from
    /// Matches the location for multi array attributes, otherwise always 0
    pub(crate) binding: usize,
    /// The size and type of the vertex attribute data
    pub(crate) format: ImageFormat,
    /// The byte offset of this attribute relative to the start of an element in the vertex input binding
    /// Will always be 0 for multi array attributes
    pub(crate) offset: usize,
}