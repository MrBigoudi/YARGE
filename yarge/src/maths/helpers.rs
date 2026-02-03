#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// The pi constant
pub const PI: f32 = std::f32::consts::PI;
/// pi / 2
pub const HALF_PI: f32 = PI / 2f32;
/// pi / 4
pub const QUARTER_PI: f32 = PI / 4f32;
/// 3*pi / 4
pub const THREE_QUARTER_PI: f32 = 3f32 * PI / 4f32;
/// 2pi
pub const TWO_PI: f32 = 2f32 * PI;
/// -pi
pub const NEG_PI: f32 = -PI;
/// -pi / 2
pub const NEG_HALF_PI: f32 = -HALF_PI;
/// -pi / 4
pub const NEG_QUARTER_PI: f32 = -QUARTER_PI;
/// -3*pi / 4
pub const NEG_THREE_QUARTER_PI: f32 = -THREE_QUARTER_PI;

/// Transforms a degree angle to radians
pub fn to_radians(angle_in_degrees: f32) -> f32 {
    (PI / 180f32) * (angle_in_degrees % 360f32)
}

/// Transforms a radian angle to degrees
pub fn to_degrees(angle_in_radians: f32) -> f32 {
    (180f32 / PI) * (angle_in_radians % TWO_PI)
}
