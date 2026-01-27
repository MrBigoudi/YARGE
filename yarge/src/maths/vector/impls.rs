#![allow(trivial_numeric_casts)]

#[allow(unused)]
use crate::{error::ErrorType, log_debug, log_error, log_info, log_warn};

#[allow(unused)]
use std::simd::prelude::*;

/// Macro to generate vector structures with SIMD operations
macro_rules! impl_vec {
    ($vec_name:ident, $simd_type:ty, $vec_type:ty, $size:tt, $has_neg:tt, $is_float:tt) => {
        paste::paste! {
            #[doc = concat!("A structure to represent a ", $size, " dimensional `", stringify!($vec_type), "` vector")]
            #[derive(Clone, Copy)]
            #[repr(C)]
            pub struct $vec_name {
                data: $simd_type,
            }
        }

        // Generate the coordinate struct based on size
        paste::paste! {
            #[doc = concat!("Private module for ", stringify!($vec_name), " coordinate access")]
            mod [<private_ $vec_name:lower>] {
                impl_vec!(@coords $size, $vec_name, $vec_type);
            }

            #[doc = concat!("Implements `Deref` to allow accessing ", impl_vec!(@coord_doc $size))]
            impl std::ops::Deref for $vec_name {
                type Target = [<private_ $vec_name:lower>]::[<Coords $vec_name>];

                fn deref(&self) -> &Self::Target {
                    let value: *const $vec_name = self;
                    unsafe { &*(value as *const Self::Target) }
                }
            }

            #[doc = concat!("Implements `DerefMut` to allow modifying ", impl_vec!(@coord_doc $size))]
            impl std::ops::DerefMut for $vec_name {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    let value: *mut $vec_name = self;
                    unsafe { &mut *(value as *mut Self::Target) }
                }
            }
        }

        paste::paste! {
            #[doc = concat!("Overrides the debug trait for ", stringify!($vec_name))]
            impl std::fmt::Debug for $vec_name {
                impl_vec!(@debug_fields $vec_name, $size);
            }

            #[doc = concat!("Overrides the display trait for ", stringify!($vec_name))]
            impl std::fmt::Display for $vec_name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "(")?;
                    impl_vec!(@display_fields f, self, $size);
                    write!(f, ")")
                }
            }

            #[doc = concat!("Sets a ", $size, " dimensional `", stringify!($vec_type), "` vector to `[", impl_vec!(@example_doc_string_tuple $is_float, 0, $size), "]` as default")]
            impl Default for $vec_name {
                fn default() -> Self {
                    Self::ZEROS
                }
            }

            #[doc = concat!("Component-wise equality comparison for ", stringify!($vec_name))]
            impl PartialEq for $vec_name {
                fn eq(&self, other: &Self) -> bool {
                    impl_vec!(@eq_check self, other, $size)
                }
            }
        }

        // Union for const construction
        paste::paste! {
            #[doc = concat!("Union to cast SIMD to array for const construction of ", stringify!($vec_name))]
            union [<UnionCast $vec_name>] {
                array: [$vec_type; impl_vec!(@simd_size $size)],
                simd: $vec_name,
            }
        }

        impl_vec!(@fn_constructor $vec_name, $vec_type, $size);

        impl $vec_name {
            //////////////////////////////////////////////////////////
            /////////////      vector creation       /////////////////
            //////////////////////////////////////////////////////////
            paste::paste! {
                impl_vec!(@fn_new $vec_name, $vec_type, $is_float, $size);

                #[doc = concat!("Creates a new ", $size, "D `", stringify!($vec_type), "` vector with all coordinates set to `value`\n\n")]
                #[doc = "# Arguments\n\n"]
                #[doc = "* `value` - The value to set for all components\n\n"]
                const fn splat(value: $vec_type) -> Self {
                    impl_vec!(@splat_init value, $size)
                }

                #[doc = concat!("Creates a new ", $size, "D `", stringify!($vec_type), "` vector filled with `value`\n\n")]
                #[doc = "# Arguments\n\n"]
                #[doc = "* `value` - The value to fill all components with\n\n"]
                #[doc = "# Examples\n\n```\n"]
                #[doc = concat!("# use yarge::maths::", stringify!($vec_name), ";\n")]
                #[doc = concat!("let v = ", stringify!($vec_name), "::filled(3", impl_vec!(@type_suffix $is_float), ");\n```")]
                pub const fn filled(value: $vec_type) -> Self {
                    Self::splat(value)
                }

                #[doc = concat!("Creates a new ", $size, "D `", stringify!($vec_type), "` vector filled with zeros `(", impl_vec!(@example_doc_string_tuple $is_float, 0, $size), ")`")]
                pub const ZEROS: Self = Self::splat(0 as $vec_type);

                #[doc = concat!("Creates a new ", $size, "D `", stringify!($vec_type), "` vector filled with ones `", impl_vec!(@example_doc_string_tuple $is_float, 1, $size), "`")]
                pub const ONES: Self = Self::splat(1 as $vec_type);

                #[doc = concat!("Creates a new ", $size, "D `", stringify!($vec_type), "` vector filled with `", stringify!($vec_type), "::MIN`")]
                pub const MIN: Self = Self::splat(<$vec_type>::MIN);

                #[doc = concat!("Creates a new ", $size, "D `", stringify!($vec_type), "` vector filled with `", stringify!($vec_type), "::MAX`")]
                pub const MAX: Self = Self::splat(<$vec_type>::MAX);
            }

            impl_vec!(@conditional_consts $has_neg, $is_float, $vec_type, $size, $vec_name);
            impl_vec!(@axis_consts $size, $vec_type, $has_neg, $vec_name);

            //////////////////////////////////////////////////////////
            /////////////     vector operations      /////////////////
            //////////////////////////////////////////////////////////

            paste::paste! {
                #[doc = concat!("Sums up all elements of the ", $size, "D vector\n\n")]
                #[doc = concat!("Returns ", impl_vec!(@sum_doc $size), "\n\n")]
                #[doc = "# Examples\n\n```\n"]
                #[doc = concat!("# use yarge::maths::", stringify!($vec_name), ";\n")]
                #[doc = concat!("let v = ", stringify!($vec_name), "::", impl_vec!(@new_example $is_float, $size), ";\n")]
                #[doc = "let sum = v.prefix_sum();\n```"]
                pub fn prefix_sum(self) -> $vec_type {
                    impl_vec!(@prefix_sum self, $size)
                }

                #[doc = concat!("Computes the dot product between two ", $size, "D `", stringify!($vec_type), "` vectors\n\n")]
                #[doc = concat!("The dot product is defined as: ", impl_vec!(@dot_doc $size), "\n\n")]
                #[doc = "# Arguments\n\n"]
                #[doc = "* `v1` - First vector\n"]
                #[doc = "* `v2` - Second vector\n\n"]
                #[doc = "# Examples\n\n```\n"]
                #[doc = concat!("# use yarge::maths::", stringify!($vec_name), ";\n")]
                #[doc = concat!("let v1 = ", stringify!($vec_name), "::", impl_vec!(@new_example $is_float, $size), ";\n")]
                #[doc = concat!("let v2 = ", stringify!($vec_name), "::", impl_vec!(@new_example $is_float, $size), ";\n")]
                #[doc = concat!("let dot = ", stringify!($vec_name), "::dot(&v1, &v2);\n```")]
                pub fn dot(v1: &$vec_name, v2: &$vec_name) -> $vec_type {
                    (v1 * v2).prefix_sum()
                }
            }

            impl_vec!(@conditional_cross $size, $vec_name, $vec_type, $has_neg);

            paste::paste! {
                #[doc = concat!("Returns the length (magnitude) of the ", $size, "D vector\n\n")]
                #[doc = concat!("The length is computed as `sqrt(", impl_vec!(@length_doc $size), ")`\n\n")]
                #[doc = "# Examples\n\n```\n"]
                #[doc = concat!("# use yarge::maths::", stringify!($vec_name), ";\n")]
                #[doc = concat!("let v = ", stringify!($vec_name), "::", impl_vec!(@new_example $is_float, $size), ";\n")]
                #[doc = "let len = v.length();\n```"]
                pub fn length(&self) -> f32 {
                    (Self::dot(self, self) as f32).sqrt()
                }

                impl_vec!(@fn_normalize $vec_name, $vec_type, $size, $is_float);
            }

            impl_vec!(@const_accessors $size, $vec_type);
        }

        // Implement all operators
        impl_vec!(@impl_ops $vec_name, $simd_type, $vec_type, $size, $has_neg);
        impl_vec!(@impl_index $vec_name, $vec_type, $size);
    };

    (@fn_normalize $vec_name:ident, $vec_type:ty, $size:tt, true) => {
        paste::paste! {
            #[doc = "Returns the normalized vector (unit vector in the same direction)\n\n"]
            #[doc = "Returns an error if the vector has zero length.\n\n"]
            #[doc = "# Errors\n\n"]
            #[doc = "Returns `ErrorType::DivisionByZero` if the vector length is zero\n\n"]
            #[doc = "# Examples\n\n```\n"]
            #[doc = concat!("# use yarge::maths::", stringify!($vec_name), ";\n")]
            #[doc = concat!("let v = ", stringify!($vec_name), "::", impl_vec!(@new_example true, $size), ";\n")]
            #[doc = "let normalized = v.normalize()?;\n"]
            #[doc = "# Ok::<(), yarge::error::ErrorType>(())\n```"]
            pub fn normalize(&self) -> Result<Self, ErrorType> {
                let length = self.length();
                if length == 0f32 {
                    crate::log_error!("Can't normalize a 0 length vector");
                    return Err(ErrorType::DivisionByZero);
                }
                Ok(self / (length as $vec_type))
            }
        }
    };
    (@fn_normalize $vec_name:ident, $vec_type:ty, $size:tt, false) => {};

    (@fn_constructor $vec_name:ident, $vec_type:ty, 2) => {
        paste::paste! {
            #[doc = concat!("Creates a new 2D `", stringify!($vec_type), "` vector given its coordinates\n\n")]
            pub const fn [<vec2 $vec_type:lower>](x: $vec_type, y: $vec_type) -> $vec_name {
                $vec_name::new(x, y)
            }
        }
    };
    (@fn_constructor $vec_name:ident, $vec_type:ty, 3) => {
        paste::paste! {
            #[doc = concat!("Creates a new 3D `", stringify!($vec_type), "` vector given its coordinates\n\n")]
            pub const fn [<vec3 $vec_type:lower>](x: $vec_type, y: $vec_type, z: $vec_type) -> $vec_name {
                $vec_name::new(x, y, z)
            }
        }
    };
    (@fn_constructor $vec_name:ident, $vec_type:ty, 4) => {
        paste::paste! {
            #[doc = concat!("Creates a new 4D `", stringify!($vec_type), "` vector given its coordinates\n\n")]
            pub const fn [<vec4 $vec_type:lower>](x: $vec_type, y: $vec_type, z: $vec_type, w: $vec_type) -> $vec_name {
                $vec_name::new(x, y, z, w)
            }
        }
    };

    (@fn_new $vec_name:ident, $vec_type:ty, $is_float:tt, 2) => {
        paste::paste! {
            #[doc = concat!("Creates a new 2D `", stringify!($vec_type), "` vector given its coordinates\n\n")]
            #[doc = "# Arguments\n\n"]
            #[doc = impl_vec!(@new_doc_args 2)]
            #[doc = "\n# Examples\n\n```\n"]
            #[doc = concat!("# use yarge::maths::", stringify!($vec_name), ";\n")]
            #[doc = concat!("let v = ", stringify!($vec_name), "::", impl_vec!(@new_example $is_float, 2), ";\n```")]
            pub const fn new(x: $vec_type, y: $vec_type) -> $vec_name {
                unsafe {
                    [<UnionCast $vec_name>] {
                        array: [x, y],
                    }
                    .simd
                }
            }
        }
    };
    (@fn_new $vec_name:ident, $vec_type:ty, $is_float:tt, 3) => {
        paste::paste! {
            #[doc = concat!("Creates a new 3D `", stringify!($vec_type), "` vector given its coordinates\n\n")]
            #[doc = "# Arguments\n\n"]
            #[doc = impl_vec!(@new_doc_args 3)]
            #[doc = "\n# Examples\n\n```\n"]
            #[doc = concat!("# use yarge::maths::", stringify!($vec_name), ";\n")]
            #[doc = concat!("let v = ", stringify!($vec_name), "::", impl_vec!(@new_example $is_float, 3), ";\n```")]
            pub const fn new(x: $vec_type, y: $vec_type, z: $vec_type) -> $vec_name {
                unsafe {
                    [<UnionCast $vec_name>] {
                        array: [x, y, z, 1 as $vec_type],
                    }
                    .simd
                }
            }
        }
    };
    (@fn_new $vec_name:ident, $vec_type:ty, $is_float:tt, 4) => {
        paste::paste! {
            #[doc = concat!("Creates a new 4D `", stringify!($vec_type), "` vector given its coordinates\n\n")]
            #[doc = "# Arguments\n\n"]
            #[doc = impl_vec!(@new_doc_args 4)]
            #[doc = "\n# Examples\n\n```\n"]
            #[doc = concat!("# use yarge::maths::", stringify!($vec_name), ";\n")]
            #[doc = concat!("let v = ", stringify!($vec_name), "::", impl_vec!(@new_example $is_float, 4), ";\n```")]
            pub const fn new(x: $vec_type, y: $vec_type, z: $vec_type, w: $vec_type) -> $vec_name {
                unsafe {
                    [<UnionCast $vec_name>] {
                        array: [x, y, z, w],
                    }
                    .simd
                }
            }
        }
    };

    // Helper rules for coordinate struct generation
    (@coords 2, $vec_name:ident, $vec_type:ty) => {
        paste::paste! {
            #[doc = "Coordinate structure for " $vec_name]
            #[repr(C)]
            pub struct [<Coords $vec_name>] {
                #[doc = "X coordinate"]
                pub x: $vec_type,
                #[doc = "Y coordinate"]
                pub y: $vec_type,
            }
        }
    };
    (@coords 3, $vec_name:ident, $vec_type:ty) => {
        paste::paste! {
            #[doc = "Coordinate structure for " $vec_name]
            #[repr(C)]
            pub struct [<Coords $vec_name>] {
                #[doc = "X coordinate"]
                pub x: $vec_type,
                #[doc = "Y coordinate"]
                pub y: $vec_type,
                #[doc = "Z coordinate"]
                pub z: $vec_type,
                #[doc = "Padding for SIMD alignment (not user-accessible)"]
                _pad: $vec_type,
            }
        }
    };
    (@coords 4, $vec_name:ident, $vec_type:ty) => {
        paste::paste! {
            #[doc = "Coordinate structure for " $vec_name]
            #[repr(C)]
            pub struct [<Coords $vec_name>] {
                #[doc = "X coordinate"]
                pub x: $vec_type,
                #[doc = "Y coordinate"]
                pub y: $vec_type,
                #[doc = "Z coordinate"]
                pub z: $vec_type,
                #[doc = "W coordinate"]
                pub w: $vec_type,
            }
        }
    };

    // Documentation helpers
    (@coord_doc 2) => { "`.x`, and `.y`" };
    (@coord_doc 3) => { "`.x`, `.y`, and `.z`" };
    (@coord_doc 4) => { "`.x`, `.y`, `.z`, and `.w`" };

    (@example_doc_string $is_float:tt, $value:expr) => {
        concat!(stringify!($value), impl_vec!(@type_suffix $is_float))
    };
    (@example_doc_string_tuple $is_float:tt, $value:expr, 2) => {
        concat!(impl_vec!(@example_doc_string $is_float, $value), ", "
        , impl_vec!(@example_doc_string $is_float, $value))
    };
    (@example_doc_string_tuple $is_float:tt, $value:expr, 3) => {
        concat!(impl_vec!(@example_doc_string $is_float, $value), ", "
        , impl_vec!(@example_doc_string $is_float, $value), ", "
        , impl_vec!(@example_doc_string $is_float, $value))
    };
    (@example_doc_string_tuple $is_float:tt, $value:expr, 4) => {
        concat!(impl_vec!(@example_doc_string $is_float, $value), ", "
        , impl_vec!(@example_doc_string $is_float, $value), ", "
        , impl_vec!(@example_doc_string $is_float, $value), ", "
        , impl_vec!(@example_doc_string $is_float, $value))
    };


    (@new_doc_args 2) => { "* `x` - X coordinate\n* `y` - Y coordinate" };
    (@new_doc_args 3) => { "* `x` - X coordinate\n* `y` - Y coordinate\n* `z` - Z coordinate" };
    (@new_doc_args 4) => { "* `x` - X coordinate\n* `y` - Y coordinate\n* `z` - Z coordinate\n* `w` - W coordinate" };
    (@new_example $is_float:tt, 2) => {
        concat!("new("
        , impl_vec!(@example_doc_string $is_float, 1), ", "
        , impl_vec!(@example_doc_string $is_float, 2)
        , ")")
    };
    (@new_example $is_float:tt, 3) => {
        concat!("new("
        , impl_vec!(@example_doc_string $is_float, 1), ", "
        , impl_vec!(@example_doc_string $is_float, 2), ", "
        , impl_vec!(@example_doc_string $is_float, 3)
        , ")")
    };
    (@new_example $is_float:tt, 4) => {
        concat!("new("
        , impl_vec!(@example_doc_string $is_float, 1), ", "
        , impl_vec!(@example_doc_string $is_float, 2), ", "
        , impl_vec!(@example_doc_string $is_float, 3), ", "
        , impl_vec!(@example_doc_string $is_float, 4)
        , ")")
    };

    (@type_suffix true) => { ".0" };
    (@type_suffix false) => { "" };

    (@sum_doc 2) => { "`x + y`" };
    (@sum_doc 3) => { "`x + y + z`" };
    (@sum_doc 4) => { "`x + y + z + w`" };

    (@dot_doc 2) => { "`v1.x * v2.x + v1.y * v2.y`" };
    (@dot_doc 3) => { "`v1.x * v2.x + v1.y * v2.y + v1.z * v2.z`" };
    (@dot_doc 4) => { "`v1.x * v2.x + v1.y * v2.y + v1.z * v2.z + v1.w * v2.w`" };

    (@length_doc 2) => { "x*x + y*y" };
    (@length_doc 3) => { "x*x + y*y + z*z" };
    (@length_doc 4) => { "x*x + y*y + z*z + w*w" };

    // SIMD size calculation
    (@simd_size 2) => { 2 };
    (@simd_size 3) => { 4 };
    (@simd_size 4) => { 4 };

    // Debug field generation
    (@debug_fields $vec_name:ident, 2) => {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct(stringify!($vec_name))
                .field("x", &self.x).field("y", &self.y)
                .finish()
        }
    };
    (@debug_fields $vec_name:ident, 3) => {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct(stringify!($vec_name))
                .field("x", &self.x).field("y", &self.y).field("z", &self.z)
                .finish()
        }
    };
    (@debug_fields $vec_name:ident, 4) => {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct(stringify!($vec_name))
                .field("x", &self.x).field("y", &self.y).field("z", &self.z).field("w", &self.w)
                .finish()
        }
    };

    // Display field generation
    (@display_fields $f:expr, $self:expr, 2) => {
        write!($f, "{}, {}", $self.x, $self.y)?
    };
    (@display_fields $f:expr, $self:expr, 3) => {
        write!($f, "{}, {}, {}", $self.x, $self.y, $self.z)?
    };
    (@display_fields $f:expr, $self:expr, 4) => {
        write!($f, "{}, {}, {}, {}", $self.x, $self.y, $self.z, $self.w)?
    };

    // Equality check
    (@eq_check $self:expr, $other:expr, 2) => {
        $self.x == $other.x && $self.y == $other.y
    };
    (@eq_check $self:expr, $other:expr, 3) => {
        $self.x == $other.x && $self.y == $other.y && $self.z == $other.z
    };
    (@eq_check $self:expr, $other:expr, 4) => {
        $self.x == $other.x && $self.y == $other.y && $self.z == $other.z && $self.w == $other.w
    };

    // Splat initialization
    (@splat_init $value:expr, 2) => { Self::new($value, $value) };
    (@splat_init $value:expr, 3) => { Self::new($value, $value, $value) };
    (@splat_init $value:expr, 4) => { Self::new($value, $value, $value, $value) };

    // Conditional constants (NEG and INF)
    (@conditional_consts false, false, $vec_type:ty, $size:tt, $vec_name:ident) => {};
    (@conditional_consts true, true, $vec_type:ty, $size:tt, $vec_name:ident) => {
        paste::paste! {
            impl_vec!(@conditional_consts true, false, $vec_type, $size, $vec_name);
            impl_vec!(@conditional_consts false, true, $vec_type, $size, $vec_name);
            #[doc = concat!("Creates a new ", $size, "D `", stringify!($vec_type), "` vector filled with `", stringify!($vec_type), "::NEG_INFINITY`")]
            pub const NEG_INFINITY: Self = Self::splat(<$vec_type>::NEG_INFINITY);
        }
    };
    (@conditional_consts true, false, $vec_type:ty, $size:tt, $vec_name:ident) => {
        paste::paste! {
            #[doc = concat!("Creates a new ", $size, "D `", stringify!($vec_type), "` vector filled with negative ones")]
            pub const NEG_ONES: Self = Self::splat(-1 as $vec_type);
        }
    };
    (@conditional_consts false, true, $vec_type:ty, $size:tt, $vec_name:ident) => {
        paste::paste! {
            #[doc = concat!("Creates a new ", $size, "D `", stringify!($vec_type), "` vector filled with `", stringify!($vec_type), "::INFINITY`")]
            pub const INFINITY: Self = Self::splat(<$vec_type>::INFINITY);
        }
    };

    // Axis constants
    (@axis_consts 2, $vec_type:ty, $has_neg:tt, $vec_name:ident) => {
        paste::paste! {
            #[doc = concat!("Creates a new 2D `", stringify!($vec_type), "` vector pointing along the positive X axis `[1, 0]`")]
            pub const X: Self = Self::new(1 as $vec_type, 0 as $vec_type);
            #[doc = concat!("Creates a new 2D `", stringify!($vec_type), "` vector pointing along the positive Y axis `[0, 1]`")]
            pub const Y: Self = Self::new(0 as $vec_type, 1 as $vec_type);
        }
        impl_vec!(@neg_axis_consts_2d $has_neg, $vec_type, $vec_name);
    };
    (@axis_consts 3, $vec_type:ty, $has_neg:tt, $vec_name:ident) => {
        paste::paste! {
            #[doc = concat!("Creates a new 3D `", stringify!($vec_type), "` vector pointing along the positive X axis `[1, 0, 0]`")]
            pub const X: Self = Self::new(1 as $vec_type, 0 as $vec_type, 0 as $vec_type);
            #[doc = concat!("Creates a new 3D `", stringify!($vec_type), "` vector pointing along the positive Y axis `[0, 1, 0]`")]
            pub const Y: Self = Self::new(0 as $vec_type, 1 as $vec_type, 0 as $vec_type);
            #[doc = concat!("Creates a new 3D `", stringify!($vec_type), "` vector pointing along the positive Z axis `[0, 0, 1]`")]
            pub const Z: Self = Self::new(0 as $vec_type, 0 as $vec_type, 1 as $vec_type);
        }
        impl_vec!(@neg_axis_consts_3d $has_neg, $vec_type, $vec_name);
    };
    (@axis_consts 4, $vec_type:ty, $has_neg:tt, $vec_name:ident) => {
        paste::paste! {
            #[doc = concat!("Creates a new 4D `", stringify!($vec_type), "` vector pointing along the positive X axis `[1, 0, 0, 0]`")]
            pub const X: Self = Self::new(1 as $vec_type, 0 as $vec_type, 0 as $vec_type, 0 as $vec_type);
            #[doc = concat!("Creates a new 4D `", stringify!($vec_type), "` vector pointing along the positive Y axis `[0, 1, 0, 0]`")]
            pub const Y: Self = Self::new(0 as $vec_type, 1 as $vec_type, 0 as $vec_type, 0 as $vec_type);
            #[doc = concat!("Creates a new 4D `", stringify!($vec_type), "` vector pointing along the positive Z axis `[0, 0, 1, 0]`")]
            pub const Z: Self = Self::new(0 as $vec_type, 0 as $vec_type, 1 as $vec_type, 0 as $vec_type);
            #[doc = concat!("Creates a new 4D `", stringify!($vec_type), "` vector pointing along the positive W axis `[0, 0, 0, 1]`")]
            pub const W: Self = Self::new(0 as $vec_type, 0 as $vec_type, 0 as $vec_type, 1 as $vec_type);
        }
        impl_vec!(@neg_axis_consts_4d $has_neg, $vec_type, $vec_name);
    };

    (@neg_axis_consts_2d true, $vec_type:ty, $vec_name:ident) => {
        paste::paste! {
            #[doc = concat!("Creates a new 2D `", stringify!($vec_type), "` vector pointing along the negative X axis `[-1, 0]`")]
            pub const NEG_X: Self = Self::new(-1 as $vec_type, 0 as $vec_type);
            #[doc = concat!("Creates a new 2D `", stringify!($vec_type), "` vector pointing along the negative Y axis `[0, -1]`")]
            pub const NEG_Y: Self = Self::new(0 as $vec_type, -1 as $vec_type);
        }
    };
    (@neg_axis_consts_2d false, $vec_type:ty, $vec_name:ident) => {};

    (@neg_axis_consts_3d true, $vec_type:ty, $vec_name:ident) => {
        paste::paste! {
            #[doc = concat!("Creates a new 3D `", stringify!($vec_type), "` vector pointing along the negative X axis `[-1, 0, 0]`")]
            pub const NEG_X: Self = Self::new(-1 as $vec_type, 0 as $vec_type, 0 as $vec_type);
            #[doc = concat!("Creates a new 3D `", stringify!($vec_type), "` vector pointing along the negative Y axis `[0, -1, 0]`")]
            pub const NEG_Y: Self = Self::new(0 as $vec_type, -1 as $vec_type, 0 as $vec_type);
            #[doc = concat!("Creates a new 3D `", stringify!($vec_type), "` vector pointing along the negative Z axis `[0, 0, -1]`")]
            pub const NEG_Z: Self = Self::new(0 as $vec_type, 0 as $vec_type, -1 as $vec_type);
        }
    };
    (@neg_axis_consts_3d false, $vec_type:ty, $vec_name:ident) => {};

    (@neg_axis_consts_4d true, $vec_type:ty, $vec_name:ident) => {
        paste::paste! {
            #[doc = concat!("Creates a new 4D `", stringify!($vec_type), "` vector pointing along the negative X axis `[-1, 0, 0, 0]`")]
            pub const NEG_X: Self = Self::new(-1 as $vec_type, 0 as $vec_type, 0 as $vec_type, 0 as $vec_type);
            #[doc = concat!("Creates a new 4D `", stringify!($vec_type), "` vector pointing along the negative Y axis `[0, -1, 0, 0]`")]
            pub const NEG_Y: Self = Self::new(0 as $vec_type, -1 as $vec_type, 0 as $vec_type, 0 as $vec_type);
            #[doc = concat!("Creates a new 4D `", stringify!($vec_type), "` vector pointing along the negative Z axis `[0, 0, -1, 0]`")]
            pub const NEG_Z: Self = Self::new(0 as $vec_type, 0 as $vec_type, -1 as $vec_type, 0 as $vec_type);
            #[doc = concat!("Creates a new 4D `", stringify!($vec_type), "` vector pointing along the negative W axis `[0, 0, 0, -1]`")]
            pub const NEG_W: Self = Self::new(0 as $vec_type, 0 as $vec_type, 0 as $vec_type, -1 as $vec_type);
        }
    };
    (@neg_axis_consts_4d false, $vec_type:ty, $vec_name:ident) => {};

    // Prefix sum
    (@prefix_sum $self:expr, 2) => {
        $self.x + $self.y
    };
    (@prefix_sum $self:expr, 3) => {
        $self.x + $self.y + $self.z
    };
    (@prefix_sum $self:expr, 4) => {
        $self.x + $self.y + $self.z + $self.w
    };

    // Cross product (only for 3D)
    (@conditional_cross 3, $vec_name:ident, $vec_type:ty, true) => {
        paste::paste! {
            #[doc = concat!("Computes the cross product between two 3D `", stringify!($vec_type), "` vectors\n\n")]
            #[doc = "The cross product of two vectors produces a third vector perpendicular to both.\n"]
            #[doc = "It is defined as:\n"]
            #[doc = "```text\n"]
            #[doc = "cross(v1, v2) = [\n"]
            #[doc = "    v1.y * v2.z - v1.z * v2.y,\n"]
            #[doc = "    v1.z * v2.x - v1.x * v2.z,\n"]
            #[doc = "    v1.x * v2.y - v1.y * v2.x\n"]
            #[doc = "]\n```\n\n"]
            #[doc = "# Arguments\n\n"]
            #[doc = "* `v1` - First vector\n"]
            #[doc = "* `v2` - Second vector\n\n"]
            #[doc = "# Examples\n\n```\n"]
            #[doc = concat!("# use yarge::maths::", stringify!($vec_name), ";\n")]
            #[doc = concat!("let v1 = ", stringify!($vec_name), "::X;\n")]
            #[doc = concat!("let v2 = ", stringify!($vec_name), "::Y;\n")]
            #[doc = concat!("let cross = ", stringify!($vec_name), "::cross(&v1, &v2);\n")]
            #[doc = concat!("assert_eq!(cross, ", stringify!($vec_name), "::Z);\n```")]
            pub fn cross(v1: &$vec_name, v2: &$vec_name) -> $vec_name {
                Self::new(
                    v1.y * v2.z - v1.z * v2.y,
                    v1.z * v2.x - v1.x * v2.z,
                    v1.x * v2.y - v1.y * v2.x,
                )
            }
        }
    };
    (@conditional_cross $size:tt, $vec_name:ident, $vec_type:ty, $has_neg:tt) => {};

    // Const accessors
    (@const_accessors 2, $vec_type:ty) => {
        paste::paste! {
            #[doc = "Const accessor for X coordinate\n"]
            #[doc = "Only used for matrix initialization"]
            pub(in crate::maths) const fn x_const(&self) -> $vec_type {
                self.data.as_array()[0]
            }
            #[doc = "Const accessor for Y coordinate\n"]
            #[doc = "Only used for matrix initialization"]
            pub(in crate::maths) const fn y_const(&self) -> $vec_type {
                self.data.as_array()[1]
            }
        }
    };
    (@const_accessors 3, $vec_type:ty) => {
        paste::paste! {
            impl_vec!(@const_accessors 2, $vec_type);
            #[doc = "Const accessor for Z coordinate\n"]
            #[doc = "Only used for matrix initialization"]
            pub(in crate::maths) const fn z_const(&self) -> $vec_type {
                self.data.as_array()[2]
            }
        }
    };
    (@const_accessors 4, $vec_type:ty) => {
        paste::paste! {
            impl_vec!(@const_accessors 3, $vec_type);
            #[doc = "Const accessor for W coordinate\n"]
            #[doc = "Only used for matrix initialization"]
            pub(in crate::maths) const fn w_const(&self) -> $vec_type {
                self.data.as_array()[3]
            }
        }
    };


    // Indexing
    (@impl_index $vec_name:ident, $vec_type:ty, 2) => {
        paste::paste! {
            #[doc = "Allows indexing into the 2D vector with `[0]` for X and `[1]` for Y\n\n"]
            #[doc = "# Panics\n\n"]
            #[doc = "Panics if index is greater than 1"]
            impl std::ops::Index<usize> for $vec_name {
                type Output = $vec_type;
                fn index(&self, index: usize) -> &Self::Output {
                    match index {
                        0 => &self.x,
                        1 => &self.y,
                        _ => panic!("Index out of bounds"),
                    }
                }
            }

            #[doc = "Allows mutable indexing into the 2D vector with `[0]` for X and `[1]` for Y\n\n"]
            #[doc = "# Panics\n\n"]
            #[doc = "Panics if index is greater than 1"]
            impl std::ops::IndexMut<usize> for $vec_name {
                fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                    match index {
                        0 => &mut self.x,
                        1 => &mut self.y,
                        _ => panic!("Index out of bounds"),
                    }
                }
            }
        }
    };
    (@impl_index $vec_name:ident, $vec_type:ty, 3) => {
        paste::paste! {
            #[doc = "Allows indexing into the 3D vector with `[0]` for X, `[1]` for Y, and `[2]` for Z\n\n"]
            #[doc = "# Panics\n\n"]
            #[doc = "Panics if index is greater than 2"]
            impl std::ops::Index<usize> for $vec_name {
                type Output = $vec_type;
                fn index(&self, index: usize) -> &Self::Output {
                    match index {
                        0 => &self.x,
                        1 => &self.y,
                        2 => &self.z,
                        _ => panic!("Index out of bounds"),
                    }
                }
            }

            #[doc = "Allows mutable indexing into the 3D vector with `[0]` for X, `[1]` for Y, and `[2]` for Z\n\n"]
            #[doc = "# Panics\n\n"]
            #[doc = "Panics if index is greater than 2"]
            impl std::ops::IndexMut<usize> for $vec_name {
                fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                    match index {
                        0 => &mut self.x,
                        1 => &mut self.y,
                        2 => &mut self.z,
                        _ => panic!("Index out of bounds"),
                    }
                }
            }
        }
    };
    (@impl_index $vec_name:ident, $vec_type:ty, 4) => {
        paste::paste! {
            #[doc = "Allows indexing into the 4D vector with `[0]` for X, `[1]` for Y, `[2]` for Z, and `[3]` for W\n\n"]
            #[doc = "# Panics\n\n"]
            #[doc = "Panics if index is greater than 3"]
            impl std::ops::Index<usize> for $vec_name {
                type Output = $vec_type;
                fn index(&self, index: usize) -> &Self::Output {
                    match index {
                        0 => &self.x,
                        1 => &self.y,
                        2 => &self.z,
                        3 => &self.w,
                        _ => panic!("Index out of bounds"),
                    }
                }
            }

            #[doc = "Allows mutable indexing into the 4D vector with `[0]` for X, `[1]` for Y, `[2]` for Z, and `[3]` for W\n\n"]
            #[doc = "# Panics\n\n"]
            #[doc = "Panics if index is greater than 3"]
            impl std::ops::IndexMut<usize> for $vec_name {
                fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                    match index {
                        0 => &mut self.x,
                        1 => &mut self.y,
                        2 => &mut self.z,
                        3 => &mut self.w,
                        _ => panic!("Index out of bounds"),
                    }
                }
            }
        }
    };

    // Operator implementations
    (@impl_ops $vec_name:ident, $simd_type:ty, $vec_type:ty, $size:tt, $has_neg:tt) => {
        impl_vec!(@impl_add $vec_name, $simd_type, $vec_type, $size);
        impl_vec!(@impl_sub $vec_name, $simd_type, $vec_type, $size, $has_neg);
        impl_vec!(@impl_mul $vec_name, $simd_type, $vec_type, $size);
        impl_vec!(@impl_div $vec_name, $simd_type, $vec_type, $size);
        impl_vec!(@impl_assign $vec_name, $simd_type, $vec_type, $size);
    };

    // Addition operators
    (@impl_add $vec_name:ident, $simd_type:ty, $vec_type:ty, $size:tt) => {
        paste::paste! {
            #[doc = concat!("Component-wise addition of two ", $size, "D vectors\n\n")]
            #[doc = concat!("`v1 + v2 = ", impl_vec!(@op_doc $size, add), "`")]
            impl std::ops::Add<$vec_name> for $vec_name {
                type Output = $vec_name;
                fn add(self, rhs: $vec_name) -> Self::Output {
                    $vec_name { data: self.data + rhs.data }
                }
            }
            #[doc = concat!("Component-wise addition of two ", $size, "D vectors (with reference)\n\n")]
            #[doc = concat!("`v1 + v2 = ", impl_vec!(@op_doc $size, add), "`")]
            impl std::ops::Add<&$vec_name> for $vec_name {
                type Output = $vec_name;
                fn add(self, rhs: &$vec_name) -> Self::Output { self.add(*rhs) }
            }
            #[doc = concat!("Component-wise addition of two ", $size, "D vectors (with reference)\n\n")]
            #[doc = concat!("`v1 + v2 = ", impl_vec!(@op_doc $size, add), "`")]
            impl std::ops::Add<$vec_name> for &$vec_name {
                type Output = $vec_name;
                fn add(self, rhs: $vec_name) -> Self::Output { (*self).add(rhs) }
            }
            #[doc = concat!("Component-wise addition of two ", $size, "D vectors (with references)\n\n")]
            #[doc = concat!("`v1 + v2 = ", impl_vec!(@op_doc $size, add), "`")]
            impl std::ops::Add<&$vec_name> for &$vec_name {
                type Output = $vec_name;
                fn add(self, rhs: &$vec_name) -> Self::Output { (*self).add(*rhs) }
            }
            #[doc = concat!("Adds a scalar to all components of the ", $size, "D vector\n\n")]
            #[doc = concat!("`v + s = ", impl_vec!(@scalar_op_doc $size, add), "`")]
            impl std::ops::Add<$vec_type> for $vec_name {
                type Output = $vec_name;
                fn add(self, rhs: $vec_type) -> Self::Output {
                    $vec_name { data: self.data + impl_vec!(@scalar_simd rhs, $size, $vec_type, $simd_type, add) }
                }
            }
            #[doc = concat!("Adds a scalar to all components of the ", $size, "D vector (with reference)\n\n")]
            #[doc = concat!("`v + s = ", impl_vec!(@scalar_op_doc $size, add), "`")]
            impl std::ops::Add<$vec_type> for &$vec_name {
                type Output = $vec_name;
                fn add(self, rhs: $vec_type) -> Self::Output { (*self).add(rhs) }
            }
            #[doc = concat!("Adds a scalar to all components of the ", $size, "D vector (with reference)\n\n")]
            #[doc = concat!("`v + s = ", impl_vec!(@scalar_op_doc $size, add), "`")]
            impl std::ops::Add<&$vec_type> for $vec_name {
                type Output = $vec_name;
                fn add(self, rhs: &$vec_type) -> Self::Output { self.add(*rhs) }
            }
            #[doc = concat!("Adds a scalar to all components of the ", $size, "D vector (with references)\n\n")]
            #[doc = concat!("`v + s = ", impl_vec!(@scalar_op_doc $size, add), "`")]
            impl std::ops::Add<&$vec_type> for &$vec_name {
                type Output = $vec_name;
                fn add(self, rhs: &$vec_type) -> Self::Output { (*self).add(*rhs) }
            }
        }
    };

    // Subtraction operators
    (@impl_sub $vec_name:ident, $simd_type:ty, $vec_type:ty, $size:tt, true) => {
        paste::paste! {
            #[doc = concat!("Component-wise negation of the ", $size, "D vector\n\n")]
            #[doc = concat!("`-v = ", impl_vec!(@neg_doc $size), "`")]
            impl std::ops::Neg for $vec_name {
                type Output = $vec_name;
                fn neg(self) -> Self::Output {
                    $vec_name { data: -self.data }
                }
            }
        }
        impl_vec!(@impl_sub_ops $vec_name, $simd_type, $vec_type, $size);
    };
    (@impl_sub $vec_name:ident, $simd_type:ty, $vec_type:ty, $size:tt, false) => {
        impl_vec!(@impl_sub_ops $vec_name, $simd_type, $vec_type, $size);
    };

    (@impl_sub_ops $vec_name:ident, $simd_type:ty, $vec_type:ty, $size:tt) => {
        paste::paste! {
            #[doc = concat!("Component-wise subtraction of two ", $size, "D vectors\n\n")]
            #[doc = concat!("`v1 - v2 = ", impl_vec!(@op_doc $size, sub), "`")]
            impl std::ops::Sub<$vec_name> for $vec_name {
                type Output = $vec_name;
                fn sub(self, rhs: $vec_name) -> Self::Output {
                    $vec_name { data: self.data - rhs.data }
                }
            }
            #[doc = concat!("Component-wise subtraction of two ", $size, "D vectors (with reference)\n\n")]
            #[doc = concat!("`v1 - v2 = ", impl_vec!(@op_doc $size, sub), "`")]
            impl std::ops::Sub<&$vec_name> for $vec_name {
                type Output = $vec_name;
                fn sub(self, rhs: &$vec_name) -> Self::Output { self.sub(*rhs) }
            }
            #[doc = concat!("Component-wise subtraction of two ", $size, "D vectors (with reference)\n\n")]
            #[doc = concat!("`v1 - v2 = ", impl_vec!(@op_doc $size, sub), "`")]
            impl std::ops::Sub<$vec_name> for &$vec_name {
                type Output = $vec_name;
                fn sub(self, rhs: $vec_name) -> Self::Output { (*self).sub(rhs) }
            }
            #[doc = concat!("Component-wise subtraction of two ", $size, "D vectors (with references)\n\n")]
            #[doc = concat!("`v1 - v2 = ", impl_vec!(@op_doc $size, sub), "`")]
            impl std::ops::Sub<&$vec_name> for &$vec_name {
                type Output = $vec_name;
                fn sub(self, rhs: &$vec_name) -> Self::Output { (*self).sub(*rhs) }
            }
            #[doc = concat!("Subtracts a scalar from all components of the ", $size, "D vector\n\n")]
            #[doc = concat!("`v - s = ", impl_vec!(@scalar_op_doc $size, sub), "`")]
            impl std::ops::Sub<$vec_type> for $vec_name {
                type Output = $vec_name;
                fn sub(self, rhs: $vec_type) -> Self::Output {
                    $vec_name { data: self.data - impl_vec!(@scalar_simd rhs, $size, $vec_type, $simd_type, sub) }
                }
            }
            #[doc = concat!("Subtracts a scalar from all components of the ", $size, "D vector (with reference)\n\n")]
            #[doc = concat!("`v - s = ", impl_vec!(@scalar_op_doc $size, sub), "`")]
            impl std::ops::Sub<$vec_type> for &$vec_name {
                type Output = $vec_name;
                fn sub(self, rhs: $vec_type) -> Self::Output { (*self).sub(rhs) }
            }
            #[doc = concat!("Subtracts a scalar from all components of the ", $size, "D vector (with reference)\n\n")]
            #[doc = concat!("`v - s = ", impl_vec!(@scalar_op_doc $size, sub), "`")]
            impl std::ops::Sub<&$vec_type> for $vec_name {
                type Output = $vec_name;
                fn sub(self, rhs: &$vec_type) -> Self::Output { self.sub(*rhs) }
            }
            #[doc = concat!("Subtracts a scalar from all components of the ", $size, "D vector (with references)\n\n")]
            #[doc = concat!("`v - s = ", impl_vec!(@scalar_op_doc $size, sub), "`")]
            impl std::ops::Sub<&$vec_type> for &$vec_name {
                type Output = $vec_name;
                fn sub(self, rhs: &$vec_type) -> Self::Output { (*self).sub(*rhs) }
            }
        }
    };

    // Multiplication operators
    (@impl_mul $vec_name:ident, $simd_type:ty, $vec_type:ty, $size:tt) => {
        paste::paste! {
            #[doc = concat!("Component-wise multiplication of two ", $size, "D vectors\n\n")]
            #[doc = concat!("`v1 * v2 = ", impl_vec!(@op_doc $size, mul), "`")]
            impl std::ops::Mul<$vec_name> for $vec_name {
                type Output = $vec_name;
                fn mul(self, rhs: $vec_name) -> Self::Output {
                    $vec_name { data: self.data * rhs.data }
                }
            }
            #[doc = concat!("Component-wise multiplication of two ", $size, "D vectors (with reference)\n\n")]
            #[doc = concat!("`v1 * v2 = ", impl_vec!(@op_doc $size, mul), "`")]
            impl std::ops::Mul<&$vec_name> for $vec_name {
                type Output = $vec_name;
                fn mul(self, rhs: &$vec_name) -> Self::Output { self.mul(*rhs) }
            }
            #[doc = concat!("Component-wise multiplication of two ", $size, "D vectors (with reference)\n\n")]
            #[doc = concat!("`v1 * v2 = ", impl_vec!(@op_doc $size, mul), "`")]
            impl std::ops::Mul<$vec_name> for &$vec_name {
                type Output = $vec_name;
                fn mul(self, rhs: $vec_name) -> Self::Output { (*self).mul(rhs) }
            }
            #[doc = concat!("Component-wise multiplication of two ", $size, "D vectors (with references)\n\n")]
            #[doc = concat!("`v1 * v2 = ", impl_vec!(@op_doc $size, mul), "`")]
            impl std::ops::Mul<&$vec_name> for &$vec_name {
                type Output = $vec_name;
                fn mul(self, rhs: &$vec_name) -> Self::Output { (*self).mul(*rhs) }
            }
            #[doc = concat!("Multiplies all components of the ", $size, "D vector by a scalar\n\n")]
            #[doc = concat!("`v * s = ", impl_vec!(@scalar_op_doc $size, mul), "`")]
            impl std::ops::Mul<$vec_type> for $vec_name {
                type Output = $vec_name;
                fn mul(self, rhs: $vec_type) -> Self::Output {
                    $vec_name { data: self.data * impl_vec!(@scalar_simd rhs, $size, $vec_type, $simd_type, mul) }
                }
            }
            #[doc = concat!("Multiplies all components of the ", $size, "D vector by a scalar (with reference)\n\n")]
            #[doc = concat!("`v * s = ", impl_vec!(@scalar_op_doc $size, mul), "`")]
            impl std::ops::Mul<$vec_type> for &$vec_name {
                type Output = $vec_name;
                fn mul(self, rhs: $vec_type) -> Self::Output { (*self).mul(rhs) }
            }
            #[doc = concat!("Multiplies all components of the ", $size, "D vector by a scalar (with reference)\n\n")]
            #[doc = concat!("`v * s = ", impl_vec!(@scalar_op_doc $size, mul), "`")]
            impl std::ops::Mul<&$vec_type> for $vec_name {
                type Output = $vec_name;
                fn mul(self, rhs: &$vec_type) -> Self::Output { self.mul(*rhs) }
            }
            #[doc = concat!("Multiplies all components of the ", $size, "D vector by a scalar (with references)\n\n")]
            #[doc = concat!("`v * s = ", impl_vec!(@scalar_op_doc $size, mul), "`")]
            impl std::ops::Mul<&$vec_type> for &$vec_name {
                type Output = $vec_name;
                fn mul(self, rhs: &$vec_type) -> Self::Output { (*self).mul(*rhs) }
            }
        }
    };

    // Division operators
    (@impl_div $vec_name:ident, $simd_type:ty, $vec_type:ty, $size:tt) => {
        paste::paste! {
            #[doc = concat!("Component-wise division of two ", $size, "D vectors\n\n")]
            #[doc = concat!("`v1 / v2 = ", impl_vec!(@op_doc $size, div), "`")]
            impl std::ops::Div<$vec_name> for $vec_name {
                type Output = $vec_name;
                fn div(self, rhs: $vec_name) -> Self::Output {
                    $vec_name { data: self.data / rhs.data }
                }
            }
            #[doc = concat!("Component-wise division of two ", $size, "D vectors (with reference)\n\n")]
            #[doc = concat!("`v1 / v2 = ", impl_vec!(@op_doc $size, div), "`")]
            impl std::ops::Div<&$vec_name> for $vec_name {
                type Output = $vec_name;
                fn div(self, rhs: &$vec_name) -> Self::Output { self.div(*rhs) }
            }
            #[doc = concat!("Component-wise division of two ", $size, "D vectors (with reference)\n\n")]
            #[doc = concat!("`v1 / v2 = ", impl_vec!(@op_doc $size, div), "`")]
            impl std::ops::Div<$vec_name> for &$vec_name {
                type Output = $vec_name;
                fn div(self, rhs: $vec_name) -> Self::Output { (*self).div(rhs) }
            }
            #[doc = concat!("Component-wise division of two " ,$size ,"D vectors (with references)\n\n")]
            #[doc = concat!("`v1 / v2 = ", impl_vec!(@op_doc $size, div), "`")]
            impl std::ops::Div<&$vec_name> for &$vec_name {
                type Output = $vec_name;
                fn div(self, rhs: &$vec_name) -> Self::Output { (*self).div(*rhs) }
            }
            #[doc = concat!("Divides all components of the ", $size, "D vector by a scalar\n\n")]
            #[doc = concat!("`v / s = ", impl_vec!(@scalar_op_doc $size, div), "`")]
            impl std::ops::Div<$vec_type> for $vec_name {
                type Output = $vec_name;
                fn div(self, rhs: $vec_type) -> Self::Output {
                    $vec_name { data: self.data / impl_vec!(@scalar_simd rhs, $size, $vec_type, $simd_type, div) }
                }
            }
            #[doc = concat!("Divides all components of the ", $size, "D vector by a scalar (with reference)\n\n")]
            #[doc = concat!("`v / s = ", impl_vec!(@scalar_op_doc $size, div), "`")]
            impl std::ops::Div<$vec_type> for &$vec_name {
                type Output = $vec_name;
                fn div(self, rhs: $vec_type) -> Self::Output { (*self).div(rhs) }
            }
            #[doc = concat!("Divides all components of the ", $size, "D vector by a scalar (with reference)\n\n")]
            #[doc = concat!("`v / s = ", impl_vec!(@scalar_op_doc $size, div), "`")]
            impl std::ops::Div<&$vec_type> for $vec_name {
                type Output = $vec_name;
                fn div(self, rhs: &$vec_type) -> Self::Output { self.div(*rhs) }
            }
            #[doc = concat!("Divides all components of the ", $size, "D vector by a scalar (with references)\n\n")]
            #[doc = concat!("`v / s = ", impl_vec!(@scalar_op_doc $size, div), "`")]
            impl std::ops::Div<&$vec_type> for &$vec_name {
                type Output = $vec_name;
                fn div(self, rhs: &$vec_type) -> Self::Output { (*self).div(*rhs) }
            }
        }
    };

    // Assignment operators
    (@impl_assign $vec_name:ident, $simd_type:ty, $vec_type:ty, $size:tt) => {
        paste::paste! {
            #[doc = concat!("Component-wise addition assignment for ", $size, "D vectors\n\n")]
            #[doc = concat!("`v1 += v2` is equivalent to `v1 = ", impl_vec!(@op_doc $size, add), "`")]
            impl std::ops::AddAssign<$vec_name> for $vec_name {
                fn add_assign(&mut self, rhs: $vec_name) {
                    self.data = self.data + rhs.data;
                }
            }
            #[doc = concat!("Component-wise addition assignment for ", $size, "D vectors (with reference)\n\n")]
            #[doc = concat!("`v1 += v2` is equivalent to `v1 = ", impl_vec!(@op_doc $size, add), "`")]
            impl std::ops::AddAssign<&$vec_name> for $vec_name {
                fn add_assign(&mut self, rhs: &$vec_name) {
                    self.add_assign(*rhs);
                }
            }
            #[doc = concat!("Adds a scalar to all components of the ", $size, "D vector in place\n\n")]
            #[doc = concat!("`v += s` is equivalent to `v = ", impl_vec!(@scalar_op_doc $size, add), "`")]
            impl std::ops::AddAssign<$vec_type> for $vec_name {
                fn add_assign(&mut self, rhs: $vec_type) {
                    self.data = self.data + impl_vec!(@scalar_simd rhs, $size, $vec_type, $simd_type, add);
                }
            }
            #[doc = concat!("Adds a scalar to all components of the ", $size, "D vector in place (with reference)\n\n")]
            #[doc = concat!("`v += s` is equivalent to `v = ", impl_vec!(@scalar_op_doc $size, add), "`")]
            impl std::ops::AddAssign<&$vec_type> for $vec_name {
                fn add_assign(&mut self, rhs: &$vec_type) {
                    self.add_assign(*rhs);
                }
            }

            #[doc = concat!("Component-wise subtraction assignment for ", $size, "D vectors\n\n")]
            #[doc = concat!("`v1 -= v2` is equivalent to `v1 = ", impl_vec!(@op_doc $size, sub), "`")]
            impl std::ops::SubAssign<$vec_name> for $vec_name {
                fn sub_assign(&mut self, rhs: $vec_name) {
                    self.data = self.data - rhs.data;
                }
            }
            #[doc = concat!("Component-wise subtraction assignment for ", $size, "D vectors (with reference)\n\n")]
            #[doc = concat!("`v1 -= v2` is equivalent to `v1 = ", impl_vec!(@op_doc $size, sub), "`")]
            impl std::ops::SubAssign<&$vec_name> for $vec_name {
                fn sub_assign(&mut self, rhs: &$vec_name) {
                    self.sub_assign(*rhs);
                }
            }
            #[doc = concat!("Subtracts a scalar from all components of the ", $size, "D vector in place\n\n")]
            #[doc = concat!("`v -= s` is equivalent to `v = ", impl_vec!(@scalar_op_doc $size, sub), "`")]
            impl std::ops::SubAssign<$vec_type> for $vec_name {
                fn sub_assign(&mut self, rhs: $vec_type) {
                    self.data = self.data - impl_vec!(@scalar_simd rhs, $size, $vec_type, $simd_type, sub);
                }
            }
            #[doc = concat!("Subtracts a scalar from all components of the ", $size, "D vector in place (with reference)\n\n")]
            #[doc = concat!("`v -= s` is equivalent to `v = ", impl_vec!(@scalar_op_doc $size, sub), "`")]
            impl std::ops::SubAssign<&$vec_type> for $vec_name {
                fn sub_assign(&mut self, rhs: &$vec_type) {
                    self.sub_assign(*rhs);
                }
            }

            #[doc = concat!("Component-wise multiplication assignment for ", $size, "D vectors\n\n")]
            #[doc = concat!("`v1 *= v2` is equivalent to `v1 = ", impl_vec!(@op_doc $size, mul), "`")]
            impl std::ops::MulAssign<$vec_name> for $vec_name {
                fn mul_assign(&mut self, rhs: $vec_name) {
                    self.data = self.data * rhs.data;
                }
            }
            #[doc = concat!("Component-wise multiplication assignment for ", $size, "D vectors (with reference)\n\n")]
            #[doc = concat!("`v1 *= v2` is equivalent to `v1 = ", impl_vec!(@op_doc $size, mul), "`")]
            impl std::ops::MulAssign<&$vec_name> for $vec_name {
                fn mul_assign(&mut self, rhs: &$vec_name) {
                    self.mul_assign(*rhs);
                }
            }
            #[doc = concat!("Multiplies all components of the ", $size, "D vector by a scalar in place\n\n")]
            #[doc = concat!("`v *= s` is equivalent to `v = ", impl_vec!(@scalar_op_doc $size, mul), "`")]
            impl std::ops::MulAssign<$vec_type> for $vec_name {
                fn mul_assign(&mut self, rhs: $vec_type) {
                    self.data = self.data * impl_vec!(@scalar_simd rhs, $size, $vec_type, $simd_type, mul);
                }
            }
            #[doc = concat!("Multiplies all components of the ", $size, "D vector by a scalar in place (with reference)\n\n")]
            #[doc = concat!("`v *= s` is equivalent to `v = ", impl_vec!(@scalar_op_doc $size, mul), "`")]
            impl std::ops::MulAssign<&$vec_type> for $vec_name {
                fn mul_assign(&mut self, rhs: &$vec_type) {
                    self.mul_assign(*rhs);
                }
            }

            #[doc = concat!("Component-wise division assignment for ", $size, "D vectors\n\n")]
            #[doc = concat!("`v1 /= v2` is equivalent to `v1 = ", impl_vec!(@op_doc $size, div), "`")]
            impl std::ops::DivAssign<$vec_name> for $vec_name {
                fn div_assign(&mut self, rhs: $vec_name) {
                    self.data = self.data / rhs.data;
                }
            }
            #[doc = concat!("Component-wise division assignment for ", $size, "D vectors (with reference)\n\n")]
            #[doc = concat!("`v1 /= v2` is equivalent to `v1 = ", impl_vec!(@op_doc $size, div), "`")]
            impl std::ops::DivAssign<&$vec_name> for $vec_name {
                fn div_assign(&mut self, rhs: &$vec_name) {
                    self.div_assign(*rhs);
                }
            }
            #[doc = concat!("Divides all components of the ", $size, "D vector by a scalar in place\n\n")]
            #[doc = concat!("`v /= s` is equivalent to `v = ", impl_vec!(@scalar_op_doc $size, div), "`")]
            impl std::ops::DivAssign<$vec_type> for $vec_name {
                fn div_assign(&mut self, rhs: $vec_type) {
                    self.data = self.data / impl_vec!(@scalar_simd rhs, $size, $vec_type, $simd_type, div);
                }
            }
            #[doc = concat!("Divides all components of the ", $size, "D vector by a scalar in place (with reference)\n\n")]
            #[doc = concat!("`v /= s` is equivalent to `v = ", impl_vec!(@scalar_op_doc $size, div), "`")]
            impl std::ops::DivAssign<&$vec_type> for $vec_name {
                fn div_assign(&mut self, rhs: &$vec_type) {
                    self.div_assign(*rhs);
                }
            }
        }
    };

    // Documentation helper for operation descriptions
    (@op_doc 2, add) => { "[v1.x + v2.x, v1.y + v2.y]" };
    (@op_doc 2, sub) => { "[v1.x - v2.x, v1.y - v2.y]" };
    (@op_doc 2, mul) => { "[v1.x * v2.x, v1.y * v2.y]" };
    (@op_doc 2, div) => { "[v1.x / v2.x, v1.y / v2.y]" };

    (@op_doc 3, add) => { "[v1.x + v2.x, v1.y + v2.y, v1.z + v2.z]" };
    (@op_doc 3, sub) => { "[v1.x - v2.x, v1.y - v2.y, v1.z - v2.z]" };
    (@op_doc 3, mul) => { "[v1.x * v2.x, v1.y * v2.y, v1.z * v2.z]" };
    (@op_doc 3, div) => { "[v1.x / v2.x, v1.y / v2.y, v1.z / v2.z]" };

    (@op_doc 4, add) => { "[v1.x + v2.x, v1.y + v2.y, v1.z + v2.z, v1.w + v2.w]" };
    (@op_doc 4, sub) => { "[v1.x - v2.x, v1.y - v2.y, v1.z - v2.z, v1.w - v2.w]" };
    (@op_doc 4, mul) => { "[v1.x * v2.x, v1.y * v2.y, v1.z * v2.z, v1.w * v2.w]" };
    (@op_doc 4, div) => { "[v1.x / v2.x, v1.y / v2.y, v1.z / v2.z, v1.w / v2.w]" };

    (@scalar_op_doc 2, add) => { "[v.x + s, v.y + s]" };
    (@scalar_op_doc 2, sub) => { "[v.x - s, v.y - s]" };
    (@scalar_op_doc 2, mul) => { "[v.x * s, v.y * s]" };
    (@scalar_op_doc 2, div) => { "[v.x / s, v.y / s]" };

    (@scalar_op_doc 3, add) => { "[v.x + s, v.y + s, v.z + s]" };
    (@scalar_op_doc 3, sub) => { "[v.x - s, v.y - s, v.z - s]" };
    (@scalar_op_doc 3, mul) => { "[v.x * s, v.y * s, v.z * s]" };
    (@scalar_op_doc 3, div) => { "[v.x / s, v.y / s, v.z / s]" };

    (@scalar_op_doc 4, add) => { "[v.x + s, v.y + s, v.z + s, v.w + s]" };
    (@scalar_op_doc 4, sub) => { "[v.x - s, v.y - s, v.z - s, v.w - s]" };
    (@scalar_op_doc 4, mul) => { "[v.x * s, v.y * s, v.z * s, v.w * s]" };
    (@scalar_op_doc 4, div) => { "[v.x / s, v.y / s, v.z / s, v.w / s]" };

    (@neg_doc 2) => { "[-v.x, -v.y]" };
    (@neg_doc 3) => { "[-v.x, -v.y, -v.z]" };
    (@neg_doc 4) => { "[-v.x, -v.y, -v.z, -v.w]" };

    // Scalar to SIMD conversion for operations
    (@scalar_simd $rhs:expr, 2, $vec_type:ty, $simd_type:ty, add) => {
        <$simd_type>::from_array([$rhs, $rhs])
    };
    (@scalar_simd $rhs:expr, 2, $vec_type:ty, $simd_type:ty, sub) => {
        <$simd_type>::from_array([$rhs, $rhs])
    };
    (@scalar_simd $rhs:expr, 2, $vec_type:ty, $simd_type:ty, mul) => {
        <$simd_type>::from_array([$rhs, $rhs])
    };
    (@scalar_simd $rhs:expr, 2, $vec_type:ty, $simd_type:ty, div) => {
        <$simd_type>::from_array([$rhs, $rhs])
    };

    (@scalar_simd $rhs:expr, 3, $vec_type:ty, $simd_type:ty, add) => {
        <$simd_type>::from_array([$rhs, $rhs, $rhs, 0 as $vec_type])
    };
    (@scalar_simd $rhs:expr, 3, $vec_type:ty, $simd_type:ty, sub) => {
        <$simd_type>::from_array([$rhs, $rhs, $rhs, 0 as $vec_type])
    };
    (@scalar_simd $rhs:expr, 3, $vec_type:ty, $simd_type:ty, mul) => {
        <$simd_type>::from_array([$rhs, $rhs, $rhs, 1 as $vec_type])
    };
    (@scalar_simd $rhs:expr, 3, $vec_type:ty, $simd_type:ty, div) => {
        <$simd_type>::from_array([$rhs, $rhs, $rhs, 1 as $vec_type])
    };

    (@scalar_simd $rhs:expr, 4, $vec_type:ty, $simd_type:ty, add) => {
        <$simd_type>::from_array([$rhs, $rhs, $rhs, $rhs])
    };
    (@scalar_simd $rhs:expr, 4, $vec_type:ty, $simd_type:ty, sub) => {
        <$simd_type>::from_array([$rhs, $rhs, $rhs, $rhs])
    };
    (@scalar_simd $rhs:expr, 4, $vec_type:ty, $simd_type:ty, mul) => {
        <$simd_type>::from_array([$rhs, $rhs, $rhs, $rhs])
    };
    (@scalar_simd $rhs:expr, 4, $vec_type:ty, $simd_type:ty, div) => {
        <$simd_type>::from_array([$rhs, $rhs, $rhs, $rhs])
    };
}

impl_vec!(Vector4f32, f32x4, f32, 4, true, true);
impl_vec!(Vector4f64, f64x4, f64, 4, true, true);
impl_vec!(Vector4i8, i8x4, i8, 4, true, false);
impl_vec!(Vector4i16, i16x4, i16, 4, true, false);
impl_vec!(Vector4i32, i32x4, i32, 4, true, false);
impl_vec!(Vector4i64, i64x4, i64, 4, true, false);
impl_vec!(Vector4isize, isizex4, isize, 4, true, false);
impl_vec!(Vector4u8, u8x4, u8, 4, false, false);
impl_vec!(Vector4u16, u16x4, u16, 4, false, false);
impl_vec!(Vector4u32, u32x4, u32, 4, false, false);
impl_vec!(Vector4u64, u64x4, u64, 4, false, false);
impl_vec!(Vector4usize, usizex4, usize, 4, false, false);

impl_vec!(Vector3f32, f32x4, f32, 3, true, true);
impl_vec!(Vector3f64, f64x4, f64, 3, true, true);
impl_vec!(Vector3i8, i8x4, i8, 3, true, false);
impl_vec!(Vector3i16, i16x4, i16, 3, true, false);
impl_vec!(Vector3i32, i32x4, i32, 3, true, false);
impl_vec!(Vector3i64, i64x4, i64, 3, true, false);
impl_vec!(Vector3isize, isizex4, isize, 3, true, false);
impl_vec!(Vector3u8, u8x4, u8, 3, false, false);
impl_vec!(Vector3u16, u16x4, u16, 3, false, false);
impl_vec!(Vector3u32, u32x4, u32, 3, false, false);
impl_vec!(Vector3u64, u64x4, u64, 3, false, false);
impl_vec!(Vector3usize, usizex4, usize, 3, false, false);

impl_vec!(Vector2f32, f32x2, f32, 2, true, true);
impl_vec!(Vector2f64, f64x2, f64, 2, true, true);
impl_vec!(Vector2i8, i8x2, i8, 2, true, false);
impl_vec!(Vector2i16, i16x2, i16, 2, true, false);
impl_vec!(Vector2i32, i32x2, i32, 2, true, false);
impl_vec!(Vector2i64, i64x2, i64, 2, true, false);
impl_vec!(Vector2isize, isizex2, isize, 2, true, false);
impl_vec!(Vector2u8, u8x2, u8, 2, false, false);
impl_vec!(Vector2u16, u16x2, u16, 2, false, false);
impl_vec!(Vector2u32, u32x2, u32, 2, false, false);
impl_vec!(Vector2u64, u64x2, u64, 2, false, false);
impl_vec!(Vector2usize, usizex2, usize, 2, false, false);
