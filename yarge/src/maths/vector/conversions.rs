#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

use crate::maths::*;

/// Macro to generate smaller vectors from 3D vectors
macro_rules! convert_3d {
    ($dst_type:ty, $first:ident, $second:ident) => {
        paste::paste! {
            #[doc = "Gets a sub 2D vector from a 3D vector"]
            pub fn [<$first$second>](&self) -> $dst_type {
                $dst_type::new(self.$first, self.$second)
            }
        }
    };
}

/// Macro to generate smaller vectors from 4D vectors
macro_rules! convert_4d {
    ($dst_type:ty, $first:ident, $second:ident, $third:ident) => {
        paste::paste! {
            #[doc = "Gets a sub 3D vector from a 4D vector"]
            pub fn [<$first$second$third>](&self) -> $dst_type {
                $dst_type::new(self.$first, self.$second, self.$third)
            }
        }
    };
    ($dst_type:ty, $first:ident, $second:ident) => {
        paste::paste! {
            #[doc = "Gets a sub 2D vector from a 4D vector"]
            pub fn [<$first$second>](&self) -> $dst_type {
                $dst_type::new(self.$first, self.$second)
            }
        }
    };
    (@homogeneous $dst_type:ty) => {
        paste::paste! {
            #[doc = "Gets a 3D vector from an homogeneous 4D vector"]
            pub fn from_homogeneous(&self) -> $dst_type {
                let homogeneous = self.w;
                $dst_type::new(
                    self.x / homogeneous,
                    self.y / homogeneous,
                    self.z / homogeneous,
                )
            }
        }
    };
}

/// Macro to generate vector conversion
macro_rules! conversions {
    ($vec:ty, $vec_type:ty, $size:literal) => {
        paste::paste! {
            impl $vec {
                conversions!(@convert $vec, $vec_type, $size);
            }
        }
    };

    (@convert $vec:ty, $vec_type:ty, 3) => {
        paste::paste! {
            convert_3d!([<Vector2$vec_type>], x, x);
            convert_3d!([<Vector2$vec_type>], x, y);
            convert_3d!([<Vector2$vec_type>], x, z);
            convert_3d!([<Vector2$vec_type>], y, x);
            convert_3d!([<Vector2$vec_type>], y, y);
            convert_3d!([<Vector2$vec_type>], y, z);
            convert_3d!([<Vector2$vec_type>], z, x);
            convert_3d!([<Vector2$vec_type>], z, y);
            convert_3d!([<Vector2$vec_type>], z, z);
        }
    };

    (@convert $vec:ty, $vec_type:ty, 4) => {
        paste::paste! {
            convert_4d!(@homogeneous [<Vector3$vec_type>]);

            convert_4d!([<Vector2$vec_type>], x, x);
            convert_4d!([<Vector2$vec_type>], x, y);
            convert_4d!([<Vector2$vec_type>], x, z);
            convert_4d!([<Vector2$vec_type>], x, w);
            convert_4d!([<Vector2$vec_type>], y, x);
            convert_4d!([<Vector2$vec_type>], y, y);
            convert_4d!([<Vector2$vec_type>], y, z);
            convert_4d!([<Vector2$vec_type>], y, w);
            convert_4d!([<Vector2$vec_type>], z, x);
            convert_4d!([<Vector2$vec_type>], z, y);
            convert_4d!([<Vector2$vec_type>], z, z);
            convert_4d!([<Vector2$vec_type>], z, w);
            convert_4d!([<Vector2$vec_type>], w, x);
            convert_4d!([<Vector2$vec_type>], w, y);
            convert_4d!([<Vector2$vec_type>], w, z);
            convert_4d!([<Vector2$vec_type>], w, w);

            convert_4d!([<Vector3$vec_type>], x, x, x);
            convert_4d!([<Vector3$vec_type>], x, x, y);
            convert_4d!([<Vector3$vec_type>], x, x, z);
            convert_4d!([<Vector3$vec_type>], x, x, w);
            convert_4d!([<Vector3$vec_type>], x, y, x);
            convert_4d!([<Vector3$vec_type>], x, y, y);
            convert_4d!([<Vector3$vec_type>], x, y, z);
            convert_4d!([<Vector3$vec_type>], x, y, w);
            convert_4d!([<Vector3$vec_type>], x, z, x);
            convert_4d!([<Vector3$vec_type>], x, z, y);
            convert_4d!([<Vector3$vec_type>], x, z, z);
            convert_4d!([<Vector3$vec_type>], x, z, w);
            convert_4d!([<Vector3$vec_type>], x, w, x);
            convert_4d!([<Vector3$vec_type>], x, w, y);
            convert_4d!([<Vector3$vec_type>], x, w, z);
            convert_4d!([<Vector3$vec_type>], x, w, w);
            
            convert_4d!([<Vector3$vec_type>], y, x, x);
            convert_4d!([<Vector3$vec_type>], y, x, y);
            convert_4d!([<Vector3$vec_type>], y, x, z);
            convert_4d!([<Vector3$vec_type>], y, x, w);
            convert_4d!([<Vector3$vec_type>], y, y, x);
            convert_4d!([<Vector3$vec_type>], y, y, y);
            convert_4d!([<Vector3$vec_type>], y, y, z);
            convert_4d!([<Vector3$vec_type>], y, y, w);
            convert_4d!([<Vector3$vec_type>], y, z, x);
            convert_4d!([<Vector3$vec_type>], y, z, y);
            convert_4d!([<Vector3$vec_type>], y, z, z);
            convert_4d!([<Vector3$vec_type>], y, z, w);
            convert_4d!([<Vector3$vec_type>], y, w, x);
            convert_4d!([<Vector3$vec_type>], y, w, y);
            convert_4d!([<Vector3$vec_type>], y, w, z);
            convert_4d!([<Vector3$vec_type>], y, w, w);

            convert_4d!([<Vector3$vec_type>], z, x, x);
            convert_4d!([<Vector3$vec_type>], z, x, y);
            convert_4d!([<Vector3$vec_type>], z, x, z);
            convert_4d!([<Vector3$vec_type>], z, x, w);
            convert_4d!([<Vector3$vec_type>], z, y, x);
            convert_4d!([<Vector3$vec_type>], z, y, y);
            convert_4d!([<Vector3$vec_type>], z, y, z);
            convert_4d!([<Vector3$vec_type>], z, y, w);
            convert_4d!([<Vector3$vec_type>], z, z, x);
            convert_4d!([<Vector3$vec_type>], z, z, y);
            convert_4d!([<Vector3$vec_type>], z, z, z);
            convert_4d!([<Vector3$vec_type>], z, z, w);
            convert_4d!([<Vector3$vec_type>], z, w, x);
            convert_4d!([<Vector3$vec_type>], z, w, y);
            convert_4d!([<Vector3$vec_type>], z, w, z);
            convert_4d!([<Vector3$vec_type>], z, w, w);

            convert_4d!([<Vector3$vec_type>], w, x, x);
            convert_4d!([<Vector3$vec_type>], w, x, y);
            convert_4d!([<Vector3$vec_type>], w, x, z);
            convert_4d!([<Vector3$vec_type>], w, x, w);
            convert_4d!([<Vector3$vec_type>], w, y, x);
            convert_4d!([<Vector3$vec_type>], w, y, y);
            convert_4d!([<Vector3$vec_type>], w, y, z);
            convert_4d!([<Vector3$vec_type>], w, y, w);
            convert_4d!([<Vector3$vec_type>], w, z, x);
            convert_4d!([<Vector3$vec_type>], w, z, y);
            convert_4d!([<Vector3$vec_type>], w, z, z);
            convert_4d!([<Vector3$vec_type>], w, z, w);
            convert_4d!([<Vector3$vec_type>], w, w, x);
            convert_4d!([<Vector3$vec_type>], w, w, y);
            convert_4d!([<Vector3$vec_type>], w, w, z);
            convert_4d!([<Vector3$vec_type>], w, w, w);
        }
    };
}



conversions!(Vector4f32, f32, 4);
conversions!(Vector4f64, f64, 4);
conversions!(Vector4i8, i8, 4);
conversions!(Vector4i16, i16, 4);
conversions!(Vector4i32, i32, 4);
conversions!(Vector4i64, i64, 4);
conversions!(Vector4isize, isize, 4);
conversions!(Vector4u8, u8, 4);
conversions!(Vector4u16, u16, 4);
conversions!(Vector4u32, u32, 4);
conversions!(Vector4u64, u64, 4);
conversions!(Vector4usize, usize, 4);

conversions!(Vector3f32, f32, 3);
conversions!(Vector3f64, f64, 3);
conversions!(Vector3i8, i8, 3);
conversions!(Vector3i16, i16, 3);
conversions!(Vector3i32, i32, 3);
conversions!(Vector3i64, i64, 3);
conversions!(Vector3isize, isize, 3);
conversions!(Vector3u8, u8, 3);
conversions!(Vector3u16, u16, 3);
conversions!(Vector3u32, u32, 3);
conversions!(Vector3u64, u64, 3);
conversions!(Vector3usize, usize, 3);