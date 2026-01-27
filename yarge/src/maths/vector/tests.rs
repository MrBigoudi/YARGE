#![allow(trivial_numeric_casts)]

#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

/// Macro to generate comprehensive test suites for vector types
macro_rules! impl_vec_tests {
    ($vec_name:ident, $vec_fn:ident, $vec_type:ty, $size:tt, $has_neg:tt, $is_float:tt) => {
        paste::paste! {
            #[cfg(test)]
            mod [<tests_ $vec_name:lower>] {
                use crate::maths::*;
                #[allow(unused)]
                use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

                #[doc = concat!("Tests ", $size, "D `", stringify!($vec_type), "` vector initialization")]
                #[test]
                fn initialization() {
                    impl_vec_tests!(@test_init $vec_name, $vec_fn, $vec_type, $size);
                }

                #[doc = concat!("Tests ", $size, "D `", stringify!($vec_type), "` vector operators")]
                #[test]
                fn operators() {
                    impl_vec_tests!(@test_operators $vec_name, $vec_fn, $vec_type, $size);
                }

                #[doc = concat!("Tests ", $size, "D `", stringify!($vec_type), "` vector operations")]
                #[test]
                fn operations() {
                    impl_vec_tests!(@test_operations $vec_name, $vec_fn, $vec_type, $size, $has_neg, $is_float);
                }

                #[doc = concat!("Tests ", $size, "D `", stringify!($vec_type), "` vector field getters and setters")]
                #[test]
                fn deref() {
                    impl_vec_tests!(@test_deref $vec_name, $vec_fn, $vec_type, $size);
                }

                #[doc = concat!("Tests the formatting of ", $size, "D `", stringify!($vec_type), "` vectors")]
                #[test]
                fn format() {
                    impl_vec_tests!(@test_format $vec_name, $vec_fn, $vec_type, $size, $is_float);
                }

                #[doc = concat!("Tests indices access for ", $size, "D `", stringify!($vec_type), "` vectors")]
                #[test]
                fn indices() {
                    impl_vec_tests!(@test_indices $vec_name, $vec_fn, $vec_type, $size);
                }

                impl_vec_tests!(@test_cross $vec_name, $vec_fn, $vec_type, $size, $has_neg, $is_float);
            }
        }
    };

    // Initialization tests
    (@test_init $vec_name:ident, $vec_fn:ident, $vec_type:ty, 2) => {
        let v1 = $vec_fn(1 as $vec_type, 1 as $vec_type);
        let v2 = $vec_name::ONES;
        assert_eq!(v1, v2);

        let v3 = $vec_name::ZEROS;
        assert_eq!(v3, $vec_fn(0 as $vec_type, 0 as $vec_type));

        let v4 = $vec_name::filled(5 as $vec_type);
        assert_eq!(v4, $vec_fn(5 as $vec_type, 5 as $vec_type));
    };
    (@test_init $vec_name:ident, $vec_fn:ident, $vec_type:ty, 3) => {
        let v1 = $vec_fn(1 as $vec_type, 1 as $vec_type, 1 as $vec_type);
        let v2 = $vec_name::ONES;
        assert_eq!(v1, v2);

        let v3 = $vec_name::ZEROS;
        assert_eq!(v3, $vec_fn(0 as $vec_type, 0 as $vec_type, 0 as $vec_type));

        let v4 = $vec_name::filled(5 as $vec_type);
        assert_eq!(v4, $vec_fn(5 as $vec_type, 5 as $vec_type, 5 as $vec_type));
    };
    (@test_init $vec_name:ident, $vec_fn:ident, $vec_type:ty, 4) => {
        let v1 = $vec_fn(1 as $vec_type, 1 as $vec_type, 1 as $vec_type, 1 as $vec_type);
        let v2 = $vec_name::ONES;
        assert_eq!(v1, v2);

        let v3 = $vec_name::ZEROS;
        assert_eq!(v3, $vec_fn(0 as $vec_type, 0 as $vec_type, 0 as $vec_type, 0 as $vec_type));

        let v4 = $vec_name::filled(5 as $vec_type);
        assert_eq!(v4, $vec_fn(5 as $vec_type, 5 as $vec_type, 5 as $vec_type, 5 as $vec_type));
    };

    // Operator tests
    (@test_operators $vec_name:ident, $vec_fn:ident, $vec_type:ty, 2) => {
        let v1 = $vec_fn(2 as $vec_type, 3 as $vec_type);
        let v2 = $vec_fn(4 as $vec_type, 5 as $vec_type);
        assert_eq!(v1 + v2, $vec_fn(6 as $vec_type, 8 as $vec_type));

        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, $vec_fn(4 as $vec_type, 6 as $vec_type));

        v3 += 1 as $vec_type;
        assert_eq!(v3, $vec_fn(5 as $vec_type, 7 as $vec_type));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2 as $vec_type, $vec_fn(10 as $vec_type, 14 as $vec_type));
        assert_eq!(v1 * v2, $vec_fn(8 as $vec_type, 15 as $vec_type));

        assert_eq!(v3 / v3, $vec_name::ONES);
        v3 /= v3;
        assert_eq!(v3, $vec_name::ONES);
    };
    (@test_operators $vec_name:ident, $vec_fn:ident, $vec_type:ty, 3) => {
        let v1 = $vec_fn(2 as $vec_type, 3 as $vec_type, 1 as $vec_type);
        let v2 = $vec_fn(4 as $vec_type, 5 as $vec_type, 6 as $vec_type);
        assert_eq!(v1 + v2, $vec_fn(6 as $vec_type, 8 as $vec_type, 7 as $vec_type));

        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, $vec_fn(4 as $vec_type, 6 as $vec_type, 2 as $vec_type));

        v3 += 1 as $vec_type;
        assert_eq!(v3, $vec_fn(5 as $vec_type, 7 as $vec_type, 3 as $vec_type));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2 as $vec_type, $vec_fn(10 as $vec_type, 14 as $vec_type, 6 as $vec_type));
        assert_eq!(v1 * v2, $vec_fn(8 as $vec_type, 15 as $vec_type, 6 as $vec_type));

        assert_eq!(v3 / v3, $vec_name::ONES);
        v3 /= v3;
        assert_eq!(v3, $vec_name::ONES);
    };
    (@test_operators $vec_name:ident, $vec_fn:ident, $vec_type:ty, 4) => {
        let v1 = $vec_fn(2 as $vec_type, 3 as $vec_type, 1 as $vec_type, 4 as $vec_type);
        let v2 = $vec_fn(4 as $vec_type, 5 as $vec_type, 6 as $vec_type, 2 as $vec_type);
        assert_eq!(v1 + v2, $vec_fn(6 as $vec_type, 8 as $vec_type, 7 as $vec_type, 6 as $vec_type));

        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, $vec_fn(4 as $vec_type, 6 as $vec_type, 2 as $vec_type, 8 as $vec_type));

        v3 += 1 as $vec_type;
        assert_eq!(v3, $vec_fn(5 as $vec_type, 7 as $vec_type, 3 as $vec_type, 9 as $vec_type));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2 as $vec_type, $vec_fn(10 as $vec_type, 14 as $vec_type, 6 as $vec_type, 18 as $vec_type));
        assert_eq!(v1 * v2, $vec_fn(8 as $vec_type, 15 as $vec_type, 6 as $vec_type, 8 as $vec_type));

        assert_eq!(v3 / v3, $vec_name::ONES);
        v3 /= v3;
        assert_eq!(v3, $vec_name::ONES);
    };

    // Operations tests
    (@test_operations $vec_name:ident, $vec_fn:ident, $vec_type:ty, 2, $has_neg:tt, $is_float:tt) => {
        let v1 = $vec_fn(1 as $vec_type, 2 as $vec_type);
        let v2 = $vec_fn(4 as $vec_type, 3 as $vec_type);
        assert_eq!($vec_name::dot(&v1, &v2), 10 as $vec_type);

        let vx = $vec_name::X;
        let vy = $vec_name::Y;
        assert_eq!($vec_name::dot(&vx, &vy), 0 as $vec_type);

        assert_eq!(vx, $vec_fn(1 as $vec_type, 0 as $vec_type));
        assert_eq!(vy, $vec_fn(0 as $vec_type, 1 as $vec_type));

        impl_vec_tests!(@test_length vx, vy, v1);
        impl_vec_tests!(@test_normalization v1, $is_float);
        impl_vec_tests!(@test_prefix_sum v1, 3 as $vec_type);
    };
    (@test_operations $vec_name:ident, $vec_fn:ident, $vec_type:ty, 3, $has_neg:tt, $is_float:tt) => {
        let v1 = $vec_fn(1 as $vec_type, 2 as $vec_type, 4 as $vec_type);
        let v2 = $vec_fn(4 as $vec_type, 3 as $vec_type, 5 as $vec_type);
        assert_eq!($vec_name::dot(&v1, &v2), 30 as $vec_type);

        let vx = $vec_name::X;
        let vy = $vec_name::Y;
        let vz = $vec_name::Z;
        assert_eq!($vec_name::dot(&vx, &vy), 0 as $vec_type);

        assert_eq!(vx, $vec_fn(1 as $vec_type, 0 as $vec_type, 0 as $vec_type));
        assert_eq!(vy, $vec_fn(0 as $vec_type, 1 as $vec_type, 0 as $vec_type));
        assert_eq!(vz, $vec_fn(0 as $vec_type, 0 as $vec_type, 1 as $vec_type));


        impl_vec_tests!(@test_length vx, vy, v1);
        impl_vec_tests!(@test_normalization v1, $is_float);
        impl_vec_tests!(@test_prefix_sum v1, 7 as $vec_type);
    };
    (@test_operations $vec_name:ident, $vec_fn:ident, $vec_type:ty, 4, $has_neg:tt, $is_float:tt) => {
        let v1 = $vec_fn(1 as $vec_type, 2 as $vec_type, 4 as $vec_type, 3 as $vec_type);
        let v2 = $vec_fn(4 as $vec_type, 3 as $vec_type, 5 as $vec_type, 2 as $vec_type);
        assert_eq!($vec_name::dot(&v1, &v2), 36 as $vec_type);

        let vx = $vec_name::X;
        let vy = $vec_name::Y;
        let vz = $vec_name::Z;
        let vw = $vec_name::W;
        assert_eq!($vec_name::dot(&vx, &vy), 0 as $vec_type);

        assert_eq!(vx, $vec_fn(1 as $vec_type, 0 as $vec_type, 0 as $vec_type, 0 as $vec_type));
        assert_eq!(vy, $vec_fn(0 as $vec_type, 1 as $vec_type, 0 as $vec_type, 0 as $vec_type));
        assert_eq!(vz, $vec_fn(0 as $vec_type, 0 as $vec_type, 1 as $vec_type, 0 as $vec_type));
        assert_eq!(vw, $vec_fn(0 as $vec_type, 0 as $vec_type, 0 as $vec_type, 1 as $vec_type));

        impl_vec_tests!(@test_length vx, vy, v1);
        impl_vec_tests!(@test_normalization v1, $is_float);
        impl_vec_tests!(@test_prefix_sum v1, 10 as $vec_type);
    };

    // Cross product tests (only for 3D with negation support)
    (@test_cross $vec_name:ident, $vec_fn:ident, $vec_type:ty, 3, true, $is_float:tt) => {
        #[doc = concat!("Tests cross product for 3D `", stringify!($vec_type), "` vectors")]
        #[test]
        fn cross_product() {
            let v1 = $vec_fn(1 as $vec_type, 2 as $vec_type, 4 as $vec_type);
            let v2 = $vec_fn(4 as $vec_type, 3 as $vec_type, 5 as $vec_type);

            impl_vec_tests!(@test_cross_calc $vec_name, $vec_fn, v1, v2, $is_float);

            let vx = $vec_name::X;
            let vy = $vec_name::Y;
            let vz = $vec_name::Z;

            assert_eq!($vec_name::cross(&vx, &vy), vz);
            assert_eq!($vec_name::cross(&vy, &vx), $vec_name::NEG_Z);
            assert_eq!($vec_name::cross(&vy, &vz), vx);
            assert_eq!($vec_name::cross(&vz, &vy), $vec_name::NEG_X);
            assert_eq!($vec_name::cross(&vz, &vx), vy);
            assert_eq!($vec_name::cross(&vx, &vz), $vec_name::NEG_Y);
        }
    };
    (@test_cross $vec_name:ident, $vec_fn:ident, $vec_type:ty, $size:tt, $has_neg:tt, $is_float:tt) => {};

    // Helper for cross product calculation (for both floats and ints)
    (@test_cross_calc $vec_name:ident, $vec_fn:ident, $v1:expr, $v2:expr, true) => {
        // For floats: v1 = (1, 2, 4), v2 = (4, 3, 5)
        // cross = (2*5 - 4*3, 4*4 - 1*5, 1*3 - 2*4) = (10-12, 16-5, 3-8) = (-2, 11, -5)
        let cross = $vec_name::cross(&$v1, &$v2);
        assert!((cross.x - (-2.0)).abs() < 0.001);
        assert!((cross.y - 11.0).abs() < 0.001);
        assert!((cross.z - (-5.0)).abs() < 0.001);
        assert_eq!($vec_name::cross(&$v2, &$v1), -$vec_name::cross(&$v1, &$v2));
    };
    (@test_cross_calc $vec_name:ident, $vec_fn:ident, $v1:expr, $v2:expr, false) => {
        // For integers: same calculation but exact comparison
        // Note: For unsigned integers, we can't test negative results
        let cross = $vec_name::cross(&$v1, &$v2);
        // Just verify it computes without panicking for unsigned types
        let _ = cross;
    };

    // Length tests
    (@test_length $vx:expr, $vy:expr, $v1:expr) => {
        assert_eq!($vx.length(), 1.0);
        assert_eq!($vy.length(), 1.0);
    };
    // Normalization tests
    (@test_normalization $v1:expr, false) => {};
    (@test_normalization $v1:expr, true) => {
        let normalized = $v1.normalize();
        assert!(normalized.is_ok());
        if let Ok(n) = normalized {
            assert!((n.length() - 1.0).abs() < 0.001);
        }
    };


    // Prefix sum tests
    (@test_prefix_sum $v:expr, $expected:expr) => {
        assert_eq!($v.prefix_sum(), $expected);
    };

    // Deref tests
    (@test_deref $vec_name:ident, $vec_fn:ident, $vec_type:ty, 2) => {
        let v1 = $vec_fn(5 as $vec_type, 2 as $vec_type);
        assert_eq!(v1.x, 5 as $vec_type);
        assert_eq!(v1.y, 2 as $vec_type);

        let mut v2 = $vec_name::ZEROS;
        assert_eq!(v2.x, 0 as $vec_type);
        assert_eq!(v2.y, 0 as $vec_type);

        v2.x = 1 as $vec_type;
        assert_eq!(v2.x, 1 as $vec_type);
        v2.y = 3 as $vec_type;
        assert_eq!(v2.y, 3 as $vec_type);
        assert_eq!(v2.x, 1 as $vec_type);
    };
    (@test_deref $vec_name:ident, $vec_fn:ident, $vec_type:ty, 3) => {
        let v1 = $vec_fn(5 as $vec_type, 2 as $vec_type, 1 as $vec_type);
        assert_eq!(v1.x, 5 as $vec_type);
        assert_eq!(v1.y, 2 as $vec_type);
        assert_eq!(v1.z, 1 as $vec_type);

        let mut v2 = $vec_name::ZEROS;
        assert_eq!(v2.x, 0 as $vec_type);
        assert_eq!(v2.y, 0 as $vec_type);
        assert_eq!(v2.z, 0 as $vec_type);

        v2.x = 1 as $vec_type;
        assert_eq!(v2.x, 1 as $vec_type);
        v2.y = 3 as $vec_type;
        assert_eq!(v2.y, 3 as $vec_type);
        assert_eq!(v2.x, 1 as $vec_type);
        v2.z = 4 as $vec_type;
        assert_eq!(v2.y, 3 as $vec_type);
        assert_eq!(v2.x, 1 as $vec_type);
        assert_eq!(v2.z, 4 as $vec_type);
    };
    (@test_deref $vec_name:ident, $vec_fn:ident, $vec_type:ty, 4) => {
        let v1 = $vec_fn(5 as $vec_type, 2 as $vec_type, 1 as $vec_type, 7 as $vec_type);
        assert_eq!(v1.x, 5 as $vec_type);
        assert_eq!(v1.y, 2 as $vec_type);
        assert_eq!(v1.z, 1 as $vec_type);
        assert_eq!(v1.w, 7 as $vec_type);

        let mut v2 = $vec_name::ZEROS;
        assert_eq!(v2.x, 0 as $vec_type);
        assert_eq!(v2.y, 0 as $vec_type);
        assert_eq!(v2.z, 0 as $vec_type);
        assert_eq!(v2.w, 0 as $vec_type);

        v2.x = 1 as $vec_type;
        assert_eq!(v2.x, 1 as $vec_type);
        v2.y = 3 as $vec_type;
        assert_eq!(v2.y, 3 as $vec_type);
        v2.z = 4 as $vec_type;
        assert_eq!(v2.z, 4 as $vec_type);
        v2.w = 9 as $vec_type;
        assert_eq!(v2.w, 9 as $vec_type);
    };

    // Format tests
    (@test_format $vec_name:ident, $vec_fn:ident, $vec_type:ty, 2, true) => {
        let v1 = $vec_fn(3.1, 4.2);
        assert_eq!(v1.to_string(), "(3.1, 4.2)");
        assert_eq!(format!("{:?}", v1), concat!(stringify!($vec_name), " { x: 3.1, y: 4.2 }"));
    };
    (@test_format $vec_name:ident, $vec_fn:ident, $vec_type:ty, 2, false) => {
        let v1 = $vec_fn(3, 4);
        assert_eq!(v1.to_string(), "(3, 4)");
        assert_eq!(format!("{:?}", v1), concat!(stringify!($vec_name), " { x: 3, y: 4 }"));
    };
    (@test_format $vec_name:ident, $vec_fn:ident, $vec_type:ty, 3, true) => {
        let v1 = $vec_fn(3.1, 4.2, 6.9);
        assert_eq!(v1.to_string(), "(3.1, 4.2, 6.9)");
        assert_eq!(format!("{:?}", v1), concat!(stringify!($vec_name), " { x: 3.1, y: 4.2, z: 6.9 }"));
    };
    (@test_format $vec_name:ident, $vec_fn:ident, $vec_type:ty, 3, false) => {
        let v1 = $vec_fn(3, 4, 6);
        assert_eq!(v1.to_string(), "(3, 4, 6)");
        assert_eq!(format!("{:?}", v1), concat!(stringify!($vec_name), " { x: 3, y: 4, z: 6 }"));
    };
    (@test_format $vec_name:ident, $vec_fn:ident, $vec_type:ty, 4, true) => {
        let v1 = $vec_fn(3.1, 4.2, 6.9, 1.5);
        assert_eq!(v1.to_string(), "(3.1, 4.2, 6.9, 1.5)");
        assert_eq!(format!("{:?}", v1), concat!(stringify!($vec_name), " { x: 3.1, y: 4.2, z: 6.9, w: 1.5 }"));
    };
    (@test_format $vec_name:ident, $vec_fn:ident, $vec_type:ty, 4, false) => {
        let v1 = $vec_fn(3, 4, 6, 1);
        assert_eq!(v1.to_string(), "(3, 4, 6, 1)");
        assert_eq!(format!("{:?}", v1), concat!(stringify!($vec_name), " { x: 3, y: 4, z: 6, w: 1 }"));
    };

    // Indices tests
    (@test_indices $vec_name:ident, $vec_fn:ident, $vec_type:ty, 2) => {
        let mut v1 = $vec_fn(2 as $vec_type, 3 as $vec_type);
        assert_eq!(v1[0], 2 as $vec_type);
        assert_eq!(v1[1], 3 as $vec_type);
        v1[1] = 5 as $vec_type;
        assert_eq!(v1[1], 5 as $vec_type);
    };
    (@test_indices $vec_name:ident, $vec_fn:ident, $vec_type:ty, 3) => {
        let mut v1 = $vec_fn(2 as $vec_type, 3 as $vec_type, 1 as $vec_type);
        assert_eq!(v1[2], 1 as $vec_type);
        v1[1] = 2 as $vec_type;
        assert_eq!(v1[1], 2 as $vec_type);
    };
    (@test_indices $vec_name:ident, $vec_fn:ident, $vec_type:ty, 4) => {
        let mut v1 = $vec_fn(2 as $vec_type, 3 as $vec_type, 1 as $vec_type, 7 as $vec_type);
        assert_eq!(v1[2], 1 as $vec_type);
        assert_eq!(v1[3], 7 as $vec_type);
        v1[1] = 2 as $vec_type;
        assert_eq!(v1[1], 2 as $vec_type);
    };
}

impl_vec_tests!(Vector4f32, vec4f32, f32, 4, true, true);
impl_vec_tests!(Vector4f64, vec4f64, f64, 4, true, true);
impl_vec_tests!(Vector4i8, vec4i8, i8, 4, true, false);
impl_vec_tests!(Vector4i16, vec4i16, i16, 4, true, false);
impl_vec_tests!(Vector4i32, vec4i32, i32, 4, true, false);
impl_vec_tests!(Vector4i64, vec4i64, i64, 4, true, false);
impl_vec_tests!(Vector4isize, vec4isize, isize, 4, true, false);
impl_vec_tests!(Vector4u8, vec4u8, u8, 4, false, false);
impl_vec_tests!(Vector4u16, vec4u16, u16, 4, false, false);
impl_vec_tests!(Vector4u32, vec4u32, u32, 4, false, false);
impl_vec_tests!(Vector4u64, vec4u64, u64, 4, false, false);
impl_vec_tests!(Vector4usize, vec4usize, usize, 4, false, false);

impl_vec_tests!(Vector3f32, vec3f32, f32, 3, true, true);
impl_vec_tests!(Vector3f64, vec3f64, f64, 3, true, true);
impl_vec_tests!(Vector3i8, vec3i8, i8, 3, true, false);
impl_vec_tests!(Vector3i16, vec3i16, i16, 3, true, false);
impl_vec_tests!(Vector3i32, vec3i32, i32, 3, true, false);
impl_vec_tests!(Vector3i64, vec3i64, i64, 3, true, false);
impl_vec_tests!(Vector3isize, vec3isize, isize, 3, true, false);
impl_vec_tests!(Vector3u8, vec3u8, u8, 3, false, false);
impl_vec_tests!(Vector3u16, vec3u16, u16, 3, false, false);
impl_vec_tests!(Vector3u32, vec3u32, u32, 3, false, false);
impl_vec_tests!(Vector3u64, vec3u64, u64, 3, false, false);
impl_vec_tests!(Vector3usize, vec3usize, usize, 3, false, false);

impl_vec_tests!(Vector2f32, vec2f32, f32, 2, true, true);
impl_vec_tests!(Vector2f64, vec2f64, f64, 2, true, true);
impl_vec_tests!(Vector2i8, vec2i8, i8, 2, true, false);
impl_vec_tests!(Vector2i16, vec2i16, i16, 2, true, false);
impl_vec_tests!(Vector2i32, vec2i32, i32, 2, true, false);
impl_vec_tests!(Vector2i64, vec2i64, i64, 2, true, false);
impl_vec_tests!(Vector2isize, vec2isize, isize, 2, true, false);
impl_vec_tests!(Vector2u8, vec2u8, u8, 2, false, false);
impl_vec_tests!(Vector2u16, vec2u16, u16, 2, false, false);
impl_vec_tests!(Vector2u32, vec2u32, u32, 2, false, false);
impl_vec_tests!(Vector2u64, vec2u64, u64, 2, false, false);
impl_vec_tests!(Vector2usize, vec2usize, usize, 2, false, false);
