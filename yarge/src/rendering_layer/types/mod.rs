#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

pub(crate) mod formats;
pub(crate) mod present;
pub(crate) mod usages;

/// The possible output for the begin frame
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum RendererBeginFrameOutput {
    /// Can pursue to the end frame
    Success,
    /// Should not present the frame
    Failure,
}

/// The possible topology for rendering primitives
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) enum PrimitiveTopology {
    PointList,
    LineList,
    #[default]
    TriangleList,
    TriangleStrip,
    TriangleFan,
    LineListWithAdjacency,
    LineStripWithAdjacency,
    TriangleListWithAdjacency,
    TriangleStripWithAdjacency,
    PatchList,
}

/// The type of rendering application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) enum RenderingApplicationType {
    /// For most of 3D applications, when displaying to a single screen, no AR, no VR, ...
    #[default]
    Classic3D,
    /// For most of 2D applications, when displaying to a single screen, no AR, no VR, ...
    Classic2D,
    /// For stereoscopic 3D applications
    Stereoscopic3D,
}

/// The transform to apply to the final image before presentation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub(crate) enum FinalTransform {
    /// Automatic depending on the hardware
    #[default]
    Automatic,
    /// No transform
    Identity,
    /// Content rotated 90 degrees clockwise
    Rotate90Clockwise,
    /// Content rotated 90 degrees counter clockwise
    Rotate90CounterClockwise,
    /// Content rotated 180 degrees
    Rotate180,
    /// Content is mirrored horizontally
    HorizontalFlip,
    /// Content is mirrored horizontally then rotated 90 degrees clockwise
    HorizontalFlipRotate90ClockWise,
    /// Content is mirrored horizontally then rotated 90 degrees counter clockwise
    HorizontalFlipRotate90CounterClockWise,
    /// Content is mirrored horizontally then rotated 180 degrees
    HorizontalFlipRotate180,
}
