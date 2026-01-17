use std::simd::prelude::*;

use crate::maths::{Vector2f32, vec2f32};

/// A structure to represent a 2x2 f32 matrix stored in column-major order
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Matrix2x2f32 {
    data: f32x4, // [m00, m10, m01, m11] - column-major
}

/// A structure to allow accessing matrix elements
mod private {
    #[repr(C)]
    pub struct MatrixElements {
        pub m00: f32,
        pub m10: f32,
        pub m01: f32,
        pub m11: f32,
    }
}

/// Implements `Deref` to allow accessing matrix elements
impl std::ops::Deref for Matrix2x2f32 {
    type Target = private::MatrixElements;

    fn deref(&self) -> &Self::Target {
        let value: *const Matrix2x2f32 = self;
        unsafe { &*(value as *const private::MatrixElements) }
    }
}

/// Implements `DerefMut` to allow modifying matrix elements
impl std::ops::DerefMut for Matrix2x2f32 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let value: *mut Matrix2x2f32 = self;
        unsafe { &mut *(value as *mut private::MatrixElements) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Matrix2x2f32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Matrix2x2f32")
            .field("m00", &self.m00)
            .field("m01", &self.m01)
            .field("m10", &self.m10)
            .field("m11", &self.m11)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Matrix2x2f32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}, {}],[{}, {}]",
            self.m00, self.m01, self.m10, self.m11
        )
    }
}

/// Sets a 2x2 matrix to identity as default
impl Default for Matrix2x2f32 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

/// Creates a 2x2 f32 matrix
pub fn mat2x2f32(row_0: &Vector2f32, row_1: &Vector2f32) -> Matrix2x2f32 {
    Matrix2x2f32::new(row_0, row_1)
}

/// A union to cast simd to array and allow const construction
union UnionCast {
    array: [f32; 4],
    simd: Matrix2x2f32,
}

impl Matrix2x2f32 {
    //////////////////////////////////////////////////////////
    /////////////      matrix creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new matrix given its elements in row-major order
    pub const fn new(row_0: &Vector2f32, row_1: &Vector2f32) -> Self {
        Self {
            data: f32x4::from_array([
                row_0.x_const(),
                row_1.x_const(),
                row_0.y_const(),
                row_1.y_const(),
            ]),
        }
    }

    /// Creates a new matrix with all elements set to `value`
    const fn splat(value: f32) -> Self {
        Self::new(&vec2f32(value, value), &vec2f32(value, value))
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
    pub const IDENTITY: Self = Self::diagonal(1., 1.);

    /// Creates a diagonal matrix
    pub const fn diagonal(d0: f32, d1: f32) -> Self {
        Self::new(&vec2f32(d0, 0.), &vec2f32(0., d1))
    }

    //////////////////////////////////////////////////////////
    /////////////     matrix operations      /////////////////
    //////////////////////////////////////////////////////////

    /// Returns the determinant of the matrix
    pub fn determinant(&self) -> f32 {
        self.m00 * self.m11 - self.m01 * self.m10
    }

    /// Returns the trace of the matrix (sum of diagonal elements)
    pub fn trace(&self) -> f32 {
        self.m00 + self.m11
    }

    /// Returns the transpose of the matrix
    pub fn transpose(&self) -> Self {
        Self::new(&vec2f32(self.m00, self.m10), &vec2f32(self.m01, self.m11))
    }

    /// Returns the inverse of the matrix, or None if not invertible
    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det.abs() < f32::EPSILON {
            return None;
        }
        let inv_det = 1.0 / det;
        Some(Self::new(
            &vec2f32(self.m11 * inv_det, -self.m01 * inv_det),
            &vec2f32(-self.m10 * inv_det, self.m00 * inv_det),
        ))
    }

    /// Creates a rotation matrix for the given angle in radians
    pub fn rotation(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(&vec2f32(cos, -sin), &vec2f32(sin, cos))
    }

    /// Creates a scaling matrix
    pub const fn scale(sx: f32, sy: f32) -> Self {
        Self::diagonal(sx, sy)
    }

    //////////////////////////////////////////////////////////
    /////////////   row and column access    /////////////////
    //////////////////////////////////////////////////////////

    /// Returns the specified row as a Vector2f32
    pub fn get_row(&self, row: usize) -> Vector2f32 {
        assert!(row < 2, "Row index out of bounds");
        match row {
            0 => vec2f32(self.m00, self.m01),
            1 => vec2f32(self.m10, self.m11),
            _ => unreachable!(),
        }
    }

    /// Returns the specified column as a Vector2f32
    pub fn get_col(&self, col: usize) -> Vector2f32 {
        assert!(col < 2, "Column index out of bounds");
        match col {
            0 => vec2f32(self.m00, self.m10),
            1 => vec2f32(self.m01, self.m11),
            _ => unreachable!(),
        }
    }

    /// Sets the specified row from a Vector2f32
    pub fn set_row(&mut self, row: usize, v: &Vector2f32) {
        assert!(row < 2, "Row index out of bounds");
        match row {
            0 => {
                self.m00 = v.x;
                self.m01 = v.y;
            }
            1 => {
                self.m10 = v.x;
                self.m11 = v.y;
            }
            _ => unreachable!(),
        }
    }

    /// Sets the specified column from a Vector2f32
    pub fn set_col(&mut self, col: usize, v: &Vector2f32) {
        assert!(col < 2, "Column index out of bounds");
        match col {
            0 => {
                self.m00 = v.x;
                self.m10 = v.y;
            }
            1 => {
                self.m01 = v.x;
                self.m11 = v.y;
            }
            _ => unreachable!(),
        }
    }
}

//////////////////////////////////////////////////////////
//////////////     matrix additions     //////////////////
//////////////////////////////////////////////////////////

/// Element-wise addition
impl std::ops::Add<Matrix2x2f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn add(self, rhs: Matrix2x2f32) -> Self::Output {
        Matrix2x2f32 {
            data: self.data + rhs.data,
        }
    }
}

impl std::ops::Add<&Matrix2x2f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn add(self, rhs: &Matrix2x2f32) -> Self::Output {
        self.add(*rhs)
    }
}

impl std::ops::Add<Matrix2x2f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn add(self, rhs: Matrix2x2f32) -> Self::Output {
        (*self).add(rhs)
    }
}

impl std::ops::Add<&Matrix2x2f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn add(self, rhs: &Matrix2x2f32) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds scalar to all elements
impl std::ops::Add<f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn add(self, rhs: f32) -> Self::Output {
        Matrix2x2f32 {
            data: self.data + f32x4::splat(rhs),
        }
    }
}

impl std::ops::Add<f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn add(self, rhs: f32) -> Self::Output {
        (*self).add(rhs)
    }
}

impl std::ops::Add<&f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn add(self, rhs: &f32) -> Self::Output {
        self.add(*rhs)
    }
}

impl std::ops::Add<&f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn add(self, rhs: &f32) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   matrix subtractions    //////////////////
//////////////////////////////////////////////////////////

/// Element-wise negation
impl std::ops::Neg for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn neg(self) -> Self::Output {
        Matrix2x2f32 { data: -self.data }
    }
}

/// Element-wise subtraction
impl std::ops::Sub<Matrix2x2f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn sub(self, rhs: Matrix2x2f32) -> Self::Output {
        Matrix2x2f32 {
            data: self.data - rhs.data,
        }
    }
}

impl std::ops::Sub<&Matrix2x2f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn sub(self, rhs: &Matrix2x2f32) -> Self::Output {
        self.sub(*rhs)
    }
}

impl std::ops::Sub<Matrix2x2f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn sub(self, rhs: Matrix2x2f32) -> Self::Output {
        (*self).sub(rhs)
    }
}

impl std::ops::Sub<&Matrix2x2f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn sub(self, rhs: &Matrix2x2f32) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Subtracts scalar from all elements
impl std::ops::Sub<f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn sub(self, rhs: f32) -> Self::Output {
        Matrix2x2f32 {
            data: self.data - f32x4::splat(rhs),
        }
    }
}

impl std::ops::Sub<f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn sub(self, rhs: f32) -> Self::Output {
        (*self).sub(rhs)
    }
}

impl std::ops::Sub<&f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn sub(self, rhs: &f32) -> Self::Output {
        self.sub(*rhs)
    }
}

impl std::ops::Sub<&f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn sub(self, rhs: &f32) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   matrix multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Matrix multiplication (not element-wise)
impl std::ops::Mul<Matrix2x2f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn mul(self, rhs: Matrix2x2f32) -> Self::Output {
        Matrix2x2f32::new(
            &vec2f32(
                self.m00 * rhs.m00 + self.m01 * rhs.m10,
                self.m00 * rhs.m01 + self.m01 * rhs.m11,
            ),
            &vec2f32(
                self.m10 * rhs.m00 + self.m11 * rhs.m10,
                self.m10 * rhs.m01 + self.m11 * rhs.m11,
            ),
        )
    }
}

impl std::ops::Mul<&Matrix2x2f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn mul(self, rhs: &Matrix2x2f32) -> Self::Output {
        self.mul(*rhs)
    }
}

impl std::ops::Mul<Matrix2x2f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn mul(self, rhs: Matrix2x2f32) -> Self::Output {
        (*self).mul(rhs)
    }
}

impl std::ops::Mul<&Matrix2x2f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn mul(self, rhs: &Matrix2x2f32) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Scalar multiplication
impl std::ops::Mul<f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn mul(self, rhs: f32) -> Self::Output {
        Matrix2x2f32 {
            data: self.data * f32x4::splat(rhs),
        }
    }
}

impl std::ops::Mul<f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn mul(self, rhs: f32) -> Self::Output {
        (*self).mul(rhs)
    }
}

impl std::ops::Mul<&f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn mul(self, rhs: &f32) -> Self::Output {
        self.mul(*rhs)
    }
}

impl std::ops::Mul<&f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn mul(self, rhs: &f32) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     matrix divisions     //////////////////
//////////////////////////////////////////////////////////

/// Element-wise division
impl std::ops::Div<Matrix2x2f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn div(self, rhs: Matrix2x2f32) -> Self::Output {
        Matrix2x2f32 {
            data: self.data / rhs.data,
        }
    }
}

impl std::ops::Div<&Matrix2x2f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn div(self, rhs: &Matrix2x2f32) -> Self::Output {
        self.div(*rhs)
    }
}

impl std::ops::Div<Matrix2x2f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn div(self, rhs: Matrix2x2f32) -> Self::Output {
        (*self).div(rhs)
    }
}

impl std::ops::Div<&Matrix2x2f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn div(self, rhs: &Matrix2x2f32) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Scalar division
impl std::ops::Div<f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn div(self, rhs: f32) -> Self::Output {
        Matrix2x2f32 {
            data: self.data / f32x4::splat(rhs),
        }
    }
}

impl std::ops::Div<f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn div(self, rhs: f32) -> Self::Output {
        (*self).div(rhs)
    }
}

impl std::ops::Div<&f32> for Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn div(self, rhs: &f32) -> Self::Output {
        self.div(*rhs)
    }
}

impl std::ops::Div<&f32> for &Matrix2x2f32 {
    type Output = Matrix2x2f32;

    fn div(self, rhs: &f32) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      matrix assign       //////////////////
//////////////////////////////////////////////////////////

impl std::ops::AddAssign<Matrix2x2f32> for Matrix2x2f32 {
    fn add_assign(&mut self, rhs: Matrix2x2f32) {
        self.data = self.data + rhs.data;
    }
}

impl std::ops::AddAssign<&Matrix2x2f32> for Matrix2x2f32 {
    fn add_assign(&mut self, rhs: &Matrix2x2f32) {
        self.add_assign(*rhs);
    }
}

impl std::ops::AddAssign<f32> for Matrix2x2f32 {
    fn add_assign(&mut self, rhs: f32) {
        self.data = self.data + f32x4::splat(rhs);
    }
}

impl std::ops::AddAssign<&f32> for Matrix2x2f32 {
    fn add_assign(&mut self, rhs: &f32) {
        self.add_assign(*rhs);
    }
}

impl std::ops::SubAssign<Matrix2x2f32> for Matrix2x2f32 {
    fn sub_assign(&mut self, rhs: Matrix2x2f32) {
        self.data = self.data - rhs.data;
    }
}

impl std::ops::SubAssign<&Matrix2x2f32> for Matrix2x2f32 {
    fn sub_assign(&mut self, rhs: &Matrix2x2f32) {
        self.sub_assign(*rhs);
    }
}

impl std::ops::SubAssign<f32> for Matrix2x2f32 {
    fn sub_assign(&mut self, rhs: f32) {
        self.data = self.data - f32x4::splat(rhs);
    }
}

impl std::ops::SubAssign<&f32> for Matrix2x2f32 {
    fn sub_assign(&mut self, rhs: &f32) {
        self.sub_assign(*rhs);
    }
}

impl std::ops::MulAssign<Matrix2x2f32> for Matrix2x2f32 {
    fn mul_assign(&mut self, rhs: Matrix2x2f32) {
        *self = *self * rhs;
    }
}

impl std::ops::MulAssign<&Matrix2x2f32> for Matrix2x2f32 {
    fn mul_assign(&mut self, rhs: &Matrix2x2f32) {
        self.mul_assign(*rhs);
    }
}

impl std::ops::MulAssign<f32> for Matrix2x2f32 {
    fn mul_assign(&mut self, rhs: f32) {
        self.data = self.data * f32x4::splat(rhs);
    }
}

impl std::ops::MulAssign<&f32> for Matrix2x2f32 {
    fn mul_assign(&mut self, rhs: &f32) {
        self.mul_assign(*rhs);
    }
}

impl std::ops::DivAssign<Matrix2x2f32> for Matrix2x2f32 {
    fn div_assign(&mut self, rhs: Matrix2x2f32) {
        self.data = self.data / rhs.data;
    }
}

impl std::ops::DivAssign<&Matrix2x2f32> for Matrix2x2f32 {
    fn div_assign(&mut self, rhs: &Matrix2x2f32) {
        self.div_assign(*rhs);
    }
}

impl std::ops::DivAssign<f32> for Matrix2x2f32 {
    fn div_assign(&mut self, rhs: f32) {
        self.data = self.data / f32x4::splat(rhs);
    }
}

impl std::ops::DivAssign<&f32> for Matrix2x2f32 {
    fn div_assign(&mut self, rhs: &f32) {
        self.div_assign(*rhs);
    }
}

//////////////////////////////////////////////////////////
//////////////   matrix-vector operations ////////////////
//////////////////////////////////////////////////////////

/// Matrix-vector multiplication: M * v
impl std::ops::Mul<Vector2f32> for Matrix2x2f32 {
    type Output = Vector2f32;

    fn mul(self, rhs: Vector2f32) -> Self::Output {
        vec2f32(
            self.m00 * rhs.x + self.m01 * rhs.y,
            self.m10 * rhs.x + self.m11 * rhs.y,
        )
    }
}

impl std::ops::Mul<&Vector2f32> for Matrix2x2f32 {
    type Output = Vector2f32;

    fn mul(self, rhs: &Vector2f32) -> Self::Output {
        self.mul(*rhs)
    }
}

impl std::ops::Mul<Vector2f32> for &Matrix2x2f32 {
    type Output = Vector2f32;

    fn mul(self, rhs: Vector2f32) -> Self::Output {
        (*self).mul(rhs)
    }
}

impl std::ops::Mul<&Vector2f32> for &Matrix2x2f32 {
    type Output = Vector2f32;

    fn mul(self, rhs: &Vector2f32) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
///////////////     matrix indices     ///////////////////
//////////////////////////////////////////////////////////
impl std::ops::Index<(usize, usize)> for Matrix2x2f32 {
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        match index {
            (0, 0) => &self.m00,
            (0, 1) => &self.m01,
            (1, 0) => &self.m10,
            (1, 1) => &self.m11,
            _ => panic!("Index out of bounds"),
        }
    }
}

impl std::ops::IndexMut<(usize, usize)> for Matrix2x2f32 {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        match index {
            (0, 0) => &mut self.m00,
            (0, 1) => &mut self.m01,
            (1, 0) => &mut self.m10,
            (1, 1) => &mut self.m11,
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
        let m1 = mat2x2f32(&vec2f32(1., 2.), &vec2f32(3., 4.));
        let m2 = Matrix2x2f32::new(&vec2f32(1., 2.), &vec2f32(3., 4.));
        assert_eq!(m1, m2);

        let identity = Matrix2x2f32::IDENTITY;
        assert_eq!(identity.m00, 1.);
        assert_eq!(identity.m11, 1.);
        assert_eq!(identity.m01, 0.);
        assert_eq!(identity.m10, 0.);
    }

    #[test]
    fn operators() {
        let m1 = mat2x2f32(&vec2f32(1., 2.), &vec2f32(3., 4.));
        let m2 = mat2x2f32(&vec2f32(5., 6.), &vec2f32(7., 8.));

        let add = m1 + m2;
        assert_eq!(add, mat2x2f32(&vec2f32(6., 8.), &vec2f32(10., 12.)));

        let sub = m2 - m1;
        assert_eq!(sub, mat2x2f32(&vec2f32(4., 4.), &vec2f32(4., 4.)));

        let scaled = m1 * 2.;
        assert_eq!(scaled, mat2x2f32(&vec2f32(2., 4.), &vec2f32(6., 8.)));
    }

    #[test]
    fn matrix_multiplication() {
        let m1 = mat2x2f32(&vec2f32(1., 2.), &vec2f32(3., 4.));
        let m2 = mat2x2f32(&vec2f32(2., 0.), &vec2f32(1., 2.));
        let result = m1 * m2;
        assert_eq!(result, mat2x2f32(&vec2f32(4., 4.), &vec2f32(10., 8.)));

        let identity = Matrix2x2f32::IDENTITY;
        assert_eq!(m1 * identity, m1);
    }

    #[test]
    fn determinant() {
        let m = mat2x2f32(&vec2f32(1., 2.), &vec2f32(3., 4.));
        assert_eq!(m.determinant(), -2.);

        let identity = Matrix2x2f32::IDENTITY;
        assert_eq!(identity.determinant(), 1.);
    }

    #[test]
    fn transpose() {
        let m = mat2x2f32(&vec2f32(1., 2.), &vec2f32(3., 4.));
        let t = m.transpose();
        assert_eq!(t, mat2x2f32(&vec2f32(1., 3.), &vec2f32(2., 4.)));
    }

    #[test]
    fn inverse() {
        let m = mat2x2f32(&vec2f32(1., 2.), &vec2f32(3., 4.));
        let inv = m.inverse().unwrap();
        let product = m * inv;

        assert!((product.m00 - 1.).abs() < 1e-5);
        assert!((product.m11 - 1.).abs() < 1e-5);
        assert!((product.m01).abs() < 1e-5);
        assert!((product.m10).abs() < 1e-5);

        let singular = mat2x2f32(&vec2f32(1., 2.), &vec2f32(2., 4.));
        assert!(singular.inverse().is_none());
    }

    #[test]
    fn rotation() {
        use std::f32::consts::PI;
        let rot90 = Matrix2x2f32::rotation(PI / 2.);

        assert!((rot90.m00 - 0.).abs() < 1e-5);
        assert!((rot90.m01 - (-1.)).abs() < 1e-5);
        assert!((rot90.m10 - 1.).abs() < 1e-5);
        assert!((rot90.m11 - 0.).abs() < 1e-5);
    }

    #[test]
    fn trace() {
        let m = mat2x2f32(&vec2f32(1., 2.), &vec2f32(3., 4.));
        assert_eq!(m.trace(), 5.);
    }

    #[test]
    fn deref() {
        let m = mat2x2f32(&vec2f32(1., 2.), &vec2f32(3., 4.));
        assert_eq!(m.m00, 1.);
        assert_eq!(m.m01, 2.);
        assert_eq!(m.m10, 3.);
        assert_eq!(m.m11, 4.);

        let mut m2 = Matrix2x2f32::IDENTITY;
        m2.m01 = 5.;
        assert_eq!(m2.m01, 5.);
    }

    #[test]
    fn format() {
        let m = mat2x2f32(&vec2f32(1., 2.), &vec2f32(3., 4.));
        let display = format!("{}", m);
        assert_eq!(display, "[1, 2],[3, 4]");
    }

    #[test]
    fn row_col_access() {
        let m = mat2x2f32(&vec2f32(1., 2.), &vec2f32(4., 5.));

        // Test get_row
        assert_eq!(m.get_row(0), vec2f32(1., 2.));
        assert_eq!(m.get_row(1), vec2f32(4., 5.));

        // Test get_col
        assert_eq!(m.get_col(0), vec2f32(1., 4.));
        assert_eq!(m.get_col(1), vec2f32(2., 5.));

        // Test set_row
        let mut m2 = Matrix2x2f32::ZEROS;
        m2.set_row(0, &vec2f32(1., 2.));
        m2.set_row(1, &vec2f32(4., 5.));
        assert_eq!(m2, m);

        // Test set_col
        let mut m3 = Matrix2x2f32::ZEROS;
        m3.set_col(0, &vec2f32(1., 4.));
        m3.set_col(1, &vec2f32(2., 5.));
        assert_eq!(m3, m);
    }

    #[test]
    fn matrix_vector_multiplication() {
        let m = mat2x2f32(&vec2f32(1., 2.), &vec2f32(4., 5.));
        let v = vec2f32(1., 2.);

        let result = m * v;
        assert_eq!(result, vec2f32(5., 14.));

        // Test with identity matrix
        let identity = Matrix2x2f32::IDENTITY;
        assert_eq!(identity * v, v);

        // Test rotation preserves length
        use std::f32::consts::PI;
        let rot = Matrix2x2f32::rotation(PI / 4.);
        let v2 = vec2f32(1., 0.);
        let rotated = rot * v2;
        let eps = 1e-5;
        assert!((rotated.length() - v2.length()).abs() < eps);
    }

    /// Tests indices access
    #[test]
    fn indices() {
        let mut m = mat2x2f32(&vec2f32(1., 2.), &vec2f32(5., 6.));
        assert_eq!(m[(0, 0)], 1.);
        assert_eq!(m[(0, 1)], 2.);
        assert_eq!(m[(1, 0)], 5.);
        assert_eq!(m[(1, 1)], 6.);

        m[(0, 0)] = 2.;
        assert_eq!(m[(1, 1)], 6.);
        assert_eq!(m[(0, 0)], 2.);
        m[(0, 1)] = 3.;
        assert_eq!(m[(1, 1)], 6.);
        assert_eq!(m[(0, 1)], 3.);
        m[(1, 0)] = 6.;
        assert_eq!(m[(1, 1)], 6.);
        assert_eq!(m[(1, 0)], 6.);
        m[(1, 1)] = 7.;
        assert_eq!(m[(1, 1)], 7.);
    }
}
