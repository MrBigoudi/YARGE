use std::simd::prelude::*;

use crate::maths::{Vector3f32, Vector4f32, vec4f32};

/// A structure to represent a 4x4 f32 matrix stored in column-major order
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Matrix4x4f32 {
    col0: f32x4, // First column [m00, m10, m20, m30]
    col1: f32x4, // Second column [m01, m11, m21, m31]
    col2: f32x4, // Third column [m02, m12, m22, m32]
    col3: f32x4, // Fourth column [m03, m13, m23, m33]
}

/// A structure to allow accessing matrix elements
mod private {
    #[repr(C)]
    pub struct MatrixElements {
        pub m00: f32,
        pub m10: f32,
        pub m20: f32,
        pub m30: f32,
        pub m01: f32,
        pub m11: f32,
        pub m21: f32,
        pub m31: f32,
        pub m02: f32,
        pub m12: f32,
        pub m22: f32,
        pub m32: f32,
        pub m03: f32,
        pub m13: f32,
        pub m23: f32,
        pub m33: f32,
    }
}

/// Implements `Deref` to allow accessing matrix elements
impl std::ops::Deref for Matrix4x4f32 {
    type Target = private::MatrixElements;

    fn deref(&self) -> &Self::Target {
        let value: *const Matrix4x4f32 = self;
        unsafe { &*(value as *const private::MatrixElements) }
    }
}

/// Implements `DerefMut` to allow modifying matrix elements
impl std::ops::DerefMut for Matrix4x4f32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value: *mut Matrix4x4f32 = self;
        unsafe { &mut *(value as *mut private::MatrixElements) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Matrix4x4f32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix4x4f32")
            .field("m00", &self.m00)
            .field("m01", &self.m01)
            .field("m02", &self.m02)
            .field("m03", &self.m03)
            .field("m10", &self.m10)
            .field("m11", &self.m11)
            .field("m12", &self.m12)
            .field("m13", &self.m13)
            .field("m20", &self.m20)
            .field("m21", &self.m21)
            .field("m22", &self.m22)
            .field("m23", &self.m23)
            .field("m30", &self.m30)
            .field("m31", &self.m31)
            .field("m32", &self.m32)
            .field("m33", &self.m33)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Matrix4x4f32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}, {}, {}],[{}, {}, {}, {}],[{}, {}, {}, {}],[{}, {}, {}, {}]",
            self.m00,
            self.m01,
            self.m02,
            self.m03,
            self.m10,
            self.m11,
            self.m12,
            self.m13,
            self.m20,
            self.m21,
            self.m22,
            self.m23,
            self.m30,
            self.m31,
            self.m32,
            self.m33
        )
    }
}

/// Sets a 4x4 matrix to identity as default
impl Default for Matrix4x4f32 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// Creates a 4x4 f32 matrix
pub fn mat4x4f32(
    row_0: &Vector4f32,
    row_1: &Vector4f32,
    row_2: &Vector4f32,
    row_3: &Vector4f32,
) -> Matrix4x4f32 {
    Matrix4x4f32::new(row_0, row_1, row_2, row_3)
}

impl Matrix4x4f32 {
    //////////////////////////////////////////////////////////
    /////////////      matrix creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new matrix given its elements in row-major order
    pub const fn new(
        row_0: &Vector4f32,
        row_1: &Vector4f32,
        row_2: &Vector4f32,
        row_3: &Vector4f32,
    ) -> Self {
        Self {
            col0: f32x4::from_array([
                row_0.x_const(),
                row_1.x_const(),
                row_2.x_const(),
                row_3.x_const(),
            ]),
            col1: f32x4::from_array([
                row_0.y_const(),
                row_1.y_const(),
                row_2.y_const(),
                row_3.y_const(),
            ]),
            col2: f32x4::from_array([
                row_0.z_const(),
                row_1.z_const(),
                row_2.z_const(),
                row_3.z_const(),
            ]),
            col3: f32x4::from_array([
                row_0.w_const(),
                row_1.w_const(),
                row_2.w_const(),
                row_3.w_const(),
            ]),
        }
    }

    /// Creates a new matrix with all elements set to `value`
    const fn splat(value: f32) -> Self {
        Self::new(
            &vec4f32(value, value, value, value),
            &vec4f32(value, value, value, value),
            &vec4f32(value, value, value, value),
            &vec4f32(value, value, value, value),
        )
    }

    /// Creates a new matrix filled with `value`
    pub const fn filled(value: f32) -> Self {
        Self::splat(value)
    }

    /// Creates a new matrix filled with ones
    pub const ONES: Self = Self::splat(1.);

    /// Creates a new matrix filled with zeros
    pub const ZEROS: Self = Self::splat(0.);

    /// Creates an identity matrix
    pub const IDENTITY: Self = Self::diagonal(1., 1., 1., 1.);

    /// Creates a diagonal matrix
    pub const fn diagonal(d0: f32, d1: f32, d2: f32, d3: f32) -> Self {
        Self::new(
            &vec4f32(d0, 0., 0., 0.),
            &vec4f32(0., d1, 0., 0.),
            &vec4f32(0., 0., d2, 0.),
            &vec4f32(0., 0., 0., d3),
        )
    }

    /// Creates a look-at matrix
    pub fn look_at(
        origin_position: &Vector3f32,
        target_position: &Vector3f32,
        world_up: &Vector3f32,
    ) -> Self {
        let forward = (origin_position - target_position)
            .normalize()
            .expect("Failed to normalize a vector when building a look at matrix");
        let right = Vector3f32::cross(world_up, &forward)
            .normalize()
            .expect("Failed to normalize a vector when building a look at matrix");
        let up = Vector3f32::cross(&forward, &right);

        let rotation = Self::new(
            &vec4f32(right.x, right.y, right.z, 1f32),
            &vec4f32(up.x, up.y, up.z, 1f32),
            &vec4f32(forward.x, forward.y, forward.z, 1f32),
            &Vector4f32::ONES,
        );

        let translation =
            Self::translation(-origin_position.x, -origin_position.y, -origin_position.z);

        rotation * translation
    }

    //////////////////////////////////////////////////////////
    /////////////     matrix operations      /////////////////
    //////////////////////////////////////////////////////////

    /// Returns the determinant of the matrix
    pub fn determinant(&self) -> f32 {
        // Using cofactor expansion along the first row
        let a00 = self.m00;
        let a01 = self.m01;
        let a02 = self.m02;
        let a03 = self.m03;

        // 3x3 subdeterminants
        let b00 = self.m11 * (self.m22 * self.m33 - self.m23 * self.m32)
            - self.m12 * (self.m21 * self.m33 - self.m23 * self.m31)
            + self.m13 * (self.m21 * self.m32 - self.m22 * self.m31);

        let b01 = self.m10 * (self.m22 * self.m33 - self.m23 * self.m32)
            - self.m12 * (self.m20 * self.m33 - self.m23 * self.m30)
            + self.m13 * (self.m20 * self.m32 - self.m22 * self.m30);

        let b02 = self.m10 * (self.m21 * self.m33 - self.m23 * self.m31)
            - self.m11 * (self.m20 * self.m33 - self.m23 * self.m30)
            + self.m13 * (self.m20 * self.m31 - self.m21 * self.m30);

        let b03 = self.m10 * (self.m21 * self.m32 - self.m22 * self.m31)
            - self.m11 * (self.m20 * self.m32 - self.m22 * self.m30)
            + self.m12 * (self.m20 * self.m31 - self.m21 * self.m30);

        a00 * b00 - a01 * b01 + a02 * b02 - a03 * b03
    }

    /// Returns the trace of the matrix (sum of diagonal elements)
    pub fn trace(&self) -> f32 {
        self.m00 + self.m11 + self.m22 + self.m33
    }

    /// Returns the transpose of the matrix
    pub fn transpose(&self) -> Self {
        Self::new(
            &vec4f32(self.m00, self.m10, self.m20, self.m30),
            &vec4f32(self.m01, self.m11, self.m21, self.m31),
            &vec4f32(self.m02, self.m12, self.m22, self.m32),
            &vec4f32(self.m03, self.m13, self.m23, self.m33),
        )
    }

    /// Returns the inverse of the matrix, or None if not invertible
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < f32::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;

        // Calculate cofactor matrix (this is lengthy for 4x4)
        let c00 = self.m11 * (self.m22 * self.m33 - self.m23 * self.m32)
            - self.m12 * (self.m21 * self.m33 - self.m23 * self.m31)
            + self.m13 * (self.m21 * self.m32 - self.m22 * self.m31);

        let c01 = -(self.m10 * (self.m22 * self.m33 - self.m23 * self.m32)
            - self.m12 * (self.m20 * self.m33 - self.m23 * self.m30)
            + self.m13 * (self.m20 * self.m32 - self.m22 * self.m30));

        let c02 = self.m10 * (self.m21 * self.m33 - self.m23 * self.m31)
            - self.m11 * (self.m20 * self.m33 - self.m23 * self.m30)
            + self.m13 * (self.m20 * self.m31 - self.m21 * self.m30);

        let c03 = -(self.m10 * (self.m21 * self.m32 - self.m22 * self.m31)
            - self.m11 * (self.m20 * self.m32 - self.m22 * self.m30)
            + self.m12 * (self.m20 * self.m31 - self.m21 * self.m30));

        let c10 = -(self.m01 * (self.m22 * self.m33 - self.m23 * self.m32)
            - self.m02 * (self.m21 * self.m33 - self.m23 * self.m31)
            + self.m03 * (self.m21 * self.m32 - self.m22 * self.m31));

        let c11 = self.m00 * (self.m22 * self.m33 - self.m23 * self.m32)
            - self.m02 * (self.m20 * self.m33 - self.m23 * self.m30)
            + self.m03 * (self.m20 * self.m32 - self.m22 * self.m30);

        let c12 = -(self.m00 * (self.m21 * self.m33 - self.m23 * self.m31)
            - self.m01 * (self.m20 * self.m33 - self.m23 * self.m30)
            + self.m03 * (self.m20 * self.m31 - self.m21 * self.m30));

        let c13 = self.m00 * (self.m21 * self.m32 - self.m22 * self.m31)
            - self.m01 * (self.m20 * self.m32 - self.m22 * self.m30)
            + self.m02 * (self.m20 * self.m31 - self.m21 * self.m30);

        let c20 = self.m01 * (self.m12 * self.m33 - self.m13 * self.m32)
            - self.m02 * (self.m11 * self.m33 - self.m13 * self.m31)
            + self.m03 * (self.m11 * self.m32 - self.m12 * self.m31);

        let c21 = -(self.m00 * (self.m12 * self.m33 - self.m13 * self.m32)
            - self.m02 * (self.m10 * self.m33 - self.m13 * self.m30)
            + self.m03 * (self.m10 * self.m32 - self.m12 * self.m30));

        let c22 = self.m00 * (self.m11 * self.m33 - self.m13 * self.m31)
            - self.m01 * (self.m10 * self.m33 - self.m13 * self.m30)
            + self.m03 * (self.m10 * self.m31 - self.m11 * self.m30);

        let c23 = -(self.m00 * (self.m11 * self.m32 - self.m12 * self.m31)
            - self.m01 * (self.m10 * self.m32 - self.m12 * self.m30)
            + self.m02 * (self.m10 * self.m31 - self.m11 * self.m30));

        let c30 = -(self.m01 * (self.m12 * self.m23 - self.m13 * self.m22)
            - self.m02 * (self.m11 * self.m23 - self.m13 * self.m21)
            + self.m03 * (self.m11 * self.m22 - self.m12 * self.m21));

        let c31 = self.m00 * (self.m12 * self.m23 - self.m13 * self.m22)
            - self.m02 * (self.m10 * self.m23 - self.m13 * self.m20)
            + self.m03 * (self.m10 * self.m22 - self.m12 * self.m20);

        let c32 = -(self.m00 * (self.m11 * self.m23 - self.m13 * self.m21)
            - self.m01 * (self.m10 * self.m23 - self.m13 * self.m20)
            + self.m03 * (self.m10 * self.m21 - self.m11 * self.m20));

        let c33 = self.m00 * (self.m11 * self.m22 - self.m12 * self.m21)
            - self.m01 * (self.m10 * self.m22 - self.m12 * self.m20)
            + self.m02 * (self.m10 * self.m21 - self.m11 * self.m20);

        // Transpose of cofactor matrix divided by determinant
        Some(Self::new(
            &vec4f32(c00 * inv_det, c10 * inv_det, c20 * inv_det, c30 * inv_det),
            &vec4f32(c01 * inv_det, c11 * inv_det, c21 * inv_det, c31 * inv_det),
            &vec4f32(c02 * inv_det, c12 * inv_det, c22 * inv_det, c32 * inv_det),
            &vec4f32(c03 * inv_det, c13 * inv_det, c23 * inv_det, c33 * inv_det),
        ))
    }

    /// Creates a rotation matrix around the X axis
    pub fn rotation_x(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(
            &vec4f32(1., 0., 0., 0.),
            &vec4f32(0., cos, -sin, 0.),
            &vec4f32(0., sin, cos, 0.),
            &vec4f32(0., 0., 0., 1.),
        )
    }

    /// Creates a rotation matrix around the Y axis
    pub fn rotation_y(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(
            &vec4f32(cos, 0., sin, 0.),
            &vec4f32(0., 1., 0., 0.),
            &vec4f32(-sin, 0., cos, 0.),
            &vec4f32(0., 0., 0., 1.),
        )
    }

    /// Creates a rotation matrix around the Z axis
    pub fn rotation_z(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(
            &vec4f32(cos, -sin, 0., 0.),
            &vec4f32(sin, cos, 0., 0.),
            &vec4f32(0., 0., 1., 0.),
            &vec4f32(0., 0., 0., 1.),
        )
    }

    /// Creates a scaling matrix
    pub const fn scale(sx: f32, sy: f32, sz: f32) -> Self {
        Self::diagonal(sx, sy, sz, 1.)
    }

    /// Creates a translation matrix
    pub const fn translation(tx: f32, ty: f32, tz: f32) -> Self {
        Self::new(
            &vec4f32(1., 0., 0., tx),
            &vec4f32(0., 1., 0., ty),
            &vec4f32(0., 0., 1., tz),
            &vec4f32(0., 0., 0., 1.),
        )
    }

    //////////////////////////////////////////////////////////
    /////////////   row and column access    /////////////////
    //////////////////////////////////////////////////////////

    /// Returns the specified row as a Vector4f32
    pub fn get_row(&self, row: usize) -> Vector4f32 {
        assert!(row < 4, "Row index out of bounds");
        match row {
            0 => vec4f32(self.m00, self.m01, self.m02, self.m03),
            1 => vec4f32(self.m10, self.m11, self.m12, self.m13),
            2 => vec4f32(self.m20, self.m21, self.m22, self.m23),
            3 => vec4f32(self.m30, self.m31, self.m32, self.m33),
            _ => unreachable!(),
        }
    }

    /// Returns the specified column as a Vector4f32
    pub fn get_col(&self, col: usize) -> Vector4f32 {
        assert!(col < 4, "Column index out of bounds");
        match col {
            0 => vec4f32(self.m00, self.m10, self.m20, self.m30),
            1 => vec4f32(self.m01, self.m11, self.m21, self.m31),
            2 => vec4f32(self.m02, self.m12, self.m22, self.m32),
            3 => vec4f32(self.m03, self.m13, self.m23, self.m33),
            _ => unreachable!(),
        }
    }

    /// Sets the specified row from a Vector4f32
    pub fn set_row(&mut self, row: usize, v: &Vector4f32) {
        assert!(row < 4, "Row index out of bounds");
        match row {
            0 => {
                self.m00 = v.x;
                self.m01 = v.y;
                self.m02 = v.z;
                self.m03 = v.w;
            }
            1 => {
                self.m10 = v.x;
                self.m11 = v.y;
                self.m12 = v.z;
                self.m13 = v.w;
            }
            2 => {
                self.m20 = v.x;
                self.m21 = v.y;
                self.m22 = v.z;
                self.m23 = v.w;
            }
            3 => {
                self.m30 = v.x;
                self.m31 = v.y;
                self.m32 = v.z;
                self.m33 = v.w;
            }
            _ => unreachable!(),
        }
    }

    /// Sets the specified column from a Vector4f32
    pub fn set_col(&mut self, col: usize, v: &Vector4f32) {
        assert!(col < 4, "Column index out of bounds");
        match col {
            0 => {
                self.m00 = v.x;
                self.m10 = v.y;
                self.m20 = v.z;
                self.m30 = v.w;
            }
            1 => {
                self.m01 = v.x;
                self.m11 = v.y;
                self.m21 = v.z;
                self.m31 = v.w;
            }
            2 => {
                self.m02 = v.x;
                self.m12 = v.y;
                self.m22 = v.z;
                self.m32 = v.w;
            }
            3 => {
                self.m03 = v.x;
                self.m13 = v.y;
                self.m23 = v.z;
                self.m33 = v.w;
            }
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Matrix4x4f32 {
    fn eq(&self, other: &Self) -> bool {
        self.m00 == other.m00
            && self.m01 == other.m01
            && self.m02 == other.m02
            && self.m03 == other.m03
            && self.m10 == other.m10
            && self.m11 == other.m11
            && self.m12 == other.m12
            && self.m13 == other.m13
            && self.m20 == other.m20
            && self.m21 == other.m21
            && self.m22 == other.m22
            && self.m23 == other.m23
            && self.m30 == other.m30
            && self.m31 == other.m31
            && self.m32 == other.m32
            && self.m33 == other.m33
    }
}

//////////////////////////////////////////////////////////
//////////////     matrix additions     //////////////////
//////////////////////////////////////////////////////////

/// Element-wise addition
impl std::ops::Add<Matrix4x4f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn add(self, rhs: Matrix4x4f32) -> Self::Output {
        Matrix4x4f32 {
            col0: self.col0 + rhs.col0,
            col1: self.col1 + rhs.col1,
            col2: self.col2 + rhs.col2,
            col3: self.col3 + rhs.col3,
        }
    }
}

impl std::ops::Add<&Matrix4x4f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn add(self, rhs: &Matrix4x4f32) -> Self::Output {
        self.add(*rhs)
    }
}

impl std::ops::Add<Matrix4x4f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn add(self, rhs: Matrix4x4f32) -> Self::Output {
        (*self).add(rhs)
    }
}

impl std::ops::Add<&Matrix4x4f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn add(self, rhs: &Matrix4x4f32) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds scalar to all elements
impl std::ops::Add<f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn add(self, rhs: f32) -> Self::Output {
        let splat = f32x4::from_array([rhs, rhs, rhs, rhs]);
        Matrix4x4f32 {
            col0: self.col0 + splat,
            col1: self.col1 + splat,
            col2: self.col2 + splat,
            col3: self.col3 + splat,
        }
    }
}

impl std::ops::Add<f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn add(self, rhs: f32) -> Self::Output {
        (*self).add(rhs)
    }
}

impl std::ops::Add<&f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn add(self, rhs: &f32) -> Self::Output {
        self.add(*rhs)
    }
}

impl std::ops::Add<&f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn add(self, rhs: &f32) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   matrix subtractions    //////////////////
//////////////////////////////////////////////////////////

/// Element-wise negation
impl std::ops::Neg for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn neg(self) -> Self::Output {
        Matrix4x4f32 {
            col0: -self.col0,
            col1: -self.col1,
            col2: -self.col2,
            col3: -self.col3,
        }
    }
}

/// Element-wise subtraction
impl std::ops::Sub<Matrix4x4f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn sub(self, rhs: Matrix4x4f32) -> Self::Output {
        Matrix4x4f32 {
            col0: self.col0 - rhs.col0,
            col1: self.col1 - rhs.col1,
            col2: self.col2 - rhs.col2,
            col3: self.col3 - rhs.col3,
        }
    }
}

impl std::ops::Sub<&Matrix4x4f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn sub(self, rhs: &Matrix4x4f32) -> Self::Output {
        self.sub(*rhs)
    }
}

impl std::ops::Sub<Matrix4x4f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn sub(self, rhs: Matrix4x4f32) -> Self::Output {
        (*self).sub(rhs)
    }
}

impl std::ops::Sub<&Matrix4x4f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn sub(self, rhs: &Matrix4x4f32) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Subtracts scalar from all elements
impl std::ops::Sub<f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn sub(self, rhs: f32) -> Self::Output {
        let splat = f32x4::from_array([rhs, rhs, rhs, rhs]);
        Matrix4x4f32 {
            col0: self.col0 - splat,
            col1: self.col1 - splat,
            col2: self.col2 - splat,
            col3: self.col3 - splat,
        }
    }
}

impl std::ops::Sub<f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn sub(self, rhs: f32) -> Self::Output {
        (*self).sub(rhs)
    }
}

impl std::ops::Sub<&f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn sub(self, rhs: &f32) -> Self::Output {
        self.sub(*rhs)
    }
}

impl std::ops::Sub<&f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn sub(self, rhs: &f32) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   matrix multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Matrix multiplication (not element-wise)
impl std::ops::Mul<Matrix4x4f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn mul(self, rhs: Matrix4x4f32) -> Self::Output {
        Matrix4x4f32::new(
            &vec4f32(
                self.m00 * rhs.m00 + self.m01 * rhs.m10 + self.m02 * rhs.m20 + self.m03 * rhs.m30,
                self.m00 * rhs.m01 + self.m01 * rhs.m11 + self.m02 * rhs.m21 + self.m03 * rhs.m31,
                self.m00 * rhs.m02 + self.m01 * rhs.m12 + self.m02 * rhs.m22 + self.m03 * rhs.m32,
                self.m00 * rhs.m03 + self.m01 * rhs.m13 + self.m02 * rhs.m23 + self.m03 * rhs.m33,
            ),
            &vec4f32(
                self.m10 * rhs.m00 + self.m11 * rhs.m10 + self.m12 * rhs.m20 + self.m13 * rhs.m30,
                self.m10 * rhs.m01 + self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31,
                self.m10 * rhs.m02 + self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32,
                self.m10 * rhs.m03 + self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33,
            ),
            &vec4f32(
                self.m20 * rhs.m00 + self.m21 * rhs.m10 + self.m22 * rhs.m20 + self.m23 * rhs.m30,
                self.m20 * rhs.m01 + self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31,
                self.m20 * rhs.m02 + self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32,
                self.m20 * rhs.m03 + self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33,
            ),
            &vec4f32(
                self.m30 * rhs.m00 + self.m31 * rhs.m10 + self.m32 * rhs.m20 + self.m33 * rhs.m30,
                self.m30 * rhs.m01 + self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31,
                self.m30 * rhs.m02 + self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32,
                self.m30 * rhs.m03 + self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33,
            ),
        )
    }
}

impl std::ops::Mul<&Matrix4x4f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn mul(self, rhs: &Matrix4x4f32) -> Self::Output {
        self.mul(*rhs)
    }
}

impl std::ops::Mul<Matrix4x4f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn mul(self, rhs: Matrix4x4f32) -> Self::Output {
        (*self).mul(rhs)
    }
}

impl std::ops::Mul<&Matrix4x4f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn mul(self, rhs: &Matrix4x4f32) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Scalar multiplication
impl std::ops::Mul<f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn mul(self, rhs: f32) -> Self::Output {
        let splat = f32x4::from_array([rhs, rhs, rhs, rhs]);
        Matrix4x4f32 {
            col0: self.col0 * splat,
            col1: self.col1 * splat,
            col2: self.col2 * splat,
            col3: self.col3 * splat,
        }
    }
}

impl std::ops::Mul<f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn mul(self, rhs: f32) -> Self::Output {
        (*self).mul(rhs)
    }
}

impl std::ops::Mul<&f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn mul(self, rhs: &f32) -> Self::Output {
        self.mul(*rhs)
    }
}

impl std::ops::Mul<&f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn mul(self, rhs: &f32) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     matrix divisions     //////////////////
//////////////////////////////////////////////////////////

/// Element-wise division
impl std::ops::Div<Matrix4x4f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn div(self, rhs: Matrix4x4f32) -> Self::Output {
        Matrix4x4f32 {
            col0: self.col0 / rhs.col0,
            col1: self.col1 / rhs.col1,
            col2: self.col2 / rhs.col2,
            col3: self.col3 / rhs.col3,
        }
    }
}

impl std::ops::Div<&Matrix4x4f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn div(self, rhs: &Matrix4x4f32) -> Self::Output {
        self.div(*rhs)
    }
}

impl std::ops::Div<Matrix4x4f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn div(self, rhs: Matrix4x4f32) -> Self::Output {
        (*self).div(rhs)
    }
}

impl std::ops::Div<&Matrix4x4f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn div(self, rhs: &Matrix4x4f32) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Scalar division
impl std::ops::Div<f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn div(self, rhs: f32) -> Self::Output {
        let splat = f32x4::from_array([rhs, rhs, rhs, rhs]);
        Matrix4x4f32 {
            col0: self.col0 / splat,
            col1: self.col1 / splat,
            col2: self.col2 / splat,
            col3: self.col3 / splat,
        }
    }
}

impl std::ops::Div<f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn div(self, rhs: f32) -> Self::Output {
        (*self).div(rhs)
    }
}

impl std::ops::Div<&f32> for Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn div(self, rhs: &f32) -> Self::Output {
        self.div(*rhs)
    }
}

impl std::ops::Div<&f32> for &Matrix4x4f32 {
    type Output = Matrix4x4f32;

    fn div(self, rhs: &f32) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      matrix assign       //////////////////
//////////////////////////////////////////////////////////

impl std::ops::AddAssign<Matrix4x4f32> for Matrix4x4f32 {
    fn add_assign(&mut self, rhs: Matrix4x4f32) {
        self.col0 = self.col0 + rhs.col0;
        self.col1 = self.col1 + rhs.col1;
        self.col2 = self.col2 + rhs.col2;
        self.col3 = self.col3 + rhs.col3;
    }
}

impl std::ops::AddAssign<&Matrix4x4f32> for Matrix4x4f32 {
    fn add_assign(&mut self, rhs: &Matrix4x4f32) {
        self.add_assign(*rhs);
    }
}

impl std::ops::AddAssign<f32> for Matrix4x4f32 {
    fn add_assign(&mut self, rhs: f32) {
        let splat = f32x4::from_array([rhs, rhs, rhs, rhs]);
        self.col0 = self.col0 + splat;
        self.col1 = self.col1 + splat;
        self.col2 = self.col2 + splat;
        self.col3 = self.col3 + splat;
    }
}

impl std::ops::AddAssign<&f32> for Matrix4x4f32 {
    fn add_assign(&mut self, rhs: &f32) {
        self.add_assign(*rhs);
    }
}

impl std::ops::SubAssign<Matrix4x4f32> for Matrix4x4f32 {
    fn sub_assign(&mut self, rhs: Matrix4x4f32) {
        self.col0 = self.col0 - rhs.col0;
        self.col1 = self.col1 - rhs.col1;
        self.col2 = self.col2 - rhs.col2;
        self.col3 = self.col3 - rhs.col3;
    }
}

impl std::ops::SubAssign<&Matrix4x4f32> for Matrix4x4f32 {
    fn sub_assign(&mut self, rhs: &Matrix4x4f32) {
        self.sub_assign(*rhs);
    }
}

impl std::ops::SubAssign<f32> for Matrix4x4f32 {
    fn sub_assign(&mut self, rhs: f32) {
        let splat = f32x4::from_array([rhs, rhs, rhs, rhs]);
        self.col0 = self.col0 - splat;
        self.col1 = self.col1 - splat;
        self.col2 = self.col2 - splat;
        self.col3 = self.col3 - splat;
    }
}

impl std::ops::SubAssign<&f32> for Matrix4x4f32 {
    fn sub_assign(&mut self, rhs: &f32) {
        self.sub_assign(*rhs);
    }
}

impl std::ops::MulAssign<Matrix4x4f32> for Matrix4x4f32 {
    fn mul_assign(&mut self, rhs: Matrix4x4f32) {
        *self = *self * rhs;
    }
}

impl std::ops::MulAssign<&Matrix4x4f32> for Matrix4x4f32 {
    fn mul_assign(&mut self, rhs: &Matrix4x4f32) {
        self.mul_assign(*rhs);
    }
}

impl std::ops::MulAssign<f32> for Matrix4x4f32 {
    fn mul_assign(&mut self, rhs: f32) {
        let splat = f32x4::from_array([rhs, rhs, rhs, rhs]);
        self.col0 = self.col0 * splat;
        self.col1 = self.col1 * splat;
        self.col2 = self.col2 * splat;
        self.col3 = self.col3 * splat;
    }
}

impl std::ops::MulAssign<&f32> for Matrix4x4f32 {
    fn mul_assign(&mut self, rhs: &f32) {
        self.mul_assign(*rhs);
    }
}

impl std::ops::DivAssign<Matrix4x4f32> for Matrix4x4f32 {
    fn div_assign(&mut self, rhs: Matrix4x4f32) {
        self.col0 = self.col0 / rhs.col0;
        self.col1 = self.col1 / rhs.col1;
        self.col2 = self.col2 / rhs.col2;
        self.col3 = self.col3 / rhs.col3;
    }
}

impl std::ops::DivAssign<&Matrix4x4f32> for Matrix4x4f32 {
    fn div_assign(&mut self, rhs: &Matrix4x4f32) {
        self.div_assign(*rhs);
    }
}

impl std::ops::DivAssign<f32> for Matrix4x4f32 {
    fn div_assign(&mut self, rhs: f32) {
        let splat = f32x4::from_array([rhs, rhs, rhs, rhs]);
        self.col0 = self.col0 / splat;
        self.col1 = self.col1 / splat;
        self.col2 = self.col2 / splat;
        self.col3 = self.col3 / splat;
    }
}

impl std::ops::DivAssign<&f32> for Matrix4x4f32 {
    fn div_assign(&mut self, rhs: &f32) {
        self.div_assign(*rhs);
    }
}

//////////////////////////////////////////////////////////
//////////////   matrix-vector operations ////////////////
//////////////////////////////////////////////////////////

/// Matrix-vector multiplication: M * v
impl std::ops::Mul<Vector4f32> for Matrix4x4f32 {
    type Output = Vector4f32;

    fn mul(self, rhs: Vector4f32) -> Self::Output {
        vec4f32(
            self.m00 * rhs.x + self.m01 * rhs.y + self.m02 * rhs.z + self.m03 * rhs.w,
            self.m10 * rhs.x + self.m11 * rhs.y + self.m12 * rhs.z + self.m13 * rhs.w,
            self.m20 * rhs.x + self.m21 * rhs.y + self.m22 * rhs.z + self.m23 * rhs.w,
            self.m30 * rhs.x + self.m31 * rhs.y + self.m32 * rhs.z + self.m33 * rhs.w,
        )
    }
}

impl std::ops::Mul<&Vector4f32> for Matrix4x4f32 {
    type Output = Vector4f32;

    fn mul(self, rhs: &Vector4f32) -> Self::Output {
        self.mul(*rhs)
    }
}

impl std::ops::Mul<Vector4f32> for &Matrix4x4f32 {
    type Output = Vector4f32;

    fn mul(self, rhs: Vector4f32) -> Self::Output {
        (*self).mul(rhs)
    }
}

impl std::ops::Mul<&Vector4f32> for &Matrix4x4f32 {
    type Output = Vector4f32;

    fn mul(self, rhs: &Vector4f32) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
///////////////     matrix indices     ///////////////////
//////////////////////////////////////////////////////////
impl std::ops::Index<(usize, usize)> for Matrix4x4f32 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        match index {
            (0, 0) => &self.m00,
            (0, 1) => &self.m01,
            (0, 2) => &self.m02,
            (0, 3) => &self.m03,
            (1, 0) => &self.m10,
            (1, 1) => &self.m11,
            (1, 2) => &self.m12,
            (1, 3) => &self.m13,
            (2, 0) => &self.m20,
            (2, 1) => &self.m21,
            (2, 2) => &self.m22,
            (2, 3) => &self.m23,
            (3, 0) => &self.m30,
            (3, 1) => &self.m31,
            (3, 2) => &self.m32,
            (3, 3) => &self.m33,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix4x4f32 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        match index {
            (0, 0) => &mut self.m00,
            (0, 1) => &mut self.m01,
            (0, 2) => &mut self.m02,
            (0, 3) => &mut self.m03,
            (1, 0) => &mut self.m10,
            (1, 1) => &mut self.m11,
            (1, 2) => &mut self.m12,
            (1, 3) => &mut self.m13,
            (2, 0) => &mut self.m20,
            (2, 1) => &mut self.m21,
            (2, 2) => &mut self.m22,
            (2, 3) => &mut self.m23,
            (3, 0) => &mut self.m30,
            (3, 1) => &mut self.m31,
            (3, 2) => &mut self.m32,
            (3, 3) => &mut self.m33,
            _ => panic!("Index out of bounds"),
        }
    }
}

//////////////////////////////////////////////////////////
///////////////     matrix tests      ////////////////////
//////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization() {
        let m1 = mat4x4f32(
            &vec4f32(1., 2., 3., 4.),
            &vec4f32(5., 6., 7., 8.),
            &vec4f32(9., 10., 11., 12.),
            &vec4f32(13., 14., 15., 16.),
        );
        let m2 = Matrix4x4f32::new(
            &vec4f32(1., 2., 3., 4.),
            &vec4f32(5., 6., 7., 8.),
            &vec4f32(9., 10., 11., 12.),
            &vec4f32(13., 14., 15., 16.),
        );
        assert_eq!(m1, m2);

        let identity = Matrix4x4f32::IDENTITY;
        assert_eq!(identity.m00, 1.);
        assert_eq!(identity.m11, 1.);
        assert_eq!(identity.m22, 1.);
        assert_eq!(identity.m33, 1.);
        assert_eq!(identity.m01, 0.);
        assert_eq!(identity.m02, 0.);
    }

    #[test]
    fn operators() {
        let m1 = mat4x4f32(
            &vec4f32(1., 2., 3., 4.),
            &vec4f32(5., 6., 7., 8.),
            &vec4f32(9., 10., 11., 12.),
            &vec4f32(13., 14., 15., 16.),
        );
        let m2 = mat4x4f32(
            &vec4f32(16., 15., 14., 13.),
            &vec4f32(12., 11., 10., 9.),
            &vec4f32(8., 7., 6., 5.),
            &vec4f32(4., 3., 2., 1.),
        );

        let add = m1 + m2;
        assert_eq!(
            add,
            mat4x4f32(
                &vec4f32(17., 17., 17., 17.),
                &vec4f32(17., 17., 17., 17.),
                &vec4f32(17., 17., 17., 17.),
                &vec4f32(17., 17., 17., 17.)
            )
        );

        let scaled = m1 * 2.;
        assert_eq!(
            scaled,
            mat4x4f32(
                &vec4f32(2., 4., 6., 8.),
                &vec4f32(10., 12., 14., 16.),
                &vec4f32(18., 20., 22., 24.),
                &vec4f32(26., 28., 30., 32.)
            )
        );
    }

    #[test]
    fn matrix_multiplication() {
        let m = mat4x4f32(
            &vec4f32(1., 2., 3., 4.),
            &vec4f32(5., 6., 7., 8.),
            &vec4f32(9., 10., 11., 12.),
            &vec4f32(13., 14., 15., 16.),
        );

        let identity = Matrix4x4f32::IDENTITY;
        assert_eq!(m * identity, m);
        assert_eq!(identity * m, m);

        let scale = mat4x4f32(
            &vec4f32(2., 0., 0., 0.),
            &vec4f32(0., 2., 0., 0.),
            &vec4f32(0., 0., 2., 0.),
            &vec4f32(0., 0., 0., 2.),
        );
        assert_eq!(m * scale, m * 2.);
    }

    #[test]
    fn determinant() {
        let identity = Matrix4x4f32::IDENTITY;
        assert_eq!(identity.determinant(), 1.);

        let singular = mat4x4f32(
            &vec4f32(1., 2., 3., 4.),
            &vec4f32(2., 4., 6., 8.),
            &vec4f32(3., 6., 9., 12.),
            &vec4f32(4., 8., 12., 16.),
        );
        assert_eq!(singular.determinant(), 0.);
    }

    #[test]
    fn transpose() {
        let m = mat4x4f32(
            &vec4f32(1., 2., 3., 4.),
            &vec4f32(5., 6., 7., 8.),
            &vec4f32(9., 10., 11., 12.),
            &vec4f32(13., 14., 15., 16.),
        );
        let t = m.transpose();
        assert_eq!(
            t,
            mat4x4f32(
                &vec4f32(1., 5., 9., 13.),
                &vec4f32(2., 6., 10., 14.),
                &vec4f32(3., 7., 11., 15.),
                &vec4f32(4., 8., 12., 16.)
            )
        );
    }

    #[test]
    fn inverse() {
        let m = Matrix4x4f32::IDENTITY;
        let inv = m.inverse().unwrap();
        assert_eq!(inv, Matrix4x4f32::IDENTITY);

        let singular = mat4x4f32(
            &vec4f32(1., 2., 3., 4.),
            &vec4f32(2., 4., 6., 8.),
            &vec4f32(3., 6., 9., 12.),
            &vec4f32(4., 8., 12., 16.),
        );
        assert!(singular.inverse().is_none());
    }

    #[test]
    fn rotation() {
        use std::f32::consts::PI;

        let rot_x = Matrix4x4f32::rotation_x(PI / 2.);
        let rot_y = Matrix4x4f32::rotation_y(PI / 2.);
        let rot_z = Matrix4x4f32::rotation_z(PI / 2.);

        let eps = 1e-5;

        assert!((rot_x.m00 - 1.).abs() < eps);
        assert!((rot_y.m11 - 1.).abs() < eps);
        assert!((rot_z.m22 - 1.).abs() < eps);
    }

    #[test]
    fn trace() {
        let m = mat4x4f32(
            &vec4f32(1., 0., 0., 0.),
            &vec4f32(0., 2., 0., 0.),
            &vec4f32(0., 0., 3., 0.),
            &vec4f32(0., 0., 0., 4.),
        );
        assert_eq!(m.trace(), 10.);
    }

    #[test]
    fn deref() {
        let mut m = Matrix4x4f32::IDENTITY;
        m.m03 = 5.;
        assert_eq!(m.m03, 5.);
    }

    #[test]
    fn format() {
        let m = Matrix4x4f32::IDENTITY;
        let display = format!("{}", m);
        assert_eq!(
            display,
            "[1, 0, 0, 0],[0, 1, 0, 0],[0, 0, 1, 0],[0, 0, 0, 1]"
        );
    }

    #[test]
    fn row_col_access() {
        let mut m = Matrix4x4f32::ZEROS;

        m.set_row(0, &vec4f32(1., 2., 3., 4.));
        m.set_col(0, &vec4f32(1., 5., 9., 13.));

        assert_eq!(m.get_row(0), vec4f32(1., 2., 3., 4.));
        assert_eq!(m.get_col(0), vec4f32(1., 5., 9., 13.));
    }

    #[test]
    fn matrix_vector_multiplication() {
        let m = Matrix4x4f32::IDENTITY;
        let v = vec4f32(1., 2., 3., 1.);

        assert_eq!(m * v, v);

        use std::f32::consts::PI;
        let rot = Matrix4x4f32::rotation_z(PI / 4.);
        let v2 = vec4f32(1., 0., 0., 1.);
        let rotated = rot * v2;

        let eps = 1e-5;
        assert!((rotated.length() - v2.length()).abs() < eps);
    }

    /// Tests indices access
    #[test]
    fn indices() {
        let mut m = mat4x4f32(
            &vec4f32(1., 2., 3., 4.),
            &vec4f32(5., 6., 7., 8.),
            &vec4f32(9., 10., 11., 12.),
            &vec4f32(13., 14., 15., 16.),
        );
        assert_eq!(m[(0, 0)], 1.);
        assert_eq!(m[(0, 1)], 2.);
        assert_eq!(m[(0, 2)], 3.);
        assert_eq!(m[(0, 3)], 4.);
        assert_eq!(m[(1, 0)], 5.);
        assert_eq!(m[(1, 1)], 6.);
        assert_eq!(m[(1, 2)], 7.);
        assert_eq!(m[(1, 3)], 8.);
        assert_eq!(m[(2, 0)], 9.);
        assert_eq!(m[(2, 1)], 10.);
        assert_eq!(m[(2, 2)], 11.);
        assert_eq!(m[(2, 3)], 12.);
        assert_eq!(m[(3, 0)], 13.);
        assert_eq!(m[(3, 1)], 14.);
        assert_eq!(m[(3, 2)], 15.);
        assert_eq!(m[(3, 3)], 16.);

        m[(0, 0)] = 2.;
        assert_eq!(m[(3, 3)], 16.);
        assert_eq!(m[(0, 0)], 2.);
        m[(0, 1)] = 3.;
        assert_eq!(m[(3, 3)], 16.);
        assert_eq!(m[(0, 1)], 3.);
        m[(0, 2)] = 4.;
        assert_eq!(m[(3, 3)], 16.);
        assert_eq!(m[(0, 2)], 4.);
        m[(0, 3)] = 5.;
        assert_eq!(m[(3, 3)], 16.);
        assert_eq!(m[(0, 3)], 5.);
        m[(1, 0)] = 6.;
        assert_eq!(m[(3, 3)], 16.);
        assert_eq!(m[(1, 0)], 6.);
        m[(1, 1)] = 7.;
        assert_eq!(m[(3, 3)], 16.);
        assert_eq!(m[(1, 1)], 7.);
        m[(1, 2)] = 8.;
        assert_eq!(m[(3, 3)], 16.);
        assert_eq!(m[(1, 2)], 8.);
        m[(1, 3)] = 9.;
        assert_eq!(m[(3, 3)], 16.);
        assert_eq!(m[(1, 3)], 9.);
        m[(2, 0)] = 10.;
        assert_eq!(m[(3, 3)], 16.);
        assert_eq!(m[(2, 0)], 10.);
        m[(2, 1)] = 11.;
        assert_eq!(m[(3, 3)], 16.);
        assert_eq!(m[(2, 1)], 11.);
        m[(2, 2)] = 12.;
        assert_eq!(m[(3, 3)], 16.);
        assert_eq!(m[(2, 2)], 12.);
        m[(2, 3)] = 13.;
        assert_eq!(m[(3, 3)], 16.);
        assert_eq!(m[(2, 3)], 13.);
        m[(3, 0)] = 14.;
        assert_eq!(m[(3, 3)], 16.);
        assert_eq!(m[(3, 0)], 14.);
        m[(3, 1)] = 15.;
        assert_eq!(m[(3, 3)], 16.);
        assert_eq!(m[(3, 1)], 15.);
        m[(3, 2)] = 16.;
        assert_eq!(m[(3, 3)], 16.);
        assert_eq!(m[(3, 2)], 16.);
        m[(3, 3)] = 17.;
        assert_eq!(m[(3, 3)], 17.);
    }
}
