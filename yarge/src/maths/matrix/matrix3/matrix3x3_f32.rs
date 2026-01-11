use std::simd::prelude::*;

use crate::maths::Vector3f32;

/// A structure to represent a 3x3 f32 matrix stored in column-major order
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Matrix3x3f32 {
    col0: f32x4, // First column [m00, m10, m20, _]
    col1: f32x4, // Second column [m01, m11, m21, _]
    col2: f32x4, // Third column [m02, m12, m22, _]
}

/// A structure to allow accessing matrix elements
mod private {
    #[repr(C)]
    pub struct MatrixElements {
        pub m00: f32,
        pub m10: f32,
        pub m20: f32,
        _pad0: f32,
        pub m01: f32,
        pub m11: f32,
        pub m21: f32,
        _pad1: f32,
        pub m02: f32,
        pub m12: f32,
        pub m22: f32,
        _pad2: f32,
    }
}

/// Implements `Deref` to allow accessing matrix elements
impl std::ops::Deref for Matrix3x3f32 {
    type Target = private::MatrixElements;

    fn deref(&self) -> &Self::Target {
        let value: *const Matrix3x3f32 = self;
        unsafe { &*(value as *const private::MatrixElements) }
    }
}

/// Implements `DerefMut` to allow modifying matrix elements
impl std::ops::DerefMut for Matrix3x3f32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value: *mut Matrix3x3f32 = self;
        unsafe { &mut *(value as *mut private::MatrixElements) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Matrix3x3f32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix3x3f32")
            .field("m00", &self.m00)
            .field("m01", &self.m01)
            .field("m02", &self.m02)
            .field("m10", &self.m10)
            .field("m11", &self.m11)
            .field("m12", &self.m12)
            .field("m20", &self.m20)
            .field("m21", &self.m21)
            .field("m22", &self.m22)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Matrix3x3f32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}, {}],[{}, {}, {}],[{}, {}, {}]",
            self.m00,
            self.m01,
            self.m02,
            self.m10,
            self.m11,
            self.m12,
            self.m20,
            self.m21,
            self.m22
        )
    }
}

/// Sets a 3x3 matrix to identity as default
impl Default for Matrix3x3f32 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// Creates a 3x3 f32 matrix
pub fn mat3x3f32(
    m00: f32,
    m01: f32,
    m02: f32,
    m10: f32,
    m11: f32,
    m12: f32,
    m20: f32,
    m21: f32,
    m22: f32,
) -> Matrix3x3f32 {
    Matrix3x3f32::new(m00, m01, m02, m10, m11, m12, m20, m21, m22)
}

impl Matrix3x3f32 {
    //////////////////////////////////////////////////////////
    /////////////      matrix creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new matrix given its elements in row-major order
    pub const fn new(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
    ) -> Self {
        Self {
            col0: f32x4::from_array([m00, m10, m20, 0.]),
            col1: f32x4::from_array([m01, m11, m21, 0.]),
            col2: f32x4::from_array([m02, m12, m22, 0.]),
        }
    }

    /// Creates a new matrix with all elements set to `value`
    const fn splat(value: f32) -> Self {
        Self::new(
            value, value, value, value, value, value, value, value, value,
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
    pub const IDENTITY: Self = Self::new(1., 0., 0., 0., 1., 0., 0., 0., 1.);

    /// Creates a diagonal matrix
    pub const fn diagonal(d0: f32, d1: f32, d2: f32) -> Self {
        Self::new(d0, 0., 0., 0., d1, 0., 0., 0., d2)
    }

    //////////////////////////////////////////////////////////
    /////////////     matrix operations      /////////////////
    //////////////////////////////////////////////////////////

    /// Returns the determinant of the matrix
    pub fn determinant(&self) -> f32 {
        self.m00 * (self.m11 * self.m22 - self.m12 * self.m21)
            - self.m01 * (self.m10 * self.m22 - self.m12 * self.m20)
            + self.m02 * (self.m10 * self.m21 - self.m11 * self.m20)
    }

    /// Returns the trace of the matrix (sum of diagonal elements)
    pub fn trace(&self) -> f32 {
        self.m00 + self.m11 + self.m22
    }

    /// Returns the transpose of the matrix
    pub fn transpose(&self) -> Self {
        Self::new(
            self.m00, self.m10, self.m20, self.m01, self.m11, self.m21, self.m02, self.m12,
            self.m22,
        )
    }

    /// Returns the inverse of the matrix, or None if not invertible
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < f32::EPSILON {
            return None;
        }

        let inv_det = 1.0 / det;

        // Calculate cofactor matrix
        let c00 = self.m11 * self.m22 - self.m12 * self.m21;
        let c01 = -(self.m10 * self.m22 - self.m12 * self.m20);
        let c02 = self.m10 * self.m21 - self.m11 * self.m20;

        let c10 = -(self.m01 * self.m22 - self.m02 * self.m21);
        let c11 = self.m00 * self.m22 - self.m02 * self.m20;
        let c12 = -(self.m00 * self.m21 - self.m01 * self.m20);

        let c20 = self.m01 * self.m12 - self.m02 * self.m11;
        let c21 = -(self.m00 * self.m12 - self.m02 * self.m10);
        let c22 = self.m00 * self.m11 - self.m01 * self.m10;

        // Transpose of cofactor matrix divided by determinant
        Some(Self::new(
            c00 * inv_det,
            c10 * inv_det,
            c20 * inv_det,
            c01 * inv_det,
            c11 * inv_det,
            c21 * inv_det,
            c02 * inv_det,
            c12 * inv_det,
            c22 * inv_det,
        ))
    }

    /// Creates a rotation matrix around the X axis
    pub fn rotation_x(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(1., 0., 0., 0., cos, -sin, 0., sin, cos)
    }

    /// Creates a rotation matrix around the Y axis
    pub fn rotation_y(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(cos, 0., sin, 0., 1., 0., -sin, 0., cos)
    }

    /// Creates a rotation matrix around the Z axis
    pub fn rotation_z(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(cos, -sin, 0., sin, cos, 0., 0., 0., 1.)
    }

    /// Creates a scaling matrix
    pub const fn scale(sx: f32, sy: f32, sz: f32) -> Self {
        Self::diagonal(sx, sy, sz)
    }

    //////////////////////////////////////////////////////////
    /////////////   row and column access    /////////////////
    //////////////////////////////////////////////////////////

    /// Returns the specified row as a Vector3f32
    pub fn get_row(&self, row: usize) -> Vector3f32 {
        assert!(row < 3, "Row index out of bounds");
        match row {
            0 => Vector3f32::new(self.m00, self.m01, self.m02),
            1 => Vector3f32::new(self.m10, self.m11, self.m12),
            2 => Vector3f32::new(self.m20, self.m21, self.m22),
            _ => unreachable!(),
        }
    }

    /// Returns the specified column as a Vector3f32
    pub fn get_col(&self, col: usize) -> Vector3f32 {
        assert!(col < 3, "Column index out of bounds");
        match col {
            0 => Vector3f32::new(self.m00, self.m10, self.m20),
            1 => Vector3f32::new(self.m01, self.m11, self.m21),
            2 => Vector3f32::new(self.m02, self.m12, self.m22),
            _ => unreachable!(),
        }
    }

    /// Sets the specified row from a Vector3f32
    pub fn set_row(&mut self, row: usize, v: Vector3f32) {
        assert!(row < 3, "Row index out of bounds");
        match row {
            0 => {
                self.m00 = v.x;
                self.m01 = v.y;
                self.m02 = v.z;
            }
            1 => {
                self.m10 = v.x;
                self.m11 = v.y;
                self.m12 = v.z;
            }
            2 => {
                self.m20 = v.x;
                self.m21 = v.y;
                self.m22 = v.z;
            }
            _ => unreachable!(),
        }
    }

    /// Sets the specified column from a Vector3f32
    pub fn set_col(&mut self, col: usize, v: Vector3f32) {
        assert!(col < 3, "Column index out of bounds");
        match col {
            0 => {
                self.m00 = v.x;
                self.m10 = v.y;
                self.m20 = v.z;
            }
            1 => {
                self.m01 = v.x;
                self.m11 = v.y;
                self.m21 = v.z;
            }
            2 => {
                self.m02 = v.x;
                self.m12 = v.y;
                self.m22 = v.z;
            }
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Matrix3x3f32 {
    fn eq(&self, other: &Self) -> bool {
        self.m00 == other.m00
            && self.m01 == other.m01
            && self.m02 == other.m02
            && self.m10 == other.m10
            && self.m11 == other.m11
            && self.m12 == other.m12
            && self.m20 == other.m20
            && self.m21 == other.m21
            && self.m22 == other.m22
    }
}

//////////////////////////////////////////////////////////
//////////////     matrix additions     //////////////////
//////////////////////////////////////////////////////////

/// Element-wise addition
impl std::ops::Add<Matrix3x3f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn add(self, rhs: Matrix3x3f32) -> Self::Output {
        Matrix3x3f32 {
            col0: self.col0 + rhs.col0,
            col1: self.col1 + rhs.col1,
            col2: self.col2 + rhs.col2,
        }
    }
}

impl std::ops::Add<&Matrix3x3f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn add(self, rhs: &Matrix3x3f32) -> Self::Output {
        self.add(*rhs)
    }
}

impl std::ops::Add<Matrix3x3f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn add(self, rhs: Matrix3x3f32) -> Self::Output {
        (*self).add(rhs)
    }
}

impl std::ops::Add<&Matrix3x3f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn add(self, rhs: &Matrix3x3f32) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds scalar to all elements
impl std::ops::Add<f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn add(self, rhs: f32) -> Self::Output {
        let splat = f32x4::from_array([rhs, rhs, rhs, 0.]);
        Matrix3x3f32 {
            col0: self.col0 + splat,
            col1: self.col1 + splat,
            col2: self.col2 + splat,
        }
    }
}

impl std::ops::Add<f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn add(self, rhs: f32) -> Self::Output {
        (*self).add(rhs)
    }
}

impl std::ops::Add<&f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn add(self, rhs: &f32) -> Self::Output {
        self.add(*rhs)
    }
}

impl std::ops::Add<&f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn add(self, rhs: &f32) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   matrix subtractions    //////////////////
//////////////////////////////////////////////////////////

/// Element-wise negation
impl std::ops::Neg for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn neg(self) -> Self::Output {
        Matrix3x3f32 {
            col0: -self.col0,
            col1: -self.col1,
            col2: -self.col2,
        }
    }
}

/// Element-wise subtraction
impl std::ops::Sub<Matrix3x3f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn sub(self, rhs: Matrix3x3f32) -> Self::Output {
        Matrix3x3f32 {
            col0: self.col0 - rhs.col0,
            col1: self.col1 - rhs.col1,
            col2: self.col2 - rhs.col2,
        }
    }
}

impl std::ops::Sub<&Matrix3x3f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn sub(self, rhs: &Matrix3x3f32) -> Self::Output {
        self.sub(*rhs)
    }
}

impl std::ops::Sub<Matrix3x3f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn sub(self, rhs: Matrix3x3f32) -> Self::Output {
        (*self).sub(rhs)
    }
}

impl std::ops::Sub<&Matrix3x3f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn sub(self, rhs: &Matrix3x3f32) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Subtracts scalar from all elements
impl std::ops::Sub<f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn sub(self, rhs: f32) -> Self::Output {
        let splat = f32x4::from_array([rhs, rhs, rhs, 0.]);
        Matrix3x3f32 {
            col0: self.col0 - splat,
            col1: self.col1 - splat,
            col2: self.col2 - splat,
        }
    }
}

impl std::ops::Sub<f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn sub(self, rhs: f32) -> Self::Output {
        (*self).sub(rhs)
    }
}

impl std::ops::Sub<&f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn sub(self, rhs: &f32) -> Self::Output {
        self.sub(*rhs)
    }
}

impl std::ops::Sub<&f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn sub(self, rhs: &f32) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   matrix multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Matrix multiplication (not element-wise)
impl std::ops::Mul<Matrix3x3f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn mul(self, rhs: Matrix3x3f32) -> Self::Output {
        Matrix3x3f32::new(
            self.m00 * rhs.m00 + self.m01 * rhs.m10 + self.m02 * rhs.m20,
            self.m00 * rhs.m01 + self.m01 * rhs.m11 + self.m02 * rhs.m21,
            self.m00 * rhs.m02 + self.m01 * rhs.m12 + self.m02 * rhs.m22,
            self.m10 * rhs.m00 + self.m11 * rhs.m10 + self.m12 * rhs.m20,
            self.m10 * rhs.m01 + self.m11 * rhs.m11 + self.m12 * rhs.m21,
            self.m10 * rhs.m02 + self.m11 * rhs.m12 + self.m12 * rhs.m22,
            self.m20 * rhs.m00 + self.m21 * rhs.m10 + self.m22 * rhs.m20,
            self.m20 * rhs.m01 + self.m21 * rhs.m11 + self.m22 * rhs.m21,
            self.m20 * rhs.m02 + self.m21 * rhs.m12 + self.m22 * rhs.m22,
        )
    }
}

impl std::ops::Mul<&Matrix3x3f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn mul(self, rhs: &Matrix3x3f32) -> Self::Output {
        self.mul(*rhs)
    }
}

impl std::ops::Mul<Matrix3x3f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn mul(self, rhs: Matrix3x3f32) -> Self::Output {
        (*self).mul(rhs)
    }
}

impl std::ops::Mul<&Matrix3x3f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn mul(self, rhs: &Matrix3x3f32) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Scalar multiplication
impl std::ops::Mul<f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn mul(self, rhs: f32) -> Self::Output {
        let splat = f32x4::from_array([rhs, rhs, rhs, 1.]);
        Matrix3x3f32 {
            col0: self.col0 * splat,
            col1: self.col1 * splat,
            col2: self.col2 * splat,
        }
    }
}

impl std::ops::Mul<f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn mul(self, rhs: f32) -> Self::Output {
        (*self).mul(rhs)
    }
}

impl std::ops::Mul<&f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn mul(self, rhs: &f32) -> Self::Output {
        self.mul(*rhs)
    }
}

impl std::ops::Mul<&f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn mul(self, rhs: &f32) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     matrix divisions     //////////////////
//////////////////////////////////////////////////////////

/// Element-wise division
impl std::ops::Div<Matrix3x3f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn div(self, rhs: Matrix3x3f32) -> Self::Output {
        Matrix3x3f32 {
            col0: self.col0 / rhs.col0,
            col1: self.col1 / rhs.col1,
            col2: self.col2 / rhs.col2,
        }
    }
}

impl std::ops::Div<&Matrix3x3f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn div(self, rhs: &Matrix3x3f32) -> Self::Output {
        self.div(*rhs)
    }
}

impl std::ops::Div<Matrix3x3f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn div(self, rhs: Matrix3x3f32) -> Self::Output {
        (*self).div(rhs)
    }
}

impl std::ops::Div<&Matrix3x3f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn div(self, rhs: &Matrix3x3f32) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Scalar division
impl std::ops::Div<f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn div(self, rhs: f32) -> Self::Output {
        let splat = f32x4::from_array([rhs, rhs, rhs, 1.]);
        Matrix3x3f32 {
            col0: self.col0 / splat,
            col1: self.col1 / splat,
            col2: self.col2 / splat,
        }
    }
}

impl std::ops::Div<f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn div(self, rhs: f32) -> Self::Output {
        (*self).div(rhs)
    }
}

impl std::ops::Div<&f32> for Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn div(self, rhs: &f32) -> Self::Output {
        self.div(*rhs)
    }
}

impl std::ops::Div<&f32> for &Matrix3x3f32 {
    type Output = Matrix3x3f32;

    fn div(self, rhs: &f32) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      matrix assign       //////////////////
//////////////////////////////////////////////////////////

impl std::ops::AddAssign<Matrix3x3f32> for Matrix3x3f32 {
    fn add_assign(&mut self, rhs: Matrix3x3f32) {
        self.col0 = self.col0 + rhs.col0;
        self.col1 = self.col1 + rhs.col1;
        self.col2 = self.col2 + rhs.col2;
    }
}

impl std::ops::AddAssign<&Matrix3x3f32> for Matrix3x3f32 {
    fn add_assign(&mut self, rhs: &Matrix3x3f32) {
        self.add_assign(*rhs);
    }
}

impl std::ops::AddAssign<f32> for Matrix3x3f32 {
    fn add_assign(&mut self, rhs: f32) {
        let splat = f32x4::from_array([rhs, rhs, rhs, 0.]);
        self.col0 = self.col0 + splat;
        self.col1 = self.col1 + splat;
        self.col2 = self.col2 + splat;
    }
}

impl std::ops::AddAssign<&f32> for Matrix3x3f32 {
    fn add_assign(&mut self, rhs: &f32) {
        self.add_assign(*rhs);
    }
}

impl std::ops::SubAssign<Matrix3x3f32> for Matrix3x3f32 {
    fn sub_assign(&mut self, rhs: Matrix3x3f32) {
        self.col0 = self.col0 - rhs.col0;
        self.col1 = self.col1 - rhs.col1;
        self.col2 = self.col2 - rhs.col2;
    }
}

impl std::ops::SubAssign<&Matrix3x3f32> for Matrix3x3f32 {
    fn sub_assign(&mut self, rhs: &Matrix3x3f32) {
        self.sub_assign(*rhs);
    }
}

impl std::ops::SubAssign<f32> for Matrix3x3f32 {
    fn sub_assign(&mut self, rhs: f32) {
        let splat = f32x4::from_array([rhs, rhs, rhs, 0.]);
        self.col0 = self.col0 - splat;
        self.col1 = self.col1 - splat;
        self.col2 = self.col2 - splat;
    }
}

impl std::ops::SubAssign<&f32> for Matrix3x3f32 {
    fn sub_assign(&mut self, rhs: &f32) {
        self.sub_assign(*rhs);
    }
}

impl std::ops::MulAssign<Matrix3x3f32> for Matrix3x3f32 {
    fn mul_assign(&mut self, rhs: Matrix3x3f32) {
        *self = *self * rhs;
    }
}

impl std::ops::MulAssign<&Matrix3x3f32> for Matrix3x3f32 {
    fn mul_assign(&mut self, rhs: &Matrix3x3f32) {
        self.mul_assign(*rhs);
    }
}

impl std::ops::MulAssign<f32> for Matrix3x3f32 {
    fn mul_assign(&mut self, rhs: f32) {
        let splat = f32x4::from_array([rhs, rhs, rhs, 1.]);
        self.col0 = self.col0 * splat;
        self.col1 = self.col1 * splat;
        self.col2 = self.col2 * splat;
    }
}

impl std::ops::MulAssign<&f32> for Matrix3x3f32 {
    fn mul_assign(&mut self, rhs: &f32) {
        self.mul_assign(*rhs);
    }
}

impl std::ops::DivAssign<Matrix3x3f32> for Matrix3x3f32 {
    fn div_assign(&mut self, rhs: Matrix3x3f32) {
        self.col0 = self.col0 / rhs.col0;
        self.col1 = self.col1 / rhs.col1;
        self.col2 = self.col2 / rhs.col2;
    }
}

impl std::ops::DivAssign<&Matrix3x3f32> for Matrix3x3f32 {
    fn div_assign(&mut self, rhs: &Matrix3x3f32) {
        self.div_assign(*rhs);
    }
}

impl std::ops::DivAssign<f32> for Matrix3x3f32 {
    fn div_assign(&mut self, rhs: f32) {
        let splat = f32x4::from_array([rhs, rhs, rhs, 1.]);
        self.col0 = self.col0 / splat;
        self.col1 = self.col1 / splat;
        self.col2 = self.col2 / splat;
    }
}

impl std::ops::DivAssign<&f32> for Matrix3x3f32 {
    fn div_assign(&mut self, rhs: &f32) {
        self.div_assign(*rhs);
    }
}

//////////////////////////////////////////////////////////
//////////////   matrix-vector operations ////////////////
//////////////////////////////////////////////////////////

/// Matrix-vector multiplication: M * v
impl std::ops::Mul<Vector3f32> for Matrix3x3f32 {
    type Output = Vector3f32;

    fn mul(self, rhs: Vector3f32) -> Self::Output {
        Vector3f32::new(
            self.m00 * rhs.x + self.m01 * rhs.y + self.m02 * rhs.z,
            self.m10 * rhs.x + self.m11 * rhs.y + self.m12 * rhs.z,
            self.m20 * rhs.x + self.m21 * rhs.y + self.m22 * rhs.z,
        )
    }
}

impl std::ops::Mul<&Vector3f32> for Matrix3x3f32 {
    type Output = Vector3f32;

    fn mul(self, rhs: &Vector3f32) -> Self::Output {
        self.mul(*rhs)
    }
}

impl std::ops::Mul<Vector3f32> for &Matrix3x3f32 {
    type Output = Vector3f32;

    fn mul(self, rhs: Vector3f32) -> Self::Output {
        (*self).mul(rhs)
    }
}

impl std::ops::Mul<&Vector3f32> for &Matrix3x3f32 {
    type Output = Vector3f32;

    fn mul(self, rhs: &Vector3f32) -> Self::Output {
        (*self).mul(*rhs)
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
        let m1 = mat3x3f32(1., 2., 3., 4., 5., 6., 7., 8., 9.);
        let m2 = Matrix3x3f32::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);
        assert_eq!(m1, m2);

        let identity = Matrix3x3f32::IDENTITY;
        assert_eq!(identity.m00, 1.);
        assert_eq!(identity.m11, 1.);
        assert_eq!(identity.m22, 1.);
        assert_eq!(identity.m01, 0.);
        assert_eq!(identity.m02, 0.);
    }

    #[test]
    fn operators() {
        let m1 = mat3x3f32(1., 2., 3., 4., 5., 6., 7., 8., 9.);
        let m2 = mat3x3f32(9., 8., 7., 6., 5., 4., 3., 2., 1.);

        let add = m1 + m2;
        assert_eq!(add, mat3x3f32(10., 10., 10., 10., 10., 10., 10., 10., 10.));

        let scaled = m1 * 2.;
        assert_eq!(scaled, mat3x3f32(2., 4., 6., 8., 10., 12., 14., 16., 18.));
    }

    #[test]
    fn matrix_multiplication() {
        let m1 = mat3x3f32(1., 2., 3., 4., 5., 6., 7., 8., 9.);
        let identity = Matrix3x3f32::IDENTITY;
        assert_eq!(m1 * identity, m1);
        assert_eq!(identity * m1, m1);

        let m2 = mat3x3f32(2., 0., 0., 0., 2., 0., 0., 0., 2.);
        assert_eq!(m1 * m2, m1 * 2.);
    }

    #[test]
    fn determinant() {
        let m = mat3x3f32(1., 2., 3., 0., 1., 4., 5., 6., 0.);
        assert_eq!(m.determinant(), 1.);

        let identity = Matrix3x3f32::IDENTITY;
        assert_eq!(identity.determinant(), 1.);

        let singular = mat3x3f32(1., 2., 3., 2., 4., 6., 3., 6., 9.);
        assert_eq!(singular.determinant(), 0.);
    }

    #[test]
    fn transpose() {
        let m = mat3x3f32(1., 2., 3., 4., 5., 6., 7., 8., 9.);
        let t = m.transpose();
        assert_eq!(t, mat3x3f32(1., 4., 7., 2., 5., 8., 3., 6., 9.));
    }

    #[test]
    fn inverse() {
        let m = mat3x3f32(1., 2., 3., 0., 1., 4., 5., 6., 0.);
        let inv = m.inverse().unwrap();
        let product = m * inv;

        let eps = 1e-5;
        assert!((product.m00 - 1.).abs() < eps);
        assert!((product.m11 - 1.).abs() < eps);
        assert!((product.m22 - 1.).abs() < eps);
        assert!((product.m01).abs() < eps);
        assert!((product.m02).abs() < eps);
        assert!((product.m10).abs() < eps);
        assert!((product.m12).abs() < eps);
        assert!((product.m20).abs() < eps);
        assert!((product.m21).abs() < eps);

        let singular = mat3x3f32(1., 2., 3., 2., 4., 6., 3., 6., 9.);
        assert!(singular.inverse().is_none());
    }

    #[test]
    fn rotation() {
        use std::f32::consts::PI;

        let rot_x = Matrix3x3f32::rotation_x(PI / 2.);
        let rot_y = Matrix3x3f32::rotation_y(PI / 2.);
        let rot_z = Matrix3x3f32::rotation_z(PI / 2.);

        let eps = 1e-5;

        // Rotation around X should leave X axis unchanged
        assert!((rot_x.m00 - 1.).abs() < eps);
        assert!((rot_x.m10).abs() < eps);
        assert!((rot_x.m20).abs() < eps);

        // Rotation around Y should leave Y axis unchanged
        assert!((rot_y.m01).abs() < eps);
        assert!((rot_y.m11 - 1.).abs() < eps);
        assert!((rot_y.m21).abs() < eps);

        // Rotation around Z should leave Z axis unchanged
        assert!((rot_z.m02).abs() < eps);
        assert!((rot_z.m12).abs() < eps);
        assert!((rot_z.m22 - 1.).abs() < eps);
    }

    #[test]
    fn trace() {
        let m = mat3x3f32(1., 2., 3., 4., 5., 6., 7., 8., 9.);
        assert_eq!(m.trace(), 15.);
    }

    #[test]
    fn deref() {
        let m = mat3x3f32(1., 2., 3., 4., 5., 6., 7., 8., 9.);
        assert_eq!(m.m00, 1.);
        assert_eq!(m.m01, 2.);
        assert_eq!(m.m02, 3.);
        assert_eq!(m.m10, 4.);
        assert_eq!(m.m11, 5.);
        assert_eq!(m.m12, 6.);
        assert_eq!(m.m20, 7.);
        assert_eq!(m.m21, 8.);
        assert_eq!(m.m22, 9.);

        let mut m2 = Matrix3x3f32::IDENTITY;
        m2.m01 = 5.;
        assert_eq!(m2.m01, 5.);
    }

    #[test]
    fn format() {
        let m = mat3x3f32(1., 2., 3., 4., 5., 6., 7., 8., 9.);
        let display = format!("{}", m);
        assert_eq!(display, "[1, 2, 3],[4, 5, 6],[7, 8, 9]");
    }

    #[test]
    fn row_col_access() {
        let m = mat3x3f32(1., 2., 3., 4., 5., 6., 7., 8., 9.);

        // Test get_row
        assert_eq!(m.get_row(0), Vector3f32::new(1., 2., 3.));
        assert_eq!(m.get_row(1), Vector3f32::new(4., 5., 6.));
        assert_eq!(m.get_row(2), Vector3f32::new(7., 8., 9.));

        // Test get_col
        assert_eq!(m.get_col(0), Vector3f32::new(1., 4., 7.));
        assert_eq!(m.get_col(1), Vector3f32::new(2., 5., 8.));
        assert_eq!(m.get_col(2), Vector3f32::new(3., 6., 9.));

        // Test set_row
        let mut m2 = Matrix3x3f32::ZEROS;
        m2.set_row(0, Vector3f32::new(1., 2., 3.));
        m2.set_row(1, Vector3f32::new(4., 5., 6.));
        m2.set_row(2, Vector3f32::new(7., 8., 9.));
        assert_eq!(m2, m);

        // Test set_col
        let mut m3 = Matrix3x3f32::ZEROS;
        m3.set_col(0, Vector3f32::new(1., 4., 7.));
        m3.set_col(1, Vector3f32::new(2., 5., 8.));
        m3.set_col(2, Vector3f32::new(3., 6., 9.));
        assert_eq!(m3, m);
    }

    #[test]
    fn matrix_vector_multiplication() {
        let m = mat3x3f32(1., 2., 3., 4., 5., 6., 7., 8., 9.);
        let v = Vector3f32::new(1., 2., 3.);

        let result = m * v;
        assert_eq!(result, Vector3f32::new(14., 32., 50.));

        // Test with identity matrix
        let identity = Matrix3x3f32::IDENTITY;
        assert_eq!(identity * v, v);

        // Test rotation preserves length
        use std::f32::consts::PI;
        let rot = Matrix3x3f32::rotation_z(PI / 4.);
        let v2 = Vector3f32::new(1., 0., 0.);
        let rotated = rot * v2;
        let eps = 1e-5;
        assert!((rotated.length() - v2.length()).abs() < eps);
    }
}
