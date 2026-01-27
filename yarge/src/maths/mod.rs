//! Maths module built with SIMD in mind
//!
//! To learn more about SIMD, please refer to the
//! [portable-simd beginner's guide](https://github.com/rust-lang/portable-simd/blob/master/beginners-guide.md)

/// A module that defines all vector types of the crate
mod vector;

pub use vector::impls::{Vector2f32 as Vector2, vec2f32 as vec2};
pub use vector::impls::{Vector2f32, vec2f32};
pub use vector::impls::{Vector2f64, vec2f64};

pub use vector::impls::{Vector2i8, vec2i8};
pub use vector::impls::{Vector2i16, vec2i16};
pub use vector::impls::{Vector2i32, vec2i32};
pub use vector::impls::{Vector2i64, vec2i64};
pub use vector::impls::{Vector2isize, vec2isize};

pub use vector::impls::{Vector2u8, vec2u8};
pub use vector::impls::{Vector2u16, vec2u16};
pub use vector::impls::{Vector2u32, vec2u32};
pub use vector::impls::{Vector2u64, vec2u64};
pub use vector::impls::{Vector2usize, vec2usize};

pub use vector::impls::{Vector3f32 as Vector3, vec3f32 as vec3};
pub use vector::impls::{Vector3f32, vec3f32};
pub use vector::impls::{Vector3f64, vec3f64};

pub use vector::impls::{Vector3i8, vec3i8};
pub use vector::impls::{Vector3i16, vec3i16};
pub use vector::impls::{Vector3i32, vec3i32};
pub use vector::impls::{Vector3i64, vec3i64};
pub use vector::impls::{Vector3isize, vec3isize};

pub use vector::impls::{Vector3u8, vec3u8};
pub use vector::impls::{Vector3u16, vec3u16};
pub use vector::impls::{Vector3u32, vec3u32};
pub use vector::impls::{Vector3u64, vec3u64};
pub use vector::impls::{Vector3usize, vec3usize};

pub use vector::impls::{Vector4f32 as Vector4, vec4f32 as vec4};
pub use vector::impls::{Vector4f32, vec4f32};
pub use vector::impls::{Vector4f64, vec4f64};

pub use vector::impls::{Vector4i8, vec4i8};
pub use vector::impls::{Vector4i16, vec4i16};
pub use vector::impls::{Vector4i32, vec4i32};
pub use vector::impls::{Vector4i64, vec4i64};
pub use vector::impls::{Vector4isize, vec4isize};

pub use vector::impls::{Vector4u8, vec4u8};
pub use vector::impls::{Vector4u16, vec4u16};
pub use vector::impls::{Vector4u32, vec4u32};
pub use vector::impls::{Vector4u64, vec4u64};
pub use vector::impls::{Vector4usize, vec4usize};

/// A module that defines all matrix types of the crate
mod matrix;

pub use matrix::matrix2::matrix2x2_f32::{Matrix2x2f32 as Matrix2x2, mat2x2f32 as mat2x2};
pub use matrix::matrix2::matrix2x2_f32::{Matrix2x2f32, mat2x2f32};

pub use matrix::matrix3::matrix3x3_f32::{Matrix3x3f32 as Matrix3x3, mat3x3f32 as mat3x3};
pub use matrix::matrix3::matrix3x3_f32::{Matrix3x3f32, mat3x3f32};

pub use matrix::matrix4::matrix4x4_f32::{Matrix4x4f32 as Matrix4x4, mat4x4f32 as mat4x4};
pub use matrix::matrix4::matrix4x4_f32::{Matrix4x4f32, mat4x4f32};
