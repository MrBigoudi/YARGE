use std::simd::prelude::*;

/// A structure to represent a 2 dimensional f32 vector
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector2 {
    data: f32x2,
}

/// A structure to be able to create `.x` and `.y` getters and setters
mod private {
    #[repr(C)]
    pub struct CoordsVector2 {
        pub x: f32,
        pub y: f32,
    }
}

/// Implements `Deref` to allow accessing `.x` and `.y`
impl std::ops::Deref for Vector2 {
    type Target = private::CoordsVector2;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self as *const Vector2 as *const private::CoordsVector2) }
    }
}

/// Implements `DerefMut` to allow modifying `.x` and `.y`
impl std::ops::DerefMut for Vector2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(self as *mut Vector2 as *mut private::CoordsVector2) }
    }
}

/// Overrides the debug trait
impl std::fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector2")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

/// Overrides the display trait
impl std::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Sets a 2 dimensional f32 vector to `[0., 0.]` as default
impl Default for Vector2 {
    fn default() -> Self {
        Self::ZEROS
    }
}

/// Creates a 2 dimensional f32 vector
pub fn vec2(x: f32, y: f32) -> Vector2 {
    Vector2::new(x, y)
}

/// A union to cast simd to array and allow const construction
union UnionCast {
    array: [f32; 2],
    simd: Vector2,
}

impl Vector2 {
    //////////////////////////////////////////////////////////
    /////////////      vector creation       /////////////////
    //////////////////////////////////////////////////////////

    /// Creates a new vector given its coordinates
    pub const fn new(x: f32, y: f32) -> Self {
        unsafe { UnionCast { array: [x, y] }.simd }
    }

    /// Creates a new vector with all coordinates set to `value`
    const fn splat(value: f32) -> Self {
        Self::new(value, value)
    }

    /// Creates a new vector filled with `values`
    pub const fn filled(value: f32) -> Self {
        Self::splat(value)
    }

    /// Creates a new vector filled with ones
    pub const ONES: Self = Self::splat(1.);

    /// Creates a new vector filled with negative ones
    pub const NEG_ONES: Self = Self::splat(-1.);

    /// Creates a new vector filled with zeros
    pub const ZEROS: Self = Self::splat(0.);

    /// Creates a new vector filled with `f32::INFINITY`
    pub const INFINITY: Self = Self::splat(f32::INFINITY);

    /// Creates a new vector filled with `f32::NEG_INFINITY`
    pub const NEG_INFINITY: Self = Self::splat(f32::NEG_INFINITY);

    /// Creates a new vector filled with `f32::MIN``
    pub const MIN: Self = Self::splat(f32::MIN);

    /// Creates a new vector filled with `f32::MAX`
    pub const MAX: Self = Self::splat(f32::MAX);

    /// Creates a new vector pointing along the positive X axis
    pub const X: Self = Self::new(1., 0.);

    /// Creates a new vector pointing along the positive Y axis
    pub const Y: Self = Self::new(0., 1.);

    /// Creates a new vector pointing along the negative X axis
    pub const NEG_X: Self = Self::new(-1., 0.);

    /// Creates a new vector pointing along the negative Y axis
    pub const NEG_Y: Self = Self::new(0., -1.);

    //////////////////////////////////////////////////////////
    /////////////     vector operations      /////////////////
    //////////////////////////////////////////////////////////

    /// Sums up the element of the vector
    pub fn prefix_sum(self) -> f32 {
        self.x + self.y
    }

    /// Dot product between two vectors
    pub fn dot(v1: &Vector2, v2: &Vector2) -> f32 {
        (v1 * v2).prefix_sum()
    }

    /// Returns the length of the vector
    pub fn length(&self) -> f32 {
        Self::dot(self, self).sqrt()
    }
}

//////////////////////////////////////////////////////////
//////////////     vector additions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            data: self.data + rhs.data,
        }
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<&Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &Vector2) -> Self::Output {
        self.add(*rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<Vector2> for &Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Components wise addition
/// v1 + v2 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::Add<&Vector2> for &Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &Vector2) -> Self::Output {
        (*self).add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v1 + f = [v1.x + f, v1.y + f]
impl std::ops::Add<f32> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: f32) -> Self::Output {
        Vector2 {
            data: self.data + f32x2::from_array([rhs, rhs]),
        }
    }
}

/// Adds `rhs` to all components of the vector
/// v1 + f = [v1.x + f, v1.y + f]
impl std::ops::Add<f32> for &Vector2 {
    type Output = Vector2;

    fn add(self, rhs: f32) -> Self::Output {
        (*self).add(rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v1 + f = [v1.x + f, v1.y + f]
impl std::ops::Add<&f32> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &f32) -> Self::Output {
        self.add(*rhs)
    }
}

/// Adds `rhs` to all components of the vector
/// v1 + f = [v1.x + f, v1.y + f]
impl std::ops::Add<&f32> for &Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &f32) -> Self::Output {
        (*self).add(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector substractions    /////////////////
//////////////////////////////////////////////////////////

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            data: self.data - rhs.data,
        }
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<&Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: &Vector2) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<Vector2> for &Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Components wise substraction
/// v1 - v2 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::Sub<&Vector2> for &Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: &Vector2) -> Self::Output {
        (*self).sub(*rhs)
    }
}

/// Substract `rhs` to all components of the vector
/// v1 - f = [v1.x - f, v1.y - f]
impl std::ops::Sub<f32> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: f32) -> Self::Output {
        Vector2 {
            data: self.data - f32x2::from_array([rhs, rhs]),
        }
    }
}

/// Substracts `rhs` to all components of the vector
/// v1 - f = [v1.x - f, v1.y - f]
impl std::ops::Sub<f32> for &Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: f32) -> Self::Output {
        (*self).sub(rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v1 - f = [v1.x - f, v1.y - f]
impl std::ops::Sub<&f32> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: &f32) -> Self::Output {
        self.sub(*rhs)
    }
}

/// Substracts `rhs` to all components of the vector
/// v1 - f = [v1.x - f, v1.y - f]
impl std::ops::Sub<&f32> for &Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: &f32) -> Self::Output {
        (*self).sub(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////   vector multiplications   ////////////////
//////////////////////////////////////////////////////////

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            data: self.data * rhs.data,
        }
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<&Vector2> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: &Vector2) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<Vector2> for &Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Components wise multiplication
/// v1 * v2 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::Mul<&Vector2> for &Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: &Vector2) -> Self::Output {
        (*self).mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v1 * f = [v1.x * f, v1.y * f]
impl std::ops::Mul<f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector2 {
            data: self.data * f32x2::from_array([rhs, rhs]),
        }
    }
}

/// Multiplies all components of the vector by `rhs`
/// v1 * f = [v1.x * f, v1.y * f]
impl std::ops::Mul<f32> for &Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Self::Output {
        (*self).mul(rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v1 * f = [v1.x * f, v1.y * f]
impl std::ops::Mul<&f32> for Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: &f32) -> Self::Output {
        self.mul(*rhs)
    }
}

/// Multiplies all components of the vector by `rhs`
/// v1 * f = [v1.x * f, v1.y * f]
impl std::ops::Mul<&f32> for &Vector2 {
    type Output = Vector2;

    fn mul(self, rhs: &f32) -> Self::Output {
        (*self).mul(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////     vector divisions     //////////////////
//////////////////////////////////////////////////////////

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<Vector2> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            data: self.data / rhs.data,
        }
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<&Vector2> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: &Vector2) -> Self::Output {
        self.div(*rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<Vector2> for &Vector2 {
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Components wise division
/// v1 / v2 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::Div<&Vector2> for &Vector2 {
    type Output = Vector2;

    fn div(self, rhs: &Vector2) -> Self::Output {
        (*self).div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v1 / f = [v1.x / f, v1.y / f]
impl std::ops::Div<f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Self::Output {
        Vector2 {
            data: self.data / f32x2::from_array([rhs, rhs]),
        }
    }
}

/// Divides all components of the vector by `rhs`
/// v1 / f = [v1.x / f, v1.y / f]
impl std::ops::Div<f32> for &Vector2 {
    type Output = Vector2;

    fn div(self, rhs: f32) -> Self::Output {
        (*self).div(rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v1 / f = [v1.x / f, v1.y / f]
impl std::ops::Div<&f32> for Vector2 {
    type Output = Vector2;

    fn div(self, rhs: &f32) -> Self::Output {
        self.div(*rhs)
    }
}

/// Divides all components of the vector by `rhs`
/// v1 / f = [v1.x / f, v1.y / f]
impl std::ops::Div<&f32> for &Vector2 {
    type Output = Vector2;

    fn div(self, rhs: &f32) -> Self::Output {
        (*self).div(*rhs)
    }
}

//////////////////////////////////////////////////////////
//////////////      vector assign       //////////////////
//////////////////////////////////////////////////////////

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::AddAssign<Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.data = self.data + rhs.data;
    }
}

/// Components wise addition
/// v1 += v2 <=> v1 = [v1.x + v2.x, v1.y + v2.y]
impl std::ops::AddAssign<&Vector2> for Vector2 {
    fn add_assign(&mut self, rhs: &Vector2) {
        self.add_assign(*rhs);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v*x + f, v*y + f]
impl std::ops::AddAssign<f32> for Vector2 {
    fn add_assign(&mut self, rhs: f32) {
        self.data = self.data + f32x2::from_array([rhs, rhs]);
    }
}

/// Adds `rhs` to all components of the vector
/// v += f <=> v = [v*x + f, v*y + f]
impl std::ops::AddAssign<&f32> for Vector2 {
    fn add_assign(&mut self, rhs: &f32) {
        self.add_assign(*rhs);
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::SubAssign<Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: Vector2) {
        self.data = self.data - rhs.data;
    }
}

/// Components wise substraction
/// v1 -= v2 <=> v1 = [v1.x - v2.x, v1.y - v2.y]
impl std::ops::SubAssign<&Vector2> for Vector2 {
    fn sub_assign(&mut self, rhs: &Vector2) {
        self.sub_assign(*rhs);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v*x - f, v*y - f]
impl std::ops::SubAssign<f32> for Vector2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.data = self.data - f32x2::from_array([rhs, rhs]);
    }
}

/// Substracts `rhs` to all components of the vector
/// v -= f <=> v = [v*x - f, v*y - f]
impl std::ops::SubAssign<&f32> for Vector2 {
    fn sub_assign(&mut self, rhs: &f32) {
        self.sub_assign(*rhs);
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::MulAssign<Vector2> for Vector2 {
    fn mul_assign(&mut self, rhs: Vector2) {
        self.data = self.data * rhs.data;
    }
}

/// Components wise multiplication
/// v1 *= v2 <=> v1 = [v1.x * v2.x, v1.y * v2.y]
impl std::ops::MulAssign<&Vector2> for Vector2 {
    fn mul_assign(&mut self, rhs: &Vector2) {
        self.mul_assign(*rhs);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v*x / f, v*y / f]
impl std::ops::MulAssign<f32> for Vector2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.data = self.data * f32x2::from_array([rhs, rhs]);
    }
}

/// Multiplies all components of the vector by `rhs`
/// v *= f <=> v = [v*x / f, v*y / f]
impl std::ops::MulAssign<&f32> for Vector2 {
    fn mul_assign(&mut self, rhs: &f32) {
        self.mul_assign(*rhs);
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::DivAssign<Vector2> for Vector2 {
    fn div_assign(&mut self, rhs: Vector2) {
        self.data = self.data / rhs.data;
    }
}

/// Components wise division
/// v1 /= v2 <=> v1 = [v1.x / v2.x, v1.y / v2.y]
impl std::ops::DivAssign<&Vector2> for Vector2 {
    fn div_assign(&mut self, rhs: &Vector2) {
        self.div_assign(*rhs);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v1 = [v.x / f, v.y / f]
impl std::ops::DivAssign<f32> for Vector2 {
    fn div_assign(&mut self, rhs: f32) {
        self.data = self.data / f32x2::from_array([rhs, rhs]);
    }
}

/// Divides all components of the vector by `rhs`
/// v /= f <=> v = [v.x / f, v.y / f]
impl std::ops::DivAssign<&f32> for Vector2 {
    fn div_assign(&mut self, rhs: &f32) {
        self.div_assign(*rhs);
    }
}

//////////////////////////////////////////////////////////
///////////////     vector tests      ////////////////////
//////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests 2 dimensional vector initialization
    #[test]
    fn initialization() {
        let v1 = vec2(1., 1.);
        let v2 = Vector2::ONES;
        assert_eq!(v1, v2);
    }

    /// Tests 2 dimensional vector operators
    #[test]
    fn operators() {
        let v1 = vec2(2., 3.);
        let v2 = vec2(4., 5.);
        assert_eq!(v1 + v2, vec2(6., 8.));
        let mut v3 = v1;
        assert_eq!(v1, v3);
        v3 += v1;
        assert_eq!(v3, vec2(4., 6.));

        v3 += 1.;
        assert_eq!(v3, vec2(5., 7.));
        assert_eq!(v3 + v3 - v3, v3);

        assert_eq!(v3 * 2., vec2(10., 14.));
        assert_eq!(v1 * v2, vec2(8., 15.));

        assert_eq!(v3 / v3, Vector2::ONES);
        v3 /= v3;
        assert_eq!(v3, Vector2::ONES);
    }

    /// Tests 2 dimensional vector operations
    #[test]
    fn operations() {
        let v1 = vec2(1., 2.);
        let v2 = vec2(4., 3.);
        assert_eq!(Vector2::dot(&v1, &v2), 10.);
        let vx = Vector2::X;
        let vy = Vector2::Y;
        assert_eq!(Vector2::dot(&vx, &vy), 0.);

        assert_eq!(vx.length(), 1.);
        assert_eq!(vy.length(), 1.);
        assert_eq!(v1.length(), 5f32.sqrt());
    }

    /// Tests 2 dimensional vector's fields getters and setters
    #[test]
    fn deref() {
        let v1 = vec2(5., 2.);
        assert_eq!(v1.x, 5.);
        assert_eq!(v1.y, 2.);
        let mut v2 = Vector2::ZEROS;
        assert_eq!(v2.x, 0.);
        assert_eq!(v2.y, 0.);
        v2.x = 1.;
        assert_eq!(v2.x, 1.);
        v2.y = 3.;
        assert_eq!(v2.y, 3.);
        assert_eq!(v2.x, 1.);
    }

    /// Tests the formatting of 2 dimensional vectors
    #[test]
    fn format() {
        let v1 = vec2(3.1, 4.2);
        assert_eq!(v1.to_string(), "(3.1, 4.2)");
        assert_eq!(format!("{:?}", v1), "Vector2 { x: 3.1, y: 4.2 }");
    }
}
